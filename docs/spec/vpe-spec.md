# Verified Process Engine (VPE) Specification
Version: Canonical v1.6

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
- domain
- process
- version (law version)
- schema_version (explicit schema linkage)
- states
- transitions
- guards
- effects
- migration rules

### Domain Schema
Defines all valid typed fields grouped by namespace:

- `rec.*` → mutable record data
- `ext.*` → external input data
- `calc.*` → derived data

System fields (`sys.*`) are:
- reserved
- not user-defined
- provided by VPE or the host
- defined by a built-in system schema

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
- schema_version
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

## 5. Schema Model

### Structure

A Domain Schema contains:

- domain
- version
- namespaces:
  - rec
  - ext
  - calc

Each namespace defines a set of fields.

### Field Definition

Each field contains:

- name
- field_type
- optional enum_values
- optional description

### Field Types

Supported types include:

- String
- Number
- Boolean
- Enum

### Enum Rules

- `field_type = Enum` requires `enum_values`
- enum_values must be non-empty
- enum_values must be unique
- enum_values must follow identifier rules

### Identifier Rules

- alphanumeric and underscore only
- no spaces
- may not start with a digit

### Namespace Rules

- only `rec`, `ext`, and `calc` may be defined in schema
- `sys` is reserved and may not be defined by users

### System Schema (`sys.*`)

VPE provides a built-in system schema, including:

- `sys.now`
- `sys.trace_id`

Properties:
- read-only
- always considered valid during compilation
- must be explicitly provided at runtime when required

---

## 6. Guards

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

## 7. Effects

Effects are structured envelopes representing intent.

Effects are:
- emitted by runtime only
- not executed by VPE
- handled externally by the host

### Effect Classification

Effects are divided into two categories:

#### Tracked Effects
Tracked effects influence business correctness and must be explicitly resolved.

Examples:
- charging a payment
- reserving inventory
- creating shipments
- external approval workflows

Requirements:
- must transition into a transient (saga) state
- must define valid exit paths
- must be resolved via subsequent events

#### Untracked Effects
Untracked effects are best-effort side effects that do not affect core business correctness.

Examples:
- sending notification emails
- emitting analytics/telemetry
- cache invalidation
- background jobs

Properties:
- do not require transient states
- do not require outcome events
- do not block process progression

### Effect Fields

Typical fields:
- type
- target
- action
- parameters
- mode (`tracked` | `untracked`, default: `untracked`)
- optional success/failure/timeout handlers (tracked only)

---

## 8. Verdict Events (Planned Events)

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

## 9. Compiler Pipeline

1. Parse Law
2. Schema validation
3. Schema/Law compatibility validation
4. Identifier validation
5. Namespace validation
6. Type validation
7. Graph construction (state indexing)
8. Topology validation (reachability, orphans)
9. Auto-transition cycle detection (`AUTO_TICK`)
10. Saga validation (tracked effects only)
11. Guard/effect compilation (Registry binding)
12. Manifest generation (per-state requirements)
13. Manifest validation
14. Digest generation (deterministic hash)

Compilation must fail on any invariant violation.

### Schema/Law Compatibility Rules

- law.domain must equal schema.domain
- law.schema_version must equal schema.version

### Additional Guarantees

- all referenced guards must exist in the registry
- all referenced fields must exist in schema or system schema
- all guard requirements must be reflected in manifests
- missing or incomplete manifest coverage is a compilation error

### Manifest Validation Rules

- every guard requirement must be present in the manifest → error
- manifest requirements not used by any guard → warning
- redundant or excessive requirements should be minimized

---

## 10. Design-Time Compilation

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

## 11. Runtime Algorithm

Given a request:

1. Validate Anchor presence
2. Validate Anchor state matches requested current state
3. Resolve state index
4. Filter transitions by action
5. Sort by priority (deterministic)
6. Validate request data against the state manifest
7. Evaluate guards sequentially (short-circuit)
8. Select first matching transition
9. Apply transition
10. Repeat via `AUTO_TICK` while valid (bounded)
11. Produce Verdict
12. If no match for non-AUTO_TICK action, return deterministic error

Runtime guarantees:
- no side effects
- no external calls
- no mutation of inputs
- no dependency on registry
- manifest requirements are enforced before guard evaluation
- guards must not observe undeclared data dependencies

---

## 12. Request Model

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

## 13. Verdict Model

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

## 14. Manifest System

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

### Manifest Enforcement

The manifest is a runtime data contract, not merely documentation.

Runtime must validate that all required context and history declared by the manifest are present before guard evaluation.

Long-term, runtime should restrict guard-visible data to the manifest-approved slice, preventing guards from relying on undeclared dependencies.

---

## 15. Migration

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

## 16. Simulation

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

## 17. Future Evolution

### 17.1 Multi-Process Orchestration

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

### 17.2 Effect Model Evolution

The effect system is designed to support both:

- strict, saga-driven workflows (tracked effects)
- lightweight, fire-and-forget application behavior (untracked effects)

Future enhancements may include:
- richer effect typing
- delivery guarantees
- observability hooks
- integration contracts

---

## 18. Rust Crate Design

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

## 19. Public API Strategy

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

## 20. Extension Model

Users extend VPE by:

- implementing Guard trait
- registering via GuardRegistry (compile-time)

Optional:
- custom effect handling in host system

---

## 21. Performance Model

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

## 22. FFI Model

- opaque engine pointer
- JSON request/response
- explicit memory management
- no panics across boundaries

FFI must not dictate internal design.

---

## 23. Error Model

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