use danta::db::connect as dbconnect;
use danta::lightning::ln::connect;
use danta::pdf::generate_pdf;
use diesel::prelude::*;
use tonic_openssl_lnd::lnrpc::invoice::InvoiceState;
use tonic_openssl_lnd::lnrpc::InvoiceSubscription;

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

        while let Some(invoice) = invoice_stream
            .message()
            .await
            .expect("Failed to receive invoices")
        {
            if let Some(state) = InvoiceState::from_i32(invoice.state) {
                if state == InvoiceState::Settled {
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
                    use danta::schema::attendees::dsl::*;
                    diesel::update(attendees.filter(hash.eq(&hash_str)))
                        .set((preimage.eq(&preimage_str), paid.eq(true)))
                        .execute(&conn)
                        .expect("Error updating attendee");
                    // finally we create the pdf ticket
                    generate_pdf(&preimage_str);
                }
            }
        }
    });

    let _ = danta::rocket().launch().await?;

    Ok(())
}
