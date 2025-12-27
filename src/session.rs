///! This modules contains the types used for parsing and validation of a configuration file.
///! These types are analogous to those in [crate::session::Session] but contain "holes" to provide
///! space for details and interactions at different levels.
///! 
///! A configuration file is read in with [read_configuration]. This function is the point of parse
///! and validation. The primary type for this is [Session]. It contains a [Details] and vector of
///! commands ([Job]).

pub mod create;
pub mod details;
pub mod job;
pub mod modifiers;

use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;
//use garde::Validate;  // May revisit this.

//use create::CreationError;
use details::{Details, DetailsConfiguration};
use job::{Jobs, JobsConfiguration};

pub type SessionConfiguration = SessionBase<DetailsConfiguration, JobsConfiguration>;
pub type Session = SessionBase<Details,Jobs>;

#[derive(Clone, Debug, Deserialize)]
pub struct SessionBase<D,J> {
    details: D,

    // This rename is to make the .toml layout nice.
    #[serde(rename = "job")]
    jobs: J,
} 
impl SessionConfiguration {
    //pub fn get_jobs(&self, job: usize) -> Option<&Job> {
        //self.jobs.get(job)
    //}

    //pub fn next_job(&self) -> &Job {
        //todo!()
    //}

}
impl Session { }
