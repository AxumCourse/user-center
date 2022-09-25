use axum::Router;
use dotenv::dotenv;
use user_center::config;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "user_center=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let cfg = config::Config::from_env().unwrap();

    tracing::info!("web服务监听于 {}", &cfg.web.addr);

    let app = Router::new();
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
