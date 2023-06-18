use crate::schema_api_dump::gallery::dsl;
use anyhow::Result;
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};
use serde::Serialize;

#[derive(Debug, Serialize, Queryable, Clone)]
pub struct ApiDump {
    pub gid: i32,
    pub title: Option<String>,
    pub title_jpn: Option<String>,
    pub category: Option<String>,
    pub uploader: Option<String>,
    pub posted: Option<i32>,
    pub thumb: Option<String>,
    pub filesize: Option<i32>,
    pub filecount: Option<i32>,
    pub expunged: Option<i32>,
    pub torrentcount: Option<i32>,
    pub torrents: Option<String>,
    pub token: Option<String>,
    pub rating: Option<f32>,
    pub artist: Option<String>,
    pub group: Option<String>,
    pub parody: Option<String>,
    pub character: Option<String>,
    pub female: Option<String>,
    pub male: Option<String>,
    pub language: Option<String>,
    pub mixed: Option<String>,
    pub other: Option<String>,
    pub cosplayer: Option<String>,
    pub rest: Option<String>,
    pub parent_gid: Option<i32>,
    pub parent_key: Option<String>,
    pub first_gid: Option<i32>,
    pub first_key: Option<String>,
    pub disowned: Option<i32>,
    pub removed: Option<i32>,
    pub dumped: Option<i32>,
}

pub struct ApiDumpService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> ApiDumpService<'a> {
    pub fn get(&mut self, id: i32) -> Result<ApiDump> {
        let results = dsl::gallery
            .filter(dsl::gid.eq(id))
            .first::<ApiDump>(self.conn)?;
        Ok(results)
    }
    pub fn get_related(&mut self, id: i32) -> Result<Vec<ApiDump>> {
        let results = dsl::gallery
            .filter(dsl::first_gid.eq(id))
            .load::<ApiDump>(self.conn)?;
        Ok(results)
    }
}
