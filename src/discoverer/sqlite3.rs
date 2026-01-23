use std::path::Path;

use serde::Serialize;
use sqlite::{Connection, State};

use crate::{
    discoverer::{Endianness, FileSignature},
    impl_discoverer,
};

const SIGNATURE: FileSignature = FileSignature {
    header: b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00",
    footer: None,
    mime: "sqlite3",
    endianness: Endianness::BigEndian,
};

impl_discoverer!(SQLITE3, SIGNATURE);

// for some types, we'll not call the FileSignature trait's metadata()
// but a custom one, as Rust doesn't yet support trait's method specialization
#[derive(Debug, Serialize)]
struct SqliteMeta {
    // table name
    name: String,

    // number of rows
    rows: i64,

    // number of cols
    cols: usize,
}

impl SQLITE3 {
    // give the list of files in the archive
    pub fn tables(path: &Path) -> anyhow::Result<Option<serde_json::Value>> {
        let conn = Connection::open(path)?;
        let tables = table_names(&conn)?;

        Ok(serde_json::to_value(tables).ok())
    }
}

fn table_names(conn: &Connection) -> anyhow::Result<Vec<SqliteMeta>> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name")?;
    let mut tables = Vec::new();
    while let State::Row = stmt.next()? {
        let table_name: String = stmt.read(0)?;
        let row_count = row_count(conn, &table_name)?;
        let col_count = col_count(conn, &table_name)?;

        let meta = SqliteMeta {
            name: table_name,
            rows: row_count,
            cols: col_count,
        };
        tables.push(meta);
    }
    Ok(tables)
}

fn col_count(conn: &Connection, table_name: &str) -> anyhow::Result<usize> {
    let pragma = format!("PRAGMA table_info(\"{}\")", table_name.replace('"', "\"\""));
    let mut stmt = conn.prepare(&pragma)?;
    let mut count = 0usize;
    while let State::Row = stmt.next()? {
        count += 1;
    }
    Ok(count)
}

fn row_count(conn: &Connection, table_name: &str) -> anyhow::Result<i64> {
    let query = format!(
        "SELECT COUNT(*) FROM \"{}\"",
        table_name.replace('"', "\"\"")
    );
    let mut stmt = conn.prepare(&query)?;
    let count = if let State::Row = stmt.next()? {
        stmt.read(0)?
    } else {
        0
    };

    Ok(count)
}
