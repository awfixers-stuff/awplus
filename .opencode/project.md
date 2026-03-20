## Summary Report: Vite-Plus Repository Structure

### 1. Overall Project Organization (Monorepo Layout)

**Structure:**
- **Root Cargo.toml**: Workspace with members in `crates/*` and `packages/cli/binding`
- **Rust Crates** (in `/crates/`):
  - `vite_command` - Command execution utilities
  - `vite_error` - Error handling
  - `vite_global_cli` - Global CLI entry point (`vp` binary)
  - `vite_install` - Package manager handling
  - `vite_js_runtime` - Node.js runtime management
  - `vite_migration` - Migration tooling
  - `vite_shared` - Shared utilities
  - `vite_static_config` - Static config extraction
  - `vite_trampoline` - Shim binary

- **TypeScript Packages** (in `/packages/`):
  - `cli` - Main CLI implementation with resolver functions
  - `core` - `@voidzero-dev/vite-plus-core`
  - `prompts` - Prompt utilities
  - `test` - `@voidzero-dev/vite-plus-test`
  - `tools` - Tool utilities

**External Dependencies (from vite-task git repo):**
The core task execution logic lives in an external repository at `https://github.com/voidzero-dev/vite-task.git`. The following are imported:
- `vite_task` - Task execution CLI
- `vite_workspace` - Workspace loading
- `vite_path` - Path utilities
- `vite_glob` - File globbing
- `vite_str` - String utilities
- `fspy` - File system access tracking

---

### 2. Package Manager Handling

**Key Files:**
- `/crates/vite_install/src/package_manager.rs` - Main package manager abstraction
- `/packages/cli/src/types/package.ts` - TypeScript package manager types

**Package Manager Abstraction:**
```rust
// crates/vite_install/src/package_manager.rs
pub enum PackageManagerType {
    Pnpm,
    Yarn,
    Npm,
}
```

**Detection Logic** (in priority order):
1. `packageManager` field in `package.json` (e.g., `"pnpm@10.0.0"`)
2. `pnpm-workspace.yaml` file → pnpm
3. `pnpm-lock.yaml` file → pnpm
4. `yarn.lock` or `.yarnrc.yml` → yarn
5. `package-lock.json` → npm

**Key Code Paths:**
- `/crates/vite_install/src/package_manager.rs:233-318` - `get_package_manager_type_and_version()` function
- `/crates/vite_install/src/package_manager.rs:102-152` - `PackageManagerBuilder::build()` for building a PackageManager

**TypeScript Side:**
- `/packages/cli/src/types/package.ts:1-6` - PackageManager enum with pnpm, npm, yarn
- `/packages/cli/src/utils/workspace.ts:31-98` - Workspace detection with package manager detection

---

### 3. Runtime Handling (Node/Deno)

**Key Files:**
- `/crates/vite_js_runtime/src/runtime.rs` - Main runtime handling

**Runtime Type:**
```rust
// crates/vite_js_runtime/src/runtime.rs
pub enum JsRuntimeType {
    Node,
    // Future: Bun, Deno
}
```

**Version Resolution Priority:**
1. `.node-version` file (highest priority)
2. `package.json#engines.node`
3. `package.json#devEngines.runtime[name="node"]` (lowest priority)

**Runtime Detection Features:**
- Supports LTS aliases: `lts/*`, `lts/iron`, `lts/-1`
- Supports `latest` alias
- Partial version support: `20`, `20.18`, `^20.18.0`

**Key Code Paths:**
- `/crates/vite_js_runtime/src/runtime.rs:245-316` - `resolve_node_version()` function
- `/crates/vite_js_runtime/src/runtime.rs:342-396` - `download_runtime_for_project()` function

**Global CLI Integration:**
- `/crates/vite_global_cli/src/js_executor.rs` - Uses `JsExecutor` to manage runtime
- Runtime can be project-specific or CLI default

---

### 4. Key Modules for Task Execution

**CLI Entry Points:**

1. **Global CLI** (`/crates/vite_global_cli/src/main.rs`)
   - Entry point for the `vp` binary
   - Handles command dispatching

2. **Local CLI** (`/packages/cli/src/bin.ts`)
   - TypeScript entry point that delegates to NAPI
   - Built-in commands (create, migrate, config, mcp, staged, --version)
   - Other commands go to Rust via NAPI

3. **NAPI Binding** (`/packages/cli/binding/src/lib.rs`)
   - Bridges JavaScript resolvers to Rust core
   - `run()` function is the main entry point

4. **CLI Implementation** (`/packages/cli/binding/src/cli.rs`)
   - Contains `CLIArgs` parser
   - `SynthesizableSubcommand` enum for built-in commands (lint, fmt, build, test, pack, dev, preview, doc, install, check)
   - `SubcommandResolver` for resolving commands to executables

**Configuration Parsing:**
- `/packages/cli/src/resolve-vite-config.ts` - Vite config resolution
- `/packages/cli/binding/src/cli.rs:583-648` - `VitePlusConfigLoader` for loading run config

**Task Graph Building:**
The task graph building is handled by the external `vite_task` crate from vite-task repository. Key concepts:
- `Session::init()` - Creates a task session
- Task execution via `Session::main(command)`
- Command handling via `CommandHandler` trait

**Command Execution/Spawning:**
- `/crates/vite_command/src/lib.rs` - Command building utilities
  - `resolve_bin()` - Resolve binary paths
  - `build_command()` - Build tokio process commands
  - `run_command()` - Execute commands
  - `run_command_with_fspy()` - Track file access

**Package Manager Command Resolution:**
- `/crates/vite_install/src/package_manager.rs:159-230` - `get_fingerprint_ignores()` for cache fingerprinting

---

### 5. Existing Package Manager Abstraction

**In Rust:**
- `PackageManager` struct with:
  - `client: PackageManagerType` (pnpm, yarn, npm)
  - Version, bin name, workspace root, install directory
- `PackageManagerBuilder` for building PackageManager instances
- Methods for resolving install commands

**In TypeScript:**
- `PackageManager` enum with pnpm, npm, yarn constants
- Resolver functions in `/packages/cli/src/resolve-*.ts` that resolve tool paths from node_modules

---

### 6. Key Resolver Functions

These are the functions that resolve tool paths from node_modules to be executed by Rust:

- `/packages/cli/src/resolve-lint.ts` - Resolves oxlint binary
- `/packages/cli/src/resolve-fmt.ts` - Resolves oxfmt binary  
- `/packages/cli/src/resolve-vite.ts` - Resolves vite binary
- `/packages/cli/src/resolve-test.ts` - Resolves vitest binary
- `/packages/cli/src/resolve-pack.ts` - Resolves tsdown binary
- `/packages/cli/src/resolve-doc.ts` - Resolves typedoc binary
