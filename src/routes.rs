use crate::db::establish_connection;
use crate::lightning::{add_invoice, lnd_client};
use crate::models::Attendee;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use rocket_dyn_templates::{context, Template};
use std::env;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name: String,
    pub lastname: String,
    pub email: String,
}

// /invoice Response
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InvoiceResponse {
    hash: String,
    request: String,
    description: String,
    amount: u32,
    success: bool,
}

#[get("/")]
pub async fn index() -> Template {
    Template::render("index", context! {})
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
pub async fn create_invoice(user: Json<User>) -> Json<InvoiceResponse> {
    dotenv().ok();
    let amount = match env::var("TICKET_AMOUNT") {
        Ok(amt) => amt.parse::<u32>().unwrap(),
        Err(_) => panic!("TICKET_AMOUNT must be set"),
    };
    let mut client = lnd_client().await.unwrap();
    let memo = "#LightningHackday POAP";
    let invoice = add_invoice(&mut client, memo, amount).await.unwrap();
    let hash = invoice
        .r_hash
        .iter()
        .map(|h| format!("{h:02X}"))
        .collect::<Vec<String>>()
        .join("");

    let response = InvoiceResponse {
        hash,
        request: invoice.payment_request,
        description: memo.to_string(),
        amount,
        success: true,
    };

    Json(response)
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}
