use routes::AuthorizationRoutes;

mod config;
mod models;
mod routes;
mod schema;
mod services;
mod database;
mod errors;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .configure(config::from_env())
        .manage_db()
        .mount_static_files()
        .mount_auth_routes()
        .launch()
        .await?;

    Ok(())
}
