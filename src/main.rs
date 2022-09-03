#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = url_shortener::rocket().ignite().await?;
    println!("Initialization...");

    #[allow(unused_variables)]
    let rocket = rocket.launch().await?;
    println!("Exiting...");

    Ok(())
}
