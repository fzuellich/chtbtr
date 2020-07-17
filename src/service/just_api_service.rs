use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    just::requests::{Chat, ChatMessage},
    just::responses::{AccesTokenResponse, ChatCreationResult, JustUserProfile, UserSearchResult},
    just::utils::{user_firstname, user_lastname},
    types::{ConnectionParameters, ConversationId, ProfileId},
};

#[derive(Deserialize)]
pub struct JustError {
    pub message: String,
}

#[async_trait::async_trait]
pub trait JustApiService {
    async fn send_message<'a, 'b>(
        &self,
        recipient: &ProfileId,
        message: &'b str,
    ) -> Result<(), String>;
    async fn ensure_access_token_available(&mut self);
    /**
     * Search a ProfileId for the given name. Returns a ProfileId when only one
     * match can be found. Otherwise None. In case of an error (REST-API, etc.)
     * a String with the error is returned
     */
    async fn search_user(&self, change_owner: &str) -> Result<Option<ProfileId>, String>;
}

// Service implements communication with a Just Server.
#[derive(Clone)]
pub struct JustApiServiceImpl {
    connection: ConnectionParameters,
    client: reqwest::Client,
}

impl JustApiServiceImpl {
    pub fn new(params: ConnectionParameters) -> JustApiServiceImpl {
        JustApiServiceImpl {
            connection: params,
            client: reqwest::Client::new(),
        }
    }

    fn make_api_url(&self, path: &str) -> String {
        format!("https://{}/{}", self.connection.domain, path)
    }

    async fn request_access_token(&self) -> String {
        let mut map = HashMap::new();
        map.insert("client_id", self.connection.client_id.as_str());
        map.insert("grant_type", "password");
        map.insert("username", &self.connection.username);
        map.insert("password", &self.connection.password);

        let response: AccesTokenResponse = self
            .client
            .post(&self.make_api_url("/toro/oauth/token"))
            .query(&map)
            .send()
            .await
            .expect("Could not send access token request.")
            .json()
            .await
            .expect("Requesting access token didn't return no JSON.");

        response.access_token
    }

    // Error message is valid as long the access token, as it's related to the API request
    async fn request_users<'a, 'b>(
        &self,
        filter: &'b str,
    ) -> Result<Option<ProfileId>, reqwest::Error> {
        let params: HashMap<&str, &str> = [("filter", filter)].iter().cloned().collect();

        println!("Requesting users");
        let result: UserSearchResult = self
            .client
            .get(&self.make_api_url("/toro/chat/api/v2/users"))
            .bearer_auth(&self.connection.oauth_token)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        let mut profiles: Vec<JustUserProfile> = result.items;
        if !profiles.is_empty() && profiles.len() == 1 {
            // call .remove, it allows us to take ownership and return the ProfileId struct
            // another option would be to call nth(), which would also allow us to remove mut
            return Ok(Some(profiles.remove(0).id.to_profile_id()));
        } else {
            info!(
                "Too many results for user search '{}'. Expected to find exactly one result.",
                filter
            );
            debug!("Profiles length is {}", profiles.len());
            return Ok(None);
        }
    }
}

#[async_trait::async_trait]
impl JustApiService for JustApiServiceImpl {
    async fn ensure_access_token_available(&mut self) {
        let access_token = self.request_access_token().await;
        let result = ConnectionParameters {
            oauth_token: access_token,
            ..self.connection.clone()
        };

        self.connection = result;
    }

    async fn send_message<'a, 'b>(
        &self,
        recipient: &ProfileId,
        message: &'b str,
    ) -> Result<(), String> {
        let profile_id = self.connection.profile_id.with_profile_prefix();
        let recipient = recipient.with_profile_prefix();
        let chat = Chat::create([&profile_id, &recipient]);
        let res: reqwest::Response = self
            .client
            .post(&self.make_api_url("/toro/chat/api/v2/chats"))
            .bearer_auth(&self.connection.oauth_token)
            .json(&chat)
            .send()
            .await
            .expect("Could not send chat creation request.");

        let res: ChatCreationResult = match res.status() {
            reqwest::StatusCode::OK => res.json().await.expect("Just didn't reply with JSON."),
            _ => {
                let error: JustError = res.json().await.expect("Just didn't reply with JSON.");
                return Err(format!(
                    "Error occurred during chat creation. Response: {}",
                    error.message
                ));
            }
        };

        let conversation: ConversationId = res.id;
        let chat_message = ChatMessage::create(
            self.connection.profile_id.clone(),
            conversation.clone(),
            message,
        );
        self.client
            .post(&format!(
                "{}/{}/messages",
                &self.make_api_url("/toro/chat/api/v2/chats"),
                conversation.value()
            ))
            .bearer_auth(&self.connection.oauth_token)
            .json(&chat_message)
            .send()
            .await
            .expect("Error while sending chat message request.")
            .text()
            .await
            .expect("Response could not be read.");

        Ok(())
    }

    // I think the error message is related to the change_owner and not the token.
    // The result/error message should be valid until the change_owner that called the function
    // is no longer valid.
    async fn search_user(&self, change_owner: &str) -> Result<Option<ProfileId>, String> {
        let search = [
            change_owner,
            user_firstname(change_owner),
            user_lastname(change_owner),
        ];
        for s in search.iter() {
            let request_result = self.request_users(s).await;
            if request_result.is_err() {
                error!("API request failed.");
                return Err(String::from("Error when searching for user."));
            }

            let result = request_result.unwrap();
            if result.is_some() {
                debug!("Found search result for '{}'.", s);
                return Ok(result);
            }

            debug!("Search for user with search '{}' returned no result.", s);
        }

        return Ok(None);
    }
}
