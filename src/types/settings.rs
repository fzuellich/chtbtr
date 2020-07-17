use super::{OwnerSettings, ReviewerSettings};
use serde::{Deserialize, Serialize};
/*
 * Should be loaded on each interaction. I guess it would be more expensive to
 * generate a hash of the file content and compare it with an existing has, than
 * to simply read the file.
 */
#[derive(Clone, Serialize, Deserialize)]
pub enum Settings {
    V1 {
        as_reviewer: ReviewerSettings,
        as_owner: OwnerSettings,
    },
}

#[cfg(test)]
mod tests {
    use super::Settings;
    use crate::types::{OwnerSettings, ReviewerSettings};

    #[test]
    fn test_settings() {
        Settings::V1 {
            as_reviewer: ReviewerSettings {
                subscribe: false,
                ignore_projects: vec![],
                ignore_topics: vec![TopicName::from("merge-commit")],
                ignore_by_username: vec![
                    GerritUsername::from("tools.just"),
                    GerritUsername::from("ec2.gerrit"),
                ],
            },
            as_owner: OwnerSettings {
                subscribe_comment: false,
                subscribe_verified: false,
                subscribe_ready_for_submit: false,
                subscribe_submitted: false,
                ignore_by_username: vec![
                    GerritUsername::from("a.user"),
                    GerritUsername::from("another.user"),
                ],
                ignore_empty_review_comments: false,
                ignore_projects: vec![],
            },
        };
    }
}
