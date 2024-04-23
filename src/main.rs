mod config;

use anyhow::Context as Ctx;
use anyhow::Result;
use config::Config;

use serenity::all::ActivityData;
use serenity::all::Ready;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        let config = data.get::<Config>().unwrap();

        println!("{}", config.presence.description);

        for (key, value) in config.responses.iter() {
            if msg.content.contains(key) {
                if let Err(why) = msg.channel_id.say(&ctx.http, value).await {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Bot Logged in as {}#{}",
            ready.user.name,
            ready.user.discriminator.unwrap()
        );

        let data = ctx.data.read().await;
        let config = data.get::<Config>().unwrap();

        ctx.set_presence(
            Some(ActivityData {
                name: config.presence.description.clone(),
                kind: config.presence.activity,
                state: None,
                url: None,
            }),
            config.presence.status,
        );
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().context("Failed to load config")?;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(config.token.clone(), intents)
        .event_handler(Handler)
        .await
        .context("Failed to initialize client")
        .unwrap();

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(config);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why}");
    }

    Ok(())
}
