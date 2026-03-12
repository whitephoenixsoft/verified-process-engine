# VPE Compiler: Pipeline Specification (V1.2)

## Phase 1: Ingestion & Binding
* **Schema Cross-Reference:** Validate the JSON `domain` against the registered `DomainSchema`.
* **Namespace Binding:** Scan all `paths` in guards/transforms. Ensure they start with `rec`, `sys`, `ext`, or `calc`.
* **Field Verification:** Cross-reference every `rec.*` path against the `DomainSchema`. If `rec.total_amount` is used but not defined in the schema, fail compilation.

## Phase 2: Topological (Graph) Audit
* **Initial State Check:** Verify the `initial_state` exists in the `states` array.
* **Orphan Detection:** * Algorithm: Traverse the graph starting from the `initial_state`.
    * Failure: Any state not reachable from the start is an "Orphan."
    * Warning: Any state with no outgoing transitions is a "Terminal" state (informational only).
* **Auto-Loop Detection (The No-Ouroboros Rule):**
    * Algorithm: Perform Depth-First Search (DFS) on the subgraph where `action == null` or `action == "AUTO_TICK"`.
    * Failure: If a back-edge is found (a cycle), reject the Law.

## Phase 3: Semantic & Type Audit
* **Type Safety:** For every Guard, compare the `value` type to the `path` type defined in the `DomainSchema`. (e.g., Cannot compare a `String` field to a `Number` value).
* **Shadowing Detection:** * Algorithm: Within a single State, check if two transitions share identical Guard logic. 
    * Warning: If Transition A is a subset of Transition B, Transition B can never fire.
* **Namespace Write Protection:** Verify that no `TransformOp` (Move/Set) targets the `sys.*` namespace.

## Phase 4: Saga & Side-Effect Safety
* **Transient State Enforcement:** * Condition: If a transition contains 1 or more `Effects` (external calls).
    * Constraint: The `target_state` of that transition MUST have `is_transient: true`.
* **Timeout Requirement:**
    * Condition: If a state is `is_transient`.
    * Constraint: It MUST contain at least one `AUTO_TICK` transition with a `TimeElapsed` or `OccurredWithin` guard to prevent the record from being stuck forever.

## Phase 5: Manifest Synthesis (The Output)
* **Requirement Aggregation:** For every state, iterate through all transitions. Call `guard.get_requirements()` for every guard present.
* **State Manifest Generation:** Create a Map of `StateName -> List<HistoryRequirement>`. 
    * *Note: Always include `LastTransition` (The Anchor) as a default requirement for every state.*
* **Registration Report:** Package the DAG, the Manifests, and any Warnings into a final result object for the Host.

## Compilation Success Object (Output Format)
```json
{
  "version": "1.2.0",
  "domain": "String",
  "digest": "Hash", // Used for O(1) change detection
  "manifest": {
    "StateName": [ "RequirementEnum" ]
  },
  "metadata": {
    "is_acyclic_auto": true,
    "has_sagas": true,
    "warnings": []
  }
}
```