mod config;

fn main() {

    let config = config::read_configuration("../assets/commands.toml".into()).unwrap();
    println!("{config:?}");
}
