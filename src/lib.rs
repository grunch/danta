
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;
pub mod routes;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::index, routes::get_all_attendees])
}