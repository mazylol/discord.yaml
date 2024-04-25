mod config;

use anyhow::Context as Ctx;
use anyhow::Result;
use config::Config;

use tracing as log;

use serenity::all::{
    ActivityData, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
    GuildId, Interaction, Ready,
};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        let config = data.get::<Config>().unwrap();

        if let Some(responses) = &config.responses {
            for (key, value) in responses.iter() {
                if msg.content.contains(key) {
                    if let Err(why) = msg.reply_ping(&ctx.http, value).await {
                        log::error!("Error sending message: {why:?}");
                    }

                    log::info!("Responded to message containing: {key}");
                }
            }
        }

        if let Some(commands) = &config.commands {
            if let Some(text) = &commands.text {
                if msg.content.starts_with(text.prefix) {
                    for command in &text.commands {
                        if msg
                            .content
                            .starts_with(&format!("{}{}", text.prefix, command.name))
                        {
                            if let Err(why) = msg.reply_ping(&ctx.http, &command.response).await {
                                log::error!("Error sending message: {why:?}");
                            }
                        }
                    }

                    log::info!("Responded to command {}", msg.content);
                }
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let data = ctx.data.read().await;
        let config = data.get::<Config>().unwrap();

        if let Interaction::Command(command) = interaction {
            if let Some(commands) = &config.commands {
                if let Some(slash) = &commands.slash {
                    for slash_command in &slash.commands {
                        match command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .content(&slash_command.response),
                                ),
                            )
                            .await
                        {
                            Ok(_) => {
                                log::info!("Responded to command: {:?}", slash_command.name)
                            }
                            Err(why) => log::error!("Error sending message: {:?}", why),
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!(
            "Bot Logged in as {}#{}",
            ready.user.name,
            ready.user.discriminator.unwrap()
        );

        let data = ctx.data.read().await;
        let config = data.get::<Config>().unwrap();

        if let Some(presence) = &config.presence {
            ctx.set_presence(
                Some(ActivityData {
                    name: presence.description.clone(),
                    kind: presence.activity,
                    state: None,
                    url: None,
                }),
                presence.status,
            );

            log::info!("Presence set to: {:?}", presence);
        }

        if let Some(commands) = &config.commands {
            if let Some(slash) = &commands.slash {
                for command in &slash.commands {
                    let command =
                        CreateCommand::new(&command.name).description(&command.description);

                    if let Err(why) = ctx
                        .http
                        .create_guild_command(GuildId::from(config.guild_id.clone()), &command)
                        .await
                    {
                        log::error!("Error creating command: {why:?}");
                    }
                }

                log::info!("Slash commands registered: {:?}", slash.commands);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

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
        log::error!("Client error: {why}");
    }

    Ok(())
}
