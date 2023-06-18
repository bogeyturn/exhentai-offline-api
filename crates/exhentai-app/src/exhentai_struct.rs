use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExHentaiResponse {
    pub id: i32,
    pub token: Option<String>,
    pub titles: Titles,
    pub categorize: Categorize,
    pub owner: Owner,
    pub pages: PageInfo,
    pub rating: Option<f32>,
    pub relations: Relations,
}

#[derive(Debug, Deserialize)]
pub struct Relations {
    pub variants: Vec<i32>,
    pub languages: Vec<i32>,
    pub related: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Titles {
    pub title: Option<String>,
    pub jpn_title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Categorize {
    pub category: Option<String>,
    pub tags: Tags,
    pub parody: Vec<String>,
    pub language: Vec<String>,
    pub cosplayer: Vec<String>,
    pub character: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
    pub male: Vec<String>,
    pub female: Vec<String>,
    pub other: Vec<String>,
    pub mixed: Vec<String>,
    pub temp: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub uploader: Option<String>,
    pub group: Vec<String>,
    pub artist: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PageInfo {
    pub page_count: i32,
    pub pages: Vec<(String, String)>,
    pub torrent: bool,
    pub thumb: Option<String>,
}
