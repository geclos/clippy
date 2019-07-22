#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_json;
extern crate elastic;

#[macro_use]
extern crate rocket;
extern crate envy;

use serde::Deserialize;
use serde_json::Value;
use elastic::client;
use elastic::http;
use elastic::*;

#[derive(Deserialize, Debug)]
struct DBConfig {
    db_host: String,
    db_password: String,
}

struct DbClient {
    db: client::Client<http::sender::SyncSender>
}

#[post("/clip")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().expect("Failed to read .env file");

    match envy::from_env::<DBConfig>() {
        Ok(config) => println!("{:?}", config),
        Err(_) => println!("Couldn't load the db config"),
    };

    let db = SyncClient::builder().build()?;

    rocket::ignite()
        .manage(DbClient { db: db })
        .mount("/", routes![index])
        .launch();

    return Ok(());
}
