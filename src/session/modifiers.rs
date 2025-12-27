use super::*;
use create::CreationError;

pub type ModifiersConfiguration = ModifiersBase<Option<String>, Option<Output>>;
pub type Modifiers = ModifiersBase<Option<String>, Output>;

/// This is a structure that contains modifiers as optional that can be used during the creation of
/// the [Session] from a configuration file wherein some modifiers may not be set so as to inherit
/// from the parent in [Details].
#[derive(Clone, Debug, Deserialize)]
pub struct ModifiersBase<W,P> {
    /// A optional wrapper command that will be prepended to each [Job]. 
    /// e.g. "time", "strace", "srun -N1"
    wrapper: W, // TODO: this should probably be an enum?

    /// Base path that should be used for STDOUT files. This is inherited by all members. This may
    /// be overwritten by the [Job].
    stdout: P, // TODO: this should probably be an enum? 

    /// Base path that should be used for STDERR files. This is inherited by all members. This may
    /// be overwritten by the [Job].
    stderr: P,  // TODO: this should probably be an enum? 
}

impl ModifiersConfiguration {
    pub fn fill_in(&mut self, other: &ModifiersConfiguration) {
        if self.wrapper.is_none() && other.wrapper.is_some() {
            self.wrapper = other.wrapper.clone();
        }
        if self.stdout.is_none() && other.stdout.is_some() {
            self.stdout = other.stdout.clone();
        }
        if self.stderr.is_none() && other.stderr.is_some() {
            self.stderr = other.stderr.clone();
        }
    }

    pub fn validate(&self) -> Result<&Self, CreationError> {
        Ok(self)
    }
} 
impl Default for ModifiersConfiguration {
    fn default() -> Self {
        ModifiersConfiguration {
            wrapper: None,
            stdout: None,
            stderr: None,
        }
    }
}

impl TryFrom<ModifiersConfiguration> for Modifiers {
    type Error = CreationError;

    fn try_from(opt_mod: ModifiersConfiguration) -> Result<Self, Self::Error> {
        opt_mod.validate()?;

        let ModifiersConfiguration {wrapper, stdout, stderr} = opt_mod;

        let stdout = if stdout.is_none() { Output::None } 
        else { stdout.expect("Should be some by virtue of conditional") };

        let stderr = if stderr.is_none() { Output::None } 
        else { stderr.expect("Should be some by virtue of conditional") };

        Ok(Modifiers { 
            wrapper,
            stdout, 
            stderr,
        })
    }
}

/// Enumeration of the ways in which the output from launched applications can be returned.
#[derive(Clone, Debug, Deserialize)]
pub enum Output {
    /// The output should only be written to a file. No output will be displayed to the terminal.
    #[serde(rename = "file")]
    #[allow(unused)] // Not currently read
    File(PathBuf),

    /// Output should only be displayed in the terminal. No output files will be written. 
    #[serde(rename = "terminal")]
    Piped,

    /// Display both to the terminal and write to a file.
    #[serde(rename = "both")]
    #[allow(unused)] // Not currently read
    Both(PathBuf),

    /// No output whatsoever is captured.
    #[serde(rename = "none")]
    None,
} 
