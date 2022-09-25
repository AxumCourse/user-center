use sqlx::{Executor, MySqlExecutor, Row};

use crate::{model, Error, Result};

pub async fn is_exists(tx: impl MySqlExecutor<'_>, email: &str) -> Result<bool> {
    let row = sqlx::query("SELECT COUNT(*) FROM emails WHERE email=?")
        .bind(email)
        .fetch_one(tx)
        .await
        .map_err(Error::from)?;
    let count: i64 = row.get(0);
    Ok(count > 0)
}
pub async fn create(
    tx: impl MySqlExecutor<'_>,
    user_id: model::user::ID,
    email: &str,
    is_primary: crate::model::mysql::Bool,
) -> Result<model::email::ID> {
    let last_insert_id = sqlx::query("INSERT INTO emails(user_id,email,is_primary) VALUES (?,?,?)")
        .bind(user_id)
        .bind(email)
        .bind(is_primary)
        .execute(tx)
        .await
        .map_err(Error::from)?
        .last_insert_id();
    Ok(last_insert_id)
}
pub async fn active(
    tx: impl MySqlExecutor<'_>,
    user_id: model::user::ID,
    email_id: model::email::ID,
) -> Result<bool> {
    let rows_affected = sqlx::query("UPDATE emails SET is_verify=? WHERE id=? AND user_id=?")
        .bind(model::mysql::TRUE)
        .bind(email_id)
        .bind(user_id)
        .execute(tx)
        .await
        .map_err(Error::from)?
        .rows_affected();
    Ok(rows_affected > 0)
}
