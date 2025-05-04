use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use serenity::model::prelude::*;

pub fn register_admin_commands(cmds: &mut Vec<fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand>) {
    cmds.push(admin_cmd1);
}

pub fn admin_cmd1(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("admin_cmd1").description("Get server ID")
}

pub async fn run_admin_cmd1(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    if let Some(guild_id) = interaction.guild_id {
        let _ = interaction
            .create_interaction_response(&ctx.http, |resp| {
                resp.interaction_response_data(|msg| {
                    msg.content(format!("Server ID: {}", guild_id.0))
                })
            })
            .await;
    }
}