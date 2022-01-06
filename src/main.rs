use std::env;

use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::{command, group, help},
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
        .help(&HELP)
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
async fn dhp(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 1_usize {
        let first = args.single::<f64>()?;
        let response = MessageBuilder::new().push(&first).build();

        msg.channel_id.say(&ctx.http, &response).await?;
    } else if args.len() == 2_usize {
        let first = args.single::<f64>()?;
        let second = args.single::<f64>()?;
        let response = MessageBuilder::new()
            .push("Found two: ")
            .push(&first)
            .push(" and ")
            .push(&second)
            .build();
        msg.channel_id.say(&ctx.http, &response).await?;
    } else if args.is_empty() {
        msg.channel_id
            .say(&ctx.http, "WIP random verse feature")
            .await?;
    } 
    else {
        msg.channel_id
            .say(&ctx.http, "Please try the help command. `++help`")
            .await?;
    }

    Ok(())
}

#[help]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Help");
                e.colour((255, 153, 0));
                e.description("How to use this bot.");
                e.field("`++dhp`", "Get a random verse", false);
                e.field("`++dhp 209`", "Get the 209th verse", false);
                e.field("`++dhp 103 106`", "Get verses from 103 to 106", false);

                e
            });
            m
        })
        .await
    {
        println!("Error executing help command: {:?}", why);
    }
    Ok(())
}
