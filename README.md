# VPE: Verified Process Engine

VPE is the **process brain of an application**.

It is a deterministic core for lawful process reasoning that helps teams stop scattering business-rule logic across controllers, services, jobs, workers, scripts, and integrations.

VPE answers one question extremely well:

> Given the process, the current context, and the relevant history, what is allowed to happen next?

It does this without owning your database, your workflows, your APIs, or your side effects.

**VPE decides. The host does.**

---

## Status

EARLY DEVELOPMENT

---

## Why VPE Exists

Most applications do not fail first because they lack business rules.
They fail because business-rule and process logic become **scattered**.

The same transition checks end up spread across:

- controllers
- services
- background jobs
- retries
- integration code
- scripts
- multiple teams' codepaths

That creates familiar problems:

- inconsistent behavior
- duplicated logic
- hidden process rules
- fragile migrations
- hard-to-audit systems
- difficult replay and debugging

Many teams eventually build private internal systems to centralize this kind of logic.
Others reach for workflow tools, rules engines, or low-code platforms.

VPE takes a different path.

It extracts the core process reasoning layer into a deterministic, reusable engine.

---

## What VPE Is

VPE belongs broadly to the **business-rule and process-governance** family.

But it is designed as a **deterministic process brain**, not as:

- a workflow engine
- a job scheduler
- a low-code platform
- a general orchestration runtime
- a traditional dynamic rules engine

VPE is the layer that decides what is lawful.
Your host application remains responsible for doing the work in the world.

---

## Core Boundary

The most important boundary in VPE is simple:

**VPE decides. The host does.**

VPE owns:

- lawful process reasoning
- deterministic evaluation
- transition legality
- context and history requirements
- emitted verdicts, events, and effect intent

The host owns:

- persistence
- external calls
- retries
- APIs
- queues
- orchestration
- side-effect execution
- transport and infrastructure

VPE is the **brain**, not the whole body.

---

## Core Philosophy

### Deterministic by Design

Every decision is evaluated as a deterministic function of explicit inputs.

`f(Process, Request) -> Verdict`

Given the same:

- process definition
- context
- relevant history

VPE produces the same result.

---

### History Is Part of Truth

VPE does not simply trust mutable current state.

It evaluates decisions against:

- Context: the current data supplied by the host
- Chronicle: the event/history slice required for evaluation

The latest relevant event, or **Anchor**, helps prove that the host is deciding from known truth rather than assumption.

---

### Verified Before Runtime

VPE compiles process definitions before execution.

The compiler rejects invalid process structure before it reaches runtime, including problems such as:

- invalid transitions
- unresolved references
- orphan states
- unsafe auto-transition patterns
- invalid schema usage

The goal is to move correctness checks earlier, not later.

---

### Stateless Runtime

The runtime does not:

- mutate your stores
- perform I/O
- call external systems
- execute effects directly

It evaluates one deterministic turn and returns a structured result.

---

### Host-Owned Reality

VPE is intentionally not your system runtime.

The host remains responsible for:

- loading data
- committing results
- dispatching effects
- coordinating workflows
- handling failures and retries

This boundary is central to the design.

---

## How VPE Fits Into a System

VPE is designed to sit inside an application architecture as the **lawful process reasoning layer**.

It fits well with:

- web backends that want one place for transition logic
- event-sourced systems that need deterministic evaluation from history
- service-oriented systems that want consistent process decisions across codepaths
- workflow/orchestration systems that need a strong decision core
- internal governance or approval systems that need explicit process law

Examples of the relationship:

- with workflow tools: **VPE decides -> workflow executes**
- with event sourcing: **events + context -> VPE -> lawful next step**
- with service architectures: **services ask VPE for consistent process decisions**

---

## How It Works

At a high level, VPE follows a consistent loop:

1. Define a process law and schema
2. Compile them into a verified process artifact
3. Ask what context and history are required for a given state
4. Load that data in the host
5. Execute one deterministic decision turn
6. Persist results and execute intended effects in the host

---

## Architecture Overview

VPE is centered around four primary layers.

### 1. Law

A declarative process definition describing:

- states
- transitions
- guards
- effects
- process structure

This is typically authored in JSON today.

### 2. Schema

A typed model describing the fields and values the law may reference.

This gives VPE explicit structure for:

- record data
- external input
- derived or reserved fields

### 3. Compiler

A validation and transformation pipeline that:

- enforces invariants
- validates law and schema usage
- resolves references
- builds a compiled process artifact
- derives manifests

### 4. Runtime

A deterministic evaluator that:

- accepts an explicit request
- evaluates guards in a stable order
- computes one lawful process result
- returns a Verdict

---

## Core Execution Model

VPE executes **one decision turn at a time**.

