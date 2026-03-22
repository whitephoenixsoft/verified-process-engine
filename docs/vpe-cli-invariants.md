# VPE CLI Invariants
Version: Canonical v1.1

## 1. Purpose Invariants
1. The CLI is a first-class delivery surface for VPE.
2. The CLI is a thin harness over the Rust library APIs.
3. The CLI must not implement separate business logic from the Rust library.
4. The CLI exists to support design-time validation, compilation, execution, simulation, and migration workflows.
5. The CLI may exist in two forms:
   - official portable CLI
   - custom project-specific CLI harness

## 2. Layering Invariants
1. The Rust library is the canonical implementation of VPE semantics.
2. The CLI must call the same compiler, runtime, simulation, and migration APIs exposed by the Rust library.
3. The CLI must not bypass compiler, runtime, or engine invariants.
4. The CLI must not introduce behavior that differs from the Rust API for the same inputs.
5. The CLI must not duplicate VPE logic that already exists in the Rust library.

## 3. Determinism Invariants
1. Given the same CLI inputs, the CLI must produce the same outputs.
2. The CLI must not read ambient wall-clock time unless explicitly requested by the user and reflected in input.
3. Any time-dependent execution must require explicit input time or derive time deterministically from provided input.
4. CLI output must reflect deterministic library behavior only.

## 4. Input/Output Invariants
1. The CLI must support JSON input and JSON output.
2. JSON is the canonical machine-readable format for all CLI commands.
3. The CLI must never require users to understand internal graph or runtime structures.
4. Inputs and outputs must use domain concepts such as schema, law, request, chronicle, manifest, verdict, simulation report, and lift result.
5. CLI commands must be scriptable through standard input/output and file-based input.
6. The CLI must preserve structured error information in its output.

## 5. Surface Consistency Invariants
1. Commands must map clearly to core VPE capabilities.
2. Equivalent typed Rust APIs and CLI commands must behave consistently.
3. A CLI command must never expose capabilities that do not exist in the Rust library.
4. A CLI command must never reinterpret library semantics for convenience.

## 6. Command Invariants
1. `validate` validates schema and law only.
2. `compile` validates and compiles schema and law into a compiled process artifact or report.
3. `manifest` inspects manifest requirements for a compiled or source-defined process.
4. `execute` performs exactly one deterministic engine turn.
5. `simulate` replays historical input using simulation semantics.
6. `lift` performs migration/lift semantics only.
7. Each command must have a single clear responsibility.

## 7. Source vs Compiled Mode Invariants
1. The CLI may support source mode and compiled mode.
2. Source mode must operate on schema and law inputs.
3. Compiled mode must operate on a valid compiled process artifact.
4. Compiled mode must not weaken validation guarantees.
5. Any compiled artifact accepted by the CLI must be produced or validated by VPE-defined compilation rules.

## 8. Artifact Legitimacy Invariants
1. The CLI must not accept arbitrary unvalidated compiled artifacts as executable truth.
2. Compiled artifacts must be immutable once produced.
3. Artifact compatibility must be checked before use.
4. Artifact digests must remain deterministic and stable for the same compiled process.
5. Artifact loading must not bypass runtime or manifest invariants.

## 9. Validation Invariants
1. CLI validation must fail early and loudly on schema or law violations.
2. Validation must include schema correctness, law correctness, registry resolution, and manifest completeness.
3. Missing guard dependencies from manifests must be treated as an error.
4. Suspicious guard requirement declarations may produce warnings.
5. Validation must be possible without requiring runtime engine registration.

## 10. Execution Invariants
1. `execute` must represent exactly one deterministic engine turn.
2. `execute` must require all data necessary to satisfy runtime invariants.
3. `execute` must validate anchor presence and state consistency.
4. `execute` must not perform side effects.
5. `execute` must emit a verdict only.

## 11. Simulation Invariants
1. `simulate` must use the same decision logic as runtime execution.
2. `simulate` must replay history incrementally using prefix-based semantics.
3. `simulate` must not execute effects.
4. `simulate` must classify outcomes consistently.
5. `simulate` must remain deterministic for the same inputs.

## 12. Migration Invariants
1. `lift` must use the same migration logic as the Rust library.
2. `lift` must remain deterministic.
3. `lift` must not rewrite history.
4. `lift` must produce a valid target state or a deterministic error.
5. `lift` must not bypass schema, namespace, or type invariants.

## 13. Error Handling Invariants
1. The CLI must return structured, deterministic errors.
2. The CLI must not panic for expected user-facing failures.
3. The CLI must preserve enough error detail for debugging and automation.
4. Schema, compile, runtime, simulation, and migration errors must remain distinguishable.
5. Non-zero process exit status must be used for failures.

## 14. Human and Machine Usability Invariants
1. The CLI must be scriptable first.
2. JSON output must be suitable for automation and CI.
3. Human-friendly presentation may be added, but must not replace machine-readable output.
4. Defaults should favor deterministic, explicit behavior over convenience magic.
5. The CLI should help users reason about laws and processes, not hide system behavior.

## 15. Distribution Invariants
1. The CLI is distributed as a binary surface over the Rust library.
2. CLI releases must remain compatible with the underlying library versioning policy.
3. The CLI must not become the only supported access path to VPE features.
4. The CLI must strengthen the ecosystem around the Rust library, not replace it.

## 16. Custom Guard Invariants
1. The official portable CLI guarantees full support for built-in guards only.
2. Custom guards are supported only when they are registered in the running CLI executable.
3. Unknown custom guards must be treated as an error by default.
4. The CLI must not silently skip, stub, or reinterpret unknown guards unless an explicit non-default mode is introduced in the future.
5. A custom CLI harness may register built-in guards and project-specific custom guards together.
6. Custom guard support in a CLI must use the same GuardRegistry model as the Rust library.
7. No runtime plugin or scripting system is required for CLI custom guard support in v1.

## 17. Custom CLI Harness Invariants
1. A project may build its own CLI harness on top of shared CLI command logic.
2. A custom CLI harness must remain thin over the Rust library.
3. A custom CLI harness must not fork or redefine VPE semantics.
4. A custom CLI harness may provide a dedicated source folder or module for project-specific guards.
5. The official CLI and custom CLI harnesses must share the same command semantics where features overlap.

## 18. Product Invariants
1. The CLI is part of the VPE product, not an afterthought.
2. The CLI should function as a harness for:
   - authoring
   - validation
   - inspection
   - experimentation
   - debugging
3. The CLI must reinforce VPE’s core values:
   - determinism
   - explicitness
   - compile-time safety
   - replayability
4. The CLI must make VPE easier to adopt without weakening VPE’s guarantees.