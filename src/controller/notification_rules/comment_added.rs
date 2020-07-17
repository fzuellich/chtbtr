use crate::{
    controller::error::NotificationRuleViolation,
    types::{CommentAddedData, OwnerSettings},
};

type IResult = Result<(), NotificationRuleViolation>;

/**
 * Only performs checks to verify that a comment notification should be created.
 */
pub fn notification_wanted(comment: &CommentAddedData, settings: &OwnerSettings) -> IResult {
    let author = &comment.author_username;
    let owner = &comment.base.change_owner_username;

    if owner == author {
        return Err(NotificationRuleViolation::AuthorAndOwnerAreTheSame);
    }

    if settings.subscribe_comment == false {
        return Err(NotificationRuleViolation::OwnerNotSubscribedToComments(owner.clone()));
    }

    if settings.ignore_by_username.contains(author) {
        return Err(NotificationRuleViolation::OwnerIgnoresCommentsByUser(owner.clone(), author.clone()));
    }

    if settings.ignore_projects.contains(&comment.base.project) {
        return Err(NotificationRuleViolation::OwnerIgnoresCommentsForProject(owner.clone(), comment.base.project.clone()));
    }

    return Ok(());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gerrit_triggers::BaseData;
    use crate::user::types::{GerritUsername, ProjectName};

    fn create_comment_added_data() -> CommentAddedData {
        CommentAddedData {
            base: BaseData {
                change_owner: String::from("Firstname Lastname"),
                change_owner_username: GerritUsername::from("change.owner"),
                change_url: String::from("change_url"),
                project: ProjectName::from("project"),
            },
            author: String::from("Firstname Lastname"),
            author_username: GerritUsername::from("comment.author"),
        }
    }

    fn create_owner_settings() -> OwnerSettings {
        OwnerSettings {
            subscribe_comment: true,
            subscribe_verified: false,
            subscribe_ready_for_submit: false,
            subscribe_submitted: false,
            ignore_empty_review_comments: false,
            ignore_by_username: vec![],
            ignore_projects: vec![],
        }
    }

    #[test]
    pub fn skip_when_author_and_owner_the_same() {
        let comment = CommentAddedData {
            author_username: GerritUsername::from("change.owner"),
            ..create_comment_added_data()
        };
        let settings = create_owner_settings();
        let result = notification_wanted(&comment, &settings);
        assert_eq!(
            result.err().unwrap(),
            NotificationRuleViolation::AuthorAndOwnerAreTheSame
        );
    }

    #[test]
    pub fn skip_when_change_owner_not_subscribed_to_comments() {
        let comment = create_comment_added_data();
        let settings = OwnerSettings {
            subscribe_comment: false,
            ..create_owner_settings()
        };
        let result = notification_wanted(&comment, &settings);
        assert_eq!(
            result.err().unwrap(),
            NotificationRuleViolation::OwnerNotSubscribedToComments
        );
    }

    #[test]
    pub fn skip_when_change_owner_ignores_user() {
        let comment = create_comment_added_data();
        let settings = OwnerSettings {
            ignore_by_username: vec![GerritUsername::from("comment.author")],
            ..create_owner_settings()
        };
        let result = notification_wanted(&comment, &settings);
        assert_eq!(
            result.err().unwrap(),
            NotificationRuleViolation::OwnerIgnoresCommentsByUser
        );
    }

    #[test]
    pub fn skip_when_change_owner_ignores_project() {
        let comment = create_comment_added_data();
        let settings = OwnerSettings {
            ignore_projects: vec![ProjectName::from("project")],
            ..create_owner_settings()
        };
        let result = notification_wanted(&comment, &settings);
        assert_eq!(
            result.err().unwrap(),
            NotificationRuleViolation::OwnerIgnoresCommentsForProject
        );
    }
}
