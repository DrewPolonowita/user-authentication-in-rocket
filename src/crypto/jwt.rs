use jsonwebtoken::{Header, encode, EncodingKey, DecodingKey, Validation, Algorithm, decode};
use rocket::serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug)]
pub enum JwtStatus {
    Valid(Claims),
    Expired(Claims)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub user_id: i64,
    pub username: String,
    pub exp: i64
}

pub fn create_jwt(username: String, user_id: i64) -> String {
    // Creates a JWT for a user to be used for authentication into protected routes
    //
    // args
    // username: A string of the username of the user creating the JWT
    //
    // returns
    // A string with is the JWT token

    let (private_key, jwt_expire_time) = get_env_vars();

    encode(
        &Header::default(),
        &Claims {
            user_id: user_id,
            username: username,
            exp: chrono::Utc::now().timestamp().checked_add(jwt_expire_time).unwrap()
        },
        &EncodingKey::from_secret(&private_key.to_string().into_bytes())
    ).unwrap()
}

pub fn decode_jwt(token: &str) -> Result<JwtStatus, jsonwebtoken::errors::Error> {
    // Decodes a JWT and returns the users claims data within, also checks if the token is expired, this is used for route authentication
    //
    // args
    // token: A JWT token to authenticate
    //
    // returns
    // A result enum with the Claims struct used to create the JWT or a jsonwebtoken error enum

    let (private_key, _jwt_expire_time) = get_env_vars();

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;

    let user_data = decode::<Claims>(
        &token, &DecodingKey::from_secret(&private_key.to_string().into_bytes()), &validation
    )?;

    if user_data.claims.exp <= chrono::Utc::now().timestamp() {
        Ok(JwtStatus::Expired(user_data.claims))
    } else {
        Ok(JwtStatus::Valid(user_data.claims))
    }
}

fn get_env_vars() -> (String, i64) {
    let Ok(private_key) = env::var("JWT_PRIVATE_KEY") else {
        eprintln!("JWT_PRIVATE_KEY is not set in .env");
        std::process::exit(1)
    };
    let Ok(jwt_expire_time) = env::var("JWT_EXPIRE_TIME") else {
        eprintln!("JWT_EXPIRE_TIME is not set in .env");
        std::process::exit(1)
    };
    let Ok(jwt_expire_time) = jwt_expire_time.parse::<i64>() else {
        eprintln!("JWT_EXPIRE_TIME is not an i64 integer in .env");
        std::process::exit(1)
    };

    (private_key, jwt_expire_time)
}