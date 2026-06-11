# Replace mlua and luau-analyze with oxau

Canopy embeds Luau twice: `mlua` (vendored at `vendor/mlua` with a local `require` bootstrap
patch) runs scripts, and `luau-analyze` (optional `typecheck` feature, disabled on macOS where it
conflicts with the embedded VM) type-checks them. Oxau (`~/git/private/oxau`) is a pure-Rust Luau
implementation whose umbrella crate covers both roles. Replacing both deps with oxau removes the
C++ toolchain build, the vendored mlua patch, the macOS typecheck exclusion, and the `typecheck`
feature flag.

Oxau's umbrella surfaces (verified against oxau main @ 8f8c281b1):

- `oxau::surface` — `SurfaceSpec`: the validated bundle of profile + audited native modules +
  checker declaration modules. One call each for `vm_builder(ambient, limits)`, `new_checker()`,
  and `compile(source, opts)`, so the checker, compiler, and runtime cannot drift apart.
- `oxau::session` — `Vm`, `VmBuilder`, `Limits`, `Deadline`, `Cancel`, `LoadedModule`,
  `RuntimeErrorKind`, `ProtectedScriptError`, the typed build/load/sandbox errors.
- `oxau::embed` — `NativeModule`, `ModuleBuilder`(+`ModuleBuilderExt`), `scoped_host_fn`,
  `Scope`, `ScopedValue`, `Stashed<…>`, `RuntimeError`, `ScriptError`, `Table`, `MarshaledValue`,
  and (under the `derive` feature) `#[derive(IntoLua/FromLua)]` for struct↔table conversion.
- `oxau::types` — `Checker`, `CheckerConfig`, `BuiltinDefinitionModule`, schema extraction.
- `oxau::compile` — `compile_for`, `restrict_compile_options`, structured `CompileError`.
- `oxau::syntax` — `parse_file`, `ParseError`/`ParseErrorKind` with `Location` for parse errors.
- `oxau::diagnostic` — `TypeDiagnostic`, `DiagnosticSeverity`, `DiagnosticLocation`.

A sync embedder takes `oxau = { default-features = false }` (drops the filesystem resolver). The
umbrella's "minimal embedding profile" pulls Tokio only with slim features and spawns no OS thread
unless a runner lane-pool or a `Cancel::after` watchdog is constructed — so canopy never needs the
`runner`/`durable` surfaces, and no async runtime to run scripts.

All canopy mlua usage lives in `crates/canopy/src/core/script/mod.rs` (~2270 lines) plus
`defs.rs`; no other file imports mlua. The migration is therefore contained, but the host model
differs: mlua creates functions/userdata dynamically on a live VM, while oxau registers native
modules at build time and brands heap handles to a `Scope`. Canopy's lifecycle fits — all commands
are known by `Canopy::finalize_api()` — but a few oxau gaps remain (below).

## Key API mapping (for zero-context pickup)

