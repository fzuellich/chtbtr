mod app_state {

    use crate::types::ConnectionParameters;

    /// Upon receiving this message the actor will set the application state. Will
    /// log an error and do nothing if called more than once.
    ///
    /// We need to ensure that all actors start with the correct configuration.
    /// Changes after startup can't be handled.
    ///
    #[derive(Debug)]
    pub struct SetAppState(pub ConnectionParameters);

    /// The actor will return the current application state.
    ///
    /// The call will panic if there is no valid state set.
    #[derive(Debug)]
    pub struct GetAppState;
}

mod user {

    use crate::types::{GerritUsername, ProfileId, Synchronization};

    /// A message that retrieves the profile id and settings mapped to a particular
    /// `GerritUsername`.
    ///
    /// In case no data is available, it will be tried to request it using the fully
    /// qualified name of the user.
    #[derive(Clone, Debug)]
    pub struct GetUserData(pub GerritUsername, pub String);

    /// Load a cache with user mapping data.
    #[derive(Debug)]
    pub struct InitializeCache(pub String);

    /// Actor executes `LoadSettings` and returns settings for a given username.
    ///
    /// If no settings for a given username are found, the actor will return
    /// a default user setting.
    #[derive(Clone, Debug)]
    pub struct LoadSettings(pub GerritUsername);

    impl From<GetUserData> for LoadSettings {
        fn from(origin: GetUserData) -> Self {
            LoadSettings(origin.0)
        }
    }

    // Save a ProfileId mapping for a GerritUsername.
    //
    // This call will fail silently (e.g. we can't write a synchronisation file) and
    // won't provide a response for the sake of performance.
    #[derive(Debug)]
    pub struct SetProfileIdMapping(pub GerritUsername, pub Synchronization<ProfileId>);
}

mod just {

    use super::user::GetUserData;
    use crate::types::{GerritUsername, ProfileId};

    #[derive(Debug)]
    pub struct SearchProfileId(pub String);

    /// Message will trigger a Chat message to be send to the given recipient.
    #[derive(Debug)]
    pub struct SendChatMessage(pub ProfileId, pub String);

    #[derive(Debug)]
    pub struct ResolveToProfileId(pub GerritUsername, pub String);

    impl From<GetUserData> for ResolveToProfileId {
        fn from(origin: GetUserData) -> Self {
            ResolveToProfileId(origin.0, origin.1)
        }
    }
}

pub use app_state::{GetAppState, SetAppState};
pub use just::{ResolveToProfileId, SearchProfileId, SendChatMessage};
pub use user::{GetUserData, InitializeCache, LoadSettings, SetProfileIdMapping};
