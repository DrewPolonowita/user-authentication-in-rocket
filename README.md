# Rocket authentication server
Author: Drew Polonowita

## Intro

An authentication server built using rust with rocket. Contains routes including \create \signin \refresh and a \test route which is protected behind user authentication. The server links to a PostgreSQL database in the backend which I have been using CockroachDB online for. Errors are logged into an error_file

## Database setup instructions (SQL commands)

Once a server has been setup, use the commands in database_setup.sql to create the SQL server

## Setup instructions

Once a pull request has been created a .env file is requried with the following required variables \
\
DATABASE_URL=\
sslmode=verify-full\
JWT_PRIVATE_KEY=\
JWT_EXPIRE_TIME=\
\
where DATABASE_URL is the link to the PostgreSQL server, JWT_PRIVATE_KEY is the private key used to encrypt JWT tokens used for user authentication and JWT_EXPIRE_TIME is the amount of time in seconds from the creation of the JWT where the user will need to create a new token with \refresh. \
\
Then provided rust has been installed https://rust-lang.org/tools/install/ in the repo directory 'cargo run' to obtain a test version or 'cargo build' can be ran to obtain an application version to be used.