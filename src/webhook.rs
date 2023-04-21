use rocket::{
	Rocket,
	Build,
	http::Status,
	response::{content::RawJson},
	Request,
	request::{self, FromRequest},
};
use crate::encryption;



#[derive(Debug)]
struct CommandHeaders {
	signature: String,
	timestamp: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for CommandHeaders {
	type Error = ();

	async fn from_request(reqest: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		request::Outcome::Success(Self {
			signature: match reqest.headers().get_one("X-Signature-Ed25519") {
				Some(h) => h.to_owned(),
				None => return request::Outcome::Failure((Status::BadRequest, ())),
			},
			timestamp: match reqest.headers().get_one("X-Signature-Timestamp") {
				Some(h) => h.to_owned(),
				None => return request::Outcome::Failure((Status::BadRequest, ())),
			},
		})
	}
}



pub fn listen() -> Rocket<Build> {
	rocket::build()
		.mount("/api/", routes![
			interaction,
		])
}



#[post("/", format = "application/json", data = "<body>")]
fn interaction(headers: CommandHeaders, body: String) -> Result<RawJson<String>, Status> {

	// verify using Ed25519 encryption
	if !encryption::verify(
		format!("{}{}", headers.timestamp, body), // timestamp + body
		headers.signature // signature
	) {
		return Err(Status::Unauthorized)
	}

	
	Ok(RawJson(r#"{ "type": 1 }"#.to_string()))

}
