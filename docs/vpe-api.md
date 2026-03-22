# VPE Public Rust API
Version: Canonical Draft v1.1

## Purpose
This document defines the intended public Rust API for the Verified Process Engine (VPE). It is written from the perspective of an application developer embedding VPE as a library.

VPE is designed to be:
- Rust-first
- deterministic
- embeddable
- host-owned for I/O, persistence, and side effects
- usable as a stable business-logic boundary
- usable both at design time (compile/validate) and runtime (execute)

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
`use vpe::prelude::*;`

## Primary Public Concepts

### ProcessRef
Identifies a specific process version.

Fields:
- `domain`
- `process`
- `version`

Purpose:
- stable process identity
- avoids loose string tuples throughout the API

### ContextMap
A flattened key-value map of typed JSON values.

Canonical namespaces:
- `sys.*`
- `rec.*`
- `ext.*`
- `calc.*`

Purpose:
- host-facing runtime input boundary
- practical for adapters and serialization

### VpeEvent
Represents a historical event relevant to evaluation.

Expected fields:
- `event_id`
- `parent_event_id`
- `trace_id`
- `timestamp`
- `actor`
- `action`
- `event_kind`
- `was_successful`
- `state_before`
- `state_after`
- `metadata`

Purpose:
- anchor validation
- replayability
- lineage
- integration with event-sourced systems

### ChronicleView
Represents the history slice used during one engine turn.

Contains:
- `anchor`
- `events`

Purpose:
- makes the anchor explicit
- lets the host supply only the required history
- provides the proof of current state

### VpeRequest
The public unit of execution.

Fields:
- `process`
- `trace_id`
- `now`
- `current_state`
- `action`
- `context`
- `chronicle`

Purpose:
- represents one deterministic turn
- carries all required evaluation input explicitly

### VpeEffect
Represents an emitted effect envelope.

Suggested fields:
- `effect_type`
- `target`
- `action`
- `params`
- `handlers`

Purpose:
- lets VPE emit orchestration intent
- lets hosts dispatch effects externally

### PlannedEvent
Represents a host-persisted event planned by one engine turn.

Suggested fields:
- `event_kind`
- `action`
- `state_before`
- `state_after`
- `metadata`

Purpose:
- allows host persistence
- supports event-sourced systems
- preserves traceability

### VpeVerdict
Represents the output of one engine turn.

Suggested fields:
- `process`
- `trace_id`
- `transition_id`
- `previous_state`
- `next_state`
- `state_patch`
- `effects`
- `emitted_events`

Purpose:
- persistence-friendly
- orchestration-friendly
- event-store-friendly
- represents intent, not execution

### StateManifest
Describes the data required to evaluate one state.

Contains:
- `history_requirements`
- `context_requirements`

Purpose:
- lets the host fetch only the history and fields needed for evaluation

### GuardRequirements
Describes the total requirements declared by a Guard.

Contains:
- `history`
- `context`

Purpose:
- allows compiler-driven manifest generation
- keeps data dependencies explicit

## Public Traits

### Guard
Primary extension trait for application-defined logic.

Responsibilities:
- evaluate against context and history
- declare required data
- provide stable identity/name

A Guard must be:
- stateless after creation
- deterministic
- thread-safe
- side-effect free

A Guard should not:
- perform I/O
- read global state
- mutate external state

Suggested responsibilities:
- `check(context, history) -> bool`
- `requirements() -> GuardRequirements`
- `name() -> &'static str`

## Registry API

### GuardRegistry
Public registry for built-in and custom guards.

Responsibilities:
- collect built-in guards
- register custom guards
- support compilation of source guard definitions into executable guard instances

### GuardRegistryBuilder
Builder used during startup.

Expected flow:
- create builder
- add built-ins
- register custom guards
- build immutable registry

Purpose:
- make startup ergonomic
- keep runtime immutable and concurrency-safe

## Compiler API

### VpeCompiler
Public design-time compiler for validation and compilation.

Purpose:
- support authoring workflows
- support CI/CD validation
- support manifest inspection
- support future CLI tooling

