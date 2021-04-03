use anyhow::{Result};
use crate::raw_client::RawGameSenseClient;
use serde_json;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct GameSenseClient {
    raw_client: Arc<Mutex<RawGameSenseClient>>,
    game: String,
    heartbeat_handle: Option<thread::JoinHandle<Result<()>>>
}

impl GameSenseClient {
    pub fn new(game: String) -> Result<GameSenseClient> {
        Ok(
            GameSenseClient {
                raw_client: Arc::new(Mutex::new(RawGameSenseClient::new()?)),
                game: game,
                heartbeat_handle: None
            }
        )
    }

    pub fn start_heartbeat(&mut self) {
        let raw_client = Arc::clone(&self.raw_client);

        let game = self.game.clone();

        self.heartbeat_handle = Some(
            thread::spawn(move || {
                loop {
                    raw_client.lock().unwrap().heartbeat(&game)?;
                    thread::sleep(Duration::from_secs(10))
                }
            })
        );
    }

    pub fn stop_heartbeat(&mut self) -> Result<()> {
        self.heartbeat_handle
            .take().expect("Trying to stop uninitialized heartbeat thread")
            .join().expect("Could not join heartbeat thread")
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
