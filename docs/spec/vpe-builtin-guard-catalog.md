# VPE Built-in Guard Catalog

Version: Draft v2  
Status: Draft  
Scope: Built-in guard definitions, parameter shapes, validation rules, manifest requirements, runtime behavior, and failure behavior  
Audience: Law authors, compiler implementers, runtime implementers, API designers, future contributors  
Related Documents: VPE Guard Architecture, VPE Law Reference, VPE Spec, VPE Invariants, VPE Core API, VPE App API

---

## Purpose

This document defines the initial set of built-in guards for VPE.

Built-in guards:

- establish the core guard language
- define standard validation behavior
- drive manifest generation
- serve as reference implementations for custom guards
- keep law authoring explicit and deterministic

This document is the guard catalog.

The Guard Architecture document defines the registry, lifecycle, compilation, and runtime model.

This catalog defines the built-in guard functions available by default.

---

## 1. Core Guard Mental Model

A guard is a deterministic predicate over explicit supplied truth.

Guards answer:

- may this transition apply?
- may this migration rule apply?

Guards do not:

- mutate context
- transition state
- emit events
- execute effects
- fetch host data
- call external systems
- access hidden runtime state

Core rule:

Guards decide applicability.  
They do not perform the action.

---

## 2. General Guard Rules

All built-in guards must be:

- deterministic
- side-effect free
- schema-valid
- explicit about inputs
- explicit about history requirements when needed
- safe for normal execution
- safe for simulation
- safe for migration/lift

All built-in guards must declare:

- required context paths
- required history/event data
- required system values
- parameter type expectations
- runtime failure behavior

---

## 3. Guard Failure vs Guard Error

A guard failure means:

- the guard evaluated successfully
- the condition was false

A guard error means:

- the guard could not be evaluated correctly

Examples of guard errors:

- required context missing
- required history missing
- invalid runtime value shape
- type mismatch
- missing sys.now for time-based guard

Missing required truth should not be treated as false unless the specific guard explicitly defines that behavior.

---

## 4. Namespace Rules

Built-in guards may read from allowed namespaces, including:

- rec.*
- ext.*
- sys.*
- calc.*

Built-in guards must not write to any namespace.

Typical meaning:

- rec.* is persistent record/application data
- ext.* is external supplied input
- sys.* is explicit system-supplied data
- calc.* is derived read-only data

All referenced paths must be declared and validated against schema or system schema.

---

## 5. Built-in Guard Families

Initial built-in guard families:

- comparison, field vs literal
- comparison, field vs field
- presence and shape
- set membership
- temporal and history
- flow fallback

The initial set should remain small, explicit, and composable.

VPE should not introduce a broad expression language prematurely.

---

## 6. Field vs Literal Comparison Guards

Field vs literal comparison guards compare one context path against one literal value.

These guards require:

- path
- value

The compiler must validate:

- path exists
- path type is compatible with value
- guard type supports the field type
- value is serializable and deterministic

Runtime must evaluate against supplied context only.

---

## 7. Equals

### Purpose

Equals checks whether a field value equals a literal value.

### Fields

- type: Equals
- path: context path
- value: literal value

### Requirements

Requires:

- the referenced path

### Validation

Compilation must validate:

- path exists
- literal value type is compatible with path type
- equality comparison is supported for the type

### Runtime Behavior

Passes when the supplied path value equals the literal value.

Fails when the supplied path value exists but is not equal.

Errors when the required path is missing or has an invalid runtime shape.

### Common Uses

- status equals expected value
- amount equals exact threshold
- flag equals true
- type equals known code

### Common Misuse

Do not use Equals for pattern matching or fuzzy comparison.

---

## 8. NotEquals

### Purpose

NotEquals checks whether a field value does not equal a literal value.

### Fields

- type: NotEquals
- path: context path
- value: literal value

### Requirements

Requires:

- the referenced path

### Validation

Compilation must validate:

- path exists
- literal value type is compatible with path type
- inequality comparison is supported for the type

