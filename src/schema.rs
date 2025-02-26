use anyhow::anyhow;
// Diesel ORM interface
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

diesel::table! {
    files (id) {
        id -> Integer,
        path -> Text,
        r#type -> Text,
    }
}

pub fn establish_sqlite3_connection(db: &str) -> anyhow::Result<SqliteConnection> {
    SqliteConnection::establish(db)
        .map_err(|e| anyhow!("error '{}' connecting to sqlite3 DB '{}'", e, db))
}