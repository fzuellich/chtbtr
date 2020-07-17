// Gather all response related structs, that are used to map JSON responses to objects.
use crate::types::ConversationId;
use crate::types::ProfileId;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
pub struct AccesTokenResponse {
    pub access_token: String,
}

// Used as intermediate representation
#[derive(Deserialize, Debug)]
pub struct JustUserProfileString(String);

impl JustUserProfileString {
    pub fn to_profile_id(&self) -> ProfileId {
        return ProfileId::try_from(self.0.as_ref())
            .expect("Couldn't parse ProfileId from JustAPI.");
    }
}

#[derive(Deserialize)]
pub struct JustUserProfile {
    //chattable: bool,
    pub id: JustUserProfileString,
    //image_id: String,
    //modify_date: u32,
    //name: String,
    //state: String,
}

#[derive(Deserialize)]
pub struct UserSearchResult {
    pub items: Vec<JustUserProfile>,
}

#[derive(Deserialize)]
pub struct ChatCreationResult {
    pub id: ConversationId,
}
