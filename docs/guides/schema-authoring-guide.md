# VPE Schema Authoring Guide
Version: Canonical v1

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

```json
{
  "domain": "OrderManagement",
  "version": "1.0.0",
  "fields": [
    {
      "name": "order_total",
      "type": "Number",
      "description": "Total order amount"
    }
  ]
}
```
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

```json
{
  "name": "status",
  "type": "String",
  "enum": ["Pending", "Approved", "Rejected"]
}
```
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

---

## 6. Field Naming Rules

All fields must:

- use snake_case
- be alphanumeric with underscores
- not start with a digit

Valid:
- `order_total`
- `customer_id`

Invalid:
- `order-total`
- `123amount`

---

## 7. Schema and Guards

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

## 8. Schema and Manifest

The compiler derives required fields from guards.

Example:

Guard:
{
  "path": "rec.order_total"
}

Manifest includes:
- `rec.order_total`

This allows:
- minimal data fetching
- predictable execution

---

## 9. Schema Evolution

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

## 10. Versioning Strategy

Each schema must include:

- domain
- version

Example:
- `OrderManagement v1.0.0`
- `OrderManagement v2.0.0`

---

## 11. Migration Considerations

When schema changes:

- use migration rules to adapt data
- avoid breaking existing laws
- ensure compatibility with guards

Example:

Move:
old_total → rec.order_total

---

## 12. Validation Rules

The compiler must validate:

1. Field exists
2. Type matches usage
3. Namespace is valid
4. Writes only occur in `rec.*`

---

## 13. Best Practices

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

## 14. Anti-Patterns

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

## 15. Example Schema

{
  "domain": "LoanApproval",
  "version": "1.0.0",
  "fields": [
    {
      "name": "amount",
      "type": "Number"
    },
    {
      "name": "applicant_id",
      "type": "String"
    },
    {
      "name": "is_high_risk",
      "type": "Boolean"
    },
    {
      "name": "submitted_at",
      "type": "DateTime"
    }
  ]
}

---

## 16. Mental Model

Think of the schema as:

- the **data contract**
- the **compiler’s type system**
- the **guard dependency map**

Without a schema:
- validation breaks
- determinism weakens
- evolution becomes dangerous

---

## 17. Summary

A good schema is:

- explicit
- stable
- minimal
- aligned with logic

The schema is not just data definition.

It is a **foundation for correctness and determinism**.