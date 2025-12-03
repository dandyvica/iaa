// module for main worker
use std::{
    fs::{self, File, FileType, Metadata, read}, io::{BufReader, Result}, path::{Path, PathBuf}, sync::Arc, thread::{self, JoinHandle}, time::{SystemTime, UNIX_EPOCH}
};

use chrono::{DateTime, NaiveDateTime, Utc};
use crossbeam_channel as channel;
use log::trace;
use walkdir::{DirEntry, DirEntryExt};
use diesel::{PgConnection, RunQueryDsl, r2d2::{ConnectionManager, PooledConnection}};

use crate::{fileinfo::FileInfo, hash::Hashes};
use crate::schema::files::dsl::files;

pub type ChanReceiver = channel::Receiver<DirEntry>;

pub fn thread_pool(n: usize, rec: ChanReceiver, pool: &Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>) -> Vec<JoinHandle<()>> {
    // create n threads for handle workers
    let mut handles = Vec::new();

    for i in 0..n {
        let pool = pool.clone();

        let rx = rec.clone();
        let id = thread::spawn(move || {
            let mut conn = pool.get().expect("db conn error");
            trace!("starting thread {}", i);
            worker(rx, &mut conn);
        });
        handles.push(id);
    }

    return handles;
}

pub fn worker(rx: ChanReceiver, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> anyhow::Result<()> {
    for entry in &rx {
        // get metadata on this file
        let mut fi = FileInfo::default();

        // copy path, name and extension
        // manage cases of Windows for UTF-16 strings
        fi.path = entry.path().to_string_lossy().into_owned();
        fi.winpath = entry.path().as_os_str().into();

        fi.name = entry.file_name().to_string_lossy().into_owned();
        fi.winname = entry.file_name().into();

        fi.r#type = file_type(&entry.file_type());

        // get metadata
        let meta = entry.metadata()?;
        fi.len = meta.len() as i32;

        // timestamps
        fi.created = systemtime_to_naive(meta.created()?);
        fi.accessed = systemtime_to_naive(meta.accessed()?);
        fi.modified = systemtime_to_naive(meta.modified()?);


        // calculate hashes
        let data = fs::read(entry.path())?;
        fi.blake3 = blake3::hash(&data).to_string();

        // insert data
        diesel::insert_into(files).values(&fi).execute(conn);


        /*         let hash = Hashes::sha256(&entry).unwrap();
        trace!(
            "{:?}: path:{} hash:{}",
            thread::current().id(),
            entry.display(),
            hash
        ); */

        trace!("{:?}", fi);
    }
    // let data = read(path)?;
    // let h = blake3::hash(&data);

    // println!("==============> starting worker");

    // while let Ok(path) = rx.recv() {
    //     let hash = Hashes::sha256(&path)?;
    //     println!("{}:{}", path.display(), hash);
    // }

    // println!("==============> end worker");

    Ok(())
}
// 
fn file_type(ft: &FileType) -> &'static str {
    if ft.is_file() {
        "F"
    } else if ft.is_dir() {
        "D"
    } else if ft.is_symlink() {
        "S"
    } else {
        "U"
    }
}

// SystemTime -> NaiveDateTime
fn systemtime_to_naive(time: SystemTime) -> NaiveDateTime {
    let dt = DateTime::<Utc>::from(time);
    dt.naive_utc()
}