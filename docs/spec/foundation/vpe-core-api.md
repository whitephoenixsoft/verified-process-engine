# VPE Core API
Version: Canonical Draft v1.3

## Purpose

This document defines the Core API for the Verified Process Engine (VPE).

The Core API is the canonical semantic API for VPE. It defines:

- compilation semantics
- runtime evaluation semantics
- version-aware truth
- request and verdict primitives
- migration/lift vocabulary
- semantic patch intent
- event/effect intent
- host-owned persistence boundaries
- host-owned effect execution boundaries

The App API wraps Core API concepts for ergonomic host use.

The Process Evolution API uses Core API concepts for optional simulation, replay, explicit migration, diagnostics, and release-readiness workflows.

---

## Core Position

VPE is a deterministic decision brain.

VPE evaluates explicit supplied truth and returns semantic intent.

VPE does not:

- fetch hidden data
- inspect host storage
- update databases
- execute effects
- require event sourcing
- require VPE-native event persistence

The host supplies truth.

VPE evaluates.

The host commits reality.

---

## Foundation Invariants

### Version Is Part Of Truth

A process instance is not only in a state.

It is in a state under a process version.

Runtime correctness depends on version truth.

### Supplied Truth Principle

VPE evaluates supplied truth, not hidden reality.

If the host supplies incorrect state, context, history, anchor, or version information, VPE evaluates that supplied truth. VPE does not independently verify host storage.

### Semantic Intent Principle

VPE does not mutate reality.

VPE returns semantic intent, including:

- verdicts
- events
- effects
- semantic patches
- lift events

The host decides how reality changes.

### Event-Compatible, Not Event-Sourced

VPE may participate in event-sourced systems, but it is not an event-sourcing framework.

Valid host architectures include:

- simple CRUD applications
- relational databases
- document databases
- event-driven systems
- event-sourced systems
- hybrid systems

VPE-native events are semantic outputs, not mandatory persistence infrastructure.

### Progressive Capability Principle

The simplest lawful VPE execution remains valid.

A simple host may use VPE with only:

- current state
- action
- context

Advanced capabilities such as history-aware guards, migration, simulation, replay, audit, effects, and lineage layer on top.

Capabilities extend VPE.

They do not redefine VPE.

### Identity Preservation Through Migration

Migration may change:

- version
- state representation
- schema shape
- semantic interpretation

Migration must not change process instance identity unless explicitly modeled as a separate process instance.

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

Common namespaces:

- sys.*
- rec.*
- ext.*
- calc.*

---

### VpeEvent

Represents a VPE semantic event.

A VPE event may be persisted, translated, summarized, or discarded by the host.

Fields may include:

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

Represents the supplied history slice for one execution.

Contains:

- anchor
- events

VPE evaluates only the supplied chronicle.

It does not fetch history.

---

### VpeRequest

The unit of runtime execution.

Fields:

- process
- trace_id
- now
- current_version
- target_version
- current_state
- action
- context
- chronicle
- anchor
- lift_strategy, when migration capability is enabled

Meanings:

- current_version = version of supplied process instance truth
- target_version = version of the process/law being evaluated against

---

### VpeEffect

Represents emitted effect intent.

Fields:

- effect_type
- target
- action
- params
- handlers

Effects are not executed by VPE.

The host dispatches effects if it accepts the verdict.

---

### SemanticPatch

Represents transformation intent.

A semantic patch is not:

- storage
- persistence
- a database update
- final stored truth

The semantic patch is instruction.

The host commit is reality.

Detailed transform behavior belongs to the VPE Transform Architecture and Built-in Transform Catalog.

---

### LiftEvent

Represents migration lineage intent.

A lift event proves:

- source version
- target version
- source state
- target state
- selected lift path
- matched migration rule
- lawful version crossing occurred

A lift event is lineage.

It is not persistence itself.

---

### PlannedEvent

Represents event intent returned to the host.

