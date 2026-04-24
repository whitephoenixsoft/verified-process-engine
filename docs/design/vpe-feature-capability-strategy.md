# VPE Feature and Capability Strategy

Status: Draft  
Scope: Packaging, feature-gating, capability layering, and release strategy for VPE  
Audience: Project architect, core implementers, API designers, future contributors  
Depends On: Core semantic center, engine/runtime boundaries, application API direction, simulation and migration roadmap

---

## 1. Purpose

This document defines the feature and capability strategy for VPE.

It exists to:

- clarify what belongs in the always-on base versus opt-in capabilities
- give the project a coherent packaging and release model
- prevent uncontrolled surface growth in the main API
- ensure simulation, migration, and future capability families land cleanly
- align compiler, engine, and application-facing layers under one strategy
- support long-term extensibility without fragmenting the semantic center

This document is not an implementation plan.  
It is a packaging and capability-boundary decision document.

---

## 2. Core Position

VPE should be organized around:

- one always-on semantic center
- one always-on runtime decisioning path
- optional capability families layered on top through explicit opt-in features

The goal is not to split VPE into many tiny pieces.  
The goal is to keep the core obvious and stable while allowing the platform to grow cleanly.

---

## 3. Governing Principles

### 3.1 Keep the semantic center always available

The semantic center of VPE should not be feature-fragmented.

A user should not need to discover feature flags just to understand what VPE fundamentally is.

### 3.2 Use features for capability families, not random convenience

A feature should represent a meaningful capability layer, such as:

- simulation
- migration

A feature should not exist merely for:
- one-off helpers
- trivial naming differences
- arbitrary convenience scattering

### 3.3 Keep runtime decisioning stable and central

Normal runtime decisioning should remain part of the default/base experience.

A user should be able to use VPE for ordinary application decision execution without enabling optional capability layers.

### 3.4 Keep process evolution distinct from runtime execution

Simulation, replay, and migration are important, but they are not the same as normal runtime decisioning.

They should be treated as process evolution capabilities layered on top of the base.

### 3.5 Features may extend, but not redefine

Optional capabilities may add new methods, types, and workflows.

They may not redefine:
- process legality
- runtime truth
- compilation meaning
- event/effect semantics

### 3.6 Preserve one process abstraction where possible

When optional capabilities are added, they should preferably extend the same central process abstraction rather than forcing users into many unrelated object models.

---

## 4. The Semantic Center

The always-on semantic center of VPE should include:

- law and schema core concepts
- compilation foundation
- runtime foundation
- engine foundation
- deterministic decision semantics
- normal decision execution
- core types and errors
- event/effect meaning
- required-data or manifest semantics needed for normal decisioning

This is the part that makes VPE be VPE.

It should remain stable and obvious.

---

## 5. Base Capability Layer

The base layer should be sufficient for normal application integration.

### 5.1 Base responsibilities

The base layer should support:

- authoring-aligned process registration through engine paths
- compilation as part of normal system operation
- process installation and lookup
- required-data inspection for runtime use
- normal runtime decision execution
- application-facing decision input and outcome types

### 5.2 Base API expectation

A host using only the base layer should be able to:

- make a process available
- look up the process
- inspect what data is required
- decide from explicit supplied truth
- persist and dispatch through host-owned logic

### 5.3 Base does not mean minimal to the point of confusion

The base should be focused, but it must still feel complete for ordinary use.

If users need multiple feature flags just to perform standard decisioning, the packaging model has failed.

---

## 6. Optional Capability Families

Optional features should represent coherent families of work that are valuable but not required for all users.

### 6.1 Simulation feature

The simulation feature should cover process evolution analysis such as:

- scenario comparison
- regression simulation
- candidate-versus-baseline comparison
- historical replay
- release-readiness analysis

Simulation should be first-class, but optional.

### 6.2 Migration feature

The migration feature should cover process version-transition support such as:

- migration planning
- lift or transform semantics
- compatibility handling where defined
- migration-oriented reports or results

Migration should be treated as a sibling capability to simulation under the broader process evolution family.

### 6.3 Replay handling

Historical replay should initially be considered part of simulation unless it grows into a clearly separate capability family later.

This avoids premature fragmentation.

---

## 7. Recommended Initial Feature Model

The recommended initial model is:

### Always-on base
- compiler foundation
- engine foundation
- runtime foundation
- application-facing runtime decisioning surface

### Opt-in features
- `simulation`
- `migration`

This is the cleanest initial layering currently visible.

---

## 8. Compiler and Engine Under the Strategy

### 8.1 Compiler should remain part of the base center

The compiler foundation should not be treated as an optional extra.

Even if some callers interact through engine registration rather than explicit compiler APIs, compilation remains part of the trusted path from authoring to execution.

### 8.2 Engine should remain part of the base center

The engine is part of the normal VPE story.

It should not require an opt-in feature merely to exist as the standard runtime integration mechanism.

### 8.3 Feature-gated additions around compiler or engine are acceptable

If later capability families require additional compiler- or engine-adjacent functionality, those additions may be feature-gated.

But the compiler and engine foundations themselves should remain part of the obvious base.

---

## 9. Application API Under the Strategy

### 9.1 Base app API

The application-facing runtime API should live in the base layer.

