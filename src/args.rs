use std::fs::OpenOptions;
use std::path::PathBuf;

//use clap::builder::styling;
use anyhow::anyhow;
use clap::builder::styling;
use clap::Parser;
use simplelog::*;

const DB: &'static str = "IAA_DB";

/// Collect artefacts from a source.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, color = clap::ColorChoice::Always)]
pub struct Args {
    /// starting directory path
    #[arg(short, long, required = true, value_name = "PATH")]
    pub dir: PathBuf,

    /// number of thread to use
    #[arg(long, short)]
    pub threads: Option<usize>,

    /// log file
    #[arg(long)]
    pub log: Option<PathBuf>,

    /// Postgresql database URL. if not specified, takes the value from the IAA_DB enviroment variable
    #[arg(long, required = false)]
    pub db: Option<String>,

    /// if set, delete all rows from the table before inserting
    #[arg(long)]
    pub overwrite: bool,

    /// if set, calculate BLAKE3 hashes
    #[arg(long)]
    pub blake3: bool,

    /// if set, calculate SHA256 hashes
    #[arg(long)]
    pub sha256: bool,

    /// if set, calculate Shannon entropy
    #[arg(long)]
    pub entropy: bool,

    /// Verbose mode (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

pub fn get_args() -> anyhow::Result<Args> {
    let mut args = Args::parse();

    // by default, use number of cores for threads
    if args.threads.is_none() {
        args.threads = Some(num_cpus::get());
    }

    // manage DB url
    if args.db.is_none() {
        match std::env::var(DB) {
            Ok(db) => args.db = Some(db),
            Err(e) => return Err(anyhow!("no PG DB provided!")),
        }
    }

    // extract loglevel from verbose flag
    let level = match args.verbose {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    // manage log file
    if let Some(path) = &args.log {
        init_write_logger(&path, level)?;
    } else {
        init_term_logger(level)?;
    }

    Ok(args)
}

// colors when displaying
const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

// get raw command line, what the user typed
pub fn raw_args() -> String {
    let args: Vec<_> = std::env::args().collect();
    args.join(" ")
}

// Initialize write logger: either create it or use it
fn init_write_logger(logfile: &PathBuf, level: log::LevelFilter) -> anyhow::Result<()> {
    if level == log::LevelFilter::Off {
        return Ok(());
    }

    // initialize logger
    let writable = OpenOptions::new().create(true).append(true).open(logfile)?;

    WriteLogger::init(
        level,
        simplelog::ConfigBuilder::new()
            .set_time_format_rfc3339()
            // .set_time_format_custom(format_description!(
            //     "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
            .build(),
        writable,
    )?;

    Ok(())
}

// Initialize terminal logger
fn init_term_logger(level: log::LevelFilter) -> anyhow::Result<()> {
    if level == log::LevelFilter::Off {
        return Ok(());
    }
    TermLogger::init(
        level,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    Ok(())
}
