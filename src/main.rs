#[macro_use] extern crate rocket;

mod app_routes;
mod models;
mod mongo;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![app_routes::send_message])
}