### Runtime Behavior

Passes when the supplied path value exists and does not equal the literal value.

Fails when the supplied path value equals the literal value.

Errors when the required path is missing or invalid.

### Common Uses

- status is not closed
- payment status is not captured
- flag is not true

### Common Misuse

Do not use NotEquals to mean missing.

Use MissingField when missing or null is the intended condition.

---

## 9. GreaterThan

### Purpose

GreaterThan checks whether a numeric or ordered field value is greater than a literal value.

### Fields

- type: GreaterThan
- path: context path
- value: literal value

### Requirements

Requires:

- the referenced path

### Validation

Compilation must validate:

- path exists
- path type supports ordering
- literal value type is compatible with path type
- comparison is deterministic

### Runtime Behavior

Passes when field value is greater than literal value.

Fails when field value is less than or equal to literal value.

Errors when path is missing or value cannot be compared.

### Common Uses

- amount greater than threshold
- count greater than limit
- score greater than risk threshold

### Common Misuse

Do not use GreaterThan on unordered strings unless string ordering is explicitly supported and deterministic.

---

## 10. GreaterThanOrEqual

### Purpose

GreaterThanOrEqual checks whether a field value is greater than or equal to a literal value.

### Fields

- type: GreaterThanOrEqual
- path: context path
- value: literal value

### Requirements

Requires:

- the referenced path

### Validation

Compilation must validate:

- path exists
- path type supports ordering
- literal value type is compatible with path type

### Runtime Behavior

Passes when field value is greater than or equal to literal value.

Fails when field value is less than literal value.

Errors when path is missing or invalid.

---

## 11. LessThan

### Purpose

LessThan checks whether a field value is less than a literal value.

### Fields

- type: LessThan
- path: context path
- value: literal value

### Requirements

Requires:

- the referenced path

### Validation

Compilation must validate:

- path exists
- path type supports ordering
- literal value type is compatible with path type

### Runtime Behavior

Passes when field value is less than literal value.

Fails when field value is greater than or equal to literal value.

Errors when path is missing or invalid.

---

## 12. LessThanOrEqual

### Purpose

LessThanOrEqual checks whether a field value is less than or equal to a literal value.

### Fields

- type: LessThanOrEqual
- path: context path
- value: literal value

### Requirements

Requires:

- the referenced path

### Validation

Compilation must validate:

- path exists
- path type supports ordering
- literal value type is compatible with path type

### Runtime Behavior

Passes when field value is less than or equal to literal value.

Fails when field value is greater than literal value.

Errors when path is missing or invalid.

---

## 13. Field vs Field Comparison Guards

Field vs field comparison guards compare two context paths.

These guards require:

- left_path
- right_path

The compiler must validate:

- both paths exist
- both types are compatible
- comparison is supported for those types

Runtime must evaluate using supplied context only.

---

## 14. FieldsEqual

### Purpose

FieldsEqual checks whether two field values are equal.

### Fields

- type: FieldsEqual
- left_path: context path
- right_path: context path

### Requirements

Requires:

- left_path
- right_path

### Validation

Compilation must validate:

- both paths exist
- both paths are type-compatible
- equality comparison is supported

### Runtime Behavior

Passes when the two values are equal.

Fails when both values exist but are not equal.

Errors when either path is missing or invalid.

### Common Uses

- request customer matches record customer
- actor id matches owner id
- external id matches stored id

---

## 15. FieldsNotEqual

### Purpose

FieldsNotEqual checks whether two field values are not equal.

### Fields

- type: FieldsNotEqual
- left_path: context path
- right_path: context path

### Requirements

Requires:

- left_path
- right_path

### Validation

Compilation must validate:

- both paths exist
- both paths are type-compatible
- inequality comparison is supported

### Runtime Behavior

Passes when both values exist and are not equal.

Fails when the values are equal.

Errors when either path is missing or invalid.

### Common Misuse

Do not use FieldsNotEqual to detect missing values.

