#![allow(dead_code)]

use discord_rich_presence::{activity, new_client, DiscordIpc};

pub enum Buttons {
    One { name: String, link: String },
    Two { name: String, link: String , name2: String, link2: String}
}

// Data struct
pub struct Data {
    id: String,
    status: String,
    details: String,
    large: bool,
    small: bool,
    want_buttons: bool,
    large_image: String,
    small_image: String,
    large_tool: String,
    small_tool: String,
    buttons: Buttons,
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
        match &data.buttons {
            Buttons::One {name, link} => {
                activity = activity.buttons(
                    vec![activity::Button::new(
                    &name,
                    &link
                )]);
            }
            Buttons::Two { name, link, name2, link2 } => {
                activity = activity.buttons(
                    vec![activity::Button::new(
                        &name,
                        &link
                    )]
                ).buttons(
                    vec![activity::Button::new(
                        &name2,
                        &link2
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

