use model::user::user::WUser;
use model::user::login::{Login,Token};
use model::model_template::ModelTemplate;
use rocket::http::RawStr;
use service::db_connector::Connection;
use service::request_config::BasicAuth;
use model::user::user::AuthGuard;
use rocket_contrib::json::Json;
use service::response_config::ResponseWithHeader;

#[get("/login")]
pub fn user_login(_conn: BasicAuth)-> ResponseWithHeader<Token> {
    let (username,password,connection) = (_conn.username,_conn.password,_conn.connection);
    let user_login = Login{ name: username, pw: password};
    let user_w_key = match WUser::get_multi_object(connection.clone()) {
        Some(value) => user_login.is_login_correct(value),
        None => return ResponseWithHeader(None)
    };
    let some_user = match user_w_key {
        Some(value) => value,
        None => return ResponseWithHeader(None),
    };
    let copyof = Token{ token: some_user.token.clone() };
    if !WUser::update_values_object(connection, vec![some_user]) {
        return ResponseWithHeader(None);
    }
    ResponseWithHeader(Some(copyof))
}

#[put("/signin",format = "application/json", data = "<register>")]
pub fn user_create(register: Json<WUser>,_conn: Connection)-> ResponseWithHeader<bool> {
    let user = register.into_inner();
    ResponseWithHeader(Some(WUser::insert_values_object(_conn, vec![user])))
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