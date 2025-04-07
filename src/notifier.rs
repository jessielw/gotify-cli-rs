// src/notifier.rs
use crate::MessagePayload;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use std::{thread, time::Duration};

/// Number of times to retry sending the message before giving up
const MAX_RETRIES: u8 = 5;
/// Delay between retries
const RETRY_DELAY_SECS: u64 = 1;

/// Sends a notification payload to the Gotify server, with retry logic.
pub fn send_notification(
    client: &Client,
    headers: &HeaderMap,
    url: &str,
    payload: &MessagePayload,
) -> Result<(), Box<dyn std::error::Error>> {
    for attempt in 1..=MAX_RETRIES {
        match client
            .post(url)
            .headers(headers.clone())
            .json(payload)
            .send()
        {
            Ok(response) if response.status().is_success() => {
                println!("Message sent successfully!");
                return Ok(());
            }
            Ok(response) => {
                eprintln!(
                    "Server responded with status {} (attempt {}/{})",
                    response.status(),
                    attempt,
                    MAX_RETRIES
                );
            }
            Err(err) => {
                eprintln!(
                    "Failed to send message (attempt {}/{}): {}",
                    attempt, MAX_RETRIES, err
                );
            }
        }

        if attempt < MAX_RETRIES {
            thread::sleep(Duration::from_secs(RETRY_DELAY_SECS));
            println!("Retrying...");
        }
    }

    eprintln!(
        "Failed to send message after {} attempts. Exiting.",
        MAX_RETRIES
    );
    Err("Failed to send notification after retries".into()) // Failure
}
