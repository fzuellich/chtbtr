use super::ConnectionParameters;
use acteur::Acteur;

/// Application state that is send between controllers.
#[derive(Clone)]
pub struct AppState {
    pub acteur: Acteur,
    pub connection: ConnectionParameters,
}
