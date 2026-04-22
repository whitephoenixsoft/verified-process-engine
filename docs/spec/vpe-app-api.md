# VPE Application Integration API (Wrapper) — Draft

Status: Draft  
Scope: Application-facing wrapper API for normal decision execution  
Audience: Application developers, service authors, host integrators, future API implementers  
Depends On: Core API, runtime semantics, event model, concurrency contract  
Does NOT define: compiler internals, CLI semantics, migration/simulation APIs, FFI boundary details

---

## 1. Purpose

This document defines the intended shape of the VPE application integration API.

It exists to:

- provide a natural application-facing surface for normal VPE usage
- reduce ceremony without hiding truth
- stage complexity while preserving semantic correctness
- establish a preferred integration surface for host applications
- distinguish application integration from core engine and tooling integration

This wrapper API is not a replacement for the core API.

The core API remains the semantic and platform integration surface.  
The wrapper API exists to make ordinary host-side decision execution feel natural.

---

## 2. Core Position

The wrapper API should be the preferred surface for normal application integration.

It is not merely for beginners.  
It should be valid even for sophisticated systems, including event-sourced hosts, as long as the caller wants an application-shaped decision interface.

The distinction is not:

- simple users use wrapper
- advanced users use core

The real distinction is:

- wrapper API is best for application integration
- core API is best for semantic, tooling, compiler, runtime, and platform integration

---

## 3. Governing Principles

### 3.1 Preserve truth

The wrapper must not change VPE semantics.

It may rename, group, and stage concepts, but it may not:

- redefine legality
- reinterpret manifests
- invent hidden execution rules
- infer missing correctness-relevant truth
- hide concurrency boundaries

### 3.2 Reduce ceremony, not responsibility

The wrapper should reduce setup friction and improve readability.

It must not:

- fetch host data automatically
- persist results automatically
- dispatch effects automatically
- conceal stale-verdict risk
- own orchestration

### 3.3 Keep the boundary visible

The wrapper must continue to reinforce:

VPE decides. The host does.

### 3.4 Application language first

The wrapper should use names that reflect what the application developer is trying to do.

It should prefer terms like:

- process
- decision
- required data
- outcome
- history

over purely engine-native terminology where appropriate.

### 3.5 Builders only where complexity is real

Normal decision execution should not require a builder.

Builders should be used when the caller wants more control, optional configuration, or fluent setup.

---

## 4. Role of the Wrapper API

The wrapper API should answer these application questions directly:

- How do I hold or reference a process?
- What data do I need before deciding?
- How do I ask what happens next?
- What do I save?
- What effects should I run?
- How do I keep the host/VPE boundary clear?

The wrapper API should not try to answer these platform questions directly:

- How do I configure the compiler in detail?
- How do I access raw artifact internals?
- How do I implement the CLI?
- How do I build migration and simulation tooling?
- How do I expose raw engine semantics externally?

Those belong more naturally to the core API.

---

## 5. Recommended Top-Level Shape

The wrapper API should revolve around four concepts:

- `ProcessHandle`
- `DecisionInput`
- `DecisionOutcome`
- `DecisionBuilder`

Optional supporting concepts may include:

- `RequiredData`
- `DecisionHistory`
- `PersistencePlan`
- `EffectPlan`

---

## 6. ProcessHandle

### 6.1 Purpose

`ProcessHandle` is the application-facing holder for a compiled and usable process.

It gives the caller a process-shaped object rather than forcing them to think first in terms of raw engine types.

### 6.2 Responsibilities

`ProcessHandle` should:

- identify the process
- expose friendly metadata
- expose required-data information for a state
- execute a decision from application-shaped input
- create a builder for more explicit setup

### 6.3 Non-responsibilities

`ProcessHandle` should not:

- own persistence
- own orchestration
- own effect dispatch
- hide the installed/compiled distinction if that distinction matters operationally

### 6.4 Suggested shape

`ProcessHandle` should support at least:

- `id()`
- `version()` or equivalent if meaningful
- `digest()`
- `required_data_for(state)`
- `decide(input)`
- `prepare_decision()`

### 6.5 Installed vs direct handles

The wrapper should allow both:

- a handle for an already-installed process in an engine
- a handle built directly around a compiled process artifact

This preserves flexibility without forcing all callers through one infrastructure pattern.

---

## 7. RequiredData

### 7.1 Purpose

The wrapper should expose state requirements in language that application developers understand.

A term like `required_data_for(state)` is more approachable than directly exposing only `manifest(state)` at the wrapper level.

### 7.2 Meaning

`RequiredData` should describe what the host must gather before it asks VPE to decide from a particular state.

This may include:

- required context paths
- required history/event requirements
- anchor or truth expectations where relevant

### 7.3 Relationship to core

`RequiredData` may internally be a wrapper over the core manifest model, but it should not reinterpret it.

---

## 8. DecisionInput

### 8.1 Purpose

`DecisionInput` is the application-facing wrapper around the runtime request shape.

It should collect the information a host normally gathers before making a decision.

### 8.2 Required contents

A `DecisionInput` should remain explicit about:

- current state
- action
- context
- history or chronicle input
- anchor or equivalent truth marker

Optional but common fields may include:

- current time
- trace or correlation ID
- actor or caller metadata if supported by the core model

### 8.3 Why this should stay explicit

The wrapper must not hide:

- current-state assumptions
- truth snapshot assumptions
- concurrency relevance
- history requirements

Those are not “advanced extras.”  
They are part of correctness.

### 8.4 Suggested shape

A reasonable starting shape is:

- `current_state`
- `action`
- `context`
- `history`
- `anchor`
- optional `now`
- optional `trace_id`

The exact core types may remain richer underneath.

---

## 9. DecisionHistory

### 9.1 Purpose

At the wrapper layer, history should be named in a way that reflects application understanding.

A wrapper term like `DecisionHistory` or `HistoryInput` may be easier to understand than exposing `ChronicleView` directly as the first concept users encounter.

### 9.2 Rule

The wrapper may rename or group history concepts, but it must not weaken the meaning of the chronicle.

The host must still supply the truth required by the process.

---

## 10. DecisionOutcome

### 10.1 Purpose

`DecisionOutcome` is the wrapper-facing result of one decision turn.

It should answer the questions application code most naturally asks after calling VPE:

- what happened
- what state changed
- what should be persisted
- what events were produced
- what effects should be considered for dispatch

### 10.2 Required properties

A `DecisionOutcome` should clearly expose:

- process identity
- from-state
- to-state
- emitted events
- effect intent
- any state patch or record patch
- enough information for the host to persist and continue safely

### 10.3 Relationship to core

`DecisionOutcome` may wrap `VpeVerdict` internally, but should not reinterpret it.

It should be a friendlier result shape, not a different truth model.

### 10.4 Helpful convenience methods

The wrapper outcome may provide helpers such as:

- `transitioned()`
- `from_state()`
- `to_state()`
- `events()`
- `effects()`
- `state_patch()`
- `persistence_plan()`
- `effect_plan()`

These helpers should clarify host action without performing host action.

---

## 11. PersistencePlan and EffectPlan

### 11.1 Purpose

The wrapper may expose clear handoff objects for the host.

These objects are useful because most host code naturally wants to separate:

- what should be persisted
- what should be dispatched

### 11.2 Rule

If provided, these plans must remain descriptive, not imperative.

They may tell the host what to do next.  
They must not do it.

### 11.3 Benefit

This helps the wrapper feel natural without violating the core boundary.

---

## 12. DecisionBuilder

### 12.1 Purpose

A builder should exist for nontrivial decision setup.

It is appropriate when:

- multiple optional fields are present
- fluent assembly improves readability
- the caller wants gradual construction
- later advanced modes may need symmetry with normal decision execution

### 12.2 Rule

A builder should not be required for the normal first-success path.

The wrapper should support direct `DecisionInput` construction for straightforward use.

### 12.3 Typical usage

A builder is appropriate for:

- verbose host code
- richer tracing metadata
- optional time overrides
- future preview or dry-run variations
- integration code that wants fluent assembly

---

## 13. Suggested Wrapper API Surface

The minimal useful surface should be small.

### 13.1 Preferred initial surface

- `ProcessHandle`
- `DecisionInput`
- `DecisionOutcome`
- `DecisionBuilder`
- `RequiredData`

