# VPE Surfaces and Responsibilities

Status: Draft  
Scope: Responsibility boundaries among the CLI, engine API, application API, and simulation capabilities  
Audience: Project architect, core implementers, API designers, future contributors

---

## 1. Purpose

This document defines the responsibility boundaries among the major VPE surfaces.

It exists to:

- clarify what each surface is for
- prevent overlap and semantic drift between surfaces
- support future API re-architecture and documentation work
- reinforce which surface should be used for which kind of work
- keep the product understandable as it grows

This document is not a workflow guide. It is a boundary and responsibility guide.

---

## 2. Core Principle

VPE decides. The host does.

Every surface in VPE must preserve this rule.

No surface should blur the distinction between:

- lawful process reasoning
- host-owned execution in the world

This is the highest-level boundary. All other surface boundaries are subordinate to it.

---

## 3. Surface Model

VPE currently has four major interaction surfaces:

- CLI
- engine API
- application API
- simulation capabilities

These surfaces are related, but they are not interchangeable.

They should be understood as different access layers around one semantic core.

---

## 4. The Semantic Center

Before describing the surfaces, the semantic center must be clear.

The semantic center of VPE is the deterministic process core.

It includes:

- law and schema semantics
- compilation semantics
- runtime decision semantics
- manifests or required-data semantics
- event/effect meaning
- migration and simulation semantics

No outer surface may redefine this center.

The surfaces exist to expose, integrate, inspect, and use this center in different ways.

---

## 5. CLI Responsibilities

### 5.1 Primary role

The CLI is VPE’s professional shell.

It is the primary surface for:

- authoring feedback
- validation
- compilation
- inspection
- debugging
- direct experimentation
- simulation from the command line

### 5.2 What the CLI is for

The CLI should be used when a developer wants to:

- validate a draft process
- compile a process artifact
- inspect outputs and diagnostics
- experiment with explicit requests
- debug why a process behaves a certain way
- compare or simulate process changes outside of host code

### 5.3 What the CLI is not for

The CLI is not the main runtime application integration surface.

It should not be treated as:

- the production host runtime
- the persistence layer
- the orchestration layer
- the semantic authority itself

### 5.4 Boundary rule

The CLI may extend tooling, but not truth.

That means the CLI may add:

- commands
- renderers
- reports
- debugging aids
- scaffolding and generation workflows

But it may not redefine:

- runtime legality
- compilation truth
- event/effect meaning
- process semantics

### 5.5 Architectural classification

The CLI is a tooling and workflow surface.  
It is not the semantic source of truth.

---

## 6. Engine API Responsibilities

### 6.1 Primary role

The engine API is the semantic and platform integration surface.

It is responsible for exposing the core engine and its lower-level integration model.

### 6.2 What the engine API is for

The engine API should be used when a developer or platform needs to:

- register schema and law
- compile or drive compilation through engine-managed flows
- install and manage compiled processes
- perform lower-level engine integration
- access raw or more exact process/runtime constructs
- support CLI, FFI, or platform-level use cases

### 6.3 What the engine API is not for

The engine API is not the ideal surface for the normal shape of application decision code.

It should not be the first surface most host business code sees if a cleaner application-facing path exists.

### 6.4 Boundary rule

The engine API owns semantic and platform-level exactness.

It should remain the right place for:

- compiler-aligned integration
- raw artifact access
- exact runtime requests and verdicts
- specialized host or infrastructure use

### 6.5 Architectural classification

The engine API is the core programmatic integration surface closest to the semantic center.

---

## 7. Application API Responsibilities

### 7.1 Primary role

The application API is the preferred application-facing integration surface for normal decision execution.

It should make ordinary host-side VPE use feel natural.

### 7.2 What the application API is for

The application API should be used when host application code wants to:

- look up an installed process
- inspect what data is required
- build a decision input
- execute a normal decision turn
- receive an outcome suitable for persistence and effect dispatch planning

### 7.3 What the application API is not for

The application API is not the surface for:

- compiler internals
- deep artifact manipulation
- CLI implementation
- raw tooling integration
- migration and simulation internals unless deliberately wrapped later
- platform assembly concerns

### 7.4 Important clarification

The application API should not be framed as merely a beginner path.

It should be the preferred application integration surface, even for sophisticated hosts, including event-sourced systems, as long as the caller wants an application-shaped decision model.

### 7.5 Boundary rule

The application API may reduce ceremony, but it may not hide correctness.

It must remain explicit enough about:

- current state
- supplied truth/history
- anchor or equivalent truth marker
- host commit responsibility
- effect intent vs host execution

