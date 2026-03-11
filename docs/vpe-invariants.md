# VPE SDK: System Invariants

## I. Namespace Invariants
1. **Reserved Categories:** Every path must start with a reserved prefix: `sys`, `rec`, `ext`, or `calc`.
2. **Read-Only System:** The `sys.` namespace is globally read-only for the Logic Engine. No Transform or Effect can mutate a `sys` variable.
3. **Write-Access:** Only the `rec.` namespace is mutable during standard transitions.
4. **Configurability:** Non-system namespaces (`rec`, `ext`, `calc`) must be explicitly defined in a Domain Schema before use.
5. The VpeRequest should include the sys.version_sequence. if the sequence in the database doesn't match the one the user is acting on, the Host rejects it before it even hits the engine.

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
