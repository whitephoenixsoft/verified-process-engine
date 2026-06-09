# VPE Process Evolution API

Status: Draft  
Scope: Application-facing and developer-facing API for simulation, replay, explicit migration workflows, diagnostics, and release-readiness analysis  
Audience: Project architect, core implementers, API designers, test authors, future contributors  
Depends On: VPE Core API, Base runtime/application API, feature strategy, semantic center, simulation and migration roadmap  
Does NOT define: CLI command syntax, compiler internals, base runtime decision API, persistence/orchestration behavior, canonical lift semantics, canonical migration status vocabulary

---

## 1. Purpose

This document defines the intended shape of the VPE process evolution API.

It exists to:

- define how simulation, replay, and migration workflows relate to the main process abstraction
- keep process evolution distinct from normal runtime decision execution
- provide a clean home for regression testing, historical replay, migration diagnostics, and release-readiness analysis
- establish how opt-in evolution capabilities should grow without bloating the base application API
- explain how process evolution uses Core-owned version and lift semantics
- prepare a coherent long-term surface for simulation, replay, explicit migration operations, and migration tooling

This document is not a workflow guide.  
It is an API and capability-boundary document for process evolution.

---

## 2. Core Position

Process evolution is the analysis and tooling layer that uses Core-owned runtime and migration semantics.

Core API owns the canonical language of:

- version truth
- version-aware execution
- lift semantics
- migration status vocabulary
- migration error vocabulary
- lift path semantics
- migration determinism
- runtime lift ordering

Process Evolution API owns the capability surfaces for:

- simulation
- replay
- comparison
- regression analysis
- explicit migration operations
- batch migration
- repair workflows
- migration diagnostics
- release-readiness analysis

The compact rule is:

Core provides the language.  
Evolution provides the capabilities.

Simulation is an analytical capability used to understand change.

Replay is a simulation-oriented capability used to evaluate historical truth against candidate process definitions.

Migration, when invoked explicitly as a workflow, belongs to process evolution tooling. However, canonical migration/lift semantics are Core-owned and may also participate in normal runtime processing when version differences are encountered.

---

## 3. Governing Principles

### 3.1 Keep runtime and evolution distinct

Process evolution should not be collapsed into ordinary runtime decision execution.

The runtime path remains focused on executing lawful decisions from current truth.

The evolution path remains focused on:

- analysis
- comparison
- replay
- diagnostics
- release readiness
- explicit migration workflows

Simulation and replay model runtime behavior, but they do not become runtime execution.

### 3.2 Keep evolution close to the process abstraction

Although process evolution is distinct from runtime use, it should still attach naturally to the central process abstraction where possible.

A user should not have to abandon the process abstraction just to simulate, replay, compare, or run explicit migration tooling.

### 3.3 Preserve the semantic center

Process evolution must not invent alternate meanings for process legality.

Simulation, replay, and migration workflows must use the same semantic center as the rest of VPE.

Where version crossing or lift is involved, Process Evolution imports Core-owned lift semantics rather than redefining them.

### 3.4 Treat evolution as opt-in capability

Simulation, replay, comparison, and explicit migration tooling should not crowd the base application API by default.

They should appear through explicit opt-in capability layers.

Feature gating does not change ownership.

If simulation is disabled:

- SimulationResult does not exist
- ReplayResult does not exist
- simulation and replay workflows are unavailable

If explicit migration tooling is disabled:

- batch migration and repair workflows may be unavailable

Core migration concepts may still exist even when explicit migration tooling is disabled, because runtime may need to report migration-related outcomes such as MigrationNotConfigured.

### 3.5 Support release and testing discipline

The evolution API should make it natural to:

- write regression-style simulation tests
- compare process versions
- replay historical truth before release
- evaluate migration readiness
- diagnose version-crossing failure
- identify release risk before runtime exposure

### 3.6 Import Core version truth