Expected public methods:
- `validate`
- `compile`

### validate
Validates a schema + law combination without storing it in an engine.

Input:
- schema
- law
- registry

Output:
- `ValidationReport`

### compile
Compiles a schema + law combination into an in-memory compiled process and report.

Input:
- schema
- law
- registry

Output:
- `CompilationResult`

Purpose:
- design-time verification
- offline or CI use
- future tooling support

## Engine API

### VpeEngineBuilder
Used to construct the engine.

Responsibilities:
- accept registry
- later may accept stores/config options
- build ready-to-use engine facade

### VpeEngine
Primary facade for application code.

Expected public methods:
- `register_schema`
- `register_schema_json`
- `register_process`
- `register_process_json`
- `validate_process`
- `manifest`
- `execute`

Optional methods behind features:
- `simulate`
- `lift`

Purpose:
- provide the product boundary
- hide compiler/runtime/storage internals
- keep embedding experience simple

## Registration APIs

### register_schema
Registers a typed domain schema.

### register_schema_json
Registers schema from JSON.

### register_process
Registers a typed law source.

### register_process_json
Registers law from JSON.

### validate_process
Validates without storing.

Registration returns report types rather than just success/failure.

## Report Types

### SchemaReport
Confirms registered domain/version.

### ValidationReport
Returns warnings and process identity.

Purpose:
- design-time validation
- startup validation
- CI diagnostics

### RegistrationReport
Returns:
- process identity
- digest
- manifests
- warnings
- metadata

Purpose:
- lets hosts inspect compile outcome
- supports startup validation and observability

### CompilationResult
Returns:
- compiled process
- registration report

Purpose:
- design-time compilation
- future artifact tooling
- engine-independent validation path

## Execution API

### manifest
Looks up the data requirements for a process state.

Inputs:
- process reference
- state name

Output:
- `StateManifest`

Purpose:
- host loads only relevant history/context

### execute
Runs one deterministic engine turn.

Input:
- `VpeRequest`

Output:
- `VpeVerdict`

Execution responsibilities:
- validate anchor presence
- validate current-state consistency
- resolve candidate transitions
- evaluate guards in priority order
- return first matching transition
- return structured failure if none match

## Simulation API
Available behind feature flag.

### SimulationRequest
Carries:
- process
- initial state
- initial context
- historical events

### SimulationReport
Carries:
- trace id
- outcome
- original final state
- simulated final state
- divergence point
- reasons

Simulation outcomes:
- `Seamless`
- `Diverted`
- `Incompatible`

Purpose:
- pre-deployment review
- migration analysis
- debugging policy changes

## Migration API
Available behind feature flag.

### LiftRequest
Carries:
- target process
- current version
- current state
- context
- history

### LiftResult
Carries:
- target process
- previous state
- landing state
- transformed context
- appended migration event

Purpose:
- lazy record lifting
- deterministic evolution between law versions

## Error API

### VpeError
Top-level error enum.

Expected categories:
- compile
- runtime
- migration
- simulation
- schema
- process not found

### Runtime Errors
Expected structured cases:
- anchor missing
- desync
- no transition found
- unknown state

### Compile Errors
Expected structured cases:
- unresolved state
- unresolved guard/effect type
- invalid identifier/path
- schema mismatch
- type mismatch
- auto-transition cycle
- saga safety violation

Public errors should be:
- non-panicking
- deterministic
- suitable for logging and diagnostics

## Feature Philosophy
The Rust library is first-class. FFI is a thin wrapper.

Suggested feature model:
- default: core engine
- `simulation`
- `migration`
- `ffi`

The Rust API must remain ergonomic even when FFI is enabled.

## Public API Principles
1. One engine call represents one deterministic turn.
2. The host owns data loading, persistence, and side effects.
3. The engine owns policy validation and decisioning.
4. Public types should be practical to construct and serialize.
5. Internal optimizations must remain private.
6. Embedding VPE should reduce business-logic sprawl, not increase ceremony.
7. Design-time validation and compilation are first-class use cases.
8. The same logic model must power compile-time validation, runtime execution, and simulation.