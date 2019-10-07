use rocket::http::Status;
use rocket::response::Redirect;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[get("/<file..>")]
pub fn redirect(file: PathBuf) -> Result<Redirect, Status> {
    match File::open(Path::new(&env::var("SMLR_DB_PATH").unwrap()).join(file)) {
        Ok(mut f) => {
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(Redirect::to(s)),
                _ => Err(Status::InternalServerError),
            }
        }
        _ => Err(Status::NotFound),
    }
}
