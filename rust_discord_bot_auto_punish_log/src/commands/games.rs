use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use rand::seq::SliceRandom;

pub fn register_games_commands(cmds: &mut Vec<fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand>) {
    cmds.push(games_cmd1);
}

pub fn games_cmd1(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("games_cmd1").description("Play rock-paper-scissors")
}

pub async fn run_games_cmd1(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let choices = ["Rock", "Paper", "Scissors"];
    let bot_choice = choices.choose(&mut rand::thread_rng()).unwrap();

    let _ = interaction
        .create_interaction_response(&ctx.http, |resp| {
            resp.interaction_response_data(|msg| msg.content(format!("I choose **{}**!", bot_choice)))
        })
        .await;
}