Process Evolution imports the Core principle:

Version is part of process truth.

An instance is not merely:

- Pending

It is:

- Pending under version X

Version participates in:

- runtime correctness
- compatibility evaluation
- lift determination
- concurrency validation
- persistence planning
- simulation correctness
- replay correctness

### 3.7 Runtime equivalence

Simulation and replay must model lawful runtime behavior.

If runtime would require lift, simulation and replay must also require lift.

A simulation result that could not occur in runtime is invalid.  
A replay result that could not occur in runtime is invalid.

Simulation differs from runtime only in that simulation does not mutate persisted reality.

---

## 4. Scope of Process Evolution

The process evolution family should include:

- simulation of candidate behavior
- comparison of current and proposed process versions
- scenario-based regression analysis
- historical replay analysis
- explicit migration operations
- batch migration
- repair workflows
- migration diagnostics
- release-readiness reporting

These are related enough to live under one broader family, even if they are exposed through separate feature flags internally.

Process Evolution does not own canonical migration semantics.

It owns workflows that use Core-owned migration semantics.

---

## 5. Relationship to the Base Application API

### 5.1 Base application API role

The base application API should remain responsible for:

- process lookup
- required-data inspection for runtime use
- decision input construction
- normal decision execution
- decision outcomes for host persistence and dispatch

The base runtime path uses Core semantics for validation, version resolution, lift, decisioning, effects, and host persistence boundaries.

### 5.2 Process evolution API role

The process evolution API should be responsible for:

- analysis of proposed process behavior
- comparison between baseline and candidate processes
- replay-driven analysis
- explicit migration workflows
- migration diagnostics
- regression reporting
- release-readiness analysis

### 5.3 Why this separation matters

If evolution concerns are mixed directly into the base runtime surface, the normal application API becomes harder to understand.

The user should not feel that every ordinary runtime process handle also carries the full surface of simulation, replay, comparison, batch migration, repair tooling, and release-readiness reporting by default.

At the same time, simulation and replay must remain runtime-equivalent.

That means they model what runtime would do without becoming runtime itself.

---

## 6. Recommended Attachment Model

### 6.1 Central process abstraction remains the anchor

The process abstraction used in the application API should remain the center.

Process evolution capabilities should preferably attach to that abstraction through opt-in extensions.

### 6.2 Why this is preferred

This allows:

- one stable process abstraction
- opt-in growth by capability
- less pressure to invent unrelated new object models
- a cleaner and more natural user experience

### 6.3 Rule

The process evolution API should extend the process abstraction.  
It should not replace it.

---

## 7. Simulation as an Evolution Capability

### 7.1 What simulation is

Simulation is not merely a preview helper and not merely an advanced extra.

Simulation is a distinct analysis capability for understanding how process behavior changes under candidate definitions or historical inputs.

Simulation is a non-mutating, runtime-equivalent diagnostic capability.

It answers:

- what would runtime do?
- what changed?
- where did behavior diverge?
- can existing truth survive this change?
- where does migration, replay, or decision execution fail?
- what category of correction is likely needed?

### 7.2 Primary uses of simulation

Simulation should support:

- scenario-based regression checks
- baseline-versus-candidate comparison
- seamless/diverted/incompatible classification
- historical replay
- migration-readiness analysis
- release-readiness analysis
- diagnostic reporting

### 7.3 What simulation is not

Simulation should not be treated as:

- normal runtime execution
- a casual substitute for `decide`
- a purely CLI-only concern
- an incidental debugging extra
- a mutation mechanism

Simulation never mutates persisted reality.

### 7.4 API consequence

Simulation should not automatically sit in the base runtime surface.

It should be available as an opt-in evolution capability.

---

## 8. Replay as Part of Simulation

### 8.1 Initial position

Historical replay should initially be treated as part of the simulation capability family.

### 8.2 Why this is cleaner

Replay is a specialized use of simulation rather than a wholly separate conceptual family at the current stage.

