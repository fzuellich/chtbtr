use actix_web::{web, Responder};

use std::ops::Deref;

use crate::{
    controller::error::ControllerError,
    types::{AppState, GerritTrigger},
};

mod comment_added;
mod error;
mod notification_rules;
mod patch_status;
mod reviewer_added;
mod util;

pub async fn comment_controller(
    trigger: web::Json<GerritTrigger>,
    state: web::Data<AppState>,
) -> impl Responder {
    let result: Result<(), ControllerError> = match trigger.deref() {
        GerritTrigger::CommentAdded(data) => {
            comment_added::comment_added_rewrite(&trigger, data, state).await
        }
        GerritTrigger::PatchStatusChanged(data) => {
            patch_status::patch_status_changed(&trigger, state, data).await
        }
        _ => Err(ControllerError::Unrecoverable(String::from(
            "Data doesn't fit endpoint",
        ))),
    };

    if result.is_err() {
        let cause = result.err().unwrap();
        error!("Error: {}", cause);
        return format!("Error in comment_controller: {}", cause);
    } else {
        return String::from("Message send!");
    }
}

pub async fn reviewer_controller(
    trigger: web::Json<GerritTrigger>,
    state: web::Data<AppState>,
) -> impl Responder {
    let result: Result<(), ControllerError> = match trigger.deref() {
        GerritTrigger::ReviewerAdded(data) => {
            reviewer_added::reviewer_added(&trigger, state, data).await
        }
        _ => Err(ControllerError::Unrecoverable(String::from(
            "Data doesn't fit endpoint",
        ))),
    };

    if result.is_err() {
        let cause = result.err().unwrap();
        info!("Error: {}", cause);
        return format!("Error in reviewer_controller: {}", cause);
    } else {
        return String::from("Message send!");
    }
}
