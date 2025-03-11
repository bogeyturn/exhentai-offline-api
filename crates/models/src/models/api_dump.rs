use diesel::{prelude::Queryable, RunQueryDsl, Selectable, SqliteConnection};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema_api_dump::gallery)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Gallery {
    pub gid: i32,
    pub token: Option<String>,
    pub title: Option<String>,
    pub title_jpn: Option<String>,
    pub posted: Option<i32>,
    pub uploader: Option<String>,
    pub category: Option<String>,
    pub rating: Option<f32>,
    pub thumb: Option<String>,
    pub filesize: Option<i32>,
    pub filecount: Option<i32>,
    pub torrentcount: Option<i32>,
    pub torrents: Option<String>,
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
    pub current_gid: Option<i32>,
    pub current_key: Option<String>,
    pub expunged: Option<i32>,
    pub disowned: Option<i32>,
    pub removed: Option<i32>,
    pub dumped: Option<i32>,
}

pub fn all(conn: &mut SqliteConnection) -> Vec<Gallery> {
    crate::schema_api_dump::gallery::table.load(conn).unwrap()
}
