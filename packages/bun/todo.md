# Bun Test Runner & Bundler Removal - Progress Tracker

**Last Updated:** 2026-03-28
**Based on:** `removal.md`

---

## Test Runner Removal

### Status: PARTIAL (In Progress)

### Completed

- [x] `src/cli/test_command.zig` - DELETED
- [x] `src/cli/test/` directory - DELETED
- [x] `test/` directory - DELETED
- [x] `test/bundler/` directory - DELETED
- [x] `src/bun.js/test/expect/` directory - DELETED (~50 files)
- [x] `src/bun.js/test/timers/` directory - DELETED
- [x] `src/bun.js/test/diff/` directory - DELETED
- [x] `src/bun.js/test/bun_test.zig` - DELETED
- [x] `src/bun.js/test/Execution.zig` - DELETED
- [x] `src/bun.js/test/Collection.zig` - DELETED
- [x] `src/bun.js/test/Order.zig` - DELETED
- [x] `src/bun.js/test/ScopeFunctions.zig` - DELETED
- [x] `src/bun.js/test/expect.zig` - DELETED
- [x] `src/bun.js/test/snapshot.zig` - DELETED
- [x] `src/bun.js/test/DoneCallback.zig` - DELETED
- [x] `src/bun.js/test/debug.zig` - DELETED
- [x] `src/bun.js/test/diff_format.zig` - DELETED
- [x] `src/bun.js/test/test.zig` - DELETED
- [x] `src/js/node/test.ts` - DELETED (or moved to `test/` which is deleted)

### Incomplete - Critical Issues

- [ ] **BROKEN**: `src/bun.js/test/jest.zig` imports non-existent `FakeTimers.zig`
  - Either delete the file or create a stub
  - Affects: `bun.zig`, `Timer.zig`

- [ ] `src/bun.js/test/pretty_format.zig` - orphaned file, likely unused

### Incomplete - Remaining References

- [ ] `src/bun.zig:3136,3212` - References `bun.jsc.Jest.bun_test.FakeTimers.current_time`
- [ ] `src/bun.js/api/Timer.zig:37,667` - Imports and uses `FakeTimers`
- [ ] `src/bun.js/api/Timer/EventLoopTimer.zig:71,96,104,185,186,188` - References `BunTest`
- [ ] `src/bun.js/api/bun/js_bun_spawn_bindings.zig:958-1021` - Uses `BunTest.bunTestTimeoutCallback`
- [ ] `src/bun.js/VirtualMachine.zig:9,571,660,2229,2273,2274` - `isBunTest`, `loadEntryPointForTestRunner`
- [ ] `src/bun.js/Debugger.zig:302` - References `BunTestStatus`
- [ ] `src/ast.zig:526` - `.bun_test` tag
- [ ] `src/ast/Parser.zig:1299,1324,1337` - `.bun_test` parsing
- [ ] `src/bun.js/HardcodedModule.zig:392,399,409` - `bun_test_aliases`
- [ ] `src/bun.js/ConsoleObject.zig:171` - `runner.bun_test_root`
- [ ] `src/generated_perf_trace_events.zig:59-61` - `TestCommand.printCodeCoverage*` events
- [ ] `src/cli/Arguments.zig:454,657,664` - References `.TestCommand`

### Files to Delete (Remaining)

```
src/bun.js/test/jest.zig           # Orphaned, imports missing file
src/bun.js/test/pretty_format.zig  # Orphaned
```

---

## Bundler Removal

### Status: NOT STARTED

### Completed

- [x] `test/bundler/` directory - DELETED

### Incomplete - Entire Bundler Still Present

- [ ] `src/bundler/` directory (~41 files) - NEEDS DELETION
  - `bundle_v2.zig` (~5000 lines)
  - `LinkerContext.zig` (~2700 lines)
  - All `linker_context/` subdirectory files (~24 files)
  - See `removal.md` section 8.1 for full list

- [ ] `src/cli/build_command.zig` - NEEDS DELETION
- [ ] `src/bun.js/api/JSBundler.zig` - NEEDS DELETION

### Incomplete - Files Importing Bundler

There are **541+ references** to `bundle_v2`/`BundleV2` in the codebase. Major categories:

#### CSS Processing (Heavy Bundler Dependencies)