| canopy/mlua today (core/script/mod.rs)        | oxau replacement                                  |
| --------------------------------------------- | ------------------------------------------------- |
| `Lua::new_with(ALL_SAFE)` + `sandbox(true)`    | `surface.vm_builder(ambient, limits).build_sandboxed()`; profile `Profile::full().without_runtime_compilation()`, no `module_source` ⇒ no `require`. Per-chunk global isolation via `vm.bind_chunk_environment(&module)` |
| clone-then-drop GC workaround (mod.rs:1171)    | delete — no equivalent hazard                      |
| `set_interrupt` deadline guard (mod.rs:1196)   | always-on bound: `Limits.gas` (logical, enforced on the sync path). Explicit wall-clock timeout (MCP): `Limits.cancel = Cancel::after(timeout)` (detached watchdog thread). `Deadline::Wall` is **not** honored on the sync path |
| `lua.load(src).into_function()`                | `surface.compile(src, &CompileOptions)` → `vm.load_named(&chunk, name)`; structured `CompileError` (`CompileErrorKind::Parse` carries the location). VM-free, so available pre-finalize |
| `RegistryKey` + `ClosureRegistry` refcounts    | `scope.stash_function()` → `Stashed<marker::Closure>`; clones share one pin, last drop releases on the next VM step. Re-acquire with `scope.fetch_function` then `scope.call_protected` (from the top level: inside `vm.step_with_limits`) |
| ~35 `lua.create_function` base API entries     | one `NativeModule` (a `canopy` library via `ModuleBinding::Library("canopy")` + a `fixtures` `Global`), each entry a `scoped_host_fn`. Its `declaration()` must match what `build()` registers (surface audit, below) |
| per-owner command tables (mod.rs:1912)         | one `NativeModule` per owner; `declaration()` is that owner's `declare <owner>: {…}` block from `defs::render_definitions` |
| `ScriptNodeId` userdata (`__eq`, `__tostring`) | plain integer at runtime; `declare class NodeId` stays in a shared declaration module (oxau checker supports `declare class` — typecheck/builtins.rs:730) |
| `Value`/`MultiValue` ↔ `ArgValue`              | scope-borrowed `ScopedValue<'s>`/`MultiValue<'s>` in host fns (`Table::pairs`/`get`/`set` for map/array conversion); top-level results via `vm.call_protected` → `Vec<RawValue>`. Optional `#[derive(oxau::embed::IntoLua/FromLua)]` for struct↔table |
| `LuaError::SyntaxError` message parsing         | `oxau::syntax::parse_file` → `ParseError`/`ParseErrorKind` with `Location` |
| `luau_analyze::Checker` (mod.rs:1221)          | `surface.new_checker()` (`oxau::types::Checker`); `TypeDiagnostic` (category, severity, `primary_location` line/col, `payload`) → `ScriptCheckDiagnostic` |
| timeout string-prefix match (canopy-mcp)       | structured `RuntimeErrorKind::Deadline`/`Cancelled` |

### Surface declaration audit (new — shapes Stage Three)

`SurfaceSpec::build()` runs `validate_host_modules`: for every `NativeModule` it parses
`declaration()` and requires the declared bindings to **exactly** match what `build()` registers —
same globals, same library members, matching function/value/table kinds — and the declared types
must be well-formed (a function binding needs a function type, etc.) and must not bind a
profile-omitted library. A mismatch is a `RunnerConfigError::InvalidHostModuleDeclaration` at build
time, not a runtime surprise. This is a strong correctness gate but means canopy's generated
`.d.luau` (`defs::render_definitions`) and its registered command functions must line up per
module. Shared types (`NodeId`, `type NodeInfo`) declared in one declaration module are visible to
the others — definition modules are parsed as a combined set — so they can live in one base module
and be referenced from each owner's `declare` block.

### Verified oxau gaps (Stage One scope)

Each gap was re-verified against oxau main @ 8f8c281b1 in the Stage One worktree
(`~/git/private/oxau-canopy`, branch `canopy-embed`). The full designs, file:line anchors, and
tests are the Stage One items.

1. **Invoke a `LoadedModule` from inside a host call** (confirmed). `LoadedModule.main` is
   `pub(crate)` (oxau-vm/src/load.rs:51) and no `Scope` method runs a loaded module. Canopy needs
   this: `canopy.send_key` and `owner.default_bindings()` trigger *bound scripts* (compiled
   chunks) **while a script is already running**, and a host function holds only `&Scope` (the VM
   is already borrowed), so it cannot call `vm.call_protected`. Stored Luau **closures** are fine
   — `scope.fetch_function` + `scope.call_protected` already cover them. Fix: mint the branded
   handle (`Scope::module_function`), so the existing scope call paths cover modules too (item 1).
