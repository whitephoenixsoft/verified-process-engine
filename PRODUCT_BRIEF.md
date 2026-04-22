# VPE Product Brief
Version: Interim Revision

## 1. Overview

The Verified Process Engine (VPE) is the **process brain of an application**.

It exists to centralize lawful process reasoning so applications do not scatter business-rule and transition logic across controllers, services, jobs, scripts, workers, and integrations.

VPE is a deterministic, compiled process-governance core.
It separates:

- **what is allowed to happen next**
- from **how the host carries it out**

VPE does not replace orchestration, storage, messaging, or execution systems.
It gives them a more disciplined decision core.

A simple way to understand the boundary is:

**VPE decides. The host does.**

---

## 2. Problem Statement

Modern systems often accumulate a structural failure before they adopt better process tooling:

- business rules spread across multiple services and codepaths
- lawful transitions are checked inconsistently in different places
- process behavior becomes partly implicit and partly duplicated
- debugging requires tracing conditions across handlers, jobs, and integrations
- process changes require touching many areas of the host application
- confidence drops because there is no single authoritative place to ask what is allowed

This is not only a rules problem.
It is a structure problem.

Many teams eventually react by:

- building internal rules layers
- adopting workflow tooling
- creating admin or low-code shells
- re-architecting their applications

But the underlying pain is often the same:

**there is no dedicated structural center for lawful process reasoning.**

---

## 3. Product Position

VPE belongs broadly to the **business-rule and process-governance engine** family.

What makes it distinctive is not that it invents the idea of centralizing business logic. Many organizations build some version of this capability privately.

What VPE does differently is attempt to make that capability:

- deterministic
- compiled
- history-aware
- host-bounded
- architecture-first
- reusable as a core substrate rather than buried inside one product or low-code shell

VPE should not be understood primarily as:

- a workflow executor
- a job scheduler
- a message broker
- a low-code runtime
- a database
- a side-effect engine

It is best understood as:

- a deterministic process-governance core
- a compiled business-rule engine for lawful transitions
- a decision layer between application state and application execution

---

## 4. Core Model

VPE evaluates process logic as a deterministic function over explicit inputs.

Conceptually:

compile(Law, Schema, Registry) → CompiledProcess  
evaluate(CompiledProcess, Request) → Verdict

Where:

- **Law** defines the process declaratively
- **Schema** defines the typed process data model
- **Registry** provides executable guard implementations
- **Request** provides explicit context, state assumptions, and relevant history
- **Verdict** describes the lawful result of evaluation

VPE’s job is to determine what is allowed given explicit known truth.
It does not perform the resulting work itself.

---

## 5. Key Capabilities

### Process-Centered Business Rules

VPE centralizes process logic that would otherwise be scattered through host code.

This allows teams to:

- ask one authoritative layer what is legal
- keep transition logic inspectable
- reduce duplication across synchronous and asynchronous codepaths

### Deterministic Execution

Given the same compiled process and the same effective inputs, VPE produces the same result.

This supports:

- repeatability
- replayability
- stronger debugging
- more credible automation

### Compile-Time Validation

VPE validates process definitions before runtime.

This enables early detection of issues such as:

- invalid transitions
- illegal structural configurations
- type mismatches
- broken references
- incomplete process structures

### History-Aware Decisions

VPE can evaluate process legality using explicit historical context when needed.

This allows decisions to depend on:

- prior events
- temporal patterns
- frequency windows
- state-transition history

### Explicit Data Requirements

VPE can expose required inputs and required historical context rather than forcing the host to guess.

This makes integration more predictable and less ad hoc.

### Versioning, Simulation, and Migration

VPE is designed to support process evolution over time.

This includes:

- versioned process definitions
- safe change analysis
- deterministic migration/lift patterns
- replay and simulation against historical truth

---

## 6. Boundary and Responsibility Model

VPE’s strongest architectural rule is:

**VPE decides. The host does.**

### VPE is responsible for:

- lawful process reasoning
- transition legality
- deterministic evaluation
- consuming explicit context and history
- producing verdicts, emitted events, and effect intent
- process compilation and validation

### The host is responsible for:

- persistence
- orchestration
- transport and APIs
- retries and scheduling
- side-effect execution
- integration with external systems
- operational runtime control

This separation is deliberate.
VPE centralizes process reasoning, not application execution.

---

## 7. Concurrency and Commit Reality

VPE’s determinism does not by itself solve multi-host coordination.

A verdict is valid relative to the specific truth supplied in the request.
The host must only commit that verdict if that truth is still current at commit time.

In practice, this means:

