use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::prelude::*;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub fn register_moderation_commands(cmds: &mut Vec<fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand>) {
    cmds.push(moderation_cmd1);
    cmds.push(warn_user);
}

pub fn moderation_cmd1(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("moderation_cmd1").description("Pretend to ban a user")
}

pub async fn run_moderation_cmd1(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let user = &interaction.user;
    let _ = interaction
        .create_interaction_response(&ctx.http, |resp| {
            resp.interaction_response_data(|msg| {
                msg.content(format!("Banning user... just kidding, {} 😄", user.name))
            })
        })
        .await;
}

pub fn warn_user(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("warn")
        .description("Warn a user")
        .create_option(|opt| {
            opt.name("user")
                .description("User to warn")
                .kind(serenity::model::prelude::command::CommandOptionType::User)
                .required(true)
        })
        .create_option(|opt| {
            opt.name("reason")
                .description("Reason for the warning")
                .kind(serenity::model::prelude::command::CommandOptionType::String)
                .required(true)
        })
}

pub async fn run_warn(ctx: &Context, interaction: &ApplicationCommandInteraction, db: Arc<Mutex<Connection>>) {
    let user = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "user")
        .and_then(|opt| opt.resolved.as_ref())
        .and_then(|v| {
            if let CommandDataOptionValue::User(user, _) = v {
                Some(user)
            } else {
                None
            }
        });

    let reason = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "reason")
        .and_then(|opt| opt.value.as_ref())
        .and_then(|v| v.as_str());

    if let (Some(user), Some(reason)) = (user, reason) {
        let mod_id = interaction.user.id.0.to_string();
        let user_id = user.id.0.to_string();
        let reason_str = reason.to_string();

        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO warnings (user_id, moderator_id, reason) VALUES (?1, ?2, ?3)",
            [&user_id, &mod_id, &reason_str],
        )
        .unwrap();

        let _ = interaction.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| d.content(format!("Warned {} for: {}", user.name, reason)))
        }).await;
    }
}

pub fn view_warnings(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("warnings")
        .description("View a user's warnings")
        .create_option(|opt| {
            opt.name("user")
                .description("User to view warnings for")
                .kind(serenity::model::prelude::command::CommandOptionType::User)
                .required(true)
        })
}

pub async fn run_view_warnings(ctx: &Context, interaction: &ApplicationCommandInteraction, db: Arc<Mutex<Connection>>) {
    let user = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "user")
        .and_then(|opt| opt.resolved.as_ref())
        .and_then(|v| {
            if let CommandDataOptionValue::User(user, _) = v {
                Some(user)
            } else {
                None
            }
        });

    if let Some(user) = user {
        let user_id = user.id.0.to_string();
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT reason, timestamp FROM warnings WHERE user_id = ?1").unwrap();
        let mut rows = stmt.query([&user_id]).unwrap();

        let mut entries = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            let reason: String = row.get(0).unwrap();
            let timestamp: String = row.get(1).unwrap();
            entries.push(format!("• {} at {}", reason, timestamp));
        }

        let response = if entries.is_empty() {
            format!("{} has no warnings.", user.name)
        } else {
            format!("Warnings for {}:
{}", user.name, entries.join("
"))
        };

        let _ = interaction.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| d.content(response))
        }).await;
    }
}

pub fn clear_warnings(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("clearwarnings")
        .description("Clear all warnings for a user")
        .create_option(|opt| {
            opt.name("user")
                .description("User to clear warnings for")
                .kind(serenity::model::prelude::command::CommandOptionType::User)
                .required(true)
        })
}

pub async fn run_clear_warnings(ctx: &Context, interaction: &ApplicationCommandInteraction, db: Arc<Mutex<Connection>>) {
    let user = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "user")
        .and_then(|opt| opt.resolved.as_ref())
        .and_then(|v| {
            if let CommandDataOptionValue::User(user, _) = v {
                Some(user)
            } else {
                None
            }
        });

    if let Some(user) = user {
        let user_id = user.id.0.to_string();
        let conn = db.lock().unwrap();
        conn.execute("DELETE FROM warnings WHERE user_id = ?1", [&user_id]).unwrap();

        let _ = interaction.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| d.content(format!("Cleared all warnings for {}", user.name)))
        }).await;
    }
}


pub fn history(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("history")
        .description("View all moderation actions across users")
}

