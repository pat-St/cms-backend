use std::io::Cursor;

use rocket::http::ContentType;
use rocket::http::Method;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

pub struct ResponseWithHeader<T>(pub Option<T>);

pub struct ImageResponse<T>(pub ContentType, pub Option<T>);

fn get_header(request: &Request) -> Response<'static> {
    let origin_url = match request.headers().get_one("Access-Control-Allow-Origin") {
        Some(v) => v.to_owned(),
        None => "*".to_owned(),
    };
    Response::build()
    .raw_header_adjoin("Access-Control-Allow-Origin", origin_url)
    .raw_header_adjoin("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE")
    .raw_header_adjoin("Access-Control-Allow-Headers", "Content-Type, authorization, apikey")
    .raw_header_adjoin("Access-Control-Allow-Credentials", "true")
    .finalize()
}

impl<'r, T> ResponseWithHeader<T> where T: serde::Serialize {
    fn response_option(req: &Request, payload: Option<T>) -> response::Result<'r> {
        match payload {
            Some(value) => {
                Response::build()
                    .header(ContentType::JSON)
                    .join(get_header(req))
                    .sized_body(Cursor::new(json!(value).0.to_string()))
                    .ok()
            }
            None => Response::build().join(get_header(req)).status(Status::UnprocessableEntity).ok()
        }
    }
    // TODO: return other type (Response)
    fn response_status(req: &Request, payload: Option<T>) -> response::Result<'r> {
        match payload {
            Some(_) =>  Response::build().join(get_header(req)).ok(),
            None => Response::build().join(get_header(req)).status(Status::UnprocessableEntity).ok()
        }
    }
}

impl<'r, T> Responder<'r> for ResponseWithHeader<T> where T: serde::Serialize {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match req.method() {
            Method::Post | Method::Put | Method::Delete | Method::Options => { Self::response_status(req, self.0) }
            Method::Get => { Self::response_option(req, self.0) }
            _ => self.respond_to(req)
        }
    }
}

impl<'r, T: Responder<'r>> Responder<'r> for ImageResponse<T> {
    #[inline(always)]
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self.1 {
            Some(value) => Response::build()
                .merge(value.respond_to(req)?)
                .header(self.0)
                .join(get_header(req))
                .ok(),
            None => Response::build()
                .join(get_header(req))
                .status(Status::UnprocessableEntity).ok()
        }
    }
}