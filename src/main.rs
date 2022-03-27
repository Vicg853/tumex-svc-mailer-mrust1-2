#![feature(proc_macro_hygiene, decl_macro)]

use rocket::routes;
use rocket::ignite;

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    ignite().mount("/hello", routes![hello]).launch();
}