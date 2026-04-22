# VPE Evolution Strategy

Status: Draft  
Scope: Product direction, API evolution, language/tooling evolution, composition model, concurrency model, adoption strategy  
Audience: Project architect, core implementers, future contributors, advanced adopters

---

## 1. Purpose

This document defines the intended evolution path for VPE.

It exists to:

- clarify what VPE is becoming beyond the current implementation phase
- separate the strict semantic core from the outer product layers
- guide API evolution without corrupting the engine’s philosophy
- establish how authoring, tooling, composition, and concurrency should mature over time
- identify the long-term role of the CLI, code generation, and process composition
- preserve architectural discipline while making the platform easier to adopt
- sharpen VPE’s product identity as the process brain of an application

This document does not define compiler internals, runtime algorithms, or CLI command syntax in detail. It defines the strategic direction that should govern those choices.

---

## 2. Core Thesis

VPE should evolve as four coordinated layers:

1. a strict deterministic execution core
2. a friendly application-facing programming surface
3. a language and tooling ecosystem around canonical process definitions
4. a composable process system in which multiple processes can cooperate through explicit handoff while the host retains orchestration ownership

These layers must evolve in that order of authority.

The core remains the semantic source of truth.  
The outer layers exist to improve authoring, integration, adoption, and advanced usage.  
No outer layer may redefine execution semantics.

---

## 3. Product Identity

### 3.1 Primary identity

VPE is the process brain of an application.

It exists to centralize lawful process reasoning so applications do not scatter transition logic, business conditions, and correctness-sensitive branching across controllers, services, jobs, handlers, scripts, and integrations.

Its value is that it saves development time while improving structural clarity. Instead of rebuilding process reasoning ad hoc in many places, the host delegates that reasoning to a compiled, deterministic, inspectable core.

### 3.2 What this identity implies

If VPE is the process brain, users will expect it to provide:

- authority over process reasoning
- consistency across host codepaths
- explainable legal outcomes
- replayable and inspectable decisions
- safe host integration under concurrency
- enough ergonomics to feel practical, not academic

### 3.3 What this identity does not imply

VPE is not the whole application. It is not:

- the database
- the orchestration engine
- the workflow runner
- the side-effect dispatcher
- the network transport layer
- the distributed consistency mechanism
- the UI or application shell

A useful formulation is:

VPE is the process brain, not the storage, nerves, muscles, or transport of the application.

### 3.4 Category stance

VPE should not be described merely as:

- a rules engine
- a workflow engine
- a state machine library
- a policy evaluator
- an event-sourcing helper

It should establish a pattern centered on verified process logic: compiled, deterministic, host-integrated, history-aware decision systems that separate legal transition logic from orchestration and effect execution.

---

## 4. Project Intent

VPE serves more than one purpose.

### 4.1 Engineering discipline

VPE should teach developers to think like engineers by making system logic:

- explicit
- typed
- validated
- deterministic
- inspectable
- replayable
- bounded by clear responsibilities

This is not merely educational messaging. The API and architecture should actively reinforce these habits.

### 4.2 A new system design pattern

VPE is intended to leave behind a recognizable pattern for a new class of system.

That pattern centers on the idea that applications should have a dedicated, verifiable process brain rather than scattered process conditions embedded everywhere in host code.

### 4.3 Ease of adoption

VPE must become easier to use without becoming less truthful.

Ease of use does not mean hiding truth, inferring hidden state, or collapsing architectural boundaries. It means reducing ceremony, improving naming, providing guided surfaces, and creating approachable authoring and integration paths.

---

## 5. Canonical Architectural Position

The canonical semantic center of VPE is:

- declarative law
- typed schema
- compile-time validation
- immutable compiled process artifacts
- explicit host-supplied inputs and chronicle
- deterministic single-process evaluation
- explicit verdicts containing state transition results, emitted events, and effect intent
- host-owned persistence, orchestration, and side-effect dispatch

Everything else in the ecosystem must lower into or align with this model.

---

## 6. Evolution Principles

### 6.1 Preserve one semantic center

The Rust core remains the authoritative semantics implementation.  
The CLI, wrappers, generators, and future language layers must not create competing semantics.