- VPE decides from explicit known truth
- the host verifies that the relevant version or anchor is still current
- stale verdicts must not become new truth
- if truth has advanced, the host must re-evaluate

This preserves VPE’s role as the process brain without making it the entire runtime authority.

---

## 8. How VPE Fits in a System

Typical architecture:

Client / API / Event Source  
→ Host loads explicit state and history  
→ VPE evaluates legality and next result  
→ Host persists outcome atomically  
→ Host executes intended effects  
→ Future requests evaluate again

VPE sits between:

- the host’s known truth
- and the host’s real-world execution

It is the reasoning center, not the whole application body.

---

## 9. Relationship to Adjacent Categories

### Workflow Engines

Workflow systems usually handle:

- execution
- retries
- scheduling
- distributed coordination

VPE provides:

- deterministic process reasoning
- validated transition logic
- explicit legality boundaries

A workflow engine may call VPE for decisions.
VPE should not become the workflow engine.

### Rules Engines

Rules engines often evaluate stateless or loosely stateful business conditions.

VPE adds:

- process structure
- transition semantics
- explicit state progression
- historical reasoning
- stronger compile-time discipline

### State Machines

State machines provide structural transitions.

VPE adds:

- richer validation
- stronger authoring boundaries
- explicit data requirements
- historical and process-aware reasoning
- migration and simulation orientation

### Policy Engines

Policy systems answer whether something is allowed.

VPE extends this into:

- lawful multi-step process evolution
- state progression
- orchestratable effect intent
- process-aware transition evaluation

### Low-Code Process Tools

Many low-code systems expose a UI over hidden internal process or business-rule cores.

VPE is not currently positioned as a low-code product.
However, its architecture is strong enough that a low-code or admin-facing layer could eventually be built above it.

The current focus is the rigorous core, not the UI shell.

---

## 10. Example Use Cases

VPE is a good fit when process correctness matters and lawful transition logic is beginning to spread across multiple codepaths.

Examples include:

### Application and Platform Flows

- approvals
- onboarding flows
- entitlement changes
- review and escalation paths
- exception handling flows

### Financial and Risk-Oriented Systems

- transaction review
- fraud-related decision paths
- compliance-oriented process gating
- multi-step approval structures

### Event-Driven and Distributed Systems

- deterministic decision layers for event consumers
- process interpretation over historical event streams
- host-side orchestration informed by validated process outcomes

### Long-Lived Business Processes

- evolving business rules over time
- replayable decision histories
- simulation before process upgrades
- migration across process versions

---

## 11. Product Surfaces

### Rust Library

The Rust implementation is the canonical semantic core.
It is the primary authority for:

- compilation
- evaluation
- validation
- manifests
- simulation and migration behavior

### CLI

The CLI is a professional shell around the core.
It makes VPE feel like a real language/tooling ecosystem through:

- validation
- compilation
- inspection
- execution harnesses
- simulation workflows
- debugging support

The CLI may become a major power-user surface, but it must remain tooling around semantic truth, not a competing semantic authority.

### FFI and Embedding

FFI support is intended for broader host integration.
Early FFI may prioritize simplicity and debuggability.
Long-term embedding should remain evolvable based on real usage patterns.

---

## 12. Design Principles

VPE is built on:

- determinism
- explicitness
- compile-time validation
- replayability
- separation of concerns
- host-bounded execution
- structural clarity for process truth

These principles are part of the product identity, not just the implementation style.

---

## 13. Why VPE

VPE helps teams:

- move scattered business-rule logic into one process-centered authority layer
- make process behavior more explicit and inspectable
- validate process structure earlier
- replay and reason about outcomes with more confidence
- evolve process logic without burying it in host code

It reduces:

- duplicated transition checks
- hidden process behavior
- architecture drift in business logic
- runtime surprises caused by under-structured process code

---

## 14. Vision

VPE aims to become a durable process-governance core for modern systems.

Its long-term value is not only that it executes decisions, but that it gives process-heavy applications a dedicated structural center for lawful reasoning.

A future shaped by VPE looks like this:

- process logic is compiled instead of improvised
- business-rule structure is explicit instead of scattered
- decisions are replayable instead of opaque
- orchestration consumes lawful process outcomes instead of inventing them ad hoc
- organizations can rely on a real process core instead of burying one inside private infrastructure or low-code shells

---

## 15. Summary

VPE is the process brain of an application.

It belongs to the business-rule and process-governance family, but is designed as a deterministic, compiled, host-bounded core for lawful process reasoning.

It exists to solve the scattered business-rule problem by giving systems a dedicated structural center for deciding what is allowed to happen next.

Its core boundary is simple:

**VPE decides. The host does.**
