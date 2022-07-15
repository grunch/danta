use cacri;
use rocket;
use cacri::lightning::subscribe::invoice;
use cacri::lightning::ln::connect;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = cacri::rocket().launch().await?;
    let mut client = connect().await.unwrap();
    // invoice(&mut client).await;

    Ok(())
}
