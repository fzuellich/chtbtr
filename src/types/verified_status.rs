use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum VerifiedStatus {
    PlusOne,
    None,
    MinusOne,
}

impl fmt::Display for VerifiedStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            VerifiedStatus::PlusOne => "+1",
            VerifiedStatus::None => "0",
            VerifiedStatus::MinusOne => "-1",
        };
        f.write_str(result)
    }
}

/**
 * Naive implementation of parsing a verified status from a str slice.
 */
impl From<&str> for VerifiedStatus {
    fn from(value: &str) -> VerifiedStatus {
        match value.trim() {
            "1" => VerifiedStatus::PlusOne,
            "-1" => VerifiedStatus::MinusOne,
            _ => VerifiedStatus::None,
        }
    }
}
