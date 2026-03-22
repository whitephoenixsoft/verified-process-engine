# VPE Development Sequencing Plan
Version: Canonical v1

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

---

## 3. Delivery Surfaces

### Layer 1: Rust Library
The canonical implementation of:
- types
- schema
- registry
- compiler
- runtime
- engine facade
- compiled process
- simulation
- migration

This layer defines all semantics.

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

The CLI must remain thin and call the Rust library only.

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

1. Rust library core
2. CLI in sync with core
3. FFI after semantics stabilize

This order is locked unless explicitly revised.

---

## 5. Development Method

For each feature slice:

1. Define the library API
2. Implement the feature in the Rust library
3. Expose the feature through the CLI
4. Use the CLI as the first usability test
5. Refine the library only if the CLI reveals a real product issue

This keeps:
- the library canonical
- the CLI practical
- the product grounded in real usage

---

## 6. Phase Plan

### Phase 1: Compiler Foundation
Library:
- schema parsing
- law parsing
- registry
- compiler validate
- compiler compile
- manifest generation
- compiled process artifact
- reports

CLI:
- `vpe validate`
- `vpe compile`
- `vpe manifest`

Goals:
- design-time workflow works
- diagnostics are useful
- manifests are inspectable
- compiler usability is proven

---

### Phase 2: Runtime Foundation
Library:
- request model
- chronicle model
- anchor validation
- runtime execute
- verdict model
- engine install
- engine execute

CLI:
- `vpe execute`

Goals:
- one deterministic turn works
- verdict is practical
- host-style execution is testable through CLI

---

### Phase 3: Simulation Foundation
Library:
- simulation request model
- replay engine
- divergence classification
- simulation report

CLI:
- `vpe simulate`

Goals:
- historical replay works
- policy change analysis is practical
- simulation output is useful to humans and automation

---

### Phase 4: Migration Foundation
Library:
- migration rules
- transform execution
- lift request/result
- migration event planning

CLI:
- `vpe lift`

Goals:
- version evolution works deterministically
- migration can be tested independently
- law/schema evolution is practical

---

### Phase 5: FFI Surface
Library:
- stabilize public API
- confirm runtime and compiler semantics

FFI:
- opaque engine handle
- JSON request/response
- explicit free functions
- no panics across boundaries

Goals:
- safe interoperability
- no semantic drift from Rust library
- no duplicate logic

---

## 7. Phase Exit Criteria

### Phase 1 Exit Criteria
- schema validates correctly
- laws validate correctly
- compiler emits manifests and digest
- compiled process is immutable and installable
- CLI validate/compile/manifest are usable

---

### Phase 2 Exit Criteria
- runtime executes one deterministic turn
- anchor validation works
- verdict shape is stable enough for host usage
- CLI execute works with JSON request/response

---

### Phase 3 Exit Criteria
- simulation uses runtime-equivalent logic
- replay is prefix-based
- simulation outcomes are classified correctly
- CLI simulate produces useful reports

---

### Phase 4 Exit Criteria
- migration rules validate correctly
- transforms are deterministic
- lift produces valid landing state or deterministic error
- CLI lift is usable

---

### Phase 5 Exit Criteria
- FFI uses canonical library APIs only
- all cross-boundary memory is explicitly managed
- FFI behavior matches Rust semantics
- panic-free ABI boundary is enforced

---

## 8. Synchronization Rules

### Library and CLI
The CLI must be developed in sync with the library.

This means:
- the CLI is added as soon as a library slice exists
- the CLI is used to pressure-test the library API
- the CLI must never contain duplicated VPE logic

---

### Library and FFI
FFI must trail the library.

This means:
- no FFI work should define semantics
- no FFI work should force premature API compromises
- the library remains the canonical source of truth

---

## 9. API Stability Rules During Development

Before public release:
- iteration is allowed
- ergonomics are prioritized
- docs must stay aligned with implementation

After public release:
- public API changes require deliberate versioning
- CLI behavior should remain stable where possible
- FFI changes must be especially conservative

---

## 10. Documentation Rules During Development

As implementation proceeds:
- docs must be updated before or with behavior changes
- examples must remain executable in spirit
- CLI behavior must match CLI spec
- API docs must match actual library surface

Documentation is part of the product and must evolve with the code.

---

## 11. Testing Philosophy

Testing will be layered:

### Library tests
- parser tests
- compiler validation tests
- runtime tests
- manifest tests
- migration tests
- simulation tests

### CLI tests
- command integration tests
- JSON input/output tests
- deterministic output tests
- exit code tests

### FFI tests
- memory safety tests
- JSON marshaling tests
- semantic parity tests

---

## 12. Product Validation Philosophy

The CLI is the first real usability harness.

If a feature is difficult to expose cleanly through the CLI, it may indicate:
- awkward library API
- unclear data contract
- weak diagnostics
- unclear product semantics

The CLI is therefore a development tool and a product validation tool.

---

## 13. Non-Goals for Early Phases

The following are explicitly not required in the earliest implementation phases:

- artifact binary serialization format
- self-hosting the CLI in VPE
- rich human-only CLI rendering
- advanced editor integrations
- complex plugin packaging
- broad FFI language bindings

These may be added later.

---

## 14. Locked Decisions

The following are locked for implementation planning:

1. Rust library is canonical
2. CLI is first-class and built in sync with library
3. FFI follows after core semantics stabilize
4. CLI and FFI must remain thin wrappers
5. JSON is the canonical machine-readable CLI format
6. Design-time workflows are first-class
7. Compiler and runtime remain separate concerns
8. CompiledProcess is immutable and installable
9. Registry is compile-time only
10. The same logic model powers compile, runtime, simulation, and migration

---

## 15. Summary

VPE will be implemented as one core platform with multiple delivery surfaces.

Build order:

- Core library first
- CLI in sync
- FFI after stabilization

This plan ensures:
- strong semantics
- practical usability
- minimal duplication
- a product that is testable, teachable, and stable