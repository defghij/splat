use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;
use garde::Validate;

pub fn read_configuration(path: PathBuf) -> Result<Config, ConfigError> {
    let tentative_config: Unvalidated = path.try_into()?;
    let validated = tentative_config.validate()?;

    Ok(validated.clone())
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    details: Session,

    #[serde(rename = "command")]
    commands: Vec<Job>,
}

/// Wrapper type used to isolate parsed/deserialized but unvalidated configuration from a
/// [PathBuf].
#[derive(Clone, Debug, Deserialize)]
struct Unvalidated(Config);
impl Unvalidated {
    fn validate(&self) -> Result<&Config, ConfigError> {
        let config = &self.0;
        config.details.validate()?;

        if config.commands.is_empty() {
            return Err(ConfigError::Validation("At least on Job must be supplied")); 
        }

        let total_value = config.commands
            .iter()
            .fold(0, |acc, wt| acc + wt.value.unwrap_or(0) );

        match config.details.shape.select  {
            shape::Select::Random => {
                if total_value >= 101 {
                    return Err(ConfigError::Validation("Provided Job weights exceed 100"));
                }
            },
            shape::Select::Linear |
            shape::Select::Interleave => {
                if total_value >= config.details.shape.steps {
                    return Err(
                        ConfigError::Validation("Individual Job steps exceeds Session total"));
                }
            },
        }

        Ok(config)
    }
}
impl TryFrom<std::path::PathBuf> for Unvalidated {
    type Error = ConfigError;

    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        let bytes = std::fs::read(value).map_err(|e| ConfigError::Io(e))?;
        let raw_toml: Config = toml::from_slice(bytes.as_slice()).map_err(|e| ConfigError::Parse(e))?;

        Ok(Unvalidated(raw_toml))
    }
}

/// Error type that captures to two ways that reading in and serializing a file into a toml-based
/// structure may fail.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Validation error: {0}")]
    Validation(&'static str),
}

#[derive(Clone, Debug, Deserialize)]
struct Job {
    /// The string that describes the 
    cmd: String,

    /// The base path name for the STDOUT output. This will be appended to if there are multiple
    ///
    /// Providing this overrides the [Session] value.
    stdout: Option<String>,

    /// The base path name for the STDERR output. This will be appended to if there are multiple
    /// steps that are requested. If no path is provided then no output file will be written.
    ///
    /// Providing this overrides the [Session] value.
    stderr: Option<String>,

    /// This is a [shape::Select] dependent value. The interpretation of this value is dependent on
    /// the variant selected.
    ///
    /// TODO: Make this an enum or struct?
    value: Option<u64>,
} impl Into<std::process::Command> for Job {
    fn into(self) -> std::process::Command {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Session {
    /// Name for the [Session]
    name: String,

    /// Whether the base program should emit a logging file.
    #[serde(default)]
    logging: bool,

    /// A optional wrapper command that will be prepended to each [Job]. 
    /// e.g. "time", "strace", "srun -N1"
    wrapper: Option<String>,

    /// Base path that should be used for STDOUT files. This is inherited by all members. This may
    /// be overwritten by the [Job].
    stdout: Option<String>,

    /// Base path that should be used for STDERR files. This is inherited by all members. This may
    /// be overwritten by the [Job].
    stderr: Option<String>,

    /// Defines the number and order of [Jobs] launched in a [Session].
    shape: shape::Shape,
} impl Session {
    fn validate(&self) -> Result<(), ConfigError> {
        if !self.name.is_ascii() { 
            return Err(ConfigError::Validation("Session name must be ascii"));
        }

        Ok(())
    }
}


mod shape {
    use super::*;

    /// The ways in which the next job may be selected.
    ///
    #[derive(Clone, Debug, Deserialize)]
    pub(super) enum Select {
        /// Randomly select the next job from the provided jobs in the [Config].
        /// The probability that the next step will belong to any particular [Job] is determined by
        /// that [Job]. Weights should total 100.
        ///
        /// TODO: If no weight is provided for a [Job], its weight defaults to an even split of the
        /// remaining probability. That is if there are three jobs where only one is given a weight 
        /// of 50 then the remaining two jobs will have a probability of 25 for a total of 100.
        Random,

        /// The next step should be pulled from the current [Job] unless its step count is
        /// exhausted. If so, pull from the next [Job] in the [Config]. The steps from the next job
        /// will not be pulled until the current [Job] is exhausted.
        /// 
        /// An error will halt all steps if an error is encountered.
        Linear,

        /// Next step is pulled from the next [Job] so long as it has not exceeded its step count.
        /// This is similar to [Select::Linear] except that a chunk determines how many steps to do
        /// before advancing to the next [Job]. This continues until total step count is reached.
        ///
        /// An error does not halt execution.
        Interleave,
    } impl Default for Select {
        fn default() -> Self { Select::Interleave }
    }

    #[derive(Clone, Debug, Deserialize)]
    pub(super) struct Shape {
        /// The method of step/job selection for this [Session].
        #[serde(default)]
        pub select: Select, 
        
        /// The total steps in this [Session]. Note, this may be overridden by the [Job]. 
        pub steps: u64,

        /// The number of steps that should be active at any particular point in time. 
        pub parallel: u64,
    }

    /* Eventually include a "shape" for jobs so that the "randomness" can be tuned rather than
      uniform across jobs
    */
}

/// TODO: These should check the variants and inner data
#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn file_validation_fails() {
        let validation: Vec<(&str, Result<(),()>)> = vec![
            ("commands.random.weights.bad.toml",     Err(())),
            ("commands.linear.total_steps.bad.toml", Err(())),
            ("select.variant.bad.toml",              Err(())),
        ];
        validation.iter().for_each(|(file, _expected)| {
            let got = read_configuration(get(file));
            assert!(got.is_err())
        });
    }

    #[test]
    fn file_validation_succeeds() {
        let validation: Vec<&str> = vec![
            "commands.random.weights.good.toml",
            "commands.linear.total_steps.good.toml",
        ];
        validation.iter().for_each(|file| {
            let got = read_configuration(get(file));
            if  got.is_err() {
                println!("Failed: {file} with\t{:?}", got.err());
                assert!(false)
            }
        });
    }

    fn get(name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_PATH"));
        path.pop(); // remove `Cargo.toml`
        path.push("assets/"); 
        path.push(name);
        path
    }
}
