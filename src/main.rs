#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use dotenv;
use envy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DBConfig {
    db_host: String,
    db_password: String,
}

#[post("/clip")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    match envy::from_env::<DBConfig>() {
        Ok(config) => println!("{:?}", config),
        Err(e) => println!("Couldn't read mailer config ({})", e),
    };

    rocket::ignite().mount("/", routes![index]).launch();
}
