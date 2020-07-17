use super::ProfileId;

/// Provides various values required to run the program.
#[derive(Clone, Debug)]
pub struct ConnectionParameters {
    pub profile_id: ProfileId,
    pub gerrit_domain: String,
    pub domain: String,
    pub username: String,
    pub password: String,
    pub oauth_token: String,
    pub data_dir: String,
    pub client_id: String,
}
