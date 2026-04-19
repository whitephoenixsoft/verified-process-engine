# VPE Law Authoring Guide
Version: Canonical v2

## 1. Mental Model

A VPE Law defines how a system makes decisions.

Think in terms of:

- State → Where the record is
- Action → What is being attempted
- Guards → Conditions that must pass
- Transition → Where we go next
- Effects → What should happen outside the engine
- Events → What is recorded as truth

Execution is:

Given (State, Action, Context, History) → produce (Next State, Effects, Events)

---

## 2. The Smallest Possible Law

A minimal valid process:

{
  "domain": "Example",
  "process": "SimpleFlow",
  "version": "1.0.0",
  "schema_version": "1.0.0",
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
- Law explicitly binds to schema_version

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

- rec.* → your data (persistent)
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
- Prefer flat structures over nesting

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
      "mode": "untracked",
      "type": "WebHook",
      "target": "Notifications",
      "action": "SendEmail",
      "params": {
        "order_id": "rec.order_id"
      }
    }
  ]
}

Rules:
- Effects are emitted, not executed
- Host system handles them
- Effects do NOT imply completion
- Effects do NOT modify state directly

---

## 8. Tracked vs Untracked Effects

### Untracked Effects (Default)

Use for:
- notifications
- analytics
- logging
- non-critical background work

Properties:
- fire-and-forget
- no lifecycle tracking
- do not require transient states

---

### Tracked Effects

Use for:
- payments
- inventory reservation
- external approvals
- any correctness-critical operation

Rules:
- must transition into a transient state
- must be resolved via events
- must not assume success

---

## 9. Saga Pattern (Transient States)

If a transition has `tracked` effects, it MUST go to a transient state.

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

## 10. Auto Transitions

Use AUTO_TICK for automatic transitions.

{
  "action": "AUTO_TICK",
  "to": "Escalated",
  "guards": [
    { "type": "TimeElapsed", "seconds": 3600 }
  ]
}

Rules:
- No external trigger required
- Must be acyclic
- Must be bounded
- Executed automatically by runtime

---

## 11. Migration Example

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

## 12. How VPE Evaluates Your Law

At runtime:

1. Find current state
2. Filter transitions by action
3. Sort by priority
4. Evaluate guards (AND logic)
5. Select first matching transition
6. Apply transition
7. Execute AUTO_TICK transitions (if any)
8. Produce verdict

Important:
- Order matters
- Priority matters
- Guards must be deterministic

---

## 13. Manifest Awareness

Every state declares what data it needs.

Example:

If you use:
- OccurredWithin(FraudCheck)
- rec.amount

Then the manifest will require:
- FraudCheck history
- rec.amount field

If the host does not provide it:
→ execution fails deterministically

---

## 14. Authoring Guidelines

Good practices:

- Use clear, descriptive state names
- Always include fallback transitions when appropriate
- Keep transitions small and focused
- Prefer multiple simple states over complex guards
- Use transient states for tracked effects
- Keep guards simple and composable
- Be explicit about behavior

---

## 15. Anti-Patterns

Avoid:

❌ Giant States  
Too many transitions in one state  

❌ Hidden Dependencies  
Using fields not declared in schema  

❌ Missing Timeout  
Transient state without AUTO_TICK  

❌ Overloaded Guards  
Too much logic in a single guard  

❌ No Fallback  
Leads to runtime errors  

❌ Misusing Untracked Effects  
Do not rely on them for correctness  

---

## 16. Design Philosophy

A good VPE law should be:

- readable
- predictable
- explicit
- testable

If someone cannot understand the flow quickly, the law is too complex.

---

## 17. Summary

VPE laws are:

- declarative
- deterministic
- validated at compile time
- executed as pure functions

Think of your law as:

→ a decision graph  
→ a contract  
→ a deterministic system  

Not as code.