extern crate gamesense;
extern crate anyhow;
extern crate serde_json;
use anyhow::{Result};
use gamesense::client::GameSenseClient;
use gamesense::handler::screen;
use serde_json::json;

fn main() -> Result<()> {
    let mut client = GameSenseClient::new("SCREEN_IMAGE", "Example OLED Image Event", "ptrstr", None)?;

    let width = 128;
    let height = 48;

    let handler = screen::ScreenHandler::new(&format!("screened-{}x{}", width, height), "one",
        screen::ScreenDataDefinition::StaticScreenDataDefinition(screen::StaticScreenDataDefinition(
            vec!(
                screen::ScreenFrameData::ImageFrameData(screen::ImageFrameData {
                    has_text: false,
                    frame_modifiers_data: Some(screen::FrameModifiersData {
                        length_millis: Some(2000),
                        icon_id: None,
                        repeats: None
                    }),
                    image_data: vec![255; width * height / 8],
                }),
            )
        ))
    );

    client.bind_event("EVENT", None, None, None, None, vec![handler])?;
    client.start_heartbeat();

    client.trigger_event_frame("EVENT", 0, json!({}))?;

    client.stop_heartbeat()?;
    Ok(())
}
