#![feature(async_closure, if_let_guard)]
#[macro_use] extern crate rocket;
use console_subscriber;


mod models;
mod mongo;
mod routes_mod;
mod guards;
mod auth;

use mongo::MessageCmsDb;
use routes_mod::*;
use auth::PublicKeys;

#[launch]
async fn rocket() -> _ {
    console_subscriber::init();

    let cms_db = MessageCmsDb::init().await;
    let pub_jwks = PublicKeys::new().await
        .expect("Failed starting auth0 public jwk set fetcher!");


    rocket::build()
        .manage(cms_db)
        .manage(pub_jwks)
        .mount("/", routes![
            sd_msg_route,
            
        ])
        .mount("/message", routes![
            gt_msg_route,
        ])
        .mount("/health", routes![
            check_health_route,
        ])
}