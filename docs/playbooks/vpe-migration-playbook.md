# VPE Migration Playbook
Version: Canonical v1

## 1. Purpose

This document explains how to use VPE migration ("lift") safely and effectively.

It is written for:
- developers using traditional CRUD/state-based systems
- developers using event sourcing or event-driven systems

The goal is to:
- explain how migration works
- define host responsibilities
- provide practical examples
- teach a reliable mental model for evolving systems

---

## 2. Migration Mental Model

Migration in VPE is called a **lift**.

A lift is:

- taking a record from an old version of a process
- applying deterministic transformation rules
- producing a new valid state under a new version

A lift does NOT:

- rewrite history
- mutate past events
- rely on external systems
- introduce non-deterministic behavior

A lift ALWAYS:

- evaluates guards
- applies transforms
- produces a new state + context
- emits a migration event

---

## 3. State vs Event (Important Concept)

### State
State is:
- the latest snapshot
- what your application reads quickly
- a convenience

### Event
An event is:
- a fact that happened
- immutable
- the source of truth

### Migration Event
A migration event is:
- proof that a record moved from version A → version B
- part of the permanent history
- required for traceability

---

## 4. Two Host Models

VPE supports both models:

### 4.1 CRUD / State-Based Systems

You likely:
- store rows in a database
- update columns directly
- do not maintain full event history

Good news:
You can still use VPE safely.

Minimum requirements:
- store current state
- store minimal event history (at least Anchor)
- append migration events
- persist state + event atomically

You do NOT need a full event store.

---

### 4.2 Event-Sourced Systems

You likely:
- store append-only event streams
- derive state via projections
- rely on replayability

VPE integrates naturally:

- emitted events become part of your stream
- migration events extend lineage
- projections update from events
- no rewriting required

---

## 5. What VPE Returns From Migration

A successful lift returns:

- new_state
- transformed_context
- migration_event

Example (conceptual):

{
  "new_state": "AwaitingPayment",
  "context": {
    "rec.first_name": "John",
    "rec.last_name": "Doe"
  },
  "event": {
    "event_kind": "MIGRATION",
    "from_version": "1.0.0",
    "to_version": "2.0.0"
  }
}

---

## 6. Host Responsibilities

After a successful lift, the host MUST:

1. Persist the new state
2. Persist the transformed context
3. Persist the migration event
4. Maintain trace_id and lineage
5. Perform all persistence atomically

The host MUST NOT:

- rewrite previous events
- drop migration events
- partially persist results
- treat effects as events

---

## 7. CRUD-Friendly Examples

### 7.1 Rename Field

Before:
- rec.user_name

After:
- rec.username

Transform:
- Move user_name → username

---

### 7.2 Split Field

Before:
- rec.full_name = "John Doe"

After:
- rec.first_name
- rec.last_name

Transform:
- parse full_name
- set new fields

---

### 7.3 Add Required Field

Before:
- no field

After:
- rec.account_status

Transform:
- set default value ("active")

---

### 7.4 Map Legacy Values

Before:
- rec.priority_code = "1"

After:
- rec.priority = "High"

Transform:
- mapping dictionary

---

### What This Replaces

Instead of:
- ad hoc migration scripts
- one-off SQL updates
- fragile backfills

You get:
- deterministic rules
- versioned logic
- repeatable transformations

---

## 8. Event-Sourced Examples

### 8.1 Append Migration Event

- append MIGRATION event to stream
- do not modify previous events

---

### 8.2 Projection Update

- apply migration event
- rebuild read model if needed

---

### 8.3 Divergent Landing States

- different records may land in different states
- based on guards

This is expected and correct.

---

### 8.4 Incompatible Records

If no rule applies:

- mark record as incompatible
- send to repair workflow
- do NOT force migration

---

## 9. Outcome Handling

### Seamless
- record transitions cleanly
- no behavior change
- safe to proceed

### Diverted
- record lands in different state
- requires awareness
- may need review

### Incompatible
- no valid rule
- requires manual intervention

---

## 10. What Not To Do

Do NOT:

- rewrite history
- drop events
- persist state without events
- execute effects during migration
- hide migration from audit logs

---

## 11. When Should You Use Migration?

You may NOT need migration immediately.

You SHOULD use migration when:

- schema changes
- fields are renamed or split
- new required data is introduced
- states are restructured
- business rules evolve across versions

Migration replaces:
- risky one-time scripts
- hidden logic in application code

---

## 12. Key Takeaways

- Migration is deterministic
- History is preserved
- Events are truth
- State is a projection
- The host is responsible for persistence
- VPE guarantees correctness of transformation

---

## 13. Final Guidance

You do NOT need to fully adopt event sourcing to use VPE.

You only need to:

- respect history
- append events
- persist atomically
- trust deterministic evaluation

VPE helps you evolve systems safely without hidden behavior.