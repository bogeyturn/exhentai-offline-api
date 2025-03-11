use std::sync::Mutex;

use actix_web::{
    post,
    web::{Data, Json},
};
use models::Category;
use serde::{Deserialize, Serialize};

use crate::{api::get_from_db, connections::Connections};

use super::search::OrderKind;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExHentaiResponse {
    pub id: i32,
    pub token: String,
    pub titles: Titles,
    pub categorize: Categorize,
    pub owner: Owner,
    pub pages: PageInfo,
    pub rating: f64,
    pub relations: Relations,
    pub deleted: bool,
    pub hitomi_backup: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relations {
    pub variants: Vec<i32>,
    pub related: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Titles {
    pub title: String,
    pub jpn_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Categorize {
    pub category: Category,
    pub tags: Tags,
    pub parody: Vec<String>,
    pub language: Vec<String>,
    pub cosplayer: Vec<String>,
    pub character: Vec<String>,
}

impl ToString for OrderKind {
    fn to_string(&self) -> String {
        match self {
            OrderKind::Id => "gid",
            OrderKind::Title => "title",
        }
        .to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub male: Vec<String>,
    pub female: Vec<String>,
    pub other: Vec<String>,
    pub mixed: Vec<String>,
    pub temp: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub uploader: Option<String>,
    pub groups: Vec<String>,
    pub artists: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    pub page_count: i32,
    pub pages: Vec<String>,
    pub hitomi_hashs: Vec<String>,
    pub thumb: String,
}

#[post("/info")]
pub async fn get_info(data: Json<i32>, conn: Data<Mutex<Connections>>) -> Json<ExHentaiResponse> {
    Json(get_from_db(&mut conn.lock().unwrap(), data.0, true))
}
