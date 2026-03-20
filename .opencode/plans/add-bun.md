# Plan: Adding Bun Package Manager and Runtime Support to Vite-Plus

## Executive Summary

Bun support requires modifications across **5 key areas**: the package manager type system, runtime type system, workspace detection, lockfile handling, and CLI integration. This is a significant but well-scoped change that follows existing patterns established for npm/pnpm/yarn.

---

## Phase 1: Package Manager Support

### 1.1 Extend `PackageManagerType` Enum

**File:** `crates/vite_install/src/package_manager.rs`

**Current State:**
```rust
pub enum PackageManagerType {
    Pnpm,
    Yarn,
    Npm,
}
```

**Changes Required:**
```rust
pub enum PackageManagerType {
    Pnpm,
    Yarn,
    Npm,
    Bun,  // ADD THIS
}
```

**Update these locations:**
- `fmt::Display` implementation (line 49-57) - add `Self::Bun => write!(f, "bun")`
- `get_package_manager_type_and_version()` (line 233-318) - detection logic
- `create_shim_files()` (line 475-516) - shim file creation
- `get_fingerprint_ignores()` (line 165-229) - cache fingerprinting
- Interactive menu (line 568-701) - add bun as an option
- Simple prompt (line 730-770) - add bun as an option
- All error variants that list supported package managers

### 1.2 Add Bun Detection Logic

**Location:** `get_package_manager_type_and_version()` function

**Detection Priority (after checking `packageManager` field):**
1. `bun.lock` (text-based, Bun v1.2+) - **NEW**
2. `bun.lockb` (binary, Bun v1.0-1.1) - **NEW**
3. Existing pnpm/yarn/npm detection remains unchanged

**Implementation:**
```rust
// Check bun.lock first (v1.2+ text format)
let bun_lock_path = workspace_root.path.join("bun.lock");
if is_exists_file(&bun_lock_path)? {
    return Ok((PackageManagerType::Bun, version, None));
}

// Check bun.lockb (v1.0-1.1 binary format)
let bun_lockb_path = workspace_root.path.join("bun.lockb");
if is_exists_file(&bun_lockb_path)? {
    return Ok((PackageManagerType::Bun, version, None));
}
```

### 1.3 Add Shim File Creation for Bun

**Location:** `create_shim_files()` function

**Bun Binary Structure:**
- Bun downloads as a single `bun` binary (not `bun.exe`)
- No separate `bunx` equivalent (uses `bun x`)
- Symlinks or copies the bun binary directly

**Implementation:**
```rust
PackageManagerType::Bun => {
    bin_names.push(("bun", "bun"));
    // bun doesn't have a separate x equivalent like pnpm/npx
}
```

### 1.4 Add Bun Fingerprint Ignores

**Location:** `get_fingerprint_ignores()` in `PackageManager::get_fingerprint_ignores()`

**Implementation:**
```rust
PackageManagerType::Bun => {
    ignores.push("!**/bun.lock".into());
    ignores.push("!**/bun.lockb".into());
    ignores.push("!**/bunfig.toml".into());  // Bun config file
}
```

### 1.5 Add Bun Package Manager Download

**Location:** `download_package_manager()` and `get_latest_version()` functions

**Key Differences from npm/pnpm/yarn:**
- Bun package on npm: `bun` (but typically downloaded directly from GitHub releases)
- Bun doesn't publish to npm registry in the same way
- Bun uses GitHub releases: `https://github.com/oven-sh/bun/releases`

**Alternative Approach:**
Bun should be downloaded from GitHub releases, not npm. This requires:
1. Adding a new `download_bun_from_github()` function, OR
2. Modifying `download_package_manager()` to handle non-npm sources

**Recommendation:** Create a dedicated `download_bun()` function since Bun's distribution is different from npm packages.

---

## Phase 2: Workspace Detection

### 2.1 Add Bun Workspace Variant

**File:** External crate `vite_workspace` (in vite-task repository)

