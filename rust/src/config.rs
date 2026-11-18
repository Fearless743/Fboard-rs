use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};

/// Runtime configuration. All values can be provided through the environment
/// (`.env` is loaded when present, but real environment variables win).
#[derive(Debug, Clone)]
pub struct Config {
    pub listen: String,
    pub database_url: String,
    pub app_url: String,
    pub app_key: String,
    /// Optional Redis URL for multi-instance WebSocket fan-out.
    pub redis_url: Option<String>,
    /// Lua plugin directories (`plugins`).
    pub plugin_dirs: Vec<PathBuf>,
    /// Directory containing the built admin SPA (`/assets/admin`).
    pub admin_dist: PathBuf,
    pub upload_dir: PathBuf,
    pub log_level: String,
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}


impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env if it exists; ignore failures (production uses real env).
        let _ = dotenvy::dotenv();

        let connection = env_or("DB_CONNECTION", "mysql");
        if connection != "mysql" && connection != "mariadb" {
            anyhow::bail!(
                "Fboard Rust core requires DB_CONNECTION=mysql/mariadb (got `{connection}`). \
                 Set DB_CONNECTION=mysql and DB_* variables, or run `fboard migrate` against MySQL."
            );
        }

        let host = env_or("DB_HOST", "127.0.0.1");
        let port = env_or("DB_PORT", "3306");
        let database = env::var("DB_DATABASE").context("DB_DATABASE is required")?;
        let user = env_or("DB_USERNAME", "root");
        let password = env::var("DB_PASSWORD").unwrap_or_default();

        let database_url = env::var("FBOARD_DATABASE_URL").unwrap_or_else(|_| {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                urlencoding::encode(&user),
                urlencoding::encode(&password),
                host,
                port,
                database
            )
        });

        let listen = env_or(
            "FBOARD_LISTEN",
            &format!("0.0.0.0:{}", env_or("PORT", "7001")),
        );
        let app_url = env_or("APP_URL", "http://127.0.0.1:7001");

        let mut plugin_dirs = Vec::new();
        for dir in env_or("FBOARD_PLUGIN_DIRS", "plugins").split(',') {
            let dir = dir.trim();
            if !dir.is_empty() {
                plugin_dirs.push(PathBuf::from(dir));
            }
        }

        let redis_url = env::var("REDIS_URL").ok().filter(|value| !value.trim().is_empty()).or_else(|| {
            let host = env::var("REDIS_HOST").ok().filter(|value| !value.trim().is_empty())?;
            let port = env_or("REDIS_PORT", "6379");
            let db = env_or("REDIS_DB", "0");
            let password = env::var("REDIS_PASSWORD").unwrap_or_default();
            if password.is_empty() {
                Some(format!("redis://{host}:{port}/{db}"))
            } else {
                Some(format!("redis://:{password}@{host}:{port}/{db}"))
            }
        });

        let admin_dist = PathBuf::from(env_or("FBOARD_ADMIN_DIST", "public/assets/admin"));
        let upload_dir = PathBuf::from(env_or("FBOARD_UPLOAD_DIR", "storage/app"));
        let log_level = env_or("FBOARD_LOG", "info");

        Ok(Self {
            listen,
            database_url,
            app_url,
            app_key: env::var("APP_KEY").unwrap_or_default(),
            redis_url,
            plugin_dirs,
            admin_dist,
            upload_dir,
            log_level,
        })
    }
}
