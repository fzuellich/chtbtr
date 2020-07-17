use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Represents a profile id in the format PROFILE,id.
// TODO: Write some tests for me!
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct ProfileId(pub u32);

impl ProfileId {
    pub fn with_profile_prefix(&self) -> String {
        format!("PROFILE,{}", self.0)
    }
}

impl fmt::Display for ProfileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PROFILE,{}", self.0)
    }
}

impl TryFrom<&str> for ProfileId {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, &'static str> {
        if !value.starts_with("PROFILE,") {
            //Err(())
            ()
        }

        let id_part = value.replace("PROFILE,", "");
        match id_part.parse() {
            Ok(int) => Ok(ProfileId { 0: int }),
            Err(_e) => Err(""),
        }
    }
}
