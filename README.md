# Fboard-rs

Fboard 面板的 Rust 重构版：保留 `v2_*` 数据库结构和 HTTP API，兼容旧客户端与 admin SPA。

- 核心 API、认证、订单、支付、订阅、节点/机器 WebSocket 均由 Rust 实现。
- 支付、协议定义、订阅输出和 Telegram 以 Rust 内置插件形式提供。
- 第三方插件使用 Lua：`plugins/<code>/plugin.json` + `init.lua`，无需动态库或 PHP。
- 可选 Redis 多实例 WebSocket 广播。

## 目录

```
rust/               Rust 服务（Cargo 工程）
  src/               API、WebSocket、插件运行时
  migrations/        MySQL schema
resources/rules/     默认 Clash/Sing-Box/Surge 模板
resources/views/     邮件模板
plugins/             用户 Lua 插件（默认不提交）
public/assets/admin  可选的 admin SPA 构建产物
```

## 快速开始

```bash
cp rust/.env.example .env.rust
set -a && . ./.env.rust && set +a

cargo run --manifest-path rust/Cargo.toml -- migrate
cargo run --manifest-path rust/Cargo.toml -- migrate status
cargo run --manifest-path rust/Cargo.toml -- serve
```

默认监听 `0.0.0.0:7001`。详细配置、插件 ABI、Docker 与验收说明见 [`rust/README.md`](rust/README.md)。

## 运维端点

`/healthz`、`/readyz`、`/metrics` 分别提供存活、数据库就绪和 Prometheus 基础指标。

## 测试

```bash
cargo test --manifest-path rust/Cargo.toml
```

## 许可

MIT
