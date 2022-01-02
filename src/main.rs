use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("This is an embed");
                        e.description("With a description");

                        e
                    });

                    m
                })
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, Discord!");

    // discord bot token
    let token = env::var("DISCORD_TOKEN").expect("Expected a token from environment.");

    // app client
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", &why)
    }
}
