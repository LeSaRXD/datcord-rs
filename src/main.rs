#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

mod commands;
// use commands::{Command, CommandOption, CommandOptionChoice};

mod webhook;



#[launch]
fn rocket() -> _ {

	// let client_id = dotenv::var("CLIENT_ID").unwrap();
	// let bot_token = dotenv::var("BOT_TOKEN").unwrap();



	webhook::listen()

}
