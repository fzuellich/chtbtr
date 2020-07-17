extern crate log;

extern crate chtbtr;

use acteur::Acteur;
use actix_web::{web, App, HttpServer};
use clap::crate_version;

use actor::messages::SetAppState;
use chtbtr::{
    actor,
    cli::parse_cli_args,
    controller,
    types::{AppState, ConnectionParameters},
};

///
/// Start a Chtbtr server.
///
/// The server will go through an initialization phase. During this phase the
/// server may panic.
///
/// Part of the initialization process is:
/// * Loading the mapping data from disk.
/// * Retrieving an OAuth token to be able to communicate with Just.
///
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let connection: ConnectionParameters = parse_cli_args();
    println!(
        "Starting chtbtr {}
... for '{}'.
... using profile '{}' on '{}'.
... listening on port 8088.",
        crate_version!(),
        connection.gerrit_domain,
        connection.profile_id,
        connection.domain
    );

    let sys = Acteur::new();
    sys.send_to_actor_sync::<actor::AppState, _>(0, SetAppState(connection.clone()));

    let app_state = web::Data::new(AppState {
        acteur: sys.clone(),
        connection,
    });

    println!("\nWe have a liftoff! 🚀");

    let result = HttpServer::new(move || {
        App::new().app_data(app_state.clone()).service(
            web::scope("/trigger")
                .route(
                    "/comment_added",
                    web::post().to(controller::comment_controller),
                )
                .route(
                    "/reviewer_added",
                    web::post().to(controller::reviewer_controller),
                ),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await;

    println!("Server is done.");

    sys.stop();
    println!("Stop command is send.");

    sys.wait_until_stopped();
    println!("Stopped, return result.");

    result
}
