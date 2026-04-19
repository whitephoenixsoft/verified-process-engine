# VPE Schema Authoring Guide
Version: Canonical v2

## 1. Purpose

The Domain Schema defines the **structure and types of data** used by a VPE process.

It serves as:
- the source of truth for all `rec.*`, `ext.*`, and `calc.*` fields
- the foundation for type safety
- the contract between the host system and the VPE engine

A well-defined schema ensures:
- compile-time validation
- deterministic execution
- safe evolution over time

---

## 2. Core Principles

### 1. Explicitness
All fields must be explicitly defined.

No implicit or dynamic fields are allowed.

---

### 2. Type Safety
Every field has a fixed type.

Types cannot change within the same schema version.

---

### 3. Namespaced Access
All fields are accessed through namespaces:

- `rec.*` → persisted record data
- `ext.*` → external inputs (read-only)
- `calc.*` → derived or computed values
- `sys.*` → system-provided values (not part of schema)

---

### 4. Compile-Time Enforcement
All field references in laws must:
- exist in the schema
- match expected types

---

### 5. Stability
Schemas should evolve **additively** when possible.

---

## 3. Schema Structure

A schema is defined as:

{
  "domain": "OrderManagement",
  "version": "1.0.0",
  "namespaces": {
    "rec": [
      {
        "name": "order_total",
        "type": "Number",
        "description": "Total order amount"
      }
    ],
    "ext": [],
    "calc": []
  }
}

Notes:
- All three namespaces must be present (may be empty)
- `sys` is reserved and must not be defined

---

## 4. Field Types

Supported types:

- `String`
- `Number`
- `Boolean`
- `DateTime` (Unix timestamp)
- `Duration` (seconds)
- `Enum` (restricted string values)

Example:

{
  "name": "status",
  "type": "Enum",
  "enum_values": ["Pending", "Approved", "Rejected"]
}

---

## 5. Namespaces

### rec.* (Record Data)

Represents persisted state.

Examples:
- `rec.order_total`
- `rec.customer_id`

Rules:
- mutable via transitions and migrations
- must be defined in schema

---

### ext.* (External Data)

Provided by the host at runtime.

Examples:
- `ext.credit_score`
- `ext.user_tier`

Rules:
- read-only
- must be defined in schema

---

### calc.* (Derived Data)

Computed outside the engine or pre-processed.

Examples:
- `calc.risk_score`
- `calc.discount_amount`

Rules:
- read-only
- must be defined in schema

---

### sys.* (System Data)

Provided by the engine or host.

Examples:
- `sys.now`
- `sys.trace_id`

Rules:
- not defined in schema
- always read-only
- validated against built-in system schema

---

## 6. Naming Conventions

### Field Names

- must use snake_case
- must be descriptive and domain-aligned
- must follow identifier rules

✔ Good:
- order_total
- customer_id
- payment_status

❌ Avoid:
- order total
- order-total
- amount1value

---

### Namespace Usage

All references must include namespace:

✔ Valid:
- rec.order_total
- ext.user_id

❌ Invalid:
- order_total
- user_id

---

### Enum Values

- must be strings
- must be stable across versions
- should follow a consistent style:
  - PascalCase OR UPPER_CASE

✔ Examples:
- Approved
- Pending
- REJECTED

---

## 7. Identifier Rules

All identifiers must:

- use dot notation: namespace.field
- contain only alphanumeric characters and underscores
- not start with a digit
- be case-sensitive

---

## 8. Schema and Guards

Guards rely on schema definitions.

Example:

{
  "type": "GreaterThan",
  "path": "rec.order_total",
  "value": 1000
}

Compiler ensures:
- `rec.order_total` exists
- type is Number
- value is compatible

---

## 9. Schema and Manifest

The compiler derives required fields from guards.

Example:

Guard:
{
  "path": "rec.order_total"
}

Manifest includes:
- `rec.order_total`

This enables:
- minimal data fetching
- predictable execution

---

## 10. Schema Evolution

### Allowed Changes

✔ Add new fields  
✔ Add enum values  
✔ Add new optional data  

---

### Restricted Changes

❌ Changing field type  
❌ Removing fields in use  
❌ Renaming fields without migration  

---

## 11. Versioning Strategy

Each schema must include:

- domain
- version

Example:
- `OrderManagement v1.0.0`
- `OrderManagement v2.0.0`

---

## 12. Schema and Law Binding

A Law must explicitly declare:

- `schema_version`

The compiler enforces:

- schema.domain == law.domain
- schema.version == law.schema_version

No implicit matching is allowed.

---

## 13. Migration Considerations

When schema changes:

- use migration rules to adapt data
- avoid breaking existing laws
- ensure compatibility with guards

Example:

Move:
old_total → rec.order_total

---

## 14. Validation Rules

The compiler must validate:

1. Field exists
2. Type matches usage
3. Namespace is valid
4. Writes only occur in `rec.*`
5. Enum fields define valid values
6. Schema does not define `sys.*`

---

## 15. Best Practices

### Keep Fields Focused
Each field should represent one concept.

---

### Prefer Flat Structure
Avoid deep nesting.

✔ Good:
rec.order_total

❌ Avoid:
rec.order.total.amount

---

### Use Enums for Controlled Values
Avoid free-form strings when possible.

---

### Align with Guards
Design fields based on actual decision logic.

---

### Avoid Overloading Fields
One field = one meaning.

---

## 16. Anti-Patterns

### Hidden Fields
Using fields not defined in schema.

---

### Type Drift
Changing field type across versions.

---

### Over-Nesting
Deep object structures that complicate access.

---

### Schema Bloat
Too many unused fields.

---

## 17. Example Schema

{
  "domain": "LoanApproval",
  "version": "1.0.0",
  "namespaces": {
    "rec": [
      { "name": "amount", "type": "Number" },
      { "name": "applicant_id", "type": "String" },
      { "name": "is_high_risk", "type": "Boolean" },
      { "name": "submitted_at", "type": "DateTime" }
    ],
    "ext": [
      { "name": "credit_score", "type": "Number" }
    ],
    "calc": [
      { "name": "risk_score", "type": "Number" }
    ]
  }
}

---

## 18. Mental Model

Think of the schema as:

- the **data contract**
- the **compiler’s type system**
- the **guard dependency map**

Without a schema:
- validation breaks
- determinism weakens
- evolution becomes dangerous

---

## 19. Summary

A good schema is:

- explicit
- stable
- minimal
- aligned with logic

The schema is not just data definition.

It is a **foundation for correctness and determinism**.