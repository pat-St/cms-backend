use crate::{service,model};
use model::user::user::{WUser};
use model::model_template::ModelTemplate;
use service::db_connector::Connection;
use rocket::Outcome;
use rocket::Outcome::{Success, Forward, Failure};
use rocket::http::{Status, HeaderMap};
use rocket::request::{self, Request, FromRequest};
use base64;

#[derive(Debug, Clone)]
pub struct AuthGuard(pub Connection);

impl AuthGuard {
    pub fn failure_result<S,R>(x: R) -> request::Outcome<S, R> { 
        Outcome::Failure((Status::BadRequest, x))
    }
    pub fn is_key_valid(key: &String, users: &[WUser]) -> bool {
        for i in users {
            if i.token.eq(key) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthGuard {
    type Error = ApiKeyError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthGuard, Self::Error> {        
        let conn: Connection = match request.guard::<Connection>() {
            Success(c) => c,
            Forward(_) => return AuthGuard::failure_result(ApiKeyError::Missing),
            Failure(_) => return AuthGuard::failure_result(ApiKeyError::Invalid),
        };
        let api_key: String = match AuthGuard::get_bearer_token(request.headers()) {
            Ok(val) => val,
            Err(e) => return AuthGuard::failure_result(e)
        };
        match request.local_cache(|| WUser::get_multi_object(conn.clone())) {
            Some(users) if AuthGuard::is_key_valid(&api_key,users.as_slice()) => Outcome::Success(AuthGuard(conn.clone())),
            Some(_) => AuthGuard::failure_result(ApiKeyError::Invalid),
            None => AuthGuard::failure_result(ApiKeyError::BadCount),
        }
    }
}

impl AuthGuard {
    pub fn get_bearer_token(h: &HeaderMap) -> Result<String,ApiKeyError> {
        if h.contains("apikey") {
            return Self::extract_api_key(h);
        }
        if h.contains("Authorization") {
            return Self::extract_auth_key(h);
        }
        return Err(ApiKeyError::Missing);
    }

    fn extract_api_key(h: &HeaderMap) -> Result<String,ApiKeyError> {
        let key: Vec<_> = h.get("apikey").collect();
        match key.len() {
            0 => return Err(ApiKeyError::Missing),
            1 => Ok(key[0].to_string()),
            _ => return Err(ApiKeyError::BadCount),
        }
    }

    fn extract_auth_key(h: &HeaderMap) -> Result<String,ApiKeyError> {
        let key: Vec<_> = h.get("Authorization").collect();
        let auth_key = match key.len() {
            0 => return Err(ApiKeyError::Missing),
            1 => key[0].to_string(),
            _ => return Err(ApiKeyError::BadCount),
        };
        let auth_split = auth_key.split_whitespace().collect::<Vec<&str>>();
        if auth_split.len() != 2 && auth_split[0] != "Bearer" {
            return Err(ApiKeyError::Missing)
        }
        Ok(auth_split[1].to_string())
    }
}

pub struct BasicAuth {
    pub username: String,
    pub password: String,
    pub connection: Connection
}

impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = ApiKeyError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<BasicAuth, Self::Error> {
        let unauth_result = |x| -> request::Outcome<BasicAuth, ApiKeyError> { 
            Outcome::Failure((Status::Unauthorized, x))
        };
        let conn: Connection = match request.guard::<Connection>() {
            Success(c) => c,
            Forward(_) => return AuthGuard::failure_result(ApiKeyError::Missing),
            Failure(_) => return AuthGuard::failure_result(ApiKeyError::Invalid),
        };
        let auth: String = match request.headers().get_one("Authorization") {
            Some(value) => value.to_owned(),
            None => return unauth_result(ApiKeyError::Invalid)
        };
        let auth_split = auth.split_whitespace().collect::<Vec<&str>>();
        if auth_split.len() != 2 && auth_split[0] != "Basic" {
            return unauth_result(ApiKeyError::Missing)
        }
        let decode: Vec<u8> = match base64::decode(auth_split[1]) {
            Ok(value) => value,
            Err(_) => return AuthGuard::failure_result(ApiKeyError::Invalid)
        };
        let basic_decode = match std::str::from_utf8(&decode) {
            Ok(value) => value.split(":").collect::<Vec<&str>>(),
            Err(_) => return AuthGuard::failure_result(ApiKeyError::Invalid)
        };
        if basic_decode.len() != 2 {
            return AuthGuard::failure_result(ApiKeyError::Invalid)
        }
        Outcome::Success(BasicAuth{username: basic_decode[0].to_owned(), password: basic_decode[1].to_owned(),connection: conn.clone()})
    }
}
