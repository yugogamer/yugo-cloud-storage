#[macro_use] extern crate rocket;
mod services;
mod database;
mod bucket;
mod utils;
use dotenv::dotenv;

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