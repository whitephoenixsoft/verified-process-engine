# The Rust Minimal API

## Core

This API focuses on the lifecycle of a Process: Compile, Store, and Evaluate.

| Function / Field                          | Description                                                |
| ----------------------------------------- | ---------------------------------------------------------- |
| `VpeCompiler::compile(json: &str)`        | Validates topology, cycles, and logic; returns a VpeDag.   |
| `VpeRegistry::insert(name, version, dag)` | Stores a compiled graph in a thread-safe `Arc<RwLock<T>>`. |
| `VpeDag::evaluate(...)`                   | The primary execution loop; returns a Verdict.             |
| `VpeDag::simulate_migration(...)`         | Checks if old data can "lift" into this DAG version        |

```Rust
// The primary entry point for the Library
pub struct VpeEngine {
    registry: Arc<VpeRegistry>,
}

impl VpeEngine {
    /// Loads and validates a new version of a process
    pub fn register_process(&self, name: &str, json: &str) -> Result<(), CompileError>;

    /// The "Pure" execution call
    pub fn execute(
        &self,
        domain: &str,
        version: &str,
        current_state: &str,
        action: &str,
        context: HashMap<String, Value>,
        history: Vec<VpeEvent>
    ) -> Result<VpeVerdict, VpeError>;
}

```

## The FFI Bridge API (The "C" Interface)

This is what the .NET or Go library "sees" when it loads your .so or .dll.

| Function              | Purpose                                                                    |
| --------------------- | -------------------------------------------------------------------------- |
| vpe_init()            | Initializes the engine and registry in Rust memory.                        |
| vpe_load_graph(json)  | Compiles and registers a graph; returns a success boolean.                 |
| vpe_evaluate_ffi(...) | Passes pointers for strings/JSON; returns a pointer to FfiVerdict.         |
| vpe_free_verdict(ptr) | CRITICAL: Frees the Rust-allocated memory for the verdict and its strings. |
## The Comprehensive JSON Schema

This JSON represents "The Law" for a specific domain (e.g., OrderManagement). It includes branching, temporal guards, and migration rules.

```JSON
{
  "domain": "OrderManagement",
  "version": "2.0.0",
  "supersedes": "1.0.0",
  "initial_state": "Draft",

  "migration_rules": [
    {
      "comment": "Move V1 'Pending' to V2 'AwaitingTax' if TaxID is missing",
      "from_state": "Pending",
      "to_state": "AwaitingTaxInfo",
      "guards": [{ "type": "MissingField", "path": "entity.TaxID" }],
      "transforms": [
        { "target": "entity.LegacyMode", "value": true }
      ]
    }
  ],

  "states": [
    {
      "name": "AwaitingPayment",
      "transitions": [
        {
          "action": "SubmitPayment",
          "to": "Processing",
          "priority": 1,
          "guards": [
            { 
              "type": "OccurredWithin", 
              "target_action": "CardValidation", 
              "window_seconds": 3600 
            }
          ],
          "effects": [
            {
              "type": "CrossDomain",
              "target": "Accounting",
              "action": "Debit",
              "on_success": "Confirm",
              "on_failure": "Reject",
              "on_timeout": "HandleStale"
            }
          ]
        }
      ]
    },
    { "name": "Processing" },
    { "name": "Closed" }
  ]
}
```

### JSON Field Explanations:
- supersedes: Tells the Migration Engine which version this DAG is allowed to "lift" records from.
- migration_rules: A list of "Mapings." If a record comes in on an old version, these rules tell the engine how to transform the data and choose the new starting state.
- priority: If multiple transitions match the same action (e.g., two "Submit" paths), the engine picks the one with the highest number.
- guards: The "Gatekeepers."
    - OccurredWithin: A Temporal Guard that scans the history for a specific event.
- effects: The "Orders."
on_success / on_failure: These are the Saga Callbacks. The Compiler ensures the to state (Processing) has handlers for these specific strings.

## Summary of Invariants for the API
1. Immutability: Once a DAG is registered in the VpeEngine, it cannot be modified. You must register a new version.
2. Memory Safety: The FFI bridge uses Box::into_raw to hand off memory. The caller owns the responsibility to call the free function.
3. Strict Typing: The JSON is strictly validated against a Schema during the register_process call. If the JSON is "loose," the Rust compiler rejects it.