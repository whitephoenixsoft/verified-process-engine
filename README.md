# VPE: Verified Process Engine

VPE is a high-performance, deterministic logic kernel written in Rust. It treats business processes as "compiled circuits," moving logic out of fragile code and into a verified, versioned, and auditable Directed Acyclic Graph (DAG).



## 1. The Core Philosophy
Traditional state machines are "goldfish"—they have no memory and no awareness of the world. VPE evolves this into a Contextual Model:

- Verified: The compiler prevents infinite loops, dead ends, and dangling saga effects at build-time.
- Stateless Core: A pure-function runtime that accepts a DAG, Current Reality (Context), and History (The Chronicle).
- Language Agnostic: High-speed FFI bridge for .NET, Go, Python, and more.

## 2. Architecture
VPE is divided into four distinct layers:

- The Law (JSON): Declarative process definitions including guards, branching, and migration rules.
- The Compiler (Rust): A validation suite that performs topological and temporal analysis to ensure the logic is "safe."
- The Registry (Opcodes): A library of logic primitives (e.g., GreaterThan, OccurredWithin) mapped to Rust traits.
- The Runtime (The Pulse): An arena-allocated DAG execution loop optimized for sub-10 microsecond decisions.



## 3. Minimal API (Rust Internal)
The Rust engine uses a simple lifecycle: Compile -> Register -> Execute.

- VpeCompiler::compile(json_string): Validates and returns a VpeDag.
- VpeRegistry::insert(name, version, dag): Stores the graph.
- VpeEngine::execute(domain, version, state, action, context, history): The primary execution call.

## 4. FFI Bridge (The C Interface)
Used by .NET/Go to interact with the Rust Kernel across the binary boundary:

- void* vpe_init(): Initializes the engine.
- bool vpe_load_graph(const char* json): Compiles and registers a graph.
- FfiVerdict* vpe_evaluate_ffi(void* dag_ptr, size_t state_idx, const char* action, ...): Cross-boundary evaluation.
- void vpe_free_verdict(FfiVerdict* ptr): Explicitly frees Rust-allocated memory.

## 5. The "Law" (JSON Specification)
VPE supports complex enterprise features like Lazy Migration and Saga Patterns directly in the schema.

{
  "domain": "OrderManagement",
  "version": "2.0.0",
  "supersedes": "1.0.0",
  "migration_rules": [
    {
      "from_state": "Pending",
      "to_state": "AwaitingTaxInfo",
      "guards": [{ "type": "MissingField", "path": "entity.TaxID" }]
    }
  ],
  "transitions": [
    {
      "action": "Approve",
      "from": "Pending",
      "to": "Approved",
      "priority": 10,
      "guards": [
        { "type": "OccurredWithin", "target": "FraudCheck", "window_seconds": 3600 }
      ],
      "effects": [
        { "type": "CrossDomain", "target": "Ledger", "action": "Debit", "on_success": "Confirm" }
      ]
    }
  ]
}



## 6. Safety & Invariants
- Deterministic Replay: Given the same history and context, the verdict is always the same.
- Cycle Detection: The compiler refuses to build automated "infinite loops."
- Saga Completeness: If an action triggers a cross-domain effect, the compiler ensures Success, Failure, and Timeout handlers exist.
- Lazy Lifting: Records are upgraded to the latest "Law" only when touched, using the defined migration rules.

## 7. Performance
VPE is built for high-throughput environments:

- Evaluation Time: Under 10 microseconds per decision.
- Memory Layout: Arena-allocated nodes with index-based navigation (no pointer chasing).
- Context: Flat, namespaced HashMaps for O(1) property access.



## 8. License
Licensed under Apache 2.0.