Keeping replay under simulation:

- avoids premature fragmentation
- keeps release-readiness analysis close to scenario comparison
- makes the capability easier to teach

### 8.3 Replay and migration

Replay may require migration.

Historical truth may originate from older process versions.

Replay therefore follows:

Historical Truth  
→ Core Lift Semantics  
→ Candidate Version  
→ Decision Evaluation  
→ Replay Report

Replay must not skip migration when runtime would require migration.

Replay migration failures are valid replay outcomes.

### 8.4 Future possibility

If replay later grows materially different operational needs, it may be revisited as its own feature family.

That does not need to be decided now.

---

## 9. Migration Workflows as Process Evolution Capabilities

### 9.1 What Process Evolution owns

Process Evolution owns explicit migration workflows.

These include:

- explicit migrate()
- batch migration
- administrative migration
- repair workflows
- pre-release migration checks
- migration diagnostics
- migration reporting
- release-readiness migration analysis

These workflows use Core-owned lift semantics and migration vocabulary.

### 9.2 What Core owns

Core owns canonical migration and lift semantics, including:

- version truth
- LiftStatus
- LiftOutcome
- LiftPath
- LiftStep
- LiftEvent
- SemanticPatch
- MigrationRuleReference
- IncompatibilityReason
- migration status vocabulary
- migration error vocabulary
- migration determinism
- ambiguity rejection
- runtime lift ordering
- migration rule selection semantics

Process Evolution may report these concepts, but it does not define them authoritatively.

### 9.3 Explicit migration operations

Explicit migration operations are evolution-facing workflows.

They may be used for:

- batch upgrades
- data repair
- release preparation
- administrative review
- migration diagnostics
- preflight validation

These operations should be exposed through Process Evolution capabilities rather than forced into the base runtime decision API.

### 9.4 On-access lift

On-access lift is a Core runtime concern.

Process Evolution may analyze, simulate, replay, or report on on-access lift behavior, but it does not own runtime lift semantics.

### 9.5 Multi-version lift

Core owns the semantic model for multi-version lift strategies such as:

- Direct
- Stepwise
- PreferDirectThenStepwise

Process Evolution may expose reporting and tooling around these strategies, especially for migration readiness and release analysis.

---

## 10. Version Crossing Semantics

### 10.1 Version crossing requires a route

Every version crossing requires a declared migration route or a declared compatibility rule.

No implicit version crossing is allowed.

A process instance must not be silently reinterpreted under a different version.

### 10.2 Version crossing does not always transform truth

A version crossing is not automatically a semantic transformation.

A migration route may preserve:

- fields
- state
- context shape
- event meaning

unchanged.

If a declared route exists and no semantic transformation occurs, the Core-owned result is Direct.

If a declared route exists and semantic transformation occurs, the Core-owned result is Lifted.

### 10.3 Default preservation

Default field/state preservation is allowed only when explicitly declared or derived from a declared compatibility rule.

Default preservation must not become implicit migration.

### 10.4 Catch-all migration rules

Catch-all migration rules may exist for ergonomic field preservation and broad compatibility handling.

However, catch-all rules must be:

- declared
- deterministic
- version-scoped
- ordered or prioritized by explicit rule semantics
- unable to hide ambiguity
- explainable in simulation output

A catch-all rule must not silently bypass more specific migration rules.

If a catch-all and a specific rule both match, deterministic selection rules must make the result unambiguous.

If deterministic selection is impossible, the Core-owned outcome is AmbiguousMigrationPath.

---

## 11. Recommended Feature Model

The recommended capability model is:

### Base

- normal runtime application API
- process handle
- required-data inspection
- decision input and outcome
- decision execution
- Core-owned version and lift semantics where needed

### Opt-in evolution capabilities

- simulation
- replay
- comparison
- explicit migration tooling
- diagnostics
- release-readiness analysis

