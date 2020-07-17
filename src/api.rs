use crate::types::GerritTrigger;
use reqwest::blocking::Response;
use serde::Serialize;

pub enum ErrorKind {
    NetworkError, // There was an issue making the request against the chtbtr backend
    BackendError(Response), // The backend replied with a bad status code (!= 200)
}

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

fn fire_request<T: Serialize + ?Sized>(url: &str, params: &T) -> Result<(), Error> {
    let response = reqwest::blocking::Client::new()
        .post(url)
        .json(params)
        .send();
    match response {
        Ok(res) => {
            if res.status().is_success() {
                Ok(())
            } else {
                Err(Error {
                    kind: ErrorKind::BackendError(res),
                })
            }
        }
        Err(_e) => Err(Error {
            kind: ErrorKind::NetworkError,
        }),
    }
}

pub fn send_request(params: &GerritTrigger) -> Result<(), Error> {
    let _fut = match params {
        GerritTrigger::PatchStatusChanged(_) => {
            fire_request("http://localhost:8088/trigger/comment_added", params)
        }
        GerritTrigger::CommentAdded(_) => {
            fire_request("http://localhost:8088/trigger/comment_added", params)
        }
        GerritTrigger::ReviewerAdded(_) => {
            fire_request("http://localhost:8088/trigger/reviewer_added", params)
        }
    };

    Ok(())
}
