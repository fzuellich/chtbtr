use serde::{Deserialize, Serialize};

/// Represents an id that identifies a conversation in Just.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConversationId(String);

impl ConversationId {
    pub fn value(&self) -> &str {
        &self.0
    }
}
