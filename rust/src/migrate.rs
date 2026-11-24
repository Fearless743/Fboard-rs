//! Versioned, repeatable schema migrations.

use std::str::FromStr;

use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{Connection, MySqlPool};

pub struct Migration {
    pub version: &'static str,
    pub up: &'static str,
    pub down: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[Migration {
    version: "001_schema",
    up: include_str!("../migrations/001_schema.sql"),
    down: include_str!("../migrations/001_schema.down.sql"),
}];

const LEDGER_DDL: &str = r#"
CREATE TABLE IF NOT EXISTS `v2_rust_migrations` (
  `version` varchar(191) NOT NULL,
  `checksum` varchar(64) NOT NULL,
  `applied_at` bigint NOT NULL,
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Status,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "up" | "" => Ok(Self::Up),
            "down" | "rollback" => Ok(Self::Down),
            "status" | "list" => Ok(Self::Status),
            other => anyhow::bail!("unknown migration direction: {other}"),
        }
    }
}

fn checksum(sql: &str) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(sql.as_bytes()))
}

async fn ensure_database(database_url: &str) -> anyhow::Result<()> {
    let options = MySqlConnectOptions::from_str(database_url)?;
    let database = options.get_database().unwrap_or("").to_string();
    if database.is_empty() {
        return Ok(());
    }
    let server_url = database_url
        .rsplitn(2, '/')
        .nth(1)
        .map(|prefix| format!("{prefix}/"))
        .unwrap_or_else(|| database_url.to_string());
    let mut conn = match sqlx::MySqlConnection::connect(&server_url).await {
        Ok(conn) => conn,
        Err(err) => {
            tracing::warn!(error = %err, "cannot connect to MySQL server to create database");
            return Ok(());
        }
    };
    sqlx::query(&format!(
        "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci",
        database.replace('`', "")
    ))
    .execute(&mut conn)
    .await?;
    Ok(())
}

pub async fn connect(database_url: &str) -> anyhow::Result<MySqlPool> {
    ensure_database(database_url).await?;
    Ok(MySqlPoolOptions::new()
        .max_connections(2)
        .connect(database_url)
        .await?)
}

async fn applied(pool: &MySqlPool, version: &str) -> anyhow::Result<bool> {
    let row: Option<i64> =
        sqlx::query_scalar("SELECT 1 FROM v2_rust_migrations WHERE version = ? LIMIT 1")
            .bind(version)
            .fetch_optional(pool)
            .await?;
    Ok(row.is_some())
}

pub async fn run(pool: &MySqlPool, direction: Direction) -> anyhow::Result<()> {
    sqlx::raw_sql(LEDGER_DDL).execute(pool).await?;
    match direction {
        Direction::Status => {
            for migration in MIGRATIONS {
                let state = if applied(pool, migration.version).await? {
                    "applied"
                } else {
                    "pending"
                };
                println!("{}  {}", migration.version, state);
            }
        }
        Direction::Up => {
            for migration in MIGRATIONS {
                if applied(pool, migration.version).await? {
                    continue;
                }
                sqlx::raw_sql(migration.up).execute(pool).await?;
                sqlx::query(
                    "INSERT INTO v2_rust_migrations (version, checksum, applied_at) VALUES (?, ?, ?) \
                     ON DUPLICATE KEY UPDATE checksum = VALUES(checksum)",
                )
                .bind(migration.version)
                .bind(checksum(migration.up))
                .bind(crate::util::now())
                .execute(pool)
                .await?;
                println!("applied {}", migration.version);
            }
        }
        Direction::Down => {
            for migration in MIGRATIONS.iter().rev() {
                if !applied(pool, migration.version).await? {
                    continue;
                }
                if migration.down.trim().is_empty() {
                    anyhow::bail!("migration {} has no rollback SQL", migration.version);
                }
                sqlx::raw_sql(migration.down).execute(pool).await?;
                sqlx::query("DELETE FROM v2_rust_migrations WHERE version = ?")
                    .bind(migration.version)
                    .execute(pool)
                    .await?;
                println!("rolled back {}", migration.version);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_are_ordered_and_have_down_sql() {
        let versions: Vec<&str> = MIGRATIONS.iter().map(|m| m.version).collect();
        let mut sorted = versions.clone();
        sorted.sort();
        assert_eq!(versions, sorted);
        for migration in MIGRATIONS {
            assert!(!migration.up.trim().is_empty());
            assert!(!migration.down.trim().is_empty());
            assert!(migration.down.to_ascii_lowercase().contains("drop table"));
        }
    }

    #[test]
    fn schema_is_idempotent() {
        assert_eq!(
            MIGRATIONS[0]
                .up
                .matches("CREATE TABLE IF NOT EXISTS")
                .count(),
            MIGRATIONS[0].up.matches("CREATE TABLE").count()
        );
    }

    #[test]
    fn parses_direction() {
        assert_eq!("up".parse::<Direction>().unwrap(), Direction::Up);
        assert_eq!("down".parse::<Direction>().unwrap(), Direction::Down);
        assert_eq!("status".parse::<Direction>().unwrap(), Direction::Status);
        assert!("sideways".parse::<Direction>().is_err());
    }
}
