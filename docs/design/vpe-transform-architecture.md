# VPE Transform Architecture

Status: Draft  
Scope: Transform registry, transform validation, migration/lift integration, semantic patch generation, runtime transform execution, and custom transform extension model  
Audience: Project architect, compiler implementers, runtime implementers, API designers, future contributors  
Related Documents: VPE Spec, VPE Invariants, VPE Law Reference, VPE Migration Playbook, VPE Core API, VPE App API, VPE Process Evolution API, VPE Guard Architecture, VPE Built-in Transform Catalog

---

## 1. Purpose

This document defines the architecture for transforms in VPE.

It exists to:

- define how transforms participate in migration/lift
- define how transform operations are validated
- define how transform requirements are derived
- define how transforms produce semantic patches
- define how built-in and custom transforms are registered
- prevent transform-specific logic from becoming hardcoded throughout the compiler
- ensure transforms remain deterministic, explicit, and host-data-safe
- provide a validation anchor for Core API, App API, Process Evolution API, Spec, and Invariants

This document is not the transform catalog.

The transform catalog defines specific transform functions.

This document defines the architecture that all transform functions must follow.

---

## 2. Core Position

Transforms define how old process meaning is lawfully lifted into new process meaning.

A transform is not a database update.

A transform is not arbitrary scripting.

A transform is a deterministic semantic operation over explicit supplied truth.

Transforms are primarily used in migration rules.

They may produce semantic patches that the host can apply to its own persistence model.

Core rule:

Guards decide whether a migration rule applies.  
Transforms define how old meaning is lifted into new meaning.

---

## 3. Relationship To Migration

Migration rules may contain transforms.

A migration rule is allowed to apply only if its guards pass.

Once a migration rule applies, its transforms define the semantic changes required to interpret old-version truth under the target version.

A migration rule may use transforms to:

- map an old state to a new state
- set newly required values
- move old values to new paths
- copy existing values
- map legacy values into new meanings
- split old fields into multiple new fields
- combine multiple fields into one new field
- cast compatible values into new types
- extract structured data using deterministic parsing
- call named custom deterministic transform definitions

Transforms are part of lawful lift.

They must be validated before runtime.

---

## 4. Relationship To Runtime Decisioning

Transforms are not normal transition logic.

Normal runtime decisioning evaluates guards, selects a transition, and emits events/effects as intent.

Transforms are used when versioned truth must be lifted before normal decisioning can proceed.

When on-access lift occurs:

1. VPE detects that the instance version differs from the target process version.
2. VPE finds a lawful lift path.
3. VPE evaluates migration guards.
4. VPE applies migration transforms semantically.
5. VPE produces a lift outcome.
6. VPE may then continue normal decisioning using lifted truth.
7. The host persists the lift event and semantic patch if it accepts the outcome.

Transforms may participate in runtime through lift, but they are not ordinary transition effects.

---

## 5. Transform Registry

### 5.1 Purpose

The TransformRegistry is the authoritative registry of available transform definitions.

A law may only reference transform operations that exist in the registry.

The compiler should not contain hardcoded logic for every transform operation.

Instead, the compiler should:

1. Look up the transform operation in TransformRegistry.
2. Ask the transform definition to validate its parameters.
3. Ask the transform definition to declare its input requirements and output writes.
4. Ask the transform definition to validate against source and target schema.
5. Build the compiled/runtime transform representation.

### 5.2 Built-in transforms

Built-in transforms are registered in the same registry as custom transforms.

There should be no semantic privilege for built-in transforms beyond being available by default.

### 5.3 Custom transforms

Custom transforms should participate in the same pipeline as built-in transforms.

A custom transform must be able to:

- validate parameters
- declare input requirements
- declare output writes
- validate schema compatibility
- build a deterministic runtime transform
- produce diagnostics
- respect determinism constraints

### 5.4 Registry invariant

The registry is the extension boundary for transform behavior.

Adding a new transform should not require modifying core compiler control flow.

---

## 6. Transform Definition

A transform definition is the compile-time description of a transform operation.

It should define:

- operation name
- expected parameter shape
- validation rules
- input requirements
- output writes
- schema compatibility rules
- runtime construction rules
- diagnostic behavior

Conceptually, every transform definition must answer:

- Is this transform usage valid?
- What input data does this transform require?
- What output paths does this transform write?
- Are the source and target types compatible?
- What runtime transform should be built?
- What errors or warnings should be emitted?

---

## 7. Runtime Transform

A runtime transform is the executable transform instance used during lift.

It should be created only after compile-time validation succeeds.

A runtime transform receives explicit supplied truth, such as:

- current context
- lifted context in progress
- source schema interpretation
- target schema interpretation
- explicit system values if allowed

