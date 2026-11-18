# Fboard Rust Core

Fboard 面板的 Rust 重构核心。保留 `v2_*` MySQL 表结构、HTTP API、admin SPA 与旧客户端兼容；插件系统使用 **Lua**，不再依赖 PHP、Composer 或动态库。

```
                    ┌──────────────────────────────┐
  admin SPA ───────▶│  fboard (Rust / axum)        │
  client app ──────▶│  · /api/v1 /api/v2           │
  fboard-node ─────▶│  · Sanctum 兼容 token        │
                    │  · 订阅生成 / 订单 / 支付     │
                    └──────────┬───────────────────┘
                               │ 进程内调用
                    ┌──────────▼───────────────────┐
                    │ plugin runtime               │
                    │  · Rust 内置核心插件          │
                    │  · Lua 用户插件 (plugins/)    │
                    └──────────────────────────────┘
```

## 设计基线

- **数据库**：沿用 `v2_*` 表与字段（`rust/migrations/001_schema.sql`）。
- **认证**：Sanctum 兼容。`personal_access_tokens.token = sha256(plain)`，客户端仍收到 `Bearer <plain>`。
- **密码**：兼容 `bcrypt` 以及历史 `md5` / `sha256` / `md5salt` / `sha256salt`。
- **响应**：`{status, message, data, error}`，admin SPA 与旧客户端无需改动。
- **插件**：核心支付/协议/输出/Telegram 编译进二进制；第三方插件是 `plugins/<code>/` 下的 Lua 脚本。
- **实时通道**：内置 `/ws` WebSocket（机器/单节点两种鉴权）；配置 Redis 后支持多实例广播、远程节点同步、设备快照与 ack 聚合。

## 快速开始

```bash
cp rust/.env.example .env.rust
set -a && . ./.env.rust && set +a

# 建表
cargo run --manifest-path rust/Cargo.toml -- migrate

# 启动
cargo run --manifest-path rust/Cargo.toml -- serve
```

默认监听 `0.0.0.0:7001`。admin SPA 产物可选，目录默认 `public/assets/admin`；未构建时 `/{secure_path}` 返回说明页，API 不受影响。

## 环境变量

| 变量 | 说明 |
|------|------|
| `DB_CONNECTION` | 必须是 `mysql` / `mariadb` |
| `DB_HOST` `DB_PORT` `DB_DATABASE` `DB_USERNAME` `DB_PASSWORD` | 数据库连接 |
| `APP_URL` | 订阅、支付回调等生成的站点地址 |
| `APP_KEY` | 用于计算默认 `secure_path`（与旧面板一致） |
| `FBOARD_LISTEN` | 监听地址，默认 `0.0.0.0:7001` |
| `FBOARD_PLUGIN_DIRS` | Lua 插件目录，默认 `plugins`，逗号分隔 |
| `REDIS_URL` / `REDIS_HOST` | 可选，多实例 WebSocket 广播与跨实例设备/ack/日志同步 |
| `FBOARD_ADMIN_DIST` | admin 产物目录，默认 `public/assets/admin` |
| `FBOARD_UPLOAD_DIR` | 上传目录，默认 `storage/app` |

## 内置功能

- **支付**：`EPay`、`AlipayF2F`、`BTCPay`、`CoinPayments`、`Coinbase`、`MGate`，包含签名、异步通知验签与幂等入账。
- **协议定义**：Shadowsocks、VMess、VLESS、Trojan、Hysteria、TUIC、AnyTLS、SOCKS、Naive、HTTP、Mieru、ShadowQUIC、Sudoku、TrustTunnel、MASQUE 等节点字段与 node config builder。
- **订阅输出**：General、Shadowsocks (SIP008)、Clash、Clash Meta、Sing-Box、Surge、Surfboard、Loon、Quantumult X、Stash、Shadowrocket。
- **Telegram**：`/start`、`/bind`、`/traffic`、`/getlatesturl`、`/unbind`、工单与支付通知。
- **管理端**：配置、套餐、用户、订单、支付、节点、机器、协议定义、插件、统计、公告、工单、优惠券、知识库、提现、流量重置。

## Lua 插件

目录结构：

