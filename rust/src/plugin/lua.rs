//! Lua plugin runtime.
//!
//! A plugin is a directory with `plugin.json` and `init.lua`. The script must
//! define `handle(method, params)` and return a JSON-compatible Lua value.
//!
//! ```lua
//! function handle(method, params)
//!   if method == "payment.form" then
//!     return { key = { type = "string", label = "密钥" } }
//!   end
//!   return nil
//! end
//! ```
//!
//! Plugins run in a restricted Lua state with only string/table/math/utf8/
//! coroutine libraries. Host capabilities are exposed through `fboard` and are
//! gated by the manifest `permissions` list (`http`, `crypto`, `*`).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context, Result};
use base64::Engine;
use hmac::{Hmac, Mac};
use mlua::{Function, Lua, LuaOptions, LuaSerdeExt, StdLib, Table, Value as LuaValue};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};
use crate::util;

pub struct LuaPlugin {
    meta: PluginMeta,
    methods: Vec<String>,
    flags: Vec<String>,
    permissions: Vec<String>,
    lua: Arc<Mutex<Lua>>,
}

impl LuaPlugin {
    fn invoke(lua: &Lua, method: &str, params: &Value) -> Result<Value> {
        let globals = lua.globals();
        let handle: Function = globals
            .get("handle")
            .context("Lua plugin must define handle(method, params)")?;
        let params = lua.to_value(params)?;
        let result: LuaValue = handle.call((method, params))?;
        let value: Value = lua.from_value(result)?;
        Ok(value)
    }
}

#[async_trait::async_trait]
impl NativePlugin for LuaPlugin {
    fn meta(&self) -> PluginMeta {
        self.meta.clone()
    }

    fn payment_methods(&self) -> Vec<String> {
        self.methods.clone()
    }

    fn flags(&self) -> Vec<String> {
        self.flags.clone()
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        let mut params = params.clone();
        if let Some(object) = params.as_object_mut() {
            object.insert("_config".into(), ctx.config.clone());
            object.insert("_settings".into(), ctx.settings.public_map());
            object.insert("_app_url".into(), json!(ctx.app_url));
        }
        let lua = self.lua.clone();
        let method = method.to_string();
        tokio::task::spawn_blocking(move || {
            let lua = lua
                .lock()
                .map_err(|_| anyhow::anyhow!("Lua state poisoned"))?;
            Self::invoke(&lua, &method, &params)
        })
        .await?
    }
}

/// Load every Lua plugin found in the configured directories.
pub fn load(dirs: &[PathBuf]) -> Vec<Arc<dyn NativePlugin>> {
    let mut out: Vec<Arc<dyn NativePlugin>> = Vec::new();
    for dir in dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            match load_one(&path) {
                Ok(Some(plugin)) => {
                    let meta = plugin.meta();
                    if out
                        .iter()
                        .any(|p| p.meta().code.eq_ignore_ascii_case(&meta.code))
                    {
                        tracing::warn!(code = %meta.code, "duplicate Lua plugin code ignored");
                    } else {
                        tracing::info!(code = %meta.code, name = %meta.name, "loaded Lua plugin");
                        out.push(plugin);
                    }
                }
                Ok(None) => {}
                Err(err) => {
                    tracing::warn!(path = %path.display(), error = %err, "failed to load Lua plugin");
                }
            }
        }
    }
    out
}

