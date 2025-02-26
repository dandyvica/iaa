// main information for a path to insert into the database

use diesel::Insertable;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// use crate::schema::files;

// Define the FILES table
diesel::table! {
    files (id) {
        id -> Integer,
        path -> Text,
        name -> Text,
        r#type -> Text
    }
}

#[derive(Debug, Default, Insertable)]
#[diesel(table_name = files)]
pub struct FileInfo {
    // full path
    pub path: String,

    // name without the path
    pub name: String,

    // extension
    pub r#type: &'static str


}