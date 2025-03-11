use std::collections::{HashMap, HashSet};

use diesel::{Connection as _, ExpressionMethods, PgConnection, SqliteConnection};
use models::api::{ExGallery, FailedSerice, TempTags};
use models::api::{Languages, Parodies};
use serde::{Deserialize, Serialize};

pub mod models;
mod schema;
mod schema_api_dump;
mod schema_failed;
mod schema_gp_crawl;

use crate::models::api::Characters;
use crate::models::api::Tags;
use crate::models::api::Users;
use diesel::RunQueryDsl;

#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    ArtistCG = 0,
    AsianPorn = 1,
    Cosplay = 2,
    Doujinshi = 3,
    GameCG = 4,
    ImageSet = 5,
    Manga = 6,
    Misc = 7,
    NonH = 8,
    Western = 9,
    Private = 10,
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "Artist CG" => Category::ArtistCG,
            "Asian Porn" => Category::AsianPorn,
            "Cosplay" => Category::Cosplay,
            "Doujinshi" => Category::Doujinshi,
            "Game CG" => Category::GameCG,
            "Image Set" => Category::ImageSet,
            "Manga" => Category::Manga,
            "Misc" => Category::Misc,
            "Non-H" => Category::NonH,
            "Western" => Category::Western,
            "private" => Category::Private,
            v => unreachable!("{}", v),
        }
    }
}

impl From<i32> for Category {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::ArtistCG,
            1 => Self::AsianPorn,
            2 => Self::Cosplay,
            3 => Self::Doujinshi,
            4 => Self::GameCG,
            5 => Self::ImageSet,
            6 => Self::Manga,
            7 => Self::Misc,
            8 => Self::NonH,
            9 => Self::Western,
            10 => Self::Private,
            _ => panic!("unexpected id"),
        }
    }
}

pub fn add_entry(conn: &mut PgConnection, item: ExGallery) {
    diesel::insert_into(schema::ex_gallery::dsl::ex_gallery)
        .values(item)
        .execute(conn)
        .unwrap();
}

pub fn add_failed(conn: &mut PgConnection, items: HashMap<i32, String>) -> HashMap<i32, i32> {
    let values = items
        .values()
        .collect::<HashSet<_>>()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    let ids = FailedSerice { conn }
        .add_all(values)
        .unwrap()
        .into_iter()
        .map(|v| (v.reason, v.id))
        .collect::<HashMap<_, _>>();
    items
        .into_iter()
        .map(|v| (v.0, *ids.get(&v.1).unwrap()))
        .collect()
}

pub fn add_users(conn: &mut PgConnection, users: HashSet<String>) -> Vec<Users> {
    for users in users.into_iter().collect::<Vec<_>>().chunks(100) {
        diesel::insert_into(schema::users::dsl::users)
            .values(
                users
                    .into_iter()
                    .map(|v| schema::users::dsl::name.eq(v))
                    .collect::<Vec<_>>(),
            )
            .execute(conn)
            .unwrap();
    }
    schema::users::dsl::users.load(conn).unwrap()
}

pub fn add_temp_tags(conn: &mut PgConnection, temp_tags: HashSet<String>) -> Vec<TempTags> {
    let temp_tags = temp_tags
        .into_iter()
        .map(|v| v.replace("_", " "))
        .collect::<HashSet<_>>();
    for temp_tags in temp_tags.into_iter().collect::<Vec<_>>().chunks(100) {
        diesel::insert_into(schema::temp_tags::dsl::temp_tags)
            .values(
                temp_tags
                    .into_iter()
                    .map(|v| schema::temp_tags::dsl::name.eq(v))
                    .collect::<Vec<_>>(),
            )
            .execute(conn)
            .unwrap();
    }
    schema::temp_tags::dsl::temp_tags.load(conn).unwrap()
}

pub fn add_tags(conn: &mut PgConnection, tags: HashSet<String>) -> Vec<Tags> {
    let tags = tags
        .into_iter()
        .map(|v| v.replace("_", " "))
        .collect::<HashSet<_>>();
    for tags in tags.into_iter().collect::<Vec<_>>().chunks(100) {
        diesel::insert_into(schema::tags::dsl::tags)
            .values(
                tags.into_iter()
                    .map(|v| schema::tags::dsl::name.eq(v))
                    .collect::<Vec<_>>(),
            )
            .execute(conn)
            .unwrap();
    }
    schema::tags::dsl::tags.load(conn).unwrap()
}

pub fn add_parodies(conn: &mut PgConnection, parodies: HashSet<String>) -> Vec<Parodies> {
    for parodies in parodies.into_iter().collect::<Vec<_>>().chunks(100) {
        diesel::insert_into(schema::parodies::dsl::parodies)
            .values(
                parodies
                    .into_iter()
                    .map(|v| schema::parodies::dsl::name.eq(v))
                    .collect::<Vec<_>>(),
            )
            .execute(conn)
            .unwrap();
    }
    schema::parodies::dsl::parodies.load(conn).unwrap()
}
pub fn add_characters(conn: &mut PgConnection, characters: HashSet<String>) -> Vec<Characters> {
    for characters in characters.into_iter().collect::<Vec<_>>().chunks(100) {
        diesel::insert_into(schema::characters::dsl::characters)
            .values(
                characters
                    .into_iter()
                    .map(|v| schema::characters::dsl::name.eq(v))
                    .collect::<Vec<_>>(),
            )
            .execute(conn)
            .unwrap();
    }
    schema::characters::dsl::characters.load(conn).unwrap()
}

pub fn add_languages(conn: &mut PgConnection, languages: HashSet<String>) -> Vec<Languages> {
    for languages in languages.into_iter().collect::<Vec<_>>().chunks(100) {
        diesel::insert_into(schema::languages::dsl::languages)
            .values(
                languages
                    .into_iter()
                    .map(|v| schema::languages::dsl::language.eq(v))
                    .collect::<Vec<_>>(),
            )
            .execute(conn)
            .unwrap();
    }
    schema::languages::dsl::languages.load(conn).unwrap()
}

pub fn establish_connection_sqlite(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_connection_pg(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
