use crate::db::connect as dbconnect;
use crate::excel;
use crate::lightning::ln::{add_invoice, connect, get_invoice};
use crate::models::{Attendee, NewAttendee};
use crate::pdf::generate_pdf;
use chrono::prelude::*;
use diesel::prelude::*;
use dotenv::dotenv;
use hex::FromHex;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use rocket_dyn_templates::{context, Template};
use std::env;
use std::fs;
use tonic_openssl_lnd::lnrpc::invoice::InvoiceState;

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct AttendeeResponse {
    pub firstname: String,
    pub data1: String,
    pub email: String,
    pub preimage: String,
    pub paid: bool,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub firstname: String,
    pub data1: String,
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
    let closing_date = env::var("CLOSING_DATE").expect("CLOSING_DATE must be set");
    let max_capacity = env::var("MAX_CAPACITY")
        .expect("MAX_CAPACITY must be set")
        .parse::<i32>()
        .expect("MAX_CAPACITY must be a number");

    let closing_date = DateTime::parse_from_rfc3339(&closing_date)
        .unwrap()
        .timestamp();
    let now = Local::now().timestamp();
    // We need to know how many users are already registered
    use crate::schema::attendees::dsl::*;
    let conn = dbconnect();
    let results = attendees
        .filter(paid.eq(true))
        .load::<Attendee>(&conn)
        .expect("Error loading attendees");

    let close = now > closing_date || results.len() as i32 >= max_capacity;
    let speakers =
        fs::read_to_string("static/speakers.json").expect("Unable to read speakers file");
    let organizers =
        fs::read_to_string("static/organizers.json").expect("Unable to read organizers file");
    let sponsors =
        fs::read_to_string("static/sponsors.json").expect("Unable to read sponsors file");
    let talks = fs::read_to_string("static/talks.json").expect("Unable to read talks file");

    let speakers: serde_json::Value =
        serde_json::from_str(&speakers).expect("speakers JSON was not well-formatted");
    let organizers: serde_json::Value =
        serde_json::from_str(&organizers).expect("organizers JSON was not well-formatted");
    let sponsors: serde_json::Value =
        serde_json::from_str(&sponsors).expect("sponsors JSON was not well-formatted");
    let talks: serde_json::Value =
        serde_json::from_str(&talks).expect("talks JSON was not well-formatted");
    Template::render(
        "index",
        context! { close, speakers, organizers, sponsors, talks },
    )
}

#[get("/check_user")]
pub async fn check_user() -> Template {
    Template::render("check_user", context! {})
}

#[get("/verify/<secret>")]
pub async fn verify_user(secret: &str) -> Template {
    use crate::schema::attendees::dsl::*;
    let conn = dbconnect();
    let results = attendees
        .filter(preimage.eq(secret))
        .load::<Attendee>(&conn)
        .expect("Error loading attendees");
    let attendee = if results.is_empty() {
        AttendeeResponse {
            ..Default::default()
        }
    } else {
        let _attendee = results.get(0).unwrap();
        AttendeeResponse {
            firstname: _attendee.firstname.clone(),
            data1: _attendee.data1.clone(),
            email: _attendee.email.clone(),
            preimage: _attendee.preimage.clone(),
            paid: _attendee.paid,
        }
    };

    Template::render("verify_user", context! { attendee })
}

#[get("/attendees/<token>")]
pub fn get_all_attendees(token: &str) -> Json<Vec<Attendee>> {
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
    // Invoice creation
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

    // We need to know if this same user already tried to pay
    use crate::schema::attendees::dsl::*;
    let conn = dbconnect();

    let results = attendees
        .filter(email.eq(&user.email.to_lowercase()))
        .load::<Attendee>(&conn)
        .expect("Error loading attendees");

    if results.is_empty() {
        let new_attendee = NewAttendee {
            hash: &hash_str,
            preimage: "",
            firstname: &user.firstname,
            lastname: "",
            email: &user.email.to_lowercase(),
            data1: &user.data1,
            paid: false,
            created_at: &chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(attendees)
            .values(&new_attendee)
            .execute(&conn)
            .expect("Error saving new attendee");

        let response = AddInvoiceResponse {
            hash: hash_str.clone(),
            request: invoice_response.payment_request,
            description: memo.to_string(),
            amount,
            success: true,
        };

        Json(response)
    } else {
        let attendee = results.get(0).unwrap();
        if attendee.paid {
            return Json(AddInvoiceResponse {
                hash: attendee.hash.clone(),
                request: "".to_string(),
                description: "".to_string(),
                amount,
                success: true,
            });
        }
        let email_str = attendee.email.to_lowercase();
        let target = attendees.filter(email.eq(&email_str));
        diesel::update(target)
            .set(hash.eq(&hash_str))
            .execute(&conn)
            .unwrap();
        let response = AddInvoiceResponse {
            hash: hash_str.clone(),
            request: invoice_response.payment_request,
            description: memo.to_string(),
            amount,
            success: true,
        };

        Json(response)
    }
}

#[get("/invoice/<hash>")]
pub async fn lookup_invoice(hash: &str) -> Json<InvoiceResponse> {
    let mut client = connect().await.unwrap();
    let hash = <[u8; 32]>::from_hex(hash).expect("Decoding failed");
    let invoice = get_invoice(&mut client, &hash).await.unwrap();
    let mut preimage = String::new();

    if let Some(state) = InvoiceState::from_i32(invoice.state) {
        if state == InvoiceState::Settled {
            preimage = invoice
                .r_preimage
                .iter()
                .map(|h| format!("{h:02x}"))
                .collect::<Vec<String>>()
                .join("");
        }
    }
    Json(InvoiceResponse {
        paid: invoice.settle_date > 0,
        preimage,
        description: invoice.memo,
    })
}

#[get("/verify?<secret>&<email_str>")]
pub fn verify(secret: Option<String>, email_str: Option<String>) -> Json<AttendeeResponse> {
    use crate::schema::attendees::dsl::*;
    let conn = dbconnect();
    // let mut results;
    let mut query = attendees.into_boxed();
    if let Some(s) = secret {
        query = query.filter(preimage.eq(s.clone()));
    } else if let Some(e) = email_str {
        query = query.filter(email.eq(e.to_lowercase().clone()));
    } else {
        return Json(AttendeeResponse {
            ..Default::default()
        });
    }
    // We add this filter bc it could be more than one records
    // with the same email with paid false and true
    // This will be removed after we change the email to be unique
    query = query.filter(paid.eq(true));

    let results = query
        .load::<Attendee>(&conn)
        .expect("Error loading attendees");

    if results.is_empty() {
        return Json(AttendeeResponse {
            ..Default::default()
        });
    }
    let attendee = results.get(0).unwrap();

    // In case the pdf doesn't exists, we generate the pdf
    generate_pdf(&attendee.preimage);

    Json(AttendeeResponse {
        firstname: attendee.firstname.to_string(),
        data1: attendee.data1.to_string(),
        email: attendee.email.to_string(),
        preimage: attendee.preimage.to_string(),
        paid: attendee.paid,
    })
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}
