# VPE SDK: System Invariants

## I. Namespace Invariants
1. **Reserved Categories:** Every path must start with a reserved prefix: `sys`, `rec`, `ext`, or `calc`.
2. **Read-Only System:** The `sys.` namespace is globally read-only for the Logic Engine. No Transform or Effect can mutate a `sys` variable.
3. **Write-Access:** Only the `rec.` namespace is mutable during standard transitions.
4. **Configurability:** Non-system namespaces (`rec`, `ext`, `calc`) must be explicitly defined in a Domain Schema before use.

## II. Identifier Invariants
1. **Format:** Must follow the pattern `namespace.sub_path.key`.
2. **Character Set:** Only alphanumeric characters and underscores are allowed.
3. **Leading Digits:** Segments cannot start with a number.
4. **Case Sensitivity:** All identifiers are case-sensitive to ensure cross-platform consistency.

## III. Structural & Security Invariants
1. **Determinism:** Given the same Context and History, the Engine must return the same Verdict.
2. **Cycle Prevention:** The Compiler must detect and block infinite "Auto-Tick" loops during registration.  A Directed Graph may contain cycles, but the subgraph formed by edges where action == null must be a Directed Acyclic Graph (DAG).
3. **Connectivity:** Every Migration 'to_state' must exist as a valid node in the target version's DAG.
4. **Zero-Trust FFI:** The Host cannot access the Engine's internal Graph; communication is limited to Pointer-based execution calls.
5. **Type Finality:** A field's DataType (String, Number, Bool) is locked at the Domain Schema level and cannot be changed by the JSON logic.

## IV. Data Lineage Invariants
1. **Traceability:** Every transition and migration must generate an event with a valid TraceID.
2. **Immutability of History:** The Chronicle (History Events) is a read-only input for Guards; the Engine cannot alter past events.

## V. History State Invariants 
1. The current_state_idx provided in a VpeRequest must match the to_state of the most recent STATE_TRANSITION event in the history. If history is present and the states do not match, the Engine must return a DesyncError
2. The Anchor Rule: Every execution must be accompanied by at least the most recent transition event
3. The Window Rule: History is only required if a Guard specifically references a past event.
4. The Conflict Rule: A write is only valid if the parent_event_id of the new event matches the event_id of the anchor provided during execution.

---

# VPE SDK: Added Invariants

## I. The Anchor Invariant
- **Proof of State:** No transition can be evaluated without the most recent `STATE_TRANSITION` event (The Anchor).
- **Desync Protection:** If the Anchor's target state does not match the Engine's current node, the Engine must throw a `DesyncError`.

## II. Automated Loop Invariant
- **The "No-Ouroboros" Rule:** A DAG may have cycles (e.g., Draft -> Review -> Draft), but any cycle consisting entirely of `AUTO_TICK` (action-less) transitions is a fatal compilation error.

## III. Optimistic Concurrency Invariant
- **The Turn Lock:** The Host must use the Anchor's ID as a "Version Gate" during the database write. A write is only successful if `Record.LastEventID == ProvidedAnchor.ID`.

## IV. Sagas
- Any transition with an External Effect **MUST** land in a Transient (Saga) state. It cannot land in a Terminal or Stable state directly.