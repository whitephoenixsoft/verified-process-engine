# VPE Law Authoring Guide
Version: Canonical v1

## 1. Mental Model

A VPE Law defines how a system makes decisions.

Think in terms of:

- State → Where the record is
- Action → What is being attempted
- Guards → Conditions that must pass
- Transition → Where we go next
- Effects → What should happen outside the engine

Execution is:

Given (State, Action, Context, History) → produce (Next State, Effects, Events)

---

## 2. The Smallest Possible Law

A minimal valid process:

{
  "domain": "Example",
  "process": "SimpleFlow",
  "version": "1.0.0",
  "initial_state": "Draft",
  "states": [
    {
      "name": "Draft",
      "transitions": [
        {
          "action": "Submit",
          "to": "Approved",
          "guards": []
        }
      ]
    },
    {
      "name": "Approved"
    }
  ]
}

Key ideas:
- No guards = always allowed
- One action → one transition
- No effects yet

---

## 3. Branching with Priority

Multiple transitions for the same action are evaluated in priority order.

{
  "name": "Submitted",
  "transitions": [
    {
      "action": "Evaluate",
      "to": "Approved",
      "priority": 10,
      "guards": [
        { "type": "LessThan", "path": "rec.amount", "value": 1000 }
      ]
    },
    {
      "action": "Evaluate",
      "to": "ManualReview",
      "priority": 1,
      "guards": []
    }
  ]
}

Behavior:
- First matching transition wins
- Higher priority is evaluated first
- Empty guards = fallback

---

## 4. Default / Fallback Path

Always include a fallback when appropriate.

{
  "action": "Evaluate",
  "to": "ManualReview",
  "priority": 1,
  "guards": []
}

This acts as:
→ “else”

Without this, you risk runtime failures (NoTransitionFound).

---

## 5. Using Context

Context is a flat map with namespaces:

- rec.* → your data
- ext.* → external inputs
- sys.* → system values (time, trace_id)
- calc.* → derived values

Example:

{
  "type": "GreaterThan",
  "path": "rec.order_total",
  "value": 10000
}

Guidelines:
- Always reference full paths
- Keep naming consistent
- Avoid deep nesting (prefer flat keys)

---

## 6. Temporal Logic

Use history-aware guards.

OccurredWithin:

{
  "type": "OccurredWithin",
  "target_action": "FraudCheck",
  "window_seconds": 86400
}

Meaning:
→ FraudCheck happened in last 24h

TimeElapsed:

{
  "type": "TimeElapsed",
  "seconds": 300
}

Meaning:
→ At least 5 minutes since last transition

---

## 7. Effects (Side Effects)

Effects represent intent, not execution.

{
  "effects": [
    {
      "type": "WebHook",
      "target": "Payments",
      "action": "Charge",
      "params": {
        "order_id": "rec.order_id"
      }
    }
  ]
}

Rules:
- Effects are emitted, not executed
- Host system handles them
- Effects make transitions non-atomic

---

## 8. Saga Pattern (Transient States)

If a transition has effects, it MUST go to a transient state.

{
  "name": "PendingPayment",
  "is_transient": true,
  "transitions": [
    {
      "action": "PAYMENT_SUCCESS",
      "to": "Paid"
    },
    {
      "action": "PAYMENT_FAILED",
      "to": "Failed"
    },
    {
      "action": "AUTO_TICK",
      "to": "Expired",
      "guards": [
        { "type": "TimeElapsed", "seconds": 300 }
      ]
    }
  ]
}

Pattern:
1. Trigger effect
2. Move to transient state
3. Wait for:
   - success
   - failure
   - timeout

---

## 9. Auto Transitions

Use AUTO_TICK for automatic transitions.

{
  "action": "AUTO_TICK",
  "to": "Escalated",
  "guards": [
    { "type": "TimeElapsed", "seconds": 3600 }
  ]
}

Rules:
- No external trigger
- Must be acyclic
- Must be bounded

---

## 10. Migration Example

Used when upgrading versions.

{
  "migration_rules": [
    {
      "from_state": "Pending",
      "to_state": "AwaitingReview",
      "guards": [],
      "transforms": [
        {
          "op": "move",
          "from": "old_total",
          "to": "rec.order_total"
        },
        {
          "op": "set",
          "target": "rec.migrated",
          "value": true
        }
      ]
    }
  ]
}

---

## 11. How VPE Evaluates Your Law

At runtime:

1. Find current state
2. Filter transitions by action
3. Sort by priority
4. Evaluate guards (AND logic)
5. First passing transition wins
6. Emit verdict

Important:
- Order matters
- Priority matters
- Guards must be deterministic

---

## 12. Manifest Awareness

Every state declares what data it needs.

Example:

If you use:
- OccurredWithin(FraudCheck)
- rec.amount

Then the manifest will require:
- FraudCheck history
- rec.amount field

If the host does not provide it → execution fails

---

## 13. Authoring Guidelines

Good practices:

- Use clear state names (e.g., PendingPayment, not State1)
- Always include fallback transitions
- Keep transitions small and focused
- Prefer multiple states over complex guards
- Use transient states for all external effects
- Keep guards simple and composable

---

## 14. Anti-Patterns

Avoid:

❌ Giant States  
Too many transitions in one state  

❌ Hidden Dependencies  
Using fields not declared in schema  

❌ Missing Timeout  
Transient state without AUTO_TICK  

❌ Overloading Guards  
Putting too much logic in one guard  

❌ No Fallback  
Leads to runtime errors  

---

## 15. Design Philosophy

A good VPE law should be:

- readable
- predictable
- explicit
- testable

If someone cannot understand the flow quickly, the law is too complex.

---

## 16. Summary

VPE laws are:

- declarative
- deterministic
- validated at compile time
- executed as pure functions

Think of your law as:

→ a circuit  
→ a contract  
→ a decision graph  

Not as code.