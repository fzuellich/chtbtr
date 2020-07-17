use serde::{Deserialize, Serialize};

use crate::types::{CodeReviewStatus, VerifiedStatus};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum PatchStatus {
    Both(CodeReviewStatus, VerifiedStatus),
    CodeReview(CodeReviewStatus),
    Verified(VerifiedStatus),
    ReadyForSubmit,
    None,
}

pub fn patch_status(
    code_review: &CodeReviewStatus,
    code_review_old: &Option<CodeReviewStatus>,
    verified: &VerifiedStatus,
    verified_old: &Option<VerifiedStatus>,
) -> PatchStatus {
    if code_review == &CodeReviewStatus::PlusTwo && verified == &VerifiedStatus::PlusOne {
        return PatchStatus::ReadyForSubmit;
    }

    if verified_old.is_some() {
        let verified = verified.clone();
        return PatchStatus::Verified(verified);
    }

    if code_review_old.is_some() {
        let code_review = code_review.clone();
        return PatchStatus::CodeReview(code_review);
    }

    return PatchStatus::None;
}

#[cfg(test)]
mod test {

    use super::patch_status;
    use super::PatchStatus;
    use crate::types::{CodeReviewStatus, VerifiedStatus};

    #[test]
    fn recognize_no_change() {
        // TODO If we have a status set, but it wasn't changed, do we want to say: PatchStatus::None?
        let expected = PatchStatus::None;

        let actual = patch_status(
            &CodeReviewStatus::PlusOne,
            &None,
            &VerifiedStatus::None,
            &None,
        );
        assert_eq!(actual, expected);

        let actual = patch_status(
            &CodeReviewStatus::None,
            &None,
            &VerifiedStatus::MinusOne,
            &None,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn recognize_ready_for_submit_on_both_change() {
        let expected = PatchStatus::ReadyForSubmit;
        let actual = patch_status(
            &CodeReviewStatus::PlusTwo,
            &Some(CodeReviewStatus::None),
            &VerifiedStatus::PlusOne,
            &Some(VerifiedStatus::None),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn recognize_ready_for_submit_on_code_review_change() {
        let expected = PatchStatus::ReadyForSubmit;
        let actual = patch_status(
            &CodeReviewStatus::PlusTwo,
            &Some(CodeReviewStatus::None),
            &VerifiedStatus::PlusOne,
            &None,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn recognize_ready_for_submit_on_verified_change() {
        let expected = PatchStatus::ReadyForSubmit;
        let actual = patch_status(
            &CodeReviewStatus::PlusTwo,
            &None,
            &VerifiedStatus::PlusOne,
            &Some(VerifiedStatus::None),
        );

        assert_eq!(actual, expected);
    }

    ///////////////////////////////////////////////////////////////////////
    // Tests that assume only one status (verified or code review) changed
    ///////////////////////////////////////////////////////////////////////

    #[test]
    fn recognize_changed_verified_status() {
        for new_verified in [
            VerifiedStatus::None,
            VerifiedStatus::MinusOne,
            VerifiedStatus::PlusOne,
        ]
        .iter()
        {
            let test_params = [
                &Some(VerifiedStatus::None),
                &Some(VerifiedStatus::PlusOne),
                &Some(VerifiedStatus::MinusOne),
            ];

            let new_code_review = &CodeReviewStatus::None;
            let old_code_review = &None;

            let expected = PatchStatus::Verified(new_verified.clone());

            for old_verified in test_params.iter() {
                let actual =
                    patch_status(new_code_review, old_code_review, new_verified, old_verified);
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn recognize_changed_code_review_status() {
        for new_code_review in [
            CodeReviewStatus::None,
            CodeReviewStatus::MinusOne,
            CodeReviewStatus::PlusOne,
            CodeReviewStatus::MinusTwo,
            CodeReviewStatus::PlusTwo,
        ]
        .iter()
        {
            let old_code_reviews = [
                Some(CodeReviewStatus::None),
                Some(CodeReviewStatus::PlusOne),
                Some(CodeReviewStatus::PlusTwo),
                Some(CodeReviewStatus::MinusOne),
                Some(CodeReviewStatus::MinusTwo),
            ];

            let new_verified = &VerifiedStatus::None;
            let old_verified = &None;

            let expected = PatchStatus::CodeReview(new_code_review.clone());

            for old_code_review in old_code_reviews.iter() {
                let actual =
                    patch_status(new_code_review, old_code_review, new_verified, old_verified);
                assert_eq!(actual, expected);
            }
        }
    }
}
