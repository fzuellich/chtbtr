use acteur::{Serve, Service, ServiceAssistant, ServiceConfiguration};
use futures::lock::{Mutex, MutexGuard};
use std::collections::HashMap;

use crate::{
    actor::{
        messages::{GetAppState, InitializeCache, ResolveToProfileId},
        AppState, UserServiceClient,
    },
    types::{GerritUsername, ProfileId, Synchronization},
    service::{ProfileIdResolver, ResolverService},
};

#[derive(Debug)]
pub struct ResolverClient(Mutex<ProfileIdResolver>);

#[async_trait::async_trait]
impl Service for ResolverClient {
    async fn initialize(system: &ServiceAssistant<Self>) -> (Self, ServiceConfiguration) {
        let state = system
            .call_actor::<AppState, GetAppState>(0, GetAppState {})
            .await
            .expect("Couldn't retrieve application state.");
        let cache: HashMap<GerritUsername, Synchronization<ProfileId>> = system
            .call_service::<UserServiceClient, InitializeCache>(InitializeCache(state.data_dir))
            .await
            .unwrap_or_else(|_| {
                warn!("ResolverClient could not get the sync data. Using empty HashMap. Performance may suffer.");
                HashMap::new()
            });

        let service = Mutex::new(ProfileIdResolver {
            cache,
            acteur: system.clone(),
        });

        (ResolverClient(service), ServiceConfiguration::default())
    }
}

#[async_trait::async_trait]
impl Serve<ResolveToProfileId> for ResolverClient {
    type Response = Option<ProfileId>;

    async fn handle(
        &self,
        message: ResolveToProfileId,
        _system: &ServiceAssistant<Self>,
    ) -> Self::Response {
        let mut instance: MutexGuard<ProfileIdResolver> = self.0.lock().await;
        let fut = instance.resolve(&message.0, &message.1).await;
        fut.ok().flatten()
    }
}
