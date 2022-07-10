#[macro_use] extern crate rocket;

use rocket_db_pools::Database;
use rocket_db_pools::mongodb;

mod app_routes;

//Opening connection to CMS MongoDb database
#[derive(Database)]
#[database("tux_cms_db")]
struct MessageCmsDb(mongodb::Client);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MessageCmsDb::init())
        .mount("/", routes![app_routes::send_message])
}