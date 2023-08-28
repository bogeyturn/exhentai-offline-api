use crate::models::api_dump::ApiDumpService;
use crate::models::failed::FailedSerice;
#[cfg(feature = "complete_offline")]
use crate::models::gp_crawl::GpCrawlService;
#[cfg(feature = "dev")]
use crate::models::hitomi::HitomiService;
#[cfg(feature = "dev")]
use crate::models::p_mixed::PMixedService;
use crate::models::ratings::RatingService;
#[cfg(feature = "complete_offline")]
use diesel::SqliteConnection;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

pub struct Connections {
    conn: PgConnection,
    #[cfg(feature = "complete_offline")]
    gp_crawl_service_conn: SqliteConnection,
}

impl Connections {
    pub fn new() -> Self {
        let conn = establish_connection_postgres();
        #[cfg(feature = "complete_offline")]
        let gp_crawl_service_conn = establish_connection_sqlite();
        Self {
            conn,
            #[cfg(feature = "complete_offline")]
            gp_crawl_service_conn,
        }
    }
    pub fn get_failed_service(&mut self) -> FailedSerice {
        FailedSerice {
            conn: &mut self.conn,
        }
    }

    #[cfg(feature = "complete_offline")]
    pub fn get_crawl_service(&mut self) -> GpCrawlService {
        GpCrawlService {
            conn: &mut self.gp_crawl_service_conn,
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

    #[cfg(feature = "dev")]
    pub fn get_hitomi_service(&mut self) -> HitomiService {
        HitomiService {
            conn: &mut self.conn,
        }
    }

    #[cfg(feature = "dev")]
    pub fn get_p_mixed_service(&mut self) -> PMixedService {
        PMixedService {
            conn: &mut self.conn,
        }
    }
}

#[cfg(feature = "complete_offline")]
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
