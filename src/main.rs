
mod session;
mod logging;

use tracing::{error, level_filters::LevelFilter};

use logging::setup_tracing;

fn main() {
    setup_tracing(LevelFilter::DEBUG);

    let session = session::create::from_path("../assets/commands.random.weights.good.toml".into());

    if session.is_err() {
        error!("Unable to create session from provided configuration file -- {}",
            session.err().expect("is err due to conditional"));
        return;
    }

    println!("{session:?}");
}
