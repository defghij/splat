

pub fn setup_tracing(level: tracing::level_filters::LevelFilter) {
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