Use Exists or MissingField explicitly.

---

## 16. FieldsGreaterThan

### Purpose

FieldsGreaterThan checks whether the left field is greater than the right field.

### Fields

- type: FieldsGreaterThan
- left_path: context path
- right_path: context path

### Requirements

Requires:

- left_path
- right_path

### Validation

Compilation must validate:

- both paths exist
- both types are compatible
- both types support deterministic ordering

### Runtime Behavior

Passes when left value is greater than right value.

Fails otherwise.

Errors when either path is missing or invalid.

---

## 17. FieldsGreaterThanOrEqual

### Purpose

FieldsGreaterThanOrEqual checks whether the left field is greater than or equal to the right field.

### Fields

- type: FieldsGreaterThanOrEqual
- left_path: context path
- right_path: context path

### Requirements

Requires:

- left_path
- right_path

### Validation

Compilation must validate:

- both paths exist
- both types are compatible
- both types support deterministic ordering

### Runtime Behavior

Passes when left value is greater than or equal to right value.

Fails otherwise.

Errors when either path is missing or invalid.

---

## 18. FieldsLessThan

### Purpose

FieldsLessThan checks whether the left field is less than the right field.

### Fields

- type: FieldsLessThan
- left_path: context path
- right_path: context path

### Requirements

Requires:

- left_path
- right_path

### Validation

Compilation must validate:

- both paths exist
- both types are compatible
- both types support deterministic ordering

### Runtime Behavior

Passes when left value is less than right value.

Fails otherwise.

Errors when either path is missing or invalid.

---

## 19. FieldsLessThanOrEqual

### Purpose

FieldsLessThanOrEqual checks whether the left field is less than or equal to the right field.

### Fields

- type: FieldsLessThanOrEqual
- left_path: context path
- right_path: context path

### Requirements

Requires:

- left_path
- right_path

### Validation

Compilation must validate:

- both paths exist
- both types are compatible
- both types support deterministic ordering

### Runtime Behavior

Passes when left value is less than or equal to right value.

Fails otherwise.

Errors when either path is missing or invalid.

---

## 20. Presence And Shape Guards

Presence and shape guards check whether data exists or is absent.

These guards are important for migration and optional fields.

Presence guards must define exactly how they treat null and missing.

---

## 21. Exists

### Purpose

Exists checks whether a path is present and not null.

### Fields

- type: Exists
- path: context path

### Requirements

Requires:

- the referenced path shape or presence check capability

### Validation

Compilation must validate:

- path is syntactically valid
- path is allowed by schema or optional schema rules
- namespace is readable

### Runtime Behavior

Passes when the path exists and value is not null.

Fails when the path is missing or null.

Errors when path access itself is invalid due to malformed context shape.

### Common Uses

- customer email exists
- payment status exists
- migration source field exists

### Common Misuse

Do not use Exists as a type validator unless type validation is explicitly part of the schema/runtime model.

---

## 22. MissingField

### Purpose

MissingField checks whether a path is missing or null.

### Fields

- type: MissingField
- path: context path

### Requirements

Requires:

- the referenced path shape or presence check capability

### Validation

Compilation must validate:

- path is syntactically valid
- namespace is readable
- path is valid under schema or allowed as optional/migrating legacy path

### Runtime Behavior

Passes when the path is missing or null.

Fails when the path exists and is not null.

Errors when path access is invalid due to malformed context shape.

### Common Uses

- old data is missing a newly required field
- migration rule detects legacy records
- optional value has not been supplied

### Common Misuse

Do not use MissingField to mean false.

False and missing are different meanings.

---

## 23. Set Membership Guards

Set membership guards check whether a field value is or is not contained in an explicit set.

These guards require:

- path
- values

The compiler must validate:

- path exists
- values array is non-empty unless explicitly allowed
- every value is compatible with path type
- membership comparison is deterministic

---

## 24. InSet

### Purpose

InSet checks whether a field value is contained in an explicit set of values.

### Fields

