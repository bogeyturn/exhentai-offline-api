use crate::schema::failed::dsl;
use anyhow::Result;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct Failed {
    gid: i32,
    reason: String,
}

pub struct FailedSerice<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> FailedSerice<'a> {
    pub fn get(&mut self, id: i32) -> Result<Failed> {
        let results = dsl::failed
            .filter(dsl::gid.eq(id))
            .first::<Failed>(self.conn)?;
        Ok(results)
    }
}
