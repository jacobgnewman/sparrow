use crate::structs::{Command, Context, Error};
use log::{info, error};


// Get Sparrow to join your current voice call
#[poise::command(slash_command, prefix_command)]
async fn join(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connection_target = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("Not in a voice channel").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("failed to get client")
        .clone();

    let (_handle_lock, success) = manager.join(guild_id, connection_target).await;

    if let Err(why) = success {
        ctx.say(format!("Failed to join: {:?}", why)).await?;
    }
    Ok(())
}


// Get Sparrow to leave it's current voice call
#[poise::command(slash_command, prefix_command)]
async fn leave(
    ctx: Context<'_>
) -> Result<(), Error> {

    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Retrieve Songbird client.")
        .clone();

    if let Err(why) = manager.leave(guild_id).await {
        ctx.say(format!("Failed to leave: {:?}", why)).await?;
    }
    Ok(())
}


#[poise::command(slash_command, prefix_command)]
async fn play(
    ctx: Context<'_>,
    #[description = "Youtube URL"] url: String
    ) -> Result<(), Error>{


    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.serenity_context()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                error!("Err starting source: {:?}", why);

                ctx.say("Error starting source, refer to logs").await?;
                return Ok(());
            },
        };
        
        handler.enqueue_source(source);


        ctx.say("Playing song").await?;
    } else {
        ctx.say("Not in a voice channel to play in").await?;
    }

    Ok(())
}

pub fn commands() -> [Command; 3] {
    [join(), leave(), play()]
}