---
description: Run isolated browser tests against the Pulse web app without managing server lifecycle
---

# Command: Browser Test

You are running agent-browser skill end-to-end tests for the Pulse web application.

## CRITICAL: Isolation Requirement

You MUST run in isolation to avoid conflicts with other agents and port/resource conflicts:

1. Generate a random isolation ID: `$RANDOM` or `$(openssl rand -hex 3)`
2. Set a unique port offset: `export PORT_OFFSET=$RANDOM`
3. Use the isolated port for all browser connections
4. Never use default ports (9222, 3000, etc.) - always add your isolation ID

Example:
```bash
export ISOLATION_ID=$(openssl rand -hex 3)
export BROWSER_PORT=$((9222 + ISOLATION_ID % 1000))
export WEB_PORT=$((3000 + ISOLATION_ID % 1000))
```

## Environment

- **App URL**: http://localhost:3000 (adjust with your ISOLATION_PORT_OFFSET)
- **Server Logs**: /tmp/pulse-web.log
- **Test Location**: src/crates/pulse-app/e2e-tests/

## Critical Rules

1. **DO NOT kill or restart the web server** - it is already running and managed externally
2. **DO NOT run `dx serve` or any server start commands** - the server is already up
3. The app is available at http://localhost:3000 - navigate there directly
4. **ALWAYS use isolation** - append your ISOLATION_ID to any ports or resource names

## Reading Server Logs

If you need to debug issues or check server-side behavior:

```bash
# View recent logs
tail -100 /tmp/pulse-web.log

# Follow logs in real-time
tail -f /tmp/pulse-web.log

# Search for errors
grep -i error /tmp/pulse-web.log
```

## Running Tests

Use agent-browser skill MCP tools to:
1. Navigate to http://localhost:3000 (use your isolated port)
2. Take snapshots with `browser_snapshot` (preferred over screenshots)
3. Interact with elements using refs from snapshots
4. Verify expected behavior

## Test Flow

1. Generate isolation ID first: `export ISOLATION_ID=$(openssl rand -hex 3)`
2. Start by navigating to the app: `browser_navigate` to http://localhost:3000
3. Take a snapshot to understand the current state
4. Perform the test actions
5. Verify results with snapshots
6. If something fails, check /tmp/pulse-web.log for server-side errors

## Troubleshooting

- If the page doesn't load, check if the server is running: `curl http://localhost:3000`
- If you see errors, read the server logs at /tmp/pulse-web.log
- Do NOT attempt to restart the server - ask the user to do it if needed
