use rocket::fs::{relative, FileServer};
use rocket::serde::json::{json, Value};

mod config;

#[macro_use]
extern crate rocket;

#[get("/hello")]
fn api_hello() -> Value {
    json!("Hello, World!")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .configure(config::from_env())
        .mount("/", FileServer::from(relative!("www/dist")))
        .mount("/api", routes![api_hello])
        .launch()
        .await?;

    Ok(())
}
