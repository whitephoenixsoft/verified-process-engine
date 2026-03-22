# VPE Design Philosophy
Version: Canonical v1

## 1. Purpose

This document explains the design philosophy behind the Verified Process Engine (VPE).

It exists to answer:

- Why VPE is designed the way it is
- What trade-offs are intentional
- What problems VPE chooses to solve — and not solve

This document is a guide for:
- maintainers
- contributors
- advanced users

---

## 2. The Core Idea

> Business logic should be deterministic, explicit, and verifiable.

Most systems fail not because they lack logic, but because their logic is:
- scattered
- implicit
- inconsistent
- hard to reason about

VPE exists to make logic:
- centralized
- structured
- provable

---

## 3. Determinism Over Convenience

VPE is built on a strict rule:

> The same inputs must always produce the same outputs.

This means:

- no ambient time
- no randomness
- no hidden state
- no external calls during evaluation

Why?

Because determinism enables:
- replayability
- debugging
- auditing
- confidence

Convenience features that weaken determinism are intentionally rejected.

---

## 4. Explicitness Over Magic

VPE avoids implicit behavior.

There is:
- no hidden data loading
- no automatic fallbacks
- no “best guess” logic

Everything required for a decision must be:
- declared
- provided
- validated

Why?

Because implicit systems:
- hide complexity
- create edge cases
- break under change

Explicit systems:
- are predictable
- are testable
- are understandable

---

## 5. Compile-Time Safety Over Runtime Recovery

VPE prefers failing early over recovering late.

The compiler enforces:
- valid states
- valid transitions
- valid references
- safe side-effect patterns

Invalid logic is rejected before runtime.

Why?

Because:
- runtime failures are expensive
- debugging production behavior is hard
- invalid logic should never execute

---

## 6. Stateless Runtime

The runtime is intentionally limited.

It:
- does not perform I/O
- does not mutate external systems
- does not persist data

It only:
- evaluates
- selects transitions
- emits a verdict

Why?

Because separating:
- **decision** from **execution**

creates systems that are:
- testable
- composable
- scalable

---

## 7. History as a First-Class Concept

VPE treats history as essential, not optional.

Decisions are based on:
- current context
- event history (Chronicle)

The latest event (Anchor) proves the current state.

Why?

Because real systems are temporal:
- “what happened before” matters
- state alone is not trustworthy
- history enables richer logic

---

## 8. Manifests Over Guessing

VPE does not guess what data is needed.

Instead, it computes:
- exact context requirements
- exact history requirements

at compile time.

Why?

Because:
- data loading becomes predictable
- performance becomes controllable
- missing data becomes impossible to ignore

---

## 9. The Host Owns Side Effects

VPE never executes effects.

It emits intent:

- “send email”
- “charge account”
- “trigger workflow”

The host system decides:
- how
- when
- if

Why?

Because:
- side effects are environment-specific
- coupling execution reduces flexibility
- separation improves reliability

---

## 10. Opinionated Boundaries

VPE is intentionally opinionated.

It does not try to:
- be a workflow engine
- replace orchestration systems
- handle infrastructure concerns
- execute long-running processes

Why?

Because systems that try to do everything:
- become complex
- lose clarity
- are harder to maintain

VPE focuses on one thing:

> Making correct decisions.

---

## 11. Stability Over Churn

VPE is designed to evolve slowly.

The goal is:
- stable semantics
- predictable behavior
- long-term trust

Not:
- constant feature expansion
- rapid redesign

Why?

Because:
- business logic must be durable
- breaking changes are costly
- trust is built on consistency

---

## 12. Composition Over Control

VPE is not the center of your system.

It is a component.

It integrates with:
- web services
- event systems
- workflow engines
- microservices

Why?

Because:
- systems are heterogeneous
- flexibility matters
- control belongs to the host

---

## 13. Trade-Offs

VPE makes deliberate trade-offs:

### Chosen
- determinism
- explicitness
- safety
- verifiability

### Sacrificed
- convenience shortcuts
- implicit behavior
- rapid flexibility
- “just make it work” features

These trade-offs are intentional.

---

## 14. When Not to Use VPE

VPE may not be appropriate when:

- logic is trivial
- determinism is not required
- workflows are purely sequential and simple
- rapid prototyping is more important than correctness

VPE is best suited for:
- complex decision systems
- high-value business logic
- systems requiring auditability

---

## 15. Guiding Questions

When making design decisions, ask:

- Does this make behavior more deterministic?
- Does this make the system more explicit?
- Does this strengthen compile-time guarantees?
- Does this improve long-term reasoning?

If not, it likely does not belong in VPE.

---

## 16. Final Thought

VPE is not trying to be clever.

It is trying to be:

- correct
- predictable
- durable

The goal is not to impress.

The goal is to build something that engineers can trust years from now.