### 6.2 Simplify by staging complexity, not by hiding truth

Convenience layers may reduce setup burden and improve readability, but they may not:

- perform hidden I/O
- infer missing truth from ambient state
- silently mutate stores
- execute side effects as if they were guaranteed outcomes
- bypass explicit chronicle or state integrity requirements

### 6.3 Keep authoring plural, execution singular

VPE may eventually support multiple authoring paths, but all executable artifacts must reduce to canonical law and schema.

### 6.4 Keep orchestration outside the engine

Even when multiple processes cooperate, the host remains responsible for orchestration. VPE evaluates legality and produces handoff intent or outputs. It does not become a workflow runner.

### 6.5 Use tooling to increase professionalism

The CLI and related tooling should make VPE feel like a serious engineering platform: compilable, inspectable, diagnosable, automatable, and extensible.

### 6.6 Design for learners and power users separately

New users need approachable defaults and successful first-use paths.  
Power users need deeper tooling, extensibility, diagnostics, and composability.  
These should be supported intentionally rather than blended into one confusing surface.

### 6.7 Treat concurrency as a first-class concern

Because VPE aspires to be the process brain of an application, it must have a clear correctness story under contention. Concurrency safety cannot remain an accidental host concern or an implied implementation detail.

### 6.8 Treat performance boundaries honestly

Convenient integration formats are acceptable early, including JSON-based FFI, but they must not be mistaken for permanently cost-free choices. Performance boundaries should be measured, documented, and evolved deliberately.

---

## 7. Evolution Tracks

The platform should evolve along eight coordinated tracks.

### 7.1 Engine evolution

The engine track governs the semantic core.

Its responsibilities include:

- compiler maturity
- schema and law validation integrity
- guard architecture maturity
- manifest correctness
- deterministic runtime execution
- migration semantics
- simulation semantics
- event/effect integrity
- inter-process handoff primitives

The engine track has the highest authority.  
No other track may override or dilute it.

### 7.2 API evolution

The API track governs how developers integrate VPE programmatically.

Its responsibilities include:

- the canonical core Rust API
- a simpler application-facing API
- builders for advanced configuration and workflows
- stable type naming and ergonomic improvements
- host integration helpers and patterns
- audience-specific examples

### 7.3 Language evolution

The language track governs how people define processes.

Its responsibilities include:

- canonical law and schema formats
- diagnostics and source mapping
- linting and authoring feedback
- optional shorthand or DSL authoring layers
- code generation and scaffolding
- compatibility and versioning promises for authoring artifacts

### 7.4 CLI evolution

The CLI track governs VPE as a professional toolchain.

Its responsibilities include:

- validation and compilation workflows
- inspection and manifest rendering
- execution and debugging workflows
- simulation and migration workflows
- packaging and automation support
- extensibility for power users
- scaffolding and generation commands

### 7.5 Composition evolution

The composition track governs cooperation among multiple processes.

Its responsibilities include:

- handoff primitives
- handoff contracts
- process-to-process input mapping
- compositional validation patterns
- federation patterns while preserving host ownership
- constraints that prevent VPE from collapsing into a workflow engine

### 7.6 Concurrency evolution

The concurrency track governs correctness when multiple hosts or workers interact with the same process instances.

Its responsibilities include:

- anchor/version-based concurrency contracts
- stale verdict detection
- atomic persistence expectations
- re-evaluation rules after contention
- host integration guidance for compare-and-swap style commits
- idempotency patterns where helpful
- documentation of correctness boundaries

### 7.7 Performance and embedding evolution

The performance track governs the operational viability of VPE across Rust and non-Rust hosts.

Its responsibilities include:

- FFI boundary design
- serialization cost awareness
- payload size management
- batched or optimized request patterns where justified
- compiled artifact reuse
- performance profiling and measurement discipline
- guidance for hot-path vs non-hot-path use

### 7.8 Adoption evolution

The adoption track governs how the platform becomes understandable and usable.

Its responsibilities include:

- README and onboarding flow
- tutorial sequencing
- starter templates
- example hosts
- reference implementations
- architectural explanation documents
- educational materials that teach the design pattern VPE represents

