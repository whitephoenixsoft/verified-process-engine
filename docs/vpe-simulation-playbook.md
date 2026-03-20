# VPE Human Simulation Playbook
Version: Canonical v1

## Purpose
Guide humans in evaluating law changes using simulation.

## Key Question
Would historical records behave correctly under the new law?

## Outcomes

Seamless  
- same effective behavior  

Diverted  
- different path but still valid  

Incompatible  
- execution fails or becomes stuck  

---

## Required Inputs

- target law
- schema
- initial context
- event history

---

## Replay Rules

1. Replay events in order  
2. Use only past history at each step  
3. Set time from event timestamp  
4. Evaluate action  
5. detect divergence immediately  

---

## Review Checklist

- state changes  
- final state differences  
- guard changes  
- effect changes  
- migration behavior  
- timeout behavior  

---

## Common Issues

Guard Tightening  
- thresholds changed  

State Splitting  
- one state becomes multiple  

Saga Introduction  
- new transient states  

Auto-Tick Behavior  
- new automatic transitions  

---

## Triage Questions

1. Was migration required?  
2. Did migration land correctly?  
3. Was context complete?  
4. Did anchor match?  
5. Did action still exist?  
6. Which guard failed?  

---

## Report Format

- trace id  
- original state  
- simulated state  
- outcome  
- divergence point  
- reason  

---

## Example Interpretation

If a process now requires payment verification:
- old flow: direct approval  
- new flow: pending payment  

Result: Diverted  

Conclusion:
- valid change  
- requires host support  

---

## Escalation Conditions

- incompatible results  
- large-scale divergence  
- unexpected timeout behavior  
- ambiguous migration  

---

## Goal

Simulation is a decision tool, not just a test.  
It helps validate law correctness before deployment.