mod commands;

use commands::{admin, fun, games, moderation};
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::prelude::*,
    prelude::*,
    Client,
};
use dotenv::dotenv;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "admin_cmd1" => commands::admin::run_admin_cmd1(&ctx, &command).await,
                "games_cmd1" => commands::games::run_games_cmd1(&ctx, &command).await,
                "fun_cmd1" => commands::fun::run_fun_cmd1(&ctx, &command).await,
                "moderation_cmd1" => commands::moderation::run_moderation_cmd1(&ctx, &command).await,
                _ => {}
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in .env")
                .parse()
                .expect("GUILD_ID must be a u64"),
        );

        let mut commands: Vec<fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand> = vec![];

        admin::register_admin_commands(&mut commands);
        games::register_games_commands(&mut commands);
        fun::register_fun_commands(&mut commands);
        moderation::register_moderation_commands(&mut commands);

        let _ = GuildId::set_application_commands(&guild_id, &ctx.http, |b| {
            for cmd in &commands {
                b.create_application_command(|c| cmd(c));
            }
            b
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let intents = GatewayIntents::non_privileged();

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}