The `WorkspaceFile` enum currently has:
- `PnpmWorkspaceYaml`
- `NpmWorkspaceJson`
- `NonWorkspacePackage`

**Changes Required (in vite-task):**
```rust
pub enum WorkspaceFile {
    PnpmWorkspaceYaml(PathBuf),
    NpmWorkspaceJson(PathBuf),
    BunWorkspace,  // ADD THIS - Bun uses package.json workspaces
}
```

### 2.2 Update Workspace Detection Logic

**Location:** `crates/vite_install/src/package_manager.rs` line 118-121

**Current:**
```rust
let is_monorepo = matches!(
    workspace_root.workspace_file,
    WorkspaceFile::PnpmWorkspaceYaml(_) | WorkspaceFile::NpmWorkspaceJson(_)
);
```

**Updated:**
```rust
let is_monorepo = matches!(
    workspace_root.workspace_file,
    WorkspaceFile::PnpmWorkspaceYaml(_) 
    | WorkspaceFile::NpmWorkspaceJson(_)
    | WorkspaceFile::BunWorkspace  // ADD THIS
);
```

### 2.3 Bun Workspace Support

Bun supports workspaces via `package.json#workspaces` field (same as npm/yarn). The workspace detection already handles this via `WorkspaceFile::NpmWorkspaceJson`. If a separate `BunWorkspace` variant is added, it should also be detected when `bun.lock` or `bun.lockb` exists alongside the workspace config.

---

## Phase 3: Bun Runtime Support

### 3.1 Extend `JsRuntimeType` Enum

**File:** `crates/vite_js_runtime/src/runtime.rs`

**Current State:**
```rust
pub enum JsRuntimeType {
    Node,
    // Future: Bun, Deno
}
```

**Changes Required:**
```rust
pub enum JsRuntimeType {
    Node,
    Bun,  // ADD THIS
}
```

**Update locations:**
- `fmt::Display` implementation
- `download_runtime()` function
- Any switch/match statements on `JsRuntimeType`

### 3.2 Create Bun Runtime Provider

**New File:** `crates/vite_js_runtime/src/providers/bun_provider.rs`

**Implementation Pattern (similar to `NodeProvider`):**
```rust
pub struct BunProvider;

impl JsRuntimeProvider for BunProvider {
    fn name(&self) -> &str { "bun" }
    
    fn binary_relative_path(&self, platform: Platform) -> &str {
        match platform {
            Platform::LinuxX64 => "bun-linux-x64/bun",
            Platform::LinuxArm64 => "bun-linux-aarch64/bun",
            Platform::MacosX64 => "bun-darwin-x64/bun",
            Platform::MacosArm64 => "bun-darwin-arm64/bun",
            Platform::WindowsX64 => "bun-windows-x64/bun.exe",
        }
    }
    
    fn get_download_info(&self, version: &str, platform: Platform) -> DownloadInfo {
        // Download from GitHub releases: oven-sh/bun
        let url = format!(
            "https://github.com/oven-sh/bun/releases/download/bun-v{}/{}",
            version,
            self.archive_name(platform)
        );
        // ...
    }
}
```

### 3.3 Bun Version Resolution

**Location:** `crates/vite_js_runtime/src/runtime.rs` - `resolve_node_version()` function

**New Priority Order:**
1. `.bun-version` file (highest) - **NEW**
2. `.node-version` file
3. `package.json#engines.node` / `package.json#engines.bun`
4. `package.json#devEngines.runtime[name="bun"]`

**Implementation:**
```rust
// Add bun version file support
if let Some(version) = read_bun_version_file(&current).await {
    // Similar to read_node_version_file
}

// Add engines.bun support
if let Some(bun) = &pkg.engines.bun {
    // Check bun version
}

// Add devEngines.runtime for bun
if let Some(bun_rt) = runtime.find_by_name("bun") {
    // ...
}
```

### 3.4 Add Bun Version File Support

