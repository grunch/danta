use rocket;
use cacri;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = cacri::rocket().launch().await?;
    Ok(())
}