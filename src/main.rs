mod commands;
use commands::{
	Command,
	// CommandOption,
	// CommandOptionChoice,
	CommandType,
};

mod webhook;
mod encryption;



#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;



#[rocket::main]
async fn main() {

	let command = match Command::new(
		"test_cmd".to_string(),
		CommandType::ChatInput,
		"test description".to_string(),
		None
	).await {
		Ok(c) => c,
		Err(e) => return println!("Failed to register command!\n{}", e),
	};



	if webhook::listen().launch().await.is_ok() { 
		// executes when the webhook is closed
		println!("Closed webhook");
	}

	if command.remove().await.is_ok() {
		println!("Successfully removed command");
	}

}
