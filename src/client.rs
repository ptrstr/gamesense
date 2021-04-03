use anyhow::{Context, Result};
use crate::raw_client::RawGameSenseClient;
use crate::timer::Timer;
use serde_json;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct GameSenseClient {
    raw_client: Arc<Mutex<RawGameSenseClient>>,
    game: String,
    heartbeat: Option<Timer>
}

impl GameSenseClient {
    pub fn new(game: &str, game_display_name: &str, developer: &str, deinitialize_timer_length_ms: Option<u16>) -> Result<GameSenseClient> {
        let client = GameSenseClient {
            raw_client: Arc::new(Mutex::new(RawGameSenseClient::new()?)),
            game: game.to_owned(),
            heartbeat: None
        };

        client.raw_client.lock().unwrap().register_game(&client.game, Some(game_display_name), Some(developer), deinitialize_timer_length_ms)?;

        Ok(client)
    }

    pub fn from_game_name(game: &str) -> Result<GameSenseClient> {
        Ok(
            GameSenseClient {
                raw_client: Arc::new(Mutex::new(RawGameSenseClient::new()?)),
                game: game.to_owned(),
                heartbeat: None
            }
        )
    }

    pub fn start_heartbeat(&mut self) {
        let raw_client = Arc::clone(&self.raw_client);

        let game = self.game.clone();

        self.heartbeat = Some(
            Timer::new(Duration::from_secs(10))
        );

        self.heartbeat.as_mut().unwrap().start(move || {
            raw_client.lock().unwrap().heartbeat(&game).ok();
        });
    }

    pub fn stop_heartbeat(&mut self) -> Result<()> {
        Ok(
            self.heartbeat.as_mut().context("Trying to stop uninitialized heartbeat thread")?.stop()?
        )
    }

    pub fn register_event(&self, event: &str) -> Result<String> {
        self.raw_client.lock().unwrap().register_event(&self.game, event, None, None, None, None)
    }

    pub fn register_event_full(&self, event: &str, min_value: Option<isize>, max_value: Option<isize>, icon_id: Option<u8>, value_optional: Option<bool>) -> Result<String> {
        self.raw_client.lock().unwrap().register_event(&self.game, event, min_value, max_value, icon_id, value_optional)
    }

    pub fn trigger_event(&self, event: &str, value: isize) -> Result<String> {
        self.raw_client.lock().unwrap().game_event(&self.game, event, value, None)
    }

    pub fn trigger_event_frame(&self, event: &str, value: isize, frame: serde_json::Value) -> Result<String> {
        self.raw_client.lock().unwrap().game_event(&self.game, event, value, Some(frame))
    }
}

impl Drop for GameSenseClient {
    fn drop(&mut self) {
        self.stop_heartbeat().ok();
    }
}
