# Verified Process Engine (VPE) Invariants
Version: Canonical v1.2

## 1. Determinism Invariants
1. VPE execution is a pure function of explicit inputs only.
2. Given the same compiled process, request, context, and chronicle slice, VPE must produce the same verdict.
3. VPE must not read ambient wall-clock time, global mutable state, randomness, or external services during evaluation.
4. All system values (e.g., `sys.now`) must be explicitly provided as inputs or deterministically derived from them.
5. The same inputs must always produce identical transitions, effects, and emitted events.
6. Determinism must hold across environments, platforms, and executions.

## 2. History and State Invariants
1. The Chronicle is the source of truth.
2. Current state is derivative and must be provable from history.
3. Every execution must include the latest transition event (Anchor).
4. The Anchor must be explicitly provided as part of the Chronicle input.
5. If the Anchor’s `state_after` does not match the requested current state, execution must fail with a desync error.
6. History is immutable.
7. Migrations append events and never rewrite history.
8. The Chronicle provided must be consistent with the requested process version.

## 3. Law and Compilation Invariants
1. A Law is declarative and versioned.
2. The Compiler must reject invalid laws at compile time.
3. All states, transitions, guards, effects, and paths must resolve during compilation.
4. The Compiler must reject illegal auto-transition cycles.
5. The Compiler must enforce side-effect safety rules.
6. The Compiler must emit per-state manifests of required data (history and context).
7. The compiled representation must be deterministic and independent of runtime conditions.
8. Compilation must not depend on external state, I/O, or runtime data.

## 4. Runtime Invariants
1. Runtime execution is state-machine-based.
2. Each execution evaluates exactly one state and one action.
3. Execution represents a single deterministic "turn" of the engine.
4. Transitions are evaluated in deterministic priority order.
5. Guards are evaluated with implicit AND semantics.
6. Guard evaluation must short-circuit on failure.
7. First matching transition wins.
8. Runtime produces a Verdict or a deterministic error and performs no side effects.
9. Runtime must validate Anchor consistency before evaluation.
10. Runtime must not mutate input context or history.

## 5. Namespace Invariants
1. Allowed namespaces: `sys.*`, `rec.*`, `ext.*`, `calc.*`.
2. `sys.*` is globally read-only.
3. Only `rec.*` may be mutated by transitions or migrations.
4. `ext.*` is read-only input provided by the host.
5. `calc.*` is derived data and not authoritative unless persisted externally.
6. Namespace usage must be validated at compile time.

## 6. Identifier and Schema Invariants
1. All paths must use dot notation.
2. Segments must be alphanumeric or underscore.
3. Segments may not start with digits.
4. Field types are defined by Domain Schema and are immutable per version.
5. All referenced fields must exist in the schema.
6. All operations must be type-safe.
7. Schema validation must occur at compile time.
8. Type mismatches must result in compilation failure.

## 7. Auto-Transition Invariants
1. Auto transitions use action `AUTO_TICK`.
2. The subgraph formed by `AUTO_TICK` transitions must be acyclic.
3. Runtime auto-evaluation must have bounded depth.
4. Auto transitions must not depend on implicit external triggers.
5. Auto transitions must remain deterministic given the same inputs.

## 8. Saga and Side-Effect Invariants
1. Transitions with effects are non-atomic.
2. Such transitions must land in transient states.
3. Transient states must define at least one timeout or failure exit.
4. Runtime emits effects but does not execute them.
5. Effects represent intent and must be handled by the host system.
6. Every external effect must have defined success, failure, and timeout handling paths.

## 9. Migration Invariants
1. Migration is lazy and deterministic.
2. Migration rules are evaluated in defined order.
3. Transforms may not write to `sys.*`.
4. Migration produces an appended event.
5. Migration must result in a valid state in the target process version.
6. Migration must not violate schema or namespace invariants.
7. Migration must preserve determinism across versions.

## 10. Interop Invariants
1. The Rust API is first-class; FFI is a thin interoperability layer.
2. FFI is zero-trust.
3. Internal structures are not exposed across boundaries.
4. JSON is used for cross-language data exchange.
5. All allocated memory crossing FFI must be explicitly freed.
6. No panics may cross FFI boundaries.

## 11. Concurrency Invariants
1. State and events must be persisted atomically.
2. Writes must be based on Anchor identity (optimistic concurrency).
3. New events must reference the Anchor.
4. The engine itself remains stateless and thread-safe.
5. The engine must not maintain mutable shared execution state between requests.

## 12. Simulation Invariants
1. Simulation replays history incrementally (prefix-based).
2. Simulation uses event timestamps as the source of time.
3. Simulation does not execute effects.
4. Simulation produces classified outcomes (Seamless, Diverted, Incompatible).
5. Simulation must use the same deterministic logic as runtime evaluation.
6. Simulation must not mutate the provided history or context.

## 13. Manifest Invariants
1. Every state must have a deterministically derived manifest.
2. A manifest defines the minimal required:
   - history data
   - context fields
3. The manifest must always include the Anchor requirement.
4. The host must supply data consistent with the manifest before execution.
5. The manifest must be derived entirely at compile time.
6. Manifest requirements must be a complete superset of all guard dependencies.

## 14. Verdict Invariants
1. Every execution must produce exactly one verdict or a deterministic error.
2. A verdict must include:
   - previous state
   - next state
   - effects
   - emitted events
3. A verdict may include a state patch limited to `rec.*`.
4. A verdict represents intent, not execution.
5. The host must persist state changes and emitted events atomically.
6. All emitted events must maintain traceability (trace_id linkage).
7. The verdict must be fully derived from inputs and contain no hidden state.