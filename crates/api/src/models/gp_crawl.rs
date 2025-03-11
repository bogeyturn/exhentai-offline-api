use crate::schema_gp_crawl::gallery_pages_meta::dsl;
use anyhow::Result;
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};
use serde::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct Crawled {
    gid: i32,
    token: Option<String>,
    parent: Option<String>,
    visible: Option<String>,
    favorited: Option<i32>,
    rated: Option<i32>,
    uploader_info: Option<String>,
    gp_tags: Option<String>,
    newer_versions: Option<String>,
    pub image_pages: Option<String>,
    gp_parsed: Option<String>,
    uploader_comment: Option<String>,
    comments_list: Option<String>,
    crawled: Option<i32>,
    account: Option<String>,
    hash: Option<String>,
}

pub struct GpCrawlService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> GpCrawlService<'a> {
    pub fn get(&mut self, id: i32) -> Result<Crawled> {
        let results = dsl::gallery_pages_meta
            .filter(dsl::gid.eq(id))
            .first::<Crawled>(self.conn)?;
        Ok(results)
    }
}
