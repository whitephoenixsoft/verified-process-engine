# VPE Built-in Transform Catalog

Status: Draft  
Scope: Built-in transform operations, parameter shapes, validation rules, semantic patch behavior, and failure behavior  
Audience: Project architect, compiler implementers, runtime implementers, law authors, API designers, future contributors  
Related Documents: VPE Transform Architecture, VPE Guard Architecture, VPE Law Reference, VPE Migration Playbook, VPE Core API, VPE App API, VPE Process Evolution API

---

## 1. Purpose

This document defines the initial built-in transform operations for VPE.

It exists to:

- document each built-in transform operation
- define expected parameters
- define validation behavior
- define source and target schema rules
- define deterministic runtime behavior
- define semantic patch output
- define common misuse and failure behavior
- provide a stable reference for law authors and compiler implementers

This document is the transform catalog.

The Transform Architecture document defines the registry and lifecycle model.

This catalog defines the built-in transform functions available by default.

---

## 2. Core Mental Model

Transforms define how old process meaning is lifted into new process meaning.

Transforms are primarily used in migration rules.

They do not update the database directly.

They produce semantic transformation intent that the host may apply to its own persistence model.

Core rule:

Guards decide whether a migration rule applies.  
Transforms define how old meaning is lifted into new meaning.

---

## 3. General Transform Rules

All built-in transforms must be:

- deterministic
- side-effect free
- schema-valid
- explicit about inputs
- explicit about outputs
- compatible with manifest and lift requirements
- safe for replay and simulation
- safe for on-access lift

Transforms must not:

- perform I/O
- call external services
- read host storage directly
- access hidden host state
- execute effects
- mutate history
- write to read-only namespaces
- silently ignore failure

---

## 4. Namespace Rules

Built-in transforms may normally read from:

- rec.*
- ext.*
- sys.*
- calc.*

Built-in transforms may normally write only to:

- rec.*

Transforms must not write to:

- sys.*
- ext.*
- calc.*

unless a future extension explicitly defines a safe derived-output namespace.

The default writable namespace is rec.*.

---

## 5. Source And Target Schema Rules

Migration transforms may involve two schema meanings:

- source schema
- target schema

The source schema describes old-version truth.

The target schema describes new-version truth.

A transform that reads old data must validate its source path against source schema or supplied old-version context.

A transform that writes new data must validate its target path against target schema.

If a transform output cannot be valid under the target schema, compilation must fail.

---

## 6. Semantic Patch Output

Every transform contributes to a semantic patch.

A semantic patch is not:

- a database update
- a physical storage rewrite
- a final stored record

A semantic patch is:

- transformation intent
- host-facing instruction
- lawful target meaning

Core rule:

The semantic patch is instruction.  
The host commit is reality.

---

## 7. Failure Behavior

Transform failure is different from migration rule non-applicability.

A migration rule is not applicable when its guards fail.

A transform failure happens after a migration rule has matched but the transform cannot be performed safely.

Examples:

- source value missing
- target path missing
- type mismatch
- map entry missing
- regex does not match
- cast fails
- output path conflict

A failed transform generally makes the lift incompatible unless another deterministic migration path applies.

---

## 8. Built-in Transform List

Initial built-in transforms:

- Set
- Move
- Copy
- Map
- Split
- Combine
- Cast
- RegexExtract
- Custom

Custom is included as a registered transform entry point, not as arbitrary inline code.

---

## 9. Set Transform

### 9.1 Purpose

Set assigns a literal value to a target path.

Use Set when the new process version requires a value that can be deterministically supplied.

Common uses:

- adding a new required field
- setting migration provenance
- assigning a default value
- setting a new status
- marking that a record was migrated

### 9.2 Conceptual Shape

Fields:

- op: set
- target: target path
- value: literal value

### 9.3 Example Meaning

Set rec.payment_status to Unknown.

Set rec.migrated_from_version to 1.0.0.

Set rec.account_status to Active.

### 9.4 Input Requirements

Set requires no source input unless the value is derived from another source, which should be modeled by another transform.

### 9.5 Output Writes

Set writes one target path.

### 9.6 Validation Rules

Compilation must validate:

- target path exists in target schema
- target namespace is writable
- literal value is compatible with target type
- target path is not written by another transform unless overwrite is explicitly allowed
- value is deterministic and serializable

### 9.7 Runtime Behavior

At runtime, Set produces a semantic patch operation assigning the literal value to the target path.

### 9.8 Failure Behavior

Set fails when:

- target path is missing
- target namespace is read-only
- value does not match target type
- value is not serializable
- target write conflicts with another transform

### 9.9 Common Misuse

Avoid using Set to hide unknown meaning.

For example, setting payment_status to Captured for old records is only valid if the domain meaning supports that interpretation.