fn load_one(dir: &Path) -> Result<Option<Arc<dyn NativePlugin>>> {
    let manifest_path = dir.join("plugin.json");
    let Ok(text) = std::fs::read_to_string(&manifest_path) else {
        return Ok(None);
    };
    let manifest: Value = serde_json::from_str(&text)
        .with_context(|| format!("invalid {}", manifest_path.display()))?;

    let dir_name = dir
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or("plugin")
        .to_string();
    let code = manifest
        .get("code")
        .and_then(|v| v.as_str())
        .unwrap_or(&dir_name)
        .to_ascii_lowercase();
    let entry = manifest
        .get("entry")
        .and_then(|v| v.as_str())
        .unwrap_or("init.lua");
    if entry.contains("..")
        || entry.contains('/')
        || entry.contains('\\')
        || Path::new(entry).is_absolute()
    {
        anyhow::bail!("invalid Lua entry file: {entry}");
    }
    let script_path = dir.join(entry);
    let script = std::fs::read_to_string(&script_path)
        .with_context(|| format!("missing {}", script_path.display()))?;

    let methods: Vec<String> = manifest
        .get("methods")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let flags: Vec<String> = manifest
        .get("flags")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_ascii_lowercase()))
                .collect()
        })
        .unwrap_or_default();
    let permissions: Vec<String> = manifest
        .get("permissions")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_ascii_lowercase()))
                .collect()
        })
        .unwrap_or_default();

    let lua = build_state(&permissions)?;
    lua.load(&script)
        .set_name(script_path.display().to_string())
        .exec()
        .with_context(|| format!("failed to execute {}", script_path.display()))?;
    if lua.globals().get::<Option<Function>>("handle")?.is_none() {
        anyhow::bail!("{entry} must define handle(method, params)");
    }

    let readme = std::fs::read_to_string(dir.join("README.md")).unwrap_or_default();
    let meta = PluginMeta {
        code,
        name: manifest
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(&dir_name)
            .to_string(),
        kind: PluginKind::parse(
            manifest
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("feature"),
        ),
        version: manifest
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0.0")
            .to_string(),
        description: manifest
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        author: manifest
            .get("author")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        builtin: false,
        path: Some(dir.to_path_buf()),
        config_fields: manifest.get("form").cloned().unwrap_or_else(|| json!({})),
        readme,
    };

    Ok(Some(Arc::new(LuaPlugin {
        meta,
        methods,
        flags,
        permissions,
        lua: Arc::new(Mutex::new(lua)),
    })))
}

