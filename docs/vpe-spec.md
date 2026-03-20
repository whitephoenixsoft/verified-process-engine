# Verified Process Engine (VPE) Specification
Version: Canonical v1

## 1. Purpose
VPE is a deterministic process engine implemented in Rust. It compiles declarative laws into optimized structures and evaluates them against explicit inputs to produce decisions.

## 2. Execution Model
compile(Law, Schema, Registry) -> CompiledProcess  
evaluate(CompiledProcess, Request) -> Verdict

## 3. Core Components

### Law
Defines states, transitions, guards, and effects.

### Domain Schema
Defines all valid typed fields.

### Registry
Maps guard/effect identifiers to Rust implementations.

### Compiler
Validates and builds compiled processes.

### Runtime
Evaluates transitions deterministically.

### Migration Engine
Handles version upgrades via transforms.

### Simulation Engine
Replays history for analysis.

---

## 4. Canonical Law Structure

- domain
- process
- version
- initial_state
- states[]
- migration_rules[]

Each state contains transitions.

Each transition contains:
- action
- target state
- priority
- guards
- effects

---

## 5. Guards

Guards implement:
- check(context, history) -> bool
- get_requirements() -> dependencies

Guards are:
- stateless
- deterministic
- precompiled

---

## 6. Effects

Effects are:
- structured envelopes
- emitted only (not executed)
- handled externally

---

## 7. Compiler Pipeline

1. Parse
2. Schema validation
3. Identifier validation
4. Type validation
5. Graph construction
6. Topology validation
7. Cycle detection
8. Saga validation
9. Guard/effect compilation
10. Manifest generation
11. Digest generation

---

## 8. Runtime Algorithm

1. Validate Anchor
2. Resolve state index
3. Filter transitions by action
4. Sort by priority
5. Evaluate guards
6. Return first match
7. Else return error

---

## 9. Manifest System

Each state declares:
- required history
- required context

Used by host to fetch minimal data.

---

## 10. Migration

Migration includes:
- rule selection
- guard evaluation
- transforms
- conditional transforms
- landing state

---

## 11. Simulation

Simulation:
- replays history
- detects divergence
- classifies outcome

---

## 12. Rust Crate Design

### Core Principle
Rust crate is first-class. FFI is a thin wrapper.

### Crate Structure

- vpe-core
  - compiler
  - runtime
  - registry
  - migration
  - simulation
  - types

- vpe-ffi (optional feature/crate)

### Feature Flags

- default: core engine only
- ffi: enables C ABI
- serde: JSON support
- simulation: enables simulation module
- migration: enables migration module

### Public API Strategy

Expose:
- Guard trait
- Effect trait (optional)
- Engine facade
- Request/Response types

Hide:
- internal graph structures
- compiled node layout

### Extension Model

Users implement:
- Guard trait
- optional Effect handlers

Then register via:
- GuardRegistry

### Zero-Cost Abstraction Goal

- no dynamic dispatch in hot path if avoidable
- use indices instead of strings
- minimize allocations during runtime

---

## 13. FFI Model

- opaque engine pointer
- JSON request/response
- explicit free functions

---

## 14. Error Model

Errors must be:
- deterministic
- structured
- non-panicking

Categories:
- compile errors
- runtime errors
- migration errors
- simulation errors
