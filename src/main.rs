mod commands;
mod structs;

use crate::structs::Data;

use dotenv::dotenv;
use std::env;

// use poise::serenity_prelude::{self as serenity, Mentionable as _, builder::*};
use poise::serenity_prelude as serenity;

use log::{debug, info};

// songbird
use songbird::SerenityInit;


#[tokio::main]
async fn main() {
    // get discord token
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("token");

    // configure poise
    let framework_options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(
                std::time::Duration::from_secs(3600),
            )),
            case_insensitive_commands: true,
            ..Default::default()
        },
        commands: commands::commands(),
        ..poise::FrameworkOptions::default()
    };

    let data = Data {
        songbird: songbird::Songbird::serenity()
    };

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // build framework
    let framework = poise::Framework::builder()
        .token(token)
        .options(framework_options)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .client_settings(|builder| builder.register_songbird());

    // go-time
    framework.run().await.unwrap();
}
