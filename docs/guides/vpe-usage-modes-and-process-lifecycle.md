# Using VPE Across the Process Lifecycle

Status: Draft  
Scope: Developer and team workflow for using VPE from authoring through release and runtime  
Audience: Developers, architects, test authors, platform teams, future contributors

---

## 1. Purpose

This document explains how VPE is used across the full lifecycle.

It exists to:

- show where the CLI, engine API, app API, and simulation fit
- explain how developers move from drafting a process to running it safely
- teach simulation as both a testing discipline and a release-readiness discipline
- reinforce the core boundary that VPE decides and the host does

This is not only an API document. It is a workflow document.

---

## 2. Core Mental Model

### 2.1 What VPE is

VPE is the process brain of an application.

It centralizes lawful process reasoning so business transition logic does not remain scattered across controllers, services, jobs, workers, scripts, and integrations.

### 2.2 What VPE is not

VPE is not:

- the database
- the workflow runner
- the side-effect dispatcher
- the orchestration engine
- the host application itself

### 2.3 Core boundary

VPE decides. The host does.

VPE determines lawful transitions from explicit supplied truth. The host gathers data, persists results, dispatches effects, and controls execution in the world.

### 2.4 Why teams use it

Teams use VPE to solve the scattered business-rule problem.

The main pain is not merely having many rules. The main pain is lacking a structural center for lawful transitions. VPE provides that center.

---

## 3. The Main Surfaces

### 3.1 CLI

The CLI is the first operational shell many developers will use.

It is used for:

- validate
- compile
- inspect
- debug
- execute explicit requests
- simulate process changes

### 3.2 Engine API

The engine API is used for:

- registering schema and law
- compiling directly or through engine-driven registration flows
- installing or managing compiled processes
- lower-level integration
- platform and component-level control

### 3.3 Application API

The application API is used for:

- looking up installed processes
- inspecting required data
- building decision inputs
- deciding from host code
- receiving outcomes for persistence and effect dispatch

### 3.4 Simulation capabilities

Simulation is used for:

- regression testing
- process evolution analysis
- change comparison
- historical replay
- release-readiness evaluation

Simulation should be treated as a first-class developer and team workflow, not merely an advanced extra.

---

## 4. Lifecycle Overview

The VPE lifecycle is:

1. Author
2. Validate
3. Compile
4. Inspect and debug
5. Integrate
6. Test
7. Simulate changes
8. Replay historical truth
9. Release
10. Run

These steps do not always occur in a perfectly rigid order, but they describe the intended workflow clearly.

---

## 5. Stage 1 — Author the Process

### 5.1 Write the schema

The schema defines the process data model.

It establishes:

- what fields exist
- what types they have
- what the law may reference safely

### 5.2 Write the law

The law defines the process behavior.

It expresses:

- states
- transitions
- guards
- effects
- other lawful process rules supported by the language

### 5.3 Authoring mindset

The goal is to move lawful process reasoning into one authoritative place.

Instead of allowing host code to decide transitions in scattered conditionals, the developer drafts the process declaratively and explicitly.

---

## 6. Stage 2 — Validate the Draft

### 6.1 Use CLI validate

The validation step checks that the law and schema are structurally sound.

It catches issues such as:

- missing references
- invalid paths
- malformed structures
- invalid guard use
- other authoring mistakes that should be discovered before integration

### 6.2 Why validation matters

Validation lets developers iterate safely.

It provides quick feedback while the process is still a draft, before the process is integrated into host code or broader workflows.

### 6.3 Intended developer experience

A developer should be able to draft, validate, correct, and validate again without needing to first embed the process into an application.

---

## 7. Stage 3 — Compile the Process

### 7.1 Use CLI compile

Compilation turns the draft into an executable process artifact.

This confirms that the process is not only structurally valid, but executable within VPE’s semantics.

### 7.2 Why compile matters

Compilation is the transition from process draft to process artifact.

It enables:

- execution
- manifests or required-data reporting
- simulation
- artifact inspection
- host integration

### 7.3 Relationship to the engine API

Some developers or hosts will compile explicitly.

Others may register schema and law through the engine API and let the engine handle compilation internally.

Both paths are valid.  
The important thing is that compilation remains part of the trusted path from authoring to use.

---

## 8. Stage 4 — Inspect and Debug the Draft

### 8.1 Use CLI diagnostics

After validation and compilation, the CLI can help developers inspect what they created.

This includes:

- diagnostics
- process shape
- required data or manifests
- compile-time feedback

### 8.2 Use CLI execute

CLI execute is useful for direct experimentation.

A developer may supply an explicit request to see what the process would do for a given state, action, context, and history.

This is especially useful when:

- learning the process behavior
- debugging a draft
- investigating why a transition does or does not happen
- quickly checking an assumption without writing host code first

### 8.3 Use CLI simulate

CLI simulate becomes useful when the developer is comparing process changes.

It helps reveal:

- seamless behavior
- diverted behavior
- incompatible states

### 8.4 Why this stage matters

This stage gives VPE an interpreter-like developer experience without giving up its compiled and deterministic model.

It lets developers think with the process before embedding it everywhere.

---

## 9. Stage 5 — Integrate the Process in the Host

### 9.1 Engine/API setup

At integration time, the host makes the process available through the engine layer.

Depending on the architecture, this may involve:

- explicit compilation followed by installation
- registration of schema and law through the engine
- process lookup by stable process reference

### 9.2 Application API usage

Once the process is available, normal host code should use the application-facing path.

The normal application path is:

- look up the process
- inspect what data is required if needed
- build a decision input
- decide
- receive a decision outcome

### 9.3 Host responsibilities

