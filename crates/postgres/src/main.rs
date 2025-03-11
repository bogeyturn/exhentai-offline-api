use postgres::start_db;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), pg_embed::pg_errors::PgEmbedError> {
    let db_path = PathBuf::from("../temp_db");
    let mut pg = start_db(db_path, 5432).await?;
    tokio::signal::ctrl_c().await.unwrap();

    pg.stop_db().await.unwrap();
    Ok(())
}
