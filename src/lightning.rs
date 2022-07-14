use dotenv::dotenv;
use std::env;
use tonic_lnd::rpc::{AddInvoiceResponse, GetInfoRequest, GetInfoResponse, Invoice};

pub async fn lnd_client() -> Result<tonic_lnd::Client, tonic_lnd::ConnectError> {
    dotenv().ok();
    let http_str = "https://".to_string();
    let address = env::var("LND_GRPC_HOST").expect("LND_GRPC_HOST must be set");
    let cert = env::var("LND_CERT_FILE").expect("LND_CERT_FILE must be set");
    let macaroon = env::var("LND_MACAROON_FILE").expect("LND_MACAROON_FILE must be set");
    let address = format!("{}{}", http_str, address);
    println!("{address}");
    // Connecting to LND requires only address, cert file, and macaroon file
    let client = tonic_lnd::connect(address, cert, macaroon)
        .await
        .expect("Failed connecting to LND");

    Ok(client)
}

pub async fn get_info(client: &mut tonic_lnd::Client) -> Result<GetInfoResponse, tonic_lnd::Error> {
    let info = client.get_info(GetInfoRequest {}).await?.into_inner();

    Ok(info)
}

pub async fn add_invoice(
    client: &mut tonic_lnd::Client,
    memo: &str,
    amount: u32,
) -> Result<AddInvoiceResponse, tonic_lnd::Error> {
    let invoice = Invoice {
        memo: memo.to_string(),
        value: amount as i64,
        expiry: 3600,
        ..Default::default()
    };
    let invoice = client.add_invoice(invoice).await?.into_inner();

    Ok(invoice)
}
