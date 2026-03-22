# VPE Maintenance & Stability Policy
Version: Canonical v1

## 1. Purpose

This document defines how the Verified Process Engine (VPE) is maintained over time.

It establishes:
- expectations for stability
- update philosophy
- compatibility guarantees
- how the project signals ongoing health

VPE is designed to be **long-lived infrastructure**, not a rapidly changing framework.

---

## 2. Core Philosophy

VPE prioritizes:

- **Determinism over novelty**
- **Stability over churn**
- **Correctness over rapid iteration**

The goal is not frequent change.

The goal is:
> A system that becomes stable because it is well-designed.

---

## 3. Semantic Stability

The following are considered **core semantics** and are expected to change rarely:

- execution model
- invariants
- law structure
- schema model
- guard contract
- manifest system
- verdict structure

### Guarantees

1. Core semantics will remain stable once validated in production use.
2. Breaking changes will be:
   - rare
   - explicit
   - versioned
   - documented with migration guidance
3. Deterministic behavior will never be compromised for convenience.

---

## 4. What Will Continue to Evolve

Even with stable semantics, VPE will continue to evolve in:

### Compatibility
- Rust version support
- dependency updates
- platform compatibility

### Ergonomics
- API improvements (non-breaking where possible)
- CLI usability
- developer experience

### Tooling
- CLI capabilities
- validation tooling
- debugging and inspection tools

### Documentation
- examples
- guides
- API clarity

### Bug Fixes
- correctness issues
- edge cases
- performance improvements

---

## 5. Release Philosophy

Releases are driven by **value**, not cadence.

Types of releases:

### Patch Releases
- bug fixes
- dependency updates
- documentation improvements

### Minor Releases
- non-breaking improvements
- new CLI capabilities
- additional built-in guards

### Major Releases
- rare
- reserved for intentional semantic evolution
- include migration guidance

---

## 6. Signals of Project Health

VPE aims to remain visibly maintained through:

- periodic releases (even if small)
- responsive issue triage
- updated documentation
- maintained examples and CLI usage
- compatibility updates

Lack of frequent feature releases does **not** indicate abandonment.

---

## 7. Backwards Compatibility

VPE strives for strong backwards compatibility.

### Principles

1. Existing Laws should continue to compile and execute across compatible versions.
2. Breaking changes require:
   - clear versioning
   - documented rationale
   - migration path
3. Deterministic behavior must remain consistent across versions for identical inputs.

---

## 8. CLI and FFI Stability

### CLI
- remains a thin wrapper over the Rust API
- JSON contracts are treated as stable interfaces
- breaking CLI changes are versioned and documented

### FFI
- treated as a stable boundary once released
- memory safety and determinism are strictly enforced
- changes are conservative and compatibility-focused

---

## 9. Contribution Philosophy

Contributions are welcome, but must align with VPE principles:

- preserve determinism
- respect invariants
- avoid introducing implicit behavior
- prefer explicit, verifiable logic

Priority is given to:
- correctness
- clarity
- long-term maintainability

---

## 10. Non-Goals

VPE does not aim to:

- rapidly add features without clear value
- chase trends or frameworks
- become a full workflow engine
- compromise determinism for convenience

---

## 11. Long-Term Vision

The long-term goal for VPE is:

- **semantic stability**
- **predictable behavior across time**
- **high trust as a decision layer**

Success looks like:

> VPE changes slowly because it was designed well,  
> but remains actively maintained, reliable, and trusted.

---

## 12. Summary

VPE is designed to be:

- stable, not stagnant
- maintained, not noisy
- predictable, not surprising

It is infrastructure.

And infrastructure earns trust by being:
- correct
- consistent
- durable