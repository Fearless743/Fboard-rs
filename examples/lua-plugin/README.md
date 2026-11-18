# Demo Lua Plugin

```bash
cp -r examples/lua-plugin plugins/demo
```

Then open the admin plugin page, enable `Demo Lua Plugin`, and trigger the
`demo` action. The plugin runs in a restricted Lua state; add `"http"` to
`permissions` only if it needs `fboard.http.request`.

See `rust/README.md` for the full host API.
