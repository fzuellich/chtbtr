use actix_web::web;

use crate::{
    actor::{messages::SendChatMessage, JustClient},
    controller::error::ControllerError,
    service::NotificationMessageComposer,
    types::{AppState, CommentAddedData, GerritTrigger, OwnerSettings},
};

use super::{notification_rules::comment_added::notification_wanted, util::extract_user_data};

pub async fn comment_added_rewrite(
    trigger: &GerritTrigger,
    comment: &CommentAddedData,
    state: web::Data<AppState>,
) -> Result<(), ControllerError> {
    let acteur = &state.acteur;
    let owner = &comment.base.change_owner;
    let username = &comment.base.change_owner_username;

    let (profile_id, settings) = extract_user_data(acteur, owner, username).await?;
    let settings: OwnerSettings = settings.into();

    notification_wanted(comment, &settings)?;

    debug!("Rule check for comment notification was passed.");
    let message = NotificationMessageComposer::create().compose(trigger)?;
    debug!("Message is composed. Dispatching actor message to trigger chat message dispatch.");
    acteur
        .send_to_service::<JustClient, SendChatMessage>(SendChatMessage(profile_id, message))
        .await;
    Ok(())
}
