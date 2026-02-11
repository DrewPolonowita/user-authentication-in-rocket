use crate::db::user::UserArgs;
use sqlx::PgPool;

/// Creates a user entry in a PostgreSQL database using SQLx. This function does not check the input
///
/// # Arguments
/// - `user`: A reference to a UserArgs struct containing a username and hashed password
/// - `pool`: A reference to a PgPool which is required to connect to SQL databases
///
/// # Returns
/// A Result enum with no data or an sql::error::Error enum if the operation is not successful
pub async fn write_user(user: &UserArgs, pool: &PgPool) -> Result<(),  sqlx::error::Error> {
    let _ = sqlx::query!(
            r#"
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            "#,
            user.username,
            user.password,
            )
        .execute(pool)
        .await?;

    Ok(())
}