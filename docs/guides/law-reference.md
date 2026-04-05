# VPE Law Reference
Version: Canonical v1

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
- initial_state: string  
- states: array  
- migration_rules: array (optional)

Example:

```json
{
  "domain": "OrderManagement",
  "process": "OrderFlow",
  "version": "1.0.0",
  "initial_state": "Draft",
  "states": [],
  "migration_rules": []
}
```
---

## 3. State Definition

Fields:

- name: string (required)
- transitions: array (optional)
- is_transient: boolean (optional, default false)

Example:

```json
{
  "name": "PendingPayment",
  "is_transient": true,
  "transitions": []
}
```
Constraints:

- State names must be unique
- All referenced states must exist
- At least one state must match initial_state

---

## 4. Transition Definition

Fields:

- action: string or "AUTO_TICK" (required)
- to: string (required)
- priority: number (optional, default 0)
- guards: array (optional)
- effects: array (optional)
- metadata: object (optional)

Example:

```json
{
  "action": "Submit",
  "to": "Approved",
  "priority": 10,
  "guards": [],
  "effects": []
}
```
Constraints:

- "to" must reference a valid state
- priority must be deterministic (integer recommended)
- transitions are grouped by action at runtime

---

## 5. Action Semantics

- action is a string identifier
- "AUTO_TICK" represents automatic transitions

Rules:

- AUTO_TICK transitions:
  - require no external action
  - must not form cycles
  - must be bounded

---

## 6. Guard Definition

Fields:

- type: string (required)
- additional fields depend on guard type

Example:

```json
{
  "type": "GreaterThan",
  "path": "rec.amount",
  "value": 1000
}
```
Constraints:

- type must exist in GuardRegistry
- all required fields must be present
- types must match schema

---

## 7. Guard Requirements

Each guard declares:

- required history
- required context fields

Compiler guarantees:

- all requirements are included in state manifest
- missing requirements → compilation error

---

## 8. Effect Definition

Effects are structured objects.

Fields:

- type: string (required)
- mode: string (optional) `[tracked | untracked (default)]`
- target: string (optional)
- action: string (optional)
- params: object (optional)
- handlers: object (optional)

Example:

```json
{
  "mode": "trecked",
  "type": "WebHook",
  "target": "Payments",
  "action": "Charge",
  "params": {
    "order_id": "rec.order_id"
  }
}
```
Constraints:

- effects are not executed by VPE
- effects must be serializable
- transitions with effects of mode `tracked` must land in transient states

---

## 9. Saga Constraints

If a transition contains `tracked` effects:

- target state must have is_transient = true

Transient states must:

- define exit transitions
- include timeout or failure path

---

## 10. Namespace Rules

Allowed prefixes:

- sys.*
- rec.*
- ext.*
- calc.*

Rules:

- sys.* is read-only
- rec.* is mutable
- ext.* is read-only
- calc.* is derived

All paths must:

- use dot notation
- match identifier rules
- exist in schema (except sys.*)

---

## 11. Identifier Rules

All identifiers must:

- be alphanumeric or underscore
- not start with a digit
- use dot notation for paths

Example valid:

rec.order_total  
ext.user_id  

Invalid:

rec.order-total  
rec.123value  

---

## 12. Schema Validation

All rec.*, ext.*, calc.* paths must:

- exist in Domain Schema
- match declared type

Type mismatches cause compilation failure.

---

## 13. Migration Rules

Structure:

```json
{
  "from_state": "OldState",
  "to_state": "NewState",
  "guards": [],
  "transforms": []
}
```
Fields:

- from_state: string (required)
- to_state: string (required)
- guards: array (optional)
- transforms: array (optional)

Constraints:

- to_state must exist in target version
- guards must be valid
- transforms must be valid

---

## 14. Transform Operations

Supported operations:

Move:

```json
{
  "op": "move",
  "from": "old_field",
  "to": "rec.new_field"
}
```
Set:
```json
{
  "op": "set",
  "target": "rec.flag",
  "value": true
}
```
Map:
```json
{
  "op": "map",
  "target": "rec.priority",
  "from": "legacy_rank",
  "mapping": {
    "1": "High",
    "2": "Medium"
  }
}
```
Constraints:

- cannot write to sys.*
- must respect schema types

---

## 15. Conditional Transforms

Structure:
```json
{
  "guards": [],
  "ops": []
}
```
Rules:

- guards evaluated first
- ops applied only if guards pass

---

## 16. Compilation Rules

Compilation must fail if:

- unknown guard type
- unknown state reference
- schema mismatch
- invalid identifier
- AUTO_TICK cycle
- saga violation
- missing manifest requirements

Warnings may be issued for:

- unreachable states
- shadowed transitions
- empty guard requirements

---

## 17. Determinism Rules

All laws must be:

- deterministic
- side-effect free
- independent of runtime environment

No:

- randomness
- external calls
- implicit time access

---

## 18. Manifest Output

For each state:

- required history (anchor + events)
- required context fields

Manifest is:

- deterministic
- complete
- compile-time derived

---

## 19. Digest

Each compiled law produces a digest:

- deterministic hash of structure
- used for change detection
- used for version validation

---

## 20. Compatibility Rules

A law is valid only if:

- all references resolve
- schema matches
- invariants are satisfied
- compilation succeeds

No partial success is allowed.