# Verified Process Engine (VPE) Specification
Version: Canonical v1.4

## 1. Purpose
VPE is a deterministic process engine implemented in Rust. It compiles declarative laws into optimized structures and evaluates them against explicit inputs to produce decisions.

VPE separates:
- **Law** (what should happen)
- **Execution** (when it happens)

VPE is:
- deterministic
- verifiable at compile time
- replayable
- host-controlled for I/O and side effects

---

## 2. Execution Model

compile(Law, Schema, Registry) -> CompiledProcess  
evaluate(CompiledProcess, Request) -> Verdict

Execution is **turn-based**:
- each call evaluates exactly one state and one action
- produces one deterministic result (Verdict or Error)

Compiled processes are:
- immutable
- fully validated
- independent of the registry at runtime

---

## 3. Core Components

### Law
A declarative, versioned definition of:
- states
- transitions
- guards
- effects
- migration rules

### Domain Schema
Defines all valid typed fields for:
- `rec.*`
- `ext.*`
- `calc.*`

System fields (`sys.*`) are:
- reserved
- read-only
- provided explicitly by the host or derived deterministically

### Registry
Maps identifiers to Rust implementations:
- Guard factories
- (optionally) effect descriptors

Scope:
- used only during compilation
- not required during runtime execution

### Compiler
Validates and transforms a Law into a compiled process:
- enforces invariants
- resolves all identifiers
- binds guards from the registry
- generates manifests
- produces a deterministic digest

### CompiledProcess
An immutable artifact produced by the compiler.

Contains:
- fully resolved state machine
- bound guard instances
- manifests
- digest

Properties:
- safe for reuse across threads
- installable into runtime without recompilation

### Runtime
Evaluates a compiled process:
- enforces state and history invariants
- evaluates transitions deterministically
- produces a Verdict

### Migration Engine (optional)
Handles version upgrades via:
- rule selection
- guard evaluation
- deterministic transforms
- landing state resolution

### Simulation Engine (optional)
Replays historical execution:
- detects divergence
- classifies outcomes
- produces diagnostics

---

## 4. Canonical Law Structure

A Law contains:

- domain
- process
- version
- initial_state
- states[]
- migration_rules[]

Each state contains:
- name
- optional flags (e.g., transient)
- transitions[]

Each transition contains:
- action (string or `AUTO_TICK`)
- target state
- priority
- guards[]
- effects[]
- optional metadata/comment

---

## 5. Guards

Guards implement:

- check(context, history) -> bool
- requirements() -> declared dependencies

Guards are:
- stateless after construction
- deterministic
- side-effect free
- thread-safe

Responsibilities:
- evaluate business conditions
- declare data requirements for manifest generation

Constraint:
- Guard requirements must fully describe all data dependencies used during evaluation

---

## 6. Effects

Effects are structured envelopes representing intent.

Effects are:
- emitted by runtime only
- not executed by VPE
- handled externally by the host

Typical fields:
- type
- target
- action
- parameters
- optional success/failure/timeout handlers

---

## 7. Verdict Events (Planned Events)

In addition to effects, VPE produces **planned events** that represent the state transition and must be persisted by the host.

These events are distinct from effects:
- **effects** = external intent
- **events** = system-of-record history

### Event Structure

Each emitted event must include:

- trace_id
- event_kind (e.g., STATE_TRANSITION, MIGRATION)
- action
- state_before
- state_after
- timestamp (provided by host or sys.now)
- metadata (optional payload)

Optional:
- parent_event_id (for lineage)
- correlation identifiers

### Properties

- events are deterministic outputs of evaluation
- events must maintain traceability
- events must be persisted atomically with state

---

## 8. Compiler Pipeline

1. Parse Law
2. Schema validation
3. Identifier validation
4. Namespace validation
5. Type validation
6. Graph construction (state indexing)
7. Topology validation (reachability, orphans)
8. Auto-transition cycle detection (`AUTO_TICK`)
9. Saga validation (transient state enforcement)
10. Guard/effect compilation (Registry binding)
11. Manifest generation (per-state requirements)
12. Manifest validation
13. Digest generation (deterministic hash)

Compilation must fail on any invariant violation.

Additional guarantees:
- all referenced guards must exist in the registry
- all referenced fields must exist in the schema
- all guard requirements must be reflected in manifests
- missing or incomplete manifest coverage is a compilation error

### Manifest Validation Rules

- every guard requirement must be present in the manifest → error
- manifest requirements not used by any guard → warning
- redundant or excessive requirements should be minimized

---

## 9. Design-Time Compilation

VPE supports compilation independent of runtime execution.

### Purpose
- developer feedback during authoring
- CI validation
- law review and inspection
- manifest and dependency analysis

