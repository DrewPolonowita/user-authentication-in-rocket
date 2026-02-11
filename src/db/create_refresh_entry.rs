use sqlx::PgPool;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RefreshToken {
    pub user_id: i64,
    pub refresh_token: String
}

/// Creates a refresh token entry is a PostgreSQL database; this function does not check the input
///
/// # Arguments
/// - `args`: A RefreshToken struct containing the user_id and the refresh token
/// - `pool`: A reference to a PgPool which is required to connect to SQL databases
///
/// # Returns
/// A Result enum with no data or an sql::error::Error enum if the operation is not successful
pub async fn create_refresh_entry(args: RefreshToken, pool: &PgPool) -> Result<(),  sqlx::error::Error> {
    let _ = sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (user_id, refresh_token)
            VALUES ($1, $2)
            ON CONFLICT (user_id)
            DO UPDATE SET
                refresh_token = $2
            "#,
            args.user_id,
            args.refresh_token,
            )
        .execute(pool)
        .await?;

    Ok(())
}