---

## 8. API Evolution Model

VPE should support three intentionally different programming surfaces.

### 8.1 Layer 1 — Core API

Audience:

- engine integrators
- advanced Rust users
- systems architects
- FFI and embedding authors

Purpose:

- expose the true model faithfully
- preserve full explicitness
- remain the semantic source of truth for all wrapper layers

Characteristics:

- explicit requests
- explicit chronicle/state inputs
- explicit manifests
- explicit verdicts
- explicit separation of evaluation from persistence and dispatch

This layer is allowed to feel strict and somewhat engine-native.  
Its job is truth, not first-use comfort.

### 8.2 Layer 2 — Simple API

Audience:

- ordinary backend developers
- first-time adopters
- teams embedding VPE into applications

Purpose:

- make the common path feel natural
- reduce ceremony for normal host integration
- preserve semantics while improving approachability

This layer should answer common questions quickly:

- How do I load or hold a process?
- How do I ask whether an action is legal?
- How do I execute a turn?
- What do I persist?
- What effects should I dispatch?

This layer should expose a smaller vocabulary and give a more application-shaped view of execution.

### 8.3 Layer 3 — Builders

Audience:

- advanced integrators
- hosts with custom setup rules
- simulation, migration, and composition tooling

Purpose:

- support explicit construction for complex workflows
- keep the simple path simple by moving advanced configuration elsewhere

Builders should be used for:

- compile configuration
- runtime request construction
- simulation inputs
- migration plans
- handoff definitions
- CLI customization or advanced integration hooks

### 8.4 Design rule

Simple things must be simple.  
Complex things may require builders.  
Nothing important should require a builder if it is part of the normal first-use path.

---

## 9. Authoring Evolution Model

VPE should support multiple authoring paths while preserving one canonical executable representation.

### 9.1 Canonical law and schema

The canonical law and schema formats remain the authoritative representation.

They must be:

- deterministic
- fully expressive for supported features
- versioned
- machine-validatable
- suitable for stable compilation and long-term tooling support

All alternate authoring modes must lower into canonical law and schema.

### 9.2 Friendly law authoring

A future human-friendly authoring form may be added if the canonical model is sufficiently stable.

A friendly authoring layer should be:

- more readable than raw machine-centric structures
- easy to diff and review in source control
- unambiguous
- suitable for precise diagnostics
- clearly compilable into canonical law

This should remain declarative.  
It must not become an imperative scripting language.

### 9.3 Schema generation

Schema generation is a strong candidate for early ergonomics improvement.

Possible sources include:

- Rust structs
- JSON models or examples
- JSON Schema
- typed domain definitions in host applications

Generated schemas should be treated as drafts or projections that are still inspectable and owned by the developer. They must compile to canonical schema artifacts.

### 9.4 Law scaffolding and templates

Before full law generation, VPE should support scaffolded patterns and starter templates.

Examples:

- approval workflow
- manual review flow
- ticket lifecycle
- retry/recovery flow
- document publication flow
- payment authorization or review flow

Scaffolding accelerates adoption without pretending business logic can be inferred automatically.

### 9.5 Law generation from higher-level sources

Generation from external models or user interfaces may eventually be supported, but it must be treated carefully.

Most domain models do not contain enough behavioral semantics to infer lawful transitions correctly. Therefore:

- generation may create a starting point
- generated output must remain inspectable and editable
- the engine may not depend on hidden generator assumptions
- generated law is not higher authority than canonical law

### 9.6 Long-term authoring stance

The long-term message should be:

VPE has one canonical executable representation, but multiple authoring paths.

---

## 10. CLI Evolution Model

The CLI should evolve from a thin consumer into a powerful professional toolchain without becoming a competing semantics engine.

### 10.1 Beginner value

For beginners, the CLI should:

- validate artifacts
- compile artifacts
- inspect manifests and requirements
- provide approachable execution examples
- scaffold starter projects or process patterns
- explain errors clearly

### 10.2 Professional value

For power users, the CLI should become a major selling point.

It should support:

