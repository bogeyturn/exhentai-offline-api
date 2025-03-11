use crate::hitomi::GG;
use actix_web::HttpResponse;
use bytes::Bytes;
use futures_util::StreamExt;
use reqwest::header::REFERER;
use std::convert::Infallible;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub async fn byte_stream(items: Vec<String>) -> HttpResponse {
    let gg = GG::new().await.unwrap();
    let items = items
        .iter()
        .map(|v| gg.generate_url(v))
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap();
    let stream = start(items);
    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .streaming(stream)
}

pub fn start(input: Vec<String>) -> ReceiverStream<Result<Bytes, Infallible>> {
    let (sender, rec) = mpsc::channel(100);
    tokio::spawn(async move {
        let client = reqwest::Client::new();
        for item in input {
            let req = client
                .get(item)
                .header(REFERER, "https://hitomi.la")
                .send()
                .await
                .unwrap();
            let len = req.content_length().unwrap();
            sender
                .send(Bytes::try_from(len.to_be_bytes().to_vec()))
                .await
                .unwrap();
            let mut stream = req.bytes_stream();
            while let Some(data) = stream.next().await {
                let se = match data {
                    Ok(v) => Bytes::try_from(v.to_vec()),
                    Err(_) => panic!("Unexpected"),
                };
                sender.send(se).await.unwrap();
            }
        }
    });
    ReceiverStream::new(rec)
}
