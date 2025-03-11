use std::sync::Mutex;

use actix_web::{
    post,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};

use crate::connections::Connections;

#[derive(Serialize, Deserialize)]
pub struct FilterRequest {
    pub filter: Vec<String>,
    pub name: String,
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