**New File:** `crates/vite_js_runtime/src/dev_engines/bun_version.rs`

**Implementation:**
```rust
pub async fn read_bun_version_file(dir: &AbsolutePath) -> Option<Str> {
    let path = dir.join(".bun-version");
    if !tokio::fs::try_exists(&path).await.unwrap_or(false) {
        return None;
    }
    let content = tokio::fs::read_to_string(&path).await.ok()?;
    let version = content.lines().next()?.trim();
    if version.is_empty() {
        return None;
    }
    Some(version.into())
}
```

---

## Phase 4: TypeScript CLI Changes

### 4.1 Update Package Manager Types

**File:** `packages/cli/src/types/package.ts`

**Current:**
```typescript
export const PackageManager = {
  pnpm: 'pnpm',
  npm: 'npm',
  yarn: 'yarn',
} as const;
```

**Updated:**
```typescript
export const PackageManager = {
  pnpm: 'pnpm',
  npm: 'npm',
  yarn: 'yarn',
  bun: 'bun',  // ADD THIS
} as const;
```

### 4.2 Update Workspace Detection

**File:** `packages/cli/src/utils/workspace.ts`

**Changes:**
- Add detection for `bun.lock` and `bun.lockb`
- Update workspace pattern handling (bun uses npm-style workspaces)

### 4.3 Add Bun Resolver

**New File:** `packages/cli/src/resolve-bun.ts`

**Pattern:** Follow existing resolvers (resolve-vite.ts, resolve-test.ts, etc.)
```typescript
export async function resolveBun(): Promise<ResolveResult> {
  // Similar to other resolvers
  // Return bun binary path and environment
}
```

### 4.4 Update NAPI Binding

**File:** `packages/cli/binding/src/package_manager.rs`

**Changes:**
- Export new `PackageManagerType::Bun` variant
- Update `PackageManagerName` conversion

---

## Phase 5: Error Handling

### 5.1 Update Error Types

**File:** `crates/vite_error/src/lib.rs`

**Changes:**
```rust
#[error("Unsupported package manager: {0}")]
UnsupportedPackageManager(Str),

// Consider adding Bun-specific errors if needed:
#[error("Bun version {version} not found")]
BunVersionNotFound { version: Str },
```

---

## Phase 6: Testing

### 6.1 Unit Tests

**Add tests to:** `crates/vite_install/src/package_manager.rs`
- `test_detect_package_manager_with_bun_lock`
- `test_detect_package_manager_with_bun_lockb`
- `test_parse_package_manager_bun_with_hash`

**Add tests to:** `crates/vite_js_runtime/src/runtime.rs`
- `test_js_runtime_type_bun_display`
- `test_download_bun_integration`
- `test_resolve_bun_version`

### 6.2 Integration Tests

Create test fixtures:
- `/test-fixtures/bun-project/` - Simple project with bun.lock
- `/test-fixtures/bun-monorepo/` - Workspace project with bun

### 6.3 Snap Tests

Add snap tests for:
- `vp install` in bun project
- `vp run` with bun dependencies
- `vp --version` showing bun runtime info

---

## Implementation Order

### Phase 1: Package Manager (Foundation)
1. ✅ Add `Bun` variant to `PackageManagerType`
2. ✅ Add detection logic for `bun.lock`/`bun.lockb`
3. ✅ Add shim file creation for bun binary
4. ✅ Add fingerprint ignores
5. ⬜ Implement bun download (from GitHub)

### Phase 2: Workspace (Dependency)
1. ⬜ Update `WorkspaceFile` enum in vite-task (if needed)
2. ✅ Update monorepo detection
3. ⬜ Verify workspace pattern handling

### Phase 3: Runtime (Independent Feature)
1. ⬜ Add `Bun` variant to `JsRuntimeType`
2. ⬜ Create `BunProvider`
3. ⬜ Add `.bun-version` file support
4. ⬜ Update version resolution