The host remains responsible for:

- gathering context and history
- passing explicit truth into VPE
- persisting results atomically
- dispatching effects if appropriate
- managing retries and orchestration

### 9.4 Boundary reminder

This boundary must remain visible:

VPE decides. The host does.

---

## 10. Stage 6 — Build Regression Tests

### 10.1 Use simulation as a harness

Simulation should be used in tests as a disciplined regression harness.

A team can define expected scenarios and process paths, then test how a proposed process change behaves against them.

### 10.2 What to test

Simulation-based regression tests should help answer:

- which paths remain seamless
- which paths intentionally diverge
- which states become incompatible unexpectedly

### 10.3 Why this matters

This changes process evolution from guesswork into evidence.

It allows teams to write explicit tests around process-law changes instead of manually probing runtime behavior after a change is deployed.

### 10.4 Practical value

A simulation harness unit test per process path is a strong discipline.

It forces developers to think deliberately about expected stability and expected change.

---

## 11. Stage 7 — Simulate Process Changes During Development

### 11.1 Compare old and new versions

A developer updating a process should be able to compare the current process and the proposed process before release.

### 11.2 Categories of result

The important categories are:

- seamless
- diverted
- incompatible

These categories make process evolution explainable.

### 11.3 Development loop

The intended loop is:

- draft a process change
- validate and compile
- simulate against expected scenarios
- inspect divergence
- revise the process
- simulate again

This loop makes process evolution iterative and measurable.

---

## 12. Stage 8 — Replay Historical Truth Before Release

### 12.1 Historical replay simulation

This is a distinct and important path.

The team should be able to take historical production truth, or production-like truth, and run it through the proposed process version before release.

### 12.2 Why this is distinct from normal testing

This is not merely ordinary unit testing or generic integration testing.

It is a release-preparation discipline.

It helps teams stop thinking:

- let’s release and manually inspect production
- let’s poke around a few records by hand

and start thinking:

- let’s evaluate the change against historical truth ahead of release

### 12.3 What teams learn

Historical replay can reveal:

- expected success rate
- divergence rate
- incompatible cases
- missed assumptions
- release risk concentration by path or state

### 12.4 Why this matters strategically

This may be one of the strongest value stories in VPE.

Many systems can express rules. Fewer systems help teams safely evolve process logic against real historical truth before release.

---

## 13. Stage 9 — Release With Confidence

### 13.1 Evidence-based release decisions

Simulation results should help guide release decisions.

Teams can use them to decide:

- whether a process change is ready
- whether more revision is needed
- whether certain incompatible cases must be addressed first

### 13.2 What changes culturally

VPE encourages a change in mindset.

Instead of treating process changes as risky host-code modifications that are only partly understood, teams can treat process changes as analyzable artifacts whose effects can be measured before release.

---

## 14. Stage 10 — Run in Production

### 14.1 Normal runtime path

Once released, the normal production path is straightforward:

- the host retrieves the installed process
- the host gathers current truth
- the host decides
- the host persists and dispatches

### 14.2 Ongoing value

The ongoing value is not only runtime execution.  
It is the fact that future process evolution now has a cleaner foundation.

The application gains:

- centralized lawful reasoning
- less scattered host logic
- clearer process authority
- safer future changes through simulation and replay

---

## 15. Choosing the Right Surface

### 15.1 Use the CLI when

Use the CLI when you are:

- authoring
- validating
- compiling
- inspecting
- debugging
- experimenting quickly with requests
- simulating process changes manually

### 15.2 Use the Engine API when

Use the engine API when you are:

- registering or installing processes
- integrating VPE at platform level
- managing lower-level component behavior
- working directly with engine-oriented infrastructure

### 15.3 Use the Application API when

Use the application API when you are:

- making normal runtime decisions in app code
- writing service logic, handlers, or workers
- integrating process decisions into application flows

### 15.4 Use simulation when

Use simulation when you are:

- writing regression tests
- evaluating law changes
- comparing versions
- replaying production truth
- assessing release readiness

---

## 16. Recommended Team Workflow

### 16.1 Individual developer workflow

A normal developer workflow may look like:

- author the process
- validate it
- compile it
- inspect and debug it
- integrate it
- test it
- simulate changes

### 16.2 Team workflow

A stronger team workflow may look like:

- review law and schema changes
- validate and compile consistently
- simulate against expected paths
- replay against historical truth
- release based on evidence, not guesswork

### 16.3 Long-term payoff

This gives teams:

- less scattered business logic
- clearer process ownership
- safer change discipline
- more confidence in release decisions

---

## 17. Common Misunderstandings

### 17.1 “The CLI is optional tooling only”

No.  
The CLI is a major part of the authoring, debugging, and experimentation workflow.

### 17.2 “Simulation is only an advanced feature”

No.  
Simulation is both a testing discipline and a release-readiness discipline.

### 17.3 “The app API is just for beginners”

No.  
The app API should be the preferred application-facing surface for normal decision execution, even in sophisticated hosts.

### 17.4 “If VPE decides, it also executes”

No.  
VPE decides. The host does.

### 17.5 “Production replay is just integration testing”

Not exactly.  
It is a distinct practice focused on validating process changes against historical truth before release.

---

## 18. Final Summary

VPE is used across a lifecycle, not just at runtime.

The CLI is the front door for drafting, validating, compiling, debugging, and experimentation.  
The engine API supports installation and lower-level integration.  
The application API supports normal host-side decision execution.  
Simulation supports both regression testing and release-readiness analysis.  
Historical replay is one of the strongest mindset shifts VPE enables.

The central rule remains:

VPE decides. The host does.