```
plugins/
  my-plugin/
    plugin.json
    init.lua
    README.md      # 可选
    public/        # 可选静态文件
```

`plugin.json`：

```json
{
  "code": "my_plugin",
  "name": "My Plugin",
  "type": "feature",
  "version": "1.0.0",
  "description": "示例插件",
  "author": "you",
  "entry": "init.lua",
  "methods": [],
  "flags": [],
  "permissions": ["http"],
  "form": {
    "enabled": {
      "type": "boolean",
      "default": true,
      "label": "启用"
    }
  }
}
```

- `type`：`payment`、`protocol`、`output`、`feature`。
- `methods`：支付插件可接受的 `v2_payment.payment` 名称。
- `flags`：自定义订阅输出匹配的客户端 UA 关键字。
- `permissions`：`http` 允许 `fboard.http.request`；`*` 允许全部 host 能力。
- `form`：管理端配置表单，格式与旧面板一致。

`init.lua` 必须定义：

```lua
function handle(method, params)
  if method == "action.execute" then
    return { ok = true, action = params.action }
  end
  return nil
end
```

宿主会在 `params` 中注入 `_config`、`_settings`、`_app_url`。常用方法：

- `payment.form` / `payment.pay` / `payment.notify`
- `protocol.definitions`
- `subscribe.render`
- `server.config`
- `hook.filter`（返回 `{ value = ... }`）
- `plugin.ui` / `plugin.ui.nav` / `action.execute`
- `telegram.webhook` / `telegram.ticket_notify` / `telegram.payment_notify`

### 宿主 API

```lua
fboard.json.encode(value)
fboard.json.decode(text)

fboard.crypto.md5(text)
fboard.crypto.sha256(text)
fboard.crypto.hmac_sha256(key, data)
fboard.crypto.base64_encode(text)
fboard.crypto.base64_decode(text)
fboard.crypto.url_encode(text)

fboard.now()
fboard.uuid()
fboard.log("info", "message")

-- 需要 plugin.json permissions 包含 "http"
local resp = fboard.http.request({
  method = "POST",
  url = "https://example.com/api",
  headers = { ["Content-Type"] = "application/json" },
  body = fboard.json.encode({ hello = "world" }),
  timeout = 15,
})
-- resp = { status = 200, body = "...", headers = {...} }
```

Lua 状态仅加载 `string` / `table` / `math` / `utf8` / `coroutine`，不提供 `os`、`io`、`package`、`debug`；内存上限 64MB。插件在独立状态中执行，调用放入 blocking 线程，避免阻塞 Tokio worker。

插件启用/停用、配置和上传在管理端完成；上传 zip 允许 `.lua`、`.json`、静态资源，解压后自动重新扫描。内置插件不可删除。

## 容器部署

```bash
# 可选：先构建 admin SPA，产物写入 public/assets/admin
cd ../admin && FBOARD_PUBLIC_ASSETS_DIR=../Fboard-rs/public/assets/admin bun run build
cd ../Fboard-rs

docker build -f rust/Dockerfile -t fboard-rs .
docker compose -f rust/docker-compose.example.yml up -d

# 首次启动前建表
docker compose -f rust/docker-compose.example.yml run --rm fboard migrate
```

镜像内包含 Rust 二进制、默认订阅/邮件模板；用户 Lua 插件挂载到 `/app/plugins`。默认监听 `7001`。

## 测试

```bash
cd rust
cargo test
cargo check
```

测试覆盖 token/密码/重置周期、内置 Clash/SingBox/EPay 等纯函数，以及 Lua 插件加载、JSON 调用与沙箱限制。

## 已知差异 / 后续工作

- WebSocket 默认单进程内存注册表；配置 `REDIS_URL` 后启用多实例广播。
- 管理端部分聚合统计（排行、队列、Horizon 面板）为简化实现。
- 多实例下的流量重置/提醒任务尚未做分布式锁，生产多副本部署前需补齐。
- 第三方 Lua 插件默认只提供 JSON/HTTP/加密等宿主能力；需要数据库访问的插件应通过宿主新增窄 API，而不是直接开库。
- 生产部署前建议补充 HTTP+数据库集成测试、支付沙箱回归、压测与健康检查。
