# VPE: Verified Process Engine Specification

## 1. Core Philosophy & Invariants

- Separation of Law and Physics: The Engine (Rust) defines how to execute (Physics), while the Graph (JSON) defines what the rules are (Law).
- History is Truth: Current state is a derivative; the Event Chronicle (immutable log) is the only primary data source.
- Determinism: Given the same DAG, ContextMap, and Chronicle, the Runtime must produce the exact same Verdict.
- The "Boring" Bridge: Communication between the host language (.NET) and the Engine (Rust) occurs via flattened, namespaced key-value maps to eliminate reflection overhead.

## 2. Component Specifications

### A. The Logic Registry (The Library)

A static mapping of string identifiers to Rust "Guard" implementations.
- Invariant: The Compiler and Runtime must share the same Registry to prevent "Logic Drift."
- Interface 
```Rust
pub trait Guard {
    fn check(&self, context: &ContextMap, history: &[VpeEvent]) -> bool;
}
```
### B. The Compiler (The Validator)

Transforms "Loose" JSON into a "Tight" Memory DAG.
- Topology Validation: Checks for orphans (unreachable states) and sinks (dead ends).
- Conflict Resolution: Detects overlapping guards on the same action. If priorities aren't explicit for overlapping logic, the compiler must fail.
- Static Inflation: Converts JSON parameters (e.g., 1000) into typed Rust values inside the Guard structs.

### C. The DAG (The Machine Code)

A memory-optimized representation of the workflow.
- Arena Allocation: Uses a `Vec<Node>` where edges point to usize indices rather than pointers.
- Structure 
```Rust
pub struct VpeDag {
    pub nodes: Vec<Node>,
}
pub struct Edge {
    pub action: String,
    pub priority: u32,
    pub target_idx: usize,
    pub guards: Vec<Box<dyn Guard>>,
    pub effects: Vec<String>,
}
```
### D. The Runtime (The Judge)

A stateless pure function that evaluates a specific action.
- Input: (CurrentState, Action, ContextMap, Chronicle)
- Execution:
    1. Locate Node by index.
    2. Filter Edges by Action.
    3. Sort by Priority (Descending).
    4. Execute Guards for each edge sequentially.
    5. Return first successful Verdict or NoTransitionError

## 3. Data Specs: The Context & Chronicle

### The Context Map (Current Reality)

A flattened HashMap<String, Value> using namespaces:
- entity.*: Fields from the object (e.g., entity.Total).
- ext.*: External data (e.g., ext.CreditScore).
- env.*: Environment data (e.g., env.Timestamp).

### The Chronicle (The History)

A sorted list of VpeEvent objects
```Rust
pub struct VpeEvent {
    pub timestamp: i64,
    pub action: String,
    pub was_successful: bool,
    pub state_before: String,
    pub state_after: String,
}
```

## 4. Multi-Path JSON Example

This example demonstrates Branching, Priority, and Temporal Guards.

```JSON
{
  "process": "LoanApproval",
  "version": "2.1.0",
  "initial_state": "Draft",
  "transitions": [
    {
      "from": "Submitted",
      "action": "Evaluate",
      "comment": "Path A: Auto-Approve small loans for premium members",
      "to": "Approved",
      "priority": 10,
      "guards": [
        { "type": "LessThan", "path": "entity.Amount", "value": 1000 },
        { "type": "IsEqual", "path": "ext.MemberTier", "value": "Premium" }
      ],
      "effects": ["NotifyCustomer", "TriggerPayout"]
    },
    {
      "from": "Submitted",
      "action": "Evaluate",
      "comment": "Path B: Escalate large loans or high-risk cases",
      "to": "PendingVP",
      "priority": 5,
      "guards": [
        { "type": "GreaterThan", "path": "entity.Amount", "value": 50000 }
      ],
      "effects": ["EmailVP", "LogHighValueRisk"]
    },
    {
      "from": "Submitted",
      "action": "Evaluate",
      "comment": "Path C: Default path for standard review",
      "to": "PendingManager",
      "priority": 1,
      "guards": [], 
      "effects": ["AssignToQueue"]
    },
    {
      "from": "PendingManager",
      "action": "Approve",
      "comment": "Temporal Guard: Requires a FraudCheck event in the last 24h",
      "to": "Approved",
      "guards": [
        { 
          "type": "OccurredWithin", 
          "target_action": "FraudCheck", 
          "window_seconds": 86400 
        }
      ]
    }
  ]
}
```

## 5. Next Checkpoint: Cycle Detection Logic

To distinguish between a Healthy Loop and an Infinite Loop, the Compiler uses Cycle Analysis:
1. Healthy Loop (State Re-entry): An action that moves from Rejected back to Draft. This is allowed because it requires an External Action (User Input) to trigger.
2. Infinite Loop (Auto-Cycle): If Path A has an effect that triggers Action B, and Action B leads back to Path A without any Guard or External Input changing, the system "oscillates."

### The Compiler Invariant for Cycles

No sequence of 'Auto-Transitions' (transitions triggered by internal effects) may form a closed loop without an intervening 'External Action' requirement.

## Summary of the VPE Pipeline

| Phase    | Responsibility          | Outcome                        |
| -------- | ----------------------- | ------------------------------ |
| JSON     | Define the Business Law | Readable "Source Code"         |
| Registry | Map Strings to Logic    | Library of available "Opcodes" |
| Compiler | Validate & Inflate      | Optimized Memory DAG           |
| DAG      | Represent the Flow      | High-speed "Instruction Set"   |
| Runtime  | Evaluate Reality        | Deterministic "Verdict"        |
