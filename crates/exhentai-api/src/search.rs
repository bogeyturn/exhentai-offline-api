use crate::connections::{establish_connection_postgres, Connections};
use crate::models::api_dump::ApiDump;
use diesel::serialize::ToSql;
use diesel::{sql_query, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
struct SearchRequest {
    data: Array,
    order: Order,
    duplicate_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct FilterRequest {
    filter: Vec<String>,
    name: String,
}

impl FilterRequest {
    fn generate_materialized_view(&self, conn: &mut Connections) {
        let db_name = "ex_gallery";
        let join_table = "p_mixed";

        let s = self
            .filter
            .iter()
            .map(|v| array("language", "", &save_sql_str(v)))
            .collect::<Vec<_>>();
        let mut f = s
            .iter()
            .enumerate()
            .map(|(index, v)| format!("WHEN {} THEN {}", v, index + 1))
            .collect::<Vec<_>>();
        f.push(format!("ELSE {}", self.filter.len() + 1));
        let mv_name = format!("{}_{}", self.name, db_name);
        let sql = format!("CREATE MATERIALIZED VIEW {mv_name} AS SELECT DISTINCT ON ({join_table}.p) ex_gallery.* FROM {db_name} JOIN {join_table} ON {db_name}.gid = {join_table}.gid ORDER BY {join_table}.p, CASE {f} END, CASE WHEN ({s}) THEN {db_name}.gid ELSE -{db_name}.gid END DESC", f = f.join(" "), s = s.join(" OR "));
        conn.get_api_dump_service().execute(&sql).unwrap();
        //REFRESH MATERIALIZED VIEW my_materialized_view;
        //https://www.postgresql.org/docs/current/rules-materializedviews.html
    }
}

impl ToString for SearchRequest {
    fn to_string(&self) -> String {
        let query = match self.data.to_string() {
            Some(v) => format!(" WHERE {}", v),
            None => String::new(),
        };
        let db = match &self.duplicate_filter {
            None => "ex_gallery".to_string(),
            Some(v) => {
                format!("{v}_ex_gallery")
            }
        };
        let sql = format!("SELECT * FROM {}{} LIMIT 10;", db, query);
        sql
    }
}

fn generate_mv() {
    let mut conn = Connections::new(false);
    FilterRequest {
        filter: vec!["english".to_string()],
        name: "english".to_string(),
    }
    .generate_materialized_view(&mut conn);
}

#[test]
fn run_search() {
    let arr = Array {
        or: true,
        items: vec![
            ItemOrArray::Array(Array {
                or: true,
                items: vec![],
            }),
            ItemOrArray::Item(Item {
                not: false,
                data: ItemData::Tag {
                    tag: "test".to_string(),
                    kind: TagKind::All,
                },
            }),
        ],
    };

    let sr = SearchRequest {
        data: arr,
        order: Order {
            desc: false,
            kind: OrderKind::Id,
        },
        duplicate_filter: Some("english".to_string()),
    };
    let mut conn = Connections::new(false);
    let res = conn.get_api_dump_service().execute(&sr.to_string());

    println!("{:#?}", res);
    panic!()
}

fn save_sql_str(str: &str) -> String {
    str.replace('\'', "''")
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

#[derive(Serialize, Deserialize)]
struct Order {
    desc: bool,
    kind: OrderKind,
}

#[derive(Serialize, Deserialize)]
enum OrderKind {
    Id,
    Title,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ItemOrArray {
    Item(Item),
    Array(Array),
}

impl ItemOrArray {
    fn to_string(&self) -> Option<String> {
        match self {
            ItemOrArray::Item(v) => Some(v.to_string()),
            ItemOrArray::Array(v) => v.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Array {
    or: bool,
    items: Vec<ItemOrArray>,
}

impl Array {
    fn to_string(&self) -> Option<String> {
        let arr: Vec<_> = self.items.iter().filter_map(|v| v.to_string()).collect();
        if arr.is_empty() {
            return None;
        }
        let v = arr.join(match self.or {
            true => " or ",
            false => " and ",
        });
        if arr.len() == 1 {
            return Some(v);
        }
        Some(format!("({})", v))
    }
}

#[derive(Serialize, Deserialize)]
struct Item {
    not: bool,
    data: ItemData,
}

#[derive(Serialize, Deserialize)]
enum ItemData {
    Id { related: bool, id: i32 },
    Title(String),
    Category(String),
    Artist(String),
    Group(String),
    Uploader(String),
    Filecount { eq: bool, bigger: bool, count: i32 },
    Rating { eq: bool, bigger: bool, rating: f32 },
    Parody(String),
    Character(String),
    Tag { tag: String, kind: TagKind },
    Language(String),
    Cosplayer(String),
}

impl ToString for Item {
    fn to_string(&self) -> String {
        let n = match self.not {
            true => "not ",
            false => "",
        };
        let n2 = match self.not {
            true => "!",
            false => "",
        };
        match &self.data {
            //TODO: related
            ItemData::Id { related, id } => format!("id {}= {}", n2, id),
            ItemData::Title(v) => format!(
                "(title {n}LIKE '%{v}%' OR title_jpn {n}LIKE '%{v}%')",
                v = save_sql_str(v)
            ),
            ItemData::Category(v) => format!("category {}= {}", n2, save_sql_str(v)),
            ItemData::Artist(v) => array("artist", n, &save_sql_str(v)),
            ItemData::Group(v) => array("group", n, &save_sql_str(v)),
            ItemData::Uploader(v) => format!("uploader {}= {}", n2, &save_sql_str(v)),
            ItemData::Filecount { eq, bigger, count } => format!(
                "filecount {}",
                rating_filecount(self.not, *bigger, *eq, count)
            ),
            ItemData::Rating { eq, bigger, rating } => format!(
                "filecount {}",
                rating_filecount(self.not, *bigger, *eq, rating)
            ),
            ItemData::Parody(v) => array("parody", n, &save_sql_str(v)),
            ItemData::Character(v) => array("character", n, &save_sql_str(v)),
            ItemData::Tag { tag, kind } => {
                let tag = &save_sql_str(tag);
                let mut k = match kind {
                    TagKind::Female => vec!["female"],
                    TagKind::Male => vec!["male"],
                    TagKind::Mixed => vec!["mixed"],
                    TagKind::Other => vec!["other"],
                    TagKind::Rest => vec!["rest"],
                    TagKind::All => vec!["female", "male", "mixed", "other", "rest"],
                }
                .iter()
                .map(|v| array(v, n, tag))
                .collect::<Vec<_>>();
                if k.len() > 1 {
                    format!("({})", k.join(" or "))
                } else {
                    k.remove(0)
                }
            }
            ItemData::Language(v) => array("language", n, &save_sql_str(v)),
            ItemData::Cosplayer(v) => array("cosplayer", n, &save_sql_str(v)),
        }
    }
}

fn array(col: &str, n: &str, v: &str) -> String {
    format!("{} {}LIKE '%''{}''%'", col, n, v)
}

fn rating_filecount(not: bool, mut bigger: bool, mut eq: bool, number: impl Display) -> String {
    if not {
        eq = !eq;
        bigger = !bigger;
    }

    let eq = match eq {
        true => "=",
        false => "",
    };
    let sb = match bigger {
        true => ">",
        false => "<",
    };
    format!("{}{} {}", sb, eq, number)
}

#[derive(Serialize, Deserialize)]
enum TagKind {
    Female,
    Male,
    Mixed,
    Other,
    Rest,
    All,
}
