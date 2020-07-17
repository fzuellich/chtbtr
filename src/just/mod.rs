pub mod requests;
pub mod responses;

pub mod utils {

    pub fn user_lastname(change_owner: &str) -> &str {
        let fullname = remove_email_from_owner(change_owner);
        // Split into two because there is a whitespace between 'lastname <email>'
        fullname
            .rsplitn(2, ' ')
            .nth(0)
            .expect("Couldn't split fullname into lastname.")
    }

    pub fn user_firstname(change_owner: &str) -> &str {
        let fullname = remove_email_from_owner(change_owner);
        fullname
            .splitn(2, ' ')
            .nth(0)
            .expect("Couldn't split fullname into firstname.")
    }

    pub fn remove_email_from_owner(change_owner: &str) -> &str {
        let start_of_email = change_owner
            .find('<')
            .expect("Expect to find symbol '<' in change owner.");

        &change_owner[0..start_of_email].trim()
    }

    #[cfg(test)]
    mod test {

        use super::{remove_email_from_owner, user_firstname, user_lastname};

        #[test]
        pub fn can_cut_off_email_part() {
            let result = remove_email_from_owner("First Last <first.last@something.com>");
            assert_eq!("First Last", result);

            let result = remove_email_from_owner("First Middle Last <first.last@something.com>");
            assert_eq!("First Middle Last", result);
        }

        #[test]
        pub fn can_find_lastname() {
            let result = user_lastname("First Last <first.last@something.com>");
            assert_eq!("Last", result);

            let result = user_lastname("First Middle Last <first.last@something.com>");
            assert_eq!("Last", result);
        }

        #[test]
        pub fn can_find_firstname() {
            let result = user_firstname("First Last <first.last@something.com>");
            assert_eq!("First", result);

            let result = user_firstname("First Middle Last <first.last@something.com>");
            assert_eq!("First", result);
        }
    }
}
