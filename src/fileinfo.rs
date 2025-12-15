// main information for a path to insert into the database

use std::ffi::OsString;
use std::fs::FileType;
use std::time::{SystemTime, UNIX_EPOCH};

use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::Insertable;

use crate::schema::{artefact, run_history};

const FT_FILE: &'static str = "F";
const FT_DIRECTORY: &'static str = "D";
const FT_SYMLINK: &'static str = "S";
const FT_UNKNOWN: &'static str = "U";

#[derive(Debug, Default, Clone, Copy, PartialEq, AsExpression)]
#[diesel(sql_type = Text)]
pub enum ForensicsFileType {
    File,
    Directory,
    Symlink,
    #[default]
    Unknown,
}

impl From<&FileType> for ForensicsFileType {
    fn from(ft: &FileType) -> Self {
        if ft.is_file() {
            Self::File
        } else if ft.is_dir() {
            Self::Directory
        } else if ft.is_symlink() {
            Self::Symlink
        } else {
            Self::Unknown
        }
    }
}

// Implement ToSql for ForensicsFileType which is a custom type
impl ToSql<Text, Pg> for ForensicsFileType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            Self::File => ToSql::<Text, Pg>::to_sql(FT_FILE, out),
            Self::Directory => ToSql::<Text, Pg>::to_sql(FT_DIRECTORY, out),
            Self::Symlink => ToSql::<Text, Pg>::to_sql(FT_SYMLINK, out),
            Self::Unknown => ToSql::<Text, Pg>::to_sql(FT_UNKNOWN, out),
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = artefact)]
pub struct FileInfo {
    // full path: on UNIX platforms, could be represented as a UTF-8 string
    pub path: String,

    // on Windows, this is UTF-16
    #[diesel(skip_insertion)]
    pub winpath: OsString,

    // name without the path
    pub name: String,

    // file extension
    pub ext: String,

    // on Windows, this is UTF-16
    #[diesel(skip_insertion)]
    pub winname: OsString,

    // extension
    pub r#type: ForensicsFileType,

    // file length
    pub len: i64,

    // creation time (only for Unices, Windows not possible)
    pub created: Option<SystemTime>,

    // last accessed time
    pub accessed: SystemTime,

    // last modified time
    pub modified: SystemTime,

    // sha256 hash
    pub sha256: String,

    // blake3 hash
    pub blake3: String,

    // Shannon entropy
    pub entropy: Option<f32>,

    // try to have kind of mime type from magic numbers
    pub mime: Option<&'static str>,

    // optional metadata for the file
    pub metadata: Option<serde_json::Value>,
}

// has to implement default manually cause SystemTime has no default
impl Default for FileInfo {
    fn default() -> Self {
        Self {
            path: String::new(),
            winpath: OsString::new(),
            name: String::new(),
            ext: String::new(),
            winname: OsString::new(),
            r#type: ForensicsFileType::default(),
            len: 0,
            created: None,
            accessed: UNIX_EPOCH,
            modified: UNIX_EPOCH,
            sha256: String::new(),
            blake3: String::new(),
            entropy: None,
            mime: None,
            metadata: None,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = run_history)]
pub struct RunHistory {
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub nb_files: i64,
    pub args: String,
    pub tags: String,
}

impl Default for RunHistory {
    fn default() -> Self {
        Self {
            start_time: SystemTime::now(),
            end_time: SystemTime::now(),
            nb_files: 0,
            args: String::new(),
            tags: String::new(),
        }
    }
}
