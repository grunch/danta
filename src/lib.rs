#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use rocket::fs::relative;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
mod lightning;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![routes::not_found])
        .mount("/public", FileServer::from(relative!("static")))
        .mount("/", routes![routes::index])
        .mount("/api", routes![routes::get_all_attendees, routes::create_invoice])
        .attach(Template::fairing())
}
