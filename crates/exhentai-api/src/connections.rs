use crate::models::api_dump::ApiDumpService;
use crate::models::failed::FailedSerice;
use crate::models::gp_crawl::GpCrawlService;
use crate::models::ratings::RatingService;
use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use std::env;

pub struct Connections {
    api_dump_conn: SqliteConnection,
    failed_conn: SqliteConnection,
    gp_crawl_service_conn: Option<SqliteConnection>,
    rating_conn: SqliteConnection,
}

impl Connections {
    pub fn new(big: bool) -> Self {
        let api_dump_conn = establish_connection(Database::ApiDump);
        let failed_conn = establish_connection(Database::Failed);
        let rating_conn = establish_connection(Database::Rating);
        let gp_crawl_service_conn = match big {
            true => Some(establish_connection(Database::GpCrawled)),
            false => None,
        };
        Self {
            api_dump_conn,
            failed_conn,
            gp_crawl_service_conn,
            rating_conn,
        }
    }
    pub fn get_services(&mut self) -> (FailedSerice, ApiDumpService, Option<GpCrawlService>) {
        let api_dump_service = ApiDumpService {
            conn: &mut self.api_dump_conn,
        };
        let failed_service = FailedSerice {
            conn: &mut self.failed_conn,
        };

        let mut gp_crawl_service = None;
        if let Some(conn) = &mut self.gp_crawl_service_conn {
            gp_crawl_service = Some(GpCrawlService { conn });
        }
        (failed_service, api_dump_service, gp_crawl_service)
    }

    pub fn get_rating_service(&mut self) -> RatingService {
        RatingService {
            conn: &mut self.rating_conn,
        }
    }
}

pub fn establish_connection(db: Database) -> SqliteConnection {
    dotenv().ok();
    let dbpath = format!(
        "DATABASE_URL_{}",
        match db {
            Database::Failed => "FAILED",
            Database::ApiDump => "API_DUMP",
            Database::GpCrawled => "GP_CRAWL",
            Database::Rating => "RATING",
        }
    );

    let database_url = env::var(dbpath).expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub enum Database {
    Failed,
    ApiDump,
    GpCrawled,
    Rating,
}
