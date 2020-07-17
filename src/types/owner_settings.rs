use super::{GerritUsername, ProjectName, Settings};
use serde::{Deserialize, Serialize};

/**
 * All settings below apply to you, only when you are the owner of a given patch.
 */
#[derive(Clone, Serialize, Deserialize)]
pub struct OwnerSettings {
    // Be notified when someone comments on your review
    pub subscribe_comment: bool,

    // IGNORED. Be notified about changes to the Verified status. You'll
    // receive notifications when your patch receives -1 or +1
    pub subscribe_verified: bool,

    // IGNORED. Be notified if a review can be submitted.
    pub subscribe_ready_for_submit: bool,

    // IGNORED. BE notified if someone submits your review.
    pub subscribe_submitted: bool,

    /*
     * IGNORED. Ignore comments that only mention a change to the review status.
     * In other words: if a reviewer sets +1/-1/+2/-2 wihtout writing a comment,
     * you won't receive a notification.
     */
    pub ignore_empty_review_comments: bool,

    /*
     * IGNORED. Ignore ALL comments by the given gerrit username.
     */
    pub ignore_by_username: Vec<GerritUsername>,

    /*
     * IGNORED. Ignore notifications for certain projects. Only when your
     * are the owner of the patch in the specified project.
     */
    pub ignore_projects: Vec<ProjectName>,
}

impl From<Settings> for OwnerSettings {
    fn from(origin: Settings) -> Self {
        match origin {
            Settings::V1 {
                as_reviewer: _,
                as_owner,
            } => as_owner,
        }
    }
}
