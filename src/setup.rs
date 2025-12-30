use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;

pub fn tracing(level: tracing::level_filters::LevelFilter) {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .with_line_number(true)
        .with_file(true)
        .with_thread_ids(false)
        .with_max_level(level)
        .without_time()
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("Subscriber setup should succeed");
}

// TODO: This currently only returns the single cli option. This will obviously not do in the
// future
pub fn cli() -> PathBuf {
    let args: ArgMatches = Command::new("splat")
        .about("Utility to spawn a bunch of processes simply and quickly")
        .version("0.1.0")
        .author("defghij@starscourge")
        .arg(
            Arg::new("file")
                .long("session")
                .short('s')
                .value_parser(clap::value_parser!(PathBuf))
                .required(true)
                .help("Path to the (S)ession configuration file"),
        )
        .get_matches();

     args.get_one::<PathBuf>("file")
        .expect("File should be a required argument that is validated by Clap")
        .clone()
}
