use std::ops::Deref;
use mysql;
use mysql::{Pool, PooledConn, QueryResult};
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::{thread, time};
use std::env;

#[derive(Debug, Clone)]
pub struct Connection {
    conn: Pool
}

impl Connection {
    pub fn connect() -> Option<Pool> {
        match mysql::Pool::new(Self::get_var()) {
            Ok(pool) => Some(pool),
            Err(e) => {
                println!("{}", e.to_string());
                None
            }
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

    fn get_var() -> String {
        return match env::var("DATABASE_URL") {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e.to_string());
                "".to_string()
            }
        }
    }

    fn reconnect(counter: u8) -> Option<Pool> {
        if counter == 0 {
            return None
        }
        match Self::connect() {
            Some(pool) => Some(pool),
            None => {
                println!("reconnect times: {}", counter);
                thread::sleep(time::Duration::from_secs((counter*2).into()));
                return Self::reconnect(counter-1)
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
                    Some(conn) => Outcome::Success(Connection { conn: conn.clone() }),
                    None => Outcome::Failure((Status::ServiceUnavailable, ())),
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