pub async fn run_history(ctx: &Context, interaction: &ApplicationCommandInteraction, db: Arc<Mutex<Connection>>) {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT user_id, reason, timestamp FROM warnings ORDER BY timestamp DESC LIMIT 10").unwrap();
    let mut rows = stmt.query([]).unwrap();

    let mut entries = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let user_id: String = row.get(0).unwrap();
        let reason: String = row.get(1).unwrap();
        let timestamp: String = row.get(2).unwrap();
        entries.push(format!("• <@{}>: {} at {}", user_id, reason, timestamp));
    }

    let response = if entries.is_empty() {
        "No moderation history found.".to_string()
    } else {
        format!("Recent moderation actions:
{}", entries.join("
"))
    };

    let _ = interaction.create_interaction_response(&ctx.http, |r| {
        r.interaction_response_data(|d| d.content(response))
    }).await;
}

// Modified warn handler with auto-punishment logic
pub async fn run_warn(ctx: &Context, interaction: &ApplicationCommandInteraction, db: Arc<Mutex<Connection>>) {
    let user = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "user")
        .and_then(|opt| opt.resolved.as_ref())
        .and_then(|v| {
            if let CommandDataOptionValue::User(user, _) = v {
                Some(user)
            } else {
                None
            }
        });

    let reason = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "reason")
        .and_then(|opt| opt.value.as_ref())
        .and_then(|v| v.as_str());

    if let (Some(user), Some(reason)) = (user, reason) {
        let mod_id = interaction.user.id.0.to_string();
        let user_id = user.id.0.to_string();
        let reason_str = reason.to_string();

        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO warnings (user_id, moderator_id, reason) VALUES (?1, ?2, ?3)",
            [&user_id, &mod_id, &reason_str],
        )
        .unwrap();

        // Check warning count
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM warnings WHERE user_id = ?1").unwrap();
        let count: i64 = stmt.query_row([&user_id], |row| row.get(0)).unwrap();

        let punishment = match count {
            3 => Some("⚠️ This user has 3 warnings. Consider muting them."),
            5 => Some("🚫 This user has 5 warnings. Consider banning them."),
            _ => None,
        };

        let mut msg = format!("Warned {} for: {}", user.name, reason);
        if let Some(extra) = punishment {
            msg.push_str(&format!("
{}", extra));
        }

        let _ = interaction.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| d.content(msg))
        }).await;
    }
}


// Utility: fetch logging channel (hardcoded or from config in future)
async fn get_log_channel(ctx: &Context) -> Option<serenity::model::id::ChannelId> {
    // Replace with your actual log channel ID
    Some(serenity::model::id::ChannelId(123456789012345678)) // replace with real channel ID
}

// Enhanced warn handler with automatic mute/ban and logging
pub async fn run_warn(ctx: &Context, interaction: &ApplicationCommandInteraction, db: Arc<Mutex<Connection>>) {
    let guild_id = interaction.guild_id;
    let user = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "user")
        .and_then(|opt| opt.resolved.as_ref())
        .and_then(|v| {
            if let CommandDataOptionValue::User(user, _) = v {
                Some(user)
            } else {
                None
            }
        });

    let reason = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "reason")
        .and_then(|opt| opt.value.as_ref())
        .and_then(|v| v.as_str());

    if let (Some(user), Some(reason), Some(guild_id)) = (user, reason, guild_id) {
        let mod_id = interaction.user.id.0.to_string();
        let user_id = user.id.0.to_string();
        let reason_str = reason.to_string();

        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO warnings (user_id, moderator_id, reason) VALUES (?1, ?2, ?3)",
            [&user_id, &mod_id, &reason_str],
        )
        .unwrap();

        // Count warnings
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM warnings WHERE user_id = ?1").unwrap();
        let count: i64 = stmt.query_row([&user_id], |row| row.get(0)).unwrap();

        let mut actions = vec![format!("Warned {} for: {}", user.name, reason)];

        if count == 3 {
            // Attempt mute (add timeout if permissions allow)
            if let Ok(member) = guild_id.member(&ctx.http, user.id).await {
                let _ = member.disable_communication_until_datetime(&ctx.http, chrono::Utc::now() + chrono::Duration::minutes(10)).await;
                actions.push("⚠️ Auto-muted for 10 minutes (3 warnings).".to_string());
            }
        } else if count >= 5 {
            // Attempt ban
            let _ = guild_id.ban_with_reason(&ctx.http, user.id, 0, reason_str.clone()).await;
            actions.push("🚫 Auto-banned after 5 warnings.".to_string());
        }

        // Respond to command
        let _ = interaction.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| d.content(actions.join("
")))
        }).await;

        // Log to a channel
        if let Some(log_ch) = get_log_channel(ctx).await {
            let _ = log_ch.say(&ctx.http, actions.join("
")).await;
        }
    }
}
