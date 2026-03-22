# VPE CLI Specification
Version: Canonical v1

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

---

## 2. Design Principles

1. The CLI is a thin wrapper over the Rust library.
2. JSON is the canonical input and output format.
3. All commands are deterministic given explicit inputs.
4. Commands map directly to core VPE capabilities.
5. The CLI exposes domain concepts, not internal structures.
6. The CLI is scriptable via stdin/stdout.
7. Human-friendly output is optional and secondary.

---

## 3. Command Overview

The CLI exposes the following commands:

- validate
- compile
- manifest
- execute
- simulate
- lift

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

### Compiled Mode (Future)

Uses:
- compiled artifact

Example:

vpe execute --compiled process.vpe --request request.json

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

---

## 8. Error Model

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

---

## 9. Human-Friendly Output (Optional)

Future flags:

--pretty  
--summary  

These must not replace JSON output.

---

## 10. Extensibility

The CLI must support future additions without breaking existing commands.

Possible future commands:

- inspect
- export
- verify
- diff

---

## 11. Summary

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