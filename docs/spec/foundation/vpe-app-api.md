# VPE Application Integration API (Wrapper) — Draft

Status: Draft  
Scope: Application-facing wrapper API for normal, version-aware decision execution  
Audience: Application developers, service authors, host integrators, future API implementers  
Depends On: Core API, runtime semantics, event model, concurrency contract, Core-defined version/lift semantics  
Does NOT define: compiler internals, CLI semantics, migration/lift semantics, migration/simulation workflows, FFI boundary details

---

## 1. Purpose

This document defines the intended shape of the VPE application integration API.

It exists to:

- provide a natural application-facing surface for normal VPE usage
- reduce ceremony without hiding truth
- stage complexity while preserving semantic correctness
- establish a preferred integration surface for host applications
- distinguish application integration from core engine and tooling integration
- support version-aware decision execution
- surface runtime lift outcomes when needed during normal application execution
- support simple state-based application integration without weakening correctness

This wrapper API is not a replacement for the Core API.

The Core API remains the canonical semantic and architectural surface of VPE.

The wrapper API exists to make ordinary host-side decision execution feel natural without redefining Core API meaning.

For simple web applications and other state-based hosts, the App API may provide convenience helpers that map host records into VPE decision truth.

Those helpers must remain explicit mappings. They must not infer correctness-relevant truth from arbitrary application state.

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
- Core API is best for semantic, tooling, compiler, runtime, and platform integration

The App API defines how applications use VPE.

The Core API defines what VPE means.

The wrapper is not a second semantic layer. It is an application-facing integration layer over Core-defined semantics.

---

## 3. Governing Principles

### 3.1 Preserve Truth

The wrapper must not change VPE semantics.

It may rename, group, and stage concepts, but it may not:

- redefine legality
- reinterpret manifests
- invent hidden execution rules
- infer missing correctness-relevant truth
- hide concurrency boundaries
- redefine version semantics
- redefine lift semantics

### 3.2 Reduce Ceremony, Not Responsibility

The wrapper should reduce setup friction and improve readability.

It must not:

- fetch host data automatically
- persist results automatically
- dispatch effects automatically
- conceal stale-verdict risk
- own orchestration

The host still supplies truth.

The host still owns persistence.

The host still dispatches effects.

The host still controls concurrency.

The wrapper may help construct decision inputs, interpret decision outcomes, or map host records into VPE-required truth, but it must not infer correctness-relevant truth from arbitrary application state.

Convenience may reduce ceremony, but it must not guess:

- current state
- current version
- target version
- context
- history
- anchor

### 3.3 Keep The Boundary Visible

The wrapper must continue to reinforce:

VPE decides. The host does.

More specifically:

- the host supplies truth
- VPE evaluates truth using Core API semantics
- VPE surfaces the decision result
- the host persists results atomically
- the host dispatches effects when appropriate

### 3.4 Application Language First

The wrapper should use names that reflect what the application developer is trying to do.

It should prefer terms like:

- process
- decision
- required data
- outcome
- history

Over purely engine-native terminology where appropriate.

Application-facing names may differ from Core API names for readability, but they must preserve Core API meaning.

### 3.5 Builders Only Where Complexity Is Real

Normal decision execution should not require a builder.

Builders should be used when the caller wants more control, optional configuration, fluent setup, or guided construction.

Builders must not create a separate execution model.

A builder-produced DecisionInput and a directly constructed DecisionInput must remain semantically equivalent when they supply the same truth.

### 3.6 Version Is Part Of Truth

Version participates in runtime correctness.

An instance is not merely in a state.

An instance is in a state under a particular process version.

The wrapper may simplify interaction with versioned truth but must not hide version information when version affects runtime behavior.

The canonical definition of version-aware truth belongs to the Core API.

### 3.7 Application Integration Without Semantic Loss

The App API should optimize application integration effort without reducing semantic correctness.

Convenience must not come at the expense of correctness-relevant truth.

### 3.8 Explicit Mapping Over Inference

The App API may provide convenience helpers for mapping host records into VPE decision truth.

