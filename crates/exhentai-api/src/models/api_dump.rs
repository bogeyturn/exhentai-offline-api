use crate::schema::ex_gallery::dsl;
use anyhow::Result;
use diesel::{
    sql_query, ExpressionMethods, PgConnection, QueryDsl, QueryResult, Queryable, QueryableByName,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, QueryableByName, Queryable, Clone)]
#[diesel(table_name = crate::schema::ex_gallery)]
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
    pub rating: Option<f64>,
    pub artist: Option<String>,
    pub group_name: Option<String>,
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
    pub conn: &'a mut PgConnection,
}

impl<'a> ApiDumpService<'a> {
    pub fn execute(&mut self, sql: &str) -> QueryResult<Vec<ApiDump>> {
        sql_query(sql).load(self.conn)
    }
    pub fn all_ids(
        &mut self,
    ) -> Vec<(
        i32,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        Option<String>,
    )> {
        dsl::ex_gallery
            .select((
                dsl::gid,
                dsl::first_gid,
                dsl::parent_gid,
                dsl::filecount,
                dsl::artist,
                dsl::group_name,
            ))
            .load(self.conn)
            .unwrap()
    }
    pub fn get(&mut self, id: i32) -> Result<ApiDump> {
        let results = dsl::ex_gallery
            .filter(dsl::gid.eq(id))
            .first::<ApiDump>(self.conn)?;
        Ok(results)
    }
    pub fn get_related(&mut self, id: i32) -> Result<Vec<ApiDump>> {
        let results = dsl::ex_gallery
            .filter(dsl::first_gid.eq(id))
            .load::<ApiDump>(self.conn)?;
        Ok(results)
    }
}
