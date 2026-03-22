# VPE CLI Specification
Version: Canonical v1.1

## 1. Purpose

This document defines the command-line interface (CLI) for the Verified Process Engine (VPE).

The CLI provides a **universal harness** for:
- authoring
- validation
- compilation
- inspection
- execution
- simulation
- migration

It is designed to be:
- deterministic
- scriptable
- JSON-first
- thin over the Rust API
- suitable for CI/CD and local development
- extensible via custom CLI harnesses

---

## 2. Design Principles

1. The CLI is a thin wrapper over the Rust library.
2. JSON is the canonical input and output format.
3. All commands are deterministic given explicit inputs.
4. Commands map directly to core VPE capabilities.
5. The CLI exposes domain concepts, not internal structures.
6. The CLI is scriptable via stdin/stdout.
7. Human-friendly output is optional and secondary.
8. The CLI must support both **design-time workflows** and **runtime evaluation**.
9. The CLI must not diverge from Rust API semantics.

---

## 3. Command Overview

The CLI exposes the following commands:

- validate
- compile
- manifest
- execute
- simulate
- lift

Each command corresponds directly to a Rust API capability.

---

## 4. Input Modes

### File Input

Inputs are provided via file paths:

--schema schema.json  
--law law.json  
--request request.json  
--input input.json  

---

### Standard Input

The CLI must support stdin using `-`:

--request -  

Example:

cat request.json | vpe execute --schema schema.json --law law.json --request -

---

## 5. Output Format

### Default Output

All commands output JSON to stdout.

Standard envelope:

{
  "success": true,
  "data": {},
  "warnings": [],
  "errors": []
}

---

### Failure Output

{
  "success": false,
  "data": null,
  "warnings": [],
  "errors": [
    {
      "code": "ERROR_CODE",
      "message": "Description of failure"
    }
  ]
}

---

### Exit Codes

- 0 → success  
- non-zero → failure  

---

## 6. Source Mode vs Compiled Mode

### Source Mode

Uses:
- schema
- law

Example:

vpe compile --schema schema.json --law law.json

---

### Compiled Mode (Planned)

Uses:
- compiled artifact

Example:

vpe execute --compiled process.vpe --request request.json

---

### Notes

1. Source mode must always perform validation before execution.
2. Compiled mode must only accept validated artifacts.
3. Compiled artifacts must include:
   - digest
   - version metadata
   - compatibility markers

---

## 7. Command Specifications

---

### 7.1 validate

Validates schema and law without compiling.

Command:

vpe validate --schema schema.json --law law.json

Output:

{
  "success": true,
  "data": {
    "process": {
      "domain": "OrderManagement",
      "version": "1.0.0"
    }
  },
  "warnings": [],
  "errors": []
}

Notes:
- Performs schema validation
- Performs law validation
- Resolves guard types via registry
- Does not produce compiled artifacts

---

### 7.2 compile

Validates and compiles a process.

Command:

vpe compile --schema schema.json --law law.json

Output:

{
  "success": true,
  "data": {
    "process": {
      "domain": "OrderManagement",
      "version": "1.0.0"
    },
    "digest": "abc123...",
    "manifest": {
      "Draft": {
        "history": ["LastTransition"],
        "context": ["rec.order_total"]
      }
    }
  },
  "warnings": [],
  "errors": []
}

Notes:
- Produces deterministic digest
- Produces per-state manifests
- May later support artifact export

---

### 7.3 manifest

Retrieves manifest for a specific state.

Command:

vpe manifest --schema schema.json --law law.json --state Draft

Output:

{
  "success": true,
  "data": {
    "state": "Draft",
    "history_requirements": ["LastTransition"],
    "context_requirements": ["rec.order_total"]
  },
  "warnings": [],
  "errors": []
}

Notes:
- Uses compiler pipeline internally
- Does not require runtime execution

---

### 7.4 execute

Executes one engine turn.

Command:

vpe execute --schema schema.json --law law.json --request request.json

---

### Request Format

{
  "process": {
    "domain": "OrderManagement",
    "version": "1.0.0"
  },
  "trace_id": "abc-123",
  "now": 1700000000,
  "current_state": "Draft",
  "action": "Submit",
  "context": {
    "rec.order_total": 500
  },
  "chronicle": {
    "anchor": {
      "state_after": "Draft",
      "timestamp": 1700000000
    },
    "events": []
  }
}

---

### Output

{
  "success": true,
  "data": {
    "previous_state": "Draft",
    "next_state": "Submitted",
    "effects": [],
    "emitted_events": []
  },
  "warnings": [],
  "errors": []
}

Notes:
- Executes exactly one deterministic turn
- Validates anchor consistency
- Does not execute side effects

---

### 7.5 simulate

Runs simulation over historical input.

Command:

vpe simulate --schema schema.json --law law.json --input simulation.json

---

### Input Format

{
  "process": {
    "domain": "OrderManagement",
    "version": "1.0.0"
  },
  "initial_state": "Draft",
  "initial_context": {},
  "history": []
}

---

### Output

{
  "success": true,
  "data": {
    "outcome": "Seamless",
    "original_final_state": "Approved",
    "simulated_final_state": "Approved",
    "divergence": null
  },
  "warnings": [],
  "errors": []
}

Notes:
- Uses same runtime logic
- Does not execute effects
- Uses event timestamps as time source

---

### 7.6 lift

Performs migration/lift operation.

Command:

vpe lift --schema schema.json --law law.json --input lift.json

---

### Input Format

{
  "target_process": {
    "domain": "OrderManagement",
    "version": "2.0.0"
  },
  "current_state": "Pending",
  "context": {},
  "history": []
}

---

### Output

{
  "success": true,
  "data": {
    "previous_state": "Pending",
    "new_state": "AwaitingPayment",
    "context": {}
  },
  "warnings": [],
  "errors": []
}

Notes:
- Applies migration rules deterministically
- Produces transformed context
- Does not mutate history

---

## 8. Custom Guard Support

### Official CLI

1. Supports built-in guards only.
2. Unknown guard types result in validation failure.
3. No dynamic plugin system is required in v1.

---

### Custom CLI Harness

Projects may create their own CLI binary:

- register custom guards
- reuse CLI command modules
- extend registry during startup

Example (conceptual):

custom-vpe-cli
  ├── main.rs
  ├── guards/
  │   ├── my_guard.rs
  │   └── mod.rs

Rules:
1. Must use GuardRegistryBuilder
2. Must not modify VPE semantics
3. Must remain deterministic

---

## 9. Error Model

Errors must include:

- code
- message

Optional:
- path
- context
- suggestion

Example:

{
  "code": "TYPE_MISMATCH",
  "message": "Field rec.amount expects Number"
}

Error categories:
- validation
- compile
- runtime
- simulation
- migration

---

## 10. Human-Friendly Output (Optional)

Future flags:

--pretty  
--summary  

Rules:
1. Must not replace JSON output
2. Must not alter data semantics
3. Must remain deterministic

---

## 11. Extensibility

The CLI must support future additions without breaking existing commands.

Possible future commands:

- inspect
- export
- verify
- diff

Future capabilities:
- compiled artifact export/import
- interactive debugging
- law diffing

---

## 12. Summary

The VPE CLI is:

- a deterministic harness
- a JSON-first interface
- a thin layer over core logic
- a bridge between humans and the engine

It enables:
- rapid iteration
- safe validation
- reproducible execution
- system-level reasoning