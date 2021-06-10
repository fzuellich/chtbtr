use crate::types::{BaseData, CommentAddedData, GerritTrigger, PatchStatus, VerifiedStatus};

pub struct NotificationMessageComposer {
}

impl NotificationMessageComposer {
    pub fn create() -> NotificationMessageComposer {
        NotificationMessageComposer { }
    }

    fn generate_patch_url<'a>(&self, base: &'a BaseData) -> String {
        base.change_url.clone()
    }

    fn compose_verified_message(&self, verified: &VerifiedStatus, data: &BaseData) -> String {
        // Feels wrong, remove &Verified...
        if &VerifiedStatus::None == verified {
            return String::from("Crazy logic error. Don't tell anyone about MetallicSheep.");
        }

        let emoji = match verified {
            VerifiedStatus::PlusOne => "🌈",  // Rainbow
            VerifiedStatus::MinusOne => "😰", // FACE_WITH_COLD_SWEAT.
            VerifiedStatus::None => "",
        };

        format!(
            "{} Verified for your patch {} {}.",
            verified,
            emoji,
            self.generate_patch_url(&data),
        )
    }

    fn compose_ready_for_submit_message(&self, data: &BaseData) -> String {
        format!(
            "☑️ A patch is ready to submit! ✨ {}", // Sparkles + Checkbox
            self.generate_patch_url(data)
        )
    }

    fn compose_comment_added_message(&self, data: &CommentAddedData) -> String {
        format!(
            "Comment was added by {}. 💬 {}",
            data.author_username,
            self.generate_patch_url(&data.base)
        )
    }

    pub fn compose<'a>(&self, value: &'a GerritTrigger) -> Result<String, ()> {
        match value {
            GerritTrigger::CommentAdded(data) => Ok(self.compose_comment_added_message(data)),
            GerritTrigger::PatchStatusChanged(data) => match &data.patch_status {
                PatchStatus::Both(_, value) | PatchStatus::Verified(value) => {
                    Ok(self.compose_verified_message(value, &data.base))
                }
                PatchStatus::ReadyForSubmit => {
                    Ok(self.compose_ready_for_submit_message(&data.base))
                }
                _ => {
                    debug!(
                        "Can't compose message for patch status {:?}.",
                        &data.patch_status
                    );
                    Err(())
                }
            },
            _ => {
                debug!("Can't compose message for type {:?}.", value);
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::NotificationMessageComposer;
    use super::VerifiedStatus;
    use crate::gerrit_triggers::GerritTrigger;
    use crate::gerrit_triggers::{BaseData, CommentAddedData, PatchStatusChangedData};
    use crate::patch_status::PatchStatus;
    use crate::user::types::{GerritUsername, ProjectName};

    #[test]
    fn notification_message_for_verified() {
        let base = BaseData {
            change_owner: String::from("change_owner"),
            change_owner_username: GerritUsername::from("change.owner"),
            change_url: String::from("2"),
            project: ProjectName::from("prj"),
        };

        let composer = NotificationMessageComposer::create(String::from("domain"));
        let message_minus_one =
            composer.compose(&GerritTrigger::PatchStatusChanged(PatchStatusChangedData {
                base: base.clone(),
                author_username: GerritUsername::from("author.username"),
                patch_status: PatchStatus::Verified(VerifiedStatus::MinusOne),
            }));
        assert_eq!(
            "-1 Verified for your patch 😰 https://domain/c/prj/+/2.",
            message_minus_one.unwrap()
        );

        let message_plus_one =
            composer.compose(&GerritTrigger::PatchStatusChanged(PatchStatusChangedData {
                base: base.clone(),
                author_username: GerritUsername::from("author.username"),
                patch_status: PatchStatus::Verified(VerifiedStatus::PlusOne),
            }));
        assert_eq!(
            "+1 Verified for your patch 🌈 https://domain/c/prj/+/2.",
            message_plus_one.unwrap()
        );
        let message_none =
            composer.compose(&GerritTrigger::PatchStatusChanged(PatchStatusChangedData {
                base: base.clone(),
                author_username: GerritUsername::from("author.username"),
                patch_status: PatchStatus::Verified(VerifiedStatus::None),
            }));
        assert_eq!(
            "Crazy logic error. Don't tell anyone about MetallicSheep.",
            message_none.unwrap()
        );
    }

    #[test]
    fn notification_message_for_ready_for_submit() {
        let base = BaseData {
            change_owner: String::from("change_owner"),
            change_owner_username: GerritUsername::from("change.owner"),
            change_url: String::from("2"),
            project: ProjectName::from("prj"),
        };

        let message = NotificationMessageComposer::create(String::from("domain")).compose(
            &GerritTrigger::PatchStatusChanged(PatchStatusChangedData {
                base: base.clone(),
                author_username: GerritUsername::from("author.username"),
                patch_status: PatchStatus::ReadyForSubmit,
            }),
        );
        assert_eq!(
            "☑️ A patch is ready to submit! ✨ https://domain/c/prj/+/2",
            message.unwrap()
        );
    }

    #[test]
    fn test_1_is_plus_one() {
        let tests = ["1", " 1", "1 ", " 1 "];
        for test_str in tests.iter() {
            let result = VerifiedStatus::from(*test_str);
            assert_eq!(
                result,
                VerifiedStatus::PlusOne,
                "VerifiedStatus::from(\"{}\") != VerifiedStatus::PlusOne",
                test_str
            );
        }
    }

    #[test]
    fn test_minus_1_is_minus_one() {
        let tests = ["-1", " -1", "-1 ", " -1 "];
        for test_str in tests.iter() {
            let result = VerifiedStatus::from(*test_str);
            assert_eq!(
                result,
                VerifiedStatus::MinusOne,
                "VerifiedStatus::from(\"{}\") != VerifiedStatus::MinusOne",
                test_str
            );
        }
    }

    #[test]
    fn test_others_are_none() {
        let tests = [
            "garbage", "-12", "-2", "+2", "2", "4", "-other", " 2", " other ",
        ];
        for test_str in tests.iter() {
            let result = VerifiedStatus::from(*test_str);
            assert_eq!(
                result,
                VerifiedStatus::None,
                "VerifiedStatus::from(\"{}\") != VerifiedStatus::None",
                test_str
            );
        }
    }

    #[test]
    fn test_comment_added_notification() {
        // 💬
        let message = NotificationMessageComposer::create(String::from("gerrit.domain")).compose(
            &GerritTrigger::CommentAdded(CommentAddedData {
                base: BaseData {
                    change_owner: String::from("change_owner"),
                    change_owner_username: GerritUsername::from("change.owner"),
                    change_url: String::from("2"),
                    project: ProjectName::from("prj"),
                },
                author: String::from("author lastname <author email>"),
                author_username: GerritUsername::from("author"),
            }),
        );
        assert_eq!(
            message.unwrap(),
            "Comment was added by author. 💬 https://gerrit.domain/c/prj/+/2"
        );
    }
}
