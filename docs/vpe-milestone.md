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