---

## 10. Move Transform

### 10.1 Purpose

Move reads a value from a source path and writes it to a target path.

Use Move for field renames or old-path to new-path migration.

### 10.2 Conceptual Shape

Fields:

- op: move
- from: source path
- to: target path

### 10.3 Example Meaning

Move rec.user_name to rec.username.

Move rec.total to rec.order_total.

### 10.4 Input Requirements

Move requires the source path.

### 10.5 Output Writes

Move writes one target path.

### 10.6 Validation Rules

Compilation must validate:

- source path exists in source schema or supplied source context
- target path exists in target schema
- target namespace is writable
- source type is compatible with target type
- target path is not written by another transform unless overwrite is explicitly allowed

### 10.7 Runtime Behavior

At runtime, Move reads the source value and writes it to the target meaning through the semantic patch.

Move does not guarantee the host physically deletes the old field.

Move means the old value now has new semantic meaning at the target path.

### 10.8 Failure Behavior

Move fails when:

- source value is missing
- source path is invalid
- target path is invalid
- source and target types are incompatible
- target write conflicts with another transform

### 10.9 Common Misuse

Do not treat Move as a storage delete operation.

The host may still retain the old physical field depending on its persistence model.

---

## 11. Copy Transform

### 11.1 Purpose

Copy reads a value from a source path and writes it to a target path without implying semantic removal from the source meaning.

Use Copy when the old meaning remains valid but the new version also needs the value somewhere else.

### 11.2 Conceptual Shape

Fields:

- op: copy
- from: source path
- to: target path

### 11.3 Example Meaning

Copy rec.email to rec.contact_email.

Copy rec.customer_id to rec.billing_customer_id.

### 11.4 Input Requirements

Copy requires the source path.

### 11.5 Output Writes

Copy writes one target path.

### 11.6 Validation Rules

Compilation must validate:

- source path exists
- target path exists
- target namespace is writable
- source and target types are compatible
- target write does not conflict

### 11.7 Runtime Behavior

At runtime, Copy reads the source value and writes an equivalent value to the target path in the semantic patch.

### 11.8 Failure Behavior

Copy fails when:

- source value is missing
- source or target path is invalid
- source and target types are incompatible
- target write conflicts

### 11.9 Common Misuse

Do not use Copy when the old field has been semantically replaced.

Use Move when the old meaning is being relocated.

---

## 12. Map Transform

### 12.1 Purpose

Map converts a source value into a target value using an explicit mapping table.

Use Map for legacy code conversion.

Common uses:

- number to enum
- string code to enum
- old status to new status
- old priority code to new priority name

### 12.2 Conceptual Shape

Fields:

- op: map
- from: source path
- target: target path
- mapping: key/value mapping table
- default: optional fallback value

### 12.3 Example Meaning

Map priority code 1 to High.

Map priority code 2 to Medium.

Map priority code 3 to Low.

Map legacy status P to PendingPayment.

### 12.4 Input Requirements

Map requires the source path.

### 12.5 Output Writes

Map writes one target path.

### 12.6 Validation Rules

Compilation must validate:

- source path exists
- target path exists
- target namespace is writable
- mapping keys are compatible with source type
- mapping values are compatible with target type
- default value, if present, is compatible with target type
- target write does not conflict

### 12.7 Runtime Behavior

At runtime, Map reads the source value and looks up the matching target value.

If a mapping exists, it writes the mapped target value.

If no mapping exists and a default is present, it writes the default.

If no mapping exists and no default is present, the transform fails.

### 12.8 Failure Behavior

Map fails when:

- source value is missing
- source value has no mapping and no default
- mapping output violates target type
- target write conflicts

### 12.9 Common Misuse

Do not use Map for complex domain logic that should be modeled as guarded migration rules.

Map should remain a deterministic value translation.

---

## 13. Split Transform

### 13.1 Purpose

Split converts one source value into multiple target values.

Use Split when a legacy field contains multiple pieces of meaning.

Common uses:

- full name to first name and last name
- compound identifier to separate parts
- delimited string to multiple fields

### 13.2 Conceptual Shape

Fields:

- op: split
- from: source path
- targets: list of target paths
- strategy: delimiter, fixed width, or other deterministic split strategy
- delimiter: optional, when strategy requires it
- parts: optional target part mapping
- fallback: optional behavior when split fails

### 13.3 Example Meaning

Split rec.full_name into rec.first_name and rec.last_name using space delimiter.

Split rec.legacy_code into rec.region and rec.local_id using hyphen delimiter.

### 13.4 Input Requirements

Split requires the source path.

### 13.5 Output Writes

Split writes multiple target paths.

### 13.6 Validation Rules

Compilation must validate:

- source path exists
- source value is compatible with the split strategy
- all target paths exist
- all target namespaces are writable
- output parts are compatible with target types
- split strategy is deterministic
- target writes do not conflict

### 13.7 Runtime Behavior

At runtime, Split reads the source value and deterministically produces target values.

The produced values are written as semantic patch operations.

### 13.8 Failure Behavior

Split fails when:

- source value is missing
- source value cannot be split according to strategy
- required number of parts is not produced
- output value does not satisfy target type
- any target write conflicts

### 13.9 Common Misuse

Do not use Split for ambiguous parsing.

For example, splitting a full name by space may not be valid if names can contain multiple spaces or cultural naming differences.

Use a custom transform or explicit repair flow when splitting is not deterministic enough.

---

## 14. Combine Transform

### 14.1 Purpose

Combine converts multiple source values into one target value.

Use Combine when the new schema consolidates separate old fields.

Common uses:

- first name and last name to full name
- region and local id to compound identifier
- date parts to a single date string

### 14.2 Conceptual Shape

Fields:

- op: combine
- from: list of source paths
- target: target path
- strategy: deterministic combine strategy
- separator: optional, when strategy requires it
- template: optional, when template-based combine is used

### 14.3 Example Meaning

Combine rec.first_name and rec.last_name into rec.full_name using space separator.

Combine rec.region and rec.local_id into rec.customer_code using hyphen separator.

### 14.4 Input Requirements

Combine requires all listed source paths.

### 14.5 Output Writes

Combine writes one target path.

### 14.6 Validation Rules

Compilation must validate:

- all source paths exist
- target path exists
- target namespace is writable
- combine strategy is deterministic
- output is compatible with target type
- target write does not conflict

### 14.7 Runtime Behavior

At runtime, Combine reads source values and deterministically constructs one target value.

The result is written as a semantic patch operation.

### 14.8 Failure Behavior

Combine fails when:

- any required source value is missing
- combine strategy cannot produce a valid result
- output violates target type
- target write conflicts

### 14.9 Common Misuse

Do not use Combine to hide missing required values.

If a source field is optional, the fallback behavior must be explicit.

---

## 15. Cast Transform

### 15.1 Purpose

Cast converts a source value from one type to another.

Use Cast when the target schema changes type but the conversion is deterministic.

Common uses:

- string to number
- number to string
- string to boolean
- number to enum when the mapping is simple and explicit
- string to enum when values already match exactly

### 15.2 Conceptual Shape

Fields:

- op: cast
- from: source path
- target: target path
- to_type: target type
- on_failure: optional failure behavior

### 15.3 Example Meaning

Cast rec.amount_text to rec.amount as number.

Cast rec.is_active_text to rec.is_active as boolean.

### 15.4 Input Requirements

Cast requires the source path.

### 15.5 Output Writes

Cast writes one target path.

### 15.6 Validation Rules

Compilation must validate:

- source path exists
- target path exists
- target namespace is writable
- requested cast is allowed
- target type matches target schema
- failure behavior is explicit when needed
- target write does not conflict

### 15.7 Runtime Behavior

At runtime, Cast reads the source value and attempts a deterministic conversion.

If successful, it writes the converted value.

If unsuccessful, it follows explicit failure behavior or fails.

### 15.8 Failure Behavior

Cast fails when:

- source value is missing
- cast is not supported
- value cannot be converted safely
- converted value violates target type
- target write conflicts

### 15.9 Common Misuse

Avoid unsafe or lossy casts unless explicitly allowed.

If value meaning changes, prefer Map instead of Cast.

---

## 16. RegexExtract Transform

### 16.1 Purpose

RegexExtract extracts named captures from a source string and writes them to target paths.

Use RegexExtract for controlled legacy string formats.

Common uses:

- parsing legacy labels
- extracting region and id from a code
- extracting tier from a formatted customer label
- splitting structured identifiers

### 16.2 Conceptual Shape

Fields:

- op: regex_extract
- from: source path
- pattern: deterministic regex pattern
- targets: capture name to target path mapping
- on_no_match: optional failure behavior

### 16.3 Example Meaning

Extract tier and id from rec.customer_label.

Pattern captures tier and id, then writes them to rec.customer_tier and rec.customer_number.

### 16.4 Input Requirements

RegexExtract requires the source path.

### 16.5 Output Writes

RegexExtract writes all target paths mapped from captures.

### 16.6 Validation Rules

Compilation must validate:

- source path exists
- source type is string-compatible
- regex compiles
- regex contains required named captures
- each capture maps to a valid target path
- each target namespace is writable
- extracted values are compatible with target types or explicitly cast
- target writes do not conflict

### 16.7 Runtime Behavior

At runtime, RegexExtract reads the source string and evaluates the regex.