- scripting and CI integration
- deep diagnostics
- artifact inspection
- simulation and migration workflows
- generation and scaffolding
- packaging and export paths
- extensibility for custom commands or integrations

### 10.3 CLI extensibility

CLI extensibility is desirable, but its authority must be constrained.

The CLI may be extended to:

- add commands
- add output renderers
- add domain integrations
- add scaffolders or generators
- add packaging and reporting workflows

The CLI may not be extended to redefine:

- compilation semantics
- runtime transition legality
- guard truth
- event/effect meaning

The Rust core remains the source of semantic truth.

### 10.4 Language identity

Because validation, compilation, diagnostics, and inspection already exist or are planned, VPE increasingly behaves like a language ecosystem. The CLI should embrace this reality carefully.

The CLI is not only a dev tool.  
It is the primary shell through which the VPE language identity will become visible.

---

## 11. Composition Evolution Model

VPE should eventually support cooperation among multiple processes, but in a way that preserves its architectural boundaries.

### 11.1 Narrow definition of handoff

A process handoff means:

- one process reaches a state or emits an outcome that is intended for another process
- the handoff is explicit and structured
- the host decides whether and how to initiate the receiving process
- each receiving process still executes through its own explicit request/evaluation cycle

### 11.2 What handoff must not mean

Handoff must not imply that VPE becomes:

- a hidden workflow orchestrator
- a distributed transaction coordinator
- an all-in-one process runtime
- a replacement for the host’s orchestration layer

### 11.3 Handoff primitives

Future composition support may include:

- handoff intents emitted from verdicts
- typed handoff payloads
- source-to-target process mapping metadata
- handoff contracts
- validation of required receiving inputs
- composition manifests for multi-process ecosystems

### 11.4 Compositional stance

VPE should support federated process composition while remaining a single-process evaluator at execution time.

That preserves determinism, architectural clarity, and host-owned orchestration.

---

## 12. Concurrency Evolution Model

Because VPE may be used by multiple hosts or workers against the same process instances, concurrency must be explicitly modeled.

### 12.1 Core concurrency stance

VPE itself evaluates a decision against an explicit snapshot of truth.  
It does not guarantee that this snapshot remains current after evaluation.  
The host must enforce atomic commit rules.

### 12.2 Anchor-based validity

A verdict is valid only relative to the anchor, current state, and chronicle assumptions supplied in the request.

If another host advances the instance before the verdict is committed, the verdict becomes stale.

### 12.3 Host responsibility

The host must atomically persist the result only if the anchor or equivalent version marker is still current.

If the anchor has changed, the host must reject the commit and re-evaluate from fresh truth.

### 12.4 Recommended model

The recommended concurrency model is optimistic concurrency with compare-and-swap style commit protection.

This should be treated as a first-class integration contract, not a hidden operational detail.

### 12.5 Why this matters

Without a clear concurrency contract:

- multiple hosts may compute conflicting valid verdicts
- adopters may incorrectly assume evaluation alone guarantees correctness
- non-atomic persistence may produce race anomalies
- trust in VPE as the process brain will weaken

### 12.6 Future guidance

VPE should eventually provide:

- explicit documentation of stale verdict handling
- host integration patterns for optimistic concurrency
- guidance on idempotency where helpful
- examples for event-sourced and CRUD-style commit patterns

---

## 13. Performance and Embedding Evolution Model

VPE must remain practical both as a Rust-native library and as an embedded engine across host environments.

### 13.1 Early FFI stance

JSON-based FFI is acceptable early because it provides:

- debuggability
- simplicity
- language neutrality
- easier testing
- safer initial boundaries

### 13.2 Performance risk

JSON-based FFI may introduce costs in:

- serialization and deserialization
- allocation churn
- payload copying
- chronicle transfer overhead
- latency in high-frequency execution paths

These costs are not necessarily unacceptable, but they must be acknowledged.

### 13.3 Strategic stance

The project should not prematurely optimize the FFI boundary.  
It should also not assume that the initial FFI form defines the permanent performance ceiling.

### 13.4 Likely future options

If measurement justifies it, later improvements may include:

- batched execution APIs
- artifact reuse patterns
- more compact serialization formats
- reduced-copy host integrations where feasibl