2. **Owned strings for runtime-built module surfaces** (confirmed). `NativeModule::name`/
   `declaration`, `ModuleBinding::Library`, `ModuleBuilder`/`ModuleBuilderExt` member names, the
   surface-audit error's `module` field, and `BuiltinDefinitionModule` are all `&'static str`
   (oxau-vm-api/src/lib.rs:488-718, oxau/src/surface.rs:66-68, oxau-typecheck/src/builtins.rs:23),
   but canopy generates owner names and the rendered declaration at runtime — and canopy-mcp
   rebuilds a fresh `Canopy` per request, so `Box::leak`-ing per build is an unbounded leak
   (item 2).
3. **Per-invocation limits on the protected/function paths** (confirmed). `Vm::call_with_limits`
   is public (oxau-vm/src/lib.rs:1636), but `call_protected_with_limits` (lib.rs:1672) and
   `call_function_with_limits` (lib.rs:2186) are `pub(crate)`. Canopy's live-eval timeout path
   needs a per-call protected run with a `Cancel`/gas override (item 3).
4. **Fresh limits for `Vm::step` (new — found during verification)**. `Vm::step` (lib.rs:1517)
   neither resets the spent-gas counter nor re-arms invocation limits, and the gas budget is a
   decrementing remainder (heap.rs:1782) armed only by the `call*` entry points — so a closure
   invoked through `step` runs on whatever budget the *previous* invocation left. Canopy invokes
   stashed key/mouse callbacks from the top level via `step` on a long-lived VM, which would drain
   toward spurious gas exhaustion. Fix: `Vm::step_with_limits` (item 4).

Closed since the first draft of this plan: **table iteration** (`Table::pairs` exists,
scope.rs:552) and **sync-without-tokio** (the documented minimal embedding profile;
`default-features = false`). Those Stage One items are dropped.

Re-entrancy note: canopy host functions reach canopy through the existing thread-local
`ScriptExecutionContext` stack (mod.rs:397-450, the raw `NonNull<Canopy>` pattern). The port
extends that stack to also carry the active `&Scope` (type-erased, same bounded-unsafe discipline)
so deep code under a host call (`commands::dispatch`, `execute_binding`) can re-enter the VM
through `scope.fetch_function`/`scope.call_protected` (closures) and the Stage-One
`Scope::module_function` (bound scripts), rather than the unavailable `&mut Vm`.

> DECIDED: canopy depends on oxau via a sibling path dep (`../private/oxau`), following the
> canopy-widgets → eguitty precedent. `default-features = false` drops the filesystem resolver
> (the umbrella's only features are `fsresolver` [default] and `derive`); add `derive` if the
> conversion derives end up used. Revisit a git dep or crates.io publication only if canopy needs
> standalone builds.

> DECIDED: the oxau gaps are fixed in oxau itself; the workarounds are each strictly worse. oxau
> is under active development (its main checkout sits on a dirty `burndown/robustness` branch), so
> Stage One is isolated in a dedicated worktree: `~/git/private/oxau-canopy`, branch
> `canopy-embed`, already created off oxau main @ 8f8c281b1. All Stage One work happens there
> under oxau's own commit gate; at stage exit the branch is rebased onto the then-current oxau
> main and merged to main. The four commits currently ahead of main on `burndown/robustness` touch
> `oxau-upstream`, typecheck tests, and `oxau-vm` driver/gc/builtins — disjoint from the files
> Stage One edits — so the rebase is expected to be clean.

1. Stage One: oxau groundwork (worktree `~/git/private/oxau-canopy`, branch `canopy-embed`)

Close the embedding gaps canopy needs. All file:line anchors verified against oxau main @
8f8c281b1 (the worktree's base). Each item lands as its own commit passing oxau's gate (`cargo
xtask tidy && cargo xtask test`). STAGE DONE: all four items landed and merged to oxau main @
cb0b7c4c4 (rebased over 13 intervening main commits; one surface.rs conflict resolved). Workflow
from here: after EVERY oxau commit, rebase `canopy-embed` onto main and merge into main
immediately — oxau is actively developed from other threads and must not diverge.

1. [x] Add `Scope::module_function(&self, module: &LoadedModule) -> Function<'s>`
       (oxau-vm/src/scope.rs): mint the scope-branded callable for a loaded module's main
       closure. `LoadedModule.main` stays `pub(crate)` (load.rs:51) — `Scope` lives in the same
       crate, and the host only ever receives the branded `Function<'s>`, so the brand still
       gates the handle. No new call path: the handle composes with the existing `Scope::call` /
       `Scope::call_protected` (nested dispatch under the active invocation's gas and
       `max_native_depth`) and `Scope::stash_function`. The module's registry pin keeps `main`
       rooted and `Vm::unload` consumes the module by value, so a dangling handle is
       unrepresentable; cross-VM misuse degrades to the same handle-validation error as
       `Vm::call_function` (lib.rs:2157) — document that threat model. Test: a `scoped_host_fn`
       runs another loaded module mid-script via `module_function` + `call_protected`; a
       recursive nest trips `max_native_depth` cleanly. DONE (oxau e47f6d782). Discovered during
       testing: `dispatch_host` *took the callable out of its registry slot* for the call, so a
       host function that re-entered the VM and recursively dispatched itself (exactly canopy's
       send_key → bound script → command path) failed with "host function is not registered".
       Fixed in the same commit: registry slots are shared `Arc<HostCallable>`s, the
       take-out/put-back panic-restore machinery is deleted.