Those mappings must be explicit and host-defined.

The App API must not inspect arbitrary host records and infer process truth on its own.

This is especially important for simple web applications, where application state may be stored in many different shapes, such as database rows, ORM entities, DTOs, view models, or domain objects.

The App API may make the simple path easier, but the host must still declare how its data maps into VPE-required truth.

Adapters are explicit truth mappers, not inference engines.

---

## 4. Role Of The Wrapper API

The wrapper API should answer these application questions directly:

- How do I hold or reference a process?
- What version of process truth am I evaluating?
- What data do I need before deciding?
- How do I ask what happens next?
- What do I save?
- What effects should I run?
- How do I keep the host/VPE boundary clear?
- How do I map a host record into decision truth?

The wrapper API should not try to answer these platform questions directly:

- How do I configure the compiler in detail?
- How do I access raw artifact internals?
- How do I implement the CLI?
- How do I build migration and simulation tooling?
- How do I expose raw engine semantics externally?

Those belong more naturally to the Core API or the Process Evolution API, depending on whether the concern is semantic meaning or workflow orchestration.

---

## 5. Recommended Top-Level Shape

The wrapper API should revolve around four primary concepts:

- ProcessHandle
- DecisionInput
- DecisionOutcome
- DecisionBuilder

Optional supporting concepts may include:

- RequiredData
- DecisionHistory
- PersistencePlan
- EffectPlan
- RecordAdapter

RecordAdapter is an optional App API convenience concept for simple state-based hosts.

It should not become a fifth semantic pillar.

It exists to help construct DecisionInput from host-owned record shapes.

---

## 6. ProcessHandle

### 6.1 Purpose

ProcessHandle is the application-facing holder for a compiled and usable process.

It gives the caller a process-shaped object rather than forcing them to think first in terms of raw engine types.

A ProcessHandle represents a specific process version.

When multiple versions are installed, each handle represents the version it was created from.

### 6.2 Responsibilities

ProcessHandle should:

- identify the process
- expose friendly metadata
- expose process version information
- expose required-data information for a state
- support version-aware decision execution
- surface runtime lift outcomes when they occur
- execute a decision from application-shaped input
- create a builder for more explicit setup
- provide optional record-based helpers for state-based hosts

### 6.3 Non-Responsibilities

ProcessHandle should not:

- own persistence
- own orchestration
- own effect dispatch
- define version semantics
- define lift semantics
- hide the installed/compiled distinction if that distinction matters operationally

### 6.4 Suggested Shape

ProcessHandle should support at least:

- id()
- version()
- digest()
- required_data_for(state)
- decide(input)
- prepare_decision()

Record-based helpers may also be provided:

- prepare_from_record(record, adapter)
- decide_record(record, adapter, action)

Record-based helpers are optional App API conveniences.

They must produce ordinary DecisionInput internally.

They must not create a separate decision model.

### 6.5 Installed Vs Direct Handles

The wrapper should allow both:

- a handle for an already-installed process in an engine
- a handle built directly around a compiled process artifact

This preserves flexibility without forcing all callers through one infrastructure pattern.

Both installed and direct handles should expose the process version used for version-aware execution.

---

## 7. RequiredData

### 7.1 Purpose

The wrapper should expose state requirements in language that application developers understand.

A term like required_data_for(state) is more approachable than directly exposing only manifest(state) at the wrapper level.

### 7.2 Meaning

RequiredData should describe what the host must gather before it asks VPE to decide from a particular versioned state.

This may include:

- required context paths
- required history/event requirements
- anchor or truth expectations where relevant
- current version requirements
- target version requirements
- migration-related requirements

RequiredData describes obligations.

The Core API defines their meaning.

For simple state-based hosts, RequiredData may guide what a record adapter must extract from the host record.

The adapter may make collection easier, but it does not change the requirement.

### 7.3 Relationship To Core

RequiredData may internally be a wrapper over the Core manifest model, but it should not reinterpret it.

The App API may present requirements in application-facing terms.

The Core API remains authoritative for what those requirements mean.

