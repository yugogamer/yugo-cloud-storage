#[macro_use] extern crate rocket;
mod services;
mod database;
mod bucket;
mod utils;
mod errors;
use crate::errors::Error;
use dotenv::dotenv;

pub type Result<T> = core::result::Result<T, Error>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok(); 

    rocket::build()
        .attach(database::load_db())
        .attach(bucket::load_bucket())
        .mount("/", routes![index])
}