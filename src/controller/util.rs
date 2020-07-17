use acteur::Acteur;

use super::error::ControllerError;
use crate::{
    actor::{messages::GetUserData, ControllerClient},
    types::{GerritUsername, ProfileId, Settings},
};

// TODO Fix error handling.
pub async fn extract_user_data(
    acteur: &Acteur,
    change_owner: &str,
    change_owner_username: &GerritUsername,
) -> Result<(ProfileId, Settings), ControllerError> {
    let get_user_data = GetUserData(change_owner_username.clone(), change_owner.to_string());

    let (profile_id, settings): (Option<ProfileId>, Option<Settings>) = acteur
        .call_service::<ControllerClient, _>(get_user_data)
        .await
        .expect("Error when calling ControllerClient actor to receive user data.");

    if profile_id.is_none() {
        warn!(
            "ControllerClient couldn't find a ProfileId for gerrit user '{}'. CommentAddedController is dropping comment notification.",
            change_owner_username
        );

        return Err(ControllerError::UserMappingError(
            "No profile id found. More information in log.".to_string(),
        ));
    }

    if settings.is_none() {
        warn!("ControllerClient couldn't find settings for gerrit user '{}'. CommentAddedController is dropping comment notification.", change_owner_username);

        return Err(ControllerError::UserMappingError(
            "No user settings found. More information in log.".to_string(),
        ));
    }

    Ok((profile_id.unwrap(), settings.unwrap()))
}
