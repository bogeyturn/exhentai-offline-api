use crate::connections::Connections;
use crate::models::api_dump::ApiDump;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExHentaiResponse {
    id: i32,
    token: Option<String>,
    titles: Titles,
    categorize: Categorize,
    owner: Owner,
    pages: PageInfo,
    rating: Option<f64>,
    my_rating: Option<i32>,
    relations: Relations,
    deleted: bool,
    hitomi_backup: bool,
}

fn process_json(s: &str) -> String {
    s.replace("\\'", "&#0039;")
        .replace('\'', "\"")
        .replace("&#0039;", "'")
}

fn str_to_vec(v: Option<String>) -> Vec<String> {
    match v {
        Some(v) => serde_json::from_str::<Vec<String>>(&process_json(&v)).unwrap(),
        None => vec![],
    }
}

struct HitomiData {
    related: Vec<i32>,
    hashs: Vec<String>,
}

pub fn get_from_db(conn: &mut Connections, id: i32, hitomi_hashs: bool) -> ExHentaiResponse {
    //TODO: add exhentai urls
    //TODO: make hitomi_data_fetchable from web
    let data = conn.get_api_dump_service().get(id).unwrap();
    let items = match hitomi_hashs {
        true => conn.get_hitomi_service().get_hashs_and_related(id).ok(),
        false => conn
            .get_hitomi_service()
            .get_related(id)
            .ok()
            .map(|v| (Some(String::new()), v)),
    };
    let hitomi = if let Some((hashs, rel)) = items {
        let hashs = if let Some(hashs) = hashs {
            let items: Vec<Value> = serde_json::from_str(&hashs).unwrap();
            items.iter().map(|v| v["hash"].to_string()).collect()
        } else {
            vec![]
        };
        let related = if let Some(rel) = rel {
            serde_json::from_str(&rel).unwrap()
        } else {
            vec![]
        };
        Some(HitomiData { related, hashs })
    } else {
        None
    };
    let same = conn.get_p_mixed_service().get(id);
    let deleted = conn.get_failed_service().get(id).is_ok();
    let my_rating = conn.get_rating_service().get(id).ok();
    (data, hitomi, same, deleted, my_rating).into()
}

impl From<(ApiDump, Option<HitomiData>, Vec<i32>, bool, Option<i32>)> for ExHentaiResponse {
    fn from(
        (value, hitomi, same, deleted, my_rating): (
            ApiDump,
            Option<HitomiData>,
            Vec<i32>,
            bool,
            Option<i32>,
        ),
    ) -> Self {
        let hitomi_backup = hitomi.is_some();
        let (related, hashs) = match hitomi {
            None => (vec![], vec![]),
            Some(v) => (v.related, v.hashs),
        };
        Self {
            id: value.gid,
            token: value.token,
            titles: Titles {
                title: value.title,
                jpn_title: value.title_jpn,
            },
            categorize: Categorize {
                category: value.category,
                tags: Tags {
                    male: str_to_vec(value.male),
                    female: str_to_vec(value.female),
                    other: str_to_vec(value.other),
                    mixed: str_to_vec(value.mixed),
                    temp: str_to_vec(value.rest),
                },
                parody: str_to_vec(value.parody),
                language: str_to_vec(value.language),
                cosplayer: str_to_vec(value.cosplayer),
                character: str_to_vec(value.character),
            },
            owner: Owner {
                uploader: value.uploader,
                groups: str_to_vec(value.group_name),
                artists: str_to_vec(value.artist),
            },
            pages: PageInfo {
                page_count: value.filecount.unwrap_or(0),
                pages: vec![],
                hitomi_hashs: hashs,
                thumb: value.thumb,
            },
            rating: value.rating,
            my_rating,
            relations: Relations {
                variants: same,
                related,
            },
            deleted,
            hitomi_backup,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relations {
    variants: Vec<i32>,
    related: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Titles {
    title: Option<String>,
    jpn_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Categorize {
    category: Option<String>,
    tags: Tags,
    parody: Vec<String>,
    language: Vec<String>,
    cosplayer: Vec<String>,
    character: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    male: Vec<String>,
    female: Vec<String>,
    other: Vec<String>,
    mixed: Vec<String>,
    temp: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    uploader: Option<String>,
    groups: Vec<String>,
    artists: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    page_count: i32,
    pages: Vec<String>,
    hitomi_hashs: Vec<String>,
    thumb: Option<String>,
}
