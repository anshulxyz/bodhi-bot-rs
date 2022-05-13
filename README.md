# Bodhi Bot ☸️

A Discord bot for referencing Buddhist verses and texts.

If you have any issues with the bot, or have a suggestion. Feel free to contact
me on [twitter.com/anshulxyz](https://twitter.com/anshulxyz)

## Scriptures

### Dhammapada

Features:-

- Fetch single verse
- Fetch a random verse
- Fetch a range of verses

## Usage example

To use the bot, you have to @\mention the bot.

To get the 14th verse of the Dhammapada

    @BodhiBot dhp 14

To get the verses from 103th till 105th of the Dhammapada

    @BodhiBot dhp 103 105

To get a random verse from the Dhammapada

    @BodhiBot dhp

Current translation in use is by Max Müller.

## Invite

Invite the bot of your server using [this link](https://discord.com/api/oauth2/authorize?client_id=828781402681507860&permissions=277025392640&scope=bot%20applications.commands)

## Code

The code is written in Rust (v1.60.0).

Libraries:-

1. [Serenity](https://github.com/serenity-rs/serenity/), for Discord API
2. [Sea-ORM](https://github.com/SeaQL/sea-orm), for ORM to access SQLite Database

I am making the source code of this bot public on order to have it serve as an
example of usage of the above libraries. This bot was previously written in
Python. A goal to rewrite it, was to learn Rust.

If you're interested in building the source code, please contact me on
[twitter](https://twitter.com/anshulxyz). You'll need a SQLite database that
follows the schema defined in [dhammapada.rs](src/data/dhammapada.rs).

