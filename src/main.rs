use std::{path::PathBuf, time::Instant};

use crossbeam_channel as channel;
use log::{debug, error, info, trace};
use threadpool::ThreadPool;
use walkdir::{DirEntry, Result, WalkDir};
use worker::worker;

mod args;
use args::CliOptions;

mod worker;
use worker::pool;

mod hash;
mod schema;
use schema::establish_sqlite3_connection;
mod fileinfo;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();

    //───────────────────────────────────────────────────────────────────────────────────
    // get cli options
    //───────────────────────────────────────────────────────────────────────────────────
    let options = CliOptions::new()?;
    debug!("options: {:?}", options);

    let conn = establish_sqlite3_connection("iaa.db")?;

    //───────────────────────────────────────────────────────────────────────────────────
    // create channels
    //───────────────────────────────────────────────────────────────────────────────────
    let (job_sender, job_receiver) = channel::unbounded::<DirEntry>();

    //───────────────────────────────────────────────────────────────────────────────────
    // start threads
    //───────────────────────────────────────────────────────────────────────────────────
    let handles = pool(options.nb_threads, job_receiver);
    info!("created {} threads", options.nb_threads);

    // walk through directory
    for entry in WalkDir::new(&options.start_path) {
        if let Ok(entry) = entry {
            // if let Ok(metadata) = entry.metadata() {
            //     if metadata.is_file() {
            //         let path = entry.path().to_owned();
            //         trace!("processing path: {}", path.display());
            //         // println!("processing path: {}", path.display());

            //         // let tx = tx.clone();
            //         job_sender.send(path).unwrap();

            //         // pool.execute(move || {
            //         //     tx.send(path).unwrap();
            //         //     // let hash = blake3_hash(&path).unwrap();
            //         //     // println!("{}:{}", hash, path.display());
            //         //     // // tx.send(hash).expect("Could not send data!");
            //         // });
            //     }
            // }
            job_sender.send(entry).unwrap();

        } else {
            error!("error processing entry '{:?}'", entry);
        }
    }

    drop(job_sender);

    // wait for threads to finish
    for id in handles {
        id.join().unwrap();
    }

    //───────────────────────────────────────────────────────────────────────────────────
    // elapsed as millis will be hopefully enough
    //───────────────────────────────────────────────────────────────────────────────────
    let elapsed = now.elapsed();
    info!("took {} millis", elapsed.as_millis());

    Ok(())
}
