use std::fs::write;
use std::path::{Path};
use std::env;
use rocket::http::uri::Uri;
use rocket::http::Status;
use rocket::response::status;
use rocket::request::Form;

use crate::utils;
use crate::auth::ApiKey;
use super::error::error;

#[derive(FromForm)]
pub struct TNew {
    url: String,
}

// XXX: this return type is a bit ugly but I couldn't find a better way?
#[post("/new", data = "<form>")]
pub fn new(form: Form<TNew>, _key: ApiKey) -> Result<String, status::Custom<&'static str>> {
    match Uri::parse(&form.url) {
        Ok(uri) => {
            // make sure this is an absolute URI
            if uri.absolute().is_none() {
                return Err(status::Custom(Status::BadRequest, "Please use absolute URIs only"));
            }

            // find a suitable path
            let mut path;
            let mut name;
            loop {
                name = utils::random();
                path = Path::new(&env::var("SMLR_DB_PATH").unwrap()).join(&name);

                if !path.exists() {
                    break;
                }
            }

            match write(path, uri.to_string()) {
                Ok(_) => Ok(format!("{}{}", env::var("SMLR_PREFIX").unwrap(), name)),
                _ => Err(status::Custom(Status::InternalServerError, error()))
            }
        },
        _ => Err(status::Custom(Status::BadRequest, "Invalid URI"))
    }
}