The process evolution API described here should assume that these capabilities may be enabled independently while still feeling coherent.

---

## 12. Recommended API Shape

### 12.1 Preferred design direction

The preferred direction is:

- base process abstraction in the application API
- opt-in extension methods for simulation and replay
- opt-in extension methods for explicit migration tooling
- dedicated evolution result/report types
- dedicated diagnostic/reporting types
- builders where complexity is real

### 12.2 Why this is preferred

This keeps:

- runtime decisioning focused
- evolution capabilities accessible
- release strategy modular
- migration tooling explicit
- future growth cleaner

---

## 13. Simulation Attachment Strategy

### 13.1 Recommended model

Simulation should attach to the process abstraction through an extension capability.

This means the user still works with a process handle, but additional simulation methods only appear when simulation support is enabled.

### 13.2 Good outcomes of this model

This gives:

- one familiar center of gravity
- explicit capability growth
- less API crowding in the base surface
- no need for users to discover a wholly separate object model too early

### 13.3 Suggested entry style

The evolution API should likely provide something conceptually like:

- prepare simulation
- simulate scenario
- compare candidate
- replay historical set
- generate simulation report
- generate release-readiness report

The exact names may evolve, but the key idea is that simulation remains clearly identified as analysis.

---

## 14. Migration Attachment Strategy

### 14.1 Recommended model

Explicit migration tooling should attach through an opt-in evolution capability.

This allows migration workflows to live near simulation and replay without making migration semantics Process Evolution-owned.

### 14.2 Why this matters

Migration tooling is a process evolution concern.

Migration semantics are a Core concern.

Keeping those separate avoids both problems:

- Core does not become a release-readiness/reporting layer
- Process Evolution does not become the authority for runtime migration legality

### 14.3 Rule

Process Evolution uses Core lift semantics.

It does not redefine them.

---

## 15. Evolution Result Types

### 15.1 Why distinct result types matter

The result of process evolution analysis is not the same as a normal runtime decision outcome.

A runtime decision outcome answers:

- what should the host do now?

An evolution result answers:

- what changed?
- how compatible is the candidate?
- where are the divergences?
- what is the release or migration risk?
- what would runtime have done?
- why did the analysis fail?
- what category of correction is likely needed?

These should remain distinct.

### 15.2 Simulation result family

Simulation should likely return dedicated types such as:

- SimulationOutcome
- SimulationReport
- ComparisonResult
- RegressionResult
- ReplayReport
- ReleaseReadinessReport

### 15.3 Migration workflow result family

Explicit migration workflows should likely return dedicated evolution-facing types such as:

- MigrationWorkflowResult
- MigrationDiagnosticReport
- BatchMigrationReport
- RepairWorkflowReport
- MigrationReadinessReport

These may contain Core-owned lift outputs such as LiftOutcome, LiftPath, LiftEvent, SemanticPatch, and migration errors.

The exact type names can evolve later, but the separation from runtime decision outcomes should remain.

---

## 16. Simulation Reporting Layers

### 16.1 Layer 1 — Overall simulation outcome

Layer 1 answers:

What happened overall?

Examples:

- Seamless
- Diverted
- Incompatible

These are useful high-level classifications but are insufficient by themselves.

### 16.2 Layer 2 — Runtime-equivalent details

Layer 2 answers:

What would runtime have done?

May include:

- LiftStatus
- LiftOutcome
- LiftPath
- migration errors
- decision result
- semantic patch summary
- effect changes

These details are imported from Core semantics where applicable.

### 16.3 Layer 3 — Diagnostic classification

Layer 3 answers:

Why did this happen?

Possible diagnostic categories include:

- MissingRoute
- AmbiguousRoute
- MissingRequiredTruth
- TransformFailed
- AnchorHistoryConflict
- DecisionDiverged
- EffectChanged
- StatePathChanged
- DecisionRejectedAfterLift
- NoTransitionAfterLift

