use rocket::serde::json::Json;

use rocket::http::{Status, CookieJar};

use crate::db::user::UserArgs;
use crate::crypto::jwt::create_jwt;

use crate::db::get_user::read_user;


use sqlx::PgPool;
use rocket::State;

use crate::db::create_refresh_entry::{create_refresh_entry, RefreshToken};
use crate::crypto::hash::PasswordHash;

use crate::crypto::create_refresh::create_refresh;

const INTERNAL_ERROR: &str = "An internal server error has occured";

/// Sign-in route to login a user into their account; finds the account with matching credentials and returns a JWT and refresh token for future authentication into protected routes. This route must be entered with json data following the User struct.
///
/// args
/// user: A struct containing the user data sent in the request. This contains a username and password
/// jar: A reference to the cookie jar provided by rocket
///
/// returns
/// Returns a tuple containing a rocket http status and a string

#[rocket::post("/signin", format = "json", data="<user>")]
pub async fn signin(user: Json<UserArgs>, jar: &CookieJar<'_>, pool: &State<PgPool>) -> (Status, String) {

    match read_user(user.username.as_str(), pool.inner()).await {
        Ok(users) => {
            let num_matched_users = users.len();

            if num_matched_users >= 2 {
                // too many accounts
                rocket::error!("Multiple accounts found with the same usernameio ");
                (Status::InternalServerError, "An error with this account has occured, please contact support".to_string())
            } else if num_matched_users == 1 {
                let matched_user = &users[0];

                // Verifing password
                let input_password = &matched_user.password;
                let hash = PasswordHash::from(input_password.clone());

                match hash.verify(&user.password) {
                    Ok(is_password_valid) => {
                        if !is_password_valid {
                            return (Status::Unauthorized, "Password is incorrect".to_string());
                        }

                        match create_refresh() {
                            Ok((cookie, refresh_token)) => {
                                jar.add_private(cookie);

                                match create_refresh_entry(RefreshToken {
                                    user_id: matched_user.user_id,
                                    refresh_token: refresh_token
                                }, pool.inner()).await {
                                    Ok(_) => {
                                        // Creating JWT
                                        let token = create_jwt(matched_user.username.clone(), matched_user.user_id);
                                        (Status::Ok, format!("AuthToken: \"{}\"", token))
                                    },
                                    Err(e) => {
                                        rocket::error!("{}", e);
                                        (Status::InternalServerError, INTERNAL_ERROR.to_string())
                                    }
                                }
                            },
                            Err(e) => {
                                rocket::error!("{:?}", e);
                                (Status::InternalServerError, INTERNAL_ERROR.to_string())
                            }
                        }
                    },
                    Err(e) => {
                        rocket::error!("{}", e);
                        (Status::InternalServerError, INTERNAL_ERROR.to_string())
                    }
                }
            } else {
                // no account
                (Status::NotFound, "No account found by that username".to_string())
            }
        }
        Err(e) => {
            rocket::error!("{}", e);
            (Status::InternalServerError, INTERNAL_ERROR.to_string())
        }
    }
}