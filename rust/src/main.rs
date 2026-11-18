use std::str::FromStr;
use std::sync::Arc;

use axum::Router;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Connection;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

mod api;
mod cache;
mod config;
mod error;
mod mail;
mod models;
mod plugin;
mod remind;
mod response;
mod routes;
mod settings;
mod state;
mod traffic;
mod util;
mod ws;
mod ws_redis;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_env()?;
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(cfg.log_level.clone()));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("serve");

    match command {
        "migrate" => {
            migrate(&cfg).await?;
            println!("schema applied");
        }
        "serve" => serve(cfg).await?,
        other => {
            eprintln!("unknown command: {other}");
            eprintln!("usage: fboard [serve|migrate]");
            std::process::exit(2);
        }
    }
    Ok(())
}

async fn migrate(cfg: &Config) -> anyhow::Result<()> {
    // Ensure the database exists (MySQL/MariaDB must already be running).
    if let Ok(options) = sqlx::mysql::MySqlConnectOptions::from_str(&cfg.database_url) {
        let database = options.get_database().unwrap_or("").to_string();
        if !database.is_empty() {
            let server_url = cfg
                .database_url
                .rsplitn(2, '/')
                .nth(1)
                .map(|prefix| format!("{prefix}/"))
                .unwrap_or_else(|| cfg.database_url.clone());
            if let Ok(mut conn) = sqlx::MySqlConnection::connect(&server_url).await {
                let _ = sqlx::query(&format!(
                    "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci",
                    database.replace('`', "")
                ))
                .execute(&mut conn)
                .await;
            }
        }
    }

    let pool = MySqlPoolOptions::new()
        .max_connections(2)
        .connect(&cfg.database_url)
        .await?;
    sqlx::raw_sql(include_str!("../migrations/001_schema.sql"))
        .execute(&pool)
        .await?;
    pool.close().await;
    Ok(())
}

async fn serve(cfg: Config) -> anyhow::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(&cfg.database_url)
        .await?;

    let loaded = settings::Settings::load(&pool).await;
    let plugin = plugin::PluginRegistry::load(&cfg);

    let state = Arc::new(AppState {
        db: pool,
        settings: tokio::sync::RwLock::new(loaded),
        cache: cache::MemoryCache::default(),
        plugin: plugin.clone(),
        ws: std::sync::Arc::new(ws::WsRegistry::default()),
        started_at: util::now(),
        cfg: cfg.clone(),
    });

    if let Some(url) = cfg.redis_url.clone() {
        match ws_redis::RedisBridge::connect(&url).await {
            Ok(bridge) => {
                state.ws.set_bridge(bridge.clone());
                tokio::spawn(bridge.run(state.clone()));
                tracing::info!("Redis WebSocket bridge enabled");
            }
            Err(err) => tracing::warn!(error = %err, "Redis WebSocket bridge unavailable"),
        }
    }

    let router: Router = routes::build(state.clone());

    // Traffic reset scheduler (monthly / first-day / yearly plans).
    tokio::spawn(traffic::scheduler(state.clone()));
    // Expiry / traffic reminder mails (opt-in via remind_mail_enable).
    tokio::spawn(remind::scheduler(state.clone()));

    let listener = TcpListener::bind(&cfg.listen).await?;
    tracing::info!("Fboard Rust core listening on {}", cfg.listen);
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        let mut signal =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).ok();
        if let Some(signal) = signal.as_mut() {
            signal.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutting down");
}
