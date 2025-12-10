// module for main worker
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
    time::SystemTime,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use crossbeam_channel as channel;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, RunQueryDsl,
};
use log::trace;
use walkdir::{DirEntry, DirEntryExt};

use crate::{
    args::Args,
    fileinfo::{FileInfo, ForensicsFileType},
};
use crate::{memory::MappedFile, schema::artefact::dsl::artefact};

pub type ChanReceiver = channel::Receiver<DirEntry>;

// buld a thread pool: each thread will start a worker aimed at inserting data into PG
pub fn thread_pool(
    n: usize,
    rec: ChanReceiver,
    pool: &Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    args: &Arc<Args>,
) -> anyhow::Result<Vec<JoinHandle<()>>> {
    // create n threads for handle workers
    let mut handles = Vec::new();

    for i in 0..n {
        let pool = pool.clone();
        let args = args.clone();

        let rx = rec.clone();
        let id = thread::spawn(move || {
            let mut conn = pool.get().expect("db conn error");
            trace!("starting thread {}", i);
            if let Err(e) = worker(rx, &mut conn, &args) {
                eprintln!("error '{e}' in closure");
            }
        });
        handles.push(id);
    }

    return Ok(handles);
}

// worker receiving DirEntry and inserting it into the PG table
pub fn worker(
    rx: ChanReceiver,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    args: &Arc<Args>,
) -> anyhow::Result<()> {
    for entry in &rx {
        // get metadata on this file
        let mut fi = FileInfo::default();

        // copy path, name and extension
        // manage cases of Windows for UTF-16 strings
        fi.path = entry.path().to_string_lossy().into_owned();
        fi.winpath = entry.path().as_os_str().into();
        fi.ext = entry
            .path()
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        fi.name = entry.file_name().to_string_lossy().into_owned();
        fi.winname = entry.file_name().into();

        fi.r#type = ForensicsFileType::from(&entry.file_type());

        // get metadata
        let meta = entry.metadata()?;
        fi.len = meta.len() as i64;

        // timestamps
        if let Ok(time) = meta.created() {
            fi.created = Some(time);
        }
        fi.accessed = meta.accessed()?;
        fi.modified = meta.modified()?;

        // calculate hashes, only for files
        if fi.r#type == ForensicsFileType::File && fi.len != 0 {
            // for other operations, we need to open and read files
            let mapped = MappedFile::try_from(entry.path())?;

            // according to oprions, call whatever is asked
            if args.blake3 {
                fi.blake3 = mapped.blake3();
            }

            if args.sha256 {
                fi.sha256 = mapped.sha256();
            }

            if args.entropy {
                fi.entropy = Some(mapped.entropy());
            }
        }

        // insert data
        diesel::insert_into(artefact).values(&fi).execute(conn)?;
        trace!("{:?}", fi);
    }

    Ok(())
}
