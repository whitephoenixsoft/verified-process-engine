# Contributing to VPE
Version: Canonical v1

## 1. Purpose

This document defines how contributions to the Verified Process Engine (VPE) are evaluated and accepted.

VPE is not a general-purpose framework.  
It is a **deterministic decision engine with strict invariants**.

Contributions must preserve that foundation.

---

## 2. Core Principle

> Correctness, determinism, and explicitness take priority over convenience and feature expansion.

If a contribution improves ergonomics but weakens guarantees, it will be rejected.

---

## 3. What VPE Is (and Is Not)

### VPE Is
- a deterministic decision engine
- a compile-time validated system
- a stateless runtime
- a host-controlled orchestration boundary

### VPE Is Not
- a workflow engine
- a job scheduler
- a dynamic scripting runtime
- a “do everything” framework

Contributions that push VPE toward these non-goals will be rejected.

---

## 4. Contribution Categories

### High-Value Contributions
- bug fixes (correctness issues)
- compiler validation improvements
- manifest accuracy improvements
- performance improvements (without semantic change)
- documentation and examples
- CLI usability improvements (without changing semantics)
- additional **safe, deterministic built-in guards**

---

### Medium-Value Contributions
- non-breaking API ergonomics
- better error messages
- developer tooling improvements

---

### Low-Value / Likely Rejected
- implicit behavior
- hidden defaults
- fallback logic that bypasses invariants
- “magic” inference that reduces explicitness
- features that belong in the host system

---

## 5. Non-Negotiable Rules

All contributions must:

1. Preserve determinism  
   - no randomness  
   - no ambient time  
   - no hidden state  

2. Preserve stateless runtime  
   - no I/O  
   - no external calls  
   - no side effects  

3. Respect compile-time validation  
   - no runtime-only validation shortcuts  
   - no bypassing compiler guarantees  

4. Respect invariants  
   - anchor validation must not be weakened  
   - manifest requirements must remain explicit  
   - schema and type safety must be enforced  

5. Remain explicit  
   - no implicit data fetching  
   - no hidden assumptions  
   - no “best guess” logic  

---

## 6. Guards

Custom and built-in guards must:

- be deterministic
- be stateless after construction
- declare requirements explicitly
- not perform I/O
- not access global state

Guards that:
- hide dependencies
- infer missing data
- rely on runtime side effects

will be rejected.

---

## 7. Compiler Changes

The compiler is the **safety boundary** of VPE.

Changes must:
- increase correctness or clarity
- never weaken validation guarantees
- maintain deterministic compilation

Any change that allows invalid laws to compile will be rejected.

---

## 8. Runtime Changes

The runtime is a **pure evaluation engine**.

Changes must:
- preserve single-turn execution
- preserve deterministic transition selection
- avoid allocations or branching that introduce unpredictability

The runtime must never:
- mutate external state
- perform side effects
- depend on external systems

---

## 9. CLI Contributions

The CLI is a **thin harness over the Rust library**.

Rules:
- must not introduce new semantics
- must not bypass library validation
- must remain JSON-first
- must remain deterministic

CLI improvements should focus on:
- usability
- clarity
- diagnostics

---

## 10. Breaking Changes

Breaking changes are:

- rare
- deliberate
- heavily scrutinized

They must include:
- clear justification
- migration guidance
- updates to invariants and spec

---

## 11. Pull Request Guidelines

A good PR should:

- explain the problem clearly
- explain why the change fits VPE philosophy
- include tests
- include documentation updates if applicable

If a PR introduces new concepts, it should:
- align with existing invariants
- not introduce ambiguity

---

## 12. Issue Guidelines

When opening an issue:

- describe expected vs actual behavior
- include minimal reproducible examples
- specify schema/law/context if relevant

---

## 13. Philosophy Alignment

Before contributing, ask:

- Does this make behavior more deterministic or less?
- Does this make the system more explicit or more implicit?
- Does this strengthen compile-time guarantees?
- Would this make reasoning about the system harder?

If the answer trends negative, the change likely does not belong in VPE.

---

## 14. Forking

VPE is licensed under MIT.

If your use case requires:
- relaxed guarantees
- implicit behavior
- different trade-offs

you are encouraged to fork and adapt VPE to your needs.

This project will remain focused on:
- determinism
- explicitness
- verifiability

---

## 15. Summary

VPE is intentionally opinionated.

It optimizes for:
- correctness
- predictability
- long-term maintainability

Not:
- rapid feature expansion
- convenience shortcuts
- implicit behavior

Contributions are welcome — as long as they respect that foundation.