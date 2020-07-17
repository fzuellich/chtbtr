extern crate clap;
use clap::{App, Arg, ArgMatches};

use crate::types::ConnectionParameters;
use crate::types::ProfileId;
use std::convert::TryFrom;

pub fn ignore_arg<'a>(name: &'a str, app: App<'a, 'a>) -> App<'a, 'a> {
    app.arg(
        Arg::with_name(name)
            .long(name)
            .help("Ignored.")
            .takes_value(true)
            .allow_hyphen_values(true) // fixes issues with --Verified -1
            .required(false),
    )
}

pub fn arg<'a>(rust_name: &'a str, long: &'a str, help: &'a str) -> Arg<'a, 'a> {
    Arg::with_name(rust_name)
        .long(long)
        .help(help)
        .takes_value(true)
        .required(true)
}

/**
 * Allow hyphens for values. This fixes issues with -1 or -2 for the CodeReview and Verified settings.
 * Make sure this doesn't break the cli, as the might collide with a short name (e.g. '-i').
 */
pub fn arg_with_hyphen<'a>(rust_name: &'a str, long: &'a str, help: &'a str) -> Arg<'a, 'a> {
    arg(rust_name, long, help).allow_hyphen_values(true)
}

pub fn reviewer_added_cli<'a>() -> App<'a, 'a> {
    let mut app = App::new("reviewer-added")
        .about(
            r"Binary to catch Gerrit's hooks plugin reviewer-added hook.
This binary is usually triggered by Gerrit and not called directly by a user.",
        )
        .arg(
            Arg::with_name("reviewer")
                .long("reviewer")
                .help(
                    "The person that owns the change.
Usually in this form: First Last <first.last@domain.com>",
                )
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("reviewer_username")
                .long("reviewer-username")
                .help("The user name for the person that is reviewer.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("change_owner_username")
                .long("change-owner-username")
                .help("The user name for the person that owns the change.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("change_owner")
                .long("change-owner")
                .help("The name of the person that owns the change. Firstname Lastname <email>")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("The project name like 'juco'.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("change_url")
                .long("change-url")
                .help("The change url. Usually a number like '12312'.")
                .takes_value(true)
                .required(true),
        );

    let ignore_args = ["change", "branch"];

    for arg in ignore_args.iter() {
        app = ignore_arg(arg, app);
    }

    app
}

pub fn get_clap<'a>() -> App<'a, 'a> {
    App::new("chtbtr")
        .about(
            r"Chatbot that can send chat notifications triggered by Gerrit hooks.
This is the server part."
        )
        .after_help("
EXAMPLES:

./chtbtr --chat-bot-profile-id \"PROFILE,1234\" \\
         --just-domain \"just.installation.social\" \\
         --gerrit-domain \"gerrit.installation.com\" \\
         --username \"user.name+chatbot@domain.com\" \\
         --password \"mysecretpassword\" \\
         --user-data \"/home/user/data_dir\" \\
         --client-id \"myclientid\"")
        .arg(Arg::with_name("profile_id")
             .long("chat-bot-profile-id")
             .help("The profile id the chatbot should use. In the form 'PROFILE,n'.")
             .takes_value(true)
             .display_order(0)
             .required(true),
        )
        .arg(Arg::with_name("domain")
             .long("just-domain")
             .help("The domain to use. Example: 'just.installation.social'")
             .takes_value(true)
             .display_order(1)
             .required(true)
        )
        .arg(Arg::with_name("gerrit_domain")
             .long("gerrit-domain")
             .help("Gerrit domain to forward notifications for. Is used to construct messages.")
             .takes_value(true)
             .display_order(2)
             .required(true)
        )
        .arg(Arg::with_name("username")
             .long("username")
             .help("The username used to retrieve an OAuth token. Usually the same account as the chatbot's profile.")
             .takes_value(true)
             .display_order(3)
             .required(true))
        .arg(Arg::with_name("password")
             .long("password")
             .help("Password used to retrieve an OAuth token.")
             .takes_value(true)
             .display_order(4)
             .required(true))
        .arg(Arg::with_name("data_dir")
             .long("data-dir")
             .help("The directory where user data is stored.")
             .takes_value(true)
             .display_order(5)
             .required(true))
        .arg(Arg::with_name("client_id")
             .long("client-id")
             .help("The OAuth client id as configured in the backend.")
             .takes_value(true)
             .display_order(6)
             .required(true))
}

fn validate_match(matches: &ArgMatches, field: &str) -> String {
    let error: &str = &format!("{} is not not set. Clap is not properly configured?", field);
    String::from(matches.value_of(field).expect(error))
}

pub fn parse_matches_into_connection_parameters(matches: &ArgMatches) -> ConnectionParameters {
    let profile_id = validate_match(matches, "profile_id");
    let profile_id =
        ProfileId::try_from(profile_id.as_str()).expect("Couldn't parse ProfileId from CLI.");
    let domain = validate_match(matches, "domain");
    let gerrit_domain = validate_match(matches, "gerrit_domain");
    let username = validate_match(matches, "username");
    let password = validate_match(matches, "password");
    let data_dir = validate_match(matches, "data_dir");
    let client_id = validate_match(matches, "client_id");

    ConnectionParameters {
        profile_id,
        domain,
        gerrit_domain,
        username,
        password,
        data_dir,
        oauth_token: String::from("notset"),
        client_id,
    }
}

pub fn parse_cli_args() -> ConnectionParameters {
    let cli_args: ArgMatches = get_clap().get_matches();
    parse_matches_into_connection_parameters(&cli_args)
}
