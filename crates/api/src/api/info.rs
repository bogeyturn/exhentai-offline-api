use crate::routes::info::{Categorize, Owner, PageInfo, Relations, Tags, Titles};
use crate::{connections::Connections, routes::info::ExHentaiResponse};
use models::models::api::ExGallery;
use models::Category;
use serde_json::Value;

fn process_json(s: &str) -> String {
    s.replace("\\'", "&#0039;")
        .replace('\'', "\"")
        .replace("&#0039;", "'")
}

fn lookup_ids(v: Vec<Option<i32>>) -> Vec<String> {
    todo!()
}

fn lookup_id(v: Option<i32>) -> Option<String> {
    todo!()
}

pub fn get_from_db(conn: &mut Connections, id: i32, hitomi_hashs: bool) -> ExHentaiResponse {
    //TODO: add exhentai urls
    //TODO: make hitomi_data_fetchable from web
    let data = conn.get_api_dump_service().get(id).unwrap();
    #[cfg(feature = "hitomi_offline")]
    let items = match hitomi_hashs {
        true => conn.get_hitomi_service().get_hashs_and_related(id).ok(),
        false => conn
            .get_hitomi_service()
            .get_related(id)
            .ok()
            .map(|v| (Some(String::new()), v)),
    };
    #[cfg(not(feature = "hitomi_offline"))]
    let items: Option<(Option<String>, Option<String>)> = None;
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
    (data, hitomi, same, deleted).into()
}

impl From<(ExGallery, Option<HitomiData>, Vec<i32>, bool)> for ExHentaiResponse {
    fn from(
        (value, hitomi, same, deleted): (ExGallery, Option<HitomiData>, Vec<i32>, bool),
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
                category: Category::from(value.category),
                tags: Tags {
                    male: lookup_ids(value.male),
                    female: lookup_ids(value.female),
                    other: lookup_ids(value.other),
                    mixed: lookup_ids(value.mixed),
                    temp: lookup_ids(value.rest),
                },
                parody: lookup_ids(value.parodies),
                language: lookup_ids(value.languages),
                cosplayer: lookup_ids(value.cosplayers),
                character: lookup_ids(value.characters),
            },
            owner: Owner {
                uploader: lookup_id(value.uploader),
                groups: lookup_ids(value.groups),
                artists: lookup_ids(value.artists),
            },
            pages: PageInfo {
                page_count: value.filecount,
                pages: vec![],
                hitomi_hashs: hashs,
                thumb: value.thumb,
            },
            rating: value.rating,
            relations: Relations {
                variants: same,
                related,
            },
            deleted,
            hitomi_backup,
        }
    }
}

struct HitomiData {
    related: Vec<i32>,
    hashs: Vec<String>,
}
