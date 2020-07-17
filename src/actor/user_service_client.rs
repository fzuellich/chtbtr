use crate::{
    actor::{
        messages::{GetAppState, InitializeCache, LoadSettings, SetProfileIdMapping},
        AppState,
    },
    types::{ConnectionParameters, GerritUsername, ProfileId, Settings, Synchronization},
    service::{FileBackedUserService, UserService},
};
use acteur::{Listen, Serve, Service, ServiceAssistant, ServiceConfiguration};
use std::{collections::HashMap, sync::Mutex};

///
/// The service is responsible for common interactions with user objects.
///
#[derive(Debug)]
pub struct UserServiceClient {
    service: Mutex<FileBackedUserService>,
}

#[async_trait::async_trait]
impl Service for UserServiceClient {
    async fn initialize(system: &ServiceAssistant<Self>) -> (Self, ServiceConfiguration) {
        let app_state: ConnectionParameters = system
            .call_actor::<AppState, _>(0, GetAppState {})
            .await
            .expect("AppState couldn't be retrieved.");
        let service = FileBackedUserService {
            data_dir: app_state.data_dir.clone(),
        };

        (
            UserServiceClient {
                service: Mutex::new(service),
            },
            ServiceConfiguration::default(),
        )
    }
}

#[async_trait::async_trait]
impl Serve<InitializeCache> for UserServiceClient {
    type Response = HashMap<GerritUsername, Synchronization<ProfileId>>;

    async fn handle(&self, _: InitializeCache, _: &ServiceAssistant<Self>) -> Self::Response {
        let instance = self.service.lock().unwrap();
        return instance.load_sync_cache();
    }
}

#[async_trait::async_trait]
impl Serve<LoadSettings> for UserServiceClient {
    type Response = Settings;

    /// Implementation of `LoadSettings`.
    ///
    /// If no service instance is initialized:
    /// * The actor will log a warning.
    /// * The actor will return default settings.
    async fn handle(&self, message: LoadSettings, _: &ServiceAssistant<Self>) -> Self::Response {
        let instance: &FileBackedUserService = &*self.service.lock().unwrap();
        instance.load_settings(&message.0).unwrap_or_else(|_| {
            warn!(
                "Couldn't retrieve settings for user '{}'. Return default settings instead.",
                message.0
            );
            panic!("No default settings implemented yet.")
        })
    }
}

#[async_trait::async_trait]
impl Listen<SetProfileIdMapping> for UserServiceClient {
    async fn handle(&self, message: SetProfileIdMapping, _: &ServiceAssistant<Self>) {
        debug!(
            "Writing profile id mapping for '{}' => '{:?}'",
            &message.0, &message.1
        );
        let instance: &FileBackedUserService = &*self.service.lock().unwrap();
        let result = instance.set_sync(&message.0, &message.1);
        if result.is_err() {
            warn!(
                "Couldn't set profile id mapping for '{}' in actor. Cause: {}",
                message.0,
                result.unwrap_err()
            );
        }
    }
}
