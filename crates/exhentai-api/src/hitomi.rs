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

pub struct GG {
    filtered: Vec<u32>,
    timestamp: u64,
    pub created: u64,
    o: u8,
    oa: u8,
}

async fn req(url: String, referer: Option<&str>) -> Result<String> {
    for _ in 0..3 {
        let client = Client::new();
        let mut req = client.get(&url);
        if let Some(r) = referer {
            req = req.header(REFERER, r);
        }

        let v = req.send().await;
        match v {
            Ok(v) => return Ok(v.text().await?),
            Err(_) => continue,
        }
    }
    Err(anyhow!("Failed to get page"))
}

fn s(h: &str) -> Result<String> {
    let re = regex::Regex::new(r"(..)(.)$")?;
    let captures = re.captures(h).ok_or_else(|| anyhow!("Failed to capture"))?;
    let m = captures
        .get(0)
        .ok_or_else(|| anyhow!("Failed to capture"))?
        .as_str();
    let result = format!(
        "{}",
        i32::from_str_radix(&format!("{}{}", &m[2..], &m[..2]), 16)?
    );
    Ok(result)
}

impl GG {
    pub fn generate_url(&self, hash: &str) -> Result<String> {
        let ext = "webp";
        Ok(format!(
            "https://{}.hitomi.la/{}/{}/{}/{}.{}",
            self.subdomain_from_url(hash, Some('a')),
            ext,
            self.timestamp,
            s(hash)?,
            hash,
            ext
        ))
    }

    fn m(&self, item: u32) -> u8 {
        if self.filtered.contains(&item) {
            return self.oa;
        }
        self.o
    }

    fn subdomain_from_url(&self, hash: &str, base: Option<char>) -> String {
        let mut retval = String::from("b");

        if let Some(b) = base {
            retval = b.to_string();
        }

        let m2 = &hash[hash.len() - 1..];
        let m1 = &hash[hash.len() - 3..hash.len() - 1];

        if let Ok(g) = i32::from_str_radix(&format!("{}{}", m2, m1), 16) {
            retval = format!("{}{}", (97 + self.m(g as u32)) as char, retval);
        }

        retval
    }

    pub async fn new() -> Result<Self> {
        let now = Utc::now().timestamp();
        let site = req(format!("https://ltn.hitomi.la/gg.js?_={}", now), None).await?;
        let splitted: Vec<&str> = site.split('\n').collect();
        let selected = splitted
            .iter()
            .filter(|n| n.contains("case"))
            .collect::<Vec<&&str>>();
        let clean: Vec<u32> = selected
            .iter()
            .map(|f| {
                f.chars()
                    .filter(|i| i.is_numeric())
                    .collect::<String>()
                    .parse::<u32>()
            })
            .collect::<Result<Vec<_>, ParseIntError>>()?;
        let o = splitted
            .iter()
            .filter(|n| n.contains("var o ="))
            .collect::<Vec<_>>()[0]
            .replace("var o = ", "")
            .replace(';', "")
            .parse::<u8>()?;

        let oa = splitted
            .iter()
            .filter(|n| n.contains("o ="))
            .collect::<Vec<_>>()[1]
            .replace("o = ", "")
            .replace("break", "")
            .replace([' ', ';'], "")
            .parse::<u8>()?;
        let timestamp = splitted
            .iter()
            .filter(|n| n.contains("b: '"))
            .collect::<Vec<_>>()[0]
            .replace("b: '", "")
            .replace("/'", "")
            .parse::<u64>()?;
        Ok(GG {
            timestamp,
            filtered: clean,
            created: now as u64,
            o,
            oa,
        })
    }
}
