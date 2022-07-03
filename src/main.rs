#[macro_use] extern crate rocket;

mod app_routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![app_routes::send_message])
}