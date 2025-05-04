use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use rand::seq::SliceRandom;

pub fn register_fun_commands(cmds: &mut Vec<fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand>) {
    cmds.push(fun_cmd1);
}

pub fn fun_cmd1(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("fun_cmd1").description("Get a random joke")
}

pub async fn run_fun_cmd1(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let jokes = [
        "Why don’t skeletons fight each other? They don’t have the guts.",
        "Why did the scarecrow win an award? He was outstanding in his field.",
        "What do you call fake spaghetti? An impasta!"
    ];
    let joke = jokes.choose(&mut rand::thread_rng()).unwrap();

    let _ = interaction
        .create_interaction_response(&ctx.http, |resp| {
            resp.interaction_response_data(|msg| msg.content(joke))
        })
        .await;
}