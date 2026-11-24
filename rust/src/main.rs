use std::sync::Arc;

use axum::Router;
use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

mod api;
mod cache;
mod config;
mod error;
mod mail;
mod migrate;
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
            let direction = args
                .get(2)
                .map(|value| value.parse::<migrate::Direction>())
                .transpose()?
                .unwrap_or(migrate::Direction::Up);
            if direction == migrate::Direction::Down
                && std::env::var("FBOARD_ALLOW_DESTRUCTIVE").as_deref() != Ok("1")
            {
                anyhow::bail!("migrate down is destructive; set FBOARD_ALLOW_DESTRUCTIVE=1 to confirm");
            }
            let pool = migrate::connect(&cfg.database_url).await?;
            migrate::run(&pool, direction).await?;
            pool.close().await;
        }
        "serve" => serve(cfg).await?,
        other => {
            eprintln!("unknown command: {other}");
            eprintln!("usage: fboard [serve|migrate [up|down|status]]");
            std::process::exit(2);
        }
    }
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

    let redis = match cfg.redis_url.clone() {
        Some(url) => match ws_redis::RedisBridge::connect(&url).await {
            Ok(bridge) => Some(bridge),
            Err(err) => {
                tracing::warn!(error = %err, "Redis unavailable; using single-instance behaviour");
                None
            }
        },
        None => None,
    };

    let state = Arc::new(AppState {
        db: pool,
        settings: tokio::sync::RwLock::new(loaded),
        cache: cache::MemoryCache::default(),
        plugin: plugin.clone(),
        ws: std::sync::Arc::new(ws::WsRegistry::default()),
        redis: redis.clone(),
        started_at: util::now(),
        cfg: cfg.clone(),
    });

    if let Some(bridge) = redis {
        state.ws.set_bridge(bridge.clone());
        tokio::spawn(bridge.run(state.clone()));
        tracing::info!("Redis coordination enabled");
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
