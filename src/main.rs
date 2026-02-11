use server::routes::accounts::create_account::{create, test_route};
use server::routes::accounts::signin::signin;
use server::routes::accounts::refresh_token::refresh;

use rustls::crypto::CryptoProvider;

use sqlx::postgres::PgPoolOptions;

use dotenv::dotenv;
use std::env;

use server::logs::log_errors::setup_logging;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // crypto setup
    CryptoProvider::install_default(
        rustls::crypto::aws_lc_rs::default_provider()
        ).expect("Failed to install default crypto provider");

    //sql stuff
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.expect("Database pool error");

    // logging setup
    let _ = setup_logging().expect("Failed to initilize logging");

    let _rocket = rocket::build()
        .manage(pool)
        .mount("/", rocket::routes!(create, signin, test_route, refresh))
        .launch()
        .await?;

    Ok(())
}