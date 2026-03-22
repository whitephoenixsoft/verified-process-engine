# VPE Development Sequencing Plan
Version: Canonical v1.1

## 1. Purpose

This document defines the implementation strategy for VPE.

It establishes:
- build order
- layer responsibilities
- development checkpoints
- the relationship between the Rust library, CLI, and FFI

This is the working plan for implementation unless explicitly revised.

---

## 2. Core Strategy

VPE will be built as:

- one canonical Rust library
- one CLI built in sync as the first real consumer
- one FFI layer added after core semantics stabilize

The CLI is not a separate implementation.  
The CLI is a thin harness over the Rust library.

The FFI is not a separate implementation.  
The FFI is a thin interoperability layer over the Rust library.

The **Compiler and Runtime are the core execution model**.  
The Engine is an orchestration layer built on top of them.

---

## 3. Delivery Surfaces

### Layer 1: Rust Library (Canonical Core)
The canonical implementation of:
- types
- schema
- registry
- compiler (design-time)
- runtime (execution)
- application layer (request orchestration)
- engine facade (optional orchestration)
- compiled process
- simulation
- migration

This layer defines all semantics.

---

### Layer 1.5: Application Layer (New)

A thin internal layer shared by CLI, Engine, and FFI.

Responsibilities:
- request normalization
- input validation (pre-runtime)
- orchestration of:
  - compile → runtime
  - manifest → data validation
- consistent error shaping

This layer:
- prevents duplication across CLI, Engine, and FFI
- is not exposed as a separate product surface
- must remain thin and deterministic

---

### Layer 2: CLI

The first operational harness for:
- validate
- compile
- manifest
- execute
- simulate
- lift

The CLI exists to:
- test product usability
- support local development
- support CI/CD
- act as a debugging and experimentation harness

The CLI must:
- remain thin
- call the application layer + library
- not duplicate logic

---

### Layer 3: FFI

The interoperability surface for:
- .NET
- Go
- Python
- other languages

The FFI must be added only after:
- core semantics are stable
- compiler and runtime APIs are proven
- CLI validates practical usability

---

## 4. Implementation Order

Implementation order is:

1. Core library (compiler + runtime + registry)
2. CLI in sync with core (early)
3. Application layer stabilization
4. Simulation & migration
5. FFI after semantics stabilize

This order is locked unless explicitly revised.

---

## 5. Development Method

For each feature slice:

1. Define the library API
2. Implement the feature in the Rust library
3. Expose the feature through the CLI
4. Use the CLI as the first usability test
5. Refine the library only if the CLI reveals a real product issue

This ensures:
- the library remains canonical
- the CLI remains practical
- the product is grounded in real usage

---

## 6. Phase Plan

### Phase 1: Compiler Foundation (Design-Time First)

Library:
- schema parsing & validation
- law parsing & validation
- registry (built-in guards)
- compiler:
  - validate()
  - compile()
- manifest generation
- compiled process structure
- digest generation
- validation & compilation reports

CLI:
- `vpe validate`
- `vpe compile`
- `vpe manifest`

Goals:
- design-time workflow works independently
- diagnostics are useful and structured
- manifests are correct and inspectable
- compiler usability is proven via CLI

---

### Phase 2: Runtime Foundation (Deterministic Turn)

Library:
- request model
- chronicle model (anchor + events)
- context model
- runtime evaluate()
- anchor validation
- verdict model
- application layer:
  - request normalization
  - invariant enforcement

CLI:
- `vpe execute`

Goals:
- one deterministic turn works end-to-end
- request shape is practical
- verdict is usable for persistence/orchestration
- CLI execute proves real-world usability

---

### Phase 3: Simulation Foundation

Library:
- simulation request model
- replay engine (prefix-based)
- divergence detection
- outcome classification
- simulation report

CLI:
- `vpe simulate`

Goals:
- replay uses runtime-equivalent logic
- divergence is explainable
- output is useful for humans and automation

---

### Phase 4: Migration Foundation

Library:
- migration rules
- transform execution (move/set/map/conditional)
- lift request/result
- migration event planning

CLI:
- `vpe lift`

Goals:
- deterministic version evolution works
- migration can be tested independently
- schema/law evolution is practical

---

### Phase 5: Engine (Orchestration Layer)

