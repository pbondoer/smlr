#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate dotenv;

use std::env;
use dotenv::dotenv;

pub mod utils;
pub mod auth;
mod routes;

fn main() {
    dotenv().expect("Failed to read .env file");

    { // Check presence of required env variables
        vec!["SMLR_DB_PATH", "SMLR_PREFIX", "SMLR_API_KEY"]
            .iter()
            .for_each(|var| {
                env::var(var).expect(&format!("Please provide {}", var));
            });
    }
    
    rocket::ignite()
        .register(catchers![routes::home::home, routes::error::error])
        .mount("/", routes![routes::home::home, routes::new::new, routes::redirect::redirect])
        .launch();
}