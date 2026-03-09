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

----

# VPE SDK: Module Specifications (V1.0 - 2026)

## 1. Registry Module (The Vocabulary)
The Registry acts as the engine's "dictionary," mapping JSON strings to Rust logic.
- **Factory Pattern:** Uses a closure-based factory to hydrate Guard traits from JSON parameters.
- **Extensibility:** Allows the Host to register custom logic (e.g., specialized math or geo-fencing) before compilation.
- **Statelessness:** Guards are immutable once registered; configuration is passed at hydration time.

## 2. Compiler Module (The Architect)
A Two-Phase validator that transforms raw JSON into a "Verified Logic Graph."
- **Phase 1 (The World):** Constructs the State Machine (DAG) and initializes the Domain Schema.
- **Phase 2 (The Bridge):** Audits Migration Rules against the baked DAG to prevent "Ghost State" errors.
- **Identifier Policing:** Validates all paths against alphanumeric standards and mandatory dot-notation.
- **Type Auditing:** Cross-references JSON values against the Domain Schema to ensure type-safe comparisons.

## 3. Runtime Module (The Muscle)
A high-performance execution loop optimized for sub-10 microsecond decisions.
- **Index Jumps:** Navigates the DAG using memory offsets (usize) rather than string lookups.
- **Short-Circuit Logic:** Evaluates guards in sequence, exiting the transition as soon as a "False" is hit.
- **Pure Functionality:** The Runtime is side-effect-free; it produces a "Verdict" rather than modifying state directly.

## 4. Simulation Engine (The Time Machine)
The Dry-Run module used for pre-deployment risk analysis.
- **History Replay:** Re-runs historical "Humidity Events" (The Chronicle) against new versions of the Law.
- **Impact Analysis:** Categorizes migrations as "Seamless" (identical outcome), "Divergent" (new outcome), or "Incompatible" (stuck).
- **TraceID Correlation:** Uses unique identifiers to track a record's journey across multiple versions.

## 5. Migration Engine (The Bridge)
Handles the physical "Lifting" of records between versions.
- **Conditional Transforms:** Uses existing Guard logic to decide when and how to reshape data.
- **TransformOps:** Supports Move (renaming), Set (injecting), and Map (translating) operations.
- **Version Drift Detection:** Provides an O(1) check to determine if a record requires a version lift.

## 6. FFI & Marshaling (The Bridge)
The C-ABI layer providing native access for .NET and Java.
- **Opaque Pointers:** Keeps Rust memory management hidden from the Host.
- **JSON Marshaling:** Uses JSON strings for complex input (Context/History) to maintain a language-agnostic API.
- **Memory Handshake:** Provides explicit "Free" functions to prevent RAM leaks across the foreign boundary.
-