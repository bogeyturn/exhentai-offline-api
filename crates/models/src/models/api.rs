use std::collections::HashMap;

use diesel::{
    prelude::{Insertable, Queryable, QueryableByName},
    query_dsl::methods::FilterDsl,
    sql_query, ExpressionMethods, PgConnection, RunQueryDsl as _, Selectable,
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Characters {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::temp_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TempTags {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = crate::schema::ex_gallery)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExGallery {
    pub gid: i32,
    pub token: String,
    pub title: String,
    pub title_jpn: Option<String>,
    pub category: i32,
    pub rating: f64,
    pub languages: Vec<Option<i32>>,
    pub female: Vec<Option<i32>>,
    pub male: Vec<Option<i32>>,
    pub mixed: Vec<Option<i32>>,
    pub other: Vec<Option<i32>>,
    pub rest: Vec<Option<i32>>,
    pub artists: Vec<Option<i32>>,
    pub groups: Vec<Option<i32>>,
    pub cosplayers: Vec<Option<i32>>,
    pub uploader: Option<i32>,
    pub disowned: bool,
    pub parent_gid: Option<i32>,
    pub first_gid: Option<i32>,
    pub parodies: Vec<Option<i32>>,
    pub characters: Vec<Option<i32>>,
    pub thumb: String,
    pub filesize: i32,
    pub filecount: i32,
    pub torrentcount: i32,
    pub torrents: String,
    pub removed: Option<i32>,
    pub expunged: bool,
    pub posted: i32,
    pub dumped: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::failed)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Failed {
    pub gid: i32,
    pub reason: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::hitomi_gallery)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HitomiGallery {
    pub id: i32,
    pub other_id: i32,
    pub type_: Option<String>,
    pub title: Option<String>,
    pub jpn_title: Option<String>,
    pub tags: Option<String>,
    pub artists: Option<String>,
    pub groups: Option<String>,
    pub parodies: Option<String>,
    pub characters: Option<String>,
    pub language: Option<String>,
    pub language_localname: Option<String>,
    pub language_url: Option<String>,
    pub languages: Option<String>,
    pub related: Option<String>,
    pub date: Option<String>,
    pub files: Option<String>,
    pub file_count: Option<i32>,
    pub scene_indexes: Option<String>,
    pub video: Option<String>,
    pub videofilename: Option<String>,
    pub galleryurl: Option<String>,
    pub blocked: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::languages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Languages {
    pub id: i32,
    pub language: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::p_mixed)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PMixed {
    pub gid: i32,
    pub p: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::parodies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Parodies {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tags {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub name: String,
}

pub struct ApiService<'a> {
    pub conn: &'a mut PgConnection,
}

pub struct FailedSerice<'a> {
    pub conn: &'a mut PgConnection,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::failed_reasons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FailedReasons {
    pub id: i32,
    pub reason: String,
}

impl<'a> FailedSerice<'a> {
    pub fn get(&mut self, id: i32) -> diesel::result::QueryResult<Failed> {
        let results = crate::schema::failed::dsl::failed
            .filter(crate::schema::failed::dsl::gid.eq(id))
            .first::<Failed>(self.conn)?;
        Ok(results)
    }
    pub fn add(&mut self, items: HashMap<i32, i32>) -> diesel::result::QueryResult<()> {
        for chunk in items.into_iter().collect::<Vec<_>>().chunks(100) {
            let c = chunk
                .into_iter()
                .map(|(gid, r)| Failed {
                    gid: *gid,
                    reason: *r,
                })
                .collect::<Vec<_>>();
            diesel::insert_into(crate::schema::failed::table)
                .values(c)
                .execute(self.conn)?;
        }
        Ok(())
    }
    pub fn add_all(
        &mut self,
        names: Vec<String>,
    ) -> diesel::result::QueryResult<Vec<FailedReasons>> {
        diesel::insert_into(crate::schema::failed_reasons::table)
            .values(
                names
                    .into_iter()
                    .map(|failed_reasons| {
                        crate::schema::failed_reasons::dsl::reason.eq(failed_reasons)
                    })
                    .collect::<Vec<_>>(),
            )
            .returning((
                crate::schema::failed_reasons::dsl::id,
                crate::schema::failed_reasons::dsl::reason,
            ))
            .get_results(self.conn)
    }
}

impl<'a> ApiService<'a> {
    pub fn execute(&mut self, sql: &str) -> diesel::result::QueryResult<Vec<ExGallery>> {
        sql_query(sql).load(self.conn)
    }
    pub fn get(&mut self, id: i32) -> diesel::result::QueryResult<ExGallery> {
        let results = crate::schema::ex_gallery::dsl::ex_gallery
            .filter(crate::schema::ex_gallery::dsl::gid.eq(id))
            .first::<ExGallery>(self.conn)?;
        Ok(results)
    }
    pub fn get_related(&mut self, id: i32) -> diesel::result::QueryResult<Vec<ExGallery>> {
        let results = crate::schema::ex_gallery::dsl::ex_gallery
            .filter(crate::schema::ex_gallery::dsl::first_gid.eq(id))
            .load::<ExGallery>(self.conn)?;
        Ok(results)
    }
}
