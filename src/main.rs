
mod session;

fn main() {
    let session = session::create::from_path("../assets/commands.toml".into())
        .unwrap();
    println!("{session:?}");
}