The host decides whether to persist it as a VPE event, translate it into a domain event, store it as audit metadata, or discard it.

---

### VpeVerdict

Represents runtime output.

Fields may include:

- process
- trace_id
- transition_id
- previous_state
- next_state
- state_patch
- effects
- emitted_events
- lift_outcome
- warnings
- errors

A verdict must distinguish:

- no migration was required
- migration occurred and decision continued
- migration failed and decision did not execute
- migration succeeded but decision failed or was rejected

Migration success does not imply decision success.

---

## Manifest Model

### StateManifest

Describes required data for a state.

A manifest is unified at the container level but phase-segregated.

Contains:

- lift_requirements
- decision_requirements

### Lift Requirements

Lift requirements are source-version requirements.

They describe old-version truth required to lawfully lift supplied truth into target-version meaning.

They may include:

- source version
- target version
- source state
- migration guard requirements
- transform input requirements
- source schema paths
- history requirements
- anchor requirements

### Decision Requirements

Decision requirements are target-version requirements.

They describe truth required to evaluate decisions under the target process version.

They may include:

- target version
- current state
- action
- decision guard requirements
- context requirements
- history requirements
- anchor requirements
- explicit system values such as now

### Manifest Rule

Lift converts supplied old truth into lawful target-version truth for decision evaluation.

Decision manifests should not require old-version fields.

Lift manifests may require old-version fields.

---

## Runtime Evaluation Sequence

Runtime evaluation follows this conceptual order:

1. Validate request shape.
2. Resolve process and target version.
3. Compare current_version and target_version.
4. If versions differ, require migration capability.
5. Resolve lawful lift path.
6. If lift fails, return deterministic lift-aware result.
7. If lift succeeds, evaluate decision against target-version truth.
8. Return verdict, events, effects, patches, and lineage intent.
9. Host commits or discards.

Decision evaluation only occurs against truth valid for the target version.

---

## Runtime Lift Rule

If current_version differs from target_version, VPE must not evaluate the normal transition directly.

VPE must either:

- produce a complete deterministic lawful lift path
- or return a deterministic lift failure

VPE must never:

- guess
- ignore version mismatch
- silently reinterpret older truth
- silently repair
- partially lift during normal runtime execution
- force incompatible truth forward

---

## Lift Concepts

### LiftStrategy

LiftStrategy values:

- Direct
- Stepwise
- PreferDirectThenStepwise

Meanings:

- Direct: use direct migration support from source version to target version.
- Stepwise: migrate through intermediate versions.
- PreferDirectThenStepwise: attempt direct first, then stepwise if direct is unavailable.

The selected strategy must produce deterministic behavior.

### LiftStatus

LiftStatus values:

- NoMigrationRequired
- Direct
- Lifted
- NoMigrationPath
- AmbiguousMigrationPath
- Incompatible

Meanings:

- NoMigrationRequired: current_version == target_version; no lift path evaluated.
- Direct: version changed, but no semantic transformation was required.
- Lifted: version changed and semantic transformation occurred.
- NoMigrationPath: no migration definition/path exists.
- AmbiguousMigrationPath: multiple valid paths or rules exist and deterministic selection is impossible.
- Incompatible: path exists, but lawful migration cannot be completed.

Do not reuse simulation vocabulary here.

Simulation vocabulary belongs to Process Evolution API.

### Migration Errors

Migration errors are distinct from migration statuses.

Examples:

- UnknownSourceVersion
- UnknownTargetVersion
- MigrationRuleInvalid
- RequiredDataMissing
- MigrationNotConfigured
- TransformFailed
- TypeMismatch
- SchemaMismatch
- MissingRequiredContext
- MissingRequiredHistory
- MissingRequiredAnchor

Status answers what happened.

Error answers why evaluation failed or could not proceed.

### LiftOutcome

LiftOutcome is the canonical Core API result of version crossing.