---

## 8. DecisionInput

### 8.1 Purpose

DecisionInput represents the truth supplied by the host for a decision request.

It is the application-facing form of the question:

What is the current versioned truth, and what action is being requested?

DecisionInput should remain explicit.

It may be constructed directly by the host.

It may also be constructed through App API helpers, such as builders or record adapters.

Those helpers do not change the meaning of DecisionInput.

They only reduce the ceremony of constructing it.

### 8.2 Required Contents

DecisionInput should include, either directly or through an explicit host-defined mapping:

- current process version
- target process version
- current state
- requested action
- required context
- required history, if any
- concurrency anchor or truth marker

Optional fields may include:

- current time
- trace or correlation identifier
- actor or caller metadata
- lift strategy, when supported

The target version normally defaults to the ProcessHandle version.

For simple state-based hosts, the required truth may come from an application record through a host-defined record adapter.

The adapter must explicitly map the record into the required DecisionInput fields.

The App API must not guess where the host stores state, version, context, history, or anchor information.

Version information is correctness-relevant truth and must remain visible.

### 8.3 Why This Should Stay Explicit

A hidden decision input would make correctness unclear.

VPE decisions are only valid relative to supplied truth.

That truth may include state, version, context, history, and an anchor.

Applications store that truth in different ways.

One application may store state in a field named status.

Another may store it in workflow_state.

Another may derive it from a domain object.

Another may reconstruct it from history.

The App API should not assume those shapes.

Instead, it should allow the host to supply truth directly or provide an explicit mapping from host records into VPE decision truth.

Current-state assumptions, truth snapshot assumptions, version assumptions, concurrency relevance, and history requirements are not advanced extras.

They are part of correctness.

### 8.4 Suggested Shape

A reasonable starting shape is:

- current_version
- target_version
- current_state
- action
- context
- history
- anchor
- optional now
- optional trace_id
- optional actor or caller metadata
- optional lift_strategy, when supported

For direct usage, the host supplies these fields explicitly.

For simple state-based usage, a host-defined adapter may supply some or all of these fields from an application record.

In both cases, the resulting DecisionInput has the same semantic meaning.

The exact Core types may remain richer underneath.

---

## 9. DecisionHistory

### 9.1 Purpose

At the wrapper layer, history should be named in a way that reflects application understanding.

A wrapper term like DecisionHistory or HistoryInput may be easier to understand than exposing ChronicleView directly as the first concept users encounter.

History may participate in both decision evaluation and version-aware execution.

### 9.2 Rule

The wrapper may rename or group history concepts, but it must not weaken the meaning of the chronicle.

The host must still supply the truth required by the process.

The wrapper must not infer missing historical truth.

For state-based hosts, history may be empty only when the process does not require history.

A record adapter may provide history when the host record or related storage contains history-relevant truth.

---

## 10. DecisionOutcome

### 10.1 Purpose

DecisionOutcome is the wrapper-facing result of one decision turn.

It should answer the questions application code most naturally asks after calling VPE:

- what happened
- what state changed
- what version was evaluated
- whether version-crossing occurred
- what should be persisted
- what events were produced
- what effects should be considered for dispatch

### 10.2 Required Properties

A DecisionOutcome should clearly expose:

- process identity
- source version
- target version
- from-state
- to-state
- emitted events
- effect intent
- any state patch or record patch
- persistence expectations
- optional lift outcome
- enough information for the host to persist and continue safely

DecisionOutcome represents what VPE concluded from supplied truth.

DecisionOutcome is not a persistence record.

A successful lift does not imply a successful decision.

Lift results and decision results remain distinct concepts.

### 10.3 Relationship To Core

DecisionOutcome may wrap VpeVerdict internally, but should not reinterpret it.

It should be a friendlier result shape, not a different truth model.

LiftOutcome is defined by the Core API.

The App API may surface lift information but does not define it.

### 10.4 Helpful Convenience Methods

The wrapper outcome may provide helpers such as:

