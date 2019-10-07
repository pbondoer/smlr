#[catch(500)]
pub fn error() -> &'static str {
    "Oops! Seems something went wrong...\n\nOpen an issue here: https://github.com/pbondoer/smlr/issues"
}
