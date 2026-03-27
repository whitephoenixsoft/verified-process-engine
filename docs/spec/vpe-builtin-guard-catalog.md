# VPE Built-in Guard Catalog
Version: Canonical v1

## Purpose
This document defines the initial set of built-in guards for VPE.

These guards:
- establish the core guard language
- define expected validation patterns
- drive manifest generation behavior
- serve as reference implementations for custom guards

This list is intentionally limited to keep the system:
- explicit
- composable
- easy to validate
- easy to extend

---

## 1. Comparison (Field vs Literal)

### Equals
Compares a field to a literal value.

Fields:
- path
- value

Example:
type: Equals  
path: rec.amount  
value: 100

---

### NotEquals
Opposite of Equals.

---

### GreaterThan
Numeric comparison.

---

### GreaterThanOrEqual

---

### LessThan

---

### LessThanOrEqual

---

## 2. Comparison (Field vs Field)

These guards compare two paths.

### FieldsEqual

Fields:
- left_path
- right_path

Example:
type: FieldsEqual  
left_path: rec.customer_id  
right_path: ext.request_customer_id

---

### FieldsNotEqual

---

### FieldsGreaterThan

---

### FieldsGreaterThanOrEqual

---

### FieldsLessThan

---

### FieldsLessThanOrEqual

---

## 3. Presence / Shape

### Exists

Fields:
- path

Meaning:
- field exists and is not null

---

### MissingField

Fields:
- path

Meaning:
- field is missing or null

---

## 4. Set Membership

### InSet

Fields:
- path
- values (array)

---

### NotInSet

---

## 5. Temporal / History

### OccurredWithin

Fields:
- event_type
- window_seconds

Meaning:
- event occurred within time window relative to sys.now

---

### OccurredAtLeast

Fields:
- event_type
- count

---

### OccurredExactly

---

### TimeElapsed

Fields:
- since_event
- seconds

---

## 6. Flow Guards

### Default

Meaning:
- always true
- used as fallback transition

---

## 7. Design Rules

1. Guards must be deterministic
2. Guards must declare all data requirements
3. Guards must not perform I/O
4. Guards must not access external state
5. Guards must be stateless after construction

---

## 8. Guard Design Principles

- Literal comparisons and field comparisons are separate guards
- Guard parameter shapes must be explicit
- No overloaded parameter semantics
- All dependencies must be discoverable at compile time

---

## 9. Future Extensions (Not Included in v1)

- Regex / pattern matching
- Expression language
- Boolean combinators (AND/OR as guards)
- Cross-process guards

These may be added in future versions after core stability.