- transitioned()
- from_state()
- to_state()
- source_version()
- target_version()
- lift()
- events()
- effects()
- state_patch()
- persistence_plan()
- effect_plan()

These helpers should clarify host action without performing host action.

---

## 11. PersistencePlan And EffectPlan

### 11.1 Purpose

The wrapper may expose clear handoff objects for the host.

These objects are useful because most host code naturally wants to separate:

- what should be persisted
- what should be dispatched

PersistencePlan may include:

- persistence expectations
- target version
- version-transition expectations

EffectPlan may describe effect intent in a way that helps the host decide what to dispatch after persistence succeeds.

### 11.2 Rule

If provided, these plans must remain descriptive, not imperative.

They may tell the host what to do next.

They must not do it.

When version-crossing occurs, version-transition artifacts and decision artifacts must be committed atomically.

PersistencePlan describes obligations.

It does not perform persistence.

EffectPlan describes dispatch intent.

It does not dispatch effects.

### 11.3 Benefit

This helps the wrapper feel natural without violating the Core boundary.

For state-based hosts using record adapters, PersistencePlan may help the host understand what must be reflected back into the application record or related storage.

The adapter does not perform persistence.

The host remains responsible for applying and committing changes atomically.

---

## 12. DecisionBuilder

### 12.1 Purpose

A DecisionBuilder is useful when direct construction of DecisionInput would be noisy.

The builder may guide the host through required fields.

It may provide defaults where the App API has a safe default, such as defaulting target version to the ProcessHandle version.

It may also support simple state-based usage by starting from a host record and a host-defined record adapter.

The builder does not define a separate execution model.

Builder-based construction and direct DecisionInput construction must remain semantically equivalent.

### 12.2 Rule

A builder should not be required for the normal first-success path.

The wrapper should support direct DecisionInput construction for straightforward use.

Builders are appropriate when:

- required input has multiple fields
- some fields have safe defaults
- validation should happen before decision execution
- direct construction would create repetitive ceremony
- simple state-based hosts need help mapping records into DecisionInput truth
- richer tracing metadata is useful
- optional time overrides are needed
- future preview or dry-run variations need fluent setup

Builders are not appropriate when they hide correctness-relevant truth.

A builder may simplify input construction.

It must not invent state, version, context, history, or anchor information.

### 12.3 Suggested Builder Style

A builder may support direct construction:

- current_version(...)
- target_version(...)
- current_state(...)
- action(...)
- context(...)
- history(...)
- anchor(...)
- decide()

A builder may also support record-based construction:

- prepare_from_record(record, adapter)
- action(...)
- target_version(...)
- decide()

In record-based construction, the adapter supplies the host-defined mapping from the application record into VPE-required truth.

The builder uses that mapping to construct an ordinary DecisionInput.

---

## 13. Record Adapters

A Record Adapter is an optional App API convenience pattern for simple state-based hosts.

A Record Adapter maps a host-owned application record into the truth required to construct a DecisionInput.

This is intended for applications where process truth is stored in ordinary application records, such as database rows, ORM entities, DTOs, or domain objects.

A Record Adapter may extract:

- current state
- current version
- context
- anchor
- optional history

A Record Adapter does not define VPE semantics.

A Record Adapter does not replace DecisionInput.

A Record Adapter only translates host-owned record shape into VPE-required decision truth.

Advanced hosts may continue constructing DecisionInput directly.

### 13.1 Simple State-Based Host Usage

For a simple web application, the normal flow may be:

1. Load an application record.
2. Use a host-defined adapter to map the record into VPE decision truth.
3. Ask VPE to decide.
4. Persist the resulting application changes atomically.
5. Dispatch effects if appropriate.

The host remains responsible for persistence.

The host remains responsible for concurrency checks.

The host remains responsible for committing the result.

### 13.2 Adapter Invariant

Adapters are explicit truth mappers, not inference engines.

The App API must not inspect arbitrary host records and infer process truth on its own.

The host must declare the mapping.

### 13.3 Suggested Adapter Shape

A record adapter may provide mappings equivalent to:

