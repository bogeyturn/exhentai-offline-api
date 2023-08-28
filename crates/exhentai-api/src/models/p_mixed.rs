use crate::schema::p_mixed;
use anyhow::Result;
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
    Selectable,
};
use serde::{Deserialize, Serialize};

pub struct PMixedService<'a> {
    pub conn: &'a mut PgConnection,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::p_mixed)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertThis {
    pub gid: i32,
    pub p: i32,
}

impl<'a> PMixedService<'a> {
    pub fn insert(&mut self, items: &[InsertThis]) {
        diesel::insert_into(p_mixed::table)
            .values(items)
            .execute(self.conn)
            .expect("TODO: panic message");
    }

    pub fn get(&mut self, id: i32) -> Vec<i32> {
        let p: Option<i32> = p_mixed::table
            .select(p_mixed::p)
            .filter(p_mixed::gid.eq(id))
            .first(self.conn)
            .unwrap();
        p_mixed::table
            .select(p_mixed::gid)
            .filter(p_mixed::p.eq(p.unwrap()))
            .load(self.conn)
            .unwrap()
    }
}
