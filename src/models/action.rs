/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Action : Actions show the results and progress of asynchronous requests to the API.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Action {
    /// Command executed in the Action
    #[serde(rename = "command")]
    pub command: String,
    #[serde(rename = "error")]
    pub error: Option<Box<crate::models::Error>>,
    /// Point in time when the Action was finished (in ISO-8601 format). Only set if the Action is finished otherwise null.
    #[serde(rename = "finished")]
    pub finished: Option<String>,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i32,
    /// Progress of Action in percent
    #[serde(rename = "progress")]
    pub progress: f32,
    /// Resources the Action relates to
    #[serde(rename = "resources")]
    pub resources: Vec<crate::models::Resource>,
    /// Point in time when the Action was started (in ISO-8601 format)
    #[serde(rename = "started")]
    pub started: String,
    /// Status of the Action
    #[serde(rename = "status")]
    pub status: Status,
}

impl Action {
    /// Actions show the results and progress of asynchronous requests to the API.
    pub fn new(
        command: String,
        error: Option<crate::models::Error>,
        finished: Option<String>,
        id: i32,
        progress: f32,
        resources: Vec<crate::models::Resource>,
        started: String,
        status: Status,
    ) -> Action {
        Action {
            command,
            error: Box::new(error),
            finished,
            id,
            progress,
            resources,
            started,
            status,
        }
    }
}

/// Status of the Action
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "success")]
    Success,
}

impl Default for Status {
    fn default() -> Status {
        Self::Error
    }
}