### Capabilities
- validate schema + law
- compile without registration
- produce:
  - validation report
  - compilation report
  - manifests
  - warnings
  - digest

### Output Artifacts
Compilation produces:
- CompiledProcess (in-memory)
- RegistrationReport
- State manifests
- Deterministic digest

CompiledProcess artifacts:
- may be cached or distributed
- may be installed into runtime without recompilation

---

## 10. Runtime Algorithm

Given a request:

1. Validate Anchor presence
2. Validate Anchor state matches requested current state
3. Resolve state index
4. Filter transitions by action
5. Sort by priority (deterministic)
6. Evaluate guards sequentially (short-circuit)
7. Select first matching transition
8. Produce Verdict
9. If no match, return deterministic error

Runtime guarantees:
- no side effects
- no external calls
- no mutation of inputs
- no dependency on registry

---

## 11. Request Model

A Request contains:

- process reference (domain, process, version)
- trace_id
- current_state
- action
- context (ContextMap)
- chronicle (Anchor + event slice)
- explicit time (e.g., `sys.now`)

Requirements:
- Anchor must be present
- Anchor state must match current_state
- data must satisfy manifest requirements
- all `sys.*` values must be explicitly provided or derived deterministically

---

## 12. Verdict Model

A Verdict contains:

- process reference
- trace_id
- previous_state
- next_state
- state_patch (context mutations)
- effects (intent only)
- emitted_events (planned, not persisted)

Properties:
- deterministic
- side-effect free
- suitable for persistence

### Host Responsibilities

The host must:

- persist state changes and emitted events atomically
- maintain event lineage and trace integrity
- execute effects separately
- ensure idempotency where required

---

## 13. Manifest System

For each state, the compiler produces a manifest describing:

- required history:
  - Anchor (always required)
  - event windows
  - specific event types
- required context:
  - specific `rec.*`, `ext.*`, `calc.*`, `sys.*` fields

Properties:
- fully derived at compile time
- deterministic
- complete with respect to all guard requirements

Purpose:
- minimal data loading
- predictable execution cost
- explicit data dependencies

---

## 14. Migration

Migration includes:

- version compatibility check
- rule selection
- migration guard evaluation
- transforms:
  - move
  - set
  - map
  - conditional transforms
- landing state resolution

Outputs:
- new state
- transformed context
- migration event

Properties:
- deterministic
- append-only (history preserved)
- must result in a valid state in the target process

### Host Responsibilities

- persist migration event
- persist transformed context atomically
- never rewrite historical events
- ensure continuity of trace_id and lineage

---

## 15. Simulation

Simulation:

- replays history incrementally (prefix-based)
- uses event timestamps as time source
- uses the same runtime logic
- does not execute effects

Outputs:
- Seamless
- Diverted
- Incompatible

Includes:
- divergence point
- diagnostic reasons

---

## 16. Future Evolution: Multi-Process Orchestration

VPE currently evaluates a single process per execution.

Future versions may introduce:
- process pipelines
- process-to-process handoff
- orchestration across multiple compiled processes

Current approach:
- host invokes VPE multiple times with different processes

Future direction:
- a higher-level process manager may coordinate multi-process flows

This is intentionally not part of the core runtime.

---

## 17. Rust Crate Design

### Core Principle
Rust crate is first-class. FFI is a thin wrapper.

### Crate Structure (Conceptual)

- vpe
  - engine
  - compiler
  - runtime
  - registry (compile-time only)
  - schema
  - types
  - migration (feature)
  - simulation (feature)

- vpe-ffi (optional)

### Feature Flags

- default: core engine
- simulation: enables simulation module
- migration: enables migration module
- ffi: enables C ABI (or separate crate)
- serde: serialization support (typically enabled)

---

## 18. Public API Strategy

Expose:
- Guard trait
- Engine facade
- Compiler (design-time API)
- CompiledProcess artifact
- Request / Verdict types
- Registry builder
- Schema and Law source types

Hide:
- compiled graph structures
- node/edge layout
- indexing mechanisms

---

## 19. Extension Model

Users extend VPE by:

- implementing Guard trait
- registering via GuardRegistry (compile-time)

Optional:
- custom effect handling in host system

---

## 20. Performance Model

Goals:
- sub-microsecond to low-microsecond decisions
- minimal allocations
- cache-friendly traversal

Strategies:
- index-based DAG (usize)
- precompiled guards
- short-circuit evaluation
- manifest-driven data loading

---

## 21. FFI Model

- opaque engine pointer
- JSON request/response
- explicit memory management
- no panics across boundaries

FFI must not dictate internal design.

---

## 22. Error Model

Errors must be:
- deterministic
- structured
- non-panicking

Categories:
- compile errors
- runtime errors (desync, no transition)
- migration errors
- simulation errors
- schema errors