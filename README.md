# VPE: Verified Process Engine

VPE is a deterministic decision engine written in Rust.  
It compiles declarative business logic into a verified, high-performance runtime that evaluates decisions as pure functions.

Instead of scattering business rules across services, controllers, and workflows, VPE centralizes them into a versioned, auditable, and replayable system of truth.

---

## Why VPE?

Most systems implement business logic as:
- nested conditionals
- duplicated validation rules
- implicit workflows spread across codebases

This leads to:
- inconsistent behavior
- hard-to-reason systems
- fragile migrations
- poor auditability

VPE replaces this with:

Logic as a compiled, deterministic system

---

## Core Philosophy

### Deterministic by Design
Every decision is a pure function:

`f(Process, Request) → Verdict`

Given the same:
- process definition
- context
- history

You always get the same result.

---

### History is Truth
State is not trusted unless it is proven.

VPE evaluates decisions against:
- Context (current data)
- Chronicle (event history)

The latest event (the Anchor) is required to prove correctness.

---

### Verified Before Runtime
VPE does not allow unsafe logic into production.

The compiler enforces:
- no infinite auto-loops
- no orphan states
- no invalid transitions
- no unsafe side-effect patterns

Errors happen at compile time, not in production.

---

### Stateless Runtime
The engine:
- does not mutate data
- does not perform I/O
- does not call external systems

It only:
- evaluates
- decides
- emits a Verdict

---

### Host Owns Reality
Your application remains in control of:
- persistence
- external calls
- workflows
- APIs

VPE acts as the decision boundary, not the system owner.

---

## How It Works

VPE follows a simple, consistent loop:

1. Define your process ("Law")
2. Compile and register it
3. Ask VPE what data it needs (Manifest)
4. Provide context + history
5. Execute one decision
6. Persist results and execute effects

---

## Architecture

VPE is composed of four primary layers:

### The Law
A declarative definition of:
- states
- transitions
- guards (conditions)
- effects (intent)

Typically defined in JSON.

---

### The Compiler
A validation and transformation pipeline that:
- enforces invariants
- resolves all references
- builds an optimized graph
- generates manifests

---

### The Registry
A mapping between:
- string identifiers in the Law
- Rust implementations (Guards)

This allows extension without changing the engine.

---

### The Runtime
A high-performance evaluator that:
- executes one state + action
- evaluates guards in priority order
- returns a deterministic Verdict

---

## Example (Rust)

```rust
use vpe::prelude::*;

// Build engine
let registry = GuardRegistry::builder()
    .with_builtins()
    .build()?;

let engine = VpeEngine::builder()
    .with_registry(registry)
    .build()?;

// Register schema + process
engine.register_schema_json(schema_json)?;
engine.register_process_json(law_json)?;

// Ask what data is needed
let process = ProcessRef::new("Lending", "LoanApproval", "2.1.0");
let manifest = engine.manifest(&process, "Submitted")?;

// Load required history + context (host responsibility)
let chronicle = load_history(&manifest)?;
let context = build_context();

// Execute one deterministic turn
let verdict = engine.execute(VpeRequest {
    process,
    trace_id: "trace-123".into(),
    now: 1_710_000_000,
    current_state: "Submitted".into(),
    action: "Evaluate".into(),
    context,
    chronicle,
})?;

// Host persists + executes effects
persist(verdict)?;
dispatch(verdict.effects)?;
```
---

## The Verdict

Every execution returns a structured result:

- next state
- effects (intent only)
- emitted events
- state changes

VPE does not execute effects.  
It tells you what should happen — your system decides how.

---

## Key Concepts

### Context
A flat map of inputs:

- rec.* → record data
- ext.* → external inputs
- sys.* → system values (e.g., time)

---

### Chronicle
A slice of history used for evaluation.

Includes:
- the latest event (Anchor)
- relevant past events

---

### Manifest
Generated at compile time.

Defines exactly what data is required:
- which history events
- which context fields

This enables:
- minimal data loading
- predictable performance

---

### Guards
Reusable logic primitives implemented in Rust.

Examples:
- Equals
- GreaterThan
- OccurredWithin (temporal logic)

---

### Effects
Structured instructions emitted by VPE:

- send email
- call service
- enqueue job

Handled entirely by the host.

---

## Features

### Deterministic Execution
Replay decisions exactly across time and environments.

### Compile-Time Safety
Invalid logic never reaches runtime.

### Temporal Logic
Reason over history:
- "3 attempts in 24 hours"
- "event occurred recently"

### Lazy Migration
Upgrade records between versions only when touched.

### Saga Safety
Enforces:
- success paths
- failure paths
- timeout handling

---

## Performance

Designed for high-throughput systems:

- Sub-microsecond to low-microsecond evaluation
- Index-based graph traversal (no string lookups)
- Minimal allocations at runtime
- Manifest-driven data access

---

## Use Cases

VPE fits naturally into:

- Web application backends
- Event-sourced systems
- Workflow orchestration
- Microservice coordination
- Decision engines / rule systems

---

## FFI Support

VPE can be used outside Rust via a C-compatible interface:

- .NET
- Go
- Python
- others

The Rust library remains the primary implementation.  
FFI is a thin interoperability layer.

---

## Design Principles

- Deterministic over clever
- Explicit over implicit
- Verified over assumed
- Host-controlled side effects
- Replayability as a first-class feature

---

## License

Apache 2.0