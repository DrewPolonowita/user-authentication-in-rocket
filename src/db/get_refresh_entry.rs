use sqlx::PgPool;
use crate::db::create_refresh_entry::RefreshToken;

/// Creates a user entry in a PostgreSQL database using SQLx. This function does not check the input
///
/// # Arguments
/// - `user`: A reference to a UserArgs struct containing a username and hashed password
/// - `pool`: A reference to a PgPool which is required to connect to SQL databases
///
/// # Returns
/// A Result enum with no data or an sql::error::Error enum if the operation is not successful
pub async fn get_refresh_entry(user_id: i64, pool: &PgPool) -> Result<RefreshToken,  sqlx::error::Error> {
    let tokens = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT * FROM refresh_tokens
            WHERE user_id = $1
            LIMIT 1
            "#,
            user_id
            )
        .fetch_all(pool)
        .await?;


    tokens.get(0).cloned().ok_or(sqlx::error::Error::RowNotFound)
}