use super::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Job {
    /// The string that describes the 
    #[allow(unused)] //not currently used
    pub cmd: String,

    /// This is a [shape::Select] dependent value. The interpretation of this value is dependent on
    /// the variant selected.
    ///
    /// TODO: Make this an enum or struct?
    pub value: Option<u64>,

    #[serde(flatten)]
    pub modifiers: Modifiers,
} 
impl Job {
    pub fn fill_in(&mut self, modifiers: &Modifiers) {
        self.modifiers.fill_in(modifiers);
    }

    pub fn validate(&self) -> Result<(), CreateError> {
        self.modifiers.validate()?;
        Ok(())
    }

    fn as_command(&self) -> std::process::Command {
        todo!()
    }
} 

#[derive(Clone, Debug, Deserialize)]
pub struct Modifiers {
    /// A optional wrapper command that will be prepended to each [Job]. 
    /// e.g. "time", "strace", "srun -N1"
    #[allow(unused)] //not currently used
    wrapper: Option<String>, // TODO: this should probably be an enum?

    /// Base path that should be used for STDOUT files. This is inherited by all members. This may
    /// be overwritten by the [Job].
    #[allow(unused)] //not currently used
    stdout: Option<PathBuf>,

    /// Base path that should be used for STDERR files. This is inherited by all members. This may
    /// be overwritten by the [Job].
    #[allow(unused)] //not currently used
    stderr: Option<PathBuf>,
} 
impl Modifiers {
    pub fn fill_in(&mut self, other: &Modifiers) {
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

    pub fn validate(&self) -> Result<(), CreateError> {
        Ok(())
    }
}
