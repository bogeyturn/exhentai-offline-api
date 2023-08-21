use crate::connections::Connections;
use crate::responses::ExHentaiResponse;
use crate::streamer::byte_stream;
use actix_web::middleware::Logger;
use actix_web::web::{Data, Json};
use actix_web::{post, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::sync::Mutex;

mod connections;
mod hitomi;
mod models;
mod responses;
mod schema;
mod schema_gp_crawl;
mod search;
mod streamer;

#[derive(Deserialize)]
struct HitomiImages {
    hashs: Vec<String>,
}

#[derive(Deserialize)]
struct EntryRequest {
    entry: i32,
}

#[derive(Deserialize)]
struct EntryRequestOption {
    entry: Option<i32>,
}

#[post("/get_hitomi_images")]
async fn get_hitomi_images(data: Json<HitomiImages>) -> HttpResponse {
    byte_stream(data.into_inner().hashs).await
}

#[post("/get_entry")]
async fn get_entry(
    data: Json<EntryRequest>,
    conn: Data<Mutex<Connections>>,
) -> Json<ExHentaiResponse> {
    Json(ExHentaiResponse::new(&conn, data.entry).await.unwrap())
}

// #[post("/get_next_entry")]
// async fn get_next_entry(
//     data: Json<EntryRequestOption>,
//     conn: Data<Mutex<Connections>>,
// ) -> Json<ExHentaiResponse> {
//     let id = data
//         .entry
//         .unwrap_or_else(|| conn.lock().unwrap().get_rating_service().get_last());
//     //get_next_entry_internal(id, conn, false).await
// }
//
// #[post("/add_rating")]
// async fn add_rating(data: Json<Rating>, conn: Data<Mutex<Connections>>) -> Json<ExHentaiResponse> {
//     let id = data.id;
//     let v = conn
//         .lock()
//         .unwrap()
//         .get_rating_service()
//         .add(data.into_inner());
//     //get_next_entry_internal(id, conn, true).await
// }

// async fn get_next_entry_internal(
//     mut id: i32,
//     conn: Data<Mutex<Connections>>,
//     add: bool,
// ) -> Json<ExHentaiResponse> {
//     loop {
//         id += 1;
//         let v = ExHentaiResponse::new(&conn, id).await;
//         match v {
//             Ok(v) => {
//                 let mut rs = conn.lock().unwrap();
//                 let checked = rs.get_rating_service().check_get(id);
//                 if v.pages.page_count == 0 || v.pages.page_count as usize != v.pages.pages.len() {
//                     let rt = if v.pages.page_count == 0 {
//                         Rating::new_err3(id)
//                     } else {
//                         Rating::new_err2(id)
//                     };
//
//                     if add {
//                         let _ = rs.get_rating_service().add(rt);
//                     }
//                 } else if v.categorize.category == Some("Game CG".to_string())
//                     || v.categorize.category == Some("Image Set".to_string())
//                     || v.categorize.category == Some("Asian Porn".to_string())
//                     || v.categorize.category == Some("Cosplay".to_string())
//                 {
//                     if add {
//                         let _ = rs.get_rating_service().add(Rating::new_game(id));
//                     }
//                 } else if let Ok(c) = checked {
//                     if c.id == id {
//                         continue;
//                     } else if add {
//                         let cto = |ve: &Vec<i32>| match ve.is_empty() {
//                             true => None,
//                             false => Some(
//                                 serde_json::to_string(
//                                     &ve.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
//                                 )
//                                 .unwrap(),
//                             ),
//                         };
//                         let rating = Rating {
//                             id,
//                             rating: c.rating,
//                         };
//                         let _ = rs.get_rating_service().add(rating);
//                     }
//                     continue;
//                 } else {
//                     return Json(v);
//                 }
//             }
//             Err(_) => {
//                 if add {
//                     let _ = conn
//                         .lock()
//                         .unwrap()
//                         .get_rating_service()
//                         .add(Rating::new_err(id));
//                 }
//             }
//         }
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(Mutex::new(Connections::new(true))))
            .service(get_hitomi_images)
            .service(get_entry)
        // .service(add_rating)
        // .service(get_next_entry)
    })
    .bind(("127.0.0.1", 8080))?
    .bind((local_ip_address::local_ip().unwrap().to_string(), 8080))?
    .run()
    .await
}
