use crate::schema::ratings;
use crate::schema::ratings::dsl;
use anyhow::Result;
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Insertable, Deserialize)]
#[diesel(table_name = ratings)]
pub struct Rating {
    pub id: i32,
    pub(crate) rating: i32,
}

impl Rating {
    pub fn new_err(id: i32) -> Self {
        Self { id, rating: -404 }
    }

    pub fn new_err2(id: i32) -> Self {
        Self { id, rating: -405 }
    }
    pub fn new_err3(id: i32) -> Self {
        Self { id, rating: -405 }
    }

    pub fn new_game(id: i32) -> Self {
        Self { id, rating: -1 }
    }
}

pub struct RatingService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> RatingService<'a> {
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

    pub fn add(&mut self, rating: Rating) -> Result<()> {
        diesel::insert_into(ratings::table)
            .values(rating)
            .execute(self.conn)?;
        Ok(())
    }

    pub fn get(&mut self, id: i32) -> QueryResult<i32> {
        dsl::ratings
            .select(dsl::rating)
            .filter(dsl::id.eq(id))
            .first(self.conn)
    }
}
