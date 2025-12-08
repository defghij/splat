use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
struct Job {
    /// The string that describes the 
    cmd: String,

    /// The base path name for the STDOUT output. This will be appended to if there are multiple
    stdout: Option<String>,

    /// The base path name for the STDERR output. This will be appended to if there are multiple
    /// steps that are requested. If no path is provided then no output file will be written.
    stderr: Option<String>,

    /// The number of times this [Session] [Job] should be repeated.
    /// This is exclusive with `weight`.
    step: u64,

    // todo: for use with a toml top-level count
    // weight
}
impl Into<std::process::Command> for Job {
    fn into(self) -> std::process::Command {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize)]
enum Shape {
    Random,
    Linear,
    Interleave,
}

#[derive(Clone, Debug, Deserialize)]
struct Details {
    /// Optional name for the [Session]
    name: Option<String>,

    /// Whether the base program should emit a logging file.
    logging: bool,

    /// Base path that should be used for STDOUT files. This is inherited by all members. This may
    /// be overwritten by the [Job].
    stdout: Option<String>,

    /// Base path that should be used for STDERR files. This is inherited by all members. This may
    /// be overwritten by the [Job].
    stderr: Option<String>,

    /// If provided determines the total number of steps in the [Session]. Each [Job] gets an
    /// proportion of the total requested steps. 
    steps: Option<u64>,

    select: Shape,
}

#[derive(Clone, Debug, Deserialize)]
struct Session {
    details: Option<Details>,

    #[serde(rename = "command")]
    commands: Vec<Job>,
}


fn main() {
    let commands: &'static str = include_str!("../assets/commands.toml");

    let session: Session = toml::from_str(commands).unwrap();
    println!("{session:?}");

    println!("Hello, world!");
}
