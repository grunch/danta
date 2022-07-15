use crate::schema::attendees;
use chrono::prelude::*;
use rocket::serde::{Deserialize, Serialize};
#[derive(Insertable)]
#[table_name = "attendees"]
pub struct NewAttendee<'a> {
    pub hash: &'a str,
    pub preimage: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
    pub paid: bool,
    pub created_at: &'a NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Attendee {
    pub id: i32,
    pub hash: String,
    pub preimage: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub paid: bool,
    pub created_at: NaiveDateTime,
}
