# VPE Guard Architecture

Status: Draft  
Scope: Guard registry, guard compilation, guard validation, manifest derivation, runtime guard evaluation, and custom guard extension model  
Audience: Project architect, compiler implementers, runtime implementers, API designers, future contributors  
Related Documents: VPE Spec, VPE Invariants, VPE Law Reference, VPE Built-in Guard Catalog, VPE Core API, VPE App API

---

## 1. Purpose

This document defines the architecture for guards in VPE.

It exists to:

- define how guards participate in compilation
- define how guard requirements produce manifest requirements
- define how built-in and custom guards are registered
- prevent guard-specific logic from becoming hardcoded throughout the compiler
- ensure guards remain deterministic, explicit, and host-data-safe
- provide a validation anchor for the Core API, App API, Spec, and Invariants

This document is not the guard catalog.

The guard catalog defines specific guard functions.

This document defines the guard architecture that all guard functions must follow.

---

## 2. Core Position

Guards decide whether a transition or migration rule is allowed to apply.

A guard is not business code.

A guard is a deterministic predicate that evaluates explicit supplied truth.

Guards must never:

- perform I/O
- access hidden host state
- mutate context
- execute effects
- read undeclared data
- depend on nondeterministic runtime behavior

The compiler and runtime must treat guards as deterministic semantic components.

---

## 3. Relationship To VPE Execution

During normal execution, VPE evaluates:

Given:

- current state
- action
- context
- history
- anchor
- version truth

VPE then:

- finds candidate transitions
- evaluates guards
- selects the first valid transition according to deterministic rules
- emits events and effects as intent

Guards determine whether a candidate transition is valid.

They do not perform the transition.

---

## 4. Relationship To Migration

Migration rules may also contain guards.

In migration, guards determine whether a migration rule is allowed to lift old-version truth into target-version truth.

The same guard architecture applies to both:

- transition guards
- migration rule guards

This means migration guards must also:

- validate at compile time
- declare all context and history requirements
- evaluate deterministically
- participate in manifest derivation
- fail safely when required truth is missing

---

## 5. Guard Registry

### 5.1 Purpose

The GuardRegistry is the authoritative registry of available guard definitions.

A law may only reference guard types that exist in the registry.

The compiler should not contain special hardcoded logic for every guard type.

Instead, the compiler should:

1. Look up the guard type in GuardRegistry.
2. Ask the guard definition to validate its parameters.
3. Ask the guard definition to declare its requirements.
4. Build the compiled/runtime guard representation.

### 5.2 Built-in guards

Built-in guards are registered in the same registry as custom guards.

There should be no semantic privilege for built-in guards beyond being available by default.

### 5.3 Custom guards

Custom guards should be able to participate in the same pipeline as built-in guards.

A custom guard must be able to:

- validate parameters
- declare requirements
- build a runtime evaluator
- provide diagnostics
- respect determinism constraints

### 5.4 Registry invariant

The registry is the extension boundary for guard behavior.

Adding a new guard should not require modifying core compiler control flow.

---

## 6. Guard Definition

A guard definition is the compile-time description of a guard.

It should define:

- guard name
- expected parameter shape
- validation rules
- context requirements
- history requirements
- runtime construction rules
- diagnostic behavior

Conceptually, every guard definition must answer:

- Is this guard usage valid?
- What data does this guard require?
- What runtime evaluator should be built?
- What errors or warnings should be emitted?

---

## 7. Guard Runtime Evaluator

A runtime guard evaluator is the executable guard instance used during evaluation.

It should be created only after compile-time validation succeeds.

A runtime guard evaluator receives explicit supplied truth, such as:

- context
- history
- system values
- anchor-derived information

It returns:

- pass
- fail
- or deterministic error if required truth is missing or invalid

The runtime evaluator must not discover new dependencies during execution.

All dependencies must already be known from compile-time validation.

---

## 8. Guard Compilation Flow

For each guard in a law:

1. Read the guard type.
2. Look up the guard definition in GuardRegistry.
3. Validate the guard parameters.
4. Validate referenced paths against schema.
5. Validate referenced history/event requirements.
6. Collect context requirements.
7. Collect history requirements.
8. Add requirements to the relevant manifest.
9. Build the runtime guard evaluator.
10. Store the compiled guard in the compiled process artifact.

If any required validation fails, compilation fails.

No partial guard compilation is allowed.

---

## 9. Guard Requirements

Every guard must declare all truth it requires.

Guard requirements may include:

- context paths
- system paths
- history/event requirements
- temporal requirements
- anchor requirements
- version/lift-related requirements when used in migration

Examples:

- `Equals(rec.amount, 100)` requires `rec.amount`
- `FieldsEqual(rec.customer_id, ext.request_customer_id)` requires both paths
- `OccurredWithin(FraudCheck, 86400)` requires relevant history and explicit time
- `TimeElapsed(...)` requires appropriate historical timing information

