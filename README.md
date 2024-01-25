# MFA OTP Service

![build workflow](https://github.com/salheb/mfa-service/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An OTP service to generate and validate users in any 2FA process intended. Written in Rust for better performance and small size.

## Local running without Docker Compose
To run this project locally with cargo run, first you need to create a .env file with the following variables:

> HOST=0.0.0.0
> 
> PORT=8080
> 
> ENVIRONMENT=dev
> 
> DATABASE_URL=postgres_database_url

Note that it's necessary to have a Postgres Database instance running and change above DATABASE_URL parameter. The PORT variable can also be changed to anyone that you would.

After this setup, you can just run the bellow command:
> cargo run

## Running with docker compose

In order to run this project using Docker Compose, which includes the Rust API server and the PostgreSQL Database server, you must create a file named ".env.docker" (without brackets) with the following content.

a) Basic environment variables

> HOST=0.0.0.0
> 
> PORT=8080
> 
> ENVIRONMENT=prod

b) Database Settings

> DATABASE_HOST=db_host
> 
> DATABASE_NAME=db_schema_name
> 
> POSTGRES_USER=db_user
> 
> POSTGRES_PASSWORD=db_password
> 
> DATABASE_URL=postegres_url_connection

After this briefly setup, just run the following command in the project root directory (Unix systems only):

> sh run.sh

## Running Diesel Migrations

In order to run Diesel Migrations (which will automatically generate the database entire schema for you), you must run the following command:

> diesel migration run

If you don't have diesel CLI installed in your local environment, <a href="https://diesel.rs/guides/getting-started.html">here</a> you can find how to do that.

In future releases, Diesel Migrations will run in an embedded way, turning this step unnecessary.

## Planned features for next major releasees

1. Embed Diesel migrations if environment is setted up to Development
2. oauth2 API authentication
3. SSL implementation
