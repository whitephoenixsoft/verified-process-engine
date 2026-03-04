# The VPE Manifesto: Logic as a Circuit

## I. Core Invariants (The "Unbreakable" Rules)

1. Pure Determinism: The Engine is a pure function. Given the same DAG, Context, and History, the Verdict must be identical. No hidden side effects, no global state.

f(DAG, Context, History) -> Verdict 

2. History as the Only Truth: The "Current State" is a cache. The Chronicle (Event Stream) is the immutable record of reality.
3. Safety by Construction: If a process can "hang" (infinite loop) or "leak" (dangling effect), the Compiler must refuse to build the DAG.
4. Zero-Trust Interop: The Engine assumes the external world (e.g., .NET) is messy. It uses flattened, namespaced maps to guard its boundaries.

## II. The Anatomy of the Machine

- The Law (JSON): A declarative, versioned document defining states, transitions, and guards. It is the "Source Code" of the business.
- The Registry (Opcodes): A static Rust library of logic primitives (e.g., GreaterThan, OccurredWithin). It maps strings to executable traits.
- The Compiler (The Judge): Validates the JSON for topological errors, overlapping guard logic, and infinite cycles. It "inflates" JSON into an optimized memory structure.
- The DAG (The Circuit): An arena-allocated, cache-friendly graph. Edges are sorted by Priority to ensure deterministic branching.
- The Runtime (The Pulse): A high-velocity evaluator that performs O(1) lookups to resolve paths and execute guards in nanoseconds.

## III. The "General Case" Extensions

- Temporal Logic: Guards can scan the Chronicle to enforce rules based on time or event frequency (e.g., "Max 3 attempts in 24 hours").
- Lazy Migration (Lifting): Records on V1 law are automatically "lifted" to V2 law upon their next interaction, guided by Migration Guards and Data Transformations.
- Saga Integrity: The Compiler enforces "Closure." If a transition emits a cross-domain effect, the DAG must define handlers for Success, Failure, and Timeout.

## IV. Operational Philosophy

- Fail Fast, Fail Loud: A logical inconsistency (like a missing failure handler) is a Compile-Time Error, not a production incident.
- Simulation as a Standard: Before any "Law" is changed, it must pass a Migration Impact Report—a dry run against production-scale data snapshots.
- Roll-Forward Only: We never undo a migration. We fix the law and move the records to the next version.

## V. The "Verdict" Contract

Every execution returns a structured Verdict:
1. Transition: Where are we going?
2. Effects: What needs to happen in the outside world?
3. Compensations: How do we undo this if the next step in the chain fails?