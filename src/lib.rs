//! SteelSeries GameSense™ client written in Rust
//!
//!
//! To use this crate, simply add it as a dependency:
//! ```toml
//! [dependency]
//! gamesense = "0.1.0"
//! ```
//!
//!
//! To get started, simply instantiate your client using the [`gamesense::client::GameSenseClient`][client::GameSenseClient] struct.
//! ```
//! let mut client = GameSenseClient::new("GAME_ID", "Game Display Name", "Author", None)?;
//! ```
//!
//! In this example, a client is created to the local API and it automatically creates a game with the provided values. Each value
//! deemed optional by the official documentation can receive `None` as a value to default it to the server's default.
//! If you wish to have a raw client for the API, you can! Simply use the [`gamesense::raw_client::RawGameSenseClient`][raw_client::RawGameSenseClient]
//!
//! For more (in-depth) examples refer to the [examples](https://github.com/ptrstr/gamesense/examples)
//!
//! For information regarding the API see the [original API documentation](https://github.com/SteelSeries/gamesense-sdk)

extern crate anyhow;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;


mod timer;
pub mod handler;
pub mod raw_client;
pub mod client;
