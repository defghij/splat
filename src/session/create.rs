use super::*;

use details::Select;
use job::Jobs;

/// Read in a toml file from a [PathBuf], attempt to validate it, and return. This function returns
/// a configuration specific [CreationError] if the validation fails. Otherwise a [Session] is returned.
pub fn from_path(path: PathBuf) -> Result<Session, CreationError> {
    let config: Unvalidated = path.try_into()?;
    let session: Validated = config.try_into()?;
    
    Ok(session.0)
}


/// Wrapper type used to isolate parsed/deserialized but unvalidated configuration from a
/// [PathBuf].
#[derive(Clone, Debug, Deserialize)]
struct Unvalidated(SessionConfiguration);
impl Unvalidated {

    /// This checks whether the constraints between `step` and `select` in [shape::Shape] and
    /// `value` in [Job] hold. If they do not hold, an error is returned.
    fn check_value_constraints(&self) -> Result<(), CreationError> {
        tracing::debug!("Checking numerical constaints on the shape of jobs");

        let total_value = self.0.jobs
            .iter()
            .fold(0, |acc, wt| acc + wt.value.unwrap_or(0) );

        match self.0.details.shape.select  {
            Select::Random => {
                if total_value >= 101 {
                    return Err(CreationError::Validation("Provided Job weights exceed 100"));
                }
            },
            Select::Linear |
            Select::Interleave => {
                if total_value > self.0.details.shape.steps {
                    return Err(
                        CreationError::Validation("Individual Job steps exceeds Session total"));
                }
            },
        }
        Ok(())
    }
}
impl TryFrom<std::path::PathBuf> for Unvalidated {
    type Error = CreationError;

    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        tracing::debug!("Converting PathBuf to an Unvalidated session");

        let bytes = std::fs::read(value).map_err(|e| Self::Error::Io(e))?;
        let raw_toml: SessionConfiguration = toml::from_slice(bytes.as_slice()).map_err(|e| Self::Error::Parse(e))?;

        Ok(Unvalidated(raw_toml))
    }
}

struct Validated(Session);
impl TryFrom<Unvalidated> for Validated {
    type Error = CreationError;

    fn try_from(value: Unvalidated) -> Result<Self, Self::Error> {
        tracing::debug!("Converting an Unvalidated session to a validated Session");

        value.check_value_constraints()?;

        let SessionConfiguration { details, jobs } = value.0;

        details.validate()?;

        if jobs.is_empty() {
            return Err(Self::Error::Validation("At least on Job must be supplied")); 
        }
        jobs.iter()
            .try_for_each(|job| job.validate() )?;

        let jobs: Result<Jobs,CreationError> = jobs.iter().map(|job| {
            // todo!: clones everywhere!
            job.clone().fill_in(&details.modifiers).clone().try_into()
        }).collect();

        let details: Details = details.try_into()?;

        let session = Session {
            details,
            jobs: jobs?,
        };

        Ok(Validated(session))
    }
}

/// Error type that captures to two ways that reading in and serializing a file into a toml-based
/// structure may fail.
#[derive(Debug, Error)]
pub enum CreationError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Validation error: {0}")]
    Validation(&'static str),
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
            let got = create::from_path(get(file));
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
            let got = create::from_path(get(file));
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