### Phase 4: CLI (Integration)
1. ⬜ Update TypeScript types
2. ⬜ Add bun resolver
3. ⬜ Update NAPI binding
4. ⬜ Update workspace detection

### Phase 5: Testing
1. ⬜ Add unit tests
2. ⬜ Add integration tests
3. ⬜ Add snap tests

---

## Key Technical Considerations

### 1. Bun vs npm Package Compatibility
Bun can install npm packages. The `node_modules` structure should be compatible with existing tooling. However:
- Bun's linker strategy (`hoisted` vs `isolated`) affects `node_modules` layout
- Cache location differs from npm/pnpm

### 2. Bun Runtime vs Node.js
Bun is Node.js-compatible but has differences:
- Some Node.js APIs behave differently
- Bun's module resolution may differ
- Native addon support is limited

### 3. Bun as Package Manager vs Runtime
Bun can serve both roles:
- As **package manager**: Uses `bun install`, creates `bun.lock`
- As **runtime**: Uses `bun run script`, `bun ./file.js`

These are independent - a project can use bun runtime with npm package manager (less common) or vice versa.

### 4. Version Resolution Strategy
For bun version resolution:
- Bun doesn't have an npm registry API
- Use GitHub releases API: `https://api.github.com/repos/oven-sh/bun/releases/latest`
- Or use bun's built-in update mechanism

---

## Files to Modify

### Rust Files (Local)
1. `crates/vite_install/src/package_manager.rs` - **PRIMARY**
2. `crates/vite_js_runtime/src/runtime.rs` - **PRIMARY**
3. `crates/vite_error/src/lib.rs` - Minor updates
4. `crates/vite_install/src/shim.rs` - Minor updates
5. `crates/vite_js_runtime/src/lib.rs` - Add module exports

### Rust Files (External - vite-task)
1. `vite_workspace` crate - Add `BunWorkspace` variant if needed

### TypeScript Files
1. `packages/cli/src/types/package.ts` - **PRIMARY**
2. `packages/cli/src/utils/workspace.ts` - Updates
3. `packages/cli/src/resolve-bun.ts` - **NEW**
4. `packages/cli/binding/src/package_manager.rs` - Updates
5. `packages/cli/binding/src/lib.rs` - Export updates

### New Files
1. `crates/vite_js_runtime/src/providers/bun_provider.rs` - **NEW**
2. `crates/vite_js_runtime/src/dev_engines/bun_version.rs` - **NEW**
3. `packages/cli/src/resolve-bun.ts` - **NEW**

---

## Comparison with Existing Patterns

### Similar to pnpm Implementation
- Detection via lockfile
- Shim file creation
- Package manager abstraction

### Differences from pnpm
- Bun downloaded from GitHub (not npm)
- No separate `bunx` binary (uses `bun x`)
- No `bunfig.toml` in shim/fingerprint logic (optional config)
- Runtime support is more integrated (bun is both pm and runtime)

### Lessons from nx/Turborepo
- Both have bun support as package manager
- Neither supports bun as primary runtime
- Vite-plus could be first to support bun runtime natively

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Bun API changes break detection | Medium | Low | Version pins, graceful fallbacks |
| Bun binary distribution format changes | Low | Medium | Version-specific download logic |
| Bun runtime incompatibilities | Medium | High | Test extensively, document limitations |
| vite-task dependency changes | Low | Medium | Coordinate with vite-task team |

---

## Conclusion

This plan provides a **systematic, low-risk approach** to adding bun support by:
1. Following existing patterns for npm/pnpm/yarn
2. Maintaining backward compatibility
3. Using phased implementation
4. Adding comprehensive tests

The implementation is estimated to require:
- **Package Manager**: 3-5 files modified, ~200 lines of code
- **Runtime Support**: 4-5 files modified, ~400 lines of code  
- **TypeScript/CLI**: 3-4 files modified, ~150 lines of code
- **Testing**: 5-10 new test cases

**Total Estimated Effort**: ~750-1000 lines across 15-20 files
