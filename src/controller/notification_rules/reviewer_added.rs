use crate::{
    controller::error::NotificationRuleViolation,
    types::{GerritUsername, ReviewerSettings},
};

type IResult = Result<(), NotificationRuleViolation>;

pub fn notification_wanted(
    change_owner: &GerritUsername,
    reviewer: &GerritUsername,
    settings: &ReviewerSettings,
) -> IResult {
    if settings.subscribe == false {
        return Err(NotificationRuleViolation::ReviewerNotSubscribedToNotification(reviewer.clone()));
    }

    let ignore_reviews_by_owner = settings.ignore_by_username.contains(change_owner);
    if ignore_reviews_by_owner {
        return Err(
            NotificationRuleViolation::ReviewerIgnoresReviewsByChangeOwner(
                reviewer.clone(),
                change_owner.clone(),
            ),
        );
    }

    Ok(())
}
