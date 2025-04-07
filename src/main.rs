use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Serialize;
use std::collections::HashMap;
mod notifier;
use notifier::send_notification;
mod utils;

/// CLI tool to send notifications via Gotify
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Gotify server URL
    #[arg(short, long)]
    server: String,

    /// Gotify app token
    #[arg(short, long)]
    app_key: String,

    /// Title of the message
    #[arg(short, long)]
    title: String,

    /// Message to send (supports markdown)
    #[arg(short, long)]
    message: String,

    /// Priority level
    #[arg(short, long, default_value_t = 5)]
    priority: u8,
}

#[derive(Serialize)]
struct MessagePayload {
    title: String,
    message: String,
    priority: u8,
    extras: HashMap<String, HashMap<String, String>>,
}

fn init_client() -> (Client, HeaderMap) {
    // build client
    let client: Client = Client::new();

    // build headers
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    (client, headers)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (client, headers) = init_client();

    let message = utils::fix_newlines(args.message);

    let payload = MessagePayload {
        title: args.title,
        message,
        priority: args.priority,
        extras: utils::get_extras(),
    };

    send_notification(
        &client,
        &headers,
        &utils::clean_url(&args.server, &args.app_key),
        &payload,
    )?;

    Ok(()) // Success, exit code 0
}