### 13.2 Initial recommended methods

On `ProcessHandle`:

- `id()`
- `required_data_for(state)`
- `decide(input)`
- `prepare_decision()`

On `DecisionOutcome`:

- `from_state()`
- `to_state()`
- `events()`
- `effects()`
- `state_patch()`

### 13.3 Optional later methods

Potentially useful later:

- `can(input)`
- `preview(input)`

These should only exist if they do not blur the meaning of a real decision evaluation.

---

## 14. Relationship to the Core API

### 14.1 Core API role

The core API remains the better fit for:

- compiler integration
- raw runtime integration
- CLI implementation
- migration tooling
- simulation tooling
- detailed diagnostics
- raw artifact handling
- custom engine/platform assembly
- FFI and low-level embedding
- any use case needing exact one-to-one semantic visibility

### 14.2 Wrapper API role

The wrapper API should be the better fit for:

- application handlers
- service-layer decision points
- worker decision execution
- CRUD-style hosts
- event-sourced hosts
- domain/application services
- host code that wants a process/decision/outcome mental model

### 14.3 Important clarification

The wrapper is not inferior because it is simpler.

It is narrower because it is focused on normal application integration rather than semantic infrastructure work.

---

## 15. Explicit Limitations of the Wrapper

The wrapper should intentionally avoid becoming:

- the compiler API
- the tooling API
- the platform assembly API
- the raw artifact API
- the migration/simulation API
- the orchestration API
- the persistence API
- the effect-dispatch API

These are not failures of the wrapper.  
They are healthy boundaries.

### 15.1 Conceptual limitation

The wrapper may flatten or rename some engine concepts for readability.

This makes it better for app code, but slightly less exact as a semantic teaching surface than the raw core API.

### 15.2 Operational limitation

The wrapper should not own installation, engine lifecycle, or custom platform assembly patterns beyond what normal application integration needs.

### 15.3 Semantic limitation

The wrapper should not become the place where new semantic modes are invented.

If a behavior changes truth, it belongs in the core first.

---

## 16. Event-Sourced Hosts

### 16.1 Position

The wrapper should remain a first-class option for event-sourced hosts.

It should not be described as unsuitable merely because the host architecture is sophisticated.

### 16.2 Why it still fits

Event-sourced hosts still perform a familiar application act:

- gather current truth
- decide what is lawful
- append resulting events
- dispatch effects if appropriate

That maps naturally to:

- `ProcessHandle`
- `DecisionInput`
- `DecisionOutcome`

### 16.3 Boundary remains

Even in event-sourced hosts, the wrapper must remain explicit about:

- history
- anchor/current truth
- snapshot-relative validity
- host-owned commit responsibility

---

## 17. Concurrency Visibility

The wrapper must not hide the concurrency boundary.

It should remain clear that:

- the decision is based on known supplied truth
- the outcome is valid relative to that truth
- the host must still commit atomically
- stale outcomes are possible if the truth has advanced

This is one of the most important constraints on wrapper design.

The wrapper may make this easier to understand.  
It may not make it disappear.

---

## 18. Naming Guidance

The wrapper should use application-facing names where that improves clarity.

Preferred wrapper-facing examples include:

- `ProcessHandle`
- `DecisionInput`
- `DecisionOutcome`
- `DecisionHistory`
- `RequiredData`

Core engine terms may still exist beneath the wrapper.

This preserves a strong distinction between:

- engine truth
- application integration language

---

## 19. Example Direction

The wrapper should make normal host code feel like:

- obtain a process
- inspect required data
- build one decision input
- call `decide`
- persist the resulting outcome
- dispatch effects from the outcome if desired

This should be the center of the wrapper experience.

---

## 20. Final Position

The wrapper API should be a first-class application integration surface.

It should not be framed as merely a beginner path.

Its purpose is to make ordinary VPE usage feel natural in real host applications while keeping the core API free to remain exact, semantic, and infrastructure-friendly.

The correct long-term distinction is:

- the core API is what VPE is
- the wrapper API is how most applications use VPE

The wrapper succeeds if it makes VPE easier to adopt without weakening any of the truths that make VPE architecturally valuable.