- current_state(record)
- current_version(record)
- context(record)
- anchor(record)
- history(record)

History may default to empty only when the process does not require history.

Target version normally defaults to the ProcessHandle version unless explicitly supplied.

### 13.4 Relationship To DecisionInput

Record adapters produce or help produce DecisionInput.

They do not change what DecisionInput means.

A decision built from a record adapter and a decision built from direct DecisionInput construction must be semantically equivalent when they supply the same truth.

---

## 14. Suggested Wrapper API Surface

The minimal useful surface should be small.

### 14.1 Preferred Initial Surface

- ProcessHandle
- DecisionInput
- DecisionOutcome
- DecisionBuilder
- RequiredData

Optional supporting concepts may include:

- DecisionHistory
- PersistencePlan
- EffectPlan
- RecordAdapter

### 14.2 Initial Recommended Methods

On ProcessHandle:

- id()
- version()
- digest()
- required_data_for(state)
- decide(input)
- prepare_decision()

Optional record-based helpers may include:

- prepare_from_record(record, adapter)
- decide_record(record, adapter, action)

On DecisionOutcome:

- transitioned()
- from_state()
- to_state()
- source_version()
- target_version()
- lift()
- events()
- effects()
- state_patch()
- persistence_plan()
- effect_plan()

Decision builders may support:

- current_version(...)
- target_version(...)
- current_state(...)
- action(...)
- context(...)
- history(...)
- anchor(...)
- decide()

Record-based helpers are optional.

They should be treated as App API convenience methods for state-based hosts.

They must produce ordinary DecisionInput internally.

They must not create a separate decision model.

### 14.3 Optional Later Methods

Potentially useful later:

- can(input)
- preview(input)

These should only exist if they do not blur the meaning of a real decision evaluation.

---

## 15. Relationship To The Core API

### 15.1 Core API Role

The Core API remains the better fit for:

- canonical runtime semantics
- version-aware truth semantics
- lift semantics
- migration semantics
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

The App API does not define:

- LiftStatus
- LiftOutcome
- LiftPath
- LiftStep
- LiftEvent
- SemanticPatch
- MigrationRuleReference
- IncompatibilityReason
- migration determinism rules

These concepts belong to the Core API.

The App API may expose them during normal application-facing execution.

### 15.2 Wrapper API Role

The wrapper API should be the better fit for:

- application handlers
- service-layer decision points
- worker decision execution
- CRUD-style hosts
- state-based hosts
- event-sourced hosts
- domain/application services
- simple record-based web applications
- host code that wants a process/decision/outcome mental model

Record adapters belong to the App API, not the Core API.

They are an application integration convenience.

The Core API defines the meaning of decision truth.

The App API may help host applications map their records into that truth.

The adapter does not define state semantics, version semantics, anchor semantics, or context semantics.

It only supplies host-owned mappings into Core-defined concepts.

### 15.3 Important Clarification

The wrapper is not inferior because it is simpler.

It is narrower because it is focused on normal application integration rather than semantic infrastructure work.

If App API documentation and Core API documentation disagree about semantic meaning, the Core API is authoritative.

---

## 16. Explicit Limitations Of The Wrapper

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

The App API does not define:

- migration semantics
- lift semantics
- replay semantics
- simulation semantics
- migration compatibility rules

Those belong to the Core API or Process Evolution API.

The App API does not inspect arbitrary host records to infer process truth.

The App API does not assume where a host stores:

- state
- version
- context
- history
- anchor

Host-defined adapters may provide those mappings explicitly.

Adapters must not become a hidden persistence model.

Adapters must not become a hidden event model.

Adapters must not become a second semantic model.

### 16.1 Conceptual Limitation

The wrapper may flatten or rename some engine concepts for readability, but it must not flatten away correctness.

If a concept is required for correctness, the wrapper must either expose it or require an explicit host-provided mapping to it.

---

## 17. Event-Sourced Hosts

Record adapters are mainly useful for state-based hosts.

Event-sourced hosts may still construct DecisionInput directly from reconstructed truth.

