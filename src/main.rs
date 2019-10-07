#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate dotenv;

/*
use std::fs::{File, write};
use std::io::Read;
use std::path::{Path, PathBuf};
use rocket::response::Redirect;
use rocket::response::status;
use rocket::request::Form;
use rocket::http::uri::Uri;
use rocket::http::Status;
*/

use dotenv::dotenv;
use std::env;

// modules
pub mod utils;
pub mod auth;
mod routes;

// entry point

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