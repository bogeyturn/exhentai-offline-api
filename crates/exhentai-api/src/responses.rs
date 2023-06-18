use crate::connections::Connections;
use crate::hitomi::get_hitomi_data;
use actix_web::web::Data;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
pub struct ExHentaiResponse {
    id: i32,
    token: Option<String>,
    titles: Titles,
    categorize: Categorize,
    owner: Owner,
    pub pages: PageInfo,
    rating: Option<f32>,
    pub relations: Relations,
}

#[derive(Debug, Serialize)]
pub struct Relations {
    pub variants: Vec<i32>,
    pub languages: Vec<i32>,
    pub related: Vec<i32>,
}

impl ExHentaiResponse {
    pub(crate) async fn new(conn: &Data<Mutex<Connections>>, id: i32) -> Result<Self> {
        let (last, mut imgs, categorize, owner, variants) = {
            let mut conn = conn.lock().unwrap();
            let (mut fs, mut api, gp) = conn.get_services();
            let item = api.get(id)?;
            let items;
            if let Some(v) = item.first_gid {
                let parent = api.get(v)?;
                let mut others = api.get_related(v)?;
                others.insert(0, parent);
                items = others;
            } else {
                items = vec![item];
            }
            let last = items.last().expect("Should be something");
            let mut imgs = vec![];
            if fs.get(last.gid).is_err() {
                if let Some(mut gp) = gp {
                    if let Ok(data) = gp.get(last.gid) {
                        let i = data.image_pages.unwrap_or_else(|| "[]".to_string());
                        imgs = serde_json::from_str::<Vec<Image>>(&process_json(&i))
                            .unwrap()
                            .iter()
                            .map(|v| (v.image_thumb.to_string(), v.image_url.to_string()))
                            .collect();
                    }
                }
            }
            let owner = Owner {
                uploader: last.uploader.clone(),
                group: extract_values(items.iter().filter_map(|v| v.group.clone()).collect()),
                artist: extract_values(items.iter().filter_map(|v| v.artist.clone()).collect()),
            };
            let categorize = Categorize {
                category: last.category.clone(),
                tags: Tags {
                    male: extract_values(items.iter().filter_map(|v| v.male.clone()).collect()),
                    female: extract_values(items.iter().filter_map(|v| v.female.clone()).collect()),
                    other: extract_values(items.iter().filter_map(|v| v.other.clone()).collect()),
                    mixed: extract_values(items.iter().filter_map(|v| v.mixed.clone()).collect()),
                    temp: extract_values(items.iter().filter_map(|v| v.rest.clone()).collect()),
                },
                parody: extract_values(items.iter().filter_map(|v| v.parody.clone()).collect()),
                language: extract_values(items.iter().filter_map(|v| v.language.clone()).collect()),
                cosplayer: extract_values(
                    items.iter().filter_map(|v| v.cosplayer.clone()).collect(),
                ),
                character: extract_values(
                    items.iter().filter_map(|v| v.character.clone()).collect(),
                ),
            };
            let variants = items.iter().map(|v| v.gid).collect();
            (last.clone(), imgs, categorize, owner, variants)
        };

        let hitomi = get_hitomi_data(last.gid).await;

        let (languages, related) = match hitomi {
            None => (vec![], vec![]),
            Some((lang, related, files)) => {
                if imgs.is_empty() {
                    files.iter().for_each(|v| {
                        imgs.push(("".to_string(), v.hash.clone()));
                    });
                }

                let lang = lang
                    .iter()
                    .map(|v| v.galleryid.parse::<i32>())
                    .collect::<Result<Vec<i32>, ParseIntError>>()?;
                let related = related.iter().map(|v| *v as i32).collect::<Vec<_>>();
                (lang, related)
            }
        };

        Ok(ExHentaiResponse {
            id: last.gid,
            token: last.token.clone(),
            titles: Titles {
                title: last.title.clone(),
                jpn_title: last.title_jpn.clone(),
            },
            categorize,
            owner,
            pages: PageInfo {
                page_count: last.filecount.unwrap_or(0),
                //TODO:
                pages: imgs,
                torrent: last.torrentcount.map_or(false, |v| v > 0),
                thumb: last.thumb.clone(),
            },
            rating: last.rating,
            relations: Relations {
                variants,
                languages,
                related,
            },
        })
    }
}

fn process_json(s: &str) -> String {
    s.replace("\\'", "&#0039;")
        .replace('\'', "\"")
        .replace("&#0039;", "'")
}
#[derive(Debug, Serialize)]
struct Titles {
    title: Option<String>,
    jpn_title: Option<String>,
}

#[derive(Debug, Serialize)]
struct Categorize {
    category: Option<String>,
    tags: Tags,
    parody: Vec<String>,
    language: Vec<String>,
    cosplayer: Vec<String>,
    character: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Tags {
    male: Vec<String>,
    female: Vec<String>,
    other: Vec<String>,
    mixed: Vec<String>,
    temp: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Owner {
    uploader: Option<String>,
    group: Vec<String>,
    artist: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PageInfo {
    pub page_count: i32,
    pub pages: Vec<(String, String)>,
    torrent: bool,
    thumb: Option<String>,
}

fn extract_values(values: Vec<String>) -> Vec<String> {
    let mut hash = HashSet::new();
    for v in values {
        if let Ok(v) = serde_json::from_str::<Vec<String>>(&process_json(&v)) {
            for item in v {
                hash.insert(item);
            }
        }
    }
    hash.into_iter().collect()
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "image_number")]
    pub image_number: i64,
    #[serde(rename = "image_number_string")]
    pub image_number_string: String,
    #[serde(rename = "image_s_hash")]
    pub image_s_hash: String,
    #[serde(rename = "image_hash")]
    pub image_hash: String,
    #[serde(rename = "image_size")]
    pub image_size: i64,
    #[serde(rename = "image_thumb")]
    pub image_thumb: String,
    #[serde(rename = "image_name")]
    pub image_name: String,
}
