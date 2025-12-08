use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;

pub fn read_configuration(path: PathBuf) -> Result<Config, ConfigError> {
    let tentative_config: Unvalidated = path.try_into()?;
    let validated = tentative_config.validate()?;

    Ok(validated.clone())
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    details: Details,

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
        if !config.details.name.is_ascii() { 
            return Err(ConfigError::Validation("Session name must be ascii".into()));
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
    Validation(String),
}

#[derive(Clone, Debug, Deserialize)]
struct Job {
    /// The string that describes the 
    cmd: String,

    /// The base path name for the STDOUT output. This will be appended to if there are multiple
    stdout: Option<String>,

    /// The base path name for the STDERR output. This will be appended to if there are multiple
    /// steps that are requested. If no path is provided then no output file will be written.
    stderr: Option<String>,
} impl Into<std::process::Command> for Job {
    fn into(self) -> std::process::Command {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Details {
    /// Optional name for the [Session]
    name: String,

    /// Whether the base program should emit a logging file.
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
    shape: shape::Session,
}

mod shape {
    use super::*;
    use garde::Validate;

    #[derive(Clone, Debug, Deserialize)]
    enum Select {
        Random,
        Linear,
        Interleave,
    } impl Default for Select {
        fn default() -> Self { Select::Interleave }
    }

    #[derive(Clone, Debug, Deserialize, Validate)]
    pub(super) struct Session {
        #[serde(default)]
        #[garde(skip)]
        select: Select, 
        
        #[garde(range(min=0, max=u64::MAX))]
        steps: u64,
    }

    /* Eventually include a "shape" for jobs so that the "randomness" can be tuned rather than
      uniform across jobs
    */
}
