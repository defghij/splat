
mod session;
mod setup;

use tracing::{error, level_filters::LevelFilter};

fn main() {
    setup::tracing(LevelFilter::DEBUG);
    let session_file_path = setup::cli();

    let session = session::create::from_path(session_file_path.clone());
    if session.is_err() {
        error!("Unable to create session from provided configuration file -- {}",
            session.err().expect("is err due to conditional"));
        return;
    } else {
        tracing::info!("Session loaded from {0}", session_file_path.display());
    }


    println!();
    println!("{session:?}");
}
