#![feature(async_closure, if_let_guard, generic_assert_internals)]
#[macro_use]
extern crate rocket;

use tokio::sync::RwLock;

#[cfg(debug_assertions)]
use console_subscriber;

mod auth;
mod guards;
mod models;
mod mongo;
mod routes_mod;
mod security;
mod error_catcher;

use rocket_cors::Cors;
use auth::PublicKeys;
use chrono::Duration;
use guards::{rate_limiter, PerMinRateLimit};
use mongo::MessageCmsDb;
use rocket::fairing::AdHoc;
use routes_mod::*;
use security::{RateLimitState, RateType, HeaderFairings};

#[launch]
async fn rocket() -> _ {
    #[cfg(debug_assertions)]
    console_subscriber::init();

    rocket::build()
        .attach(AdHoc::try_on_ignite(
            "Message CMS DB Connection",
            |rocket_build| async { Ok(rocket_build.manage(MessageCmsDb::init().await)) },
        ))
        .attach(AdHoc::try_on_ignite(
            "Auth0 Public JWKS",
            |rocket_build| async {
                match PublicKeys::new().await {
                    Ok(state) => Ok(rocket_build.manage(state)),
                    Err(e) => {
                        error!("Failed to load Auth0 public keys: {}", e);
                        Err(rocket_build)
                    }
                }
            },
        ))
        .attach(AdHoc::try_on_ignite(
            "Per minute rate limit state handler",
            |rocket_build| async {
                let state_wrapper = PerMinRateLimit(RwLock::new(RateLimitState::new(
                    RateType::new(Duration::minutes(1), 25),
                    Duration::minutes(1),
                )));

                Ok(rocket_build.manage(state_wrapper))
            },
        ))
        .attach(AdHoc::on_request(
            "Per minute rate limit handler",
            rate_limiter,
        ))
        .attach(AdHoc::on_response("Response headers filter fairing", HeaderFairings::header_res_filter))
        .attach(Cors::from_options(&HeaderFairings::rocket_cors_config()).expect("Failed to attach CORS"))
        .mount("/", routes![sd_msg_route])
        .mount("/health", routes![check_health_route])
        .mount(
            "/message",
            routes![
                gt_msg_route,
                get_msg_no_id_route,
                get_msg_route,
                toggle_read_archive_route,
                del_msg_route,
                del_msg_no_id_route
            ],
        )
        .register("/", catchers![
            error_catcher::not_found,
            error_catcher::internal_server_error,
            error_catcher::bad_request,
            error_catcher::unauthorized,
            error_catcher::forbidden,
            error_catcher::enhance_calm,
            error_catcher::enhance_calm2
        ])
}
