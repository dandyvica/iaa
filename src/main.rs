use std::{
    sync::Arc,
    time::{Instant, SystemTime},
    u64,
};

// crates
use crossbeam_channel as channel;
use diesel::RunQueryDsl;
use humantime::format_duration;
use log::{debug, error, info, trace};

use walkdir::{DirEntry, WalkDir};

// local modules
mod args;
use args::get_args;

mod worker;
use worker::thread_pool;

mod fileinfo;
mod hash;
mod memory;
mod pool;
mod schema;
use pool::establish_pool;
mod discoverer;

use schema::artefact::dsl::artefact;

use crate::{args::raw_args, fileinfo::RunHistory, schema::run_history::dsl::run_history};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();

    //───────────────────────────────────────────────────────────────────────────────────
    // get cli options
    //───────────────────────────────────────────────────────────────────────────────────
    let command_line = raw_args();
    let args = get_args()?;
    debug!("options: {:?}", args);

    let max_count = args.n.unwrap_or_else(|| u64::MAX);

    //───────────────────────────────────────────────────────────────────────────────────
    // start recording history
    //───────────────────────────────────────────────────────────────────────────────────
    let mut history = RunHistory::default();
    history.args = command_line;

    //───────────────────────────────────────────────────────────────────────────────────
    // create a connection pool for PG
    //───────────────────────────────────────────────────────────────────────────────────
    let pool = establish_pool(&args.db.clone().unwrap(), args.threads.unwrap() as u32);

    //───────────────────────────────────────────────────────────────────────────────────
    // delete all rows first if requested
    //───────────────────────────────────────────────────────────────────────────────────
    if args.overwrite {
        let mut conn = pool.get()?;
        diesel::delete(artefact).execute(&mut conn)?;
    }

    //───────────────────────────────────────────────────────────────────────────────────
    // create channels
    //───────────────────────────────────────────────────────────────────────────────────
    let (job_sender, job_receiver) = channel::unbounded::<DirEntry>();

    //───────────────────────────────────────────────────────────────────────────────────
    // start threads
    //───────────────────────────────────────────────────────────────────────────────────
    let mut history_conn = pool.get()?;
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

            // stops after n rounds
            if file_count >= max_count {
                return Ok(());
            }
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
    info!(
        "took {} for {file_count} artefacts",
        format_duration(elapsed)
    );

    //───────────────────────────────────────────────────────────────────────────────────
    // end up history
    //───────────────────────────────────────────────────────────────────────────────────
    history.nb_files = file_count as i64;
    history.end_time = SystemTime::now();
    history.elapsed = format_duration(elapsed).to_string();

    diesel::insert_into(run_history)
        .values(&history)
        .execute(&mut history_conn)?;

    Ok(())
}
