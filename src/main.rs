#![feature(async_closure, if_let_guard)]
#[macro_use] extern crate rocket;

use std::str::FromStr;
use tokio::sync::RwLock;

#[cfg(debug_assertions)]
use console_subscriber;

mod models;
mod mongo;
mod routes_mod;
mod guards;
mod auth;
mod security;

use rocket::fairing::AdHoc;
use chrono::Utc;
use mongo::MessageCmsDb;
use routes_mod::*;
use auth::PublicKeys;
use guards::{PerMinRateLimit, rate_limiter};
use security::{RateType, RateLimitState};

#[launch]
async fn rocket() -> _ {
    #[cfg(debug_assertions)]
    console_subscriber::init();
    
    rocket::build()
        .attach(AdHoc::try_on_ignite("Message CMS DB Connection", |rocket_build| async {
            Ok(rocket_build.manage(MessageCmsDb::init().await))
        }))
        .attach(AdHoc::try_on_ignite("Auth0 Public JWKS", |rocket_build| async {
            match PublicKeys::new().await {
                Ok(state) => Ok(rocket_build.manage(state)),
                Err(e) => {
                    error!("Failed to load Auth0 public keys: {}", e);
                    Err(rocket_build)
                }
            }
        }))
        .attach(AdHoc::try_on_ignite("Per minute rate limit state handler", |rocket_build| async {
            let state_wrapper = PerMinRateLimit(RwLock::new(RateLimitState::new(
                RateType::PerMinute(20), 
                Utc::now()
            )));
            
            Ok(rocket_build.manage(state_wrapper))
        }))
        .attach(AdHoc::on_request("Per minute rate limit handler", rate_limiter))
        .mount("/", routes![
            sd_msg_route
        ])
        .mount("/health", routes![
            check_health_route
        ])
        .mount("/message", routes![
            gt_msg_route,
            get_msg_no_id_route,
            get_msg_route,
            toggle_read_archive_route,
            del_msg_route,
            del_msg_no_id_route
        ])
}
