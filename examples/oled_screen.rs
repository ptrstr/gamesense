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
                screen::ScreenFrameData::MultiLineFrameData(screen::MultiLineFrameData {
                    frame_modifiers_data: Some(screen::FrameModifiersData {
                        length_millis: Some(3000),
                        icon_id: Some(0),
                        repeats: None
                    }),
                    lines: vec![
                        screen::LineData {
                            type_options: screen::LineDataType::TextModifiersData(screen::TextModifiersData {
                                has_text: true,
                                prefix: None,
                                suffix: None,
                                bold: None,
                                wrap: None
                            }),
                            data_accessor_data: Some(screen::DataAccessorData {
                                arg: None,
                                context_frame_key: Some(String::from("artist"))
                            })
                        },
                        screen::LineData {
                            type_options: screen::LineDataType::TextModifiersData(screen::TextModifiersData {
                                has_text: true,
                                prefix: None,
                                suffix: None,
                                bold: None,
                                wrap: None
                            }),
                            data_accessor_data: Some(screen::DataAccessorData {
                                arg: None,
                                context_frame_key: Some(String::from("album"))
                            })
                        },
                        screen::LineData {
                            type_options: screen::LineDataType::ProgressBarData(screen::ProgressBarData {
                                has_progress_bar: true
                            }),
                            data_accessor_data: Some(screen::DataAccessorData {
                                arg: None,
                                context_frame_key: None // Some(String::from("song"))
                            })
                        },
                    ]
                })
            )
        ))
    );

    client.bind_event("EVENT", None, None, None, None, vec![handler])?;
    client.start_heartbeat();
    for i in 0..100 {
        client.trigger_event_frame("EVENT", i, json!({
            "artist": "Three Days Grace",
            "album": "One-X",
            "song": "Gone Forever"
        }))?;
    }
    client.stop_heartbeat()?;
    Ok(())
}
