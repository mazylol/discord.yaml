mod config;

use config::Config;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        let config = data.get::<Config>().unwrap();

        for (key, value) in config.responses.iter() {
            if msg.content.contains(key) {
                if let Err(why) = msg.channel_id.say(&ctx.http, value).await {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::load().unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(config.token.clone(), intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(config);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why}");
    }
}
