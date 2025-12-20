use super::*;

use details::Select;

/// Read in a toml file from a [PathBuf], attempt to validate it, and return. This function returns
/// a configuration specific [Error] if the validation fails. Otherwise a [Session] is returned.
pub fn from_path(path: PathBuf) -> Result<Session, Error> {
    let config: Unvalidated = path.try_into()?;
    
    Ok(config.validate()?
             .finalize()?)
}

/// Error type that captures to two ways that reading in and serializing a file into a toml-based
/// structure may fail.
#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Validation error: {0}")]
    Validation(&'static str),
}

/// Wrapper type used to isolate parsed/deserialized but unvalidated configuration from a
/// [PathBuf].
#[derive(Clone, Debug, Deserialize)]
struct Unvalidated(Session);
impl Unvalidated {
    fn validate(self) -> Result<Validated, Error> {
        self.0.details.validate()?;

        if self.0.jobs.is_empty() {
            return Err(Error::Validation("At least on Job must be supplied")); 
        }
        self.0.jobs
            .iter()
            .try_for_each(|job| job.validate() )?;

        self.check_value_constraints()?;

        Ok(Validated(self.0))
    }

    /// This checks whether the constraints between `step` and `select` in [shape::Shape] and
    /// `value` in [Job] hold. If they do not hold, an error is returned.
    fn check_value_constraints(&self) -> Result<(), Error> {
        let total_value = self.0.jobs
            .iter()
            .fold(0, |acc, wt| acc + wt.value.unwrap_or(0) );

        match self.0.details.shape.select  {
            Select::Random => {
                if total_value >= 101 {
                    return Err(Error::Validation("Provided Job weights exceed 100"));
                }
            },
            Select::Linear |
            Select::Interleave => {
                if total_value >= self.0.details.shape.steps {
                    return Err(
                        Error::Validation("Individual Job steps exceeds Session total"));
                }
            },
        }
        Ok(())
    }
}
impl TryFrom<std::path::PathBuf> for Unvalidated {
    type Error = Error;

    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        let bytes = std::fs::read(value).map_err(|e| Error::Io(e))?;
        let raw_toml: Session = toml::from_slice(bytes.as_slice()).map_err(|e| Error::Parse(e))?;

        Ok(Unvalidated(raw_toml))
    }
}

struct Validated(Session);
impl Validated {
    /// Uses the validated inner data to create a new Session which filters settings from the top
    /// level `details` into the `commands`.
    fn finalize(&mut self) -> Result<Session, Error> {

        // Fill in any blank optional modifiers in the [Job]s.
        let modifiers = self.0.details.modifiers.clone();
        self.0.jobs.iter_mut().for_each(|job| job.fill_in(&modifiers));

        Ok(self.0.clone())
    }
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
