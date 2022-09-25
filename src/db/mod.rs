pub mod email;
pub mod user;

#[cfg(test)]
mod tests {
    use sqlx::MySqlPool;
    #[tokio::test]
    async fn test_db_create_user() {
        let dsn = std::env::var("MYSQL_DSN").unwrap();
        let pool = MySqlPool::connect(&dsn).await.unwrap();
        let (user_id, email_id) = super::user::create(&pool, "axum_rs", "team@axum.rs", "axum.rs")
            .await
            .unwrap();
        assert!(user_id > 0);
        assert!(email_id > 0);
    }
    #[tokio::test]
    async fn test_db_active_user() {
        let dsn = std::env::var("MYSQL_DSN").unwrap();
        let pool = MySqlPool::connect(&dsn).await.unwrap();
        let (is_user_ok, is_email_ok) = super::user::active(&pool, 1, 1).await.unwrap();
        assert!(is_user_ok);
        assert!(is_email_ok);
    }
}
