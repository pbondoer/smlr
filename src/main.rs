#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs::{File, write};
use std::io::Read;
use std::path::{Path, PathBuf};
use rocket::response::Redirect;
use rocket::response::status;
use rocket::request::Form;
use rocket::http::uri::Uri;
use rocket::http::Status;
use rand::Rng;

fn random() -> String {
    format!("{:04x}", rand::thread_rng().gen::<u32>())
}

#[catch(500)]
fn generic_error() -> &'static str {
    "Oops! Seems something went wrong...\n\nOpen an issue here: https://github.com/pbondoer/smlr/issues"
}

#[catch(404)]
#[get("/")]
fn home() -> &'static str {
    "smlr - the tiny link shortener written in Rust\n\nhttps://github.com/pbondoer/smlr"
}


#[derive(FromForm)]
struct TNew {
    url: String,
}

#[post("/new", data = "<form>")]
fn new(form: Form<TNew>) -> Result<String, status::Custom<&'static str>> {
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
                name = random();
                path = Path::new("data").join(&name);

                if !path.exists() {
                    break;
                }
            }

            match write(path, uri.to_string()) {
                Ok(_) => Ok(format!("https://smlr.it/{}", name)),
                _ => Err(status::Custom(Status::InternalServerError, generic_error()))
            }
        },
        _ => Err(status::Custom(Status::BadRequest, "Invalid URI"))
    }
}

#[get("/<file..>")]
fn file(file: PathBuf) -> Result<Redirect, Status> {
    match File::open(Path::new("data").join(file)) {
        Ok(mut f) => {
            let mut contents = String::new();
            match f.read_to_string(&mut contents) {
                Ok(_) => Ok(Redirect::to(contents)),
                _ => Err(Status::InternalServerError)
            }
        },
        _ => Err(Status::NotFound),
    }
}

fn main() {
    rocket::ignite()
        .register(catchers![home, generic_error])
        .mount("/", routes![home, new, file])
        .launch();
}