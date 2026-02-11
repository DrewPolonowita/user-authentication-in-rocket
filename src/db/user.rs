use rocket::serde::{Deserialize, Serialize};

use rocket::request::{FromRequest, Outcome};
use rocket::{Request, http::Status};

use sqlx::FromRow;

use crate::crypto::jwt::{JwtStatus, decode_jwt};

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserArgs {
    pub username: String,
    pub password: String
}

pub struct AuthUser {
    pub user_id: isize,
    pub username: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        match request.headers().get_one("Authorization") {
            Some(header) => {
                let Some(key) = header.strip_prefix("Bearer ") else {
                    return Outcome::Forward(Status::BadRequest);
                };

                match decode_jwt(key) {
                    Ok(JwtStatus::Valid(user)) => {
                        Outcome::Success(AuthUser {
                            user_id: user.user_id as isize,
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