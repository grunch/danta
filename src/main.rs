use cacri;
use cacri::db::connect as dbconnect;
use cacri::lightning::ln::connect;
use diesel::prelude::*;
use rocket;
use tonic_lnd::rpc::InvoiceSubscription;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut client = connect().await.unwrap();
    tokio::spawn(async move {
        let mut invoice_stream = client
            .subscribe_invoices(InvoiceSubscription {
                add_index: 0,
                settle_index: 0,
            })
            .await
            .expect("Failed to call subscribe_invoices")
            .into_inner();

        println!("{invoice_stream:?}");
        while let Some(invoice) = invoice_stream
            .message()
            .await
            .expect("Failed to receive invoices")
        {
            if invoice.settle_date > 0 {
                let hash_str = invoice
                    .r_hash
                    .iter()
                    .map(|h| format!("{h:02x}"))
                    .collect::<Vec<String>>()
                    .join("");

                let preimage_str = invoice
                    .r_preimage
                    .iter()
                    .map(|p| format!("{p:02x}"))
                    .collect::<Vec<String>>()
                    .join("");
                // Now we know the user paid we update db
                let conn = dbconnect();
                use cacri::schema::attendees::dsl::*;
                diesel::update(attendees.filter(hash.eq(&hash_str)))
                    .set((preimage.eq(preimage_str), paid.eq(true)))
                    .execute(&conn)
                    .expect("Error updating attendee");
            }
        }
    });

    let _ = cacri::rocket().launch().await?;

    Ok(())
}
