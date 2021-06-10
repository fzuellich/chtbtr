use actix_web::web;

use crate::{
    actor::{messages::SendChatMessage, JustClient},
    controller::error::ControllerError,
    service::NotificationMessageComposer,
    types::{GerritTrigger, PatchStatusChangedData, AppState, OwnerSettings},
};

use super::{
    notification_rules::patch_status::check_notification_settings, util::extract_user_data,
};

pub async fn patch_status_changed(
    trigger: &GerritTrigger,
    state: web::Data<AppState>,
    data: &PatchStatusChangedData,
) -> Result<(), ControllerError> {
    let acteur = state.acteur.clone();
    let owner = &data.base.change_owner;
    let username = &data.base.change_owner_username;

    let (profile_id, settings) = extract_user_data(&acteur, owner, username).await?;
    let settings: OwnerSettings = settings.into();

    check_notification_settings(&settings, data)?;

    let message = NotificationMessageComposer::create().compose(&trigger)?;
    debug!("Message is composed. Dispatching actor message to trigger chat message dispatch.");
    acteur
        .send_to_service::<JustClient, SendChatMessage>(SendChatMessage(profile_id, message))
        .await;

    Ok(())
}