The App API should support both styles.

State-based hosts may map records into DecisionInput.

Event-sourced hosts may reconstruct truth from history and then construct DecisionInput directly.

Neither style changes VPE semantics.

The wrapper should remain compatible with event-sourced hosts.

An event-sourced host may use the wrapper even if it already has rich infrastructure.

In that case:

- current state may come from projection
- history may come from an event stream or chronicle view
- anchor may come from stream revision
- persistence may mean appending events rather than updating a row

Event-sourced hosts may reconstruct versioned truth from history rather than storing versioned snapshots.

Versioned truth remains part of the supplied decision input.

The wrapper should not assume a CRUD-only persistence model.

---

## 18. Concurrency Visibility

Decision outcomes remain valid only relative to supplied truth.

Truth includes:

- version
- state
- context
- history
- anchor

Version advancement is truth advancement.

A version transition is considered a truth transition for concurrency purposes.

The wrapper should make this visible.

A decision outcome should not look like a command that can be applied blindly later.

It is the result of evaluating a particular truth snapshot.

If that truth changes before commit, the host must re-evaluate or otherwise handle the conflict.

For state-based hosts using record adapters, the adapter should expose the concurrency anchor or truth marker from the host record.

Examples include:

- row version
- revision number
- updated-at marker
- entity tag
- compare-and-swap token

The App API should not invent this anchor.

The host must supply it directly or through an explicit adapter mapping.

---

## 19. Naming Guidance

The wrapper should use names that help application developers understand what they are doing.

Good general wrapper names include:

- process
- decision
- required data
- outcome
- history
- persistence plan
- effect plan

Prefer adapter names that make the mapping role clear.

Good adapter names include:

- RecordAdapter
- DecisionRecordAdapter
- RecordDecisionAdapter
- StateRecordAdapter

Avoid names that imply VPE owns the host model.

Avoid names that suggest inference or automatic discovery.

The adapter maps host records into VPE truth.

It does not make host records into VPE records.

Naming should preserve the host/VPE boundary.

---

## 20. Example Direction

Examples should make the easy path visible without hiding the correctness boundary.

### 20.1 General Decision Execution Example

A general application example should show:

1. Obtaining or receiving a ProcessHandle.
2. Inspecting RequiredData if needed.
3. Gathering versioned truth.
4. Building a DecisionInput.
5. Calling decide(input).
6. Receiving a DecisionOutcome.
7. Persisting required changes atomically.
8. Dispatching effects only after persistence succeeds.

The reader should be able to understand:

- what truth the host supplied
- what VPE decided
- what the host must save
- what effects the host may dispatch
- where the host/VPE boundary remains

### 20.2 Simple State-Based Record Adapter Example

A simple state-based application example should show:

1. Loading an application record.
2. Passing the record and adapter into the App API.
3. Requesting an action.
4. Receiving a DecisionOutcome.
5. Saving the updated application record atomically using the supplied anchor.
6. Dispatching effects only after persistence succeeds.

The reader should be able to understand:

- where state came from
- where version came from
- where context came from
- where anchor came from
- what VPE decided
- what the host must commit

---

## 21. Final Position

The App API remains the preferred application-facing integration surface.

The App API is:

- application-facing
- version-aware
- lift-aware
- concurrency-aware
- friendly to simple state-based hosts
- compatible with event-sourced hosts

The Core API defines:

- semantic meaning
- architectural meaning
- version semantics
- lift semantics
- migration semantics

The Process Evolution API uses those semantics for:

- migration workflows
- replay
- simulation
- diagnostics
- readiness analysis

For simple state-based applications, the App API may support record-adapter helpers that reduce ceremony.

These helpers preserve the same boundary:

Host records contain application data.

Host adapters map that data into VPE decision truth.

VPE evaluates the decision.

The host commits the result.

Final rule:

Host supplies versioned truth.

VPE evaluates truth using Core API semantics.

VPE surfaces runtime lift results when necessary.

Host commits atomically.

The Core API defines what VPE means.

The App API defines how applications use VPE.
