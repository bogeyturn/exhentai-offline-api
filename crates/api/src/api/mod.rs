pub use crate::api::info::get_from_db;

mod info;
mod search;

#[cfg(feature = "file_stream")]
use crate::api::streamer::HitomiImages;
#[cfg(feature = "file_stream")]
use crate::streamer::byte_stream;

#[cfg(feature = "file_stream")]
mod streamer;

#[cfg(feature = "file_stream")]
#[post("/get_hitomi_images")]
pub async fn get_hitomi_images(data: Json<HitomiImages>) -> HttpResponse {
    byte_stream(data.into_inner().hashs).await
}
