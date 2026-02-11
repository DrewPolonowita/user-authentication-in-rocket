use rocket::request::{FromRequest, Outcome};
use rocket::{Request, http::Status, http::CookieJar};

use crate::db::get_refresh_entry::get_refresh_entry;
use crate::crypto::jwt::{JwtStatus, decode_jwt, create_jwt};
use crate::crypto::hash::PasswordHash;

use sqlx::PgPool;
use rocket::State;

#[derive(Debug)]
pub struct RefreshUser {
    pub user_id: i64,
    pub username: String
}

const INTERNAL_ERROR: &str = "An internal server error has occured";

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RefreshUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(header) => {
                let Some(key) = header.strip_prefix("Bearer ") else {
                    return Outcome::Forward(Status::BadRequest);
                };
                match decode_jwt(key) {
                    Ok(JwtStatus::Expired(user)) => {
                        Outcome::Success(RefreshUser {
                            user_id: user.user_id,
                            username: user.username
                        })
                    },
                    Ok(JwtStatus::Valid(user)) => {
                        Outcome::Success(RefreshUser {
                            user_id: user.user_id,
                            username: user.username
                        })
                    },
                    _ => Outcome::Error((Status::Unauthorized, ()))
                }
            },
            None => Outcome::Forward(Status::Unauthorized)
        }
    }
}

/// Refresh JWT Route, to remake a JWT token from an expired JWT Token, The request must contain the valid refresh token in the HTTP-Only Cookies.
///
/// # Arguments
/// - `user`: A RefreshUser struct containing the user_id and the username
/// - `jar`: A reference to the cookie jar provided by rocket
///
/// # Returns
/// Returns a tuple containing a rocket http status and a String containing the new JWT token or an error message
#[rocket::get("/refresh")]
pub async fn refresh(user: RefreshUser, jar: &CookieJar<'_>, pool: &State<PgPool>) -> (Status, String) {
    match get_refresh_entry(user.user_id, pool.inner()).await {
        Ok(token) => {
            let Some(cookie) = jar.get_private("refresh_token") else {
                return (Status::Unauthorized, "No valid refresh token".to_string());
            };

            let hash_refresh_token = PasswordHash::from(token.refresh_token);

            match hash_refresh_token.verify(&cookie.value().to_string()) {
                Ok(is_token_valid) => {
                    if is_token_valid {
                        return (Status::Unauthorized, "Refresh token is invalid".to_string());
                    }

                    let token = create_jwt(user.username, user.user_id);
                    (Status::Ok, format!("AuthToken: \"{}\"", token))
                },
                Err(e) => {
                    rocket::error!("{}", e);
                    return (Status::InternalServerError, INTERNAL_ERROR.to_string());
                }
            }
        },
        Err(sqlx::error::Error::RowNotFound) => {
            (Status::NotFound, "User doesn't exist".to_string())
        },
        Err(e) => {
            rocket::error!("{}", e);
            (Status::InternalServerError, INTERNAL_ERROR.to_string())
        }
    }
}