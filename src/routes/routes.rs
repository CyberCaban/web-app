use diesel::associations::HasTable;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::{serde::json::Json, State};
use serde_json::{json, Value};

use crate::database::Connection;
use crate::models;
use crate::models::User;
use crate::schema;
use crate::schema::users::{self, dsl::*};

#[derive(serde::Serialize)]
enum ApiResponse {
    Ok,
    Err,
}

#[derive(serde::Serialize)]
enum RegisterError {
    UserAlreadyExists,
    WeakPassword,
}

#[derive(serde::Deserialize)]
pub struct Msg<'a> {
    msg: &'a str,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewUser<'a> {
    username: &'a str,
    password: &'a str,
}

#[get("/hello")]
pub fn api_hello() -> Value {
    json!("Hello, World!")
}

#[post("/hello", format = "json", data = "<msg>")]
pub fn api_hello_post(msg: Json<Msg<'_>>) -> Value {
    json!({"msg": format!("Hello, {}!", msg.msg)})
}

#[get("/get_users")]
pub fn api_get_users(db: &State<Connection>) -> Value {
    use self::models::User;
    let conn = &mut *db.get();
    match users.load::<User>(conn) {
        Ok(result) => json!(result),
        Err(e) => json!( e.to_string() ),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub fn api_register(db: &State<Connection>, user: Json<NewUser<'_>>) -> Result<Json<User>, Value> {
    use self::models::User;
    let conn = &mut *db.get();
    match users
        .filter(users::username.eq(user.username.to_lowercase()))
        .first::<User>(conn)
    {
        Ok(_) => return Err(json!(RegisterError::UserAlreadyExists)),
        Err(_) => {
            if user.password.len() < 8 {
                return Err(json!(RegisterError::WeakPassword));
            }

            let new_user = User {
                id: uuid::Uuid::new_v4(),
                username: user.username.to_string(),
                password: user.password.to_string(),
            };
            diesel::insert_into(schema::users::table)
                .values(&new_user)
                .execute(conn)
                .expect("Error saving new user");
            Ok(Json(new_user))
        }
    }
}
