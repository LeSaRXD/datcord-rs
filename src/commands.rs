use reqwest::{
	Client,
	Response,
	header
};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};



lazy_static! {

	static ref CLIENT: Client = Client::new();

	static ref CLIENT_ID: String = dotenv::var("CLIENT_ID").unwrap();
	static ref BOT_TOKEN: String = dotenv::var("BOT_TOKEN").unwrap();
	
}



#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CommandType {
	ChatInput = 1,
	User = 2,
	Message = 3,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
	pub id: String,
	name: String,
	#[serde(rename = "type")]
	typ: CommandType,
	description: String,
	options: Option<Vec<CommandOption>>,
}
impl Command {

	// creates a new command and registers it in the discord api
	pub async fn new(name: String, typ: CommandType, description: String, options: Option<Vec<CommandOption>>) -> Result<Self, reqwest::Error> {
		
		let command = Self { id: "0".to_string(), name, typ, description, options };

		match CLIENT
			.post(format!("https://discord.com/api/v10/applications/{}/commands", CLIENT_ID.to_string()))
			.header(header::AUTHORIZATION, format!("Bot {}", BOT_TOKEN.to_string()))
			.json(&command)
			.send()
			.await {
				Ok(r) => r.json::<Command>().await,
				Err(e) => Err(e),
			}

	}

	// removes current command from the discord api
	pub async fn remove(self) -> Result<Response, reqwest::Error> {

		CLIENT
			.delete(format!("https://discord.com/api/v10/applications/{}/commands/{}", dotenv::var("CLIENT_ID").unwrap(), self.id))
			.header(header::AUTHORIZATION, format!("Bot {}", dotenv::var("BOT_TOKEN").unwrap()))
			.send()
			.await

	}

	// gets all commands from the discord api
	pub async fn get_all() -> Result<Vec<Command>, reqwest::Error> {

		match CLIENT
			.get(format!("https://discord.com/api/v10/applications/{}/commands", CLIENT_ID.to_string()))
			.header(header::AUTHORIZATION, format!("Bot {}", BOT_TOKEN.to_string()))
			.send()
			.await {
				Ok(r) => r.json::<Vec<Command>>().await,
				Err(e) => Err(e),
			}

	}
	
}



#[derive(Serialize, Deserialize, Debug)]
pub struct CommandOption {
	name: String,
	#[serde(rename = "type")]
	typ: u8,
	description: String,
	required: bool,
	choices: Option<Vec<CommandOptionChoice>>,
}
impl CommandOption {
	pub	fn new(name: String, typ: u8, description: String, required: bool, choices: Option<Vec<CommandOptionChoice>>) -> Self {
		Self { name, typ, description, required, choices }
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
