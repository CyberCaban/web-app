use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::http::CookieJar;
use rocket::response::content::RawHtml;
use rocket::{serde::json::Json, State};
use serde_json::{json, Value};

use crate::database::Connection;
use crate::errors::{ApiError, LoginError, RegisterError};
use crate::models::User;
use crate::models;
use crate::schema::users::{self, dsl::*};

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
    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return ApiError::from_error(&e).to_json(),
    };
    let res = users.load::<User>(&mut *conn);
    match res {
        Ok(result) => json!(result),
        Err(e) => ApiError::from_error(&e).to_json(),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub fn api_register(
    db: &State<Connection>,
    user: Json<NewUser<'_>>,
    cookies: &CookieJar<'_>,
) -> Result<Json<User>, Value> {
    use self::models::User;
    use crate::schema;

    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return Err(json!(ApiError::from_error(&e))),
    };
    match users
        .filter(users::username.eq(user.username.to_lowercase()))
        .first::<User>(&mut *conn)
    {
        Ok(_) => {
            return Err(
                ApiError::new("UserAlreadyExists", RegisterError::UserAlreadyExists).to_json(),
            )
        }
        Err(_) => {
            if user.password.len() < 8 {
                return Err(ApiError::new("WeakPassword", RegisterError::WeakPassword).to_json());
            }
            let new_user = User {
                id: uuid::Uuid::new_v4(),
                username: user.username.to_string(),
                password: user.password.to_string(),
            };
            if let Err(e) = diesel::insert_into(schema::users::table)
                .values(&new_user)
                .execute(&mut *conn)
            {
                return Err(ApiError::from_error(&e).to_json());
            }

            cookies.add(("token", new_user.id.to_string()));
            Ok(Json(new_user))
        }
    }
}

#[post("/login", format = "json", data = "<user>")]
pub fn api_login(
    db: &State<Connection>,
    user: Json<NewUser<'_>>,
    cookies: &CookieJar<'_>,
) -> Result<Json<User>, Value> {
    use self::models::User;
    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return Err(json!(ApiError::from_error(&e))),
    };
    let usrs = users
        .filter(users::username.eq(user.username.to_lowercase()))
        .first::<User>(&mut *conn);
    println!("{:?}", &usrs);
    match usrs
    {
        Err(_) => Err(ApiError::new("UserNotFound", LoginError::UserNotFound).to_json()),
        Ok(usr) => {
            if usr.password != user.password {
                Err(ApiError::new("WrongPassword", LoginError::WrongPassword).to_json())
            } else {
                cookies.add(("token", usr.id.to_string()));
                Ok(Json(usr))
            }
        }
    }
}

#[post("/logout")]
pub fn api_logout(cookies: &CookieJar<'_>) -> Value {
    cookies.remove("token");
    json!("Logged out")
}

