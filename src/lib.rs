// src/lib.rs

//! # Discord Notification Sender
//!
//! A simple library for sending notifications to Discord channels using a Discord bot.
//!
//! ## Example
//!
//! ```no_run
//! use discord_notify::DiscordBot;///!
//! #[tokio::main]
//! async fn main() {
//!     // Ensure you have a .env file with a valid DISCORD_TOKEN
//!     let channel_id = "<YOUR CHANNEL ID>";
//!     let identifier = "My Discord Bot";
//!     let bot = DiscordBot::new(identifier, channel_id);
//!     if let Err(e) = bot.send_notification("Hello from the Discord Notification Sender!").await {
//!         eprintln!("Failed to send notification: {}", e);
//!     }
//! }
//! ```


use reqwest::Client;
use std::env;
use serde_json::json;
use prettytable::{Table, row};

fn pretty_print_bot_details (bot: &DiscordBot) -> () {
    let mut table = Table::new();
    table.add_row(row!["BOT_ATTRIBUTES"]);
    table.add_row(row!["IDENTIFIER", bot.identifier]);
    table.add_row(row!["CHANNEL ID", bot.channel_id]);
    table.printstd();
}


#[derive(Clone)]
pub struct DiscordBot {
    identifier: String,
    token: String,
    channel_id: String,
    /// The base URL for the Discord API.
    ///
    /// This defaults to "https://discord.com/api/v9" but can be overridden (e.g., in tests).
    api_base_url: String,
    client: Client,
}
impl DiscordBot {
    /// Creates a new `DiscordBot` using the default Discord API base URL.
    ///
    /// The Discord bot token is read from the environment variable `DISCORD_TOKEN`.
    pub fn new(identifier: &str, channel_id: &str) -> Self {
        // Load .env file (if it exists) to set environment variables.
        dotenv::dotenv().ok();
        let token = env::var("DISCORD_TOKEN")
            .expect("DISCORD_TOKEN must be set in the environment or .env file");
        let bot = DiscordBot {
            identifier: identifier.to_string(),
            token,
            channel_id: channel_id.to_string(),
            api_base_url: "https://discord.com/api/v9".to_string(),
            client: Client::new(),
        };
        pretty_print_bot_details(&bot);
        bot
    }

    /// Creates a new `DiscordBot` with a custom API base URL.
    ///
    /// This is useful for testing.
    pub fn new_with_base_url(identifier: &str, channel_id: &str, base_url: &str) -> Self {
        dotenv::dotenv().ok();
        let token = env::var("DISCORD_TOKEN")
            .expect("DISCORD_TOKEN must be set in the environment or .env file");
        DiscordBot {
            identifier: identifier.to_string(),
            token,
            channel_id: channel_id.to_string(),
            api_base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    /// Sends a notification (message) to the Discord channel.
    ///
    /// # Arguments
    ///
    /// * `message` - The content of the message to be sent.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the message was sent successfully.
    /// * An error if the request failed.
    pub async fn send_notification(&self, message: &str) -> Result<(), reqwest::Error> {
        let url = format!("{}/channels/{}/messages", self.api_base_url, self.channel_id);
        let body = json!({ "content": message });
        self.client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.token))
            .json(&body)
            .send()
            .await?
            .error_for_status()?; // Return error if the HTTP status is not success.
        Ok(())
    }
}