- type: InSet
- path: context path
- values: array of literal values

### Requirements

Requires:

- the referenced path

### Validation

Compilation must validate:

- path exists
- values is an array
- all values are compatible with path type
- membership comparison is supported for the type
- duplicate values are either normalized or warned

### Runtime Behavior

Passes when field value is contained in values.

Fails when field value exists but is not contained in values.

Errors when path is missing or invalid.

### Common Uses

- status is one of allowed statuses
- region is supported
- priority is in allowed set

---

## 25. NotInSet

### Purpose

NotInSet checks whether a field value is not contained in an explicit set of values.

### Fields

- type: NotInSet
- path: context path
- values: array of literal values

### Requirements

Requires:

- the referenced path

### Validation

Compilation must validate:

- path exists
- values is an array
- all values are compatible with path type
- membership comparison is supported for the type

### Runtime Behavior

Passes when field value exists and is not contained in values.

Fails when field value is contained in values.

Errors when path is missing or invalid.

### Common Misuse

Do not use NotInSet to handle missing fields.

Use MissingField explicitly if missing is valid.

---

## 26. Temporal And History Guards

Temporal and history guards evaluate supplied chronicle/history.

They must not fetch history themselves.

They must not call the system clock directly.

If time is required, it must be supplied explicitly through sys.now or event timestamps.

History guards are critical for replay, simulation, and deterministic runtime behavior.

---

## 27. OccurredWithin

### Purpose

OccurredWithin checks whether a specific event or action occurred within a time window relative to explicit time.

### Fields

Supported naming should be normalized. The preferred fields are:

- type: OccurredWithin
- target_action or event_type
- window_seconds

Optional or future fields may include:

- since
- anchor
- include_current_event

### Requirements

Requires:

- relevant history for the target action/event
- event timestamps
- explicit sys.now or equivalent evaluation time

### Validation

Compilation must validate:

- target action/event identifier is valid where possible
- window_seconds is positive
- required history can be represented in manifest
- explicit time requirement is declared

### Runtime Behavior

Passes when at least one matching event/action occurred within the time window.

Fails when no matching event/action occurred within the time window.

Errors when required history or explicit time is missing.

### Common Uses

- fraud check occurred within 24 hours
- approval occurred within policy window
- payment attempt happened recently

### Common Misuse

Do not use OccurredWithin without supplying deterministic time.

---

## 28. OccurredAtLeast

### Purpose

OccurredAtLeast checks whether an event/action occurred at least a given number of times in supplied history.

### Fields

- type: OccurredAtLeast
- target_action or event_type
- count

Optional future fields:

- window_seconds
- since_anchor

### Requirements

Requires:

- relevant history for target action/event
- enough history to count occurrences

### Validation

Compilation must validate:

- count is a non-negative integer
- target action/event identifier is valid where possible
- required history is declared

### Runtime Behavior

Passes when matching event/action count is greater than or equal to count.

Fails when count is lower.

Errors when required history is missing or incomplete.

### Common Uses

- at least one payment attempt happened
- fraud check has occurred
- retries have reached threshold

### Common Misuse

Do not use OccurredAtLeast when ordering or timing matters.

Use a temporal guard when timing is part of the rule.

---

## 29. OccurredExactly

### Purpose

OccurredExactly checks whether an event/action occurred exactly a given number of times in supplied history.

### Fields

- type: OccurredExactly
- target_action or event_type
- count

Optional future fields:

- window_seconds
- since_anchor

### Requirements

Requires:

- relevant history for target action/event
- complete enough history to count occurrences

### Validation

Compilation must validate:

- count is a non-negative integer
- target action/event identifier is valid where possible
- required history is declared
- manifest can describe enough history completeness for exact count

### Runtime Behavior

Passes when matching count equals expected count.

Fails when count differs.

Errors when required history is missing or incomplete.

### Common Uses

- exactly one approval exists
- exactly zero retry attempts exist
- exactly one migration event exists

### Common Misuse

Use carefully.