It returns:

- semantic patch operation
- transformed value
- deterministic failure if required truth is missing or invalid

A runtime transform must not discover new dependencies during execution.

All dependencies must already be known from compile-time validation.

---

## 8. Transform Compilation Flow

For each transform in a migration rule:

1. Read the transform operation.
2. Look up the transform definition in TransformRegistry.
3. Validate transform parameters.
4. Validate source paths against source schema.
5. Validate target paths against target schema.
6. Validate type compatibility.
7. Validate namespace write rules.
8. Collect input requirements.
9. Collect output writes.
10. Add requirements to migration/lift requirements.
11. Build runtime transform representation.
12. Store compiled transform in the compiled process artifact.

If any required validation fails, compilation fails.

No partial transform compilation is allowed.

---

## 9. Transform Requirements

Every transform must declare all truth it requires.

Transform requirements may include:

- source context paths
- target context paths when read-before-write is required
- system paths if explicitly allowed
- history requirements if the transform depends on history
- version metadata
- state metadata

Examples:

- Move rec.old_name to rec.name requires rec.old_name
- Copy rec.email to rec.contact_email requires rec.email
- Split rec.full_name to rec.first_name and rec.last_name requires rec.full_name
- Map rec.priority_code to rec.priority requires rec.priority_code
- RegexExtract rec.label to rec.tier and rec.number requires rec.label

A transform must not read data it did not declare.

---

## 10. Output Writes

Every transform must declare all paths it may write.

Output writes are required for:

- schema validation
- namespace enforcement
- conflict detection
- semantic patch construction
- diagnostics

A transform must not write to:

- sys.*
- ext.*
- calc.*

unless a future extension explicitly allows a safe derived-output namespace.

The normal writable namespace is:

- rec.*

---

## 11. Schema Validation

Transforms must validate all referenced paths against schema.

Validation must confirm:

- input paths exist when required
- target paths exist in target schema
- namespace rules are respected
- value types are compatible
- literal values match target type
- maps only produce target-compatible values
- casts are safe and deterministic
- split/combine operations have valid target shapes
- custom transforms declare target-compatible outputs

If a transform writes a value that cannot be valid under the target schema, compilation must fail.

---

## 12. Source Schema And Target Schema

Migration transforms may reason across two schema meanings:

- source schema
- target schema

The source schema describes the old-version truth.

The target schema describes the new-version truth.

Transforms must validate against both where applicable.

Examples:

- Move reads from source schema and writes to target schema.
- Set writes a literal compatible with target schema.
- Map reads a source-compatible value and writes a target-compatible value.
- Split reads one source field and writes multiple target fields.
- Combine reads multiple source fields and writes one target field.

---

## 13. Determinism Requirements

All transforms must be deterministic.

A transform must not depend on:

- random values
- current system time unless supplied explicitly
- external service calls
- database reads
- network calls
- global mutable state
- hidden host state
- unordered collection behavior
- locale-dependent parsing unless locale is explicit and deterministic

If time or locale is required, it must be supplied explicitly.

---

## 14. Semantic Patch

A transform produces or contributes to a semantic patch.

A semantic patch is a host-facing description of lawful transformation intent.

It is not:

- a database update
- a final stored record
- a physical storage rewrite
- a guarantee that host storage has the same shape as VPE context paths

A semantic patch may contain operations such as:

- set value
- move value
- copy value
- map value
- split value
- combine values
- cast value
- extract values
- custom deterministic transform result

Core rule:

The semantic patch is instruction.  
The host commit is reality.

---

## 15. Relationship To Lift Event

A lift result should include both:

- semantic patch
- lift event

The semantic patch describes transformation intent.

The lift event records lineage.

The lift event proves:

- source version
- target version
- source state
- target state
- selected lift strategy
- matched migration rule or path
- lawful version transition occurred

Core rule:

The lift event is lineage.  
The semantic patch is instruction.  
The host commit is reality.

---

## 16. Transform Errors Vs Transform Non-Applicability

Transform architecture should distinguish transform failure from migration rule non-applicability.

### Migration rule non-applicability

A migration rule is not applicable when its guards fail.

In that case, transforms are not applied.

### Transform failure

A transform failure means the rule matched, but the transformation could not be performed safely.

Examples:

- required input missing
- type conversion failed
- regex did not match and no fallback was defined
- map did not contain a value and no default was defined
- custom transform failed deterministically
- output target violates schema

Transform failure should produce an incompatible lift result unless explicitly recoverable by another deterministic rule.

---

## 17. Transform Ordering

Transforms within a migration rule must execute in deterministic order.

The order in the law is the execution order unless another ordering rule is explicitly defined.

