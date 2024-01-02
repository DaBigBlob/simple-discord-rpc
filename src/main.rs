use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ClapCli {
    #[arg(short, long = "client-id")]
    client_id: Option<String>,

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
    let args = ClapCli::parse();

    let mut client = match DiscordIpcClient::new(
        &args.client_id.unwrap_or("1191646640000487476".to_string())
    ) {
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
    .state(&args.state.unwrap_or("default state".to_string()))
    .details(&args.details.unwrap_or("default details".to_string()))
    // .buttons(vec![activity::Button::new(
    //     "A button",
    //     "https://github.com",
    // )])
    .assets(
        activity::Assets::new()
            .large_image(&args.large_image_key.unwrap_or("https://www.rust-lang.org/static/images/rust-logo-blk.png".to_string()))
            .large_text(&args.large_image_text.unwrap_or("made with rust".to_string()))
            .small_image(&args.small_image_key.unwrap_or("https://www.rust-lang.org/static/images/rust-logo-blk.png".to_string()))
            .small_text(&args.small_image_text.unwrap_or("made with rust".to_string()))
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