pub const DEFAULT_SETTINGS: &'static str = r#"V1 (
    /*
     * All settings below apply to you, only when you are a reviewer of a given
     * patch.
     */
    as_reviewer: (
        // Be notified when you are added as reviewer
        subscribe: false,

        /*
         * Ignore ALL notifications for these topics, e.g. [("merge-commit")],
         * on patches that you are added as a reviewer.
         *
         * This means you wont receive notifications that you are added as a
         * reviewer, or that the review was updated etc.
         */
        //ignore_topics: [("my-topic")]
        ignore_topics: [],

        /*
         * Ignore notifications for certain projects. Only active when you are a
         * reviewer for the patch.
         */
        //ignore_projects: [("my-project"), ("another-project")]
        ignore_projects: [],

        /*
         * Ignore ALL comments by the given gerrit username.
         */
        ignore_by_username: [("tools.just"), ("ec2.gerrit")],
    ),

    /**
     * All settings below apply to you, only when you are the owner of a given patch.
     */
    as_owner: (
        /*
         * Be notified when someone comments on your patch.
         */
        subscribe_comment: false,

        /*
         * Be notified about changes to the Verified status. You'll receive
         * notifications when your patch receives -1 or +1.
         */
        subscribe_verified: false,

        /*
         * Be notified if your patch can be submitted.
         */
        subscribe_ready_for_submit: false,

        /*
         * Be notified if someone submits your patch.
         */
        subscribe_submitted: false,

       /*
        * Ignore comments that only mention a change to the review status. In
        * other words: if a reviewer sets +1/-1/+2/-2 wihtout writing a comment,
        * you won't receive a notification.
        */
       ignore_empty_review_comments: false,

        /*
         * Ignore ALL comments by the given gerrit username.
         */
        ignore_by_username: [("tools.just"), ("ec2.gerrit")],

        /*
         * Ignore ALL notifications for certain projects. Only when you are the
         * owner of the patch in the specified project.
         */
        //ignore_projects: [("my-project"), ("another-project")]
        ignore_projects: [],
    )
)"#;