Suggested fields:

- status
- from_version
- to_version
- from_state
- to_state
- strategy
- path
- semantic_patch
- lift_event
- warnings
- incompatibility_reason
- errors

### LiftPath

LiftPath records how truth moved from source version to target version.

Fields:

- from_version
- to_version
- strategy
- steps

Invariant:

A lift path must be complete and deterministic.

If any required step is missing, ambiguous, invalid, or incompatible, the lift fails.

### LiftStep

Each LiftStep includes:

- from_version
- to_version
- from_state
- to_state
- matched migration rule reference
- semantic patch
- lift event
- warnings

A stepwise lift is valid only if all steps are valid.

Partial paths are invalid for normal runtime execution.

### MigrationRuleReference

Identifies the migration rule used.

Suggested fields:

- source_version
- source_state
- target_version
- target_state
- rule_id or rule_index
- law_digest, when available

Purpose:

- audit
- explainability
- debugging
- lift path reconstruction

---

## Capability Boundary

Migration/lift support may be exposed through an optional Rust feature or capability.

Feature gating does not move semantic ownership out of Core API.

Core API owns the migration/lift vocabulary and result semantics.

Process Evolution API owns higher-level migration workflows, simulation, replay, diagnostics, and reporting.

If migration capability is not enabled and current_version != target_version, execution must not continue silently.

The result must be explicit, such as:

- MigrationNotConfigured

---

## Explicit Lift Operation

Core API defines the primitive semantics of:

lift(input) -> LiftOutcome

Ownership boundary:

- Core API defines the contract.
- Process Evolution API enables or provides the actual lift capability as an optional extension.
- App API may wrap lift through a host-friendly interface.

Core defines the contract.

Evolution provides the capability.

App exposes the convenience.

---

## CompiledProcess

An immutable, validated, executable process artifact.

Properties:

- created only by the compiler
- contains fully bound guards
- contains manifests and digest
- may contain migration rules
- may contain transform requirements
- may contain supported source-version metadata when derivable
- does not expose internal graph structure

Methods:

- process_ref()
- digest()
- manifest(state)

---

## Registry Concepts

### GuardRegistry

Stores both built-in and custom guards.

Used at compile time.

Guard Architecture owns guard registry lifecycle, validation, manifest derivation, and runtime evaluator rules.

### TransformRegistry

Stores built-in and custom transforms.

Used for migration/lift compilation and runtime transform construction.

Transform Architecture owns transform registry lifecycle, validation, output writes, runtime transform construction, and custom transform extension rules.

Core API may expose transform-facing concepts but should not duplicate transform architecture.

---

## Compiler API

### VpeCompiler

The design-time compiler.

Construction:

- VpeCompiler::with_builtins()
- VpeCompiler::builder()

### validate

validate(schema, law) -> ValidationReport

validate_json(schema_json, law_json) -> ValidationReport

### compile

compile(schema, law) -> CompilationResult

compile_json(schema_json, law_json) -> CompilationResult

### CompilationResult

Fields:

- process: CompiledProcess
- report: RegistrationReport

### Validation Behavior

The compiler must validate:

- schema correctness
- identifier structure
- namespace usage
- type safety
- guard existence
- transform existence when migration capability is enabled
- manifest completeness
- auto-transition cycles
- saga safety
- unreachable/orphan states

Additional rules:

- guard requirements must fully cover dependencies
- transform requirements must fully cover dependencies
- missing manifest coverage must be an error
- suspicious empty requirements may produce warnings

---

## Engine API

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

Feature-gated extension methods may include:

- simulate
- lift

### install

install(process: CompiledProcess) -> RegistrationReport

Behavior:

- validates artifact compatibility
- does not recompile
- stores process for execution

Runtime or registry may need access to multiple compiled process versions for stepwise lift.

---

## Execution API

### manifest

manifest(process, state) -> StateManifest

### execute

execute(request) -> VpeVerdict

