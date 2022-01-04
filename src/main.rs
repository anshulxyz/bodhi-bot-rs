use std::env;

use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _msg: Message) {}

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, Discord!");

    // discord bot token
    let token = env::var("DISCORD_TOKEN").expect("Expected a token from environment.");

    let framework = StandardFramework::new()
        .configure(|f| {
            f.with_whitespace(false)
                .prefix("++")
                .case_insensitivity(false)
                .delimiters(vec![" "])
        })
        .group(&GENERAL_GROUP);

    // app client
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", &why)
    }
}

#[group]
#[commands(dhp)]
struct General;

#[command]
async fn dhp(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let response = MessageBuilder::new()
        .push("All arguments are: ")
        .push(&args.rest())
        .build();

    msg.channel_id.say(&ctx.http, &response).await?;
    Ok(())
}
