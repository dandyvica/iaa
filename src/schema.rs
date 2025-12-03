// Define the FILES table
diesel::table! {
    files (id) {
        id -> Integer,
        path -> Text,
        name -> Text,
        r#type -> Text,
        len -> Integer,
        created -> Timestamp,
        accessed -> Timestamp,
        modified -> Timestamp,
        blake3 -> Text
    }
}

/* pub fn establish_sqlite3_connection(db: &str) -> anyhow::Result<SqliteConnection> {
    SqliteConnection::establish(db)
        .map_err(|e| anyhow!("error '{}' connecting to sqlite3 DB '{}'", e, db))
} */
