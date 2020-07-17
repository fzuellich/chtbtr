use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{ConversationId, ProfileId};

#[derive(Serialize, Deserialize, Debug)]
pub enum ChatType {
    #[serde(rename = "ONE_ON_ONE")]
    OneOnOne,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChatMessageType {
    #[serde(rename = "TEXT")]
    Text,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat<'a> {
    pub id: Uuid,
    pub r#type: ChatType,
    pub title: &'a str,
    // Use ProfileId
    pub participants: [&'a str; 2],
}

impl Chat<'_> {
    pub fn create(participants: [&str; 2]) -> Chat {
        Chat {
            id: Uuid::new_v4(),
            r#type: ChatType::OneOnOne,
            title: "Gerrit Notifications",
            participants,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: Uuid,
    pub chat_id: ConversationId,
    pub r#type: ChatMessageType,
    pub create_date: DateTime<Utc>,
    pub author_id: ProfileId,
    pub text: String,
}

impl ChatMessage {
    pub fn create(author_id: ProfileId, chat_id: ConversationId, text: &str) -> ChatMessage {
        ChatMessage {
            id: Uuid::new_v4(),
            chat_id,
            r#type: ChatMessageType::Text,
            create_date: Utc::now(),
            author_id,
            text: String::from(text),
        }
    }
}
