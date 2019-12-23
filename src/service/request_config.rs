use model::user::user::{AuthGuard, WUser};
use model::model_template::ModelTemplate;
use service::db_connector::Connection;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use base64;

impl AuthGuard {
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
        let conn = request.guard::<Connection>().unwrap();
        let key: Vec<_> = request.headers().get("apikey").collect();
        let api_key: String = match key.len() {
            0 => return Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 => key[0].to_string(),
            _ => return Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        };
        let cached_users = request.local_cache(|| WUser::get_multi_object(conn.clone()));
        match cached_users {
            Some(users) if AuthGuard::is_key_valid(&api_key,users.as_slice()) => Outcome::Success(AuthGuard(conn.clone())),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

pub struct BasicAuth {
    pub username: String,
    pub password: String,
    pub connection: Connection
}

impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<BasicAuth, Self::Error> {
        let conn = request.guard::<Connection>().unwrap();
        let auth: String = match request.headers().get_one("Authorization") {
            Some(value) => value.to_owned(),
            None => return Outcome::Failure((Status::BadRequest, ()))
        };
        let auth_split = auth.split_whitespace().collect::<Vec<&str>>();
        if auth_split.len() != 2 && auth_split[0] != "Basic" {
            return Outcome::Failure((Status::Unauthorized, ()))
        }
        let decode: Vec<u8> = match base64::decode(auth_split[1]) {
            Ok(value) => value,
            Err(_) => return Outcome::Failure((Status::BadRequest, ()))
        };
        let basic_decode = match std::str::from_utf8(&decode) {
            Ok(value) => value.split(":").collect::<Vec<&str>>(),
            Err(_) => return Outcome::Failure((Status::BadRequest, ()))
        };
        if basic_decode.len() != 2 {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        Outcome::Success(BasicAuth{username: basic_decode[0].to_owned(), password: basic_decode[1].to_owned(),connection: conn.clone()})
    }
}
