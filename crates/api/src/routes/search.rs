use std::sync::Mutex;

use actix_web::{
    post,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};

use crate::connections::Connections;

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub data: Array,
    pub order: Order,
    pub duplicate_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub desc: bool,
    pub kind: OrderKind,
}

#[derive(Serialize, Deserialize)]
pub enum OrderKind {
    Id,
    Title,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub not: bool,
    pub data: ItemData,
}

#[derive(Serialize, Deserialize)]
pub enum TagKind {
    Female,
    Male,
    Mixed,
    Other,
    Rest,
    All,
}

#[derive(Serialize, Deserialize)]
pub enum ItemData {
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

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemOrArray {
    Item(Item),
    Array(Array),
}

#[derive(Serialize, Deserialize)]
pub struct Array {
    pub or: bool,
    pub items: Vec<ItemOrArray>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) jpn_title: Option<String>,
    pub(crate) thumb: String,
}

#[post("/search")]
pub async fn search_ex(
    data: Json<SearchRequest>,
    conn: Data<Mutex<Connections>>,
) -> Json<Vec<SearchResponse>> {
    println!("{}", data.to_string());
    let v = conn
        .lock()
        .unwrap()
        .get_api_dump_service()
        .execute(&data.to_string())
        .unwrap();
    Json(
        v.into_iter()
            .map(|v| SearchResponse {
                id: v.gid,
                title: v.title,
                jpn_title: v.title_jpn,
                thumb: v.thumb,
            })
            .collect(),
    )
}