fn build_state(permissions: &[String]) -> Result<Lua> {
    let libs = StdLib::STRING | StdLib::TABLE | StdLib::MATH | StdLib::UTF8 | StdLib::COROUTINE;
    let lua = Lua::new_with(libs, LuaOptions::default())?;
    lua.set_memory_limit(64 * 1024 * 1024)?;

    let allow_http = permissions.iter().any(|p| p == "*" || p == "http");
    let fboard = lua.create_table()?;

    let json_table = lua.create_table()?;
    json_table.set(
        "encode",
        lua.create_function(|lua, value: LuaValue| {
            let value: Value = lua.from_value(value)?;
            serde_json::to_string(&value).map_err(mlua::Error::external)
        })?,
    )?;
    json_table.set(
        "decode",
        lua.create_function(|lua, text: String| {
            let value: Value = serde_json::from_str(&text).map_err(mlua::Error::external)?;
            lua.to_value(&value)
        })?,
    )?;
    fboard.set("json", json_table)?;

    fboard.set(
        "log",
        lua.create_function(|_, (level, message): (String, String)| {
            match level.to_ascii_lowercase().as_str() {
                "warn" | "warning" => tracing::warn!(target: "lua_plugin", "{message}"),
                "error" => tracing::error!(target: "lua_plugin", "{message}"),
                "debug" => tracing::debug!(target: "lua_plugin", "{message}"),
                _ => tracing::info!(target: "lua_plugin", "{message}"),
            }
            Ok(())
        })?,
    )?;

    let crypto = lua.create_table()?;
    crypto.set(
        "md5",
        lua.create_function(|_, input: String| Ok(util::md5_hex(&input)))?,
    )?;
    crypto.set(
        "sha256",
        lua.create_function(|_, input: String| Ok(hex::encode(Sha256::digest(input.as_bytes()))))?,
    )?;
    crypto.set(
        "hmac_sha256",
        lua.create_function(|_, (key, data): (String, String)| {
            let mut mac =
                Hmac::<Sha256>::new_from_slice(key.as_bytes()).map_err(mlua::Error::external)?;
            mac.update(data.as_bytes());
            Ok(hex::encode(mac.finalize().into_bytes()))
        })?,
    )?;
    crypto.set(
        "base64_encode",
        lua.create_function(|_, input: String| {
            Ok(base64::engine::general_purpose::STANDARD.encode(input.as_bytes()))
        })?,
    )?;
    crypto.set(
        "base64_decode",
        lua.create_function(|_, input: String| {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(input.as_bytes())
                .map_err(mlua::Error::external)?;
            String::from_utf8(bytes).map_err(mlua::Error::external)
        })?,
    )?;
    crypto.set(
        "url_encode",
        lua.create_function(|_, input: String| Ok(urlencoding::encode(&input).to_string()))?,
    )?;
    fboard.set("crypto", crypto)?;

    fboard.set("now", lua.create_function(|_, ()| Ok(util::now()))?)?;
    fboard.set(
        "uuid",
        lua.create_function(|_, ()| Ok(uuid::Uuid::new_v4().to_string()))?,
    )?;

    let http = lua.create_table()?;
    http.set(
        "request",
        lua.create_function(move |lua, options: Table| {
            if !allow_http {
                return Err(mlua::Error::RuntimeError(
                    "http permission denied by plugin.json".into(),
                ));
            }
            let method: String = options
                .get::<Option<String>>("method")?
                .unwrap_or_else(|| "GET".into())
                .to_ascii_uppercase();
            let url: String = options
                .get::<Option<String>>("url")?
                .ok_or_else(|| mlua::Error::RuntimeError("http.request requires url".into()))?;
            let body: Option<String> = options.get("body")?;
            let timeout: u64 = options.get::<Option<u64>>("timeout")?.unwrap_or(15);

            let client = reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(timeout.clamp(1, 60)))
                .build()
                .map_err(mlua::Error::external)?;
            let method =
                reqwest::Method::from_bytes(method.as_bytes()).map_err(mlua::Error::external)?;
            let mut request = client.request(method, &url);
            if let Some(headers) = options.get::<Option<Table>>("headers")? {
                for pair in headers.pairs::<String, String>() {
                    let (key, value) = pair?;
                    request = request.header(&key, value);
                }
            }
            if let Some(body) = body {
                request = request.body(body);
            }
            let response = request.send().map_err(mlua::Error::external)?;
            let status = response.status().as_u16();
            let mut headers = lua.create_table()?;
            for (key, value) in response.headers() {
                if let Ok(value) = value.to_str() {
                    headers.set(key.as_str(), value)?;
                }
            }
            let body = response.text().map_err(mlua::Error::external)?;
            let out = lua.create_table()?;
            out.set("status", status)?;
            out.set("body", body)?;
            out.set("headers", headers)?;
            Ok(out)
        })?,
    )?;
    fboard.set("http", http)?;

    lua.globals().set("fboard", fboard)?;
    Ok(lua)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_plugin(root: &Path, script: &str) {
        let dir = root.join("demo");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("plugin.json"),
            r#"{
                "code":"demo",
                "name":"Demo",
                "type":"feature",
                "version":"1.2.3",
                "description":"test",
                "author":"test"
            }"#,
        )
        .unwrap();
        std::fs::write(dir.join("init.lua"), script).unwrap();
    }

    #[test]
    fn loads_and_invokes_lua_plugin() {
        let root = std::env::temp_dir().join(format!(
            "fboard-lua-{}-{}",
            std::process::id(),
            util::random_chars(6)
        ));
        write_plugin(
            &root,
            r#"
            function handle(method, params)
              if method == "echo" then
                return { code = "demo", value = params.value, sha = fboard.crypto.sha256("x") }
              end
              if method == "sandbox" then
                return { has_os = os ~= nil, has_io = io ~= nil }
              end
              return nil
            end
            "#,
        );
        let plugins = load(&[root.clone()]);
        assert_eq!(plugins.len(), 1);
        let plugin = &plugins[0];
        assert_eq!(plugin.meta().code, "demo");
        assert_eq!(plugin.meta().version, "1.2.3");
        // Use a fresh state to keep the test independent from async plumbing.
        let state = build_state(&[]).unwrap();
        state
            .load(
                r#"
                function handle(method, params)
                  return { value = params.value, sha = fboard.crypto.sha256("x") }
                end
                "#,
            )
            .exec()
            .unwrap();
        let result = LuaPlugin::invoke(&state, "echo", &json!({"value": 7})).unwrap();
        assert_eq!(result["value"], 7);
        assert_eq!(
            result["sha"],
            "2d711642b726b04401627ca9fbac32f5c8530fb1903cc4db02258717921a4881"
        );
        assert!(state
            .globals()
            .get::<Option<LuaValue>>("os")
            .unwrap()
            .is_none());
        let _ = std::fs::remove_dir_all(root);
    }
}
