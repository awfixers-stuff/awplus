# Bun Test Runner and Bundler Removal Guide

**Document Version:** 1.0
**Date:** 2026-03-26
**Purpose:** Comprehensive guide for removing test runner and bundler components from Bun while preserving JavaScript runtime and package manager functionality.

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Test Runner Code Locations](#2-test-runner-code-locations)
3. [Bundler Code Locations](#3-bundler-code-locations)
4. [Critical Shared Components](#4-critical-shared-components)
5. [Cross-Dependency Analysis](#5-cross-dependency-analysis)
6. [What Must Be Kept](#6-what-must-be-kept)
7. [What Must Be Removed](#7-what-must-be-removed)
8. [Detailed File Inventory](#8-detailed-file-inventory)
9. [Removal Phases](#9-removal-phases)
10. [Dependencies by Component](#10-dependencies-by-component)
11. [Risks and Mitigations](#11-risks-and-mitigations)
12. [Verification Checklist](#12-verification-checklist)
13. [Estimated Impact](#13-estimated-impact)

---

## 1. Executive Summary

### Overview

This document provides a detailed analysis and implementation plan for removing two major components from the Bun project:

1. **Test Runner** (`bun test`) - The Jest-compatible test framework
2. **Bundler** (`bun build`) - The JavaScript/TypeScript bundler with code splitting, tree-shaking, etc.

### Goals

- Remove test runner and bundler codebases
- Preserve JavaScript runtime functionality
- Preserve package manager functionality
- Minimize changes to shared infrastructure
- Maintain build system integrity

### Non-Goals

This document does NOT cover removing:

- HTTP server/client functionality
- Web APIs (fetch, WebSocket, etc.)
- Node.js compatibility layer
- FFI capabilities
- SQLite integration
- Shell execution

---

## 2. Test Runner Code Locations

### 2.1 CLI Entry Point

| File                       | Lines | Purpose                                                        |
| -------------------------- | ----- | -------------------------------------------------------------- |
| `src/cli/test_command.zig` | ~2044 | Main CLI test command handler, reporter output, test filtering |

### 2.2 Test Scanner

| File                       | Purpose                                                             |
| -------------------------- | ------------------------------------------------------------------- |
| `src/cli/test/Scanner.zig` | Finds test files matching patterns (`*.test.ts`, `*.spec.js`, etc.) |

### 2.3 Core Test Framework (Zig)

Directory: `src/bun.js/test/`

| File                 | Purpose                                                                      |
| -------------------- | ---------------------------------------------------------------------------- |
| `jest.zig`           | Core `TestRunner` struct, Jest-compatible API (`test()`, `describe()`, etc.) |
| `bun_test.zig`       | `BunTest` and `BunTestRoot` - manages test files, phases, hooks              |
| `Execution.zig`      | Test execution engine - concurrent groups, hooks, timeouts, retry logic      |
| `Collection.zig`     | Test collection phase - discovers `test()` and `describe()` calls            |
| `Order.zig`          | Converts collected tests into execution order                                |
| `ScopeFunctions.zig` | `test()`, `describe()` and variants (skip, only, todo, concurrent)           |
| `expect.zig`         | Main expect API, delegates to individual matchers                            |
| `snapshot.zig`       | Snapshot testing support                                                     |
| `DoneCallback.zig`   | Done callback for async tests                                                |
| `debug.zig`          | Debug logging utilities                                                      |
| `diff_format.zig`    | Diff formatting for test output                                              |
| `pretty_format.zig`  | Pretty formatting for test output                                            |
| `test.zig`           | General test utilities                                                       |

### 2.4 Expect Matchers

Directory: `src/bun.js/test/expect/`

Contains ~50 files for individual matchers:

- `toBe.zig`, `toEqual.zig`, `toStrictEqual.zig`
- `toBeNull.zig`, `toBeUndefined.zig`, `toBeDefined.zig`
- `toBeTrue.zig`, `toBeFalse.zig`, `toBeTruthy.zig`, `toBeFalsy.zig`
- `toBeGreaterThan.zig`, `toBeLessThan.zig`, `toBeGreaterThanOrEqual.zig`, `toBeLessThanOrEqual.zig`
- `toContain.zig`, `toContainEqual.zig`, `toContainKey.zig`, `toContainKeys.zig`, `toContainValue.zig`, `toContainValues.zig`
- `toMatch.zig`, `toMatchObject.zig`, `toMatchSnapshot.zig`, `toMatchInlineSnapshot.zig`
- `toThrow.zig`, `toThrowErrorMatchingSnapshot.zig`, `toThrowErrorMatchingInlineSnapshot.zig`
- `toHaveProperty.zig`, `toHaveLength.zig`, `toHaveReturned.zig`, `toHaveReturnedWith.zig`
- `toBeInstanceOf.zig`, `toBeArray.zig`, `toBeObject.zig`, `toBeString.zig`, `toBeNumber.zig`
- `toBeEmpty.zig`, `toBeEmptyObject.zig`, `toContainAllKeys.zig`, `toContainAllValues.zig`
- `toStartWith.zig`, `toEndWith.zig`, `toInclude.zig`, `toIncludeRepeated.zig`
- `toBeCloseTo.zig`, `toBeNil.zig`, `toBeOneOf.zig`
- And many more...

### 2.5 Test Timers

Directory: `src/bun.js/test/timers/`

| File             | Purpose                                    |
| ---------------- | ------------------------------------------ |
| `FakeTimers.zig` | Fake timer implementation for test control |

### 2.6 Test Diff Utilities

Directory: `src/bun.js/test/diff/`

| File                   | Purpose                       |
| ---------------------- | ----------------------------- |
| `diff_match_patch.zig` | Diff algorithm implementation |
| `printDiff.zig`        | Diff printing utilities       |

### 2.7 Node.js Test Module

| File                  | Purpose                                                             |
| --------------------- | ------------------------------------------------------------------- |
| `src/js/node/test.ts` | Node.js `test` module compatibility (mostly throws not implemented) |

### 2.8 Internal Zig Tests

| File                | Purpose                                          |
| ------------------- | ------------------------------------------------ |
| `src/main_test.zig` | Internal Zig unit tests (for testing Bun itself) |

### 2.9 Test Harness

| File              | Purpose                                                         |
| ----------------- | --------------------------------------------------------------- |
| `test/harness.ts` | Test utilities: `bunExe()`, `bunEnv`, `tempDir()`, `gc()`, etc. |

---

## 3. Bundler Code Locations

### 3.1 Main Bundler Directory

Directory: `src/bundler/`

| File                           | Lines | Purpose                                                        |
| ------------------------------ | ----- | -------------------------------------------------------------- |
| `bundle_v2.zig`                | ~5000 | Core `BundleV2` struct - main orchestrator for bundling        |
| `LinkerContext.zig`            | ~2700 | Central linking orchestrator - coordinates all bundling phases |
| `Graph.zig`                    | -     | Module dependency graph representation                         |
| `ParseTask.zig`                | ~1500 | Task for parsing source files in parallel                      |
| `Chunk.zig`                    | -     | Represents output chunks (groups of files for combined output) |
| `AstBuilder.zig`               | -     | Builds AST for generated code                                  |
| `barrel_imports.zig`           | -     | Barrel file optimization                                       |
| `ThreadPool.zig`               | -     | Thread pool for parallel bundling                              |
| `BundleThread.zig`             | -     | Bundler thread management                                      |
| `DeferredBatchTask.zig`        | -     | Deferred batch processing                                      |
| `IndexStringMap.zig`           | -     | String mapping utility                                         |
| `PathToSourceIndexMap.zig`     | -     | Maps file paths to source indices                              |
| `HTMLImportManifest.zig`       | -     | Tracks HTML imports from server-side code                      |
| `ServerComponentParseTask.zig` | -     | Special parsing for Server Components                          |
| `entry_points.zig`             | -     | Entry point generation                                         |
| `LinkerGraph.zig`              | -     | Graph data structure for linking phase                         |

### 3.2 Linker Context Subdirectory

Directory: `src/bundler/linker_context/`

| File                                    | Purpose                                        |
| --------------------------------------- | ---------------------------------------------- |
| `scanImportsAndExports.zig`             | Phase 1 - analyzes import/export relationships |
| `computeChunks.zig`                     | Phase 3 - determines chunk structure           |
| `computeCrossChunkDependencies.zig`     | Phase 4 - resolves chunk dependencies          |
| `renameSymbolsInChunk.zig`              | Symbol renaming/minification                   |
| `generateChunksInParallel.zig`          | Parallel chunk generation                      |
| `generateCodeForFileInChunkJS.zig`      | JS code generation per file                    |
| `generateCompileResultForJSChunk.zig`   | JS chunk compilation                           |
| `generateCompileResultForCssChunk.zig`  | CSS chunk compilation                          |
| `generateCompileResultForHtmlChunk.zig` | HTML chunk compilation                         |
| `generateCodeForLazyExport.zig`         | Deferred code for JSON/CSS loaders             |
| `convertStmtsForChunk.zig`              | AST statement transformation                   |
| `convertStmtsForChunkForDevServer.zig`  | Dev server statement conversion                |
| `findAllImportedPartsInJSOrder.zig`     | Determines part order in JS chunks             |
| `findImportedCSSFilesInJSOrder.zig`     | Orders CSS imports in JS                       |
| `findImportedFilesInCSSOrder.zig`       | Processes CSS @import statements               |
| `postProcessJSChunk.zig`                | Final JS chunk processing                      |
| `postProcessCSSChunk.zig`               | Final CSS chunk processing                     |
| `postProcessHTMLChunk.zig`              | Final HTML chunk processing                    |
| `prepareCssAstsForChunk.zig`            | CSS AST preparation                            |
| `writeOutputFilesToDisk.zig`            | Writes chunks to filesystem                    |
| `OutputFileListBuilder.zig`             | Builds output file list                        |
| `MetafileBuilder.zig`                   | Generates metadata files                       |
| `StaticRouteVisitor.zig`                | Static route handling                          |
| `doStep5.zig`                           | Namespace export creation                      |

### 3.3 CLI Build Command

| File                        | Purpose                             |
| --------------------------- | ----------------------------------- |
| `src/cli/build_command.zig` | CLI command handler for `bun build` |

### 3.4 JavaScript API

| File                           | Purpose                                    |
| ------------------------------ | ------------------------------------------ |
| `src/bun.js/api/JSBundler.zig` | `Bun.build()` JavaScript API (~2000 lines) |

### 3.5 File System Router

| File                                   | Purpose                                    |
| -------------------------------------- | ------------------------------------------ |
| `src/bun.js/api/filesystem_router.zig` | File-based routing (may have bundler deps) |

### 3.6 Bundler Tests

Directory: `test/bundler/`

Contains 47 test files including:

- `bundler_compile.test.ts`
- `bundler_minify.test.ts`
- `bundler_plugin.test.ts`
- `bundler_splitting.test.ts`
- `bundler_cjs.test.ts`
- `bundler_jsx.test.ts`
- `bundler_html.test.ts`
- `bundler_barrel.test.ts`
- `bun-build-api.test.ts`
- And many more...

---

## 4. Critical Shared Components

These components are used by **multiple features** and **MUST BE KEPT**:

### 4.1 Module Resolution

| File                             | Purpose               | Used By                       |
| -------------------------------- | --------------------- | ----------------------------- |
| `src/resolver/resolver.zig`      | Main module resolver  | Runtime, Package Manager, CLI |
| `src/resolver/resolve_path.zig`  | Path utilities        | Everywhere                    |
| `src/resolver/package_json.zig`  | Package.json handling | PM, CLI                       |
| `src/resolver/dir_info.zig`      | Directory caching     | PM, Bundler                   |
| `src/resolver/tsconfig_json.zig` | TypeScript config     | CLI, Runtime                  |
| `src/resolver/data_url.zig`      | Data URL resolution   | Bundler, Runtime              |

### 4.2 JavaScript Parser/Printer

| File                 | Purpose         | Used By                |
| -------------------- | --------------- | ---------------------- |
| `src/js_parser.zig`  | JS/TS parsing   | Runtime, PM, CLI, REPL |
| `src/js_lexer.zig`   | Tokenization    | Runtime, Bundler       |
| `src/js_printer.zig` | Code generation | PM, CLI commands       |

### 4.3 Other Essential Infrastructure

| File                | Purpose             | Used By                    |
| ------------------- | ------------------- | -------------------------- |
| `src/options.zig`   | Configuration       | Runtime, PM, CLI           |
| `src/bunfig.zig`    | Config file parsing | Runtime, PM, CLI           |
| `src/cache.zig`     | Caching             | Runtime (uses js_parser)   |
| `src/linker.zig`    | Runtime linker      | Runtime (has resolver dep) |
| `src/router.zig`    | Router              | Runtime (has resolver dep) |
| `src/ast/Macro.zig` | Macro execution     | Runtime (has resolver dep) |

---

## 5. Cross-Dependency Analysis

### 5.1 Resolver Dependencies

The resolver (`src/resolver/resolver.zig`) is imported by **21 files**:

```
src/
├── bun.zig (line 201)              ← KEEP
├── transpiler.zig (line 1649)       ← KEEP
├── router.zig (line 961)           ← STUB
├── options.zig (line 2728)          ← STUB
├── linker.zig (line 407)            ← STUB
├── js_parser.zig (line 1214)        ← KEEP
├── bunfig.zig (line 1284)           ← KEEP
├── bun.js/VirtualMachine.zig        ← KEEP (Runtime)
├── bun.js/api/JSBundler.zig         ← REMOVE
├── bun.js/api/filesystem_router.zig  ← STUB
├── bundler/bundle_v2.zig            ← REMOVE
├── bundler/ParseTask.zig            ← REMOVE
├── bundler/LinkerContext.zig        ← REMOVE
├── bundler/Graph.zig                ← REMOVE
├── ast/Macro.zig                    ← KEEP (Runtime)
└── OutputFile.zig                  ← STUB
```

### 5.2 js_printer Dependencies (469 files)

The js_printer is used extensively across the codebase:

| Category        | Count | Example Files                                      |
| --------------- | ----- | -------------------------------------------------- |
| Package Manager | ~10   | `install/pnpm.zig`, `install/PackageManager/*.zig` |
| CLI Commands    | ~15   | `cli/pack_command.zig`, `cli/publish_command.zig`  |
| Bundler         | ~15   | `bundler/linker_context/*.zig`                     |
| Runtime         | ~5    | `fmt.zig`, `repl.zig`                              |
| Tests           | Many  | Various test files                                 |

### 5.3 js_parser Dependencies

| Category        | Usage                  |
| --------------- | ---------------------- |
| Runtime         | AST manipulation, REPL |
| Package Manager | Lockfile parsing       |
| Bundler         | File parsing           |
| CLI             | Various commands       |

### 5.4 BundleV2 Dependencies

The bundler's main file imports from these external modules:

```zig
// src/bundler/bundle_v2.zig imports:
const CacheSet = @import("../cache.zig");
const lex = @import("../js_lexer.zig");
const Logger = @import("../logger.zig");
const js_printer = @import("../js_printer.zig");
const linker = @import("../linker.zig");
const Fs = @import("../fs.zig");
const _resolver = @import("../resolver/resolver.zig");
const resolve_path = @import("../resolver/resolve_path.zig");
const runtime = @import("../runtime.zig");
const Timer = @import("../system_timer.zig");
const HTMLScanner = @import("../HTMLScanner.zig");
const NodeFallbackModules = @import("../node_fallbacks.zig");
const CacheEntry = @import("../cache.zig").Fs.Entry;
const URL = @import("../url.zig").URL;
const DataURL = @import("../resolver/resolver.zig").DataURL;
const options = @import("../options.zig");
const bun = @import("bun");
```

### 5.5 Test Runner Dependencies

The test command imports:

```zig
// src/cli/test_command.zig imports:
const DotEnv = @import("../env_loader.zig");
const Scanner = @import("./test/Scanner.zig");
const bun_test = @import("../bun.js/test/bun_test.zig");
const options = @import("../options.zig");
const resolve_path = @import("../resolver/resolve_path.zig");
const FileSystem = @import("../fs.zig").FileSystem;
const which = @import("../which.zig").which;
const bun = @import("bun");
```

---

## 6. What Must Be Kept

### 6.1 Core Runtime

```
src/
├── bun.zig                     # Root module (re-exports everything)
├── runtime.zig                  # JavaScript runtime
├── bun.js/                      # JS runtime bindings
│   ├── api/                     # Runtime APIs (NOT JSBundler.zig)
│   ├── node/                    # Node.js compatibility
│   └── webcore/                 # Web APIs
├── js_parser.zig                # JS parser (needed by PM)
├── js_lexer.zig                # Lexer
├── js_printer.zig              # Printer (needed by PM)
├── resolver/                    # Module resolution
│   ├── resolver.zig
│   ├── resolve_path.zig
│   ├── package_json.zig
│   ├── dir_info.zig
│   ├── tsconfig_json.zig
│   └── data_url.zig
├── install/                     # Package manager
│   ├── install.zig
│   ├── PackageManager/
│   └── lockfile/
├── cli/                         # CLI (except test_command.zig, build_command.zig)
├── http/                        # HTTP client/server
├── webcrypto/                   # WebCrypto
├── sqlite/                      # SQLite
├── shell/                       # Shell implementation
├── css/                         # CSS processing
└── [other essential modules]
```

### 6.2 Package Manager Dependencies

The package manager (`src/install/`) has these imports:

- `src/resolver/dir_info.zig` - For directory resolution
- `bun.js_printer` - For JSON output
- Standard library

**The package manager does NOT import from `src/bundler/` and should survive bundler removal with minimal changes.**

---

## 7. What Must Be Removed

### 7.1 Test Runner (Primary)

```
src/
├── cli/test_command.zig
├── cli/test/
│   └── Scanner.zig
├── bun.js/test/                    # Entire directory (~50 files)
│   ├── jest.zig
│   ├── bun_test.zig
│   ├── Execution.zig
│   ├── Collection.zig
│   ├── Order.zig
│   ├── ScopeFunctions.zig
│   ├── expect.zig
│   ├── snapshot.zig
│   ├── DoneCallback.zig
│   ├── debug.zig
│   ├── diff_format.zig
│   ├── pretty_format.zig
│   ├── test.zig
│   ├── timers/
│   │   └── FakeTimers.zig
│   ├── expect/                    # ~50 matcher files
│   └── diff/
│       ├── diff_match_patch.zig
│       └── printDiff.zig
├── js/node/test.ts
└── main_test.zig

test/
├── harness.ts
├── bake/                          # Dev server/HMR tests
├── bundler/                       # Bundler tests (also removed)
├── cli/                           # CLI tests
├── integration/                   # E2E tests
├── js/                            # Runtime tests
├── napi/                          # N-API tests
├── regression/                    # Regression tests
├── runners/                       # Alternative test runners
├── v8/                            # V8 tests
├── fixtures/                      # Test fixtures
└── internal/                      # Internal tests
```

### 7.2 Bundler (Primary)

```
src/
├── bundler/                       # Entire directory (~45 files)
│   ├── bundle_v2.zig
│   ├── LinkerContext.zig
│   ├── Graph.zig
│   ├── ParseTask.zig
│   ├── Chunk.zig
│   ├── AstBuilder.zig
│   ├── barrel_imports.zig
│   ├── ThreadPool.zig
│   ├── BundleThread.zig
│   ├── DeferredBatchTask.zig
│   ├── IndexStringMap.zig
│   ├── PathToSourceIndexMap.zig
│   ├── HTMLImportManifest.zig
│   ├── ServerComponentParseTask.zig
│   ├── entry_points.zig
│   ├── LinkerGraph.zig
│   └── linker_context/            # ~24 files
├── cli/build_command.zig
├── bun.js/api/JSBundler.zig
├── bun.js/api/filesystem_router.zig  # (audit first)
└── transpiler.zig                 # (audit - may have essential passes)

test/
└── bundler/                       # 47 test files
```

### 7.3 Secondary Removals (Audit Required)

```
src/
├── router.zig                     # May need bundler-specific code removed
├── options.zig                    # May need bundler options removed
├── OutputFile.zig                 # May need bundler-specific output types
└── [any other files with bundle_v2 imports]
```

---

## 8. Detailed File Inventory

### 8.1 Complete Bundler File List (41 files)

```
src/bundler/
├── bundle_v2.zig
├── LinkerContext.zig
├── Graph.zig
├── ParseTask.zig
├── Chunk.zig
├── AstBuilder.zig
├── barrel_imports.zig
├── ThreadPool.zig
├── BundleThread.zig
├── DeferredBatchTask.zig
├── IndexStringMap.zig
├── PathToSourceIndexMap.zig
├── HTMLImportManifest.zig
├── ServerComponentParseTask.zig
├── entry_points.zig
├── LinkerGraph.zig
└── linker_context/
    ├── scanImportsAndExports.zig
    ├── computeChunks.zig
    ├── computeCrossChunkDependencies.zig
    ├── renameSymbolsInChunk.zig
    ├── generateChunksInParallel.zig
    ├── generateCodeForFileInChunkJS.zig
    ├── generateCompileResultForJSChunk.zig
    ├── generateCompileResultForCssChunk.zig
    ├── generateCompileResultForHtmlChunk.zig
    ├── generateCodeForLazyExport.zig
    ├── convertStmtsForChunk.zig
    ├── convertStmtsForChunkForDevServer.zig
    ├── findAllImportedPartsInJSOrder.zig
    ├── findImportedCSSFilesInJSOrder.zig
    ├── findImportedFilesInCSSOrder.zig
    ├── postProcessJSChunk.zig
    ├── postProcessCSSChunk.zig
    ├── postProcessHTMLChunk.zig
    ├── prepareCssAstsForChunk.zig
    ├── writeOutputFilesToDisk.zig
    ├── OutputFileListBuilder.zig
    ├── MetafileBuilder.zig
    ├── StaticRouteVisitor.zig
    └── doStep5.zig
```

### 8.2 Complete Test Runner File List (~55 Zig files)

```
src/cli/test/
└── Scanner.zig

src/bun.js/test/
├── jest.zig
├── bun_test.zig
├── Execution.zig
├── Collection.zig
├── Order.zig
├── ScopeFunctions.zig
├── expect.zig
├── snapshot.zig
├── DoneCallback.zig
├── debug.zig
├── diff_format.zig
├── pretty_format.zig
├── test.zig
├── timers/
│   └── FakeTimers.zig
├── expect/
│   ├── toBe.zig
│   ├── toEqual.zig
│   ├── toStrictEqual.zig
│   ├── toBeNull.zig
│   ├── toBeUndefined.zig
│   ├── toBeDefined.zig
│   ├── toBeTrue.zig
│   ├── toBeFalse.zig
│   ├── toBeTruthy.zig
│   ├── toBeFalsy.zig
│   ├── toBeGreaterThan.zig
│   ├── toBeLessThan.zig
│   ├── toBeGreaterThanOrEqual.zig
│   ├── toBeLessThanOrEqual.zig
│   ├── toContain.zig
│   ├── toContainEqual.zig
│   ├── toContainKey.zig
│   ├── toContainKeys.zig
│   ├── toContainValue.zig
│   ├── toContainValues.zig
│   ├── toMatch.zig
│   ├── toMatchObject.zig
│   ├── toMatchSnapshot.zig
│   ├── toMatchInlineSnapshot.zig
│   ├── toThrow.zig
│   ├── toThrowErrorMatchingSnapshot.zig
│   ├── toThrowErrorMatchingInlineSnapshot.zig
│   ├── toHaveProperty.zig
│   ├── toHaveLength.zig
│   ├── toHaveReturned.zig
│   ├── toHaveReturnedWith.zig
│   ├── toHaveBeenCalled.zig
│   ├── toHaveBeenCalledWith.zig
│   ├── toHaveBeenCalledTimes.zig
│   ├── toHaveBeenLastCalledWith.zig
│   ├── toHaveBeenNthCalledWith.zig
│   ├── toHaveBeenCalledOnce.zig
│   ├── toHaveReturnedTimes.zig
│   ├── toHaveNthReturnedWith.zig
│   ├── toHaveReturnedWith.zig
│   ├── toHaveLastReturnedWith.zig
│   ├── toBeInstanceOf.zig
│   ├── toBeArray.zig
│   ├── toBeObject.zig
│   ├── toBeString.zig
│   ├── toBeNumber.zig
│   ├── toBeBoolean.zig
│   ├── toBePositive.zig
│   ├── toBeNegative.zig
│   ├── toBeOdd.zig
│   ├── toBeEven.zig
│   ├── toBeInteger.zig
│   ├── toBeFinite.zig
│   ├── toBeNaN.zig
│   ├── toBeEmpty.zig
│   ├── toBeEmptyObject.zig
│   ├── toBeArrayOfSize.zig
│   ├── toBeCloseTo.zig
│   ├── toBeNil.zig
│   ├── toBeOneOf.zig
│   ├── toStartWith.zig
│   ├── toEndWith.zig
│   ├── toInclude.zig
│   ├── toIncludeRepeated.zig
│   ├── toContainAllKeys.zig
│   ├── toContainAllValues.zig
│   ├── toContainAnyKeys.zig
│   ├── toContainAnyValues.zig
│   ├── toBeDate.zig
│   ├── toBeValidDate.zig
│   ├── toBeSymbol.zig
│   ├── toBeTypeOf.zig
│   ├── toBeFunction.zig
│   └── toEqualIgnoringWhitespace.zig
└── diff/
    ├── diff_match_patch.zig
    └── printDiff.zig
```

---

## 9. Removal Phases

### Phase 1: Preparation (Weeks 1-2)

#### 1.1 Audit Shared Dependencies

- [ ] Trace all imports of `src/resolver/resolver.zig`
- [ ] Trace all imports of `src/js_parser.zig`
- [ ] Trace all imports of `src/js_printer.zig`
- [ ] Identify which CLI commands depend on bundler/test
- [ ] Check if `src/transpiler.zig` can be split

#### 1.2 Create Stubs

Create stub modules for removed components to prevent import errors:

```
src/bundler_stub.zig      # Stub for bundler types
src/test_runner_stub.zig  # Stub for test runner types
```

#### 1.3 Identify Optional Dependencies

Some dependencies can be made optional via comptime flags:

- `src/router.zig` - bundler-specific routes
- `src/options.zig` - bundler-specific options
- `src/OutputFile.zig` - bundler-specific output types

### Phase 2: Remove Test Runner (Weeks 2-3)

#### 2.1 Remove Test Framework Core

```bash
# Delete test framework
rm -rf src/bun.js/test/

# Delete test scanner
rm -rf src/cli/test/

# Delete test command
rm src/cli/test_command.zig

# Delete Node.js test module
rm src/js/node/test.ts

# Delete internal tests
rm src/main_test.zig
```

#### 2.2 Update CLI Entry Point

Edit `src/cli.zig`:

```zig
// REMOVE this line:
const TestCommand = @import("./cli/test_command.zig").TestCommand;

// REMOVE TestCommand from Command.Tag switch cases
```

#### 2.3 Remove Test Files

```bash
# Delete all test directories
rm -rf test/bake/
rm -rf test/bundler/
rm -rf test/cli/
rm -rf test/integration/
rm -rf test/js/
rm -rf test/napi/
rm -rf test/regression/
rm -rf test/runners/
rm -rf test/v8/
rm -rf test/fixtures/
rm -rf test/internal/
rm test/harness.ts
```

#### 2.4 Update Build Configuration

- Update `CMakeLists.txt` or build scripts to remove test targets
- Update `package.json` test scripts
- Update CI/CD pipelines

### Phase 3: Remove Bundler (Weeks 3-4)

#### 3.1 Remove Bundler Core

```bash
# Delete bundler directory
rm -rf src/bundler/

# Delete CLI build command
rm src/cli/build_command.zig

# Delete JavaScript API
rm src/bun.js/api/JSBundler.zig

# Delete file system router (if bundler-dependent)
rm src/bun.js/api/filesystem_router.zig
```

#### 3.2 Update CLI Entry Point

Edit `src/cli.zig`:

```zig
// REMOVE this line:
pub const BuildCommand = @import("./cli/build_command.zig").BuildCommand;

// REMOVE BuildCommand from Command.Tag switch cases
```

#### 3.3 Update Root Module

Edit `src/bun.zig`:

```zig
// REMOVE these lines:
pub const bundle_v2 = @import("./bundler/bundle_v2.zig");
pub const Loader = bundle_v2.Loader;
pub const BundleV2 = bundle_v2.BundleV2;
pub const ParseTask = bundle_v2.ParseTask;
```

#### 3.4 Stub Remaining Dependencies

Identify and stub remaining imports:

```zig
// In files that imported bundler components:
const BundleV2 = bun.BundleV2;  // Remove or stub
const ParseTask = bun.ParseTask; // Remove or stub
```

### Phase 4: Cleanup and Verification (Weeks 4-5)

#### 4.1 Clean Up Transpiler

Audit `src/transpiler.zig`:

- [ ] Identify bundler-specific passes
- [ ] Extract or stub bundler-specific code
- [ ] Keep essential transpilation features

#### 4.2 Clean Up Router

Audit `src/router.zig`:

- [ ] Identify bundler-specific route types
- [ ] Stub or remove bundler-specific code

#### 4.3 Clean Up Options

Audit `src/options.zig`:

- [ ] Remove bundler-specific options
- [ ] Keep essential runtime/PM options

#### 4.4 Update Documentation

- Update README.md
- Update CLI help text
- Update man pages

---

## 10. Dependencies by Component

### 10.1 Package Manager Dependency Graph

```
src/install/
├── install.zig
│   ├── FolderResolution (from ./resolvers/)
│   └── [no bundler imports]
├── PackageManager/
│   ├── PackageManager.zig
│   │   └── DirInfo (from ../resolver/dir_info.zig)  ← KEEP
│   ├── updatePackageJSONAndInstall.zig
│   │   └── JSPrinter (from bun.js_printer)  ← KEEP
│   ├── security_scanner.zig
│   │   └── JSPrinter (from bun.js_printer)  ← KEEP
│   └── WorkspacePackageJSONCache.zig
│       └── JSPrinter (from bun.js_printer)  ← KEEP
└── lockfile/
    ├── bun.lock.zig
    │   └── JSPrinter (from bun.js_printer)  ← KEEP
    └── [other lockfile files]

Conclusion: Package manager has ZERO bundler imports
```

### 10.2 Runtime Dependency Graph

```
src/bun.js/
├── VirtualMachine.zig
│   └── Resolver (from ../resolver/resolver.zig)  ← KEEP
├── api/
│   ├── server.zig (HTTP server)
│   ├── FFI.zig
│   ├── crypto.zig
│   └── [other APIs - no bundler imports]
└── node/
    ├── fs.zig
    ├── path.zig
    ├── process.zig
    └── [other Node.js APIs]

Conclusion: Runtime has minimal resolver dependency
```

### 10.3 CLI Dependency Graph

```
src/cli/
├── install_command.zig      ← KEEP
├── add_command.zig         ← KEEP
├── remove_command.zig      ← KEEP
├── run_command.zig         ← KEEP
├── x_command.zig (bunx)    ← KEEP
├── update_command.zig       ← KEEP
├── init_command.zig        ← KEEP
├── create_command.zig      ← KEEP
├── pack_command.zig         ← KEEP (uses js_printer)
├── publish_command.zig      ← KEEP (uses js_printer)
├── pm_view_command.zig     ← KEEP
├── pm_trusted_command.zig   ← KEEP
├── pm_version_command.zig   ← KEEP
├── pm_pkg_command.zig       ← KEEP
├── repl_command.zig         ← KEEP
├── [test_command.zig]       ← REMOVE
└── [build_command.zig]      ← REMOVE
```

---

## 11. Risks and Mitigations

### 11.1 High-Risk Items

| Risk                                     | Probability | Impact   | Mitigation                                                  |
| ---------------------------------------- | ----------- | -------- | ----------------------------------------------------------- |
| Hidden bundler dependencies in runtime   | Medium      | High     | Thorough grep for `bundle_v2`, `BundleV2` across all source |
| Resolver changes break runtime           | Low         | Critical | Keep resolver unchanged, only stub bundler usage            |
| Build system dependencies on test runner | High        | Medium   | Update CMakeLists.txt, Makefile, etc.                       |
| Transpiler has bundled code              | Medium      | Medium   | Audit transpiler.zig for bundler-specific passes            |
| CSS processing depends on bundler        | Medium      | Low      | Extract CSS handling if needed                              |

### 11.2 Medium-Risk Items

| Risk                                  | Probability | Impact | Mitigation                                   |
| ------------------------------------- | ----------- | ------ | -------------------------------------------- |
| CLI commands depend on bundler        | Medium      | Medium | Audit each command individually              |
| JS class bindings generated from test | High        | Low    | Regenerate bindings after removing test code |
| Documentation references bundler/test | High        | Low    | Update docs after removal                    |

### 11.3 Low-Risk Items

| Risk                     | Probability | Impact | Mitigation                  |
| ------------------------ | ----------- | ------ | --------------------------- |
| Third-party integrations | Low         | Low    | Monitor CI after removal    |
| Performance benchmarks   | Low         | Low    | Remove or update benchmarks |

---

## 12. Verification Checklist

### 12.1 Build Verification

```bash
# After each phase, verify:
- [ ] `bun bd` completes successfully
- [ ] No linker errors about missing symbols
- [ ] No compiler warnings about unused imports
- [ ] Build produces working binary
```

### 12.2 Runtime Verification

```bash
# Test basic runtime functionality:
- [ ] `bun --help` shows no test/build commands
- [ ] `bun -e "console.log('hello')"` works
- [ ] `bun file.js` executes JavaScript
- [ ] `bun REPL` works
```

### 12.3 Package Manager Verification

```bash
# Test package manager functionality:
- [ ] `bun install` works
- [ ] `bun add <package>` works
- [ ] `bun remove <package>` works
- [ ] `bun pm cache` commands work
- [ ] Lockfile operations work
```

### 12.4 CLI Verification

```bash
# Test CLI commands:
- [ ] `bun run <script>` works
- [ ] `bun x <package>` works
- [ ] `bun init` works
- [ ] `bun create <template>` works
- [ ] `bun update` works
- [ ] `bun --version` works
- [ ] `bun --help` shows expected commands
```

### 12.5 Test Exclusion Verification

```bash
# Verify removed:
- [ ] `bun test` command no longer exists
- [ ] `bun build` command no longer exists
- [ ] Test files no longer exist
- [ ] Bundler files no longer exist
```

### 12.6 Integration Verification

```bash
# Test real-world usage:
- [ ] Can run existing Node.js projects
- [ ] Package installation works for real packages
- [ ] Environment variables work
- [ ] File system operations work
```

---

## 13. Estimated Impact

### 13.1 Files to Delete

| Category          | Count  | Description                         |
| ----------------- | ------ | ----------------------------------- |
| Bundler (Zig)     | 41     | Core bundler implementation         |
| Test Runner (Zig) | ~55    | Core test framework                 |
| Bundler Tests     | 47     | test/bundler/\*.test.ts             |
| Other Tests       | ~1200  | Various test files                  |
| CLI Commands      | 2      | test_command.zig, build_command.zig |
| **Total**         | ~1,345 |                                     |

### 13.2 Lines of Code Removed

| Category         | Estimated LOC | Notes                    |
| ---------------- | ------------- | ------------------------ |
| Bundler Core     | ~80,000       | Including linker_context |
| Test Runner Core | ~50,000       | All test framework files |
| Bundler Tests    | ~15,000       | Test files               |
| Other Tests      | ~200,000      | All other test files     |
| CLI Commands     | ~5,000        | Both command files       |
| **Total**        | ~350,000      |                          |

### 13.3 Files to Keep (with modifications)

| File                 | Modification                             |
| -------------------- | ---------------------------------------- |
| `src/cli.zig`        | Remove TestCommand, BuildCommand imports |
| `src/bun.zig`        | Remove bundle_v2 exports                 |
| `src/router.zig`     | Stub bundler-specific routes             |
| `src/options.zig`    | Remove bundler options                   |
| `src/transpiler.zig` | May need cleanup                         |

### 13.4 Files to Audit

| File                                   | Reason                          |
| -------------------------------------- | ------------------------------- |
| `src/bun.js/api/filesystem_router.zig` | May have bundler deps           |
| `src/OutputFile.zig`                   | May have bundler-specific types |
| `src/cache.zig`                        | Uses js_parser                  |
| Various CLI commands                   | Individual audit                |

---

## Appendix A: Import Patterns to Search

When auditing for dependencies, search for these patterns:

```bash
# Bundler imports
grep -r "bundle_v2" src/
grep -r "BundleV2" src/
grep -r "LinkerContext" src/
grep -r "ParseTask" src/

# Test imports
grep -r "bun_test" src/
grep -r "TestRunner" src/
grep -r "test_command" src/

# Shared component imports
grep -r "@import.*resolver" src/
grep -r "resolver.zig" src/
```

---

## Appendix B: Build System Files to Update

| File                      | Changes Needed              |
| ------------------------- | --------------------------- |
| `CMakeLists.txt`          | Remove test/bundler targets |
| `Makefile`                | Remove test/bundler targets |
| `package.json`            | Remove test scripts         |
| `.github/workflows/*.yml` | Update CI/CD pipelines      |
| `scripts/`                | Remove test/bundler scripts |

---

## Appendix C: Questions to Resolve Before Implementation

1. **Package Manager Scope**: Should `bun pm` commands be preserved? (Yes, they should)
2. **HTTP Server**: Should `bun dev` or any server features be preserved? (Yes)
3. **FFI**: Should FFI capabilities be preserved? (Yes)
4. **Shell**: Should shell execution be preserved? (Yes)
5. **Internal Tests**: Should `src/main_test.zig` be removed? (Yes, or keep minimal)
6. **Test Harness**: Should `test/harness.ts` be kept for any reason? (No, remove)
7. **CLI Run Command**: Should `bun run` be preserved? (Yes, but audit dependencies)
8. **Transpiler**: Should transpiler-only mode be preserved? (Yes, for REPL/runtime)

---

## Appendix D: References

- **Main CLI Entry**: `src/cli.zig` (1810 lines)
- **Root Module**: `src/bun.zig` (3804 lines)
- **Bundler Entry**: `src/bundler/bundle_v2.zig` (~5000 lines)
- **Test Command**: `src/cli/test_command.zig` (~2044 lines)
- **Resolver**: `src/resolver/resolver.zig` (~4400 lines)

---

## Appendix E: CLI Command Inventory

### Commands to KEEP

| Command       | File                  | Purpose                  |
| ------------- | --------------------- | ------------------------ |
| `bun install` | `install_command.zig` | Package installation     |
| `bun add`     | `add_command.zig`     | Add packages             |
| `bun remove`  | `remove_command.zig`  | Remove packages          |
| `bun update`  | `update_command.zig`  | Update packages          |
| `bun run`     | `run_command.zig`     | Run scripts              |
| `bun x`       | `bunx_command.zig`    | Execute packages         |
| `bun init`    | `init_command.zig`    | Initialize project       |
| `bun create`  | `create_command.zig`  | Create from templates    |
| `bun pm`      | Various               | Package manager commands |
| `bun repl`    | `repl_command.zig`    | REPL                     |
| `bun help`    | Built-in              | Help text                |

### Commands to REMOVE

| Command     | File                | Purpose     |
| ----------- | ------------------- | ----------- |
| `bun test`  | `test_command.zig`  | Test runner |
| `bun build` | `build_command.zig` | Bundler     |

---

## Appendix F: Vendor Dependencies Impact

The bundler may use these vendored dependencies:

- `vendor/zlib/` - Compression
- `vendor/brotli/` - Brotli compression

**Audit required** to determine if these are only used by bundler or also by runtime/HTTP.

---

## Appendix G: Key Code Locations Reference

### Entry Points

| Component      | File                        | Key Structs/Functions                      |
| -------------- | --------------------------- | ------------------------------------------ |
| Test Runner    | `src/cli/test_command.zig`  | `TestCommand.run()`, `CommandLineReporter` |
| Test Framework | `src/bun.js/test/jest.zig`  | `TestRunner`, `Jest` module                |
| Bundler        | `src/bundler/bundle_v2.zig` | `BundleV2`, `generateFromJavaScript()`     |
| Build CLI      | `src/cli/build_command.zig` | `BuildCommand`                             |

### Exports in Root Module

| Export           | Location           | Used By               |
| ---------------- | ------------------ | --------------------- |
| `bun.bundle_v2`  | `src/bun.zig:1757` | Bundler, some CLI     |
| `bun.transpiler` | `src/bun.zig:1104` | Runtime, bundler, CLI |
| `bun.js_parser`  | `src/bun.zig:1107` | Everywhere            |
| `bun.js_printer` | `src/bun.zig:1108` | PM, CLI, bundler      |
| `bun.resolver`   | `src/bun.zig:201`  | Runtime, PM, CLI      |

---

_Document generated: 2026-03-26_
