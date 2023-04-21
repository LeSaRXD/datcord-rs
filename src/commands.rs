use reqwest::{
	Client,
	Response,
	header
};
use serde::{Serialize, Deserialize};



lazy_static! {
	static ref CLIENT: Client = Client::new();
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
	pub id: Option<String>,
	name: String,
	r#type: u8,
	description: String,
	options: Option<Vec<CommandOption>>,
}
impl Command {
	pub fn new(name: String, r#type: u8, description: String, options: Option<Vec<CommandOption>>) -> Self {
		Self { id: None, name, r#type, description, options }
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandOption {
	name: String,
	r#type: u8,
	description: String,
	required: bool,
	choices: Option<Vec<CommandOptionChoice>>,
}
impl CommandOption {
	pub	fn new(name: String, r#type: u8, description: String, required: bool, choices: Option<Vec<CommandOptionChoice>>) -> Self {
		Self { name, r#type, description, required, choices }
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandOptionChoice {
	name: String,
	value: String,
}
impl CommandOptionChoice {
	pub fn new(name: String, value: String) -> Self {
		Self { name, value }
	}
}



#[derive(Debug)]
pub enum GetError {
	ReqwestError(reqwest::Error),
	JsonError(serde_json::Error),
}
pub async fn get_all(client_id: &String, bot_token: &String) -> Result<Vec<Command>, GetError> {

	let json = match CLIENT
		.get(format!("https://discord.com/api/v10/applications/{}/commands", client_id))
		.header(header::AUTHORIZATION, format!("Bot {}", bot_token))
		.send()
		.await {
			Ok(r) => match r.text().await {
				Ok(t) => t,
				Err(e) => return Err(GetError::ReqwestError(e)),
			},
			Err(e) => return Err(GetError::ReqwestError(e)),
		};

	match serde_json::from_str::<Vec<Command>>(&json) {
		Ok(v) => Ok(v),
		Err(e) => Err(GetError::JsonError(e)),
	}

}

pub async fn remove(command_id: &String, client_id: &String, bot_token: &String) -> Result<Response, reqwest::Error> {

	CLIENT
		.delete(format!("https://discord.com/api/v10/applications/{}/commands/{}", client_id, command_id))
		.header(header::AUTHORIZATION, format!("Bot {}", bot_token))
		.send()
		.await

}

pub async fn register(command: &Command, client_id: &String, bot_token: &String) -> Result<Response, reqwest::Error> {

	CLIENT
		.post(format!("https://discord.com/api/v10/applications/{}/commands", client_id))
		.header(header::AUTHORIZATION, format!("Bot {}", bot_token).as_str())
		.json(&command)
		.send()
		.await

}
