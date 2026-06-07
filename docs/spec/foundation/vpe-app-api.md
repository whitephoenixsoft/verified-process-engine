# VPE App API Foundation

## Purpose

The App API remains the preferred application-facing integration surface.

Expand responsibilities to include:

- version-aware decision execution
- surfacing runtime lift outcomes
- version-aware decision input construction

Clarify:

The Core API remains the canonical semantic and architectural surface of VPE.

The App API exists to make ordinary host-side decision execution feel natural without redefining Core API meaning.

---

## Governing Principles

Add:

### Version Is Part Of Truth

Version participates in runtime correctness.

An instance is not merely in a state.

An instance is in a state under a particular process version.

The wrapper may simplify interaction with versioned truth but must not hide version information when version affects runtime behavior.

The canonical definition of version-aware truth belongs to the Core API.

### Application Integration Without Semantic Loss

The App API should optimize application integration effort without reducing semantic correctness.

Convenience must not come at the expense of correctness-relevant truth.

---

## ProcessHandle

Expand responsibilities:

- identify the process
- expose friendly metadata
- expose process version information
- expose required-data information
- support version-aware decision execution
- surface runtime lift outcomes
- create builders when appropriate

Clarify:

A ProcessHandle represents a specific process version.

When multiple versions are installed, each handle represents the version it was created from.

Suggested shape:

- id()
- version()
- digest()
- required_data_for(state)
- decide(input)
- prepare_decision()

---

## RequiredData

RequiredData describes what the host must gather before VPE can evaluate a decision from a particular versioned state.

RequiredData may include:

- required context paths
- required history requirements
- anchor expectations
- current version requirements
- target version requirements
- migration-related requirements

RequiredData describes obligations.

The Core API defines their meaning.

---

## DecisionInput

DecisionInput remains an explicit representation of supplied truth.

Required fields:

- current_version
- target_version
- current_state
- action
- context
- history
- anchor

Optional fields:

- now
- trace_id
- actor metadata
- lift_strategy

Clarify:

target_version normally defaults to the ProcessHandle version.

DecisionInput represents supplied truth at evaluation time.

Version information is correctness-relevant truth and must remain visible.

---

## DecisionHistory

No structural changes.

Clarify:

History may participate in both decision evaluation and version-aware execution.

The wrapper must not infer missing historical truth.

---

## DecisionOutcome

Expand required properties:

- process identity
- source version
- target version
- from-state
- to-state
- emitted events
- effect intent
- persistence expectations
- optional lift outcome

Clarify:

DecisionOutcome represents what VPE concluded from supplied truth.

DecisionOutcome is not a persistence record.

LiftOutcome is defined by the Core API.

The App API may surface lift information but does not define it.

A successful lift does not imply a successful decision.

---

## PersistencePlan

PersistencePlan remains descriptive.

It may include:

- persistence expectations
- target version
- version-transition expectations

When version-crossing occurs, version-transition artifacts and decision artifacts must be committed atomically.

PersistencePlan describes obligations.

It does not perform persistence.

---

## DecisionBuilder

Support:

- current_version(...)
- target_version(...)
- lift_strategy(...)

Builder support remains optional.

Direct construction and builder construction must remain semantically equivalent.

---

## Relationship To Core API

Add:

### Version And Lift Semantics

The App API does not define:

- LiftStatus
- LiftOutcome
- LiftPath
- LiftStep
- LiftEvent
- SemanticPatch
- MigrationRuleReference
- IncompatibilityReason
- migration determinism rules

These concepts belong to the Core API.

The App API may expose them during normal application-facing execution.

Expand Core API responsibilities to explicitly include:

- canonical runtime semantics
- version-aware truth semantics
- lift semantics
- migration semantics

---

## Explicit Limitations

The App API does not define:

- migration semantics
- lift semantics
- replay semantics
- simulation semantics
- migration compatibility rules

Those belong to the Core API or Process Evolution API.

---

## Event-Sourced Hosts

Clarify:

Event-sourced hosts may reconstruct versioned truth from history rather than storing versioned snapshots.

Versioned truth remains part of the supplied decision input.

---

## Concurrency Visibility

Decision outcomes remain valid only relative to supplied truth.

Truth includes:

- version
- state
- context
- history
- anchor

Version advancement is truth advancement.

A version transition is considered a truth transition for concurrency purposes.

---

## Final Position

The App API remains the preferred application-facing integration surface.

The App API is:

- version-aware
- lift-aware
- concurrency-aware

The Core API defines:

- semantic meaning
- architectural meaning
- version semantics
- lift semantics
- migration semantics

The Process Evolution API uses those semantics for:

- migration workflows
- replay
- simulation
- diagnostics
- readiness analysis

Final rule:

Host supplies versioned truth.

VPE evaluates truth using Core API semantics.

VPE surfaces runtime lift results when necessary.

Host commits atomically.

The Core API defines what VPE means.

The App API defines how applications use VPE.