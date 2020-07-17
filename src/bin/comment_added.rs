extern crate chtbtr;
extern crate clap;
extern crate reqwest;

use chtbtr::api::{send_request, ErrorKind};
use clap::ArgMatches;

mod cli {
    use chtbtr::{
        cli::{arg, arg_with_hyphen, ignore_arg},
        types::{patch_status, CodeReviewStatus, GerritUsername, ProjectName, VerifiedStatus, BaseData, GerritTrigger, PatchStatusChangedData, CommentAddedData},
    };
    use clap::{App, ArgMatches};

    pub fn create_cli<'a>() -> App<'a, 'a> {
        let required_args = [
            arg(
                "change_owner",
                "change-owner",
                "The human-readable form of the change owners name. 'First Last <email>'.",
            ),
            arg(
                "change_owner_username",
                "change-owner-username",
                "The owner's username that is used to login with Gerrit.",
            ),
            arg(
                "change_url",
                "change-url",
                "Usually just a change id like '29415'.",
            ),
            arg(
                "project",
                "project",
                "The project name the change belongs to.",
            ),
            arg(
                "author",
                "author",
                "The human-readable form of the comment authors name. 'First Last <email>'.",
            ),
            arg(
                "author_username",
                "author-username",
                "The author's username that is used to login with Gerrit.",
            ),
            arg_with_hyphen(
                "verified",
                "Verified", // important, this must be uppercase!
                "The verified status of the commit at the moment the comment was send.",
            ),
            arg_with_hyphen(
                "verified-old",
                "Verified-oldValue",
                "The verified status before this verified status. Should only be supplied if the comment changes the verified status."
            ).required(false),
            arg_with_hyphen(
                "code-review",
                "Code-Review",
                "The code review status for the commit at the moment the comment was send."
            ),
            arg_with_hyphen(
                "code-review-old",
                "Code-Review-oldValue",
                "The code review status before this one. Should only be present if status changed"
            ).required(false)
        ];

        // We need to parse them so clap doesn't panic, but we don't want them.
        let ignored_args = ["change", "branch", "topic", "commit", "comment"];

        let mut app = App::new("comment-added").about(
            r"Binary to catch Gerrit's hooks plugin comment-added hook.
This binary is usually triggered by Gerrit and not called directly by a user.",
        );

        for arg in required_args.iter() {
            app = app.arg(arg);
        }

        for ignored_field in ignored_args.iter() {
            app = ignore_arg(ignored_field, app);
        }

