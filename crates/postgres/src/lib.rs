use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_fetch::{PgFetchSettings, PG_V15};
use pg_embed::postgres::{PgEmbed, PgSettings};
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;

pub async fn start_db(
    db_path: PathBuf,
    port: u16,
) -> Result<PgEmbed, pg_embed::pg_errors::PgEmbedError> {
    // Ensure the directory exists before setting up
    if !db_path.exists() {
        fs::create_dir_all(&db_path).await.unwrap();
    }

    let pg_settings = PgSettings {
        database_dir: db_path,
        port,
        user: "postgres".to_string(),
        password: "password".to_string(),
        auth_method: PgAuthMethod::Plain,
        persistent: true,
        timeout: Some(Duration::from_secs(15)),
        migration_dir: None,
    };

    let fetch_settings = PgFetchSettings {
        version: PG_V15,
        ..Default::default()
    };

    let mut pg = PgEmbed::new(pg_settings, fetch_settings).await?;

    if !pg.database_exists("postgres").await.unwrap_or(false) {
        pg.setup().await?;
    }

    pg.start_db().await?;

    println!("PostgreSQL is running at: {}", pg.db_uri);
    Ok(pg)
}
