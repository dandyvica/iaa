use std::{path::PathBuf, sync::Arc, time::Instant};

// crates
use crossbeam_channel as channel;
use log::{debug, error, info, trace};
use threadpool::ThreadPool;
use walkdir::{DirEntry, Result, WalkDir};
use worker::worker;


// local modules
mod args;
use args::CliOptions;

mod worker;
use worker::thread_pool;

mod hash;
mod schema;

mod fileinfo;

mod pool;
use pool::establish_pool;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();

    //───────────────────────────────────────────────────────────────────────────────────
    // get cli options
    //───────────────────────────────────────────────────────────────────────────────────
    let options = CliOptions::new()?;
    debug!("options: {:?}", options);

    let pool = establish_pool();
    let connection_pool= Arc::new(pool);

    //───────────────────────────────────────────────────────────────────────────────────
    // create channels
    //───────────────────────────────────────────────────────────────────────────────────
    let (job_sender, job_receiver) = channel::unbounded::<DirEntry>();

    //───────────────────────────────────────────────────────────────────────────────────
    // start threads
    //───────────────────────────────────────────────────────────────────────────────────
    let handles = thread_pool(options.nb_threads, job_receiver, &connection_pool);
    info!("created {} threads", options.nb_threads);

    // walk through directory
    for entry in WalkDir::new(&options.start_path) {
        if let Ok(entry) = entry {
            job_sender.send(entry).unwrap();
        } else {
            error!("error processing entry '{:?}'", entry);
        }
    }

    drop(job_sender);

    // wait for threads to finish
    for id in handles {
        id.join()
            .map_err(|e| format!("error {:?}: unable to join thread", e));
    }

    //───────────────────────────────────────────────────────────────────────────────────
    // elapsed as millis will be hopefully enough
    //───────────────────────────────────────────────────────────────────────────────────
    let elapsed = now.elapsed();
    info!("took {} millis", elapsed.as_millis());

    Ok(())
}
