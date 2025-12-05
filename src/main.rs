use std::{sync::Arc, time::Instant};

// crates
use crossbeam_channel as channel;
use diesel::RunQueryDsl;
use log::{debug, error, info, trace};

use walkdir::{DirEntry, WalkDir};

// local modules
mod args;
use args::get_args;

mod worker;
use worker::thread_pool;

mod hash;
mod schema;

mod fileinfo;

mod pool;
use pool::establish_pool;

use schema::files::dsl::files;

mod memory;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();

    //───────────────────────────────────────────────────────────────────────────────────
    // get cli options
    //───────────────────────────────────────────────────────────────────────────────────
    let args = get_args()?;
    debug!("options: {:?}", args);

    //───────────────────────────────────────────────────────────────────────────────────
    // create a connection pool for PG
    //───────────────────────────────────────────────────────────────────────────────────
    let pool = establish_pool(&args.db);

    //───────────────────────────────────────────────────────────────────────────────────
    // delete all rows first if requested
    //───────────────────────────────────────────────────────────────────────────────────
    if args.overwrite {
        let mut conn = pool.get()?;
        diesel::delete(files).execute(&mut conn)?;
    }

    //───────────────────────────────────────────────────────────────────────────────────
    // create channels
    //───────────────────────────────────────────────────────────────────────────────────
    let (job_sender, job_receiver) = channel::unbounded::<DirEntry>();

    //───────────────────────────────────────────────────────────────────────────────────
    // start threads
    //───────────────────────────────────────────────────────────────────────────────────
    let connection_pool = Arc::new(pool);
    let args = Arc::new(args);
    let handles = thread_pool(args.threads.unwrap(), job_receiver, &connection_pool, &args)?;
    info!("created {} threads", args.threads.unwrap());

    //───────────────────────────────────────────────────────────────────────────────────
    // walk through directory
    //───────────────────────────────────────────────────────────────────────────────────
    let mut file_count = 0u64;
    for entry in WalkDir::new(&args.dir) {
        if let Ok(entry) = entry {
            file_count += 1;
            job_sender.send(entry)?;
        } else {
            error!("error processing entry '{:?}'", entry);
        }
    }
    drop(job_sender);

    //───────────────────────────────────────────────────────────────────────────────────
    // wait for threads to finish
    //───────────────────────────────────────────────────────────────────────────────────
    for id in handles {
        id.join()
            .map_err(|e| format!("error {:?}: unable to join thread", e));
    }

    //───────────────────────────────────────────────────────────────────────────────────
    // elapsed as millis will be hopefully enough
    //───────────────────────────────────────────────────────────────────────────────────
    let elapsed = now.elapsed();
    info!("took {} millis for {file_count} artefacts", elapsed.as_millis());

    Ok(())
}
