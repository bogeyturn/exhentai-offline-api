use crate::schema_rating::ratings::dsl;
use anyhow::Result;
use diesel::prelude::*;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Insertable, Deserialize)]
#[diesel(table_name = crate::schema_rating::ratings)]
pub struct Rating {
    pub id: i32,
    pub(crate) same: Option<String>,
    pub(crate) other_lang: Option<String>,
    pub(crate) related: Option<String>,
    pub(crate) rating: i32,
}

impl Rating {
    pub fn new_err(id: i32) -> Self {
        Self {
            id,
            same: None,
            other_lang: None,
            related: None,
            rating: -404,
        }
    }
}

pub struct RatingService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> RatingService<'a> {
    pub fn check_get(&mut self, id: i32) -> Result<Rating> {
        if let Ok(v) = dsl::ratings
            .filter(dsl::id.eq(id))
            .first::<Rating>(self.conn)
        {
            return Ok(v);
        }
        let result = dsl::ratings
            .filter(
                dsl::same
                    .like(format!("%\"{}\"%", id))
                    .or(dsl::other_lang.like(format!("%\"{}\"%", id))),
            )
            .first::<Rating>(self.conn)?;
        Ok(result)
    }

    pub fn get_last(&mut self) -> i32 {
        let results: QueryResult<i32> = dsl::ratings
            .select(dsl::id)
            .order_by(dsl::id.desc())
            .first(self.conn);
        if let Ok(results) = results {
            return results;
        }
        0
    }

    //insert into table
    pub fn add(&mut self, rating: Rating) -> Result<()> {
        diesel::insert_into(crate::schema_rating::ratings::table)
            .values(rating)
            .execute(self.conn)?;
        Ok(())
    }
}
