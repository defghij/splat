
mod session;

fn main() {
    let config = session::read_configuration("../assets/commands.toml".into())
        .unwrap();
    println!("{config:?}");

}
