#[cfg(test)]
pub mod test {
	use rocket::local::Client;
	use rocket::http::Status;
	#[test]
	fn default_rest() {
		let client = Client::new(backend::rocket()).expect("I'm up!");
		let mut response = client.get("/").dispatch();
		assert_eq!(response.status(), Status::Ok);
		assert_eq!(response.body_string(), Some("I'm up!".into()));
	}
}