### 16.4 Layer 4 — Remediation category

Layer 4 may answer:

What kind of fix is likely needed?

Possible advisory categories include:

- Add Migration Route
- Fix Historical Data
- Add Transform
- Repair Chronicle
- Resolve Ambiguity
- Update Law
- Add Required Truth

Remediation categories are advisory.

They must not replace deterministic diagnostics.

---

## 17. Non-Seamless Explanation Invariant

Simulation must be explanatory, not merely classificatory.

Every non-seamless simulation result should include:

- deterministic reason
- deterministic location
- deterministic diagnostic classification

Simulation should identify:

- what failed
- where it failed
- why it failed
- what category of fix is required

A vague simulation is not sufficient for release readiness.

---

## 18. Migration During Simulation

If runtime would require migration, simulation must also require migration.

Simulation must never:

- bypass migration
- ignore migration failures
- assume latest-version truth
- reinterpret older truth using newer semantics
- fabricate migration behavior

Migration failures encountered during simulation are valid simulation outcomes.

Simulation may surface:

- LiftOutcome
- LiftPath
- migration status
- migration errors
- migration diagnostics
- furthest reachable version
- required migration truth
- incompatibility reasons

These are reported as analytical artifacts.

They are not committed.

---

## 19. Migration During Replay

Replay may require migration.

Historical truth may originate from older process versions.

Replay must follow Core version-transition semantics.

Replay reports may include:

- migration status
- lift path
- failed migration step
- required missing data
- incompatibility reason
- ambiguity diagnostics
- anchor/history conflicts

Replay migration failures are valid replay outcomes.

---

## 20. Scenario Simulation

### 20.1 Purpose

Scenario simulation should support explicit scenario-based analysis.

This is especially useful for:

- unit tests
- regression harnesses
- law change review
- expected-path verification

### 20.2 Why it matters

This allows teams to define process paths intentionally and then test whether a candidate process behaves as expected.

### 20.3 Relationship to the API

Scenario simulation should be one of the simplest evolution entry points.

Scenario simulation should still follow runtime-equivalent semantics.

If the scenario crosses versions, it must use Core migration semantics.

---

## 21. Historical Replay

### 21.1 Purpose

Historical replay should support running production-like or production-derived truth through a candidate process before release.

### 21.2 Why it matters

Historical replay is one of the strongest practical disciplines VPE can enable.

It helps teams stop manually probing production behavior after the fact and instead evaluate process changes against real historical truth before release.

### 21.3 Relationship to the API

Historical replay should feel like a natural evolution mode, not an unrelated subsystem.

Historical replay is especially valuable because it reveals whether existing truth can survive version crossing.

---

## 22. Regression Testing Role

### 22.1 Simulation as test harness

The evolution API should make it natural to use simulation in tests as a regression harness.

### 22.2 What this enables

Teams can define:

- expected seamless paths
- intended divergences
- known incompatible paths
- expected migration success
- expected migration failure
- expected diagnostic categories
- expected remediation categories

Then verify those expectations against process changes.

### 22.3 Why this belongs in the evolution API

This is not normal runtime use and not merely a CLI concern.

It is part of the developer-facing evolution story.

---

## 23. Release Readiness

### 23.1 Purpose

Release readiness analysis should evaluate whether a candidate law is safe to expose to runtime.

It must consider both behavioral compatibility and migration readiness.

### 23.2 Migration readiness

A candidate law may preserve decision behavior but still be unsafe if existing truth cannot lawfully reach the target version.

Release readiness should consider:

- behavioral divergence
- decision incompatibility
- migration completeness
- migration ambiguity
- missing migration rules
- missing required migration truth
- incompatible historical states
- transform failures
- anchor/history conflicts

A release may fail readiness because migration fails, even if decision behavior appears otherwise valid.

### 23.3 Reporting

Release-readiness reports should summarize:

- seamless cases
- diverted cases
- incompatible cases
- migration failure counts
- migration failure categories
- common missing routes
- common missing required truth
- ambiguous route groups
- transform failures
- decision failures after lift
- remediation categories

---

## 24. Non-Mutation Rule

Simulation and replay never mutate persisted reality.

When simulation or replay produces migration outputs, those outputs are analytical artifacts only.

Simulation/replay migration outputs must not be treated as committed lineage.

In simulation/replay:

LiftEvent = analytical lineage  
SemanticPatch = analytical instruction  
Host commit = absent

This differs from runtime/explicit migration, where the host may choose to persist returned outputs.

---

## 25. CLI Relationship

### 25.1 CLI remains important

The CLI should continue to expose process evolution functionality for:

- authoring feedback
- direct experimentation
- debugging
- ad hoc comparison
- simulation reporting
- replay reporting
- migration diagnostics
- release-readiness checks

### 25.2 API relationship

The existence of CLI simulation does not reduce the need for a process evolution API.

The CLI serves human workflows.  
The evolution API serves code-based workflows, tests, and host-side analysis.

### 25.3 Rule

The CLI may expose process evolution capabilities.

It should not be the only practical way to use them.

---

## 26. What the Process Evolution API Should Not Become

The process evolution API should not become:

- the base runtime API
- the owner of Core migration semantics
- a hidden replacement for normal decisioning
- a persistence framework
- an orchestration framework
- a catch-all developer-tools bucket with no conceptual center

Its center should remain clear:

process change analysis, replay, diagnostics, explicit migration workflows, and release-readiness support.

---

## 27. Recommended Immediate Design Decisions

The following decisions are recommended now.

### 27.1 Keep base runtime decisioning separate

Do not place simulation, replay, or explicit migration tooling in the always-on base runtime surface by default.

### 27.2 Import Core migration semantics

Do not define canonical lift semantics in the Process Evolution API.

Reference Core as the authority for version truth, lift outcomes, migration vocabulary, migration errors, and migration determinism.

### 27.3 Attach evolution through extensions

Prefer opt-in extension methods on the central process abstraction.

### 27.4 Keep replay under simulation initially

Avoid premature fragmentation.

### 27.5 Keep distinct evolution result types

Do not reuse ordinary runtime decision outcomes as the primary analysis results.

### 27.6 Treat simulation as runtime-equivalent analysis

Simulation should model what runtime would do without mutating reality.

### 27.7 Require explanatory simulation output

Non-seamless simulation results must explain what failed, where it failed, why it failed, and what category of fix is likely needed.

---

## 28. Open Questions

The following questions remain for later refinement:

- what exact extension trait shapes should be used
- whether builders are the best primary entry point for simulation, replay, and explicit migration workflows
- which replay helpers belong directly in the simulation feature
- whether test harness helpers remain inside simulation or eventually deserve a separate dev/test layer
- how much low-level candidate-process control should be exposed directly to users
- whether remediation categories should become first-class enums or advisory report fields
- how detailed release-readiness reports should be by default
- how catch-all migration rule reporting should be summarized in large migrations

These questions are real, but they do not need to block the architectural direction.

---

## 29. Final Summary

The VPE Process Evolution API should provide a clean, first-class home for simulation, replay, explicit migration workflows, diagnostics, and release-readiness analysis without bloating the base application runtime surface.

The right model is:

- Core API for version-aware truth and lawful migration semantics
- base runtime API for normal decisioning
- Process Evolution API for analysis and tooling workflows
- one central process abstraction extended by capability rather than replaced by unrelated new objects

Core provides the language.

Evolution provides the capabilities.

Runtime remains focused.  
Evolution remains powerful.  
Release and testing workflows remain first-class.

The guiding rule is:

Normal decisioning belongs in the base runtime API.  
Canonical migration semantics belong in Core.  
Process evolution belongs in opt-in evolution capabilities.