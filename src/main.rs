#[macro_use] extern crate prettytable;
mod lib;
use crate::lib::DiscordBot;

/// A simple Discord bot for sending notifications.



#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{server_url, Mock};
    use serde_json::json;
    use std::env;

    #[tokio::test]
    async fn test_send_notification_success() {
        // Set the Discord token for testing.
        env::set_var("DISCORD_TOKEN", "test_token");
        let channel_id = "1234";
        // Use the mockito server URL as the API base URL.
        let base_url = &server_url();

        // Create a mock for the expected Discord API endpoint.
        let _m = Mock::given("POST")
            .and(mockito::path(format!("/channels/{}/messages", channel_id)))
            .and(mockito::header("authorization", "Bot test_token"))
            .and(mockito::json_body(json!({
                "embeds": [{
                    "title": "TEST_BOT",
                    "description": "Hello, World!",
                    "color": 3447003
                }]
            })))
            .respond_with(mockito::response().status(200))
            .expect(1)
            .create();

        let bot = DiscordBot::new_with_base_url("TEST_BOT", channel_id, base_url);
        let result = bot.send_notification("Hello, World!").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_notification_failure() {
        env::set_var("DISCORD_TOKEN", "test_token");
        let channel_id = "1234567890";
        let base_url = &server_url();

        // Create a mock that returns a 400 Bad Request.
        let _m = Mock::given("POST")
            .and(mockito::path(format!("/channels/{}/messages", channel_id)))
            .and(mockito::header("authorization", "Bot test_token"))
            .and(mockito::json_body(json!({
                "embeds": [{
                    "title": "ROGUE_BOT",
                    "description": "Failure message",
                    "color": 3447003
                }]
            })))
            .respond_with(mockito::response().status(400))
            .expect(1)
            .create();

        let bot = DiscordBot::new_with_base_url("ROGUE_BOT", channel_id, base_url);
        let result = bot.send_notification("Failure message").await;
        assert!(result.is_err());    }


}


#[tokio::main]
async fn main() {
    let start_time = std::time::Instant::now();
    // real_test_send_notification().await;
    let elapsed_time = start_time.elapsed();
    println!("Elapsed wall time: {:?}", elapsed_time);
}