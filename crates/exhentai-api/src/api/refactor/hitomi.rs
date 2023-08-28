use anyhow::anyhow;
use anyhow::Result;
use chrono::Utc;
use reqwest::header::REFERER;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::num::ParseIntError;

#[derive(Serialize, Deserialize)]
pub struct HitmoiFile {
    pub hash: String,
    pub haswebp: i64,
    pub name: String,
    pub height: i64,
    pub width: i64,
    pub hasavif: i64,
    pub single: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct HitomiLanguage {
    pub url: String,
    pub galleryid: String,
    pub language_localname: String,
    pub name: String,
}

pub type HitomiLanguages = Vec<HitomiLanguage>;
pub type HitomiRelated = Vec<i64>;
pub type HitomiFiles = Vec<HitmoiFile>;

pub async fn get_hitomi_data(id: i32) -> Option<(HitomiLanguages, HitomiRelated, HitomiFiles)> {
    for _ in 0..3 {
        let request = request(id).await;
        if let Ok(response) = request {
            let val: Value = match serde_json::from_str(&response) {
                Ok(val) => val,
                Err(_) => return None,
            };
            let files: HitomiFiles = match serde_json::from_value(val["files"].clone()) {
                Ok(val) => val,
                Err(_) => return None,
            };
            let related: HitomiRelated = match serde_json::from_value(val["related"].clone()) {
                Ok(val) => val,
                Err(_) => return None,
            };
            let languages: HitomiLanguages = match serde_json::from_value(val["languages"].clone())
            {
                Ok(val) => val,
                Err(_) => return None,
            };
            return Some((languages, related, files));
        }
    }
    None
}

async fn request(id: i32) -> Result<String> {
    Ok(
        req(format!("https://ltn.hitomi.la/galleries/{}.js", id), None)
            .await?
            .replace("var galleryinfo = ", ""),
    )
}