Library:
- process registration (schema + compiled process)
- process lookup
- execution orchestration
- version handling (optional)
- integration with migration (optional)

Goals:
- host-friendly embedding API
- not required for CLI usage
- does not redefine runtime/compiler semantics

---

### Phase 6: FFI Surface

Library:
- stabilize public API
- confirm runtime + compiler behavior

FFI:
- opaque engine pointer
- JSON request/response
- explicit free functions
- no panics across boundary

Goals:
- safe interoperability
- zero semantic drift
- no duplicated logic

---

## 7. Phase Exit Criteria

### Phase 1 Exit Criteria
- schema validation works
- law validation works
- compiler produces:
  - digest
  - manifests
  - reports
- compiled process is deterministic and immutable
- CLI validate/compile/manifest are usable in practice

---

### Phase 2 Exit Criteria
- runtime executes one deterministic turn
- anchor validation enforced
- verdict shape is stable
- CLI execute works with real JSON inputs
- no hidden state required

---

### Phase 3 Exit Criteria
- simulation uses runtime logic
- replay is prefix-based
- outcomes are correctly classified
- CLI simulate produces meaningful diagnostics

---

### Phase 4 Exit Criteria
- migration rules validate
- transforms are deterministic
- lift produces valid state or deterministic error
- CLI lift is usable

---

### Phase 5 Exit Criteria
- engine provides stable orchestration API
- engine does not duplicate compiler/runtime logic
- engine remains optional for CLI usage

---

### Phase 6 Exit Criteria
- FFI uses canonical APIs only
- memory is explicitly managed
- no panics cross boundary
- behavior matches Rust semantics

---

## 8. Synchronization Rules

### Library and CLI
The CLI must be developed in sync with the library.

Rules:
- CLI is added as soon as a feature exists
- CLI is the first usability test
- CLI must not duplicate logic
- CLI must use application layer when available

---

### Library and FFI
FFI must trail the library.

Rules:
- FFI does not define semantics
- FFI must not force API compromises
- Rust library remains canonical

---

## 9. API Stability Rules During Development

Before public release:
- iteration is allowed
- ergonomics are prioritized
- docs must stay aligned

After public release:
- changes require versioning discipline
- CLI behavior should remain stable
- FFI changes must be conservative

---

## 10. Documentation Rules During Development

- docs must be updated with behavior changes
- examples must remain realistic and executable in spirit
- CLI behavior must match CLI spec
- API docs must reflect actual usage patterns

Documentation is part of the product.

---

## 11. Testing Philosophy

### Library tests
- schema validation
- compiler validation
- manifest correctness
- runtime evaluation
- migration correctness
- simulation correctness

### CLI tests
- command integration tests
- JSON I/O tests
- deterministic output tests
- exit code tests

### FFI tests
- memory safety
- JSON marshaling
- semantic parity

---

## 12. Product Validation Philosophy

The CLI is the first real usability harness.

If a feature is hard to expose in CLI, it likely indicates:
- awkward API
- unclear data model
- weak diagnostics
- flawed product semantics

The CLI is both:
- a development tool
- a product validation tool

---

## 13. Non-Goals for Early Phases

Not required early:

- compiled artifact binary format
- runtime plugin system
- scripting for guards
- self-hosting CLI in VPE
- rich human-only CLI rendering
- advanced editor integrations
- broad FFI bindings

---

## 14. Locked Decisions

1. Rust library is canonical
2. CLI is first-class and built in sync
3. FFI follows after stabilization
4. CLI and FFI are thin wrappers
5. JSON is canonical CLI format
6. Design-time workflows are first-class
7. Compiler and runtime are separate
8. CompiledProcess is immutable
9. Registry is required at compile time
10. Same logic model powers:
   - compile
   - runtime
   - simulation
   - migration
11. CLI is the primary usability validation surface
12. Engine is orchestration, not the core

---

## 15. Summary

VPE is built as a unified decision platform with multiple surfaces.

Core truth:
- Compiler + Runtime + Registry

Delivery surfaces:
- Rust API
- CLI
- FFI

Build order:
- Core library first
- CLI in sync (early)
- Engine as orchestration
- FFI after stabilization

This ensures:
- strong semantics
- real usability
- minimal duplication
- a system that is testable, explainable, and scalable