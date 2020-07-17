extern crate chtbtr;
extern crate clap;

use chtbtr::{
    api::{send_request, ErrorKind},
    cli::reviewer_added_cli,
    types::{GerritTrigger, GerritUsername, ReviewerAddedData},
};
use clap::ArgMatches;

fn parse_matches_into_struct(matches: &ArgMatches) -> GerritTrigger {
    let change_url = matches
        .value_of("change_url")
        .expect("change-url is not set!");
    let reviewer = matches.value_of("reviewer").expect("reviewer is not set!");
    let project = matches.value_of("project").expect("project is not set!");
    let reviewer_username = matches
        .value_of("reviewer_username")
        .expect("reviewer-username is missing");
    let change_owner = matches
        .value_of("change_owner")
        .expect("change-owner is missing.");
    let change_owner_username = matches
        .value_of("change_owner_username")
        .expect("change-owner-username is missing.");

    GerritTrigger::ReviewerAdded(ReviewerAddedData {
        change_url: String::from(change_url),
        project: String::from(project),
        reviewer: String::from(reviewer),
        reviewer_username: GerritUsername::from(reviewer_username),
        change_owner: String::from(change_owner),
        change_owner_username: GerritUsername::from(change_owner_username),
    })
}

fn main() {
    let matches: ArgMatches = reviewer_added_cli().get_matches();
    let trigger_parameters = parse_matches_into_struct(&matches);
    send_request(&trigger_parameters).unwrap_or_else(|error| match error.kind() {
        ErrorKind::NetworkError => {
            panic!("Couldn't send trigger request to chtbtr server on localhost:8088.")
        }
        ErrorKind::BackendError(res) => {
            panic!("Request to trigger message failed. Response was: {:?}", res)
        }
    });
}
