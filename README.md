A simple discord bot struct in Rust that just sends notifications on a channel ID.

 ```rust
 use discord_notify::DiscordBot;
 #[tokio::main]
 async fn main() {
     // Ensure you have a .env file with a valid DISCORD_TOKEN
     let channel_id = "<YOUR CHANNEL ID>";
     let identifier = "My Discord Bot";
     let bot = DiscordBot::new(identifier, channel_id);
     if let Err(e) = bot.send_notification("Hello from the Discord Notification Sender!").await {
         eprintln!("Failed to send notification: {}", e);
     }
 }
 ```

![alt text](meme.png)