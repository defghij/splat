use super::*;

use modifiers::{Modifiers, ModifiersConfiguration};
use create::CreationError;


pub type JobConfiguration = JobBase<Option<u64>, ModifiersConfiguration>;
pub type JobsConfiguration = Vec<JobConfiguration>;
pub type Job = JobBase<u64, Modifiers>;
pub type Jobs = Vec<Job>;


#[derive(Clone, Debug, Deserialize)]
pub struct JobBase<V,M> {
    /// The string that describes the 
    #[allow(unused)] //not currently used
    pub cmd: String,

    /// This is a [shape::Select] dependent value. The interpretation of this value is dependent on
    /// the variant selected.
    ///
    /// TODO: Make this an enum or struct?
    pub value: V,

    #[serde(flatten)]
    pub modifiers: M,
} 
impl JobConfiguration {
    pub fn fill_in(&mut self, modifiers: &ModifiersConfiguration) -> &Self {
        self.modifiers.fill_in(modifiers);
        self
    }

    pub fn validate(&self) -> Result<(), CreationError> {
        self.modifiers.validate()?;
        Ok(())
    }

} 
impl Job {
    fn _as_command(&self) -> std::process::Command {
        todo!()
    }

}
impl TryFrom<JobConfiguration> for Job {
    type Error = CreationError;

    fn try_from(jobconfig: JobConfiguration) -> Result<Self, Self::Error> {
        let JobConfiguration { cmd, value, modifiers } = jobconfig;

        let value = if value.is_some() { value.expect("Should be Some by virtue of the conditional") }
        else { return Err(CreationError::Validation("Expected a value but found 'None'")); };

        let modifiers: Modifiers = modifiers.try_into()?;

        Ok(Job { 
            cmd, 
            value, 
            modifiers, 
        })
    }
}

