# VPE Example Usage
Version: Canonical Draft v1

## Purpose
This document shows how a Rust application is expected to embed and use VPE in practice.

The examples focus on product ergonomics, not internal implementation.

## Common Integration Pattern
Across all host styles, the same basic pattern applies:

1. Build a registry
2. Build an engine
3. Register schema and process law
4. Ask for the state manifest
5. Fetch the required context and history
6. Execute one deterministic turn
7. Persist results
8. Dispatch effects externally

This is the intended standard usage model for business logic.

---

## Example 1: Web Application Embed

### Scenario
A Rust web backend receives an HTTP request to evaluate a loan application.

The application:
- loads the record from its database
- asks VPE what history is needed
- adapts request/DB data into context
- executes VPE
- persists the result
- dispatches effects

### Startup Flow
- create `GuardRegistry`
- enable built-ins
- create `VpeEngine`
- register domain schema
- register process law

### Request-Time Flow
The host application:
- identifies the process version
- loads the current record
- calls `manifest(process, current_state)`
- loads `ChronicleView`
- builds `ContextMap`
- calls `execute(request)`
- commits the resulting state and events atomically
- dispatches emitted effects

### Why This Fit Matters
This is the intended “adapters before and after VPE” pattern.

The web application keeps ownership of:
- HTTP
- auth
- persistence
- service clients

VPE owns:
- process law
- branching rules
- temporal logic
- legality of transitions

### Practical Value
This pattern keeps business logic centralized and replayable while allowing each application to keep its own infrastructure choices.

---

## Example 2: Event-Sourced Service

### Scenario
A Rust service handles commands against an event stream.

The service:
- loads a projection
- asks VPE for the current-state manifest
- loads the anchor and required event slice
- executes VPE
- appends resulting events with optimistic concurrency

### Request-Time Flow
The host application:
- receives a command
- loads the current projection
- identifies current state and anchor event
- calls `manifest(process, current_state)`
- loads only the required events
- builds `ContextMap` from projection and command data
- calls `execute(request)`
- appends `verdict.emitted_events` using expected-anchor semantics

### Why This Fit Matters
VPE is highly compatible with event-sourced architectures because it already centers:
- history
- state proof
- append-only semantics
- replayability
- deterministic outcomes

### Practical Value
This lets teams keep their event store and projections while moving business logic into a clearer, verified decision layer.

---

## Example 3: Workflow / Orchestration Bridge

### Scenario
A Rust service uses a workflow runtime or task scheduler to execute external work.

VPE:
- decides the next state
- emits effect envelopes
- moves into transient waiting states

The workflow/runtime:
- executes the external call
- retries if needed
- later feeds a callback action back into VPE

### Request-Time Flow
Initial turn:
- host loads state and history
- host executes VPE with action such as `SubmitPayment`
- verdict includes effect envelopes
- host maps those effects into workflow tasks

Callback turn:
- workflow completes or fails
- host translates result into action such as `GATEWAY_SUCCESS` or `GATEWAY_FAILURE`
- host calls VPE again for the next deterministic turn

### Why This Fit Matters
VPE remains the pure policy/process brain.
The workflow engine remains the execution/runtime layer.

### Practical Value
This separation keeps orchestration explicit and testable while avoiding framework lock-in.

---

## Example 4: Central Logic for a Website Platform

### Scenario
A team wants all major business rules for a website to flow through VPE.

Typical adapters:
- input adapter from HTTP/request model to `ContextMap`
- persistence adapter from DB/events to `ChronicleView`
- output adapter from `VpeVerdict` to UI/API response and side effects

### Recommended Pattern
- keep adapters thin
- keep VPE laws focused on business meaning
- keep controllers/services small
- treat VPE as the decision boundary

### Practical Value
This helps reduce duplicated branching logic spread across controllers, services, and jobs.

---

## Example 5: Central Orchestrator in Microservices

### Scenario
A distributed system wants VPE to coordinate decisions across services.

VPE emits:
- transitions
- effects
- planned events

The host system translates these into:
- commands
- messages
- service calls
- new domain events

### Recommended Pattern
Use VPE as:
- the deterministic law evaluator
- the state-transition authority
- the saga policy validator

Do not use VPE as:
- the message bus
- the effect executor
- the persistence layer

### Practical Value
This keeps orchestration rules explicit without requiring VPE to own the entire distributed runtime.

---

## Example 6: Interacting with Another Orchestration Engine

### Scenario
A team already has an orchestration or workflow platform and wants VPE to provide safer decision logic.

Recommended division of labor:

VPE owns:
- whether a transition is legal
- whether side-effect structure is safe
- what callbacks/failures/timeouts are expected

Other orchestration engine owns:
- durable execution
- retries
- backoff
- worker coordination
- task scheduling

### Practical Value
This makes VPE compatible with existing orchestration investments instead of forcing replacement.

---

## Example 7: Using Custom Guards

### Scenario
A Rust application needs domain-specific logic such as:
- `IsTrustedAccount`
- `InServiceArea`
- `MeetsInternalRiskScore`

### Recommended Pattern
- implement the public `Guard` trait
- register the guard in `GuardRegistry`
- reference it in the law JSON/source
- let the compiler and manifest system treat it like any other guard

### Practical Value
This gives teams extension power without exposing internal runtime layout.

---

## Recommended Embedding Rules

### Rule 1
Always ask VPE for the manifest before loading history for execution.

### Rule 2
Always provide explicit time via `sys.now`.

### Rule 3
Always include the Anchor in `ChronicleView`.

### Rule 4
Treat `VpeVerdict` as a decision artifact, not as already-executed work.

### Rule 5
Persist state and emitted events atomically.

### Rule 6
Keep adapters around VPE thin and boring.

---

## Why This Matters
The long-term goal is for VPE to make business logic feel like engineering rather than ad hoc control flow.

Used well, VPE should help teams:
- reduce duplicated business logic
- improve auditability
- improve replayability
- improve migration safety
- move logic out of fragile request handlers and scattered services

The ideal outcome is that embedding VPE feels natural enough that it becomes a standard boundary for serious business logic in Rust systems.