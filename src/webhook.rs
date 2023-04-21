use rocket::{
	Rocket,
	Build,
	http::Status,
	response::{content::RawJson},
	serde::Deserialize,
	Request,
	request::FromRequest,
};



#[derive(Deserialize)]
struct CommandData {
	r#type: u8,
}

#[derive(Debug)]
struct CommandHeaders {
	signature: Option<String>,
	timestamp: Option<String>,
}
impl<'a, 'r> FromRequest<'a, 'r> for CommandHeaders {

	fn from_request(request: &'a Request<'r>) -> Self {
		Self {
			signature: request.headers().get_one("X-Signature-Ed25519").map(|s| s.to_owned()),
			timestamp: request.headers().get_one("X-Signature-Timestamp").map(|s| s.to_owned()),
		}
	}

}



pub fn listen() -> Rocket<Build> {
	rocket::build()
		.mount("/api/", routes![
			index,
			command,
		])
}

#[get("/")]
fn index() -> Status {
	Status::Ok
}

#[post("/", format = "application/json", data = "<command_data>")]
fn command(command_data: String, headers: CommandHeaders) -> RawJson<String> {
	// println!("{}\n{:?}\n{:?}", command_data.r#type, command_data.signature, command_data.timestamp);
	println!("{:?}\n{}", headers, command_data);

	// match command_data.r#type {
	// 	1 => RawJson(r#"{ "type": 1 }"#.to_string()),
	// 	_ => RawJson(r#"{ "type": 0 }"#.to_string()),
	// }
	RawJson(r#"{ "type": 1 }"#.to_string())

}