If the regex matches, named captures are written to target paths.

If the regex does not match, the transform follows explicit no-match behavior or fails.

### 16.8 Failure Behavior

RegexExtract fails when:

- source value is missing
- source value is not string-compatible
- regex does not match and no fallback exists
- required capture is missing
- extracted value violates target type
- target write conflicts

### 16.9 Common Misuse

Do not use RegexExtract for loosely structured or unpredictable data.

If the format is not stable, use a custom transform or repair workflow.

---

## 17. Custom Transform

### 17.1 Purpose

Custom allows domain-specific deterministic transform logic to be registered and referenced by name.

Custom is not arbitrary inline scripting.

It is a registry-backed transform entry point.

### 17.2 Conceptual Shape

Fields:

- op: custom
- name: registered transform name
- params: transform-specific parameters
- inputs: optional explicit input paths
- outputs: optional explicit output paths

### 17.3 Example Meaning

Apply registered transform SplitFullName.

Apply registered transform NormalizeLegacySku.

Apply registered transform ParseCustomerSegment.

### 17.4 Input Requirements

Custom transform input requirements are declared by the registered transform definition.

The law may also provide explicit inputs if the transform definition supports or requires them.

### 17.5 Output Writes

Custom transform output writes are declared by the registered transform definition.

The law may also provide explicit outputs if the transform definition supports or requires them.

### 17.6 Validation Rules

Compilation must validate:

- custom transform name exists in TransformRegistry
- parameters match registered parameter shape
- declared inputs exist
- declared outputs exist
- output namespaces are writable
- output types are compatible with target schema
- transform declares deterministic behavior
- output writes do not conflict

### 17.7 Runtime Behavior

At runtime, VPE invokes the compiled registered transform using explicit supplied truth.

The custom transform returns deterministic semantic patch operations or deterministic failure.

### 17.8 Failure Behavior

Custom fails when:

- transform is not registered
- parameters are invalid
- required input is missing
- output violates schema
- custom transform returns deterministic failure
- output write conflicts

### 17.9 Common Misuse

Do not use Custom to bypass law clarity.

If behavior can be expressed with built-in transforms, prefer built-in transforms.

Custom should be reserved for deterministic domain-specific transformations that are too specialized for built-ins.

---

## 18. Duplicate Output Path Rule

By default, two transforms in the same migration rule must not write the same target path.

Duplicate writes are ambiguous unless the transform model explicitly supports ordered overwrite.

Recommended default:

- duplicate target writes fail compilation
- intentional overwrite is not supported initially

This keeps migration behavior deterministic and easier to debug.

---

## 19. Missing Input Rule

A transform must not silently treat missing input as empty, null, or false unless the transform explicitly defines that behavior.

Missing input should generally cause transform failure.

If fallback behavior is desired, it must be explicit.

Examples of explicit fallback behavior:

- default value
- on_missing behavior
- alternate migration rule
- repair workflow

---

## 20. Fallback Behavior

Some transforms may support fallback behavior.

Fallback behavior must be explicit.

Examples:

- Map default
- Cast on_failure
- RegexExtract on_no_match
- Split fallback

Fallback must be deterministic.

Fallback must still satisfy target schema.

---

## 21. Transform Diagnostics

Each built-in transform should produce clear diagnostics.

Diagnostics should identify:

- transform operation
- migration rule location
- source path
- target path
- invalid parameter
- missing source
- missing target
- type mismatch
- namespace violation
- output conflict
- fallback issue
- likely correction when safe

Diagnostics should be stable enough for CLI output and tests.

---

## 22. Transform Catalog Summary

Built-in transform purposes:

- Set: assign a literal target value
- Move: move old meaning to a new target path
- Copy: duplicate existing meaning into a new target path
- Map: convert old value to new value through explicit table
- Split: convert one old value into multiple target values
- Combine: convert multiple old values into one target value
- Cast: convert compatible types deterministically
- RegexExtract: extract structured values from controlled string patterns
- Custom: call registered deterministic transform logic

---

## 23. Anti-Patterns

Avoid:

- using transforms as database scripts
- using transforms as hidden business logic
- using transforms to guess missing data
- using regex for unstable formats
- using cast where map is semantically required
- using custom transforms when built-ins are sufficient
- silently ignoring missing input
- silently overwriting target paths
- allowing arbitrary inline scripts
- treating semantic patches as persistence

---

## 24. Final Summary

Built-in transforms provide the standard operations needed to lift old process meaning into new process meaning.

They are deterministic, schema-validated, and registry-backed.

They produce semantic patch operations.

They do not persist data.

They do not execute effects.

They do not fetch hidden host data.

The governing rule is:

Transforms define lift.  
Semantic patches describe intent.  
The host commits reality.