use serde::{Deserialize, Serialize};

/*
 * Is usually created only at startup for each user. Basically a mapping
 * from gerrit username to Just ProfileId.
 */
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Synchronization<V> {
    // Use this profile id as receiver for notifications
    Some(V),
    NotMappedYet,
    None,
}

impl<V> Synchronization<V> {
    /**
     * Either return an Option for Some or None; or return Option from Provided closure.
     */
    pub fn or_try_resolve_mut(
        self,
        mut closure: impl FnMut() -> Result<Option<V>, String>,
    ) -> Result<Option<V>, String> {
        match self {
            Synchronization::Some(v) => Ok(Some(v)),
            Synchronization::None => Ok(None),
            Synchronization::NotMappedYet => closure(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProfileId;

    #[test]
    fn test_synchronization() {
        Synchronization::Some(ProfileId { 0: 0 });
    }
}
