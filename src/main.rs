use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

use clap::{Parser, builder::Str};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ClapCli {
    #[arg(short, long = "client-id")]
    client_id: String,

    #[arg(short, long = "state")]
    state: Option<String>,

    #[arg(short, long = "details")]
    details: Option<String>,

    #[arg(short, long = "large-image-key")]
    large_image_key: Option<String>,

    #[arg(short, long = "large-image-text")]
    large_image_text: Option<String>,

    #[arg(short, long = "small-image-key")]
    small_image_key: Option<String>,

    #[arg(short, long = "small-image-text")]
    small_image_text: Option<String>
}

fn main() {
    let mut client = match DiscordIpcClient::new("771124766517755954") {
        Ok(c) => c,
        Err(er) => {
            print!("{}", er.to_string());
            std::process::exit(2);
        }
    };

    match client.connect() {
        Err(er) => {
            print!("{}", er.to_string());
            std::process::exit(2);
        },
        _ => ()
    };
    
    match client.set_activity(activity::Activity::new()
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
    )) {
        Err(er) => {
            print!("{}", er.to_string());
            std::process::exit(2);
        },
        _ => ()
    };

    std::thread::park();

    match client.close() {
        Err(er) => {
            print!("{}", er.to_string());
            std::process::exit(2);
        },
        _ => ()
    };
}