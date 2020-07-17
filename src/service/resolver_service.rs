use acteur::ServiceAssistant;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::{
    actor::{
        messages::{SearchProfileId, SetProfileIdMapping},
        JustClient, ResolverClient, UserServiceClient,
    },
    types::{GerritUsername, ProfileId, Synchronization},
};

/// Resolver maps a username or name to a ProfileId and caches the result.
///
/// Expects to be initialized with a cache. Resolver will automatically try to
/// resolve unavailable mappings using the Just API.
///
/// A mapping is represented by three states:
///
/// * A mapping has been found.
/// * A mapping can't be resolved (i.e. no ProfileId was found).
/// * A mapping is unavailable and no attempt has been made to resolve it.
#[async_trait]
pub trait ResolverService {
    /// Resolver will go through a three step process to map a username to a
    /// ProfileId.
    ///
    ///   1. Look up the username in the cache and return the saved status.
    ///   2. Look up the mapping from disk. We do this to ensure that a manual
    ///      change on the disk can be discovered.
    ///   3. Make a request to the Just API trying different combinations of the
    ///      name associated with the Gerrit account. We do this in case no name
    ///      is found, e.g. because there is an unsupported Umlaut or too many
    ///      users are found for only part of the name.
    ///
    /// The result of the resolution process will be saved to disk. In case of an error
    /// we don't touch the disk and return the error.
    async fn resolve(
        &mut self,
        username: &GerritUsername,
        name: &str,
    ) -> Result<Option<ProfileId>, String>;
}

#[derive(Debug)]
pub struct ProfileIdResolver {
    // just_api_actor: Addr<JustApiActor>,
    pub cache: HashMap<GerritUsername, Synchronization<ProfileId>>,
    pub acteur: ServiceAssistant<ResolverClient>,
}

#[async_trait]
impl ResolverService for ProfileIdResolver {
    async fn resolve(
        &mut self,
        username: &GerritUsername,
        name: &str,
    ) -> Result<Option<ProfileId>, String> {
        let result = self.lookup_cache(username);
        debug!("Looking up {} from cache. Result: {:?}.", username, result);
        match result {
            Synchronization::Some(v) => Ok(Some(v)),
            Synchronization::None => Ok(None),
            Synchronization::NotMappedYet => {
                debug!(
                    "Handling missing ProfileId mapping for Gerrit user '{}'",
                    username
                );
                let stuff = self.request_mapping(name);
                let request_result = stuff.await?;
                debug!("Request was successful. Result is {:?}.", request_result);
                if let Some(profile_id_from_api) = request_result {
                    let sync_result = Synchronization::Some(profile_id_from_api.clone());
                    self.cache.insert(username.clone(), sync_result.clone());
                    self.acteur
                        .send_to_service::<UserServiceClient, SetProfileIdMapping>(
                            SetProfileIdMapping(username.clone(), sync_result.clone()),
                        )
                        .await;
                    Ok(Some(profile_id_from_api))
                } else {
                    self.cache.insert(username.clone(), Synchronization::None);
                    self.acteur
                        .send_to_service::<UserServiceClient, SetProfileIdMapping>(
                            SetProfileIdMapping(username.clone(), Synchronization::None),
                        )
                        .await;
                    Ok(None)
                }
            }
        }
    }
}

impl ProfileIdResolver {
    pub fn new(
        cache: HashMap<GerritUsername, Synchronization<ProfileId>>,
        acteur: ServiceAssistant<ResolverClient>,
    ) -> ProfileIdResolver {
        ProfileIdResolver { cache, acteur }
    }

    /**
     * Returning Synchronization here, is just easier than
     * `Option<Option<ProfileId>>`, after all, we need to make sure to understand
     * the difference between a missing entry and an entry that can't be mapped
     * (e.g. multiple results when asking API).
     */
    fn lookup_cache(&self, username: &GerritUsername) -> Synchronization<ProfileId> {
        match self.cache.get(username) {
            Some(status) => status.clone(),
            // No entry found most likely means we haven't tried yet.
            None => Synchronization::NotMappedYet,
        }
    }

    async fn request_mapping(&self, name: &str) -> Result<Option<ProfileId>, String> {
        self.acteur
            .call_service::<JustClient, SearchProfileId>(SearchProfileId(name.to_string()))
            .await
            .expect("Couldn't send")
        //.unwrap_or("Couldn't send SearchProfileId  message to JustClient actor.".to_string())
    }
}
