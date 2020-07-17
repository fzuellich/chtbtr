use super::GerritUsername;
use std::path::Path;
use std::path::PathBuf;

///
/// A helper object to construct paths to user-related files.
///
/// Can construct a path to:
/// * A users synchronisation file, that holds the mapping information to his
///   `ProfileId`.
/// * The users settings file.
///
pub struct PathToUserData {
    path: PathBuf,
}

impl PathToUserData {
    /// This function appends the filename of the synchronisation file to a
    /// given path.
    ///
    /// You can use this function, if you have a path that already points to a
    /// valid user-folder in the data repository.
    ///
    pub fn synchronization_with_path(path: &Box<PathBuf>) -> PathToUserData {
        let mut path = PathBuf::from(path.as_path());
        path.push("sync.ron");

        PathToUserData { path }
    }

    pub fn sync(data_dir: &str, username: &GerritUsername) -> PathToUserData {
        let path: PathBuf = [data_dir, &username.0, "sync.ron"].iter().collect();
        PathToUserData { path }
    }

    pub fn settings(data_dir: &str, username: &GerritUsername) -> PathToUserData {
        let path: PathBuf = [data_dir, &username.0, "settings.ron"].iter().collect();
        PathToUserData { path }
    }

    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }
}

impl AsRef<Path> for PathToUserData {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}
