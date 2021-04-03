extern crate reqwest;
extern crate anyhow;
extern crate serde_json;
use std::env;
use std::fs;
use anyhow::{Result, anyhow};
use serde_json::json;

pub struct GameSenseClient {
    client: reqwest::blocking::Client,
    address: String,
}

impl GameSenseClient {
    pub fn new() -> Result<GameSenseClient> {
        let path = match env::consts::OS {
            "macos" => Ok(String::from("/Library/Application Support/SteelSeries Engine 3/coreProps.json")),
            "windows" => Ok(env::var("PROGRAMDATA")? + "/SteelSeries/SteelSeries Engine 3/coreProps.json"),
            _ => Err(anyhow!("Platform must be either MacOS or Windows. Got {}", env::consts::OS))
        };

        let config: serde_json::Value = serde_json::from_str(&fs::read_to_string(path?)?)?;

        return Ok(GameSenseClient {
            client: reqwest::blocking::Client::new(),
            address: config["address"].as_str().expect("`address` not found").to_owned()
        });
    }

    fn send_data(&self, endpoint: &str, data: &serde_json::Value) -> Result<String> {
        Ok(
            self.client.post(format!("http://{}/{}", self.address, endpoint))
                .json(data)
                .send()?
                .text()?
        )
    }

    pub fn game_event(&self, game: &str, event: &str, value: isize) -> Result<String> {
        self.game_event_frame(game, event, value, None)
    }

    pub fn game_event_frame(&self, game: &str, event: &str, value: isize, frame: Option<serde_json::Value>) -> Result<String> {
        let mut data = json!({
            "game": game,
            "event": event,
            "value": value
        });

        if let Some(frame_value) = frame {
            data.as_object_mut().unwrap().insert(String::from("frame"), frame_value);
        }

        self.send_data("game_event", &data)
    }

    pub fn heartbeat(&self, game: &str) -> Result<String> {
        let data = json!({
            "game": game
        });

        self.send_data("game_heartbeat", &data)
    }

    pub fn register_game(&self, game: &str, game_display_name: &str, developer: &str) -> Result<String> {
        let data = json!({
            "game": game,
            "game_display_name": game_display_name,
            "developer": developer
        });

        self.send_data("game_metadata", &data)
    }

    pub fn register_event(&self, game: &str, event: &str, min_value: isize, max_value: isize, icon_id: u8, value_optional: bool) -> Result<String> {
        let data = json!({
            "game": game,
            "event": event,
            "min_value": min_value,
            "max_value": max_value,
            "icon_id": icon_id,
            "value_optional": value_optional
        });

        self.send_data("register_game_event", &data)
    }
}
