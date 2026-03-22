# VPE Product Brief
Version: Canonical v1

## 1. Overview

The Verified Process Engine (VPE) is a deterministic decision engine that compiles declarative process logic into an executable model.

VPE separates:
- **what should happen** (Law)
- from **how and when it happens** (Execution systems)

VPE is designed to:
- centralize business logic
- make decisions deterministic and replayable
- enable compile-time validation of workflows
- integrate cleanly with existing systems

VPE is not a replacement for orchestration, storage, or execution frameworks.  
It is a **decision layer** that complements them.

---

## 2. Problem Statement

Modern systems often suffer from:

- business logic scattered across services
- implicit decision-making hidden in code
- inconsistent behavior across environments
- inability to replay or audit decisions
- fragile workflows with runtime failures instead of compile-time validation

This leads to:
- difficult debugging
- slow iteration
- increased operational risk
- lack of confidence in system behavior

---

## 3. Solution

VPE introduces a new layer:

> A **deterministic, compiled decision engine** that evaluates process logic as a pure function.

Core model:

compile(Law, Schema, Registry) → CompiledProcess  
evaluate(CompiledProcess, Request) → Verdict

Where:
- **Law** defines the process declaratively
- **Schema** defines valid data
- **Registry** defines executable logic (guards)
- **Request** provides current context and history
- **Verdict** defines the next state and intended effects

---

## 4. Key Capabilities

### Deterministic Execution
- Same inputs → same outputs
- No hidden state or side effects
- Fully replayable

### Compile-Time Validation
- Detect invalid workflows before runtime
- Enforce:
  - no illegal cycles
  - valid transitions
  - type-safe logic
  - saga completeness

### History-Aware Decisions
- Decisions can depend on:
  - past events
  - time windows
  - event frequency

### Explicit Data Dependencies
- Per-state manifests define required:
  - history
  - context fields

### Separation of Concerns
- VPE decides
- host systems execute

### Versioning & Migration
- Processes are versioned
- Records can be deterministically lifted between versions

---

## 5. What VPE Is

VPE is:

- a decision engine
- a process compiler
- a deterministic state transition evaluator
- a bridge between data and orchestration

---

## 6. What VPE Is Not

VPE is not:

- a workflow executor
- a job scheduler
- a message broker
- a database
- a side-effect execution engine

VPE does not:
- call external services
- perform I/O
- manage infrastructure

---

## 7. How VPE Fits in a System

Typical architecture:

Client / API / Event Source  
→ VPE (decision layer)  
→ Host system executes effects  
→ Events persisted  
→ Next request evaluated

VPE sits between:
- **data**
- and **action**

---

## 8. Ecosystem Compatibility

VPE is designed to complement existing tools, not replace them.

### Workflow Engines (Temporal, Airflow, Argo)

These systems handle:
- execution
- retries
- scheduling
- distributed coordination

VPE provides:
- deterministic decision-making
- validated transition logic

**Integration pattern:**
- workflow calls VPE to decide next step
- workflow executes resulting effects

---

### Rules Engines (Drools, DMN)

These systems provide:
- stateless rule evaluation

VPE extends this with:
- state
- history
- temporal logic
- process flow

---

### State Machines (XState, Step Functions)

These systems provide:
- structural state transitions

VPE adds:
- compile-time guarantees
- rich guard logic
- history-aware decisions
- migration support

---

### Policy Engines (OPA)

OPA provides:
- deterministic policy evaluation

VPE extends this into:
- multi-step processes
- state transitions
- orchestration intent

---

### Event-Sourced Systems

Event systems provide:
- data and history

VPE provides:
- deterministic interpretation of that history
- consistent decision outcomes

---

## 9. Example Use Cases

### Web Application Logic
- approvals
- onboarding flows
- entitlement decisions

### Financial Systems
- loan approval
- fraud detection
- transaction validation

### Event-Driven Systems
- orchestrating microservices
- reacting to domain events

### Workflow Orchestration
- decision layer for Temporal/Argo flows

### Versioned Business Processes
- evolving rules safely over time
- replaying historical decisions

---

## 10. Product Surfaces

### Rust Library
- canonical implementation
- high-performance execution
- embeddable in applications

### CLI
- design-time validation
- execution harness
- simulation and debugging tool

### FFI (Planned)
- integration with:
  - .NET
  - Go
  - Python

---

## 11. Design Principles

VPE is built on:

- determinism
- explicitness
- compile-time safety
- separation of concerns
- replayability
- portability

---

## 12. Why VPE

VPE enables teams to:

- move business logic out of code
- reason about processes explicitly
- validate logic before deployment
- replay and audit decisions
- evolve systems safely

It reduces:
- hidden complexity
- runtime surprises
- duplicated logic

---

## 13. Vision

VPE aims to become:

> the standard way to define, validate, and execute business logic in modern systems

A future where:
- business processes are compiled, not improvised
- decisions are deterministic, not incidental
- systems are explainable, not opaque

---

## 14. Summary

VPE is a:

- deterministic decision engine
- process compiler
- bridge between data and execution

It complements:
- workflow engines
- rules engines
- policy systems
- event-driven architectures

It enables:
- safer systems
- clearer logic
- faster iteration
- stronger guarantees