# Setup

## Requirements

- Install `rustup`, either through your packet manager or by following the instructions on https://rustup.rs/
- Use `rustup` to install the nighly version, which is needed for rocket (until it's next release): `rustup default nightly`
- Install diesel_cli, either through your packet manager or `cargo install diesel_cli --no-default-features --features mysql`

    > Important: You will need `libmysqlclient` to be able to compile diesel_cli.\
    > If you install it using the method above it will only work with mysql databases.\
    > Should you want to use it with all supported ones, remove the feature params and provide the neccessary libraries.

- Install just from your package manager or by running `cargo install just`

That should be all the software required to get this thing running.

## Configuring

Create a file called `.env`. This will contain the variables used to connect to the database. Just exporting them would be fine too.\
`DATABASE_URL` will be used by diesel_cli to execute the migrations. Format: `mysql://user:pass@ip:port/database_name`\
`ROCKET_DATABASES` will be used by rocket to use the connection the program. Format: `{main={url='mysql://user:pass@ip:port/database_name'}}`

You can alternativly put a filename instead of the mysql url. This will not need a database server.

Run `diesel migrations run` to populate the database.\
After that calling `just run` will start up the rocket webserver and all should be good to go.

Navigating to `localhost:8000/seeding` will populate the database with testing data.
