extern crate gamesense;
extern crate anyhow;
use anyhow::{Result};
use gamesense::client::GameSenseClient;

fn main() -> Result<()> {
    let mut client = GameSenseClient::new("EVENT_LOOP", "Example Event Loop", "ptrstr", None)?;
    client.register_event("EVENT")?;
    client.start_heartbeat();
    for i in 0..60 {
        client.trigger_event("EVENT", i)?;
    }
    client.stop_heartbeat()?;
    Ok(())
}
