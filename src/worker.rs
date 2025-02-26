// module for main worker
use std::{
    fs::{self, read, File, Metadata},
    io::{BufReader, Result},
    path::{Path, PathBuf},
    thread::{self, JoinHandle},
};

use crossbeam_channel as channel;
use log::trace;
use walkdir::{DirEntry, DirEntryExt};

use crate::{fileinfo::FileInfo, hash::Hashes};
pub type ChanReceiver = channel::Receiver<DirEntry>;

pub fn pool(n: usize, rec: ChanReceiver) -> Vec<JoinHandle<()>> {
    // create n threads for handle workers
    let mut handles = Vec::new();

    for i in 0..n {
        let rx = rec.clone();
        let id = thread::spawn(move || {
            trace!("starting thread {}", i);
            worker(rx);
        });
        handles.push(id);
    }

    return handles;
}

pub fn worker(rx: ChanReceiver) -> anyhow::Result<()> {
    for entry in &rx {
        // get metadata on this file
        let mut fi = FileInfo::default();

        // copy path, name and extension
        fi.path = entry.path().to_string_lossy().into_owned();
        fi.name = entry.file_name().to_os_string().into_string().unwrap();
        fi.r#type = file_type(&entry.metadata().unwrap());
        //fi.path = entry.into_os_string().into_string().unwrap();


        // calculate hash
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

fn file_type(entry: &Metadata) -> &'static str {
    if entry.is_file() {
        "F"
    } else if entry.is_dir() {
        "D"
    } else if entry.is_symlink() {
        "S"
    } else {
        "U"
    }

}
