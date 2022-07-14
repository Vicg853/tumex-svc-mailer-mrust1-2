#[macro_use] extern crate rocket;

mod app_routes;
mod models;
mod mongo;

use mongo::MessageCmsDb;

#[launch]
async fn rocket() -> _ {
    let cms_db = MessageCmsDb::init().await;

    rocket::build()
        .manage(cms_db)
        .mount("/", routes![app_routes::send_message])
}