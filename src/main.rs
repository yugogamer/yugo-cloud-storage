#[macro_use] extern crate rocket;
mod services;
mod database;
mod bucket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::load_db())
        .attach(bucket::load_bucket())
        .mount("/", routes![index])
}