### 7.6 Architectural classification

The application API is the host-facing runtime surface.

It is not semantically authoritative, but it should be first-class.

---

## 8. Simulation Responsibilities

### 8.1 Primary role

Simulation is a developer and team analysis surface.

It exists to help teams understand how process changes behave before release.

### 8.2 What simulation is for

Simulation should be used for:

- process regression testing
- comparing current and proposed process versions
- detecting seamless, diverted, and incompatible outcomes
- historical replay
- release-readiness analysis

### 8.3 What simulation is not for

Simulation is not only a specialized CLI command and not only an advanced feature.

It is a distinct product value surface.

It should not be treated merely as:

- an incidental debugging extra
- a hidden implementation detail of the CLI
- a low-priority add-on

### 8.4 Boundary rule

Simulation should remain aligned with the same semantic center as normal execution.

It may compare or analyze outcomes, but it must not invent alternate truth models.

### 8.5 Architectural classification

Simulation is an analysis and evolution surface.

It sits alongside runtime execution as a core reason to use VPE.

---

## 9. How the Surfaces Relate

The surfaces should be understood in this relationship:

- the semantic center defines truth
- the engine API exposes truth programmatically at a lower level
- the application API exposes truth for normal host runtime use
- the CLI exposes truth for human workflows, diagnostics, and experimentation
- simulation exposes truth for process evolution, regression, and release safety

None of these surfaces should be allowed to collapse into each other completely.

That would create confusion and blur responsibilities.

---

## 10. Recommended Usage Mapping

### 10.1 Use the CLI when

Use the CLI when the goal is:

- authoring feedback
- validation
- compilation
- debugging
- direct request experimentation
- ad hoc simulation and inspection

### 10.2 Use the engine API when

Use the engine API when the goal is:

- registering or installing processes
- integrating VPE at platform level
- accessing more exact or lower-level semantic constructs
- supporting infrastructure-oriented integration

### 10.3 Use the application API when

Use the application API when the goal is:

- normal decision execution in host code
- service-layer use
- handler or worker integration
- ordinary runtime use of an installed process

### 10.4 Use simulation when

Use simulation when the goal is:

- regression testing
- law/process change analysis
- historical replay
- release-readiness assessment

---

## 11. Overlap Rules

Some overlap is acceptable, but only in controlled ways.

### 11.1 CLI and engine API

The CLI may rely on engine semantics internally, but it must not become a second semantic engine.

### 11.2 Engine API and application API

The application API may wrap the engine API, but it must not replace or redefine it.

### 11.3 CLI and simulation

The CLI may expose simulation, but simulation must also be understood as a wider capability, especially for code-based testing and replay workflows.

### 11.4 Application API and simulation

In the future, application-facing simulation helpers may exist, but simulation should retain its identity as an analysis surface rather than being absorbed into ordinary runtime execution.

---

## 12. Surface Drift Risks

The main drift risks are:

### 12.1 CLI drift

The CLI becomes so powerful that users start treating CLI behavior as semantic truth.

### 12.2 Application API drift

The application API becomes so convenience-focused that it hides correctness boundaries or begins acting like a host framework.

### 12.3 Engine API drift

The engine API becomes overloaded with application-facing convenience concerns that belong in the application layer.

### 12.4 Simulation drift

Simulation is treated as optional tooling instead of as a first-class process evolution and release-readiness capability.

These risks should be actively watched.

---

## 13. Design Rules for Future Changes

When adding features, ask:

### 13.1 Which surface owns this concern?

A feature should have a clear home.  
It should not be added everywhere by default.

### 13.2 Does this feature change truth or expose truth?

If it changes truth, it belongs in the semantic center first.  
If it exposes truth, it may belong in a surface.

### 13.3 Does this reduce ceremony or hide correctness?

Reducing ceremony is good.  
Hiding correctness is not.

### 13.4 Does this strengthen the process brain, or expand the body?

If it expands the body, it may still be useful, but it likely belongs outside the core or in a later layer.

---

## 14. Final Summary

The VPE surfaces are not interchangeable.

They exist to serve different responsibilities around one deterministic semantic center.

- The CLI is the professional shell for authoring, validation, debugging, and experimentation.
- The engine API is the semantic and platform integration surface.
- The application API is the preferred host-facing surface for normal decision execution.
- Simulation is the analysis and evolution surface for testing, comparison, replay, and release readiness.

Each of these surfaces adds value. None of them should be allowed to redefine the semantic center.

The governing rule remains:

VPE decides. The host does.