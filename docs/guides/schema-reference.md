# VPE Schema Reference
Version: Canonical v2

## 1. Purpose

This document defines the **formal, compiler-enforced specification** of the VPE Domain Schema.

It is the authoritative reference for:
- field definitions
- type system
- namespace resolution
- validation rules

The compiler MUST enforce all rules defined here.

---

## 2. Schema Definition

A schema is a JSON object with the following structure:

{
  "domain": "string",
  "version": "string",
  "namespaces": {
    "rec": [ FieldDefinition ],
    "ext": [ FieldDefinition ],
    "calc": [ FieldDefinition ]
  }
}

Notes:
- `rec`, `ext`, and `calc` are required namespaces (may be empty arrays)
- `sys` is NOT allowed in schema (reserved by VPE)

---

## 3. FieldDefinition

Each field is defined as:

{
  "name": "string",
  "type": "VpeType",
  "description": "string (optional)",
  "enum_values": ["string"] (optional)
}

---

## 4. VpeType

Supported types:

- String
- Number
- Boolean
- DateTime
- Duration
- Enum

---

## 5. Type Semantics

### String
- UTF-8 string
- no implicit coercion

### Number
- JSON number
- treated as double precision or integer (implementation-defined)
- must support comparison operations

### Boolean
- true / false

### DateTime
- represented as Unix timestamp (integer seconds)
- must support comparison operations
- must be compatible with `sys.now`

### Duration
- represented as integer seconds
- used for time windows and comparisons

### Enum
- restricted string values
- must include `enum_values` in definition

Example:

{
  "name": "tier",
  "type": "Enum",
  "enum_values": ["Gold", "Silver", "Bronze"]
}

---

## 6. Namespaces

### Overview

Field access is namespaced:

- `rec.*` → Record fields (schema-defined, writable)
- `ext.*` → External fields (schema-defined, read-only)
- `calc.*` → Derived fields (schema-defined, read-only)
- `sys.*` → System fields (built-in, read-only)

---

### Namespace Resolution Rules

Given a path:

rec.order_total

The compiler must:

1. Split prefix and field:
   - prefix = rec
   - field = order_total

2. Resolve namespace:
   - rec → Record
   - ext → External
   - calc → Derived
   - sys → System

3. Validate field existence:
   - required for rec/ext/calc
   - validated against built-in schema for sys

---

## 7. Namespace Constraints

### rec.*
- must exist in schema
- writable
- used for persistent state

### ext.*
- must exist in schema
- read-only
- provided by host

### calc.*
- must exist in schema
- read-only
- derived externally

### sys.*
- not defined in schema
- read-only
- provided by engine or host
- validated against built-in system schema

---

## 8. Naming Conventions

### Field Names

- must be descriptive and domain-relevant
- should use snake_case
- must follow identifier rules

Examples:

✔ valid:
- order_total
- customer_id
- payment_status

❌ invalid:
- order total
- order-total
- 1amount

---

### Namespace Usage

Paths must always be explicit:

✔ valid:
- rec.order_total
- ext.user_id

❌ invalid:
- order_total
- user_id

---

### Enum Values

- should be stable and descriptive
- must be strings
- should use PascalCase or UPPER_CASE consistently

✔ valid:
- Approved
- Pending
- REJECTED

---

## 9. Identifier Rules

All identifiers must:

1. Use dot notation:
   namespace.field

2. Follow naming rules:
   - alphanumeric + underscore only
   - no spaces
   - no special characters
   - cannot start with digit

3. Be case-sensitive

---

## 10. Path Validation

For every path used in:

- guards
- transforms
- effects (parameters)

The compiler must:

1. Validate structure (namespace + field)
2. Validate namespace is allowed
3. Validate field exists (including sys via system schema)
4. Resolve type

---

## 11. Type Validation Rules

For any operation:

path OP value

The compiler must ensure:

1. Field type matches operation
2. Value type matches field type

---

### Examples

✔ Valid:

rec.amount (Number) > 100

✔ Valid:

rec.status (Enum) == "Approved"

❌ Invalid:

rec.amount (Number) == "100"

❌ Invalid:

rec.status (Enum) == 1

---

## 12. Enum Validation

For Enum fields:

1. `enum_values` must be provided
2. must contain at least one value
3. values must be unique
4. values must follow identifier rules
5. runtime values must match one of the allowed values

---

## 13. Write Constraints

### Allowed Writes

- rec.*

### Forbidden Writes

- sys.*
- ext.*
- calc.*

Compiler MUST reject:

{
  "target": "sys.now"
}

---

## 14. Schema Lookup

The compiler must maintain:

Map<NamespaceCategory, Map<FieldName, VpeType>>

Example:

rec → { order_total: Number }

---

## 15. Guard Type Compatibility

Each guard defines expected input types.

Compiler must enforce compatibility.

Example:

GreaterThan:
- requires Number or DateTime

OccurredWithin:
- requires Duration

---

## 16. Temporal Validation

### DateTime comparisons

Allowed:
- DateTime vs DateTime
- DateTime vs sys.now

### Duration usage

Used in:
- time windows
- elapsed checks

---

## 17. Missing Fields

If a field is referenced but not defined:

→ Compilation MUST fail

---

## 18. Unknown Namespaces

If prefix is not:

- sys
- rec
- ext
- calc

→ Compilation MUST fail

---

## 19. Duplicate Fields

Duplicate field names within the same namespace:

→ Compilation MUST fail

---

## 20. Schema Versioning

Each schema is uniquely identified by:

- domain
- version

The compiler must:

- allow multiple versions
- bind process to exact schema version

---

## 21. Schema Binding

During compilation:

- Law must explicitly declare `schema_version`
- Schema domain must match law domain
- Schema version must match law.schema_version
- all paths must resolve against that schema

---

## 22. System Schema

VPE provides a built-in system schema.

Examples include:

- sys.now → DateTime
- sys.trace_id → String

Properties:
- read-only
- always available at compile time
- must be explicitly provided at runtime if required

---

## 23. Determinism Requirement

Schema must not introduce:

- dynamic typing
- runtime-dependent resolution
- implicit conversions

---

## 24. Error Conditions

Compiler MUST error on:

- unknown field
- invalid namespace
- type mismatch
- invalid enum value
- invalid identifier
- illegal write target
- schema/law mismatch

---

## 25. Minimal Schema Example

{
  "domain": "Example",
  "version": "1.0.0",
  "namespaces": {
    "rec": [
      { "name": "amount", "type": "Number" },
      { "name": "status", "type": "Enum", "enum_values": ["A", "B"] }
    ],
    "ext": [],
    "calc": []
  }
}

---

## 26. Internal Representation (Conceptual)

Schema should compile to:

NamespaceCategory → FieldName → VpeType

Example:

rec → amount → Number  
rec → status → Enum  

---

## 27. Summary

The schema is:

- the type system of VPE
- the validator of all field access
- the contract between host and engine

Strict enforcement ensures:

- deterministic execution
- compile-time safety
- predictable evolution