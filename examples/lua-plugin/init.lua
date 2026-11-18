-- Fboard Lua plugin example.
-- Copy this directory to plugins/demo and enable it from the admin panel.

function handle(method, params)
  if method == "action.execute" then
    local message = (params._config and params._config.message) or "hello"
    return {
      ok = true,
      action = params.action,
      message = message,
      uuid = fboard.uuid(),
    }
  end

  if method == "hook.filter" then
    -- Hook filters must return { value = ... }.
    return { value = params.value }
  end

  if method == "plugin.ui" then
    return {
      {
        slot = "dashboard.top",
        component = "markdown",
        content = "Demo Lua plugin loaded.",
      },
    }
  end

  return nil
end
