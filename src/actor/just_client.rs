use std::{collections::HashMap, time::Instant};

use acteur::{Listen, Serve, Service, ServiceAssistant, ServiceConfiguration};
use futures::lock::Mutex;
use reqwest::blocking::Client;

use crate::{
    actor::{
        messages::{GetAppState, SearchProfileId, SendChatMessage},
        AppState,
    },
    service::JustError,
    just::{requests::*, responses::*, utils::*},
    types::*,
};

#[derive(Debug)]
pub struct JustClient {
    sender: Mutex<ProfileId>,
    domain: Mutex<String>,
    oauth_token: Mutex<String>,
}

impl JustClient {
    pub fn new(profile_id: ProfileId, domain: String, oauth_token: String) -> JustClient {
        JustClient {
            sender: Mutex::new(profile_id),
            domain: Mutex::new(domain),
            oauth_token: Mutex::new(oauth_token),
        }
    }

    pub async fn send_chat_message(&self, receiver: &ProfileId, message: &str) {
        let client = Client::new();
        let sender = self.sender.lock().await;
        let domain = self.domain.lock().await;
        let oauth_token = self.oauth_token.lock().await;

        // Implement with into?
        let sender_as_string = sender.with_profile_prefix();
        let recipient_as_string = receiver.with_profile_prefix();
        let chat = Chat::create([&sender_as_string, &recipient_as_string]);
        let res = client
            .post(&make_api_url(&domain, "/toro/chat/api/v2/chats"))
            .bearer_auth(&*oauth_token)
            .json(&chat)
            .send()
            .expect("Could not send chat creation request.");

        let res: ChatCreationResult = match res.status() {
            reqwest::StatusCode::OK => res.json().expect("Just didn't reply with JSON."),
            _ => {
                let error: JustError = res.json().expect("Just didn't reply with JSON.");
                error!(
                    "Error occurred during chat creation. Response: {}",
                    error.message
                );
                return;
            }
        };

        let conversation: ConversationId = res.id;
        let chat_message = ChatMessage::create(receiver.clone(), conversation.clone(), message);
        client
            .post(&format!(
                "{}/{}/messages",
                &make_api_url(&domain, "/toro/chat/api/v2/chats"),
                conversation.value()
            ))
            .bearer_auth(&*oauth_token)
            .json(&chat_message)
            .send()
            .expect("Error while sending chat message request.")
            .text()
            .expect("Response could not be read.");
    }

    // Error message is valid as long the access token, as it's related to the API request
    async fn request_users<'a, 'b>(
        &self,
        filter: &'b str,
    ) -> Result<Option<ProfileId>, reqwest::Error> {
        let params: HashMap<&str, &str> = [("filter", filter)].iter().cloned().collect();

        let oauth_token = self.oauth_token.lock().await;
        let domain = self.domain.lock().await;

        debug!("Requesting users for filter '{}'.", filter);
        let result: UserSearchResult = Client::new()
            .get(&make_api_url(&domain, "/toro/chat/api/v2/users"))
            .bearer_auth(&*oauth_token)
            .query(&params)
            .send()?
            .json()?;

        let mut profiles: Vec<JustUserProfile> = result.items;
        if !profiles.is_empty() && profiles.len() == 1 {
            // call .remove, it allows us to take ownership and return the ProfileId struct
            // another option would be to call nth(), which would also allow us to remove mut
            debug!("Found exactly one user.");
            return Ok(Some(profiles.remove(0).id.to_profile_id()));
        } else {
            info!(
                "Too many results for user search '{}'. Expected to find exactly one result. Found {}.",
                filter,
                profiles.len()
            );
            return Ok(None);
        }
    }

    pub async fn search_profile_id(&self, change_owner: &str) -> Result<Option<ProfileId>, String> {
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

fn make_api_url(domain: &str, path: &str) -> String {
    format!("https://{}/{}", domain, path)
}

pub async fn get_oauth_token(params: &ConnectionParameters) -> String {
    let mut map = HashMap::new();
    map.insert("client_id", params.client_id.as_str());
    map.insert("grant_type", "password");
    map.insert("username", params.username.as_str());
    map.insert("password", params.password.as_str());

    let response: AccesTokenResponse = reqwest::blocking::Client::new()
        .post(&make_api_url(&params.domain, "/toro/oauth/token"))
        .query(&map)
        .send()
        .expect("Could not send access token request.")
        .json()
        .expect("Requesting access token didn't return no JSON.");

    response.access_token
}

#[async_trait::async_trait]
impl Service for JustClient {
    async fn initialize(system: &ServiceAssistant<Self>) -> (Self, ServiceConfiguration) {
        let state: ConnectionParameters = system
            .call_actor::<AppState, GetAppState>(0, GetAppState {})
            .await
            .expect("Could not retrieve application state.");

        print!("JustClient actor is starting. Requesting OAuth token...");
        let receive_oauth_token_start = Instant::now();
        let oauth_token = get_oauth_token(&state).await;
        println!("{}ms.", receive_oauth_token_start.elapsed().as_millis());
        info!(
            "JustClient actor is starting. Requesting OAuth token took {}ms.",
            receive_oauth_token_start.elapsed().as_millis()
        );

        // TODO Try out the non blocking reqwest
        let just_client =
            JustClient::new(state.profile_id.clone(), state.domain.clone(), oauth_token);
        (just_client, ServiceConfiguration::default())
    }
}

#[async_trait::async_trait]
impl Serve<SearchProfileId> for JustClient {
    type Response = Result<Option<ProfileId>, String>;

    async fn handle(&self, message: SearchProfileId, _: &ServiceAssistant<Self>) -> Self::Response {
        self.search_profile_id(&message.0).await
    }
}

/// Implements a fire-and-forget API to send chat message. There will be no result
/// informing the caller about the success or failure of the call.
///
/// However, should an error occur this method will log a warning.
#[async_trait::async_trait]
impl Listen<SendChatMessage> for JustClient {
    async fn handle(&self, message: SendChatMessage, _: &ServiceAssistant<Self>) {
        debug!("Sending '{}' a chat message '{}'.", &message.0, &message.1);
        self.send_chat_message(&message.0, &message.1).await;
    }
}
