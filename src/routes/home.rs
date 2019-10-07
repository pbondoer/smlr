#[catch(404)]
#[get("/")]
pub fn home() -> &'static str {
    "smlr - a tiny link shortener written in Rust\n\nhttps://github.com/pbondoer/smlr"
}
