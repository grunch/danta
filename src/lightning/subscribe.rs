use crate::db::connect;
use diesel::prelude::*;
use tonic_openssl_lnd::lnrpc::InvoiceSubscription;
use tonic_openssl_lnd::LndLightningClient;

pub async fn invoice(client: &mut LndLightningClient) {
    use crate::schema::attendees::dsl::*;
    let conn = connect();
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
            println!("Preimage: {preimage_str}");
            diesel::update(attendees.filter(hash.eq(&hash_str)))
                .set((preimage.eq(preimage_str), paid.eq(true)))
                .execute(&conn)
                .expect("Error updating attendee");
        }
    }
}
