# Verified Process Engine (VPE) Invariants
Version: Canonical v1

## 1. Determinism Invariants
1. VPE execution is a pure function of explicit inputs only.
2. Given the same compiled process, request, context, and chronicle slice, VPE must produce the same verdict.
3. VPE must not read ambient wall-clock time, global mutable state, randomness, or external services during evaluation.
4. Any `sys.*` value must be explicitly provided or derived from provided inputs.

## 2. History and State Invariants
1. The Chronicle is the source of truth.
2. Current state is derivative and must be provable from history.
3. Every execution must include the latest transition event (Anchor).
4. If the Anchor’s `state_after` does not match the requested current state, execution must fail.
5. History is immutable.
6. Migrations append events and never rewrite history.

## 3. Law and Compilation Invariants
1. A Law is declarative and versioned.
2. The Compiler must reject invalid laws at compile time.
3. All states, transitions, guards, effects, and paths must resolve during compilation.
4. The Compiler must reject illegal auto-transition cycles.
5. The Compiler must enforce side-effect safety rules.
6. The Compiler must emit per-state manifests of data requirements.

## 4. Runtime Invariants
1. Runtime execution is state-machine-based.
2. Evaluation occurs against one state and one action.
3. Transitions are evaluated in deterministic priority order.
4. Guards are evaluated with implicit AND semantics.
5. First matching transition wins.
6. Runtime produces a Verdict and performs no side effects.

## 5. Namespace Invariants
1. Allowed namespaces: `sys.*`, `rec.*`, `ext.*`, `calc.*`.
2. `sys.*` is read-only.
3. Only `rec.*` may be mutated by transitions or migrations.
4. `ext.*` is read-only input.
5. `calc.*` is derived and non-authoritative unless persisted externally.

## 6. Identifier and Schema Invariants
1. All paths must use dot notation.
2. Segments must be alphanumeric or underscore.
3. Segments may not start with digits.
4. Field types are defined by Domain Schema and are immutable per version.
5. All referenced fields must exist in the schema.
6. All operations must be type-safe.

## 7. Auto-Transition Invariants
1. Auto transitions use action `AUTO_TICK`.
2. The `AUTO_TICK` subgraph must be acyclic.
3. Runtime auto-evaluation must have bounded depth.

## 8. Saga and Side-Effect Invariants
1. Transitions with effects are non-atomic.
2. Such transitions must land in transient states.
3. Transient states must define a timeout exit.
4. Runtime emits effects; host executes them.

## 9. Migration Invariants
1. Migration is lazy and deterministic.
2. Migration rules are evaluated in defined order.
3. Transforms may not write to `sys.*`.
4. Migration produces an appended event.

## 10. Interop Invariants
1. FFI is zero-trust.
2. Internal structures are not exposed.
3. JSON is used for cross-language data exchange.
4. All allocated memory crossing FFI must be explicitly freed.
5. No panics may cross FFI boundaries.

## 11. Concurrency Invariants
1. State and events must be persisted atomically.
2. Writes must be based on Anchor identity.
3. New events must reference the Anchor.

## 12. Simulation Invariants
1. Simulation replays history incrementally (prefix-based).
2. Simulation uses event timestamps as time source.
3. Simulation does not execute effects.
4. Simulation produces classified outcomes.