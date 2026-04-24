# VPE Process Evolution API

Status: Draft  
Scope: Application-facing and developer-facing API for simulation, replay, and migration-oriented process evolution capabilities  
Audience: Project architect, core implementers, API designers, test authors, future contributors  
Depends On: Base runtime/application API, feature strategy, semantic center, simulation and migration roadmap  
Does NOT define: CLI command syntax, compiler internals, base runtime decision API, persistence/orchestration behavior

---

## 1. Purpose

This document defines the intended shape of the VPE process evolution API.

It exists to:

- define how simulation and migration capabilities should relate to the main process abstraction
- keep process evolution distinct from normal runtime decision execution
- provide a clean home for regression testing, historical replay, and release-readiness analysis
- establish how opt-in evolution capabilities should grow without bloating the base application API
- prepare a coherent long-term surface for simulation, replay, and migration

This document is not a workflow guide.  
It is an API and capability-boundary document for process evolution.

---

## 2. Core Position

Process evolution is a first-class capability family in VPE, but it is not the same as normal runtime decisioning.

Normal runtime decisioning answers:

- what should happen now?

Process evolution answers questions like:

- what would happen if this process changes?
- does the candidate process remain seamless?
- where does it diverge?
- which historical cases become incompatible?
- can this instance or history be lifted safely to a new version?

These questions are central to VPE’s value, but they should not overload the base runtime surface.

Therefore:

- runtime decisioning belongs in the base application API
- process evolution belongs in opt-in evolution capabilities

---

## 3. Governing Principles

### 3.1 Keep runtime and evolution distinct

Process evolution should not be collapsed into ordinary runtime decision execution.

The runtime path must remain focused on deciding from current truth.  
The evolution path must remain focused on analysis, comparison, replay, and transition.

### 3.2 Keep evolution close to the process abstraction

Although process evolution is distinct from runtime use, it should still attach naturally to the central process abstraction where possible.

A user should not have to abandon the process abstraction just to simulate or migrate.

### 3.3 Preserve the semantic center

Simulation and migration must use the same semantic center as the rest of VPE.

They may expose analysis and transition behavior.  
They may not invent alternate truth models or alternate meanings for process legality.

### 3.4 Treat evolution as opt-in capability

Simulation, replay, and migration should not crowd the base application API by default.

They should appear through explicit opt-in capability layers.

### 3.5 Support release and testing discipline

The evolution API should make it natural to:

- write regression-style simulation tests
- compare process versions
- replay historical truth before release
- prepare and reason about migration safely

---

## 4. Scope of Process Evolution

The process evolution family should include:

- simulation of candidate behavior
- comparison of current and proposed process versions
- scenario-based regression analysis
- historical replay analysis
- migration-oriented version transition support

These are related enough to live under one broader family, even if they are exposed through separate feature flags internally.

---

## 5. Relationship to the Base Application API

### 5.1 Base application API role

The base application API should remain responsible for:

- process lookup
- required-data inspection for runtime use
- decision input construction
- normal decision execution
- decision outcomes for host persistence and dispatch

### 5.2 Process evolution API role

The process evolution API should be responsible for:

- analysis of proposed process behavior
- comparison between baseline and candidate processes
- replay-driven analysis
- migration-oriented transition support

### 5.3 Why this separation matters

If evolution concerns are mixed directly into the base runtime surface, the normal application API becomes harder to understand.

The user should not feel that every ordinary runtime process handle also carries the full surface of analysis, replay, and migration by default.

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

### 7.2 Primary uses of simulation

Simulation should support:

- scenario-based regression checks
- baseline-versus-candidate comparison
- seamless/diverted/incompatible classification
- historical replay
- release-readiness analysis

### 7.3 What simulation is not

Simulation should not be treated as:

- normal runtime execution
- a casual substitute for `decide`
- a purely CLI-only concern
- an incidental debugging extra

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

### 8.3 Future possibility

If replay later grows materially different operational needs, it may be revisited as its own feature family.

That does not need to be decided now.

---

## 9. Migration as an Evolution Capability

### 9.1 What migration is

Migration is the response to process evolution when version transition must be handled explicitly.

If simulation helps teams understand what changes, migration helps define how certain changes are handled safely.

### 9.2 Why migration belongs near simulation

Simulation and migration are related because they both address process evolution.

Simulation asks:
- what changes?

Migration asks:
- how do we move safely when change is accepted?

### 9.3 API consequence

Migration should be treated as a sibling capability to simulation within the process evolution family.

It should not be forced into the base runtime decision API.

---

## 10. Recommended Feature Model

The recommended capability model is:

### Base
- normal runtime application API
- process handle
- required-data inspection
- decision input and outcome
- decision execution

### Opt-in evolution capabilities
- `simulation`
- `migration`

The process evolution API described here should assume that these capabilities may be enabled independently while still feeling coherent.

---

## 11. Recommended API Shape

### 11.1 Preferred design direction

The preferred direction is:

- base process abstraction in the application API
- opt-in extension methods for simulation and later migration
- dedicated evolution result/report types
- builders where complexity is real

### 11.2 Why this is preferred

This keeps:

- runtime decisioning focused
- evolution capabilities accessible
- release strategy modular
- future growth cleaner

---

## 12. Simulation Attachment Strategy

### 12.1 Recommended model