A guard must not read data it did not declare.

---

## 10. Manifest Derivation

State manifests are derived from guard requirements.

For each state, the compiler must collect requirements from all guards that may be evaluated from that state.

This includes:

- transition guards
- automatic transition guards
- history-aware guards
- migration guards when relevant to lift requirements

The manifest must be complete.

If a guard requirement is missing from the manifest, that is a compiler error.

If a manifest includes unused requirements, that may be a warning.

---

## 11. Schema Validation

Guards that reference paths must validate those paths against the schema.

Validation must confirm:

- path exists
- namespace is allowed
- value type is compatible with the guard
- literal values match expected type
- field-to-field comparisons are type-compatible
- writable/read-only namespace rules are respected where relevant

For example:

- numeric comparison guards require numeric-compatible fields
- set membership guards require compatible value sets
- presence guards may apply broadly
- history guards must validate referenced event/action identifiers where applicable

---

## 12. Determinism Requirements

All guards must be deterministic.

A guard must not depend on:

- random values
- current system time unless supplied explicitly as `sys.now`
- external service calls
- database reads
- network calls
- global mutable state
- hidden host state
- unordered collection behavior

If time is required, it must come from explicit supplied system context.

---

## 13. Namespaces

Guards may read from allowed namespaces according to VPE path rules.

Common namespaces include:

- `rec.*`
- `ext.*`
- `sys.*`
- `calc.*`

Guard behavior must respect namespace meaning.

Examples:

- `rec.*` is process/application record data
- `ext.*` is external supplied input
- `sys.*` is system-supplied explicit values
- `calc.*` is derived read-only data

Guards may read from allowed namespaces but must not mutate any namespace.

---

## 14. History-Aware Guards

History-aware guards are guards that depend on event or chronicle information.

Examples include:

- event occurred within a time window
- event occurred at least N times
- elapsed time since an event
- action occurred before another action

History-aware guards must declare:

- event/action identifiers required
- time window or count requirements
- whether ordering matters
- whether anchor information is required
- whether explicit `sys.now` is required

History-aware guards must evaluate only against supplied history.

They must not fetch history themselves.

---

## 15. Temporal Guards

Temporal guards must use explicit time.

They must not call the system clock directly.

Time should be supplied through system context, such as `sys.now`.

This ensures:

- replayability
- deterministic simulation
- consistent testing
- reproducible decisions

Temporal guard requirements must be reflected in manifests.

---

## 16. Default Guard

The Default guard is a special built-in guard that always passes.

It is used for fallback transitions.

Rules:

- Default must be deterministic.
- Default requires no context.
- Default requires no history.
- Default should usually have lower priority than specific guarded transitions.
- Default must not be used to hide ambiguity.

The transition ordering and priority system must still ensure deterministic selection.

---

## 17. Ambiguity

Guard evaluation must not introduce ambiguity.

Ambiguity may occur when:

- two transitions for the same action have the same priority
- multiple migration rules match equally
- ordering is not deterministic
- guard definitions are overloaded ambiguously
- guard parameters allow multiple interpretations

VPE must reject or deterministically fail ambiguous guard situations.

It must not silently choose an arbitrary path.

---

## 18. Runtime Evaluation Rules

At runtime, guard evaluation should follow these rules:

1. Candidate transitions are selected by state and action.
2. Candidates are ordered deterministically.
3. Guards are evaluated in deterministic order.
4. Guards within a transition are treated as AND logic unless otherwise specified.
5. The first candidate whose guards all pass is selected.
6. If no candidate passes, execution fails with a deterministic no-transition result or error.
7. Guard failures must not mutate state.
8. Guard failures must not execute effects.

The runtime may short-circuit guard evaluation within a transition, but short-circuiting must not change observable semantics.

---

## 19. Guard Errors vs Guard Failure

The architecture should distinguish guard failure from guard error.

### Guard failure

A guard failure means:

- the guard evaluated successfully
- the condition was false

Example:

- `rec.amount > 100` is false

### Guard error

A guard error means:

- the guard could not be evaluated correctly

Examples:

- required context missing
- required history missing
- type mismatch
- invalid runtime value shape

Guard errors should be deterministic and explainable.

Missing required truth should not be treated as a normal false condition unless the guard is explicitly designed that way.

---

## 20. Compile-Time Errors

Compilation must fail when:

- guard type is unknown
- guard parameters are invalid
- referenced paths do not exist
- parameter types do not match schema
- guard requirements cannot be derived
- guard definition violates registry constraints
- guard use creates invalid manifest requirements
- guard use violates determinism constraints
- guard ambiguity cannot be resolved

No invalid guard should reach runtime.

