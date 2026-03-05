# VPE COMPONENT API SPECIFICATION

## 1. THE LOGIC REGISTRY (GuardRegistry)
This module acts as the "Linker." It stores the definitions of what the engine "knows" how to do.

- GuardRegistry::new(): Creates a fresh registry with core system guards (e.g., Equals, Exists).
- GuardRegistry::register_custom(id: &str, factory: Fn): Allows developers to inject domain-specific logic.
- GuardRegistry::get_implementation(id: &str): Returns the trait object for the Compiler to bind into the DAG.

## 2. THE COMPILER MODULE (VpeCompiler)
The Compiler is a stateful service that requires the GuardRegistry to "hydrate" the JSON into a runnable graph.

- VpeCompiler::new(registry: Arc<GuardRegistry>): Initialized with the available logic blueprints.
- VpeCompiler::validate(json: &str): Performs a "Dry Parse" to catch syntax and topological errors.
- VpeCompiler::compile(json: &str): The "Assembler." It produces a VpeDag where all strings are resolved to indices and trait pointers.

## 3. THE DAG STORAGE (ProcessStore)
This is where the "Laws" live once they are compiled.

- ProcessStore::insert(domain: &str, version: &str, dag: VpeDag): Adds a new law to the library.
- ProcessStore::get(domain: &str, version: &str): Retrieves a specific version of a process.
- ProcessStore::get_latest(domain: &str): Retrieves the most recent version for new records.

## 4. THE RUNTIME & EVALUATOR (VpeRuntime)
The high-speed execution core.

- VpeRuntime::evaluate(dag, current_state_idx, action, context, history): The O(1) loop that returns a Verdict.
- VpeRuntime::check_guards(edge, context, history): Internal helper that short-circuits if any guard fails.

## 5. THE MIGRATION & TRANSFORM MODULE (MigrationEngine)
Handles "Lifting" records between versions of the law.

- MigrationEngine::needs_lift(record_version, target_version): Boolean check for version drift.
- MigrationEngine::transform(context, transforms_json): Reshapes data (e.g., merging fields) based on migration rules.
- MigrationEngine::lift(record, target_dag): Runs Migration Guards to determine the "Landing State" in the new version.

## 6. THE DRY-RUN MODULE (SimulationEngine)
Used for pre-deployment impact analysis.

- SimulationEngine::replay_history(target_dag, history, initial_context): Re-runs every past event against new rules.
- SimulationEngine::analyze_impact(records_sample, new_dag): Produces a report of Seamless vs. Incompatible migrations.

## 7. THE CORE FACADE (VpeEngine)
The primary entry point that orchestrates all other modules.
```Rust
pub struct VpeEngine {
    guards: Arc<GuardRegistry>,
    processes: Arc<ProcessStore>,
    compiler: VpeCompiler,
}

impl VpeEngine {
    pub fn register_process(&self, json: &str) -> Result<(), VpeError>;
    pub fn execute(&self, request: VpeRequest) -> Result<VpeVerdict, VpeError>;
    pub fn simulate(&self, domain: &str, target_version: &str, data: VpeSnapshot) -> SimulationReport;
}
```
## 8. THE FFI BRIDGE (The "C" Interface)
The binary gateway for .NET, Go, and Python.

- vpe_init(): Returns a pointer to the VpeEngine.
- vpe_load_law(engine_ptr, json): Compiles and stores a process.
- vpe_evaluate_ffi(engine_ptr, domain, action, ...): The main execution bridge.
- vpe_free_verdict(ptr): The mandatory memory cleanup for FFI-allocated results.

## 9. THE COMPREHENSIVE JSON SCHEMA ("The Law")
This represents a single version of a process flow.
```JSON
{
  "domain": "OrderManagement",
  "version": "2.0.0",
  "supersedes": ["1.0.0", "1.1.0"],
  "initial_state": "Draft",

  "migration_rules": [
    {
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
    }
  ]
}
```
## 10. SUMMARY OF INVARIANTS

- Decoupled Registry: The Logic (Guards) is defined once at start-up; the Laws (DAGs) are loaded dynamically.
- Index-Based DAG: Once compiled, all state references are usize indices, making the Runtime branch-prediction friendly.
- Forced Cleanup: Any FFI-allocated struct has a corresponding vpe_free_* function to prevent memory leaks in the host language.
- Determinism: The Runtime never queries an external DB. All data must be in the Context or History.