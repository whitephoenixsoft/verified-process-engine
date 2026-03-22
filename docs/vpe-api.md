# VPE Public Rust API
Version: Canonical Draft v1.2

## Purpose
This document defines the intended public Rust API for the Verified Process Engine (VPE). It is written from the perspective of an application developer embedding VPE as a library.

VPE is designed to be:
- Rust-first
- deterministic
- embeddable
- host-owned for I/O, persistence, and side effects
- usable as a stable business-logic boundary
- usable both at design time (compile/validate) and runtime (execute)

---

## Design Goals
The public API should:
- make one engine turn feel natural
- expose extension points without leaking internals
- support web applications, event-sourced systems, and workflow/orchestration hosts
- support design-time validation and compilation
- avoid forcing FFI-shaped compromises onto Rust users

The public API should not expose:
- compiled graph internals
- arena/index storage details
- optimization passes
- runtime scheduling internals

---

## Crate Surface
The user-facing crate is `vpe`.

Suggested public modules:
- `prelude`
- `engine`
- `compiler`
- `registry`
- `schema`
- `types`
- `simulation`
- `migration`
- `error`

Typical import style:
use vpe::prelude::*;

---

## Primary Public Concepts

### ProcessRef
Identifies a specific process version.

Fields:
- domain
- process
- version

---

### ContextMap
A flattened key-value map of typed JSON values.

Namespaces:
- sys.*
- rec.*
- ext.*
- calc.*

---

### VpeEvent
Represents a historical event.

Fields:
- event_id
- parent_event_id
- trace_id
- timestamp
- actor
- action
- event_kind
- was_successful
- state_before
- state_after
- metadata

---

### ChronicleView
Represents the history slice for one execution.

Contains:
- anchor
- events

---

### VpeRequest
The unit of execution.

Fields:
- process
- trace_id
- now
- current_state
- action
- context
- chronicle

---

### VpeEffect
Represents an emitted effect.

Fields:
- effect_type
- target
- action
- params
- handlers

---

### PlannedEvent
Represents a host-persisted event.

---

### VpeVerdict
Represents execution output.

Fields:
- process
- trace_id
- transition_id
- previous_state
- next_state
- state_patch
- effects
- emitted_events

---

### StateManifest
Describes required data for a state.

Contains:
- history_requirements
- context_requirements

Invariant:
- Must include Anchor requirement
- Must fully cover all guard dependencies

---

### GuardRequirements
Describes dependencies declared by a Guard.

Contains:
- history
- context

---

### CompiledProcess
An immutable, validated, executable process artifact.

Properties:
- created only by the compiler
- contains fully bound guards
- contains manifests and digest
- does not expose internal graph structure

Methods:
- process_ref()
- digest()
- manifest(state)

---

## Public Traits

### Guard

Responsibilities:
- evaluate logic
- declare requirements
- expose identity

Methods:
- check(context, history) -> bool
- requirements() -> GuardRequirements
- name() -> &'static str

Constraints:
- deterministic
- stateless after construction
- thread-safe
- no I/O or global state

---

## Registry API

### GuardRegistry
Stores both built-in and custom guards.

Used only at compile time.

---

### GuardRegistryBuilder

Usage:
- with_builtins()
- register_guard(...)
- build()

---

## Compiler API

### VpeCompiler

The design-time compiler.

Owns a GuardRegistry internally.

Construction:

- VpeCompiler::with_builtins()
- VpeCompiler::builder()

---

### VpeCompilerBuilder

Methods:
- new()
- with_registry(registry)
- build()

---

### validate

validate(schema, law) -> ValidationReport  
validate_json(schema_json, law_json) -> ValidationReport

---

### compile

compile(schema, law) -> CompilationResult  
compile_json(schema_json, law_json) -> CompilationResult

---

### CompilationResult

Fields:
- process: CompiledProcess
- report: RegistrationReport

---

### Validation Behavior

The compiler must validate:

- schema correctness
- identifier structure
- namespace usage
- type safety
- guard existence
- manifest completeness
- auto-transition cycles
- saga safety
- unreachable/orphan states

Additional rules:

- guard requirements must fully cover dependencies
- missing manifest coverage must be an error
- suspicious empty requirements may produce warnings

---

## Engine API

### VpeEngineBuilder

Used to construct engine.

---

### VpeEngine

Primary runtime facade.

Methods:

- register_schema
- register_schema_json
- register_process
- register_process_json
- validate_process
- install
- manifest
- execute

Optional:
- simulate
- lift

---

### install

install(process: CompiledProcess) -> RegistrationReport

Behavior:
- validates artifact compatibility
- does not recompile
- stores process for execution

---

## Registration APIs

Typed + JSON supported for all:

- register_schema
- register_schema_json
- register_process
- register_process_json

---

## Report Types

### SchemaReport

---

### ValidationReport

---

### RegistrationReport

Fields:
- process identity
- digest
- manifests
- warnings
- metadata

---

## Execution API

### manifest

manifest(process, state) -> StateManifest

---

### execute

execute(request) -> VpeVerdict

---

## Simulation API (feature)

---

## Migration API (feature)

---

## Error API

### VpeError

Categories:
- compile
- runtime
- migration
- simulation
- schema
- process not found

---

## Feature Philosophy

- Rust API is primary
- FFI is thin
- features:
  - default
  - simulation
  - migration
  - ffi

---

## Public API Principles

1. One call = one deterministic turn
2. Compiler and runtime are separate
3. Registry is compile-time only
4. CompiledProcess is immutable and self-contained
5. Host owns I/O and persistence
6. Typed + JSON APIs are always paired
7. Manifest defines required data contract
8. Validation must fail early and loudly
9. Runtime must never depend on registry
10. Same logic model powers compile, runtime, and simulation