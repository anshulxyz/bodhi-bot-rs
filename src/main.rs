use rand::Rng;

use std::env;
use std::mem;
mod data;

use data::dhammapada as dhp;

use sea_orm::{Database, DatabaseConnection, EntityTrait};
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
};
struct Handler;

struct DBConnection;

impl TypeMapKey for DBConnection {
    type Value = DatabaseConnection;
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, Discord!");

    // discord bot token
    let token = env::var("DISCORD_TOKEN").expect("Expected a token from environment.");

    let database_url = env::var("DATABASE_URL").expect("Expected a database url from environment.");

    // database connection
    let db: DatabaseConnection = Database::connect(database_url)
        .await
        .expect("Error creating database connection");

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
        .type_map_insert::<DBConnection>(db)
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
    let client_data = ctx.data.read().await;
    let db = client_data
        .get::<DBConnection>()
        .expect("Expected DBConnection in TypeMap");

    if args.len() == 1_usize {
        // single verse number
        let first = args.single::<i32>()?;
        let verse: dhp::Model = dhp::Entity::find_by_id(first)
            .one(db)
            .await
            .expect("Error fetching a verse")
            .unwrap();
        if let Err(why) = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Dhammapada");
                    e.colour((255, 153, 0));
                    e.field(verse.num, verse.muller, false);
                    e
                });
                m
            })
            .await
        {
            println!("Error sending single verse. {:?}", why)
        }
    } else if args.len() == 2_usize {
        // verse number range
        let mut first_num = args.single::<i32>()?;
        let mut last_num = args.single::<i32>()?;

        // in case user enter bigger number before smaller
        if first_num > last_num {
            mem::swap(&mut first_num, &mut last_num);
        }

        // since Discord only allows 25 fields in embed at max
        if last_num - first_num > 25 {
            last_num = 25;
        }

        let mut verses = Vec::new();

        // loop over first verse to last and add tuple to the vector
        for n in first_num..=last_num {
            let verse: dhp::Model = dhp::Entity::find_by_id(n)
                .one(db)
                .await
                .expect("Error fetching a verse for id:{n}")
                .unwrap();
            verses.push((verse.num, verse.muller, false))
        }

        if let Err(why) = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Dhammapada");
                    e.colour((255, 153, 0));
                    e.fields(verses);
                    e
                });
                m
            })
            .await
        {
            println!("Error sending multi verse. {:?}", why)
        }
    } else if args.is_empty() {
        // send a random verse
        let mut rng = rand::rngs::OsRng;
        let random_num: i32 = rng.gen_range(1..=423);

        let verse: dhp::Model = dhp::Entity::find_by_id(random_num)
            .one(db)
            .await
            .expect("Error fetching a verse")
            .unwrap();

        if let Err(why) = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Dhammapada");
                    e.colour((255, 153, 0));
                    e.field(verse.num, verse.muller, false);
                    e
                });
                m
            })
            .await
        {
            println!("Error sending random verse. {:?}", why)
        }
    } else {
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
