use crate::api::info::{get_from_db, ExHentaiResponse};
use crate::api::search::{FilterRequest, SearchRequest};
#[cfg(feature = "file_stream")]
use crate::api::streamer::HitomiImages;
use crate::connections::Connections;
use crate::models::api_dump::ApiDump;
#[cfg(feature = "file_stream")]
use crate::streamer::byte_stream;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use std::sync::Mutex;

mod info;
mod search;
#[cfg(feature = "file_stream")]
mod streamer;

#[post("/search")]
pub async fn search_ex(
    data: Json<SearchRequest>,
    conn: Data<Mutex<Connections>>,
) -> Json<Vec<ApiDump>> {
    Json(
        conn.lock()
            .unwrap()
            .get_api_dump_service()
            .execute(&data.to_string())
            .unwrap(),
    )
}

#[post("/create_search_filter")]
pub async fn create_filter(data: Json<FilterRequest>, conn: Data<Mutex<Connections>>) -> Json<()> {
    data.generate_materialized_view(&mut conn.lock().unwrap());
    Json(())
}

#[post("/update_search_filter")]
pub async fn update_filter(data: Json<String>, conn: Data<Mutex<Connections>>) -> Json<()> {
    conn.lock()
        .unwrap()
        .get_api_dump_service()
        .execute(&format!("REFRESH MATERIALIZED VIEW {}_ex_gallery;", data.0))
        .unwrap();
    Json(())
}

#[cfg(feature = "file_stream")]
#[post("/get_hitomi_images")]
pub async fn get_hitomi_images(data: Json<HitomiImages>) -> HttpResponse {
    byte_stream(data.into_inner().hashs).await
}

#[post("/info")]
pub async fn get_info(data: Json<i32>, conn: Data<Mutex<Connections>>) -> Json<ExHentaiResponse> {
    Json(get_from_db(&mut conn.lock().unwrap(), data.0, true))
}
