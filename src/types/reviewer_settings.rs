use super::{GerritUsername, ProjectName, Settings, TopicName};
use serde::{Deserialize, Serialize};

/// All settings below apply to you, only when you are a reviewer of a given
/// patch.
#[derive(Clone, Serialize, Deserialize)]
pub struct ReviewerSettings {
    // Be notified when you are added as reviewer
    pub subscribe: bool,

    /*
     * IGNORED. Ignore ALL notifications for these topics, e.g.
     * ["merge-commit"], on patches that you are added as a reviewer.
     *
     * This means you wont receive notifications that you are added as a
     * reviewer, or that the review was updated etc.
     */
    pub ignore_topics: Vec<TopicName>,

    /*
     * IGNORED. Ignore notifications for certain projects.
     * Only active when you are a reviewer for the patch.
     */
    pub ignore_projects: Vec<ProjectName>,

    /*
     * IGNORED. Ignore ALL reviews coming from the given gerrit username.
     */
    pub ignore_by_username: Vec<GerritUsername>,
}

impl From<Settings> for ReviewerSettings {
    fn from(origin: Settings) -> Self {
        match origin {
            Settings::V1 {
                as_reviewer,
                as_owner: _,
            } => as_reviewer,
        }
    }
}
