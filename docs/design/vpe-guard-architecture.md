# VPE Guard Compilation Architecture
Version: Canonical v1 (Forward Design)

## Purpose

This document defines the long-term architecture for compiling guards in VPE.

It addresses:
- how guards participate in compilation
- how manifests are derived
- how custom guards achieve parity with built-in guards
- how to avoid hardcoding logic in the compiler

---

## 1. Problem Statement

Early implementations of VPE may include compiler logic such as:

- matching guard types by string
- manually validating parameters per guard
- manually deriving manifest requirements

Example (anti-pattern):

if guard_type == "Equals" {
    validate_equals(...)
}

Problems:
- not extensible
- duplicates logic
- custom guards cannot participate equally
- compiler becomes tightly coupled to built-ins

---

## 2. Design Goal

All guards — built-in or custom — must:

- validate themselves at compile time
- declare their own requirements
- construct their runtime representation

The compiler should:
- orchestrate
- not specialize

---

## 3. Core Concept: Guard Definition

Each guard is defined by a compile-time + runtime pair.

Conceptual structure:

GuardDefinition:
- name
- parameter validation
- requirement derivation
- runtime construction

---

## 4. Conceptual Traits

Compile-time:

trait GuardDefinition {
    fn name(&self) -> &'static str;

    fn validate(
        &self,
        params: &Value,
        schema: &DomainSchema
    ) -> Result<GuardValidation, CompileError>;

    fn build(
        &self,
        params: &Value
    ) -> Result<Box<dyn Guard>, CompileError>;
}

---

Runtime:

trait Guard {
    fn check(&self, context: &ContextMap, history: &[Event]) -> bool;
}

---

Validation output:

GuardValidation:
- requirements (context + history)
- warnings

---

## 5. Compilation Flow

For each guard in a transition:

1. Lookup GuardDefinition in registry
2. Call validate(params, schema)
3. Receive:
   - validated parameters
   - requirements
   - warnings
4. Add requirements to state manifest
5. Call build(params) to create runtime guard
6. Store compiled guard in CompiledProcess

---

## 6. Manifest Derivation

Manifest is built from guard requirements.

Example:

Guard: Equals  
params:
- path: rec.amount

Result:
- manifest requires rec.amount

---

Example:

Guard: FieldsEqual  
params:
- left_path: rec.a
- right_path: rec.b

Result:
- manifest requires rec.a and rec.b

---

## 7. Built-in Guards

Built-in guards are implemented using GuardDefinition.

They are registered in the registry at startup.

Example:

registry.register(EqualsDefinition)
registry.register(FieldsEqualDefinition)

The compiler treats them exactly like custom guards.

---

## 8. Custom Guards

Users implement GuardDefinition and register it.

Example:

struct MyCustomGuard;

impl GuardDefinition for MyCustomGuard {
    fn name(&self) -> &'static str {
        "MyCustomGuard"
    }

    fn validate(...) -> ... {
        // validate params
        // return requirements
    }

    fn build(...) -> ... {
        // return runtime guard
    }
}

Result:
- full compile-time validation
- full manifest integration
- no compiler changes required

---

## 9. Benefits

### Extensibility
No compiler modification required for new guards.

---

### Consistency
Built-in and custom guards behave identically.

---

### Determinism
All requirements are declared and verified at compile time.

---

### Maintainability
Compiler remains simple and orchestration-focused.

---

## 10. Transitional Strategy

Short-term (current phase):
- compiler may include limited hardcoded validation

Mid-term:
- migrate built-in guards into GuardDefinition implementations

Long-term:
- compiler contains zero guard-specific logic

---

## 11. Design Principles

1. Compiler orchestrates, guards validate
2. All dependencies must be explicit
3. No hidden data access
4. No special cases for built-ins
5. Registry is the single source of guard behavior

---

## 12. Future Considerations

- static typing for guard params
- macro-based guard definitions
- compile-time code generation
- WASM or plugin-based guards (optional future)

---

## 13. Summary

The long-term VPE architecture requires:

- guards to own their validation and requirements
- the compiler to remain generic
- the registry to act as the extension boundary

This ensures:
- scalability
- extensibility
- clean separation of concerns
- first-class support for custom logic