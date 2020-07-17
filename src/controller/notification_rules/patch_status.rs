use crate::{
    controller::error::NotificationRuleViolation,
    types::{OwnerSettings, PatchStatus, PatchStatusChangedData, VerifiedStatus},
};

pub fn check_notification_settings(
    settings: &OwnerSettings,
    data: &PatchStatusChangedData,
) -> Result<(), NotificationRuleViolation> {
    if data.author_username == data.base.change_owner_username {
        return Err(NotificationRuleViolation::AuthorAndOwnerAreTheSame);
    }

    let change_owner = data.base.change_owner_username.clone();

    match data.patch_status {
        PatchStatus::Both(_, verified_status) | PatchStatus::Verified(verified_status) => {
            if settings.subscribe_verified == false {
                return Err(NotificationRuleViolation::OwnerNotSubscribedToVerfiedNotification(change_owner));
            }

            if verified_status == VerifiedStatus::None {
                return Err(NotificationRuleViolation::NoPatchStatusSet);
            }
        }
        PatchStatus::ReadyForSubmit => {
            if settings.subscribe_ready_for_submit == false {
                return Err(NotificationRuleViolation::OwnerNotSubscribedToSubmitNotification(change_owner));
            }
        }
        // NOT Supported yet
        PatchStatus::CodeReview(_) => {}
        PatchStatus::None => return Err(NotificationRuleViolation::NoPatchStatusSet),
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gerrit_triggers::{BaseData, PatchStatusChangedData};
    use crate::patch_status::PatchStatus;
    use crate::types::{CodeReviewStatus, VerifiedStatus};
    use crate::user::types::GerritUsername;
    use crate::user::types::OwnerSettings;
    use crate::user::types::ProjectName;

    fn create_owner_settings() -> OwnerSettings {
        OwnerSettings {
            subscribe_comment: true,
            subscribe_verified: false,
            subscribe_ready_for_submit: false,
            subscribe_submitted: true,
            ignore_empty_review_comments: false,
            ignore_by_username: vec![],
            ignore_projects: vec![],
        }
    }

    fn create_patch_status_changed_data() -> PatchStatusChangedData {
        PatchStatusChangedData {
            base: BaseData {
                change_owner: String::from("Firstname Lastname"),
                change_owner_username: GerritUsername::from("change.owner"),
                change_url: String::from("change_url"),
                project: ProjectName::from("project"),
            },
            author_username: GerritUsername::from("author.user"),
            patch_status: PatchStatus::None,
        }
    }

    #[test]
    pub fn no_error_when_verified_plus_one() {
        let settings = OwnerSettings {
            subscribe_verified: true,
            ..create_owner_settings()
        };

        let data = PatchStatusChangedData {
            patch_status: PatchStatus::Verified(VerifiedStatus::PlusOne),
            ..create_patch_status_changed_data()
        };

        let result = check_notification_settings(&settings, &data);
        assert_eq!(true, result.is_ok());
    }

    #[test]
    pub fn fail_when_author_and_owner_are_the_same() {
        let settings = create_owner_settings();
        let data = PatchStatusChangedData {
            author_username: GerritUsername::from("change.owner"),
            patch_status: PatchStatus::Verified(VerifiedStatus::PlusOne),
            ..create_patch_status_changed_data()
        };
        let result = check_notification_settings(&settings, &data);
        assert_eq!(
            NotificationRuleViolation::AuthorAndOwnerAreTheSame,
            result.err().unwrap()
        );
    }

    #[test]
    pub fn fail_when_owner_not_subscribed_to_submit_notification() {
        // GIVEN we are not subscribed to ready for submit
        let settings = create_owner_settings();
        // AND the patch status is ready for submit
        let data = PatchStatusChangedData {
            patch_status: PatchStatus::ReadyForSubmit,
            ..create_patch_status_changed_data()
        };

        let result = check_notification_settings(&settings, &data);

        // THEN an error is generated
        assert_eq!(
            NotificationRuleViolation::OwnerNotSubscribedToSubmitNotification,
            result.err().unwrap()
        );
    }

    #[test]
    pub fn fail_when_owner_not_subscribed_to_verified_notification() {
        // GIVEN we are not subscribed to verified ...
        let settings = create_owner_settings();
        // AND we the PatchStatus changed to Verified +1
        let data = PatchStatusChangedData {
            patch_status: PatchStatus::Verified(VerifiedStatus::PlusOne),
            ..create_patch_status_changed_data()
        };

        let result = check_notification_settings(&settings, &data);

        // THEN an error is generated.
        assert_eq!(
            NotificationRuleViolation::OwnerNotSubscribedToVerfiedNotification,
            result.err().unwrap()
        );
    }

    #[test]
    pub fn fail_when_owner_not_subscribed_to_verified_notification_and_both_changed() {
        // GIVEN we are not subscribed to verified ...
        let settings = create_owner_settings();
        // AND we the PatchStatus changed to Verified +1
        let data = PatchStatusChangedData {
            patch_status: PatchStatus::Both(CodeReviewStatus::PlusOne, VerifiedStatus::PlusOne),
            ..create_patch_status_changed_data()
        };

        let result = check_notification_settings(&settings, &data);

        // THEN an error is generated.
        assert_eq!(
            NotificationRuleViolation::OwnerNotSubscribedToVerfiedNotification,
            result.err().unwrap()
        );
    }

    #[test]
    pub fn fail_when_no_patch_status_is_given() {
        // GIVEN we are subscribed to everything
        let settings = create_owner_settings();
        // AND the PatchStatus is None
        let data = PatchStatusChangedData {
            patch_status: PatchStatus::None,
            ..create_patch_status_changed_data()
        };

        let result = check_notification_settings(&settings, &data);

        // THEN an error is generated.
        assert_eq!(
            NotificationRuleViolation::NoPatchStatusSet,
            result.err().unwrap()
        );
    }

    /// The test covers the Job-retrigger case in Jenkins.
    #[test]
    pub fn fail_when_verified_status_switches_to_none() {
        // GIVEN we are subscribed to verified
        let settings = OwnerSettings {
            subscribe_verified: true,
            ..create_owner_settings()
        };
        // AND the PatchStatus is None
        let data = PatchStatusChangedData {
            patch_status: PatchStatus::Verified(VerifiedStatus::None),
            ..create_patch_status_changed_data()
        };

        let result = check_notification_settings(&settings, &data);

        // THEN an error is generated
        assert_eq!(
            NotificationRuleViolation::NoPatchStatusSet,
            result.err().unwrap()
        );
    }
}
