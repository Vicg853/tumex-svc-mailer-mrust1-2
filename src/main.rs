#[macro_use] extern crate rocket;

mod app_routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/mailer", routes![app_routes::send_message])
}