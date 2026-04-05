# VPE Law Patterns & Anti-Patterns
Version: Canonical v1

## 1. Purpose

This document provides real-world patterns and anti-patterns for writing VPE Laws.

It focuses on:
- practical design decisions
- maintainability
- correctness under evolution
- avoiding production issues

---

## 2. Pattern: Simple Approval Flow

Use when:
- linear process
- minimal branching

```json
{
  "name": "Draft",
  "transitions": [
    {
      "action": "Submit",
      "to": "Approved",
      "guards": []
    }
  ]
}
```
Why it works:
- clear intent
- minimal complexity
- easy to reason about

---

## 3. Pattern: Priority-Based Branching

Use when:
- multiple decision paths
- clear precedence rules

```json
{
  "name": "Submitted",
  "transitions": [
    {
      "action": "Evaluate",
      "to": "AutoApproved",
      "priority": 10,
      "guards": [
        { "type": "LessThan", "path": "rec.amount", "value": 1000 }
      ]
    },
    {
      "action": "Evaluate",
      "to": "ManualReview",
      "priority": 1,
      "guards": []
    }
  ]
}
```
Why it works:
- deterministic ordering
- explicit fallback
- avoids ambiguity

---

## 4. Pattern: Explicit Fallback

Always define a fallback.
```json
{
  "guards": []
}
```
Why:
- prevents runtime errors
- documents default behavior

---

## 5. Pattern: State Decomposition

Instead of:

❌ One complex state

Use:

✔ Multiple smaller states

Example:

Draft → Validation → Review → Approved

Why:
- improves readability
- simplifies guards
- easier to evolve

---

## 6. Pattern: Saga (External Interaction)

Use when:
- external systems are involved

Flow:

1. Trigger effect
2. Move to transient state
3. Wait for outcome

```json
{
  "action": "SubmitPayment",
  "to": "PendingPayment",
  "effects": [ 
    "mode": "tracked"
    ... 
  ]
}
```
Transient state:

```json
{
  "name": "PendingPayment",
  "is_transient": true,
  "transitions": [
    { "action": "SUCCESS", "to": "Paid" },
    { "action": "FAILURE", "to": "Failed" },
    {
      "action": "AUTO_TICK",
      "to": "Expired",
      "guards": [{ "type": "TimeElapsed", "seconds": 300 }]
    }
  ]
}
```
Why it works:
- prevents stuck processes
- models real-world async behavior

---

## 7. Pattern: Temporal Enforcement

Use history-based guards.

```json
{
  "type": "OccurredWithin",
  "target_action": "Login",
  "window_seconds": 3600
}
```
Why:
- supports rate limiting
- enables fraud detection
- enforces SLAs

---

## 8. Pattern: Guard Composition

Prefer multiple simple guards:

✔ Good:
```json
[
  { "type": "GreaterThan", "path": "rec.amount", "value": 1000 },
  { "type": "Equals", "path": "ext.tier", "value": "Premium" }
]
```
❌ Bad:

One large custom guard doing everything

Why:
- reusable
- testable
- manifest-friendly

---

## 9. Pattern: Explicit Timeout

Every transient state must have a timeout.

```json
{
  "action": "AUTO_TICK",
  "to": "Failed",
  "guards": [{ "type": "TimeElapsed", "seconds": 300 }]
}
```
Why:
- prevents stuck records
- guarantees progress

---

## 10. Pattern: Migration with Minimal Change

Prefer small transformations.

✔ Good:
- rename fields
- set defaults

❌ Bad:
- large structural rewrites

Why:
- safer upgrades
- easier debugging

---

## 11. Anti-Pattern: Giant State

❌ One state with many transitions

Problems:
- hard to reason about
- prone to overlap bugs
- difficult to test

Fix:
→ split into multiple states

---

## 12. Anti-Pattern: Missing Fallback

❌ No default transition

Problem:
- runtime failure
- undefined behavior

Fix:
→ always include fallback

---

## 13. Anti-Pattern: Hidden Data Dependency

❌ Guard uses a field not clearly defined

Problem:
- runtime failure
- unclear manifest

Fix:
→ ensure schema + manifest alignment

---

## 14. Anti-Pattern: No Timeout in Saga

❌ Transient state without AUTO_TICK

Problem:
- record gets stuck forever

Fix:
→ always define timeout path

---

## 15. Anti-Pattern: Overloaded Guards

❌ Complex custom logic in one guard

Problem:
- hard to debug
- hard to reuse

Fix:
→ split into smaller guards

---

## 16. Anti-Pattern: Business Logic in Effects

❌ Using effects to encode logic

Problem:
- breaks determinism model
- hides decision rules

Fix:
→ keep logic in guards

---

## 17. Anti-Pattern: Ambiguous Priorities

❌ Same priority for overlapping guards

Problem:
- non-obvious behavior

Fix:
→ use clear priority ordering

---

## 18. Anti-Pattern: Schema Drift

❌ Using fields inconsistently across versions

Problem:
- migration complexity
- bugs in guards

Fix:
→ evolve schema deliberately

---

## 19. Design Heuristics

A good law:

- reads top-to-bottom clearly
- has explicit fallback paths
- uses small, composable guards
- isolates side effects in transient states
- is easy to simulate

---

## 20. Summary

Patterns help you:

- write readable laws
- avoid runtime errors
- design for evolution

Anti-patterns help you:

- avoid production issues
- keep systems maintainable

A well-written law is:

- predictable
- explicit
- safe to change