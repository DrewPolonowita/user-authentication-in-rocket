use rocket::serde::json::Json;
use crate::db::create_user::write_user;

use rocket::State;

use rocket::http::Status;
use crate::db::user::{UserArgs, AuthUser};

use sqlx::PgPool;

use crate::crypto::hash::PasswordHash;

const INTERNAL_ERROR: &str = "An internal server error has occured";
const DATABASE_CONFLICT_CODE: &str = "23505";

/// Create account route to create a user account; this function hashes the password and stores the data in a PostegreSQL database. This route must be entered with valid JSON data following the structure of the Users struct in the Users file.
///
/// # Arguments
/// - `user`: A struct containing the user data sent in the request. This contains a username and password
/// - `pool`: A reference to a State<PgPool> which is required for connecting to an SQL database
///
/// # Returns
/// Returns a tuple containing a rocket http status code and a string reference to be returned to the client
#[rocket::post("/create", format = "json", data="<user>")]
pub async fn create(user: Json<UserArgs>, pool: &State<PgPool>) -> (Status, &'static str) {

    let Ok(hashed_password) = PasswordHash::try_from(user.password.as_str()) else {
        return (Status::InternalServerError, INTERNAL_ERROR);
    };
    match PasswordHash::try_from(user.password.as_str()) {
        Ok(hashed_password) => {
            match write_user(&UserArgs {
                username: user.username.clone(),
                password: hashed_password.value()
            }, pool.inner()).await {

                Ok(_) => (Status::Created, "Account created successfully"),

                // For database errors
                Err(sqlx::error::Error::Database(e)) => {

                    let db_error = e.downcast_ref::<sqlx::postgres::PgDatabaseError>();

                    if db_error.code() == DATABASE_CONFLICT_CODE {
                        (Status::Conflict, "User already exists")
                    } else {
                        rocket::error!("{}", e);
                        (Status::InternalServerError, INTERNAL_ERROR)
                    }
                },

                // For generic errors
                Err(e) => {
                    rocket::error!("{}", e);
                    (Status::InternalServerError, INTERNAL_ERROR)
                }
            }
        },
        Err(e) => {
            rocket::error!("{}", e);
            (Status::InternalServerError, INTERNAL_ERROR)
        }
    }
}

#[rocket::get("/test")]
pub fn test_route(user: AuthUser) -> String {
    format!("Hello there: {}, UserID: {}", user.username, user.user_id)
}