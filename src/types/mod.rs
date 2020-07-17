mod app_state;
mod code_review_status;
mod connection_parameters;
mod conversation_id;
mod gerrit_triggers;
mod owner_settings;
mod patch_status;
mod path_to_user_data;
mod profile_id;
mod reviewer_settings;
mod settings;
mod synchronization;
mod verified_status;

pub use self::app_state::AppState;
pub use self::code_review_status::CodeReviewStatus;
pub use self::connection_parameters::ConnectionParameters;
pub use self::conversation_id::ConversationId;
pub use self::gerrit_triggers::{
    BaseData, CommentAddedData, GerritTrigger, PatchStatusChangedData, ReviewerAddedData,
};
pub use self::owner_settings::OwnerSettings;
pub use self::patch_status::{patch_status, PatchStatus};
pub use self::path_to_user_data::PathToUserData;
pub use self::profile_id::ProfileId;
pub use self::reviewer_settings::ReviewerSettings;
pub use self::settings::Settings;
pub use self::synchronization::Synchronization;
pub use self::verified_status::VerifiedStatus;

use serde::{Deserialize, Serialize};
use std::fmt;

#[macro_use]
mod macros {
    /// Implements the std::convert::From trait for a struct. The struct can
    /// only have one field of type String. The implementation allows to call
    /// from(&str), where no special parsing is required.
    macro_rules! from_for_string_struct {
        ($type:ident) => {
            impl From<&str> for $type {
                fn from(value: &str) -> $type {
                    $type {
                        0: String::from(value),
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicName(pub String);

from_for_string_struct!(TopicName);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectName(pub String);

impl fmt::Display for ProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

from_for_string_struct!(ProjectName);

#[derive(PartialEq, Eq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct GerritUsername(pub String);

from_for_string_struct!(GerritUsername);

impl fmt::Display for GerritUsername {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
