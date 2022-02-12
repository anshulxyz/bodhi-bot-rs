# Bodhi Bot ☸️

A Discord bot for referencing Buddhist verses and texts. \
To see the usage of the bot `++help`.

If you have any issues with the bot, or have a suggestion. Feel free to contact
me on [twitter.com/anshulxyz](https://twitter.com/anshulxyz)

## Scripts

Currently available scripts/books/texts:

- Dhammapada

## Features

- Fetch single verse
- Fetch a random verse
- Fetch a range of verses

## Translations

Currently available translation(s):

- Max Müller

Adding more translations/versions and scripts soon.

## Usage example

To get the 14th verse of the Dhammapada

```
++dhp 14
```

To get the verses from 103th till 105th of the Dhammapada
```
++dhp 103 105
```

To get a random verse from the Dhammapada
```
++dhp
```

## Invite

Invite the bot of your server using [this link](https://discord.com/api/oauth2/authorize?client_id=828781402681507860&permissions=274877925376&scope=bot)

## Code

### Run
```
git clone https://github.com/anshulxyz/bodhi-bot-rs.git
cd bodhi-bot-rs
docker build -t bodhi .
docker run --env-file .env bodhi
```
