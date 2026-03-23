# VPE Schema Reference
Version: Canonical v1

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
  "fields": [ FieldDefinition ]
}

---

## 3. FieldDefinition

Each field is defined as:

{
  "name": "string",
  "type": "VpeType",
  "description": "string (optional)",
  "enum": ["string"] (optional)
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

---

### Number
- JSON number
- treated as double precision or integer (implementation-defined)
- must support comparison operations

---

### Boolean
- true / false

---

### DateTime
- represented as Unix timestamp (integer seconds)
- must support comparison operations
- must be compatible with `sys.now`

---

### Duration
- represented as integer seconds
- used for time windows and comparisons

---

### Enum
- restricted string values
- must include `enum` array in definition

Example:

{
  "name": "tier",
  "type": "Enum",
  "enum": ["Gold", "Silver", "Bronze"]
}

---

## 6. Namespaces

### Overview

Field access is namespaced:

- `rec.*` → Record fields (schema-defined)
- `ext.*` → External fields (schema-defined)
- `calc.*` → Derived fields (schema-defined)
- `sys.*` → System fields (implicit)

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
   - not required for sys

---

## 7. Namespace Constraints

### rec.*
- must exist in schema
- writable
- used for persistent state

---

### ext.*
- must exist in schema
- read-only
- provided by host

---

### calc.*
- must exist in schema
- read-only
- derived externally

---

### sys.*
- not defined in schema
- read-only
- provided by engine or host

---

## 8. Identifier Rules

All identifiers must:

1. Use dot notation:
   namespace.field

2. Follow naming rules:
   - alphanumeric + underscore
   - no spaces
   - no special characters
   - cannot start with digit

3. Be case-sensitive

---

## 9. Path Validation

For every path used in:

- guards
- transforms
- effects (parameters)

The compiler must:

1. Validate structure (namespace + field)
2. Validate namespace is allowed
3. Validate field exists (except sys.*)
4. Resolve type

---

## 10. Type Validation Rules

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

rec.status (String Enum) == "Approved"

❌ Invalid:

rec.amount (Number) == "100"

❌ Invalid:

rec.status (Enum) == 1

---

## 11. Enum Validation

For Enum fields:

1. Value must be string
2. Value must exist in enum list

---

## 12. Write Constraints

### Allowed Writes

- rec.*

---

### Forbidden Writes

- sys.*
- ext.*
- calc.*

Compiler MUST reject:

{
  "target": "sys.now"
}

---

## 13. Schema Lookup

The compiler must maintain:

Map<NamespaceCategory, Map<FieldName, VpeType>>

Example:

rec → { order_total: Number }

---

## 14. Guard Type Compatibility

Each guard defines expected input types.

Compiler must enforce compatibility.

Example:

GreaterThan:
- requires Number or DateTime

OccurredWithin:
- requires Duration

---

## 15. Temporal Validation

Special rules apply:

### DateTime comparisons

Allowed:
- DateTime vs DateTime
- DateTime vs sys.now

---

### Duration usage

Used in:
- time windows
- elapsed checks

---

## 16. Missing Fields

If a field is referenced but not defined:

→ Compilation MUST fail

---

## 17. Unknown Namespaces

If prefix is not:

- sys
- rec
- ext
- calc

→ Compilation MUST fail

---

## 18. Duplicate Fields

Duplicate field names in schema:

→ Compilation MUST fail

---

## 19. Schema Versioning

Each schema is uniquely identified by:

- domain
- version

The compiler must:

- allow multiple versions
- bind process to exact schema version

---

## 20. Schema Binding

During compilation:

- process must reference a schema
- all paths must resolve against that schema

---

## 21. Determinism Requirement

Schema must not introduce:

- dynamic typing
- runtime-dependent resolution
- implicit conversions

---

## 22. Error Conditions

Compiler MUST error on:

- unknown field
- invalid namespace
- type mismatch
- invalid enum value
- invalid identifier
- illegal write target

---

## 23. Minimal Schema Example

{
  "domain": "Example",
  "version": "1.0.0",
  "fields": [
    { "name": "amount", "type": "Number" },
    { "name": "status", "type": "Enum", "enum": ["A", "B"] }
  ]
}

---

## 24. Internal Representation (Conceptual)

Schema should compile to:

NamespaceCategory → FieldName → VpeType

Example:

rec → amount → Number  
rec → status → Enum  

---

## 25. Summary

The schema is:

- the type system of VPE
- the validator of all field access
- the contract between host and engine

Strict enforcement ensures:

- deterministic execution
- compile-time safety
- predictable evolution