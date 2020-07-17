use acteur::{Actor, ActorAssistant, Receive, Respond};

use crate::{
    actor::messages::{GetAppState, SetAppState},
    types::ConnectionParameters,
};

/// A struct wrapping the app state in a way that other actors can interact with
/// it.
///
/// It is assumed that the actor should be initialised and available before all
/// other actor-related service.
///
/// This is implemented as `acteur::Actor`. We only need to handle a handful of
/// messages and this way we don't need to use a mutex.
#[derive(Debug)]
pub struct AppState(Option<ConnectionParameters>);

#[async_trait::async_trait]
impl Actor for AppState {
    type Id = u8;

    async fn activate(_: Self::Id, _: &ActorAssistant<Self>) -> Self {
        info!("AppState is activated");
        AppState(None)
    }
}

#[async_trait::async_trait]
impl Receive<SetAppState> for AppState {
    async fn handle(&mut self, message: SetAppState, _: &ActorAssistant<AppState>) {
        if self.0.is_some() {
            error!(
                "You attempt to set the application state more than once. This is not supported."
            );
        } else {
            self.0 = Some(message.0);
        }
    }
}

#[async_trait::async_trait]
impl Respond<GetAppState> for AppState {
    type Response = ConnectionParameters;
    async fn handle(&mut self, _: GetAppState, _: &ActorAssistant<AppState>) -> Self::Response {
        self.0.clone().expect("You tried to retrieve the application state before setting it. Make sure to send a `SetAppState` message.")
    }
}
