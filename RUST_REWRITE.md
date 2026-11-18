# Fboard Rust 重构基线

本仓库是 Fboard 的 Rust 独立实现，Laravel/PHP 代码已移除：

- `rust/`：Fboard 核心 API（axum + sqlx/MySQL），兼容现有 `v2_*` schema 与 admin SPA API。
- `rust/src/plugin/`：插件门面、Rust 内置插件与 Lua 运行时。
- `plugins/`：用户 Lua 插件目录（`plugin.json` + `init.lua`）。

## 本次交付

### 已完成并在 MariaDB 11 上验证

- Sanctum 兼容认证：注册、登录、找回密码、邮件验证码、快速登录、token2Login。
- 用户中心：信息、改密、重置订阅密钥、订阅信息、流量日志、邀请、公告、知识库、工单、提现、礼品卡基础兑换。
- 套餐 / 订单 / 支付：
  - 多周期套餐 `prices` 解析与价格（分）计算；
  - 优惠券、VIP 折扣、余额抵扣、升级折抵、邀请返佣字段；
  - 免费订单直接开通；
  - 内置支付插件：`EPay`、`AlipayF2F`、`BTCPay`、`CoinPayments`、`Coinbase`、`MGate`；签名/验签与幂等入账已实现。
- 订阅：
  - 按权限组、流量、可见性筛选节点；虚拟节点继承父节点；
  - 动态端口、倍率、Shadowsocks 2022 密钥、REALITY 公钥规范化；
  - 内置输出插件渲染 General / Shadowsocks / Clash / Clash Meta / SingBox / Surge / Surfboard / Loon / QuantumultX / Stash / Shadowrocket；
  - 未移植的客户端格式回退到 General URI；
  - 订阅响应头 `subscription-userinfo` 等。
- 节点 / 机器 REST：
  - `handshake`、`config`、`user`、`push`、`report`、`alive`、`alivelist`、`status`；
  - server token 与 machine token 两种鉴权；
  - 流量写回用户与日统计，机器负载快照与历史。
- 管理端核心 CRUD：
  - 配置、邮件模板（列表/详情/保存/重置/测试）、套餐、权限组、路由、网络/证书模板、节点、协议定义、机器；
  - 用户、订单、支付、公告、工单、提现、优惠券、礼品卡模板/兑换码、知识库；
  - 统计概览、订单统计、流量排行、审计日志、流量重置日志；
  - 插件列表、启停、配置、动作、UI 扩展、上传。
- 静态托管：`/{secure_path}` 注入 `window.settings` 与 `/assets/admin/locales/*.js`；提供 `/settings.js`、`/settings.local.js`、`/locales/*`、`/images/*`、`/assets/admin`、`/plugins/{code}/...`。
- 订阅路径按 `subscribe_path` 设置注册（修复此前把动态段当作字面量、误吞 `/locales`、`/images` 请求的问题）。

### 明确未完成

| 区域 | 状态 | 说明 |
|------|------|------|
| WebSocket 实时同步 | 已实现（单实例 + 可选 Redis） | 机器/节点握手、全量同步、设备广播、内核操作与日志；Redis pub/sub 跨实例下发/同步/ack |
| 管理端统计排行/队列面板 | 已实现 | 统计接口返回真实聚合；队列面板返回 Horizon 兼容结构（Rust 核心不跑 Laravel 队列） |
| 机器升级/重启/日志 | 已实现 | 通过 WebSocket 下发内核启停/重启/升级并回传 ack/日志；离线机器返回明确错误 |
| 礼品卡复杂规则 | 已实现 | 余额/流量/设备/有效期/套餐/重置包、盲盒、节日加成、邀请返利、使用限制 |
| Telegram 功能插件 | 已实现 | Rust 内置 webhook、命令、工单与支付通知；已实测 |
| Lua 插件系统 | 已实现 | `plugins/<code>/plugin.json` + `init.lua`；支持启停/配置/UI/action/上传，Rust 直接加载 |
| 插件隔离 | 已实现 | 受限 Lua 状态（无 os/io/package/debug）、64MB 内存上限、HTTP 权限白名单 |
| 容器镜像 | 已实现 | `rust/Dockerfile` 多阶段构建 + `rust/docker-compose.example.yml`（MariaDB/Redis） |
| 支付插件 | 已实现 | EPay / AlipayF2F / BTCPay / CoinPayments / Coinbase / MGate |
| 邮件模板 | 已实现 | 内置 `resources/views/mail/default/*.html` 渲染、DB 覆盖、占位符默认值、邮件日志 |
| 到期/流量提醒 | 已实现 | `remind_mail_enable` 开启后后台每 30 分钟检查，24 小时内去重 |
| 后台高级节点批量替换 | 已实现 | 批量删除/重置/排序/字段替换/批量更新均已实现 |
| admin SPA 逐页回归 | 已完成基础回归 | Playwright 跑通 21 个页面与主要弹窗，0 控制台/网络错误；新增页面需继续覆盖 |

## 运行

```bash
cp rust/.env.example .env.rust
set -a && . ./.env.rust && set +a

cargo run --manifest-path rust/Cargo.toml -- migrate
cargo run --manifest-path rust/Cargo.toml -- serve
```

## 验收脚本（手动）

```bash
# 登录
curl -s -X POST localhost:7001/api/v1/passport/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"admin@example.com","password":"admin12345"}'

# 套餐 / 订阅
curl -s localhost:7001/api/v1/guest/plan/fetch
curl -s localhost:7001/s/<token> -A 'v2rayN/6.31' | base64 -d
curl -s localhost:7001/s/<token> -A 'Clash/1.18'
curl -s localhost:7001/s/<token> -A 'sing-box/1.10'

# 节点
curl -s 'localhost:7001/api/v2/server/handshake?token=<server_token>&node_id=1'
curl -s 'localhost:7001/api/v2/server/user?token=<server_token>&node_id=1'
curl -s -X POST 'localhost:7001/api/v2/server/push?token=<server_token>&node_id=1' \
  -H 'Content-Type: application/json' -d '{"1":[1024,2048]}'
```

## 下一步建议

1. 多实例部署已支持 Redis pub/sub；继续补充 Redis 断线重连与远程节点 TTL 监控指标。
2. 完善 Lua 插件 SDK/示例，并补充 HTTP、加密与订阅输出的兼容性测试。
3. 继续用 admin SPA 覆盖新增页面与写操作，补充 Lua 插件上传/启用流程回归。
4. 增加 Rust 集成测试（testcontainers / MariaDB）与 schema 差异校验。
5. 在 CI 中构建并发布 Rust 镜像；补充容器启动时自动 `migrate` 的可选开关。
