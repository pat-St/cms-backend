use rocket::{Request, Response};
use rocket::http::{Status};
use rocket::response;

#[catch(503)]
pub fn service_not_available(_req: &Request) -> response::Result<'static>{
	Response::build()
		.raw_header("Access-Control-Allow-Origin", "*")
		.status(Status::InternalServerError).ok()
}

#[catch(422)]
pub fn unprocess_entity(_req: &Request) ->  response::Result<'static>{
	Response::build()
		.raw_header("Access-Control-Allow-Origin", "*")
		.status(Status::UnprocessableEntity).ok()
}

#[catch(404)]
pub fn not_found_entity(_req: &Request) ->  response::Result<'static>{
	Response::build()
		.raw_header("Access-Control-Allow-Origin", "*")
		.status(Status::NotFound).ok()
}
#[catch(400)]
pub fn bad_request(_req: &Request) ->  response::Result<'static>{
	Response::build()
		.raw_header("Access-Control-Allow-Origin", "*")
		.status(Status::BadRequest).ok()
}