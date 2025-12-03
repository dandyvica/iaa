// main information for a path to insert into the database

use std::ffi::OsString;
use std::time::SystemTime;

use diesel::prelude::*;
use diesel::sql_types::Integer;
use diesel::Insertable;
use chrono::{NaiveDate, NaiveDateTime};

use crate::schema::files;

#[derive(Debug, Default, Insertable)]
#[diesel(table_name = files)]
pub struct FileInfo {
    // full path: on UNIX platforms, could be represented as a UTF-8 string
    pub path: String,

    // on Windows, this is UTF-16 
    #[diesel(skip_insertion)]
    pub winpath: OsString,

    // name without the path
    pub name: String,

    // on Windows, this is UTF-16 
    #[diesel(skip_insertion)]
    pub winname: OsString,    

    // extension
    pub r#type: &'static str,

    // file length
    pub len: i32,

    // creation time
    pub created: NaiveDateTime,

    // last accessed time
    pub accessed: NaiveDateTime,

    // last modified time
    pub modified: NaiveDateTime,

    // blake3 hash
    pub blake3: String
}
