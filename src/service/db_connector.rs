use std::ops::Deref;
use mysql;
use mysql::{Pool, PooledConn, QueryResult};
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::{thread, time};
use std::env;
use rocket::config::ConfigError;


#[derive(Debug, Clone)]
pub struct Connection {
    conn: Pool
}
const ENV_VAR: &'static str = "DATABASE_URL";

impl Connection {
    pub fn connect() -> Result<Pool,Status> {
        let vars = match Self::get_var() {
            Ok(val) => val,
            Err(_e) => return Err(Status::InternalServerError)
        };
        match mysql::Pool::new(vars) {
            Ok(pool) => Ok(pool),
            Err(e) => {
                println!("{}", e.to_string());
                Err(Status::BadGateway)
            }
        }
    }

    pub fn main_connect() -> Option<Pool> {
        match Self::connect() {
            Ok(val) => Some(val),
            Err(_e) => None
        }
    }

    pub fn get_connection(&self) -> Option<PooledConn> {
        match self.conn.get_conn() {
            Ok(value) => Some(value),
            Err(e) => {
                println!("{}", e.to_string());
                None
            }
        }
    }

    pub fn row_to_vec<T>(result: QueryResult) -> Vec<T> where T: mysql::prelude::FromValue {
        result
            .map(|value| mysql::from_row_opt(value.unwrap()).ok())
            .filter(|value| value.is_some())
            .map(|value| value.unwrap())
            .collect()
    }

    fn get_var() -> Result<String,ConfigError> {
        match env::var(ENV_VAR) {
            Ok(val) => Ok(val),
            Err(e) => {
                println!("Error: {}", e.to_string());
                Err(ConfigError::Missing(e.to_string()))
            }
        }
    }

    fn reconnect(counter: u8) -> Result<Pool,Status> {
        if counter == 0 {
            return Err(Status::ServiceUnavailable)
        }
        match Self::connect() {
            Ok(pool) => Ok(pool),
            Err(e) => {
                println!("reconnect times: {}", counter);
                match e {
                    Status::InternalServerError => Err(Status::InternalServerError),
                    Status::BadGateway | _=> {
                        thread::sleep(time::Duration::from_secs((counter*2).into()));
                        Self::reconnect(counter-1)
                    },
                }

            }

        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        match request.guard::<State<Option<Pool>>>() {
            Outcome::Success(value) if value.is_some() => Outcome::Success(Connection { conn: value.clone().unwrap() }),
            Outcome::Success(_) => {
                match Self::reconnect(3){
                    Ok(conn) => Outcome::Success(Connection { conn: conn.clone() }),
                    Err(status) => Outcome::Failure((status, ())),
                }

            },
            Outcome::Failure(_) => Outcome::Failure((Status::InternalServerError, ())),
            Outcome::Forward(e) => Outcome::Forward(e)
        }
    }
}

impl Deref for Connection {
    type Target = Pool;
    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

