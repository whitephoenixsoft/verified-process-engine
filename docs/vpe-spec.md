# Verified Process Engine (VPE) Specification
Version: Canonical v1.2

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

### Registry
Maps identifiers to Rust implementations:
- Guard factories
- (optionally) effect descriptors

### Compiler
Validates and transforms a Law into a compiled process:
- enforces invariants
- resolves all identifiers
- generates manifests
- produces a deterministic digest

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

## 7. Compiler Pipeline

1. Parse Law
2. Schema validation
3. Identifier validation
4. Type validation
5. Graph construction (state indexing)
6. Topology validation (reachability, orphans)
7. Auto-transition cycle detection (`AUTO_TICK`)
8. Saga validation (transient state enforcement)
9. Guard/effect compilation (Registry binding)
10. Manifest generation (per-state requirements)
11. Digest generation (deterministic hash)

Compilation must fail on any invariant violation.

---

## 8. Design-Time Compilation

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

This mode does not require:
- runtime engine
- persistence layer
- external systems

---

## 9. Runtime Algorithm

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

---

## 10. Request Model

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

---

## 11. Verdict Model

A Verdict contains:

- process reference
- trace_id
- previous_state
- next_state
- state_patch (context mutations)
- effects (intent only)
- emitted events (planned, not persisted)

Properties:
- deterministic
- side-effect free
- suitable for persistence

The host is responsible for:
- atomic persistence of state + events
- execution of effects

---

## 12. Manifest System

For each state, the compiler produces a manifest describing:

- required history:
  - Anchor (always required)
  - event windows
  - specific event types
- required context:
  - specific `rec.*`, `ext.*`, `calc.*`, `sys.*` fields

Purpose:
- minimal data loading
- predictable execution cost
- explicit data dependencies

---

## 13. Migration

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

Properties:
- deterministic
- append-only (history preserved)
- produces a migration event

---

## 14. Simulation

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

## 15. Rust Crate Design

### Core Principle
Rust crate is first-class. FFI is a thin wrapper.

### Crate Structure (Conceptual)

- vpe
  - engine
  - compiler
  - runtime
  - registry
  - schema
  - types
  - migration (feature)
  - simulation (feature)

- vpe-ffi (optional)

### Feature Flags

- default: core engine
- simulation: enables simulation module
- migration: enables migration module
- ffi: enables C ABI
- serde: serialization

---

## 16. Public API Strategy

Expose:
- Guard trait
- Engine facade
- Compiler (design-time API)
- Request / Verdict types
- Registry builder
- Schema and Law source types

Hide:
- compiled graph structures
- node/edge layout
- indexing mechanisms

---

## 17. Extension Model

Users extend VPE by:

- implementing Guard trait
- registering via GuardRegistry

Optional:
- custom effect handling in host system

---

## 18. Performance Model

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

## 19. FFI Model

- opaque engine pointer
- JSON request/response
- explicit memory management
- no panics across boundaries

FFI must not dictate internal design.

---

## 20. Error Model

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