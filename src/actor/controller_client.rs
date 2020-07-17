use acteur::{Serve, Service, ServiceAssistant, ServiceConfiguration};

use crate::{
    actor::{
        messages::{GetUserData, LoadSettings, ResolveToProfileId},
        ResolverClient, UserServiceClient,
    },
    types::{ProfileId, Settings},
};

/// An actor service that is a facade to other services and used to group repetitively
/// used functions in one point.
#[derive(Debug)]
pub struct ControllerClient;

#[async_trait::async_trait]
impl Service for ControllerClient {
    async fn initialize(_: &ServiceAssistant<Self>) -> (Self, ServiceConfiguration) {
        (ControllerClient {}, ServiceConfiguration::default())
    }
}

#[async_trait::async_trait]
impl Serve<GetUserData> for ControllerClient {
    type Response = (Option<ProfileId>, Option<Settings>);
    async fn handle(
        &self,
        message: GetUserData,
        assistant: &acteur::ServiceAssistant<Self>,
    ) -> Self::Response {
        // Don't await both, use something like join!
        let profile_id: Option<ProfileId> = assistant
            .call_service::<ResolverClient, ResolveToProfileId>(message.clone().into())
            .await
            .ok()
            .flatten();

        let settings: Option<Settings> = assistant
            .call_service::<UserServiceClient, LoadSettings>(message.into())
            .await
            .ok();

        (profile_id, settings)
    }
}
