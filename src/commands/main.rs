use crate::structs::{Command, Context, Error};
use poise::serenity_prelude as serenity;

// Open register slash command dialogue
#[poise::command(slash_command, prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

// Display user info
#[poise::command(slash_command, prefix_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

// Test command
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

// Joke response command
#[poise::command(slash_command, prefix_command)]
pub async fn pong(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("*chirp* this isn't how this works.. *chirp*")
        .await?;
    Ok(())
}

pub fn commands() -> [Command; 4] {
    [register(), userinfo(), ping(), pong()]
}