Simulation should attach to the process abstraction through an extension capability.

This means the user still works with a process handle, but additional simulation methods only appear when simulation support is enabled.

### 12.2 Good outcomes of this model

This gives:

- one familiar center of gravity
- explicit capability growth
- less API crowding in the base surface
- no need for users to discover a wholly separate object model too early

### 12.3 Suggested entry style

The evolution API should likely provide something conceptually like:

- prepare simulation
- simulate scenario
- compare candidate
- replay historical set

The exact names may evolve, but the key idea is that simulation remains clearly identified as analysis.

---

## 13. Migration Attachment Strategy

### 13.1 Recommended model

Migration should also attach through an opt-in extension capability.

This allows migration to sit beside simulation cleanly as another process evolution mode.

### 13.2 Why this matters

If simulation is an extension but migration later lands elsewhere, the evolution family will feel inconsistent.

Keeping both as parallel capability layers is cleaner.

---

## 14. Evolution Result Types

### 14.1 Why distinct result types matter

The result of process evolution analysis is not the same as a normal runtime decision outcome.

A runtime decision outcome answers:
- what should the host do now?

An evolution result answers:
- what changed?
- how compatible is the candidate?
- where are the divergences?
- what is the release or migration risk?

These should remain distinct.

### 14.2 Simulation result family

Simulation should likely return dedicated types such as:

- simulation outcome
- simulation report
- comparison result
- replay report

### 14.3 Migration result family

Migration should likely return dedicated types such as:

- migration result
- lift result
- compatibility report
- migration plan or summary where appropriate

The exact type names can evolve later, but the separation from runtime decision outcomes should remain.

---

## 15. Scenario Simulation

### 15.1 Purpose

Scenario simulation should support explicit scenario-based analysis.

This is especially useful for:

- unit tests
- regression harnesses
- law change review
- expected-path verification

### 15.2 Why it matters

This allows teams to define process paths intentionally and then test whether a candidate process behaves as expected.

### 15.3 Relationship to the API

Scenario simulation should be one of the simplest evolution entry points.

---

## 16. Historical Replay

### 16.1 Purpose

Historical replay should support running production-like or production-derived truth through a candidate process before release.

### 16.2 Why it matters

Historical replay is one of the strongest practical disciplines VPE can enable.

It helps teams stop manually probing production behavior after the fact and instead evaluate process changes against real historical truth before release.

### 16.3 Relationship to the API

Historical replay should feel like a natural evolution mode, not an unrelated subsystem.

---

## 17. Regression Testing Role

### 17.1 Simulation as test harness

The evolution API should make it natural to use simulation in tests as a regression harness.

### 17.2 What this enables

Teams can define:

- expected seamless paths
- intended divergences
- known incompatible paths

Then verify those expectations against process changes.

### 17.3 Why this belongs in the evolution API

This is not normal runtime use and not merely a CLI concern.

It is part of the developer-facing evolution story.

---

## 18. CLI Relationship

### 18.1 CLI remains important

The CLI should continue to expose simulation-related functionality for:

- authoring feedback
- direct experimentation
- debugging
- ad hoc comparison

### 18.2 API relationship

The existence of CLI simulation does not reduce the need for a process evolution API.

The CLI serves human workflows.  
The evolution API serves code-based workflows, tests, and host-side analysis.

### 18.3 Rule

The CLI may expose process evolution capabilities.  
It should not be the only practical way to use them.

---

## 19. What the Process Evolution API Should Not Become

The process evolution API should not become:

- the base runtime API
- a hidden replacement for normal decisioning
- a persistence framework
- an orchestration framework
- a catch-all developer-tools bucket with no conceptual center

Its center should remain clear:

process change analysis and version transition support.

---

## 20. Recommended Immediate Design Decisions

The following decisions are recommended now:

### 20.1 Keep base runtime decisioning separate
Do not place simulation and migration in the always-on base runtime surface by default.

### 20.2 Attach evolution through extensions
Prefer opt-in extension methods on the central process abstraction.

### 20.3 Keep replay under simulation initially
Avoid premature fragmentation.

### 20.4 Keep distinct evolution result types
Do not reuse ordinary runtime decision outcomes as the primary analysis results.

### 20.5 Treat migration as the sibling of simulation
Design the family now so later migration addition feels natural.

---

## 21. Open Questions

The following questions remain for later refinement:

- what exact extension trait shapes should be used
- whether builders are the best primary entry point for simulation and migration
- which replay helpers belong directly in the simulation feature
- whether test harness helpers remain inside simulation or eventually deserve a separate dev/test layer
- how much low-level candidate-process control should be exposed directly to users

These questions are real, but they do not need to block the architectural direction.

---

## 22. Final Summary

The VPE process evolution API should provide a clean, first-class home for simulation, replay, and migration-oriented capabilities without bloating the base application runtime surface.

The right model is:

- base runtime API for normal decisioning
- opt-in evolution capabilities for process change analysis and version transition support
- one central process abstraction extended by capability rather than replaced by unrelated new objects

This gives VPE a cleaner long-term shape.

Runtime remains focused.  
Evolution remains powerful.  
Release and testing workflows remain first-class.  
Future migration support gains a natural place to live.

The guiding rule is:

Normal decisioning belongs in the base runtime API.  
Process evolution belongs in opt-in evolution capabilities.