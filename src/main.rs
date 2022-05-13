use rand::Rng;

use std::collections::HashSet;
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
    http::Http,
    model::{channel::Message, gateway::Ready, gateway::Activity},
    prelude::*,
    utils::MessageBuilder,
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
    let token = env::var("BODHI_TOKEN").expect("Expected a discord token from environment.");

    let http = Http::new(&token);

    // refer: https://github.com/serenity-rs/serenity/blob/3a64da19e75f2c70830beeca9c0963f7d579a992/examples/e05_command_framework/src/main.rs#L228-L245
    // This is for fetching owner's ID so that it can be used to make Owner Only commands
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // relative location of the SQLite database file
    let database_url = env::var("DATABASE_URL").expect("Expected a database url from environment.");

    let db: DatabaseConnection = Database::connect(database_url)
        .await
        .expect("Error creating database connection");

    // build the framework, can specify a prefic here (which I am not using)
    let framework = StandardFramework::new()
        .configure(|f| {
            f.with_whitespace(false)
                .on_mention(Some(bot_id))
                .case_insensitivity(true)
                .delimiters(vec![" "])
                .owners(owners)
        })
        .help(&HELP)
        .group(&OWNER_GROUP)
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES;

    // app client
    let mut client = Client::builder(&token, intents)
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
#[commands(dhp, invite)]
struct General;

#[command]
async fn dhp(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let client_data = ctx.data.read().await;
    let db = client_data
        .get::<DBConnection>()
        .expect("Expected DBConnection in TypeMap");

    // single number as argument, user requesting one verse
    if args.len() == 1_usize {
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
                    e.title("Dhammapada :wheel_of_dharma:");
                    e.colour((255, 153, 0));
                    e.field(verse.num, verse.muller, false);
                    e.footer(|f| f.text("Translation: Max Müller"));
                    e
                });
                m
            })
            .await
        {
            println!("Error sending single verse. {:?}", why)
        }
    } else if args.len() == 2_usize {
        // user requesting a range of verses
        let mut first_num = args.single::<i32>()?;
        let mut last_num = args.single::<i32>()?;

        // since 423 is the last verse
        if first_num > 423 {
            first_num = 423;
        }
        if last_num > 423 {
            last_num = 423;
        }

        // swap in case user enters bigger number before smaller
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
                    e.title("Dhammapada :wheel_of_dharma:");
                    e.colour((255, 153, 0));
                    e.fields(verses);
                    e.footer(|f| f.text("Translation: Max Müller"));
                    e
                });
                m
            })
            .await
        {
            println!("Error sending multi verse. {:?}", why)
        }
    } else if args.is_empty() {
        // if no arguments are given, then return a random verse
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
                    e.title("Dhammapada :wheel_of_dharma:");
                    e.colour((255, 153, 0));
                    e.field(verse.num, verse.muller, false);
                    e.footer(|f| f.text("Translation: Max Müller"));
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
            .say(&ctx.http, "Please try the help command. `@BodhiBot help`")
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
                e.field("Dhammapada (dhp)", "———", false);
                e.field("`@BodhiBot dhp`", "Get a random verse", false);
                e.field(
                    "`@BodhiBot dhp 209`",
                    "Get the 209th verse. Total 423 verses.",
                    false,
                );
                e.field(
                    "`@BodhiBot dhp 103 106`",
                    "Get verses from 103 to 106",
                    false,
                );
                e.field("Others", "———", false);
                e.field("`@BodhiBot invite`", "Invite link for this bot", false);

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

#[command]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Invite");
                e.colour((255, 153, 0));
                // e.description("Invite Bot link");
                e.field("—", "[Click here to invite the bot](https://discord.com/api/oauth2/authorize?client_id=828781402681507860&permissions=277025392640&scope=bot%20applications.commands)", false);

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

#[group]
#[commands(stats, activity)]
#[owners_only]
struct Owner;

/// A Owner only command that returns the bumber of servers/guilds the bot is part of.
#[command]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    let guilds = ctx.cache.guilds().len();

    let response = MessageBuilder::new()
        .push("Guilds in the Cache: ")
        .push(guilds)
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error executing stats command: {:?}", why);
    }

    Ok(())
}

#[command]
async fn activity(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let name = args.message();
    ctx.set_activity(Activity::playing(&name)).await;
    println!("Status set");
    Ok(())
}
