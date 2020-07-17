use crate::types::{GerritUsername, ProjectName};
use std::error;
use std::fmt;

// TODO declare and use: type Result<S> = Result<S, ControllerError>

#[derive(Clone, Debug, PartialEq)]
pub enum NotificationRuleViolation {
    AuthorAndOwnerAreTheSame,
    NoPatchStatusSet,

    /// Which owner is not subscribed
    OwnerNotSubscribedToComments(GerritUsername),

    /// Which owner ignores which users comments?
    OwnerIgnoresCommentsByUser(GerritUsername, GerritUsername),

    /// Which owner ignores what project?
    OwnerIgnoresCommentsForProject(GerritUsername, ProjectName),

    /// Which owner is not subscribed to a submit notification?
    OwnerNotSubscribedToSubmitNotification(GerritUsername),

    /// Which owner is not subscribed?
    OwnerNotSubscribedToVerfiedNotification(GerritUsername),

    /// Which reviewer is not subscribed to notifications?
    ReviewerNotSubscribedToNotification(GerritUsername),

    /// Which reviewer ignores what change owner?
    ReviewerIgnoresReviewsByChangeOwner(GerritUsername, GerritUsername),
}

#[derive(Debug, Clone)]
pub enum ControllerError {
    RuleViolation(NotificationRuleViolation),
    Unrecoverable(String),
    Unspecified(String),
    UserMappingError(String),
}

impl fmt::Display for ControllerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ControllerError::Unspecified(message) => write!(f, "Error in controller: {}", message),
            ControllerError::Unrecoverable(message) => {
                write!(f, "Error in controller: {}", message)
            }
            ControllerError::RuleViolation(reason) => {
                write!(f, "Notification wasn't send due to settings: {}", reason)
            }
            ControllerError::UserMappingError(reason) => {
                write!(f, "Couldn't retrieve user information: {}", reason)
            }
        }
    }
}

impl error::Error for ControllerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<String> for ControllerError {
    fn from(err: String) -> ControllerError {
        ControllerError::Unspecified(err)
    }
}

impl From<NotificationRuleViolation> for ControllerError {
    fn from(err: NotificationRuleViolation) -> ControllerError {
        ControllerError::RuleViolation(err)
    }
}

impl fmt::Display for NotificationRuleViolation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            NotificationRuleViolation::AuthorAndOwnerAreTheSame => "Author and owner are the same.".to_string(),
            NotificationRuleViolation::NoPatchStatusSet => "No patch status set.".to_string(),
            NotificationRuleViolation::OwnerNotSubscribedToComments(owner) => format!("{} is not subscribed to comments.", owner),
            NotificationRuleViolation::OwnerIgnoresCommentsByUser(owner, user) => format!("{} ignores comments by {}.", owner, user),
            NotificationRuleViolation::OwnerIgnoresCommentsForProject(owner, project) => format!("{} ignores comments for project {}.", owner, project),
            NotificationRuleViolation::OwnerNotSubscribedToSubmitNotification(owner) => format!("{} ignores submit notifications.", owner),
            NotificationRuleViolation::OwnerNotSubscribedToVerfiedNotification(owner) => format!("{} ignores verified notifications.", owner),
            NotificationRuleViolation::ReviewerNotSubscribedToNotification(reviewer) => format!("{} ignores notifications to reviews.", reviewer),
            NotificationRuleViolation::ReviewerIgnoresReviewsByChangeOwner(reviewer, owner) => format!("{} ignores reviews from {}.", reviewer, owner),
        };

        write!(f, "{}", message)
    }
}

impl From<()> for ControllerError {
    fn from(_err: ()) -> ControllerError {
        ControllerError::Unspecified(String::from("Unknown error occurred."))
    }
}

impl From<actix::prelude::MailboxError> for ControllerError {
    fn from(err: actix::prelude::MailboxError) -> ControllerError {
        ControllerError::Unspecified(format!(
            "There was a MailboxError. Perhaps too many requests? Cause: {}.",
            err
        ))
    }
}
