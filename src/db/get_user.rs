use sqlx::PgPool;
use crate::db::user::User;

/// Creates a user entry in a PostgreSQL database using SQLx. This function does not check the input
///
/// # Arguments
/// - `user`: A reference to a UserArgs struct containing a username and hashed password
/// - `pool`: A reference to a PgPool which is required to connect to SQL databases
///
/// # Returns
/// A Result enum with no data or an sql::error::Error enum if the operation is not successful
pub async fn read_user(username: &str, pool: &PgPool) -> Result<Vec<User>,  sqlx::error::Error> {
    let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE username = $1
            "#,
            username
            )
        .fetch_all(pool)
        .await?;

    Ok(user)
}