Later transforms may depend on outputs of earlier transforms only if this is explicitly allowed and deterministic.

If transforms write the same target path ambiguously, compilation must fail unless the overwrite order is explicitly allowed by the transform model.

Recommended default:

- duplicate writes to the same target path are invalid
- intentional overwrite must be explicit if ever supported

---

## 18. Conflict Detection

The compiler should detect transform conflicts.

Conflicts may include:

- two transforms writing the same target path
- transform writes to read-only namespace
- transform writes a value incompatible with target schema
- transform output conflicts with state mapping
- custom transform declares unknown outputs
- transform depends on output that may not exist
- transform order ambiguity

Ambiguous transform output must fail compilation or deterministic validation.

It must not be silently resolved.

---

## 19. Built-In Transform Families

The initial built-in transform families include:

- simple assignment
- field movement
- field copying
- value mapping
- field splitting
- field combining
- type casting
- regex extraction
- named custom deterministic transform

These should remain small, explicit, and composable.

A general scripting language should not be introduced prematurely.

---

## 20. Set Transform

Set assigns a literal value to a target path.

Use for:

- new required fields
- migration flags
- default values
- provenance fields

Example meaning:

- set rec.payment_status to Unknown
- set rec.migrated_from_version to 1.0.0

Validation:

- target path must exist in target schema
- target namespace must be writable
- value must match target type

---

## 21. Move Transform

Move reads a value from a source path and writes it to a target path.

Use for:

- field renames
- old path to new path migration

Validation:

- source path must exist in source schema or supplied context
- target path must exist in target schema
- source and target types must be compatible
- target namespace must be writable

Move does not imply the host physically deletes the old field.

It describes semantic movement into new meaning.

---

## 22. Copy Transform

Copy reads a value from a source path and writes it to a target path without implying semantic removal from the source meaning.

Use for:

- denormalized target values
- preserving old value while adding new path
- duplicating value into new schema shape

Validation:

- source path must exist
- target path must exist
- types must be compatible

---

## 23. Map Transform

Map converts a source value into a target value using an explicit mapping table.

Use for:

- number to enum
- string code to enum
- legacy status to new status
- priority code to priority name

Validation:

- source path must exist
- target path must exist
- mapping keys must be compatible with source type
- mapping values must be compatible with target type
- missing source value behavior must be explicit

If no mapping entry matches and no default exists, transform fails.

---

## 24. Split Transform

Split converts one source value into multiple target values.

Use for:

- full name to first and last name
- compound identifier to components
- delimited values to multiple fields

Validation:

- source path must exist
- all target paths must exist
- target namespace must be writable
- split strategy must be deterministic
- failure behavior must be explicit

Common split strategies may include:

- delimiter
- fixed width
- regex extraction

Regex-based split may be modeled as RegexExtract instead.

---

## 25. Combine Transform

Combine converts multiple source values into one target value.

Use for:

- first and last name to full name
- multiple code fields into a composite key
- separate date parts into one value

Validation:

- all source paths must exist
- target path must exist
- combine strategy must be deterministic
- output must match target type

---

## 26. Cast Transform

Cast converts a value from one type to another.

Use for:

- string to number
- number to string
- string to boolean
- number to enum where rules are explicit

Validation:

- source path must exist
- target path must exist
- cast must be deterministic
- invalid cast behavior must be explicit
- target type must be satisfied

Unsafe or lossy casts should either be rejected or require explicit opt-in.

---

## 27. RegexExtract Transform

RegexExtract extracts named captures from a source string and writes them to target paths.

Use for:

- parsing legacy labels
- extracting identifiers
- splitting structured strings
- parsing controlled legacy formats

Validation:

- source path must exist and be string-compatible
- regex must compile
- regex must be deterministic
- named captures must map to target paths
- target paths must exist
- extracted values must satisfy target types or be cast explicitly

If regex does not match and no fallback is defined, transform fails.

---

## 28. Custom Transform

Custom transforms support domain-specific deterministic transformation logic.

A custom transform must be named and registered.

It must declare:

- name
- parameter shape
- input paths
- output paths
- type rules
- determinism guarantees
- runtime behavior
- diagnostic behavior

Custom transforms must not be arbitrary inline code in the law.

The law should reference the custom transform by stable name.

---

## 29. Custom Transform Requirements

A custom transform must:

- have a stable name
- declare parameter shape
- validate parameters
- validate schema references
- declare all input requirements
- declare all output writes
- build a deterministic runtime transform
- produce diagnostics
- avoid hidden state
- avoid I/O
- be stateless after construction

A custom transform should behave exactly like a built-in transform once registered.

---

## 30. Transform Diagnostics

Transform validation should produce diagnostics useful to:

- law authors
- compiler users
- CLI users
- API integrators

Diagnostics should identify:

- transform operation
- migration rule where used
- invalid parameter
- missing source path
- missing target path
- type mismatch
- invalid namespace write
- conflicting output path
- missing map entry behavior
- regex mismatch behavior
- suggested correction when safe

Diagnostics should be deterministic and stable enough for CLI output and tests.

---

## 31. Compile-Time Errors

Compilation must fail when:

- transform operation is unknown
- transform parameters are invalid
- source path does not exist
- target path does not exist
- output namespace is not writable
- source/target types are incompatible
- transform requirements cannot be derived
- transform output conflicts are unresolved
- custom transform is not registered
- custom transform violates registry constraints
- transform behavior is nondeterministic
- transform ambiguity cannot be resolved

No invalid transform should reach runtime.

---

## 32. Runtime Errors

Runtime transform errors may occur when:

- required source value is missing
- supplied context violates expected runtime shape
- required history is missing when explicitly needed
- regex does not match
- mapping table has no matching entry
- cast fails
- custom transform fails deterministically

Runtime transform errors should be reported clearly.

A failed transform during lift generally means the lift is incompatible unless the law provides another deterministic applicable lift path.

---

## 33. Transform Registry Invariants

The following invariants must hold:

- all transform operations must resolve through the registry
- built-in and custom transforms follow the same pipeline
- compiler does not hardcode transform-specific behavior long-term
- transform input requirements are complete
- transform output writes are complete
- transforms are deterministic
- transforms do not execute effects
- transforms do not perform I/O
- transforms do not access undeclared data
- transforms do not write read-only namespaces
- unknown transforms fail compilation
- ambiguous transform behavior fails deterministically
- transform output conflicts fail unless explicitly resolved by rule

---

## 34. Relationship To Core API

The Core API should define or expose canonical transform-related concepts needed by compiler and runtime.

This may include:

- TransformRegistry
- TransformDefinition
- TransformValidation
- TransformRequirement
- TransformWrite
- TransformOperation
- SemanticPatch
- runtime transform evaluator
- transform diagnostics

The Core API owns the exact semantics.

---

## 35. Relationship To App API

The App API should not expose transform internals as its normal surface.

However, App API concepts such as LiftOutcome, SemanticPatch, and PersistencePlan are downstream of transform behavior.

Application developers should see:

- lift status
- semantic patch
- lift event
- persistence obligations
- deterministic incompatibility when transform cannot apply

They should not need to understand transform compilation internals for normal use.

---

## 36. Relationship To CLI

The CLI should expose transform diagnostics clearly.

CLI validation and compilation should help users understand:

- unknown transform operations
- invalid transform parameters
- missing source paths
- missing target paths
- type mismatches
- output conflicts
- custom transform registration issues

The CLI may present semantic patches or lift reports, but it must not redefine transform semantics.

---

## 37. Relationship To Simulation

Simulation must use the same compiled transform semantics as runtime lift.

Simulation may reveal that migration is required.

If simulation evaluates lift behavior, it must use the same transform model as runtime migration.

Simulation must not use a separate transform evaluator.

---

## 38. Relationship To Migration

Transforms are the operational body of migration.

A migration rule without transforms may still map state directly.

A migration rule with transforms defines semantic changes required for lift.

If a migration rule matches but its transforms fail, the lift cannot lawfully proceed unless another deterministic migration path applies.

---

## 39. Future Considerations

Future transform architecture may include:

- richer type inference
- code generation for transform definitions
- macro helpers
- WASM/plugin transforms
- configurable transform libraries
- transform dry-run reports
- transform coverage analysis
- transform conflict visualization

These should not be introduced until the core registry and semantic patch model are stable.

---

## 40. Anti-Patterns

Avoid:

- hardcoding each transform in compiler control flow
- allowing transforms to fetch their own data
- allowing runtime-only dependency discovery
- allowing arbitrary inline scripts
- allowing nondeterministic transforms
- treating semantic patches as database writes
- letting custom transforms bypass requirement declaration
- letting built-in transforms behave differently from custom transforms
- silently overwriting target paths
- silently ignoring transform failures
- hiding transform incompatibility as normal guard failure

---

## 41. Final Summary

Transforms are deterministic semantic operations over explicit supplied truth.

They define how old process meaning is lifted into new process meaning.

The TransformRegistry is the extension boundary.

The compiler should orchestrate transform validation and requirement collection, not hardcode transform behavior.

The runtime should apply compiled transforms against supplied context only.

The semantic patch system depends on complete transform inputs and outputs.

The governing rule is:

Guards decide applicability.  
Transforms define lift.  
The semantic patch is instruction.  
The host commit is reality.