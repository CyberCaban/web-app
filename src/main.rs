use diesel::RunQueryDsl;
use rocket::fs::{relative, FileServer};
use rocket::serde::json::{json, Json, Value};

mod config;
mod models;
mod schema;
mod services;

pub use models::*;

#[macro_use]
extern crate rocket;

#[get("/hello")]
fn api_hello() -> Value {
    json!("Hello, World!")
}

#[derive(serde::Deserialize)]
struct Msg {
    msg: String,
}

#[post("/hello", format = "json", data = "<msg>")]
fn api_hello_post(msg: Json<Msg>) -> Value {
    json!({"msg": format!("Hello, {}!", msg.msg)})
}

#[get("/get_users")]
fn api_get_users() -> Value {
    use self::models::User;

    let conn = &mut services::connect_pg();

    let users = self::schema::users::dsl::users
        .load::<User>(conn)
        .expect("Error loading users");
    json!(users)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .configure(config::from_env())
        .mount("/", FileServer::from(relative!("www/dist")))
        .mount("/api", routes![api_hello, api_hello_post, api_get_users])
        .launch()
        .await?;

    Ok(())
}
