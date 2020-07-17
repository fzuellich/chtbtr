CHTBTR_TEST_DATA:=chtbtr_test.env

# Include data from chtbtr environemnt
include ${CHTBTR_TEST_DATA}
export

setup_data_template: SHELL:=/usr/bin/fish
setup_data_template: # Ask user a few questions regarding credentials that shouldn't be published to Github
	@echo "=== Basics ===" \
		&& read -P "Local directory for data: " -c "tmpdata" REPLY \
    && echo "TEST_DATA_DIR=$$REPLY" > ${CHTBTR_TEST_DATA} \
		&& read -P "Just test domain (example.de): " -c "$$JUST_TEST_DOMAIN" REPLY \
    && echo "JUST_TEST_DOMAIN=$$REPLY" >> ${CHTBTR_TEST_DATA} \
		&& read -P "Gerrit domain (example.de): " -c "$$GERRIT_TEST_DOMAIN" REPLY \
    && echo "GERRIT_TEST_DOMAIN=$$REPLY" >> ${CHTBTR_TEST_DATA} \
		&& read -P "ProfileId for Chatbot: " -c "$$TEST_PROFILE_ID" REPLY \
    && echo "TEST_PROFILE_ID=$$REPLY" >> ${CHTBTR_TEST_DATA} \
		&& echo "=== OAuth ===" \
		&& read -P "OAuth2 client-id: " -c "$$OAUTH_CLIENT_ID" REPLY \
    && echo "OAUTH_CLIENT_ID=$$REPLY" >> ${CHTBTR_TEST_DATA} \
		&& read -P "OAuth username: " -c "$$OAUTH_USERNAME" REPLY \
    && echo "OAUTH_USERNAME=$$REPLY" >> ${CHTBTR_TEST_DATA} \
		&& read -sP "OAuth password: " REPLY \
    && echo "OAUTH_PASSWORD=$$REPLY" >> ${CHTBTR_TEST_DATA} \

run_test_server : # Run a local chtbtr server connected to a Just test instance
	@RUST_BACKTRACE=1 \
	RUST_LOG=debug cargo run --bin chtbtr -- \
		--data-dir=$$TEST_DATA_DIR \
		--just-domain=$$JUST_TEST_DOMAIN \
		--gerrit-domain=$$GERRIT_TEST_DOMAIN \
		--password="$$OAUTH_PASSWORD" \
		--username="$$OAUTH_USERNAME" \
		--client-id=$$OAUTH_CLIENT_ID \
		--chat-bot-profile-id=$$TEST_PROFILE_ID

test_smoke : test_simple_comment test_comment_verified test_comment_verified_minus_one test_comment_both test_comment_ready_for_submit test_comment_ready_for_submit_verified_changed test_comment_ready_for_submit_both test_reviewer_added # Runs a few happy path tests

test_simple_comment : # Trigger a simple comment that doesn't change the patch status
	cargo run --bin comment_added -- \
		--change-owner "fzuellich <user@example>" \
		--change-owner-username="fzuellich" \
		--author-username="tools" \
		--author="tools <>" \
		--change-url 123 \
		--project juco \
		--Verified 0 \
		--Code-Review 0 \

test_comment_verified :
	cargo run --bin comment_added -- \
		--change-owner "fzuellich <user@example>" \
		--change-owner-username="fzuellich" \
		--author-username="tools" \
		--author="tools <>" \
		--change-url 123 \
		--project juco \
		--Verified 1 \
		--Code-Review 0 \
		--Verified-oldValue 0

test_comment_verified_minus_one :
	cargo run --bin comment_added -- \
		--change-owner "fzuellich <user@example>" \
		--change-owner-username="fzuellich" \
		--author-username="tools" \
		--author="tools <>" \
		--change-url 123 \
		--project juco \
		--Verified -1 \
		--Code-Review 0 \
		--Verified-oldValue 0

test_comment_both :
	cargo run --bin comment_added -- \
		--change-owner "fzuellich <user@example>" \
		--change-owner-username="fzuellich" \
		--author-username="tools" \
		--author="tools <>" \
		--change-url 123 \
		--project juco \
		--Verified 1 \
		--Code-Review 1 \
		--Code-Review-oldValue 0 \
		--Verified-oldValue 0

test_comment_ready_for_submit : # CodeReview changed to +2
	cargo run --bin comment_added -- \
		--change-owner "fzuellich <user@example>" \
		--change-owner-username="fzuellich" \
		--author-username="tools" \
		--author="tools <>" \
		--change-url 123 \
		--project juco \
		--Verified 1 \
		--Code-Review 2 \
		--Code-Review-oldValue 0

test_comment_ready_for_submit_verified_changed : # Only verified changed to +1
	cargo run --bin comment_added -- \
		--change-owner "fzuellich <user@example>" \
		--change-owner-username="fzuellich" \
		--author-username="tools" \
		--author="tools <>" \
		--change-url 123 \
		--project juco \
		--Verified 1 \
		--Verified-oldValue -1 \
		--Code-Review 2 \

test_comment_ready_for_submit_both : # +1 and +2 for patch
	cargo run --bin comment_added -- \
		--change-owner "fzuellich <user@example>" \
		--change-owner-username="fzuellich" \
		--author-username="tools" \
		--author="tools <>" \
		--change-url 123 \
		--project juco \
		--Verified 1 \
		--Verified-oldValue 0 \
		--Code-Review 2 \
		--Code-Review-oldValue 0

test_reviewer_added : # +1 and +2 for patch
	cargo run --bin reviewer_added -- \
		--change-owner "fzuellich <user@example>" \
		--change-owner-username "fzuellich" \
		--change-url 123 \
		--project juco \
		--reviewer "fzuellich <user@example>" \
		--reviewer-username "fzuellich"

deploy : # Deploy to production
	cargo build --release \
		&& ssh root@$$GERRIT_TEST_DOMAIN "systemctl stop chtbtr" \
		&& scp target/release/chtbtr root@$$GERRIT_TEST_DOMAIN:/opt/chtbtr/chtbtr \
		&& ssh root@$$GERRIT_TEST_DOMAIN "chown chtbtr: /opt/chtbtr/chtbtr" \
		&& scp target/release/comment_added root@$$GERRIT_TEST_DOMAIN:/home/git/review_site/hooks/comment-added \
		&& ssh root@$$GERRIT_TEST_DOMAIN "chown git: /home/git/review_site/hooks/comment-added" \
		&& scp target/release/reviewer_added root@$$GERRIT_TEST_DOMAIN:/home/git/review_site/hooks/reviewer-added \
		&& ssh root@$$GERRIT_TEST_DOMAIN "chown git: /home/git/review_site/hooks/reviewer-added" \
		&& ssh root@$$GERRIT_TEST_DOMAIN "systemctl start chtbtr" \
    && sleep 2 \
		&& ssh root@$$GERRIT_TEST_DOMAIN "systemctl status chtbtr"
