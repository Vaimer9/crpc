use discord_rich_presence::{activity, new_client, DiscordIpc};

// Data struct
pub struct Data {
    id: String,
    status: String,
    details: String,
    large: bool,
    small: bool,
    large_image: String,
    small_image: String,
    large_tool: String,
    small_tool: String
}


fn start(data: Data) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = new_client(&data.id)?;

    client.connect()?;
    client.set_activity(activity::Activity::new()
        .state(&data.status)
        .details(&data.details)
        .assets(
            activity::Assets::new()
                .large_image(&data.large_image)
                .large_text(&data.large_tool)
                .small_image(&data.small_image)
                .small_text(&data.small_tool)
        )
        
    )?;

    loop {}
    Ok(())
}