One call is one deterministic turn.

---

## Atomic Commit Guidance

Core API returns intent.

Core API does not commit.

If lift and decision occur together, host persistence must be atomic across:

- semantic patch
- lift event
- target version update
- normal decision events
- normal state/context changes
- anchor/version advancement

Partial lift persistence is invalid.

The host must commit all accepted intent or discard the verdict.

---

## Event Persistence Boundary

Hosts may:

- store VPE events
- translate VPE events into domain events
- store audit records
- store metadata
- discard VPE events

If the host discards VPE events or lineage, future replay, audit, migration traceability, or history-aware guards may require equivalent truth to be supplied through other means.

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

Migration incompatibility is often a lawful process result, not an engine crash.

Do not collapse all lift failure into generic execution failure.

---

## Feature Philosophy

Suggested features:

- default
- simulation
- migration
- ffi

Rules:

- Rust API is primary.
- FFI is thin.
- Normal runtime decisioning belongs to the base.
- Simulation and migration are optional capability families.
- Optional features may extend Core concepts but must not redefine Core semantics.

---

## Relationship To App API

The App API wraps Core API for normal application use.

App API may expose:

- DecisionInput
- DecisionOutcome
- RequiredData
- PersistencePlan
- EffectPlan
- ProcessHandle
- builder APIs

Canonical meaning comes from Core API.

App API surfaces lift outcomes; it does not define lift semantics.

---

## Relationship To Process Evolution API

Process Evolution API uses Core API migration/lift concepts for:

- explicit migration
- simulation
- replay
- batch migration
- repair workflows
- release readiness
- diagnostics and reporting

Process Evolution API owns capability orchestration.

Core API owns semantic vocabulary.

---

## Relationship To Guard Architecture

Guard Architecture defines:

- guard registry lifecycle
- guard validation
- requirement derivation
- runtime guard evaluation
- custom guard extension model

Core API may expose guard-facing concepts, but should not duplicate Guard Architecture.

Guards decide applicability.

They do not mutate state, execute effects, fetch data, or own persistence.

---

## Relationship To Transform Architecture

Transform Architecture defines:

- transform registry lifecycle
- transform validation
- requirement derivation
- runtime transform execution
- output writes
- custom transform extension model
- semantic patch generation

Core API may expose transform-facing concepts, but should not duplicate Transform Architecture.

Transforms define lift.

They do not persist data, execute effects, fetch hidden host data, or mutate reality.

---

## Non-Responsibilities

Core API does not:

- update databases
- rewrite records
- execute effects
- perform host commits
- require event sourcing
- own storage layout
- orchestrate batch migration workflows
- own simulation
- own replay
- own release readiness reporting

Core API returns lawful semantic intent.

The host persists reality.

---

## Public API Principles

1. One call equals one deterministic turn.
2. Compiler and runtime are separate.
3. Version is part of truth.
4. Registry concepts are compile-time extension boundaries.
5. CompiledProcess is immutable and self-contained.
6. Host owns I/O and persistence.
7. Host owns effect dispatch.
8. Typed and JSON APIs are paired where applicable.
9. Manifest defines required data contract.
10. Lift requirements are source-version requirements.
11. Decision requirements are target-version requirements.
12. Validation must fail early and loudly.
13. Runtime must never depend on hidden host state.
14. VPE is event-compatible, not event-sourced.
15. Core provides semantic language; other APIs provide experience.

---

## Final Summary

The VPE Core API defines the semantic language of VPE.

It owns compilation, runtime evaluation, version truth, lift semantics, verdict production, event/effect intent, semantic patch intent, and host boundary rules.

The App API makes Core ergonomic.

The Process Evolution API uses Core semantics for optional simulation, replay, and migration workflows.

The Guard Architecture and Transform Architecture documents define registry lifecycle and validation details for guards and transforms.

Core provides the language.

Other APIs provide the experience.