use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954")?;
    client.connect()?;
    
    client.set_activity(activity::Activity::new()
    .state("A test")
    .details("A placeholder")
    // .buttons(vec![activity::Button::new(
    //     "A button",
    //     "https://github.com",
    // )])
    .assets(
        activity::Assets::new()
            .large_image("https://logseq.com/logo-with-border.5bf84f43.png")
            .large_text("lol")
            .small_image("https://logseq.com/logo-with-border.5bf84f43.png")
            .small_text("lol")
    ))?;

    std::thread::park();

    client.close()?;
    Ok(())
}