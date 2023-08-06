use crate::structs::{Command, Context, Error};

#[poise::command(slash_command, subcommands("today", "random", "number"))]
async fn xkcd(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

// Fetch the latest XKCD comic
#[poise::command(slash_command, prefix_command)]
async fn today(ctx: Context<'_>) -> Result<(), Error> {
    let response = reqwest::get("https://xkcd.com/info.0.json")
        .await?
        .json::<serde_json::Value>()
        .await?;

    reply_xkcd(response, ctx).await;

    Ok(())
}

// Fetch a random xkcd comic
#[poise::command(slash_command, prefix_command)]
async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let response = reqwest::get("https://c.xkcd.com/random/comic/")
        .await
        .expect("response");
    let num = response.url().path().replace("/", "");

    let response = reqwest::get(format!("https://xkcd.com/{}/info.0.json", num))
        .await?
        .json::<serde_json::Value>()
        .await?;

    reply_xkcd(response, ctx).await;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn number(
    ctx: Context<'_>,
    #[description = "index of XKCD comic"] arg1: u32,
) -> Result<(), Error> {
    let response = reqwest::get(format!("https://xkcd.com/{}/info.0.json", arg1))
        .await?
        .json::<serde_json::Value>()
        .await?;

    reply_xkcd(response, ctx).await;

    Ok(())
}

async fn reply_xkcd(response: serde_json::Value, ctx: Context<'_>) {
    let image_link = response["img"].as_str().unwrap();

    ctx.send(|msg| {
        msg.content(format!("xkcd: {}", response["num"],))
            .embed(|e| {
                e.image(image_link)
                    .footer(|f| f.text(response["alt"].as_str().unwrap()))
            })
    })
    .await
    .expect("send formatted message");
}

pub fn commands() -> [Command; 1] {
    [xkcd()]
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn xkcdreq() {
        let response = reqwest::get("https://c.xkcd.com/random/comic/")
            .await
            .expect("response");
        let num = response.url().path().replace("/", "");

        println!("{:?}", num);
    }
}
