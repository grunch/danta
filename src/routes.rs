use crate::db::establish_connection;
use crate::models::Attendee;
use rocket::serde::json::Json;
use diesel::prelude::*;

#[get("/attendees")]
pub fn get_all_attendees() -> Json<Vec<Attendee>> {
    use crate::schema::attendees::dsl::*;
    let conn = establish_connection();
    let results = attendees
        .load::<Attendee>(&conn)
        .expect("Error loading attendees");

    Json(results)
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}