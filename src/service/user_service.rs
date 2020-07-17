use crate::{
    default::DEFAULT_SETTINGS,
    types::{GerritUsername, PathToUserData, ProfileId, Settings, Synchronization},
};
use ron;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub trait UserService {
    /**
     * Load a map of Gerrit username to available Synchronization data.
     */
    fn load_sync_cache(&self) -> HashMap<GerritUsername, Synchronization<ProfileId>>;
    fn load_settings(&self, user: &GerritUsername) -> Result<Settings, String>;
    // fn save_settings(&self, user: &GerritUsername, settings: &Settings) -> Result<(), &str>;
    // fn load_sync(&self, user: &GerritUsername);
    fn save_sync(
        &self,
        user: &GerritUsername,
        profile_id: &ProfileId,
    ) -> Result<Synchronization<ProfileId>, String>;
    fn set_sync(
        &self,
        user: &GerritUsername,
        profile_id: &Synchronization<ProfileId>,
    ) -> Result<(), String>;
}

#[derive(Clone, Debug)]
pub struct FileBackedUserService {
    pub data_dir: String,
}

impl FileBackedUserService {
    fn synchronization_from_path(&self, path: &Path) -> Option<Synchronization<ProfileId>> {
        match fs::read_to_string(path) {
            Ok(content) => match ron::de::from_str(&content) {
                Ok(s) => return Some(s),
                Err(e) => {
                    println!("Couldn't deserialize {}. Cause: {}.", path.display(), e);
                    return None;
                }
            },
            Err(e) => {
                println!(
                    "Couldn't read file content from {}. Cause: {}.",
                    path.display(),
                    e
                );
                return None;
            }
        }
    }
}

impl UserService for FileBackedUserService {
    // TODO use async file api
    fn load_sync_cache(&self) -> HashMap<GerritUsername, Synchronization<ProfileId>> {
        let mut result = HashMap::new();
        let user_repo = match fs::read_dir(&self.data_dir) {
            Ok(dir) => dir,
            Err(e) => panic!(
                "Couldn't read repository data from path {}. Cause: {}.",
                self.data_dir, e
            ),
        };

        for entry in user_repo {
            let entry: fs::DirEntry = entry.expect("Couldn't read entry in data repository.");
            if entry.path().is_file() {
                continue;
            }

            let entry_boxed = Box::new(entry.path());
            let syncfile: PathToUserData = PathToUserData::synchronization_with_path(&entry_boxed);

            let username: GerritUsername = match entry.path().file_name() {
                Some(os_str) => {
                    GerritUsername::from(String::from(os_str.to_string_lossy()).as_str())
                }
                None => {
                    println!(
                        "Couldn't read username from path {}.",
                        syncfile.as_path().display()
                    );
                    continue;
                }
            };

            let optional_sync: Option<Synchronization<ProfileId>> =
                self.synchronization_from_path(syncfile.as_path());
            debug!(
                "Parsing synchronization status for {}. Result {:?}.",
                username, optional_sync
            );
            if let Some(sync) = optional_sync {
                result.insert(username, sync);
            }
        }

        result
    }

    fn save_sync(
        &self,
        user: &GerritUsername,
        profile_id: &ProfileId,
    ) -> Result<Synchronization<ProfileId>, String> {
        let profile_id = Synchronization::Some(profile_id.clone());
        self.set_sync(user, &profile_id)?;
        return Ok(profile_id);
    }

    fn set_sync(
        &self,
        user: &GerritUsername,
        profile_id: &Synchronization<ProfileId>,
    ) -> Result<(), String> {
        let as_str = match ron::ser::to_string(profile_id) {
            Ok(result) => result,
            Err(e) => {
                return Err(format!(
                    "Error serializing Synchronization struct. Cause: {}",
                    e
                ))
            }
        };

        let path = PathToUserData::sync(&self.data_dir, user);
        return fs::create_dir_all(&path.as_path().parent().expect(
            "Expected to find a parent directory. Guess this shouldn't have happened. Fix me.",
        ))
        .and_then(|_| {
            fs::write(&path, as_str)?;
            return Ok(());
        })
        .or_else(|e| {
            Err(format!(
                "Error writing sync data to {}. Cause: {}.",
                path.as_path().display(),
                e
            ))
        });
    }

    fn load_settings(&self, user: &GerritUsername) -> Result<Settings, String> {
        let path_to_settings = PathToUserData::settings(&self.data_dir, user);
        let file: Result<String, std::io::Error> = fs::read_to_string(&path_to_settings);

        let settings_as_str: String = match file {
            Ok(result) => result,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    fs::create_dir_all(
                        &path_to_settings
                            .as_path()
                            .parent()
                            .expect("No parent directory for path to  settings file."),
                    )
                    .expect("Couldn't create directories for settings file.");
                    fs::write(&path_to_settings, DEFAULT_SETTINGS)
                        .expect("Couldn't write default file.");
                    String::from(DEFAULT_SETTINGS)
                }
                _ => {
                    error!("Error reading settings for {}. Cause: {}.", user, e);
                    return Err(format!(
                        "Error reading settings for {}. Cause: {}.",
                        user, e
                    ));
                }
            },
        };

        match ron::de::from_str(&settings_as_str) {
            Ok(settings) => Ok(settings),
            Err(e) => {
                error!("Error deserializing settings for {}. Cause: {}.", user, e);
                Err(format!(
                    "Error deserializing settings for {}. Cause: {}.",
                    user, e
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ProfileId;

    fn assert_synchronization(
        actual: &Synchronization<ProfileId>,
        expect: &Synchronization<ProfileId>,
    ) {
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_load_sync_cache() {
        let data_dir = "tests/user2/load_sync_cache";

        let user_service = FileBackedUserService {
            data_dir: String::from(data_dir),
        };
        let actual = user_service.load_sync_cache();

        let user_a = (
            GerritUsername::from("user.a"),
            Synchronization::Some(ProfileId { 0: 0 }),
        );
        let user_b = (
            GerritUsername::from("user.b"),
            Synchronization::Some(ProfileId { 0: 1 }),
        );
        let user_no_profile_id = (
            GerritUsername::from("user.no_profile_id"),
            Synchronization::None,
        );
        let user_no_settings = (
            GerritUsername::from("user.no_settings"),
            Synchronization::Some(ProfileId { 0: 312 }),
        );

        assert_eq!(actual.len(), 4);
        assert_synchronization(actual.get(&user_a.0).unwrap(), &user_a.1);
        assert_synchronization(actual.get(&user_b.0).unwrap(), &user_b.1);
        assert_synchronization(
            actual.get(&user_no_profile_id.0).unwrap(),
            &user_no_profile_id.1,
        );
        assert_synchronization(
            actual.get(&user_no_settings.0).unwrap(),
            &user_no_settings.1,
        );
    }
}
