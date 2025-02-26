use std::fs::OpenOptions;
use std::path::PathBuf;

use clap::builder::styling;
use clap::{crate_version, Arg, ArgAction, Command};
use simplelog::*;

//───────────────────────────────────────────────────────────────────────────────────
// This structure holds the command line arguments.
//───────────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Default, Clone)]
pub struct CliOptions {
    // starting directory path
    pub start_path: PathBuf,

    // number of threads
    pub nb_threads: usize,
}

impl CliOptions {
    pub fn new() -> anyhow::Result<Self> {
        // save all cli options into a structure
        let mut options = CliOptions::default();

        // colors when displaying
        const STYLES: styling::Styles = styling::Styles::styled()
            .header(styling::AnsiColor::Green.on_default().bold())
            .usage(styling::AnsiColor::Green.on_default().bold())
            .literal(styling::AnsiColor::Blue.on_default().bold())
            .placeholder(styling::AnsiColor::Cyan.on_default());

        let matches = Command::new("A DNS query tool inspired by dig, drill and dog")
            .version(crate_version!())
            .long_version(crate_version!())
            .styles(STYLES)
            .author("Alain Viguier dandyvica@gmail.com")
            .arg(
                Arg::new("dir")
                    .short('d')
                    .long("dir")
                    .long_help("Starting directory from which to walk.")
                    .action(ArgAction::Append)
                    .value_name("PATH")
                    .value_parser(clap::value_parser!(PathBuf))
                    .default_value("."),
            )
            .arg(
                Arg::new("threads")
                    .short('t')
                    .long("threads")
                    .long_help("Number of threads to start.")
                    .action(ArgAction::Append)
                    .value_name("THREADS")
                    .value_parser(clap::value_parser!(usize)),
            )
            .arg(
                Arg::new("log")
                    .long("log")
                    .long_help("Save debugging info into the file LOG.")
                    .action(ArgAction::Set)
                    .value_name("LOG")
                    .value_parser(clap::value_parser!(PathBuf)),
            )
            .get_matches();

        //───────────────────────────────────────────────────────────────────────────────────
        // manage options
        //───────────────────────────────────────────────────────────────────────────────────
        options.start_path = matches.get_one::<PathBuf>("dir").unwrap().clone();
        options.nb_threads = *matches
            .get_one::<usize>("threads")
            .unwrap_or(&num_cpus::get());

        if let Some(path) = matches.get_one::<PathBuf>("log") {
            init_write_logger(path, log::LevelFilter::Trace)?;
        } else {
            init_term_logger(log::LevelFilter::Trace)?;
        }

        Ok(options)
    }
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
