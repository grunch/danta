#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

pub mod db;
mod lightning;
pub mod models;
pub mod routes;
pub mod schema;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![routes::not_found])
        .mount("/public", FileServer::from(relative!("static")))
        .mount("/", routes![routes::index])
        .mount(
            "/api",
            routes![routes::get_all_attendees, routes::create_invoice],
        )
        .attach(Template::fairing())
}
