use crate::models::api_dump::ApiDumpService;
use crate::models::failed::FailedSerice;
use crate::models::gp_crawl::GpCrawlService;
use crate::models::ratings::RatingService;
use diesel::{Connection, PgConnection, SqliteConnection};
use dotenvy::dotenv;
use std::env;

pub struct Connections {
    conn: PgConnection,
    gp_crawl_service_conn: Option<SqliteConnection>,
}

impl Connections {
    pub fn new(big: bool) -> Self {
        let conn = establish_connection_postgres();
        let gp_crawl_service_conn = match big {
            true => Some(establish_connection_sqlite()),
            false => None,
        };
        Self {
            conn,
            gp_crawl_service_conn,
        }
    }
    pub fn get_failed_service(&mut self) -> FailedSerice {
        FailedSerice {
            conn: &mut self.conn,
        }
    }

    pub fn get_crawl_service(&mut self) -> Option<GpCrawlService> {
        if let Some(conn) = &mut self.gp_crawl_service_conn {
            Some(GpCrawlService { conn })
        } else {
            None
        }
    }

    pub fn get_api_dump_service(&mut self) -> ApiDumpService {
        ApiDumpService {
            conn: &mut self.conn,
        }
    }

    pub fn get_rating_service(&mut self) -> RatingService {
        RatingService {
            conn: &mut self.conn,
        }
    }
}

pub fn establish_connection_sqlite() -> SqliteConnection {
    dotenv().ok();
    let dbpath = "DATABASE_URL_GP_CRAWL";

    let database_url = env::var(dbpath).expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_connection_postgres() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
