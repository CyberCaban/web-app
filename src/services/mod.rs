use std::env;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub fn connect_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
