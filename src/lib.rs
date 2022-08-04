#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

pub mod db;
pub mod lightning;
pub mod models;
pub mod pdf;
pub mod routes;
pub mod schema;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![routes::not_found])
        .mount("/files", FileServer::from(relative!("files")))
        .mount("/public", FileServer::from(relative!("static")))
        .mount("/", routes![routes::index])
        .mount(
            "/api",
            routes![
                routes::get_all_attendees,
                routes::create_invoice,
                routes::lookup_invoice,
                routes::verify,
            ],
        )
        .attach(Template::fairing())
}
