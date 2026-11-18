#!/usr/bin/env bash
# Minimal smoke test against a running Rust core.
#
# Usage:
#   BASE=http://127.0.0.1:7001 \
#   EMAIL=admin@example.com PASSWORD=admin12345 \
#   SERVER_TOKEN=... SUBSCRIBE_TOKEN=... \
#   MACHINE_ID=1 MACHINE_TOKEN=... ./rust/scripts/smoke.sh
set -euo pipefail

BASE="${BASE:-http://127.0.0.1:7001}"
EMAIL="${EMAIL:-admin@example.com}"
PASSWORD="${PASSWORD:-admin12345}"
SERVER_TOKEN="${SERVER_TOKEN:-}"
SUBSCRIBE_TOKEN="${SUBSCRIBE_TOKEN:-}"
MACHINE_ID="${MACHINE_ID:-0}"
MACHINE_TOKEN="${MACHINE_TOKEN:-}"

say() { printf '\n== %s\n' "$1"; }

say "guest config"
curl -fsS "$BASE/api/v1/guest/comm/config" | head -c 300; echo

say "login"
LOGIN=$(curl -fsS -X POST "$BASE/api/v1/passport/auth/login" \
  -H 'Content-Type: application/json' \
  -d "{\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\"}")
AUTH=$(printf '%s' "$LOGIN" | python3 -c 'import sys,json;print(json.load(sys.stdin)["data"]["auth_data"])')
printf '%s\n' "$LOGIN" | head -c 300; echo

say "user info"
curl -fsS "$BASE/api/v1/user/info" -H "Authorization: $AUTH" | head -c 300; echo

say "plans"
curl -fsS "$BASE/api/v1/guest/plan/fetch" | head -c 300; echo

if [ -n "$SUBSCRIBE_TOKEN" ]; then
  say "subscription (general)"
  curl -fsS "$BASE/s/$SUBSCRIBE_TOKEN" -A 'v2rayN/6.31' | base64 -d | head -3
  say "subscription (clash)"
  curl -fsS "$BASE/s/$SUBSCRIBE_TOKEN" -A 'Clash/1.18' | head -3
  say "subscription (sing-box)"
  curl -fsS "$BASE/s/$SUBSCRIBE_TOKEN" -A 'sing-box/1.10' | head -c 200; echo
fi

if [ -n "$SERVER_TOKEN" ]; then
  say "node handshake"
  curl -fsS "$BASE/api/v2/server/handshake?token=$SERVER_TOKEN&node_id=1"; echo
  say "node config"
  curl -fsS "$BASE/api/v2/server/config?token=$SERVER_TOKEN&node_id=1" | head -c 300; echo
  say "node users"
  curl -fsS "$BASE/api/v2/server/user?token=$SERVER_TOKEN&node_id=1" | head -c 300; echo
fi

if [ "$MACHINE_ID" != "0" ] && [ -n "$MACHINE_TOKEN" ]; then
  say "machine nodes"
  curl -fsS -X POST "$BASE/api/v2/server/machine/nodes" \
    -H 'Content-Type: application/json' \
    -d "{\"machine_id\":$MACHINE_ID,\"token\":\"$MACHINE_TOKEN\"}"; echo
fi

say "admin core"
for path in \
  "/api/v2/admin/config/fetch?key=app_name" \
  "/api/v2/admin/plan/fetch" \
  "/api/v2/admin/server/manage/getNodes" \
  "/api/v2/admin/plugin/getPlugins" \
  "/api/v2/admin/stat/getStats"; do
  printf '%-52s ' "$path"
  curl -fsS "$BASE$path" -H "Authorization: $AUTH" >/dev/null && echo OK || echo FAIL
done

say "done"
