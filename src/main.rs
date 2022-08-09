#![feature(async_closure, if_let_guard)]
#[macro_use] extern crate rocket;
use console_subscriber;

mod models;
mod mongo;
mod routes_mod;
mod guards;
mod auth;
mod fairings;

use rocket::fairing::AdHoc;
use mongo::MessageCmsDb;
use routes_mod::*;
use auth::PublicKeys;

#[launch]
async fn rocket() -> _ {
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
