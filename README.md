<!-- # iaa
Index and Analyze

## install PG dev libraries
$ sudo apt install libpq-dev

## install Podman -->

# Index And Analyze

A lightweight forensic artefact database and query platform for disk images and directories.

This project is a **backend-first tool** for extracting file metadata (timestamps, hashes, entropy, and file-type specific metadata) and storing it in PostgreSQL for fast, flexible querying. It is written in Rust and designed to be extensible via file-type plugins.

> ⚠️ This is an experimental tool for personal use and research.  
> It is not a full forensic suite.

---

## Features

- Recursively scans a directory or mounted disk image
- Extracts common file metadata:
  - file name path, size, extension
  - timestamps (created, modified, accessed)
  - hashes (SHA256, Blake3)
  - Shanon entropy
  - ...
- Extracts file-type specific metadata and stores it in a JSONB column
  - e.g., SQLite file table names & row counts, PNG dimensions & bit depth, etc
- Stores all artefacts in PostgreSQL for powerful SQL queries
- Extensible plugin architecture for new file types (not yet)
- Designed for automation and integration into DFIR workflows

---

## Why this exists

Most forensic tools generate reports or proprietary databases. This project focuses on **structured, queryable storage** of artefacts, enabling:

- fast triage across many disk images
- ad-hoc queries without scripting
- correlation between artefacts across file types
- reproducible research workflows

---

## Requirements

- Rust (stable)
- PostgreSQL 18+


---

## Installation

```console
$ git clone https://github.com/dandyvica/iaa.git
$ cd iaa
$ cargo build --release
$ target/release/iaa -h
Collect artefacts from a source

Usage: iaa [OPTIONS] --dir <PATH>

Options:
  -d, --dir <PATH>         starting directory path
  -t, --threads <THREADS>  number of thread to use
      --log <LOG>          log file
      --db <DB>            Postgresql database URL. if not specified, takes the value from the IAA_DB enviroment variable
      --overwrite          if set, delete all rows from the table before inserting
      --blake3             if set, calculate BLAKE3 hashes
      --sha256             if set, calculate SHA256 hashes
      --entropy            if set, calculate Shannon entropy
      --discover           if set, analyze file signatures to discover file type and add some metadat
  -v, --verbose...         Verbose mode (-v, -vv, -vvv)
  -n, --n <COUNT>          stop after COUNT files
      --dry-run            don't insert data in the database, just print out file details
  -h, --help               Print help
  -V, --version            Print version
```

## TODO
It's only v0.1.0.

What I have in mind:

* add TOML configration file for more flexibility
* add batch insert possibility
* add more file type metadata discovering
* hone error management
* define plugin mecanism
* extend unit tests and GH actions
* ...