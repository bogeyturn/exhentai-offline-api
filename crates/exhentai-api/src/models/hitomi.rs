use crate::schema::hitomi_gallery;
use crate::schema::hitomi_gallery::dsl;
use anyhow::Result;
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Insertable, Deserialize)]
#[diesel(table_name = hitomi_gallery)]
struct HitomiEntry {
    id: i32,
    other_id: i32,
    type_: Option<String>,
    title: Option<String>,
    jpn_title: Option<String>,
    tags: Option<String>,
    artists: Option<String>,
    groups: Option<String>,
    parodies: Option<String>,
    characters: Option<String>,
    language: Option<String>,
    language_localname: Option<String>,
    language_url: Option<String>,
    languages: Option<String>,
    related: Option<String>,
    date: Option<String>,
    files: Option<String>,
    file_count: Option<i32>,
    scene_indexes: Option<String>,
    video: Option<String>,
    videofilename: Option<String>,
    galleryurl: Option<String>,
    blocked: Option<String>,
}

pub struct HitomiService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> HitomiService<'a> {
    pub fn get(
        &mut self,
        id: i32,
    ) -> QueryResult<(i32, Option<String>, Option<String>, Option<String>)> {
        dsl::hitomi_gallery
            .filter(hitomi_gallery::id.eq(id))
            .select((
                hitomi_gallery::id,
                hitomi_gallery::languages,
                hitomi_gallery::artists,
                hitomi_gallery::groups,
            ))
            .first(self.conn)
    }
}
