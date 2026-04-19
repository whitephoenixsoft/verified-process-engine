# VPE Law Reference
Version: Canonical v2

## 1. Overview

This document defines the exact structure and rules of a VPE Law.

A Law is a JSON document that describes:
- states
- transitions
- guards
- effects
- migration rules

All fields are validated at compile time.

---

## 2. Root Object

Required fields:

- domain: string  
- process: string  
- version: string  
- schema_version: string  
- initial_state: string  
- states: array  
- migration_rules: array (optional)

Example:

{
  "domain": "OrderManagement",
  "process": "OrderFlow",
  "version": "1.0.0",
  "schema_version": "1.0.0",
  "initial_state": "Draft",
  "states": [],
  "migration_rules": []
}

---

## 3. Law and Schema Binding

The Law must explicitly declare:

- `schema_version`

Compiler MUST enforce:

- law.domain == schema.domain
- law.schema_version == schema.version

No implicit matching or inference is allowed.

---

## 4. State Definition

Fields:

- name: string (required)
- transitions: array (optional)
- is_transient: boolean (optional, default false)

Example:

{
  "name": "PendingPayment",
  "is_transient": true,
  "transitions": []
}

Constraints:

- State names must be unique
- All referenced states must exist
- initial_state must match one of the states

---

## 5. Transition Definition

Fields:

- action: string or "AUTO_TICK" (required)
- to: string (required)
- priority: number (optional, default 0)
- guards: array (optional)
- effects: array (optional)
- metadata/comment: object or string (optional)

Example:

{
  "action": "Submit",
  "to": "Approved",
  "priority": 10,
  "guards": [],
  "effects": []
}

Constraints:

- "to" must reference a valid state
- priority must be deterministic (integer recommended)
- transitions are grouped by action at runtime

---

## 6. Action Semantics

- action is a string identifier
- "AUTO_TICK" represents automatic transitions

Rules:

- AUTO_TICK transitions:
  - require no external action
  - must not form cycles
  - must be bounded
  - execute automatically while valid

---

## 7. Guard Definition

Fields:

- type: string (required)
- additional fields depend on guard type

Example:

{
  "type": "GreaterThan",
  "path": "rec.amount",
  "value": 1000
}

Constraints:

- type must exist in GuardRegistry
- all required fields must be present
- types must match schema

---

## 8. Guard Requirements

Each guard declares:

- required history
- required context fields

Compiler guarantees:

- all requirements are included in state manifest
- missing requirements → compilation error

---

## 9. Effect Definition

Effects are structured objects.

Fields:

- type: string (required)
- mode: string (optional) `tracked | untracked` (default: untracked)
- target: string (optional)
- action: string (optional)
- params: object (optional)
- handlers: object (optional, tracked only)

Example:

{
  "mode": "tracked",
  "type": "WebHook",
  "target": "Payments",
  "action": "Charge",
  "params": {
    "order_id": "rec.order_id"
  }
}

Constraints:

- effects are not executed by VPE
- effects must be serializable
- tracked effects must follow saga constraints

---

## 10. Effect Classification

### Tracked Effects

- influence business correctness
- require explicit resolution

Rules:

- transitions with tracked effects must land in transient states
- transient state must define at least one exit path
- completion must be proven via events

---

### Untracked Effects

- best-effort side effects
- do not influence correctness

Rules:

- may land in non-transient states
- do not require outcome events
- must not introduce hidden dependencies

---

## 11. Saga Constraints

If a transition contains tracked effects:

- target state must have `is_transient = true`

Transient states must:

- define at least one exit transition
- allow forward progression (no dead ends)

---

## 12. Namespace Rules

Allowed prefixes:

- sys.*
- rec.*
- ext.*
- calc.*

Rules:

- sys.* is read-only and system-defined
- rec.* is mutable
- ext.* is read-only
- calc.* is read-only

All paths must:

- use dot notation
- match identifier rules
- exist in schema or system schema

---

## 13. Identifier Rules

All identifiers must:

- be alphanumeric or underscore
- not start with a digit
- use dot notation for paths
- be case-sensitive

Examples:

Valid:
- rec.order_total
- ext.user_id

Invalid:
- rec.order-total
- rec.123value

---

## 14. Schema Validation

All rec.*, ext.*, calc.* paths must:

- exist in Domain Schema
- match declared type

sys.* paths must:

- exist in system schema

Type mismatches cause compilation failure.

---

## 15. Migration Rules

Structure:

{
  "from_state": "OldState",
  "to_state": "NewState",
  "guards": [],
  "transforms": []
}

Fields:

- from_state: string (required)
- to_state: string (required)
- guards: array (optional)
- transforms: array (optional)

Constraints:

- to_state must exist in target process
- guards must be valid
- transforms must be valid

---

## 16. Transform Operations

Supported operations:

Move:

{
  "op": "move",
  "from": "old_field",
  "to": "rec.new_field"
}

Set:

{
  "op": "set",
  "target": "rec.flag",
  "value": true
}

Map:

{
  "op": "map",
  "target": "rec.priority",
  "from": "legacy_rank",
  "mapping": {
    "1": "High",
    "2": "Medium"
  }
}

Constraints:

- cannot write to sys.*, ext.*, or calc.*
- must respect schema types

---

## 17. Conditional Transforms

Structure:

{
  "guards": [],
  "ops": []
}

Rules:

- guards evaluated first
- ops applied only if guards pass

---

## 18. Compilation Rules

Compilation must fail if:

- unknown guard type
- unknown state reference
- schema/law mismatch
- invalid identifier
- AUTO_TICK cycle
- saga violation
- missing manifest requirements

Warnings may be issued for:

- unreachable states
- ambiguous transitions (same action + same priority)
- unused manifest requirements

---

## 19. Determinism Rules

All laws must be:

- deterministic
- side-effect free
- independent of runtime environment

No:

- randomness
- external calls
- implicit time access

---

## 20. Manifest Output

For each state:

- required history (anchor + events)
- required context fields

Manifest is:

- deterministic
- complete
- compile-time derived

---

## 21. Digest

Each compiled law produces a digest:

- deterministic hash of structure
- used for change detection
- used for version validation

---

## 22. Compatibility Rules

A law is valid only if:

- all references resolve
- schema matches explicitly
- invariants are satisfied
- compilation succeeds

No partial success is allowed.