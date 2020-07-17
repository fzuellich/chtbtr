use actix_web::web;

use super::{notification_rules::reviewer_added::notification_wanted, util::extract_user_data};
use crate::{
    actor::{messages::SendChatMessage, JustClient},
    controller::error::ControllerError,
    types::{AppState, GerritTrigger, ReviewerAddedData, ReviewerSettings},
};

pub async fn reviewer_added(
    _: &GerritTrigger,
    state: web::Data<AppState>,
    data: &ReviewerAddedData,
) -> Result<(), ControllerError> {
    let acteur = state.acteur.clone();
    let _domain = state.connection.gerrit_domain.clone();
    let change_owner = &data.change_owner_username;
    let reviewer_username = &data.reviewer_username;
    let reviewer = &data.reviewer;

    let (reviewer_id, settings) = extract_user_data(&acteur, reviewer, reviewer_username).await?;
    let settings: ReviewerSettings = settings.into();

    notification_wanted(change_owner, reviewer_username, &settings)?;

    let message = format!(
        "You were added as reviewer. https://gerrit.just-ag.com/c/{}/+/{}",
        data.project, data.change_url
    );

    debug!("Message is composed. Dispatching actor message to trigger chat message dispatch.");
    acteur
        .send_to_service::<JustClient, SendChatMessage>(SendChatMessage(reviewer_id, message))
        .await;
    Ok(())
}
