#[macro_use] extern crate rocket;

mod models;
mod mongo;
mod routes_mod;

use mongo::MessageCmsDb;
use routes_mod::*;

#[launch]
async fn rocket() -> _ {
    let cms_db = MessageCmsDb::init().await;

    rocket::build()
        .manage(cms_db)
        .mount("/", routes![
            sd_msg_route,
            
        ])
}