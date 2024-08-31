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
        Err(e) => json!(e.to_string()),
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

#[get("/toro")]
pub fn toro() -> &'static str {
    "
::::::::::::::::::::::..::::::::::::::::::::::::''::''''''''''''''''''''''
::::::::::::::::~~[[%%==..::::''''''''''''::::''''::::::''''''''----~~====
::::::::::::::--[[OO##@@[[''::------------::''--''::--==''::''''----<<<<~~
::::::::::::''**%%88####QQ88''''--------''::''===={{88##88~~''''----**<<<<
::::::::::::~~**[[OO######QQ00::''------''::--((%%008888@@00::''----<<{{**
::::::::::''==**[[88######@@QQ%%~~~~--''''''((%%%%0000OO####<<''------~~--
''''::''::--==**%%88######@@@@@@##OO88008800%%%%008888OOOO##00''--~~======
''....::::--==**%%OO######@@##############@@##OO000088OOOO##OO--~~****((**
::::::::::~~==(([[88############OOOO########@@@@##OO88OOOO##@@<<~~((((((((
::::''''::~~<<{{%%88OOOO######OOOOOO######@@@@@@@@@@@@##OOOO@@{{~~********
::::''''''==<<(([[0088OOOOOOOOOOOOOO##@@####@@@@@@@@@@QQ@@##@@{{~~((**((((
::::''----==<<(([[%%0088OOOOOOOOOOOO####@@@@@@@@@@@@@@QQQQQQQQ{{~~((((((((
::::----~~<<<<(({{%%000088OOOOOOOOOO##@@##@@@@@@@@@@QQQQQQQQ&&**~~**<<**<<
::::::--==<<**{{[[%%8888OOOOOOOOOOOO######@@@@@@@@@@QQQQQQQQ&&{{~~(((({{((
::::::~~<<<<<<<<==~~~~==((%%OO##OOOOOO####@@@@@@@@QQ@@@@@@QQ&&00~~********
::::----''::--''--<<**==~~''~~{{88####@@@@@@QQQQ@@{{==~~==**00@@{{<<******
::::..--**((~~....((8800OO00[[**~~~~<<**[[OO{{**<<~~<<**{{**''{{{{**(((({{
::..''**{{**  ..  ~~%%%%%%%%%%00[[''..  ==%%~~[[00OOOOOO@@{{..(([[--<<==~~
::..--**[[''  ..  <<[[[[[[[[((<<<<--==<<**--((QQ@@########<<..==QQ--''~~::
::..''(([[''    --%%%%%%00%%[[<<~~::{{((<<--<<@@@@@@@@@@QQ{{..''##<<::==''
''::::(([[[[****%%%%%%0000%%{{<<~~''00{{<<--~~QQ@@@@@@@@@@##--::00((--==''
''~~----**{{[[[[000000008800{{<<--==OO[[<<((--[[QQQQ@@@@@@@@88====--''====
''==**==''--~~~~~~<<******<<~~~~<<8888%%((0088==((00OO##@@OO%%(({{~~~~----
--<<********(({{((==<<(((({{%%88##OO8800%%88##@@%%**((((((((**(({{<<''--''
--<<**==<<**{{{{%%{{[[000088OOOOOOOOOOOO8888OO88<<--00OO000088OO##**::~~''
''<<****<<<<**((((((%%0000888888OO######OOOOOO8888==[[OO0088##OO##<<::==''
::<<**(({{(((((((([[888800OO[[**##00((##@@##@@##{{<<OO##OO8888OO##~~::<<--
==~~****(({{{{(({{[[008800OO<<<<[[==**<<%%00%%<<<<OO########@@QQ%%''::==--
<<******(((((((((({{[[00008888****88@@00((**((00##OO####OO##@@@@==''::~~==
<<**((((((((((((****(([[%%00OOOOOO88OO##########OOOO##OOOOOO@@{{::''''--<<
(({{{{((((((((******(((({{008888888888OOOOOOOOOOOOOOOOOOOO##((::''''''''<<
((((((((((((************{{[[0000000000888888OOOO####OOOO%%==::''''''''''<<
**(({{{{((**********((**(({{%%%%%%%%%%00888888[[((((**==''::''''''''''::<<
**(({{((****((((<<(({{**<<(({{{{[[%%88OO##@@@@--..::....::::::''''''''::==
(((({{{{{{{{{{****((((<<==<<(([[88OO@@@@QQQQ&&88%%8800{{<<~~--::::::::..~~
{{(([[{{{{[[{{******<<==<<(({{00OO##@@@@QQQQQQQQQQ@@QQ@@##OO88[[((<<==~~~~
(((([[(({{{{**(({{[[((<<**(({{%%88OO##@@QQQQQQQQQQ@@########OO8888OO888888
"
}
