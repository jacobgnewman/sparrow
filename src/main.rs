use std::env;

// serenity junk
use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};


// songbird
use songbird::SerenityInit;


use reqwest;
use rand;

#[group]
#[commands(ping, pong, join, leave, play, xkcd)] // stop
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn pong(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "*chirp* this isn't how this works.. *chirp*").await?;
    Ok(())
}

#[command]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connection_target = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "Not in a voice channel").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Retrieve Songbird client.")
        .clone();

    let (_handle_lock, success) = manager.join(guild_id, connection_target).await;

    if let Err(why) = success {
        msg.reply(ctx, format!("Failed to join: {:?}", why)).await?;
    }
    Ok(())
}

#[command]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Retrieve Songbird client.")
        .clone();

    if let Err(why) = manager.leave(guild_id).await {
        msg.reply(ctx, format!("Failed to leave: {:?}", why)).await?;
    }
    Ok(())
}

#[command]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            },
        };

        handler.play_source(source);

        check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    Ok(())
}

#[command]
async fn xkcd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let num = args.single::<u32>();

    let response = match num {
        Ok(num) => {
            reqwest::get(format!("https://xkcd.com/{}/info.0.json", num))
                .await?
                .json::<serde_json::Value>()
                .await?
        }
        Err(_) => {
            let initial = reqwest::get("https://xkcd.com/info.0.json")
                .await?
                .json::<serde_json::Value>()
                .await?;
            let max = initial["num"].as_u64().unwrap();
            let random = rand::random::<u64>() % max;
            reqwest::get(format!("https://xkcd.com/{}/info.0.json", random))
                .await?
                .json::<serde_json::Value>()
                .await?
        }
    };

    let image_link = response["img"].as_str().unwrap();
    let image = reqwest::get(image_link).await?.bytes().await?;

    msg.channel_id.send_message(&ctx.http, |m| {
        m.reference_message(msg);
        m.content(
            format!("xkcd: {}\nAlt text: {}", response["num"], response["alt"].as_str().unwrap()),
        );
        m.add_file((image.as_ref(), "xkcd.png"));
        m
    }).await?;

    Ok(())
}

fn check_msg(result: serenity::Result<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}