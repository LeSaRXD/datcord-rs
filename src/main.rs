#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

mod commands;
mod webhook;
mod encryption;



#[rocket::main]
async fn main() -> Result<(), rocket::Error>{

	webhook::listen().launch().await?;

	Ok(())

}