2. [x] (DONE — oxau 933e9408e) Accept owned strings for runtime-built module surfaces. In
       oxau-vm-api/src/lib.rs:
       `NativeModule::name`/`declaration` return `&str` (borrowed from self — static impls
       coerce); `ModuleBuilder::{function,host_callable,constant,table}` take `name: &str` (the
       installer interns names into the heap, registry.rs:194-207, so nothing retains them);
       `ModuleBinding::Library` holds `Cow<'static, str>` (drops `Copy`, keeps `Clone`; ~49
       construction sites, eased by a `ModuleBinding::library(impl Into<Cow<'static, str>>)`
       helper); `ModuleTableEntry.name`/`ModuleTable::entry` take `Cow<'static, str>`; relax
       `From<&'static str> for ModuleValue` to `From<&str>` (it already copies). Thread through
       oxau-vm/src/registry.rs (`ModuleInstaller.module` and `ModuleInstallError.{module,member}`
       → `String`), oxau/src/embed.rs (`ModuleBuilderExt` name params → `&str`),
       oxau/src/surface.rs (`RunnerConfigError::InvalidHostModuleDeclaration.module` → `String`;
       the shape-audit names at surface.rs:473-533 and 711 → `String`), and
       oxau-typecheck/src/builtins.rs:23 (`BuiltinDefinitionModule.{name,source}` →
       `Cow<'static, str>`, keeping the `BUILTIN_DEFINITION_MODULES` const via `Cow::Borrowed`;
       the struct drops `Copy`). Update in-tree implementors: oxau/src/runner.rs, the four
       examples, oxau/tests/{reference_harness,runner_concurrency}.rs, and
       oxau-vm/src/execute/tests.rs. Test: a `NativeModule` whose name, declaration, and member
       names are runtime `String`s builds, installs, and passes the surface audit; an audit
       mismatch reports the runtime module name.
3. [x] (DONE — oxau 9900f6654) Make `Vm::call_protected_with_limits` (oxau-vm/src/lib.rs:1672) and
       `Vm::call_function_with_limits` (lib.rs:2186) `pub`, mirroring the already-public
       `call_with_limits` (lib.rs:1636), with `# Errors` docs. The umbrella's `session` module
       re-exports `Vm` itself, so no re-export changes. Test: a per-call `Cancel`/gas override on
       the protected path overrides the builder default, and the default is restored for the next
       invocation.
4. [x] (DONE — oxau cb0b7c4c4) Add `Vm::step_with_limits(limits, f) -> Result<R, RuntimeError>` (new
       gap, found during verification): `Vm::step` (lib.rs:1517) neither resets the spent-gas
       counter nor re-arms invocation limits, so a closure invoked through `step` runs on the
       decrementing budget the previous `call*` left behind (heap.rs:1782) — canopy's top-level
       key/mouse callback invocations on a long-lived VM would drain toward spurious gas
       exhaustion. Mirror `call_with_limits`'s arming around the existing step body: overlay onto
       the builder defaults, `reset_gas_spent`, `apply_invocation_limits` before `f`;
       `apply_default_limits` after (unless poisoned). Plain `step` keeps its value-plumbing
       semantics unchanged. Test: two closure invocations through `step_with_limits` each get the
       full gas budget; a per-step `Cancel` override is honored; builder defaults are back for
       the next `call`.

2. Stage Two: dependency switch and typecheck port (canopy) — DONE

Bring oxau in and replace `luau-analyze` first — the checker port is independent of the VM port,
kills the macOS exclusion immediately, and is low-risk.

1. [x] Add the oxau dependency to `crates/canopy/Cargo.toml`. NOTE: the dep points at the
       worktree (`../private/oxau-canopy/crates/oxau`) for now — the main oxau checkout sits on
       `burndown/robustness`, which predates the Stage One merge. Flip to `../private/oxau` in
       Stage Four once that checkout includes oxau main @ cb0b7c4c4. `default-features = false`;
       the `derive` feature was not needed. mlua kept in place; both coexist during this stage.
