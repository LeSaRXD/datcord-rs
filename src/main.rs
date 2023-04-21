use reqwest::{
	Client,
	header::{HeaderMap, HeaderValue, self}
};
use serde::Serialize;



#[derive(Serialize)]
struct Command {
	name: String,
	r#type: u8,
	description: String,
	options: Option<Vec<CommandOption>>,
}

#[derive(Serialize)]
struct CommandOption {
	name: String,
	r#type: u8,
	description: String,
	required: bool,
	choices: Option<Vec<CommandOptionChoice>>,
}

#[derive(Serialize)]
struct CommandOptionChoice {
	name: String,
	value: String,
}



#[tokio::main(flavor = "current_thread")]
async fn main() {

	let client_id = dotenv::var("CLIENT_ID").unwrap();
	let bot_token = dotenv::var("BOT_TOKEN").unwrap();



	let command = Command {
		name: "cmd".to_string(),
		r#type: 1,
		description: "test command".to_string(),
		options: Some(vec![
			CommandOption {
				name: "op".to_string(),
				description: "option description".to_string(),
				r#type: 3,
				required: true,
				choices: Some(vec![
					CommandOptionChoice {
						name: "Delete".to_string(),
						value: "del".to_string(),
					},
				]),
			}
		]),
	};

	let mut headers = HeaderMap::new();
	headers.insert(header::AUTHORIZATION, HeaderValue::from_str(format!("Bot {}", bot_token).as_str()).unwrap());

	let client = Client::new();
	match client
	.post(format!("https://discord.com/api/v10/applications/{}/commands", client_id))
	.headers(headers)
	.json(&command)
	.send()
	.await {
		Ok(r) => println!("{}", r.text().await.unwrap()),
		Err(_) => println!("error"),
	}

}
