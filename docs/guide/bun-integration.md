# Bun Runtime Integration

This document describes how Bun runtime management is integrated into vite-plus and how to enable it when ready.

## Current Status

**Bun shims are available but bun runtime management is not yet implemented.**

The following shims are now available:
- `bun` - The Bun runtime
- `bunx` - Bun's package executor (equivalent to npx)

When you run `bun` or `bunx`, the shim currently passthroughs to your system-installed bun.

## Architecture

### Shim System

The shim system intercepts tool invocations and routes them through vite-plus:

| Shim Tool | Runtime | Tools Managed |
|-----------|---------|---------------|
| `node`, `npm`, `npx` | Node.js | node, npm, npx, package binaries |
| `bun`, `bunx` | Bun | bun, bunx |

### Runtime Trait

The `Runtime` trait in `crates/vite_global_cli/src/runtime/mod.rs` defines the interface for runtime management:

```rust
pub trait Runtime: Send + Sync {
    fn name(&self) -> &'static str;
    fn shim_tools(&self) -> &[&str];
    fn resolve_version(&self, cwd: &AbsolutePath) -> Result<Resolution, Error>;
    fn ensure_installed(&self, version: &str) -> Result<(), Error>;
    fn locate_tool(&self, version: &str, tool: &str) -> Result<AbsolutePathBuf, Error>;
    fn find_system_tool(&self, tool: &str) -> Option<AbsolutePathBuf>;
}
```

### Implementations

- **NodeRuntime** (`runtime/node.rs`) - Fully implemented
- **BunRuntime** (`runtime/bun.rs`) - Stub implementation (passthrough to system bun)

## Enabling Bun Runtime Management

When Bun runtime management is ready, follow these steps to enable it:

### 1. Implement BunRuntime

Update `crates/vite_global_cli/src/runtime/bun.rs` to implement full version resolution and installation:

```rust
impl Runtime for BunRuntime {
    fn resolve_version(&self, cwd: &AbsolutePath) -> Result<Resolution, Error> {
        // Read from sources in order of priority:
        // 1. .bun-version file
        // 2. package.json engines.bun
        // 3. bunfig.toml [install] version
        // 4. BUN_VERSION env var
        // 5. Global default (from bun config)
    }

    fn ensure_installed(&self, version: &str) -> Result<(), Error> {
        // Download bun from api.bun.sh/install
        // Extract to $VITE_PLUS_HOME/js_runtime/bun/<version>/
    }
}
```

### 2. Enable Bun Runtime in get_runtime_for_tool

In `crates/vite_global_cli/src/runtime/mod.rs`, uncomment the Bun runtime:

```rust
pub fn get_runtime_for_tool(tool: &str) -> Option<&'static dyn Runtime> {
    if matches!(tool, "node" | "npm" | "npx") {
        return Some(&NodeRuntime as &'static dyn Runtime);
    }

    // Uncomment when BunRuntime is ready:
    if matches!(tool, "bun" | "bunx") {
        return Some(&BunRuntime as &'static dyn Runtime);
    }

    None
}
```

### 3. Update Shim Dispatch

In `crates/vite_global_cli/src/shim/dispatch.rs`, update the bun handling to use the runtime trait:

```rust
// Replace:
if is_bun_tool(tool) {
    tracing::debug!("bun tool detected, passthrough to system bun");
    return passthrough_to_system(tool, args);
}

// With:
if is_bun_tool(tool) {
    if let Some(runtime) = runtime::get_runtime_for_tool(tool) {
        // Use runtime trait methods
        let cwd = current_dir()?;
        let resolution = runtime.resolve_version(&cwd)?;
        runtime.ensure_installed(&resolution.version)?;
        let tool_path = runtime.locate_tool(&resolution.version, tool)?;
        // Execute tool...
    } else {
        return passthrough_to_system(tool, args);
    }
}
```

## Version Resolution Sources

Bun supports version resolution from multiple sources:

### Priority Order

1. **Session Override** - Set via `vp env use --bun <version>`
2. **Environment Variable** - `BUN_VERSION` env var
3. **Version File** - `.bun-version` in project root
4. **Package.json** - `engines.bun` field
5. **Bunfig.toml** - `[install]` section
6. **Global Default** - From `bun config` output

### Version File Format

`.bun-version`:
```
1.3.0
```

### Package.json Field

```json
{
  "engines": {
    "bun": ">=1.0.0"
  }
}
```

## Installation

Bun will be installed to `$VITE_PLUS_HOME/js_runtime/bun/<version>/`:

```
~/.vite-plus/
└── js_runtime/
    └── bun/
        ├── 1.3.0/
        │   └── bin/
        │       └── bun
        └── 1.4.0/
            └── bin/
                └── bun
```

### Download Source

Bun is downloaded from `https://api.bun.sh/install/<version>` for the appropriate platform.

## Configuration

### Environment Variables

- `VITE_PLUS_RUNTIME` - Force a specific runtime (`node` or `bun`)
- `BUN_VERSION` - Pin bun version
- `VITE_PLUS_BUN_VERSION` - Alternative to BUN_VERSION

### Shim Mode

Configure how shims behave with `vite-task.json`:

```json
{
  "shimMode": "managed" | "system-first" | "system-only"
}
```

- `managed` (default) - Use vite-plus managed runtimes
- `system-first` - Try system runtimes first, fall back to managed
- `system-only` - Never use managed runtimes

## Deprecation Path

When bun runtime management is fully enabled, the following Node.js-specific commands will be deprecated:

- `vp env setup` - Will create bun shims instead
- `vp env install` - Bun self-manages
- `vp env use` - Add `--runtime` flag for runtime selection
- `vp env pin` - Add `--runtime` flag
- `vp env list-remote` - Separate commands for node vs bun

The `vp pm` command will remain available for npm/pnpm/yarn but may be deprecated in favor of bun's all-in-one approach.

## Troubleshooting

### Bun Not Found

If you see "bun not found", ensure:
1. Bun is installed on your system: `curl -fsSL https://bun.sh/install | bash`
2. The bun shim is in your PATH

### Version Resolution Issues

Check which version is being resolved:
```bash
vp env current  # For Node.js
# Bun: check .bun-version file or bun --version
```

### Debug Mode

Enable debug output:
```bash
VITE_PLUS_DEBUG_SHIM=1 bun <args>
```
