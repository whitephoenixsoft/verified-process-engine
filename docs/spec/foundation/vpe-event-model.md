# VPE Event Model Semantics
Version: Canonical v1

## 1. Purpose

This document defines the event model used by VPE.

It establishes:
- what an event represents
- how events relate to state
- how events are produced and consumed
- how traceability and lineage are preserved

This model is foundational to:
- runtime correctness
- migration integrity
- simulation accuracy
- auditability

---

## 2. Core Principle

Events are the **source of truth**.

State is a **derived projection**.

VPE does not trust state unless it is provable from events.

---

## 3. Event Categories

VPE defines the following event categories:

### 3.1 STATE_TRANSITION
Represents a successful state change.

Triggered by:
- runtime execution

---

### 3.2 MIGRATION
Represents a version upgrade (lift).

Triggered by:
- migration engine

---

### 3.3 (Future) SYSTEM / CUSTOM EVENTS
Reserved for:
- host-defined extensions
- orchestration-level signals

These must not violate determinism.

---

## 4. Event Structure

A VPE event must contain:

- event_id (unique identifier)
- trace_id (groups related events)
- event_kind (STATE_TRANSITION, MIGRATION, etc.)
- action (triggering action)
- timestamp (deterministic input or host-provided)
- state_before
- state_after

Optional fields:

- parent_event_id (links to prior event)
- metadata (structured payload)
- process (domain/process/version reference)

---

## 5. Trace and Lineage

### 5.1 trace_id

All events in a logical flow must share the same `trace_id`.

Purpose:
- correlate decisions
- support distributed tracing
- enable debugging and replay

---

### 5.2 parent_event_id

Defines causal linkage between events.

Rules:
- each event may reference a parent
- forms a chain (or DAG) of decisions
- enables full lineage reconstruction

---

### 5.3 Anchor Event

The **Anchor** is the latest STATE_TRANSITION event.

It represents:
- the current proven state
- the starting point for the next evaluation

Runtime REQUIREMENT:
- Anchor must be present
- Anchor.state_after must equal requested current_state

---

## 6. Event Ordering

Events must be:

- ordered by timestamp (logical time)
- consistent within a trace
- append-only

VPE does NOT:
- reorder events
- infer missing events
- resolve conflicts

Ordering is the responsibility of the host.

---

## 7. Event Production

### 7.1 Runtime

Produces:
- exactly one STATE_TRANSITION event per successful execution

Includes:
- previous_state
- next_state
- action
- timestamp

---

### 7.2 Migration

Produces:
- exactly one MIGRATION event per successful lift

Includes:
- from_version
- to_version
- previous_state
- new_state

---

### 7.3 Simulation

Produces:
- NO events

Simulation is:
- read-only
- analytical

---

## 8. Events vs Effects

This distinction is critical.

### Events
- facts
- must be persisted
- represent what happened

### Effects
- intent
- must NOT be persisted as facts
- represent what should happen externally

Example:

- Event: "Order moved from Pending → Approved"
- Effect: "Send confirmation email"

---

## 9. Host Responsibilities

The host must:

1. Persist events immutably
2. Maintain event ordering
3. Maintain trace_id consistency
4. Persist events atomically with state
5. Provide Anchor for each execution

The host must NOT:

- modify past events
- drop events
- create events outside VPE semantics (for VPE flows)
- treat effects as events

---

## 10. State Derivation

State is derived as:

latest_state = last_event.state_after

The system may:
- cache state
- store it in a database
- use projections

But:

State MUST always be reproducible from events.

---

## 11. Event Minimalism

VPE intentionally keeps events minimal.

Events should:
- capture transitions
- preserve traceability
- avoid embedding full context snapshots

Large payloads should:
- live in external systems
- be referenced via metadata if needed

---

## 12. Determinism Requirements

Events must be:

- reproducible from the same inputs
- independent of runtime environment
- independent of wall-clock unless explicitly provided

The same request must produce:
- identical event structure
- identical field values

---

## 13. Multi-Process Considerations (Future)

Future orchestration may involve:

- multiple processes emitting events
- cross-process trace_id propagation
- chained transitions across domains

Current guidance:

- maintain consistent trace_id across processes
- treat each process evaluation as producing its own event
- rely on host orchestration for sequencing

---

## 14. Error Handling

If execution fails:

- NO event is emitted

Errors must:
- be explicit
- be deterministic
- not mutate history

---

## 15. Integration Patterns

### 15.1 CRUD Systems

Minimum viable pattern:

- store current_state
- store last_event (Anchor)
- optionally store full history

---

### 15.2 Event-Sourced Systems

Recommended pattern:

- append all VPE events to stream
- rebuild projections from events
- use trace_id for correlation

---

## 16. Example

STATE_TRANSITION event:

{
  "event_id": "evt-123",
  "trace_id": "trace-abc",
  "event_kind": "STATE_TRANSITION",
  "action": "Submit",
  "timestamp": 1700000000,
  "state_before": "Draft",
  "state_after": "Submitted"
}

MIGRATION event:

{
  "event_id": "evt-456",
  "trace_id": "trace-abc",
  "event_kind": "MIGRATION",
  "timestamp": 1700001000,
  "from_version": "1.0.0",
  "to_version": "2.0.0",
  "state_before": "Pending",
  "state_after": "AwaitingPayment"
}

---

## 17. Key Takeaways

- Events are truth
- State is derived
- Events are immutable
- Effects are not events
- Anchor is required
- Traceability is mandatory
- Determinism is enforced

This model enables:
- replayability
- auditability
- safe evolution
- cross-system reasoning