That turn is evaluated against:

- the compiled process
- the current state
- the requested action
- explicit context
- the relevant chronicle/history

The result is a **Verdict** that may include:

- the next state
- emitted events
- intended effects
- process outcome details

The Verdict is not the same thing as committed truth.
The host still decides whether and how to persist it.

---

## Key Concepts

### Law

The declarative definition of how a process may behave.

### Schema

The typed structure that law references.

### Manifest

A compile-time derived description of what data VPE needs for a given state or execution context.

This tells the host exactly what to load, including:

- required context fields
- required history/events

### Context

The current input map provided by the host.

Typical namespaces include:

- `rec.*` for record data
- `ext.*` for external host input
- `sys.*` for reserved/system values

### Chronicle

The slice of event history used for evaluation.

This includes the latest relevant event, or **Anchor**, plus any earlier required history.

### Verdict

The structured result of a single deterministic turn.

### Guards

Reusable logic primitives that evaluate process conditions.

### Effects

Structured effect intent emitted by VPE.

Effects are not executed by VPE. They are executed by the host.

---

## Concurrency and Commit Boundary

VPE evaluates against the truth the host provides.

That means a valid verdict is valid **relative to the supplied snapshot of truth**.
If another host advances the same process instance before commit, the verdict may become stale.

The intended model is:

- VPE decides from explicit known truth
- the host commits only if that truth is still current
- otherwise the host must discard or re-evaluate

This is part of the boundary implied by:

**VPE decides. The host does.**

---

## What Makes VPE Different

### A structured answer to scattered business rules

VPE gives process-heavy applications a dedicated structural center for lawful decisions.

### Deterministic and inspectable

The same process and inputs produce the same result.
That makes replay, debugging, and reasoning much stronger.

### Compiled rather than improvised

VPE validates process logic before runtime instead of letting correctness emerge from ad hoc host code.

### Host-bounded by design

VPE does not try to become your workflow runtime, scheduler, or full application platform.

### Tooling-friendly

Because law and schema are explicit artifacts, VPE naturally supports validation, manifests, simulation, migration, and CLI-based workflows.

---

## Features

### Deterministic execution

Replay decisions exactly across environments and over time.

### Compile-time safety

Reject invalid process structures before runtime.

### Manifest-driven data requirements

Know exactly what data is required before execution.

### Temporal and historical reasoning

Evaluate decisions using relevant event history.

### Migration support

Support process evolution without treating all historical data as disposable.

### Simulation support

Test how revised process logic would behave against existing histories.

---

## CLI

VPE includes a CLI that acts as a design-time and runtime harness for the core engine.

It supports workflows such as:

- validation
- compilation
- manifest inspection
- execution
- simulation
- migration

Example:

```sh
vpe validate --schema schema.json --law law.json
vpe execute --schema schema.json --law law.json --request request.json
```

The CLI is built on the same core semantics as the Rust library.
It may become a major professional shell for power users, but the engine remains the semantic source of truth.

---

## FFI and Embedding

VPE can be embedded outside Rust through a thin interoperability layer.

Potential host environments include:

- .NET
- Go
- Python
- others

The Rust core remains the reference implementation.
FFI exists to extend reach, not to redefine semantics.

---

## Example (Rust)

```rust
use vpe::prelude::*;

let registry = GuardRegistry::builder()
    .with_builtins()
    .build()?;

let engine = VpeEngine::builder()
    .with_registry(registry)
    .build()?;

engine.register_schema_json(schema_json)?;
engine.register_process_json(law_json)?;

let process = ProcessRef::new("Lending", "LoanApproval", "2.1.0");
let manifest = engine.manifest(&process, "Submitted")?;

let chronicle = load_history(&manifest)?;
let context = build_context();

let verdict = engine.execute(VpeRequest {
    process,
    trace_id: "trace-123".into(),
    now: 1_710_000_000,
    current_state: "Submitted".into(),
    action: "Evaluate".into(),
    context,
    chronicle,
})?;

persist(verdict)?;
dispatch(verdict.effects)?;
```

This example illustrates the core relationship:

- VPE determines the lawful result
- the host loads data, persists outcomes, and dispatches effects

---

## Design Principles

- Deterministic over clever
- Explicit over implicit
- Verified over assumed
- Process reasoning centralized, execution host-owned
- Replayability as a first-class feature
- Tooling that serves truth, not hidden behavior

---

## Project Direction

VPE is early, but the direction is clear.

It is being developed as:

- a strict deterministic core
- a reusable process-governance substrate
- a professional CLI/tooling ecosystem
- a foundation that could support richer authoring layers later

It is not being built first as a low-code shell or an all-in-one orchestration runtime.

---

## License

MIT License