Exact count requires confidence that supplied history is complete enough.

---

## 30. TimeElapsed

### Purpose

TimeElapsed checks whether at least a given amount of time has elapsed since an event, action, transition, or anchor.

### Fields

Preferred fields:

- type: TimeElapsed
- seconds

Optional selector fields:

- since_event
- since_action
- since_anchor
- since_state_entry

The catalog should normalize this shape before implementation.

### Requirements

Requires:

- relevant timestamp source
- explicit sys.now or equivalent evaluation time
- history or anchor data needed by the selected since field

### Validation

Compilation must validate:

- seconds is positive
- one deterministic since source is specified or a default source is defined
- required history/anchor/time is declared

### Runtime Behavior

Passes when elapsed time is greater than or equal to seconds.

Fails when elapsed time is less than seconds.

Errors when the timestamp source or sys.now is missing.

### Common Uses

- payment timeout after 300 seconds
- escalation after 1 hour
- cooling period has elapsed

### Common Misuse

Do not let TimeElapsed call the system clock directly.

Time must be supplied explicitly for deterministic execution and replay.

---

## 31. Flow Guards

Flow guards help express fallback behavior.

They must not introduce ambiguity.

---

## 32. Default

### Purpose

Default is an always-true guard used for fallback transitions.

### Fields

- type: Default

### Requirements

Requires:

- no context
- no history
- no system values

### Validation

Compilation must validate:

- no unsupported parameters are supplied
- transition priority/order makes fallback deterministic
- Default is not used in a way that creates ambiguity

### Runtime Behavior

Always passes.

### Common Uses

- fallback transition
- else branch
- final catch-all path

### Common Misuse

Do not use Default with the same priority as more specific guarded transitions if that creates ambiguity.

Default should usually have lower priority than specific transitions.

---

## 33. Guard Catalog Summary

Built-in guard purposes:

- Equals: field equals literal
- NotEquals: field does not equal literal
- GreaterThan: field greater than literal
- GreaterThanOrEqual: field greater than or equal to literal
- LessThan: field less than literal
- LessThanOrEqual: field less than or equal to literal
- FieldsEqual: field equals field
- FieldsNotEqual: field does not equal field
- FieldsGreaterThan: left field greater than right field
- FieldsGreaterThanOrEqual: left field greater than or equal to right field
- FieldsLessThan: left field less than right field
- FieldsLessThanOrEqual: left field less than or equal to right field
- Exists: field exists and is not null
- MissingField: field is missing or null
- InSet: field value is in explicit set
- NotInSet: field value is not in explicit set
- OccurredWithin: event/action occurred within time window
- OccurredAtLeast: event/action occurred at least N times
- OccurredExactly: event/action occurred exactly N times
- TimeElapsed: required time has elapsed
- Default: always true fallback guard

---

## 34. Anti-Patterns

Avoid:

- using missing data as false by default
- using NotEquals to mean MissingField
- using Default to hide ambiguous transition design
- using temporal guards without explicit time
- using exact history guards without complete history
- using field comparison guards on incompatible types
- relying on host data not declared in the manifest
- encoding large expression logic inside too many guards
- adding custom guards before built-ins are insufficient

---

## 35. Future Guard Candidates

Potential future guards may include:

- RegexMatch
- StartsWith
- EndsWith
- Contains
- IsType
- IsValidEmail
- IsValidUuid
- Between
- DateBefore
- DateAfter
- EventOccurredBefore
- EventOccurredAfter
- Boolean combinators

These should not be added until the core guard registry, manifest model, and built-in catalog are stable.

---

## 36. Final Summary

Built-in guards provide the standard predicates needed for deterministic process decisioning.

They are explicit, schema-validated, manifest-aware, and side-effect free.

They do not fetch data.

They do not mutate state.

They do not execute effects.

They determine whether a transition or migration rule may apply.

The governing rule is:

Guards decide applicability.  
VPE transitions only after guards pass.  
The host supplies the truth guards evaluate.