#![feature(proc_macro_hygiene, decl_macro)]

mod routes;
use rocket::ignite;

#[macro_use] extern crate rocket;



fn main() {
    ignite().mount("/api/mailer", rocket::routes![routes::send_message]).launch();
}