use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use std::env;

// Based on https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html#example-1

pub struct ApiKey(String);

fn is_valid(key: &str) -> bool {
    key == env::var("SMLR_API_KEY").unwrap()
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("X-Api-Key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}
