use super::*;

use modifiers::{ ModifiersConfiguration, Modifiers};
use create::CreationError;

pub type DetailsConfiguration = DetailBase<ModifiersConfiguration>;
pub type Details = DetailBase<Modifiers>;

#[derive(Clone, Debug, Deserialize)]
pub struct DetailBase<M> {
    /// Name for the [Details]
    pub name: String,

    /// Whether the base program should emit a logging file.
    #[serde(default)]
    #[allow(unused)] //not currently used
    pub logging: bool,

    /// Options to modify the commands provides in [Job]s of the [Session]. 
    #[serde(flatten)]
    pub modifiers: M,

    /// Defines the number and order of [Jobs] launched in a Session.
    pub shape: Shape,
} 
impl DetailsConfiguration {
    /// Validates basic features of the [Details] that do not depend on any external structure.
    /// Currently, these validations are just "is it ascii?". This may change.
    pub fn validate(&self) -> Result<(), CreationError> {
        tracing::debug!("Doing validation on the configuration of the session details");
        if !self.name.is_ascii() { 
            return Err(CreationError::Validation("Session name must be ascii"));
        }

        self.modifiers.validate()?;
        self.shape.validate()?;
        Ok(())
    }
}
impl TryFrom<DetailsConfiguration> for Details {
    type Error = CreationError;

    fn try_from(value: DetailsConfiguration) -> Result<Self, Self::Error> {
        if !value.name.is_ascii() { 
            return Err(CreationError::Validation("Session name must be ascii"));
        }

        value.modifiers.validate()?;
        value.shape.validate()?;

        Ok(Details { 
            name:      value.name,
            logging:   value.logging,
            modifiers: value.modifiers.validate()?.clone().try_into()?,
            shape:     value.shape.validate()?.clone()
        })
    }
}




/// The ways in which the next job may be selected.
///
#[derive(Clone, Debug, Deserialize)]
pub enum Select {
    /// Randomly select the next job from the provided jobs in the [Session].
    /// The probability that the next step will belong to any particular [Job] is determined by
    /// that [Job]. Weights should total 100.
    ///
    /// TODO: If no weight is provided for a [Job], its weight defaults to an even split of the
    /// remaining probability. That is if there are three jobs where only one is given a weight 
    /// of 50 then the remaining two jobs will have a probability of 25 for a total of 100.
    Random,

    /// The next step should be pulled from the current [Job] unless its step count is
    /// exhausted. If so, pull from the next [Job] in the [Session]. The steps from the next job
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
} 
impl Default for Select {
    fn default() -> Self { Select::Linear }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Shape {
    /// The method of step/job selection for this [Session].
    #[serde(default)]
    pub select: Select, 
    
    /// The total steps in this [Session]. Note, this may be overridden by the [Job]. 
    pub steps: u64,

    /// The number of steps that should be active at any particular point in time. 
    pub parallel: u64,
} 
impl Shape {
    pub fn validate(&self) -> Result<&Self, CreationError> {
        if self.steps == 0 || self.parallel == 0{
            return Err(CreationError::Validation("`steps` and `parallel` must both be greater than zero"));
        }
        if self.steps < self.parallel {
            return Err(CreationError::Validation("`parallel` may not exceed `steps`"));
        }
        Ok(self)
    }
}
