use models::Category;

use crate::{
    connections::Connections,
    routes::{
        filter::FilterRequest,
        search::{Array, Item, ItemData, ItemOrArray, SearchRequest, TagKind},
    },
};
use std::fmt::Display;

impl FilterRequest {
    pub fn generate_materialized_view(&self, conn: &mut Connections) {
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
        let order = format!(
            "ORDER BY {} {}",
            self.order.kind.to_string(),
            match self.order.desc {
                true => "DESC",
                false => "ASC",
            }
        );
        let sql = format!("SELECT * FROM {db}{query} {order} LIMIT 10;");
        sql
    }
}

fn save_sql_str(str: &str) -> String {
    str.replace('\'', "''").replace('\\', "\\\\")
}

fn save_sql_like_str(str: &str) -> String {
    str.replace('\\', "\\\\")
        .replace('\'', "''")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

impl ItemOrArray {
    fn to_string(&self) -> Option<String> {
        match self {
            ItemOrArray::Item(v) => Some(v.to_string()),
            ItemOrArray::Array(v) => v.to_string(),
        }
    }
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
            ItemData::Id { related, id } => format!("gid {}= {}", n2, id),
            ItemData::Title(v) => format!(
                "(title {n}LIKE '%{v}%' OR title_jpn {n}LIKE '%{v}%')",
                v = save_sql_like_str(v)
            ),
            ItemData::Category(v) => {
                format!("category {}= {}", n2, Category::from(v.as_str()) as i32)
            }
            ItemData::Rating { eq, bigger, rating } => format!(
                "rating {}",
                rating_filecount(self.not, *bigger, *eq, rating)
            ),
            ItemData::Language(v) => array("languages", n, &save_sql_str(v)),
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
            ItemData::Artist(v) => array("artists", n, &save_sql_str(v)),
            ItemData::Group(v) => array("groups", n, &save_sql_str(v)),
            ItemData::Cosplayer(v) => array("cosplayers", n, &save_sql_str(v)),
            ItemData::Uploader(v) => format!(
                "uploader {n2}= (SELECT id FROM users WHERE name = '{}' LIMIT 1)",
                &save_sql_str(v)
            ),
            ItemData::Filecount { eq, bigger, count } => format!(
                "filecount {}",
                rating_filecount(self.not, *bigger, *eq, count)
            ),

            ItemData::Parody(v) => array("parodies", n, &save_sql_str(v)),
            ItemData::Character(v) => array("characters", n, &save_sql_str(v)),
        }
    }
}

fn array(col: &str, n: &str, v: &str) -> String {
    let subquery = match col {
        "female" | "male" | "mixed" | "other" => "tags WHERE name",
        "rest" => "temp_tags WHERE name",
        "characters" => "characters WHERE name",
        "parodies" => " parodies WHERE name",
        "cosplayers" | "artists" | "groups" => "users WHERE name",
        "languages" => "languages WHERE language",
        _ => unreachable!(),
    };
    format!("{n} ({col} @> ARRAY[(SELECT id FROM {subquery} = '{v}' LIMIT 1)])")
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