#[get("/toro", format = "html")]
pub fn toro() -> RawHtml<String> {
    let toro: &str = "
::::::::::::::::::::::..::::::::::::::::::::::::''::''''''''''''''''''''''
::::::::::::::::~~[[%%==..::::''''''''''''::::''''::::::''''''''----~~====
::::::::::::::--[[OO##@@[[''::------------::''--''::--==''::''''----&lt;&lt;&lt;&lt;~~
::::::::::::''**%%88####QQ88''''--------''::''===={{88##88~~''''----**&lt;&lt;&lt;&lt;
::::::::::::~~**[[OO######QQ00::''------''::--((%%008888@@00::''----&lt;&lt;{{**
::::::::::''==**[[88######@@QQ%%~~~~--''''''((%%%%0000OO####&lt;&lt;''------~~--
''''::''::--==**%%88######@@@@@@##OO88008800%%%%008888OOOO##00''--~~======
''....::::--==**%%OO######@@##############@@##OO000088OOOO##OO--~~****((**
::::::::::~~==(([[88############OOOO########@@@@##OO88OOOO##@@&lt;&lt;~~((((((((
::::''''::~~&lt;&lt;{{%%88OOOO######OOOOOO######@@@@@@@@@@@@##OOOO@@{{~~********
::::''''''==&lt;&lt;(([[0088OOOOOOOOOOOOOO##@@####@@@@@@@@@@QQ@@##@@{{~~((**((((
::::''----==&lt;&lt;(([[%%0088OOOOOOOOOOOO####@@@@@@@@@@@@@@QQQQQQQQ{{~~((((((((
::::----~~&lt;&lt;&lt;&lt;(({{%%000088OOOOOOOOOO##@@##@@@@@@@@@@QQQQQQQQ&amp;&amp;**~~**&lt;&lt;**&lt;&lt;
::::::--==&lt;&lt;**{{[[%%8888OOOOOOOOOOOO######@@@@@@@@@@QQQQQQQQ&amp;&amp;{{~~(((({{((
::::::~~&lt;&lt;&lt;&lt;&lt;&lt;&lt;&lt;==~~~~==((%%OO##OOOOOO####@@@@@@@@QQ@@@@@@QQ&amp;&amp;00~~********
::::----''::--''--&lt;&lt;**==~~''~~{{88####@@@@@@QQQQ@@{{==~~==**00@@{{&lt;&lt;******
::::..--**((~~....((8800OO00[[**~~~~&lt;&lt;**[[OO{{**&lt;&lt;~~&lt;&lt;**{{**''{{{{**(((({{
::..''**{{**  ..  ~~%%%%%%%%%%00[[''..  ==%%~~[[00OOOOOO@@{{..(([[--&lt;&lt;==~~
::..--**[[''  ..  &lt;&lt;[[[[[[[[((&lt;&lt;&lt;&lt;--==&lt;&lt;**--((QQ@@########&lt;&lt;..==QQ--''~~::
::..''(([[''    --%%%%%%00%%[[&lt;&lt;~~::{{((&lt;&lt;--&lt;&lt;@@@@@@@@@@QQ{{..''##&lt;&lt;::==''
''::::(([[[[****%%%%%%0000%%{{&lt;&lt;~~''00{{&lt;&lt;--~~QQ@@@@@@@@@@##--::00((--==''
''~~----**{{[[[[000000008800{{&lt;&lt;--==OO[[&lt;&lt;((--[[QQQQ@@@@@@@@88====--''====
''==**==''--~~~~~~&lt;&lt;******&lt;&lt;~~~~&lt;&lt;8888%%((0088==((00OO##@@OO%%(({{~~~~----
--&lt;&lt;********(({{((==&lt;&lt;(((({{%%88##OO8800%%88##@@%%**((((((((**(({{&lt;&lt;''--''
--&lt;&lt;**==&lt;&lt;**{{{{%%{{[[000088OOOOOOOOOOOO8888OO88&lt;&lt;--00OO000088OO##**::~~''
''&lt;&lt;****&lt;&lt;&lt;&lt;**((((((%%0000888888OO######OOOOOO8888==[[OO0088##OO##&lt;&lt;::==''
::&lt;&lt;**(({{(((((((([[888800OO[[**##00((##@@##@@##{{&lt;&lt;OO##OO8888OO##~~::&lt;&lt;--
==~~****(({{{{(({{[[008800OO&lt;&lt;&lt;&lt;[[==**&lt;&lt;%%00%%&lt;&lt;&lt;&lt;OO########@@QQ%%''::==--
&lt;&lt;******(((((((((({{[[00008888****88@@00((**((00##OO####OO##@@@@==''::~~==
&lt;&lt;**((((((((((((****(([[%%00OOOOOO88OO##########OOOO##OOOOOO@@{{::''''--&lt;&lt;
(({{{{((((((((******(((({{008888888888OOOOOOOOOOOOOOOOOOOO##((::''''''''&lt;&lt;
((((((((((((************{{[[0000000000888888OOOO####OOOO%%==::''''''''''&lt;&lt;
**(({{{{((**********((**(({{%%%%%%%%%%00888888[[((((**==''::''''''''''::&lt;&lt;
**(({{((****((((&lt;&lt;(({{**&lt;&lt;(({{{{[[%%88OO##@@@@--..::....::::::''''''''::==
(((({{{{{{{{{{****((((&lt;&lt;==&lt;&lt;(([[88OO@@@@QQQQ&amp;&amp;88%%8800{{&lt;&lt;~~--::::::::..~~
{{(([[{{{{[[{{******&lt;&lt;==&lt;&lt;(({{00OO##@@@@QQQQQQQQQQ@@QQ@@##OO88[[((&lt;&lt;==~~~~
(((([[(({{{{**(({{[[((&lt;&lt;**(({{%%88OO##@@QQQQQQQQQQ@@########OO8888OO888888
    ";
    RawHtml(
        format!(
            "
    <html>
        <head>
            <title>Toro</title>
        </head>
        <body style='background-color: black;'>
            <pre style='font-family: monospace; color: white'>
                {toro}
            </pre>
        </body>
    </html>
    "
        )
        .to_string(),
    )
}
