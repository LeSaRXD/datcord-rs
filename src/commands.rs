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



#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CommandType {
	None,
	ChatInput = 1,
	User = 2,
	Message = 3,
}



#[async_trait]
pub trait Command {

	async fn register(&self) -> Result<Response, reqwest::Error>;

	// removes current command from the discord api
	async fn remove(self) -> Result<Response, reqwest::Error>;

}

#[derive(Serialize, Deserialize)]
pub struct GlobalCommand {
	id: String,
	name: String,
	#[serde(rename = "type")]
	typ: CommandType,
	description: String,
	options: Option<Vec<CommandOption>>,
}
impl GlobalCommand {
	// creates a new command and registers it in the discord api
	pub async fn new(name: String, typ: CommandType, description: String, options: Option<Vec<CommandOption>>) -> Result<Self, reqwest::Error> {
		
		let command = Self { id: "0".to_string(), name, typ, description, options };
		match command.register().await {
			Ok(r) => r.json::<Self>().await,
			Err(e) => Err(e),
		}

	}

	pub async fn get_all() -> Result<Vec<Self>, reqwest::Error> {

		let res = CLIENT
			.get(format!("https://discord.com/api/v10/applications/{}/commands", *CLIENT_ID))
			.header(header::AUTHORIZATION, format!("Bot {}", *BOT_TOKEN))
			.send()
			.await?;

		res.json::<Vec<Self>>().await

	}

}
#[async_trait]
impl Command for GlobalCommand {

	async fn register(&self) -> Result<Response, reqwest::Error> {

		CLIENT
			.post(format!("https://discord.com/api/v10/applications/{}/commands", *CLIENT_ID))
			.header(header::AUTHORIZATION, format!("Bot {}", *BOT_TOKEN))
			.json(&self)
			.send()
			.await 

	}
	async fn remove(self) -> Result<Response, reqwest::Error> {

		CLIENT
			.delete(format!("https://discord.com/api/v10/applications/{}/commands/{}", dotenv::var("CLIENT_ID").unwrap(), self.id))
			.header(header::AUTHORIZATION, format!("Bot {}", dotenv::var("BOT_TOKEN").unwrap()))
			.send()
			.await

	}

}



fn zero_string() -> String {
	"".to_string()
}
#[derive(Serialize, Deserialize)]
pub struct GuildCommand {
	id: String,
	#[serde(default = "zero_string")]
	guild_id: String,
	name: String,
	#[serde(rename = "type")]
	typ: CommandType,
	description: String,
	options: Option<Vec<CommandOption>>,
}
impl GuildCommand {

	// creates a new command and registers it in the discord api
	pub async fn new(name: String, guild_id: String, typ: CommandType, description: String, options: Option<Vec<CommandOption>>) -> Result<Self, reqwest::Error> {
		
		let command = Self { id: "0".to_string(), guild_id, name, typ, description, options };
		match command.register().await {
			Ok(r) => match r.json::<Self>().await {
				Ok(mut c) => {
					c.guild_id = command.guild_id;
					Ok(c)
				},
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}

	}
	
	pub async fn get_all(guild_id: &String) -> Result<Vec<Self>, reqwest::Error> {

		let res = CLIENT
			.get(format!("https://discord.com/api/v10/applications/{}/guilds/{}/commands", *CLIENT_ID, guild_id))
			.header(header::AUTHORIZATION, format!("Bot {}", *BOT_TOKEN))
			.send()
			.await?;

		res.json::<Vec<Self>>().await

	}

}
#[async_trait]
impl Command for GuildCommand {

	async fn register(&self) -> Result<Response, reqwest::Error> {

		CLIENT
			.post(format!("https://discord.com/api/v10/applications/{}/guilds/{}/commands", *CLIENT_ID, self.guild_id))
			.header(header::AUTHORIZATION, format!("Bot {}", *BOT_TOKEN))
			.json(&self)
			.send()
			.await

	}
	async fn remove(self) -> Result<Response, reqwest::Error> {

		CLIENT
			.delete(format!("https://discord.com/api/v10/applications/{}/guilds/{}/commands/{}", dotenv::var("CLIENT_ID").unwrap(), self.guild_id, self.id))
			.header(header::AUTHORIZATION, format!("Bot {}", dotenv::var("BOT_TOKEN").unwrap()))
			.send()
			.await

	}

}



#[derive(Serialize, Deserialize)]
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



#[derive(Serialize, Deserialize)]
pub struct CommandOptionChoice {
	name: String,
	value: String,
}
impl CommandOptionChoice {
	pub fn new(name: String, value: String) -> Self {
		Self { name, value }
	}
}
