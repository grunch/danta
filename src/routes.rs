use crate::db::connect as dbconnect;
use crate::excel;
use crate::lightning::ln::{add_invoice, connect, get_invoice};
use crate::models::{Attendee, NewAttendee};
use crate::pdf::generate_pdf;
use diesel::prelude::*;
use dotenv::dotenv;
use hex::FromHex;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use rocket_dyn_templates::{context, Template};
use std::env;
use tonic_openssl_lnd::lnrpc::invoice::InvoiceState;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct VerifyResponse {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub verified: bool,
}

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

#[get("/attendees/<token>")]
pub fn show_all_attendees(token: &str) -> Template {
    dotenv().ok();
    let ttoken = env::var("TOKEN").expect("TOKEN must be set");
    if ttoken != token {
        panic!("Invalid token");
    }

    use crate::schema::attendees::dsl::*;
    let conn = dbconnect();
    let results = attendees
        .load::<Attendee>(&conn)
        .expect("Error loading attendees");

    excel::generate_file(&results);
    Template::render("attendees", context! { attendees: results })
}

#[post("/invoice", format = "application/json", data = "<user>")]
pub async fn create_invoice(user: Json<User>) -> Json<AddInvoiceResponse> {
    dotenv().ok();
    let amount = match env::var("EVENT_TICKET_AMOUNT") {
        Ok(amt) => amt.parse::<u32>().unwrap(),
        Err(_) => panic!("EVENT_TICKET_AMOUNT must be set"),
    };
    let memo = env::var("EVENT_TICKET_DESCRIPTION").expect("EVENT_TICKET_DESCRIPTION must be set");
    let mut client = connect().await.unwrap();
    let invoice_response = add_invoice(&mut client, &memo, amount).await.unwrap();
    let hash_str = invoice_response
        .r_hash
        .iter()
        .map(|h| format!("{h:02x}"))
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
        request: invoice_response.payment_request,
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
    let mut preimage = invoice
        .r_preimage
        .iter()
        .map(|h| format!("{h:02x}"))
        .collect::<Vec<String>>()
        .join("");

    if let Some(state) = InvoiceState::from_i32(invoice.state) {
        if state == InvoiceState::Settled {
            generate_pdf(&preimage);
        } else {
            preimage = "".to_string();
        }
    }
    Json(InvoiceResponse {
        paid: invoice.settle_date > 0,
        preimage,
        description: invoice.memo,
    })
}

#[get("/verify/<secret>")]
pub async fn verify(secret: &str) -> Json<VerifyResponse> {
    use crate::schema::attendees::dsl::*;
    let conn = dbconnect();
    let results = attendees
        .filter(preimage.eq(&secret))
        .load::<Attendee>(&conn)
        .expect("Error loading attendees");

    let attendee = results.get(0).unwrap();
    Json(VerifyResponse {
        firstname: attendee.firstname.to_string(),
        lastname: attendee.lastname.to_string(),
        email: attendee.email.to_string(),
        verified: true,
    })
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}
