use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum CodeReviewStatus {
    PlusTwo,
    PlusOne,
    None,
    MinusOne,
    MinusTwo,
}

impl fmt::Display for CodeReviewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            CodeReviewStatus::PlusTwo => "+2",
            CodeReviewStatus::PlusOne => "+1",
            CodeReviewStatus::None => "0",
            CodeReviewStatus::MinusOne => "-1",
            CodeReviewStatus::MinusTwo => "-2",
        };
        f.write_str(result)
    }
}

/**
 * Naive implementation of parsing a verified status from a str slice.
 */
impl From<&str> for CodeReviewStatus {
    fn from(value: &str) -> CodeReviewStatus {
        match value.trim() {
            "2" => CodeReviewStatus::PlusTwo,
            "1" => CodeReviewStatus::PlusOne,
            "-1" => CodeReviewStatus::MinusOne,
            "-2" => CodeReviewStatus::MinusTwo,
            _ => CodeReviewStatus::None,
        }
    }
}
