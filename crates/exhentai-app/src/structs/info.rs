use serde::Serialize;
use serde::Deserialize;

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