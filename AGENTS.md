# Fboard-rs — AGENTS.md

Rust 版 Fboard 面板。仓库根目录即部署仓库，Laravel/PHP 代码已移除。

## 命令

```bash
cargo test --manifest-path rust/Cargo.toml
cargo run --manifest-path rust/Cargo.toml -- migrate
cargo run --manifest-path rust/Cargo.toml -- migrate status
cargo run --manifest-path rust/Cargo.toml -- serve
```

- 仅支持 MySQL/MariaDB，`DB_CONNECTION=mysql`。
- 迁移可重复执行；`migrate down` 需 `FBOARD_ALLOW_DESTRUCTIVE=1`。
- 默认监听 `7001`，环境变量见 `rust/.env.example`。
- 测试需要 Rust stable；Lua 运行时使用 `mlua` vendored 构建。

## 结构

| 路径 | 说明 |
|------|------|
| `rust/src/api/` | HTTP API（v1/v2、admin） |
| `rust/src/ws.rs` / `ws_redis.rs` | 节点/机器 WebSocket 与多实例桥接 |
| `rust/src/plugin/` | 插件门面、内置插件、Lua 运行时 |
| `rust/migrations/` | MySQL schema（`001_schema.sql`） |
| `resources/rules/` | 默认订阅模板 |
| `resources/views/mail/` | 邮件模板（HTML + `{{var}}`） |
| `plugins/` | 用户 Lua 插件，运行时扫描 |
| `public/assets/admin/` | 可选 admin SPA 产物 |

## 插件

- 核心能力不依赖外部插件；第三方扩展用 Lua。
- 插件目录：`plugins/<code>/plugin.json` + `init.lua`。
- `init.lua` 必须定义 `handle(method, params)`，返回 Lua table/string/number/bool。
- 能力通过 `fboard` 表提供：`json`、`crypto`、`http`（需 `permissions: ["http"]`）、`log`、`now`、`uuid`。
- 不要引入原生动态库；插件只允许 Lua 源码和静态资源。

## 约定

- 保持 `v2_*` 表与 API 响应兼容，admin SPA 不改。
- 不在根目录恢复 Laravel、Composer、PHP 或插件 sidecar。
- 修改 schema 时同时更新 `rust/migrations/001_schema.sql`、对应的 `.down.sql` 与 `RUST_REWRITE.md`。
- 提交前运行 `cargo test`；涉及插件时额外跑一次 Lua 示例。
- 多实例部署必须配置 `REDIS_URL`，否则定时任务和配置失效只在单实例生效。