        app
    }

    pub fn parse_matches_into_struct(matches: &ArgMatches) -> GerritTrigger {
        if matches.is_present("verified-old") || matches.is_present("code-review-old") {
            parse_patch_status_change(matches)
        } else {
            parse_comment_added(matches)
        }
    }

    fn parse_base_data(matches: &ArgMatches) -> BaseData {
        let change_url = String::from(
            matches
                .value_of("change_url")
                .expect("change-url is not set!"),
        );
        let change_owner = matches
            .value_of("change_owner")
            .expect("change-owner is not set!")
            .trim_matches('"');
        let change_owner_username = GerritUsername::from(
            matches
                .value_of("change_owner_username")
                .expect("change-owner-username is not set!"),
        );
        let change_owner = String::from(change_owner);
        let project = String::from(matches.value_of("project").expect("project is not set."));

        BaseData {
            change_owner,
            change_owner_username,
            change_url,
            project: ProjectName::from(project.as_str()),
        }
    }

    fn parse_patch_status_change(matches: &ArgMatches) -> GerritTrigger {
        let base = parse_base_data(matches);

        // The requested values are configured to be required arguments. If we don't get
        // them, then something is wrong with the clap configuration, which is unlikely.
        let author_username = GerritUsername::from(
            matches
                .value_of("author_username")
                .expect("author-username is not set!"),
        );
        let verified = matches.value_of("verified").expect("verified is not set!");
        let old_verified = matches
            .value_of("verified-old")
            .and_then(|v| Some(VerifiedStatus::from(v)))
            .or(None);

        let code_review = matches
            .value_of("code-review")
            .expect("code-review is not set!");
        let old_code_review = matches
            .value_of("code-review-old")
            .and_then(|c| Some(CodeReviewStatus::from(c)))
            .or(None);

        let patch_status_changed = patch_status(
            &CodeReviewStatus::from(code_review),
            &old_code_review,
            &VerifiedStatus::from(verified),
            &old_verified,
        );

        GerritTrigger::PatchStatusChanged(PatchStatusChangedData {
            base,
            author_username,
            patch_status: patch_status_changed,
        })
    }

    fn parse_comment_added(matches: &ArgMatches) -> GerritTrigger {
        let base = parse_base_data(matches);

        // The requested values are configured to be required arguments. If we don't get
        // them, then something is wrong with the clap configuration, which is unlikely.
        let author = String::from(
            matches
                .value_of("author")
                .expect("author is not set.")
                .trim_matches('"'),
        );
        let author_username = GerritUsername::from(
            matches
                .value_of("author_username")
                .expect("author-username is not set!"),
        );

        GerritTrigger::CommentAdded(CommentAddedData {
            base,
            author,
            author_username,
        })
    }

    #[cfg(test)]
    mod test {

        use super::create_cli;
        use crate::cli::parse_matches_into_struct;
        use chtbtr::gerrit_triggers::GerritTrigger;
        use chtbtr::patch_status::PatchStatus;
        use chtbtr::types::{CodeReviewStatus, VerifiedStatus};
        use clap::ArgMatches;

        fn base_args() -> Vec<&'static str> {
            let mut args = base_args_without_patch_status();
            args.extend(vec!["--Verified", "0", "--Code-Review", "0"]);
            return args;
        }

        fn base_args_without_patch_status() -> Vec<&'static str> {
            vec![
                "comment_added",
                "--change-url",
                "12345",
                "--change-owner",
                r#""First Last <first.last@domain.top>""#,
                "--change-owner-username",
                "user.name",
                "--project",
                "juco",
                "--author",
                r#""Another Name <email>""#,
                "--author-username",
                "user.name",
            ]
        }

        #[test]
        fn clap_matches_required_fields() {
            let args = base_args();
            let matches = create_cli().get_matches_from(args);

            assert_eq!(
                matches.value_of("change_owner").unwrap(),
                r#""First Last <first.last@domain.top>""#
            );
            assert_eq!(matches.value_of("change_url").unwrap(), "12345");
            assert_eq!(matches.value_of("project").unwrap(), "juco");
        }

        #[test]
        fn clap_handles_arguments_with_hyphen_values() {
            let mut base_args = base_args_without_patch_status();
            base_args.extend(vec![
                "--Verified",
                "-1",
                "--Code-Review",
                "-2",
                "--Verified-oldValue",
                "-1",
            ]);

            create_cli().get_matches_from(base_args);
        }

        #[test]
        fn clap_manages_plus_one_argument_for_verified() {
            let mut base_args = base_args_without_patch_status();
            base_args.extend(vec!["--Verified", "+1", "--Code-Review", "+2"]);

            create_cli().get_matches_from(base_args);
        }

        #[test]
        fn clap_manages_with_multiline_comment() {
            let mut base_args = base_args_without_patch_status();
            base_args.extend(vec![
                "--comment",
                r#"Rebased
patchset."#,
                "--Verified",
                "1",
                "--Code-Review",
                "2",
            ]);
            let matches = create_cli().get_matches_from(base_args);
            assert!(matches.is_present("comment"));
            assert!(matches.is_present("code-review"));
            assert!(matches.is_present("verified"));
        }

        #[test]
        fn removes_quotes_from_author() {
            let base_args = base_args();
            let matches: ArgMatches = create_cli().get_matches_from(base_args);

            let mystruct: GerritTrigger = parse_matches_into_struct(&matches);
            match mystruct {
                GerritTrigger::CommentAdded(data) => {
                    assert_eq!(data.author, "Another Name <email>")
                }
                _ => panic!("Returned wrong struct type."),
            };
        }

        #[test]
        fn removes_quotes_from_change_owner() {
            let base_args = base_args();
            let matches: ArgMatches = create_cli().get_matches_from(base_args);

            let mystruct: GerritTrigger = parse_matches_into_struct(&matches);
            match mystruct {
                GerritTrigger::CommentAdded(data) => {
                    assert_eq!(data.base.change_owner, "First Last <first.last@domain.top>")
                }
                _ => panic!("Returned wrong struct type."),
            };
        }

        #[test]
        fn triggers_verified_change_when_verified_old_value_set() {
            let mut base_args = base_args_without_patch_status();
            base_args.extend(vec![
                "--Verified",
                "-1",
                "--Verified-oldValue",
                "0",
                "--Code-Review",
                "0",
            ]);
            let matches: ArgMatches = create_cli().get_matches_from(base_args);
            let mystruct: GerritTrigger = parse_matches_into_struct(&matches);

            if let GerritTrigger::PatchStatusChanged(data) = mystruct {
                // Most important here is that we have a VerifiedStatusChanged
                match data.patch_status {
                    PatchStatus::Verified(verified) => {
                        assert_eq!(verified, VerifiedStatus::MinusOne)
                    }
                    _ => panic!("Didn't find PatchStatus::Verified!"),
                }
            } else {
                panic!("Wrong gerrit trigger generated from parameters.");
            }
        }

        #[test]
        fn triggers_code_review_change_when_only_code_review_old_value_set() {
            let mut base_args = base_args_without_patch_status();
            base_args.extend(vec![
                "--Code-Review",
                "-1",
                "--Code-Review-oldValue",
                "0",
                "--Verified",
                "0",
            ]);
            let matches: ArgMatches = create_cli().get_matches_from(base_args);
            let mystruct: GerritTrigger = parse_matches_into_struct(&matches);

            if let GerritTrigger::PatchStatusChanged(data) = mystruct {
                // Most important here is that we have a VerifiedStatusChanged
                match data.patch_status {
                    PatchStatus::CodeReview(review) => {
                        assert_eq!(review, CodeReviewStatus::MinusOne)
                    }
                    _ => panic!("Didn't find PatchStatus::CodeReview!"),
                }
            } else {
                panic!("Wrong gerrit trigger generated from parameters.");
            }
        }

        #[test]
        fn triggers_ready_for_submit() {
            let mut base_args = base_args_without_patch_status();
            base_args.extend(vec![
                "--Verified",
                "1",
                "--Code-Review",
                "2",
                "--Code-Review-oldValue",
                "0",
            ]);
            let matches: ArgMatches = create_cli().get_matches_from(base_args);
            let mystruct: GerritTrigger = parse_matches_into_struct(&matches);

            if let GerritTrigger::PatchStatusChanged(data) = mystruct {
                // Most important here is that we have a VerifiedStatusChanged
                assert_eq!(data.patch_status, PatchStatus::ReadyForSubmit);
            } else {
                panic!("Wrong gerrit trigger generated from parameters.");
            }
        }

        #[test]
        fn send_comment_information() {
            let base_args = base_args();
            let matches: ArgMatches = create_cli().get_matches_from(base_args);
            let mystruct: GerritTrigger = parse_matches_into_struct(&matches);

            if let GerritTrigger::CommentAdded(_data) = mystruct {
                return;
            } else {
                panic!("Wrong gerrit trigger generated from parameters.");
            }
        }
    }
}

fn main() {
    let matches: ArgMatches = cli::create_cli().get_matches();
    let params = cli::parse_matches_into_struct(&matches);

    send_request(&params).unwrap_or_else(|error| {
        match error.kind() {
            ErrorKind::BackendError(res) => {
                println!("Request to trigger message failed. Response was: {:?}", res)
            }
            ErrorKind::NetworkError => {
                println!("Couldn't send trigger request to chtbtr server on localhost:8088.")
            }
        };
    })
}
