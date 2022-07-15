use crate::db::connect as dbconnect;
use crate::lightning::ln::{add_invoice, get_invoice, connect};
use crate::models::{Attendee, NewAttendee};
use diesel::prelude::*;
use dotenv::dotenv;
use hex::FromHex;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use rocket_dyn_templates::{context, Template};
use std::env;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InvoiceResponse {
    paid: bool,
    preimage: String,
    description: String,
}
// /invoice Response
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddInvoiceResponse {
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
    let conn = dbconnect();
    let results = attendees
        .load::<Attendee>(&conn)
        .expect("Error loading attendees");

    Json(results)
}

#[post("/invoice", format = "application/json", data = "<user>")]
pub async fn create_invoice(user: Json<User>) -> Json<AddInvoiceResponse> {
    dotenv().ok();
    let amount = match env::var("TICKET_AMOUNT") {
        Ok(amt) => amt.parse::<u32>().unwrap(),
        Err(_) => panic!("TICKET_AMOUNT must be set"),
    };
    let mut client = connect().await.unwrap();
    let memo = "#LightningHackday POAP";
    let invoice = add_invoice(&mut client, memo, amount).await.unwrap();
    let hash_str = invoice
        .r_hash
        .iter()
        .map(|h| format!("{h:02X}"))
        .collect::<Vec<String>>()
        .join("");

    use crate::schema::attendees::dsl::*;
    let conn = dbconnect();
    let new_attendee = NewAttendee {
        hash: &hash_str,
        preimage: "",
        firstname: &user.firstname,
        lastname: &user.lastname,
        email: &user.email,
        paid: false,
        created_at: &chrono::Utc::now().naive_utc(),
    };
    let response = AddInvoiceResponse {
        hash: hash_str.clone(),
        request: invoice.payment_request,
        description: memo.to_string(),
        amount,
        success: true,
    };

    diesel::insert_into(attendees)
        .values(&new_attendee)
        .execute(&conn)
        .expect("Error saving new attendee");

    Json(response)
}

#[get("/invoice/<hash>")]
pub async fn lookup_invoice(hash: &str) -> Json<InvoiceResponse> {
    let mut client = connect().await.unwrap();
    let hash = <[u8; 32]>::from_hex(hash).expect("Decoding failed");
    let invoice = get_invoice(&mut client, &hash).await.unwrap();
    let preimage = invoice
        .r_preimage
        .iter()
        .map(|h| format!("{h:02X}"))
        .collect::<Vec<String>>()
        .join("");

    let preimage = if invoice.settled {
        preimage
    } else {
        "".to_string()
    };
    Json(InvoiceResponse {
        paid: invoice.settled,
        preimage,
        description: invoice.memo,
    })
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}
