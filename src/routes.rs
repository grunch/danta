use crate::db::establish_connection;
use crate::models::{Attendee, User};
use crate::lightning::{lnd_client, get_info, add_invoice};
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::Request;
use rocket_dyn_templates::{Template, context};

#[get("/")]
pub async fn index() -> Template {
    let mut client = lnd_client().await.unwrap();
    let info = get_info(&mut client).await.unwrap();
    println!("{:#?}", info);
    Template::render("index", context! { name: "grunch" })
}

#[get("/attendee")]
pub fn get_all_attendees() -> Json<Vec<Attendee>> {
    use crate::schema::attendees::dsl::*;
    let conn = establish_connection();
    let results = attendees
        .load::<Attendee>(&conn)
        .expect("Error loading attendees");

    Json(results)
}

#[post("/invoice", format = "application/json", data = "<user>")]
pub async fn create_invoice(user: Json<User>) -> String {
    let mut client = lnd_client().await.unwrap();
    let invoice = add_invoice(&mut client, "prueba", 66666).await.unwrap();
    println!("{invoice:?}");
    invoice.payment_request
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}