---

## 21. Runtime Errors

Runtime guard errors may occur when:

- required manifest data was not supplied by the host
- supplied context violates expected runtime shape
- supplied history is incomplete or inconsistent
- supplied time is missing for temporal guards
- anchor expectations are not met

These errors are not compiler failures.

They represent host-supplied truth problems.

The runtime should report them clearly.

---

## 22. Guard Diagnostics

Guard validation should produce diagnostics useful to:

- law authors
- compiler users
- CLI users
- API integrators

Diagnostics should identify:

- guard type
- state or migration rule where used
- transition or rule location
- invalid parameter
- missing path
- type mismatch
- required data
- suggested correction when safe

Diagnostics should be deterministic and stable enough for CLI output and tests.

---

## 23. Guard Catalog Relationship

The built-in guard catalog documents the initial built-in guards.

The catalog should define for each guard:

- name
- purpose
- parameters
- required context
- required history
- type constraints
- examples
- failure behavior
- common misuse

The architecture document defines how those catalog entries plug into VPE.

---

## 24. Built-In Guard Families

The initial built-in guard families include:

- literal comparisons
- field-to-field comparisons
- presence and shape checks
- set membership checks
- temporal/history checks
- fallback/default flow guard

These should remain small, explicit, and composable.

Complex expression languages should not be introduced prematurely.

---

## 25. Custom Guard Requirements

A custom guard must:

- have a stable name
- declare parameter shape
- validate parameters
- validate schema references
- declare all requirements
- build a deterministic runtime evaluator
- produce diagnostics
- avoid hidden state
- avoid I/O
- be stateless after construction

A custom guard should behave exactly like a built-in guard once registered.

---

## 26. Guard Registry Invariants

The following invariants must hold:

- all guard types must resolve through the registry
- built-in and custom guards follow the same pipeline
- compiler does not hardcode guard-specific behavior long-term
- guard requirements are complete
- manifests reflect guard requirements
- guards are deterministic
- guards do not mutate state
- guards do not execute effects
- guards do not access undeclared data
- unknown guards fail compilation
- ambiguous guard behavior fails deterministically

---

## 27. Relationship To Core API

The Core API should expose or define the canonical guard-related concepts needed by compiler and runtime.

This may include:

- GuardRegistry
- GuardDefinition
- GuardValidation
- GuardRequirement
- compiled guard representation
- runtime guard evaluator
- guard diagnostics

The Core API owns the exact semantics.

---

## 28. Relationship To App API

The App API should not expose guard internals as its normal surface.

However, App API concepts such as RequiredData are downstream of guard requirements.

Therefore, App API must remain consistent with guard-derived manifests.

Application developers should see:

- required data
- deterministic failure when required data is missing
- clear errors when host truth is incomplete

They should not need to understand guard compilation internals for normal use.

---

## 29. Relationship To CLI

The CLI should expose guard diagnostics clearly.

CLI validation and compilation should help users understand:

- unknown guard types
- invalid guard parameters
- missing schema paths
- type mismatches
- manifest requirements
- history requirements

The CLI may present guard-derived manifests, but it must not redefine guard semantics.

---

## 30. Relationship To Simulation

Simulation must use the same compiled guard semantics as normal runtime execution.

Simulation must not use a separate guard evaluator.

This ensures simulation answers remain faithful to runtime behavior.

If simulation changes guard behavior, simulation becomes untrustworthy.

---

## 31. Relationship To Migration

Migration guards must use the same architecture as transition guards.

A migration rule may only apply if its guards pass.

If migration guard requirements are missing, the lift cannot lawfully proceed.

If no migration rule applies, the lift result may be incompatible.

---

## 32. Future Considerations

Future guard architecture may include:

- richer static typing for guard parameters
- code generation for guard definitions
- macro helpers
- WASM or plugin-based guards
- optional expression language
- boolean combinators
- cross-process guards

These should not be introduced until the core registry and manifest model is stable.

---

## 33. Anti-Patterns

Avoid:

- hardcoding each guard in compiler control flow
- allowing guards to fetch their own data
- allowing runtime-only dependency discovery
- treating missing data as false by default
- using arbitrary inline scripts
- adding large expression languages too early
- letting custom guards bypass manifest generation
- letting built-in guards behave differently from custom guards
- allowing nondeterministic guard behavior

---

## 34. Final Summary

Guards are deterministic predicates over explicit supplied truth.

They decide whether a transition or migration rule may apply.

The GuardRegistry is the extension boundary.

The compiler should orchestrate guard validation and requirement collection, not hardcode guard behavior.

The runtime should evaluate compiled guards against supplied context and history only.

The manifest system depends on complete guard requirements.

The governing rule is:

Guards decide applicability.  
They do not mutate state, execute effects, fetch data, or own persistence.