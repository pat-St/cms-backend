use crate::{service,model};
use model::user::user::WUser;
use model::user::login::{Login,Token};
use model::model_template::ModelTemplate;
use rocket::http::RawStr;
use service::db_connector::Connection;
use service::request_config::BasicAuth;
use service::request_config::AuthGuard;
use rocket_contrib::json::Json;
use service::response_config::ResponseWithHeader;
use model::user::register::{Register, ChangePW};

#[get("/login")]
pub fn user_login(_conn: BasicAuth)-> ResponseWithHeader<Token> {
    let (username,password,connection) = (_conn.username,_conn.password,_conn.connection);
    let user_login = Login{ name: username.clone(), pw: password};
    let found_user = match WUser::get_user_by_name(connection.clone(), &*username) {
        Some(value) => value,
        None => return ResponseWithHeader(None)
    };
    let user_w_key = match user_login.is_login_correct(&found_user) {
        Some(value) => value,
        None => return ResponseWithHeader(None)
    };
    let token_obj = Token{ token: user_w_key.clone() };
    let mut some_user = found_user;
    some_user.token = user_w_key;
    if !WUser::update_values_object(connection, vec![some_user]) {
        return ResponseWithHeader(None);
    }
    ResponseWithHeader(Some(token_obj))
}

#[post("/signin",format = "application/json", data = "<register>")]
pub fn user_create(register: Json<Register>,_conn: Connection)-> ResponseWithHeader<bool> {
    let user = register.into_inner();
    match Register::add_new_user(_conn, user) {
        true => ResponseWithHeader(Some(true)),
        false => ResponseWithHeader(None)
    }
}

#[get("/logout/<username>")]
pub fn user_logout(_conn: Connection, username: &RawStr) -> ResponseWithHeader<bool> {
    let all_users = match WUser::get_multi_object(_conn.clone()) {
        Some(value) => value,
        None => return ResponseWithHeader(None),
    };
    let update_result = match Login::logout_user(username.as_str().to_string(), all_users) {
        Some(user) => WUser::update_values_object(_conn, vec![user]),
        None => false
    };
    ResponseWithHeader(Some(update_result))
}

#[put("/changepw",format = "application/json", data = "<changepw>")]
pub fn change_pw(changepw: Json<ChangePW>,_conn: Connection)-> ResponseWithHeader<bool> {
    let user = changepw.into_inner();
    ResponseWithHeader(Some(ChangePW::change_pw(_conn, user)))
}
#[get("/")]
pub fn check_token(_conn: AuthGuard)-> ResponseWithHeader<bool> {
    ResponseWithHeader(Some(true))
}

#[options("/")]
pub fn options_response() -> ResponseWithHeader<bool> {
     ResponseWithHeader(Some(true))
}
#[options("/<_username>")]
pub fn options_signing_response(_username: &RawStr) -> ResponseWithHeader<bool> {
    ResponseWithHeader(Some(true))
}

#[options("/logout/<_username>")]
pub fn post_mock(_username: &RawStr)-> ResponseWithHeader<bool> {
    ResponseWithHeader(Some(true))
}