# smlr
> A tiny URL shortener written in Rust

## Goals
- Provide a simple, low-latency API to shorten links (using `rocket` crate)
- Minimal configuration set-up (using `dotenv` crate)

## Non-goals
- All-in-one solution for link shortening
  - This includes: link expiry, passwords, etc.
- Deduplication

I may write a smarter more feature-complete version, but this is
all I need as of right now 😄

## Running
You can run this project with `cargo run`.

For production use, make sure to create a `Rocket.toml` and `.env`
configuration, and run using `cargo run --release`.

## Building
You can build this project with `cargo build`.

## To-do
- [ ] API key to prevent unauthorized use
- [ ] Write some tests
- [ ] Refactor into nicer modules

## License
![AGPL logo](https://www.gnu.org/graphics/agplv3-155x51.png "GNU Affero General Public License")

`smlr` is licensed under the [**GNU Affero General Public License
3.0**](LICENSE).
