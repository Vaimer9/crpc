#![allow(dead_code)]

use crate::utils::read::Formatted;

use discord_rich_presence::{activity, new_client, DiscordIpc};

// Data struct
pub struct Data {
    id: String,
    status: String,
    details: String,
    large: bool,
    small: bool,
    want_buttons: bool,
    button_numbers: u8,
    large_image: String,
    small_image: String,
    large_tool: String,
    small_tool: String,
    buttons: Vec<String>,
}

impl Data {
    pub fn from_json(fr: Formatted) -> Self {
        Data {
            id: fr.id,
            status: fr.status,
            details: fr.details,
            large: fr.large,
            small: fr.small,
            want_buttons: fr.want_buttons,
            button_numbers: fr.button_numbers,
            large_image: fr.large_image,
            small_image: fr.small_image,
            large_tool: fr.large_tool,
            small_tool: fr.small_tool,
            buttons: fr.buttons
        }
    }
}


pub fn start(data: Data) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = new_client(&data.id)?;

    client.connect()?;

    let mut activity = activity::Activity::new();


    activity = activity.state(&data.status);
    activity = activity.details(&data.details);
    
    if data.large { activity = activity.assets(activity::Assets::new()
        .large_image(&data.large_image)
        .large_text(&data.large_tool)
    )};

    if data.small { activity = activity.assets(activity::Assets::new()
        .small_image(&data.large_image)
        .small_text(&data.large_tool)
    )};

    if data.want_buttons {
        match &data.button_numbers {
            1 | &_ => {
                activity = activity.buttons(
                    vec![activity::Button::new(
                    &data.buttons[0],
                    &data.buttons[1]
                )]);
            }
            2 => {
                activity = activity.buttons(
                    vec![activity::Button::new(
                        &data.buttons[0],
                        &data.buttons[1]
                    )]
                ).buttons(
                    vec![activity::Button::new(
                        &data.buttons[2],
                        &data.buttons[3]
                    )]
                );
            }
        }
    }

    client.set_activity(activity)?;

    loop {}

    #[allow(unreachable_code)]
    Ok(())
}
