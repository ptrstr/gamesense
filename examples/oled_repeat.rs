extern crate gamesense;
extern crate anyhow;
extern crate serde_json;
use anyhow::{Result};
use gamesense::client::GameSenseClient;
use gamesense::handler::screen;
use serde_json::json;

fn main() -> Result<()> {
    let mut client = GameSenseClient::new("EVENT_SCREEN", "Example OLED Event", "ptrstr", None)?;

    let handler = screen::ScreenHandler::new("screened", "one",
        screen::ScreenDataDefinition::StaticScreenDataDefinition(screen::StaticScreenDataDefinition(
            vec!(
                screen::ScreenFrameData::SingleLineFrameData(screen::SingleLineFrameData {
                    frame_modifiers_data: Some(screen::FrameModifiersData {
                        length_millis: Some(1000),
                        icon_id: Some(screen::Icon::Kill),
                        repeats: None
                    }),
                    line: screen::LineData {
                        type_options: screen::LineDataType::TextModifiersData(screen::TextModifiersData {
                            has_text: true,
                            prefix: Some(String::from("")),
                            suffix: Some(String::from(" kills")),
                            bold: None,
                            wrap: None
                        }),
                        data_accessor_data: Some(screen::DataAccessorData {
                            arg: None,
                            context_frame_key: Some(String::from("kills"))
                        })
                    }
                }),
                screen::ScreenFrameData::SingleLineFrameData(screen::SingleLineFrameData {
                    frame_modifiers_data: Some(screen::FrameModifiersData {
                        length_millis: Some(1000),
                        icon_id: Some(screen::Icon::Headshot),
                        repeats: Some(screen::Repeat::Bool(true))
                    }),
                    line: screen::LineData {
                        type_options: screen::LineDataType::TextModifiersData(screen::TextModifiersData {
                            has_text: true,
                            prefix: Some(String::from("")),
                            suffix: Some(String::from(" headshots")),
                            bold: None,
                            wrap: None
                        }),
                        data_accessor_data: Some(screen::DataAccessorData {
                            arg: None,
                            context_frame_key: Some(String::from("headshots"))
                        })
                    }
                })
            )
        ))
    );

    client.bind_event("EVENT", None, None, None, None, vec![handler])?;
    client.start_heartbeat();

    client.trigger_event_frame("EVENT", 0, json!({
        "kills": 23,
        "headshots": 7,
    }))?;

    std::thread::sleep(std::time::Duration::from_secs(7));

    client.trigger_event_frame("EVENT", 0, json!({
        "kills": 24,
        "headshots": 8,
    }))?;

    client.stop_heartbeat()?;
    Ok(())
}