This should include the normal decisioning surface, such as:

- process handle or equivalent
- required-data inspection
- decision input
- decision outcome
- decision builder if needed

### 9.2 Why this belongs in base

Normal host decisioning is not optional.

It is the core application-facing use of VPE and should not be gated behind optional capability flags.

### 9.3 What should not be in the base app API

The base app API should not be crowded with process evolution methods by default.

That means simulation and migration should not automatically appear in the runtime surface unless their features are enabled.

---

## 10. Extension Methods and Feature-Gated Growth

### 10.1 Preferred growth pattern

Optional capability families should preferably attach to central types through extension methods or equivalent opt-in expansion mechanisms.

This allows the process abstraction to remain stable while capabilities expand intentionally.

### 10.2 Why this is preferred

This gives users:

- one familiar process abstraction
- opt-in expansion by capability
- less need to learn unrelated helper objects
- clearer release boundaries

### 10.3 Rule

The base process abstraction should remain meaningful on its own.

Extensions should enrich it, not rescue it from incompleteness.

---

## 11. Why Features Are Better Than Premature Crate Explosion

### 11.1 Features reduce early complexity

At the current stage, a single main crate with opt-in features is likely easier to reason about than many small crates.

It reduces:

- package sprawl
- version coordination burden
- documentation complexity
- mental overhead for adopters

### 11.2 Features preserve a coherent product story

Users can understand:

- base VPE for runtime decisioning
- simulation when they need process evolution analysis
- migration when they need version-transition support

This is easier to teach than many small packages too early.

### 11.3 Future crate separation remains possible later

If capability families become significantly larger or more operationally distinct, later crate separation may still be justified.

That decision should be made from actual maturity and need, not premature structure.

---

## 12. Risks This Strategy Avoids

This strategy helps prevent:

### 12.1 Core fragmentation

Users should not have to piece together what “real VPE” means from many optional parts.

### 12.2 Base API bloat

Simulation and migration should not crowd the normal runtime surface by default.

### 12.3 Premature packaging sprawl

The project should not multiply crates or surfaces before their boundaries are fully mature.

### 12.4 Capability ambiguity

Simulation and migration should land as clear capability families, not as scattered helper methods with no stable home.

---

## 13. Risks to Watch Even With Features

Features are useful, but they can still be misused.

### 13.1 Too many small features

If features become overly granular, users will face the same confusion as if the crate were fragmented.

### 13.2 Hidden essential behavior

If the base is made too thin, users may feel forced to discover the “real” product through feature flags.

### 13.3 Semantic divergence

Feature-gated capabilities must still align with the same semantic center and must not redefine core truths.

### 13.4 Extension overload

If too many methods are attached through optional features without a clear capability story, the API may still become hard to navigate.

---

## 14. Recommended Capability Taxonomy

The current taxonomy should be:

### 14.1 Semantic center
Always on.

Includes:
- law/schema core
- compiler foundation
- engine foundation
- runtime foundation
- normal decisioning

### 14.2 Runtime application integration
Always on.

Includes:
- process lookup or handle
- required-data inspection
- decision input and outcome
- normal host-side execution flow

### 14.3 Process evolution
Opt-in.

Initially includes:
- simulation
- replay under simulation
- later migration as a sibling capability

This taxonomy keeps the center stable while giving evolution capabilities room to grow.

---

## 15. Release Strategy Implications

This strategy supports a cleaner release story.

### 15.1 Early releases

Focus on:
- compiler maturity
- engine/runtime maturity
- stable application-facing runtime decisioning

### 15.2 Later releases

Add:
- simulation as an opt-in capability
- migration as an opt-in capability
- future evolution helpers only when their conceptual family is clear

### 15.3 Why this matters

This prevents pressure to force unfinished process evolution features into the base runtime API.

It allows the core runtime story to stabilize first.

---

## 16. Recommended Immediate Decisions

The following decisions are recommended now:

### 16.1 Keep compiler and engine in the base
Do not make the semantic center feel optional.

### 16.2 Keep runtime decisioning in the base
Normal host integration should remain obvious and complete.

### 16.3 Plan simulation as a feature
Treat it as the first process evolution capability family.

### 16.4 Plan migration as a feature
Treat it as the next sibling capability family.

### 16.5 Attach capability growth through extensions
Prefer expanding central types through opt-in extension methods rather than proliferating unrelated new objects.

---

## 17. Future Questions

The following questions should stay open for later refinement:

- whether replay eventually deserves its own feature beyond simulation
- whether test harness helpers belong inside simulation or in a later dev/test feature
- whether crate separation becomes justified later by scale or dependency divergence
- whether certain diagnostics or explainability capabilities should remain base or become optional later

These are important, but they do not need to be settled immediately.

---

## 18. Final Summary

VPE should use one coherent capability strategy across compiler, engine, and application-facing integration.

The semantic center should remain always available and obvious.  
Normal runtime decisioning should remain part of the base experience.  
Process evolution capabilities such as simulation and migration should be added as opt-in feature families.

This gives VPE:

- a stable core
- a cleaner runtime API
- a controlled growth model
- a natural release strategy
- room for simulation and migration to mature without bloating the base surface

The guiding rule is:

Base for core decisioning infrastructure.  
Features for optional process-evolution capabilities.