- [ ] `src/css/css_parser.zig` - Uses `bun.bundle_v2.Index`, `bun.bundle_v2.Ref`
- [ ] `src/css/css_internals.zig` - Uses `bun.bundle_v2.Index`
- [ ] `src/css/values/ident.zig` - Uses `bun.bundle_v2.Ref`
- [ ] `src/css/printer.zig` - Uses `bun.bundle_v2.Ref`, `bun.bundle_v2.Index`
- [ ] `src/css/` - Multiple files depend on bundler types

#### Package Manager Commands

- [ ] `src/cli/install_command.zig:28,45` - Uses `bun.bundle_v2.BundleV2.DependenciesScanner`
- [ ] `src/cli/create_command.zig:1650,1665` - Uses `bun.bundle_v2.BundleV2.DependenciesScanner`

#### Other Critical Files

- [ ] `src/transpiler.zig:724` - Uses `bun.bundle_v2.Index.invalid`
- [ ] `src/safety/alloc.zig:34` - Uses `bun.bundle_v2.allocatorHasPointer`
- [ ] `src/options.zig:1796` - Comment mentions `bundle_v2`
- [ ] `src/js_printer.zig:431` - Uses `bun.bundle_v2.MangledProps`
- [ ] `src/install/PackageManager/updatePackageJSONAndInstall.zig:700,718` - Uses `BundleV2.DependenciesScanner`
- [ ] `src/crash_handler.zig:111-117` - References `bun.bundle_v2.LinkerContext`
- [ ] `src/create/SourceFileProjectGenerator.zig` - Uses `BundleV2`

#### All `src/bundler/` files (Need deletion after updating dependents)

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
└── linker_context/         # ~24 files
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

---

## Cross-Cutting Tasks

### CLI Entry Point Updates

- [ ] `src/cli.zig` - Remove `BuildCommand` references (keep `TestCommand` removed)

### Module Export Updates

- [ ] `src/bun.zig` - Remove `bundle_v2` exports (line ~1757)
  ```zig
  // REMOVE:
  pub const bundle_v2 = @import("./bundler/bundle_v2.zig");
  pub const Loader = bundle_v2.Loader;
  pub const BundleV2 = bundle_v2.BundleV2;
  pub const ParseTask = bundle_v2.ParseTask;
  ```

### Build System Updates

- [ ] `CMakeLists.txt` - Remove bundler/test targets
- [ ] `package.json` - Remove test scripts
- [ ] `.github/workflows/*.yml` - Update CI/CD pipelines
- [ ] `scripts/` - Remove test/bundler scripts

### Stub Files to Create

- [ ] `src/test_runner_stub.zig` - Stub test runner types (if needed by runtime)
- [ ] `src/bundler_stub.zig` - Stub bundler types (for CSS processing)

---

## Files Requiring Audit

| File                                   | Issue                                  |
| -------------------------------------- | -------------------------------------- |
| `src/bun.js/api/filesystem_router.zig` | May have bundler deps                  |
| `src/OutputFile.zig`                   | May have bundler-specific types        |
| `src/router.zig`                       | May need bundler-specific code removed |
| `src/cache.zig`                        | Uses js_parser                         |

---

## Verification Checklist

After each phase:

### Build Verification

- [ ] `bun bd` completes successfully
- [ ] No linker errors about missing symbols
- [ ] No compiler warnings about unused imports

### Runtime Verification

- [ ] `bun --help` shows no test/build commands
- [ ] `bun -e "console.log('hello')"` works
- [ ] `bun file.js` executes JavaScript

### Package Manager Verification

- [ ] `bun install` works
- [ ] `bun add <package>` works
- [ ] `bun remove <package>` works

### CLI Verification

- [ ] `bun run <script>` works
- [ ] `bun x <package>` works
- [ ] `bun init` works
- [ ] `bun --help` shows expected commands

---

## Summary

| Component         | Status | Files Deleted | Files Remaining                          |
| ----------------- | ------ | ------------- | ---------------------------------------- |
| Test Runner (Zig) | ~80%   | ~55           | 2 (jest.zig, pretty_format.zig - broken) |
| Bundler (Zig)     | 0%     | 0             | ~41 + dependents                         |
| Test Framework    | ~90%   | Most          | 2 files, 541+ refs remain                |
| CLI Commands      | 50%    | 1/2           | build_command.zig remains                |

**Estimated Lines Removed:** ~350,000 (from removal.md)
**Estimated Lines Remaining:** TBD
