# VPE MASTER SPECIFICATION: REGISTRY, COMPILER, RUNTIME, & EVOLUTION

## 1. THE REGISTRY (The Logic Library)
The Registry is a thread-safe "Dictionary of Truth" that maps JSON strings to executable Rust code.

- Trait Definition: 'Guard' must implement 'fn check(&self, context: &ContextMap, history: &[VpeEvent]) -> bool'.
- Storage: Uses a 'HashMap<String, Arc<Box<dyn Guard + Send + Sync>>>'.
- Lifetime: Initialized once at startup; immutable thereafter to ensure high-concurrency safety.
- Linking: Every guard 'type' in the JSON must have a corresponding entry in the Registry.



## 2. THE COMPILER (The Verification Pipeline)
The Compiler is a multi-pass assembler that ensures "The Law" is mathematically sound before it is executed.

- Pass 1 (Structural): Ensures 'initial_state' exists and all transition targets are valid states.
- Pass 2 (Logic): Verifies all Guard 'types' and Effect 'types' exist in the Registry.
- Pass 3 (Safety):
  - Cycle Detection: DFS traversal to find automated infinite loops.
  - Saga Completeness: Ensures every 'Effect' has handlers for 'on_success', 'on_failure', and 'on_timeout'.
- Pass 4 (Inflation): Replaces string names with 'usize' indices and arena-allocates the DAG for O(1) lookup.



## 3. THE RUNTIME (The Execution Pulse)
A stateless, pure-function evaluator that executes logic in sub-10 microsecond intervals.

- Input: (CurrentState, Action, ContextMap, Chronicle).
- Algorithm:
  1. Jump to 'Node' by index.
  2. Filter 'Edges' by 'Action' string.
  3. Sort by 'Priority' (descending).
  4. Sequentially execute 'Guards'.
  5. Return first valid 'Verdict' or a 'NoTransitionFound' error.
- Output: (NextState, NextStateIndex, Effects).



## 4. THE MIGRATION ENGINE (The Lazy Lifter)
Handles "Version Drift" by upgrading records from an old version (V_n) to the latest version (V_n+1) during execution.

- Version Compatibility: Uses a 'supersedes' array to define valid migration paths.
- Transformation Pipeline:
  - Field Mapping: Renaming or moving context data (e.g., 'user_id' to 'account_id').
  - Computed Defaults: Generating new required fields via expressions (e.g., 'FullName' = 'First' + 'Last').
  - Type Casting: Normalizing data types for the new Registry version.
- Migration Guards: Special guards that run only during a version jump to decide the "Landing State" (e.g., splitting one legacy state into two new specialized states).



## 5. THE DRY RUN UTILITY (The Impact Tool)
A simulation layer to run "What If" scenarios against production data snapshots without side effects.

- Virtual Replay: Re-processes the entire 'History Chronicle' of a record against the new DAG version.
- Impact Buckets:
  - Seamless: No state change or data loss.
  - Diverted: Moved to a "Repair" or "Wait" state due to new rules.
  - Incompatible: The record's history violates an unbreakable invariant in the new version.
- Isolation: Effects are generated and logged for analysis but never dispatched to external systems.



## 6. INTEGRATED DATA FLOW (The Execution Chain)
When 'execute()' is called, the Rust Core follows this sequence:

1. Identification: Compare Record.SchemaId against Registry.CurrentId.
2. Lifting (If Required): Apply Transformations and Migration Guards.
3. Landing: Update Record to the new DAG's entry-point state.
4. Evaluation: Run the standard Runtime logic for the requested Action.
5. Verdict: Return NextState, Effects, and the new SchemaId.

## 7. SYSTEM INVARIANTS
- No Manual Overrides: State transitions must only occur via the Engine.
- Historical Integrity: The Chronicle is immutable; migrations append a 'LiftEvent' rather than rewriting history.
- Atomic State: The Host must save the State, Context, and SchemaId in a single transaction.

---
## Summary of the Process V1.1

1. RegisterRegister: You send the JSON. The Compiler checks for loops and builds a Manifest of what history each state needs.
2. Inquiry: Your .NET app asks the Engine for the Manifest of the current state.
3. Data Fetch: Your .NET app queries the DB for: LastEvent (The Anchor) + SpecificEvents (from Manifest).
4. Execute: The Engine verifies the Anchor, runs the guards against the thin history, and gives you a Verdict + a New Event.
5. Commit: Your .NET app saves the new state and the new event to the DB in one transaction, using the Anchor's ID to ensure no one else moved the record while you were calculating.

---

## Saga Process

1. Phase A: The "Side-Effect" Flag
In the JSON Law, if a transition involves an external call (e.g., Effect: "Call_Payment_Gateway"), the Compiler flags this transition as Non-Atomic.
2. Phase B: Compiler Enforcement (The "Saga Guardrail")

### How the "Saga State" Works in the DAG
Instead of Draft -> Approved, the Law must look like this:
1. Action: "Submit"
2. Move to: Pending_External_Verification (The Saga State)
3. Requirement: This state must have at least two exits:
    - Success Path: (Triggered by a callback or system event).
    - Timeout/Failure Path: (Triggered by a sys.now check or a manual "Cancel").
