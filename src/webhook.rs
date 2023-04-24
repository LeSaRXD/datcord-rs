use rocket::{
	Rocket,
	http::Status,
	response::{content::RawJson},
	Request,
	request::{self, FromRequest}, Ignite,
};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::encryption;



#[derive(Debug)]
struct VerificationHeaders {
	signature: String,
	timestamp: String,
}
#[async_trait]
impl<'r> FromRequest<'r> for VerificationHeaders {
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



#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
enum InteractionType {
	Ping = 1,
	ApplicationCommand = 2,
	MessageComponent = 3,
	ApplicationCommandAutocomplete = 4,
	ModalSubmit = 5,
}

#[derive(Serialize, Deserialize, Debug)]
struct Interaction {
	id: String,
	application_id: String,
	#[serde(rename = "type")]
	typ: InteractionType,
	token: String,
	version: u8,
}



// rocket
pub async fn listen() -> Result<Rocket<Ignite>, rocket::Error> {
	rocket::build()
		.mount("/api/", routes![
			interaction,
		])
		.launch()
		.await
}



// receive interaction
#[post("/", format = "application/json", data = "<body>")]
fn interaction(headers: VerificationHeaders, body: String) -> Result<RawJson<String>, Status> {

	// verify using Ed25519 encryption
	if !encryption::verify(
		format!("{}{}", headers.timestamp, body), // timestamp + body
		headers.signature // signature
	) {
		return Err(Status::Unauthorized);
	}

	println!("{}", body);

	let interaction: Interaction = match serde_json::from_str(body.as_str()) {
		Ok(i) => i,
		Err(e) => {
			println!("ERROR!!!\n{}", e);
			return Err(Status::BadRequest)
		},
	};

	println!("{:?}", interaction);

	match interaction.typ {
		InteractionType::Ping => Ok(RawJson(r#"{ "type": 1 }"#.to_string())),
		_ => Ok(RawJson(r#"{ "type": 1 }"#.to_string())),
	}

	// Ok(RawJson(r#"{ "type": 1 }"#.to_string()))

}