2. [x] Port `check_script_with_definitions` (mod.rs:1221-1257) to `oxau::types::Checker`. The
       cleanest route is to build the `SurfaceSpec` (Stage Three needs it anyway) and call
       `surface.new_checker()`; the interim route is `Checker::with_builtins` over
       `builtin_environment_for_with_definition_modules` fed the rendered declaration as
       `BuiltinDefinitionModule`s plus the preamble. Map `TypeDiagnostic` (category,
       `severity`, `primary_location` begin line/col) → `ScriptCheckDiagnostic`. Preserve the
       `--!strict` prefix of `strict_source` (or set `CheckerConfig.source_mode_override`).
3. [x] Remove `luau-analyze` from `crates/canopy/Cargo.toml`, delete the `typecheck` feature, the
       `#[cfg(all(feature = "typecheck", not(target_os = "macos")))]` gates, and the
       `ScriptCheckResult::unavailable` shim (mod.rs:1259-1269) — checking is now unconditional.
       Re-evaluate whether the `timed_out`/`cancelled` fields on `ScriptCheckResult` still apply
       (oxau's bounded checker can report admission/cancellation; confirm the mapping).
4. [x] Update consumers of the feature: `crates/canopy-mcp/Cargo.toml` feature list, the macOS/
       non-macOS test splits in canopy-mcp (script.rs:708-740) and mod.rs tests, and xtask docs.
       Also fixed pre-existing drift against the sibling tmcp checkout (`mark_as_error` →
       `with_is_error(true)`) and canopyctl clippy warnings, both blocking validation.
5. [x] Validate: `cargo nextest run --all --all-features` (434 passed), clippy clean, fmt.
       Found during validation: the oxau checker (correctly) rejects passing a *named-args
       table* to a typed owner function whose declaration takes positional params — e.g.
       `script_target.set_optional({ count = 14 })` against `(count: number?) -> ()`. The old
       luau-analyze checker let this slide. The named-table convention remains supported at
       runtime and through the untyped `canopy.cmd(name, { ... })` path; the two test scripts
       using it through typed owner functions were routed through `canopy.cmd` instead
       (test_script_commands.rs:101, canopy-mcp script.rs:667). `ScriptCheckResult` lost its
       always-false `timed_out`/`cancelled` fields and the `unavailable` constructors;
       `is_ok()` is now just `!has_errors()`. timed_out/cancelled removal is reflected in
       `format_typecheck_diagnostics`; docs/scripting.md updated.

3. Stage Three: VM port (crates/canopy/src/core/script/mod.rs) — DONE

Replace the mlua VM with an oxau retained session. This is the large change; land it as one
coherent changeset that keeps every existing script test green.

1. [x] Restructure `LuauHost` lifecycle around `SurfaceSpec`: pre-finalize, collect native-module
       builders (base `canopy`+`fixtures` module, per-owner command modules) with declarations
       that exactly match their registrations (surface audit); `finalize()` builds the
       `SurfaceSpec`, then `surface.vm_builder(ambient, Limits { gas, max_memory_bytes, .. })
       .build_sandboxed()`. `compile()` stays available pre-finalize via `surface.compile` /
       `compile_for`; execution before finalize becomes an explicit error.
2. [x] Port the base API (`register_base_api`, mod.rs:1325-1889) to one `NativeModule`: a `canopy`
       library plus the `fixtures` global, each entry a `scoped_host_fn` reaching canopy through
       the thread-local `with_current_canopy`. Its `declaration()` is the preamble's `declare
       canopy: {…}` + `fixtures` + shared `declare class NodeId` / `type NodeInfo`.
3. [x] Port command registration (`register_commands`, mod.rs:1912-1994) to per-owner native
       modules (keyword-suffix rule `luau_global_owner_name` unchanged); each module's
       `declaration()` is that owner's rendered `declare` block. Keep `dispatch_command` and the
       `ArgValue` arg-building logic (`build_args_from_values`), reimplementing the value
       conversions over `ScopedValue`/`Table::pairs`.
4. [x] Replace `ScriptCache`/`RegistryKey` plumbing: scripts cache `LoadedModule` (+ source);
       stored Luau closures become `Stashed<marker::Closure>`. Delete the refcount and deferred-
       sweep machinery in `ClosureRegistry` if `Stashed`'s release-on-next-step semantics cover the
       unbind-during-callback case (verify against the nested-callback tests in
       test_script_framework.rs).
5. [x] Extend `ScriptExecutionContext`/`SCRIPT_GLOBAL` (mod.rs:397-450) to carry the active
       `&Scope` (type-erased) so nested execution (send_key → bound script, `default_bindings()`,
       key/mouse callbacks) routes through `scope.fetch_function`/`scope.call_protected` (closures)
       and the Stage-One `Scope::module_function` (bound scripts). Top-level eval uses
       `vm.call_protected`; a top-level closure invocation (a binding firing outside any script)
       runs inside `vm.step_with_limits` so every callback gets a fresh gas budget.
6. [x] Port execution entry points (`execute_value_inner`, mod.rs:2040-2063): `vm.call_protected`
       for the default path; per-call timeout via the now-public `call_protected_with_limits` with
       `Limits.cancel = Cancel::after(ms)` (drop `ScriptInterruptGuard`); convert `Vec<RawValue>`
       results to `ArgValue`. Per-chunk global isolation via `bind_chunk_environment` to match
       mlua `sandbox(true)` (verify smoke scripts relying on cross-statement globals still pass).
7. [x] Replace `ScriptNodeId` userdata with a plain integer runtime value; keep `declare class
       NodeId end` in the shared declaration; drop the `UserData` impl and
       `create_userdata`/`borrow` paths (mod.rs:152-165, 561-576).
8. [x] Map errors: script failures → `error::Error::Script` carrying `RuntimeErrorKind` +
       traceback (from `ProtectedScriptError`/`ScriptError`); syntax errors via
       `oxau::syntax::parse_file` → `error::ParseError` (replaces `format_parse_error` string
       parsing); add a structured timeout variant from `RuntimeErrorKind::Deadline`/`Cancelled` so
       canopy-mcp stops string-matching `SCRIPT_TIMEOUT_PREFIX`. Delete the GC clone/drop
       workaround (mod.rs:1171-1180).
9. [x] Validate: all script tests (mod.rs unit tests, test_script_commands.rs,
       test_script_framework.rs, inputmap tests, widget Luau tests, canopy-mcp tests, todo smoke
       suite via `cargo xtask smoke`) pass; clippy clean; fmt.

4. Stage Four: downstream cleanup and removal — DONE (except the dep-path flip, item 5)

1. [x] Update canopy-mcp for the structured timeout error: canopy gained
       `error::Error::ScriptTimeout { timeout_ms }` (Display keeps the old "script evaluation
       exceeded …ms" text, so canopyctl expectations are unchanged); canopy-mcp's
       `SCRIPT_TIMEOUT_PREFIX` string-match is gone, `is_script_timeout` matches the variant.
2. [x] Removed the mlua dependency from `crates/canopy/Cargo.toml`, deleted `vendor/mlua/` and
       the workspace `[patch.crates-io]` entry, refreshed `Cargo.lock`. NOTE: mlua (crates-io,
       unpatched) still appears in the lockfile transitively via canopy-widgets → eguitty `itty`;
       removing that is eguitty's concern, but canopy itself no longer compiles any C++.
3. [x] Updated docs: `docs/scripting.md` (oxau VM, surface audit, sandbox/chunk environments,
       gas + memory ceilings, wall-clock watchdog timeouts, unconditional typechecking). No
       mlua/luau-analyze references remained in `docs/architecture.md`, `docs/fixtures.md`, or
       `plans/roadmap.md`.
4. [x] Full sweep: clippy clean (0 warnings), fmt applied, `cargo nextest run --all
       --all-features` 434/434 passed, `cargo xtask smoke` all 6 scripts pass. Script-execution
       performance: the todo smoke suite ran 0.73-0.81s under mlua (Stage Two runs) and
       0.81-0.85s under oxau (debug builds) — comparable for TUI automation workloads.
       `crates/canopy/benches` is an unrelated in-flight core-layout bench (not script-related,
       not yet wired as a bench target), so it was left alone.
5. [ ] Flip the oxau dependency path from `../private/oxau-canopy/crates/oxau` (the Stage One
       worktree) to `../private/oxau/crates/oxau` once the main oxau checkout includes oxau main
       @ cb0b7c4c4 (it currently sits on the `burndown/robustness` branch, which predates the
       Stage One merge). One-line change in `crates/canopy/Cargo.toml`; the worktree can be
       removed afterwards (`git -C ~/git/private/oxau worktree remove ~/git/private/oxau-canopy`).
