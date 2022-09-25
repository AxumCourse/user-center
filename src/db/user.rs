use sqlx::{Executor, MySqlExecutor, MySqlPool, Row};

use crate::{model, utils, Error, Result};

pub async fn create(
    pool: &MySqlPool,
    username: &str,
    email: &str,
    password: &str,
) -> Result<(model::user::ID, model::email::ID)> {
    let mut tx = pool.begin().await.map_err(Error::from)?;
    let username_is_exists = match is_exists(&mut tx, username).await {
        Ok(exists) => exists,
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(err);
        }
    };
    if username_is_exists {
        return Err(Error::already_exists("用户名已存在"));
    }
    let email_is_exists = match super::email::is_exists(&mut tx, email).await {
        Ok(exists) => exists,
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(err);
        }
    };
    if email_is_exists {
        return Err(Error::already_exists("邮箱已存在"));
    }
    let user_id: model::user::ID = match insert(&mut tx, username, email, password).await {
        Ok(id) => id,
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(err);
        }
    };
    let email_id = match super::email::create(&mut tx, user_id, email, model::mysql::TRUE).await {
        Ok(id) => id,
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(err);
        }
    };
    tx.commit().await.map_err(Error::from)?;
    Ok((user_id, email_id))
}

pub async fn is_exists(tx: impl MySqlExecutor<'_>, username: &str) -> Result<bool> {
    let row = sqlx::query("SELECT COUNT(*) FROM users WHERE username=?")
        .bind(username)
        .fetch_one(tx)
        .await
        .map_err(Error::from)?;
    let count: i64 = row.get(0);
    Ok(count > 0)
}

pub async fn insert(
    tx: impl MySqlExecutor<'_>,
    username: &str,
    email: &str,
    password: &str,
) -> Result<model::user::ID> {
    let hashed_pwd = utils::password::hash(password)?;
    let now = utils::time::now();
    let id = sqlx::query(
        "INSERT INTO users (username,email,password,dateline,status) VALUES (?,?,?,?,?)",
    )
    .bind(username)
    .bind(email)
    .bind(hashed_pwd)
    .bind(now)
    .bind(model::user::STATUS_PENDING)
    .execute(tx)
    .await
    .map_err(Error::from)?
    .last_insert_id();
    Ok(id)
}
pub async fn set_status(
    tx: impl MySqlExecutor<'_>,
    user_id: model::user::ID,
    status: model::user::Status,
) -> Result<bool> {
    let rows_affected = sqlx::query("UPDATE users SET status=? WHERE id=?")
        .bind(status)
        .bind(user_id)
        .execute(tx)
        .await
        .map_err(Error::from)?
        .rows_affected();
    Ok(rows_affected > 0)
}
pub async fn active(
    pool: &MySqlPool,
    user_id: model::user::ID,
    email_id: model::email::ID,
) -> Result<(bool, bool)> {
    let mut tx = pool.begin().await.map_err(Error::from)?;
    let is_user_ok = match set_status(&mut tx, user_id, model::user::STATUS_OK).await {
        Ok(ok) => ok,
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(err);
        }
    };
    let is_email_ok = match super::email::active(&mut tx, user_id, email_id).await {
        Ok(ok) => ok,
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(err);
        }
    };
    tx.commit().await.map_err(Error::from)?;
    Ok((is_user_ok, is_email_ok))
}
