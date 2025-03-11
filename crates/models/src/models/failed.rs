use diesel::{prelude::Queryable, RunQueryDsl, Selectable, SqliteConnection};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema_failed::failed)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Failed {
    pub gid: i32,
    pub reason: String,
}

pub fn get_all(conn: &mut SqliteConnection) -> Vec<Failed> {
    crate::schema_failed::failed::table.load(conn).unwrap()
}
