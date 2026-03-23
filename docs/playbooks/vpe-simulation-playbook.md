# VPE Human Simulation Playbook
Version: Canonical v1.1

## Purpose

Guide humans in evaluating law changes using simulation.

Simulation answers:

Would historical records behave correctly under the new law?

It is a:
- design-time tool
- risk assessment tool
- debugging tool

Simulation does NOT:
- mutate data
- execute effects
- change history

---

## Core Mental Model

Simulation replays **events as truth** against a **new law**.

At each step:
- state is derived from prior events
- context is evaluated
- a decision is re-computed

You are asking:

"If this had always been the law, what would have happened?"

---

## Outcomes

### Seamless
- same effective behavior
- same or equivalent state transitions
- no operational impact

---

### Diverted
- different state path
- still valid under new law
- may require:
  - new effects
  - new handling logic
  - updated expectations

---

### Incompatible
- execution fails or becomes stuck
- no valid transition exists
- requires:
  - migration rules
  - data repair
  - or law adjustment

---

## Required Inputs

- target law
- schema
- initial context
- event history (chronicle)

Event history must include:
- ordered events
- valid Anchor (latest state transition)

---

## Replay Rules

1. Replay events in chronological order  
2. At each step, only use history up to that point (prefix-based)  
3. Set time explicitly from the event timestamp (`sys.now`)  
4. Evaluate the action using the new law  
5. Compare the resulting transition with historical outcome  
6. Detect divergence immediately  

---

## What "Divergence" Means

A divergence occurs when:

- the computed next_state differs from historical state_after  
- a guard that previously passed now fails (or vice versa)  
- an action no longer has a valid transition  
- effects differ meaningfully  
- execution halts (no valid path)

Divergence is a **signal**, not necessarily a failure.

---

## Review Checklist

During simulation, review:

- state transition differences  
- final state differences  
- guard behavior changes  
- effect changes (intent differences)  
- migration behavior (if applicable)  
- timeout / AUTO_TICK behavior  
- event consistency (state_before → state_after chain)  

---

## Two Usage Modes

### 1. CRUD / State-Based Systems

Focus on:

- final state differences  
- whether new logic breaks existing flows  
- whether new required fields exist  
- whether migration is needed  

You may not have full event history:
- ensure at least Anchor correctness  
- simulate with best available history  

---

### 2. Event-Sourced Systems

Focus on:

- full replay correctness  
- event-by-event divergence  
- projection impact  
- lineage consistency  

Simulation should mirror:
- actual event streams
- real ordering and timestamps

---

## Common Change Patterns

### Guard Tightening
- stricter thresholds
- may cause failures

---

### State Splitting
- one state becomes multiple
- leads to divergence

---

### Saga Introduction
- new transient states
- requires host handling

---

### Auto-Tick Changes
- new automatic transitions
- may create unexpected flows

---

### Migration Introduction
- records may need lift before evaluation
- absence of migration may cause incompatibility

---

## Triage Questions

1. Was migration required?  
2. Did migration land in a valid state?  
3. Was context complete per manifest?  
4. Did Anchor match expected state?  
5. Does the action still exist?  
6. Which guard changed behavior?  
7. Did effects change meaningfully?  

---

## Report Format

A simulation report should include:

- trace_id  
- original_final_state  
- simulated_final_state  
- outcome (Seamless / Diverted / Incompatible)  
- divergence_point (event index or action)  
- reason (guard failure, missing transition, etc.)  

---

## Example Interpretation

Scenario:
- old flow: Draft → Approved  
- new flow: Draft → PendingPayment → Approved  

Simulation result:
- state diverges at approval step  
- new intermediate state introduced  

Outcome:
- Diverted  

Conclusion:
- valid change  
- requires host support for new state  

---

## Escalation Conditions

Escalate when:

- incompatible results appear  
- divergence rate is high across records  
- unexpected AUTO_TICK behavior occurs  
- migration paths are unclear or missing  
- simulation results are inconsistent  

---

## Key Distinction: Events vs Effects

During simulation:

- Events are replayed and evaluated  
- Effects are ignored (not executed)  

You are validating:
- decisions
- transitions
- correctness

NOT:
- integrations
- side effects

---

## Goal

Simulation is a **decision tool**, not just a test.

It helps you:
- validate law correctness
- understand impact of changes
- detect risks before deployment
- reason about system behavior over time