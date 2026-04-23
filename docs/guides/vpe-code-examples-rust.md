# VPE Rust Code Usage Examples (Baseline)
Version: Draft v1

## Purpose
This document provides concrete Rust examples of how to embed and use VPE in real applications.

These examples are intended as:
- a baseline for API validation
- a reference for developers integrating VPE
- a sanity check for ergonomics before deeper implementation

---

# Example 1: Web Application Embed

## Startup

```rust
use serde_json::json;
use std::collections::HashMap;
use vpe::prelude::*;

fn build_engine(schema_json: &str, law_json: &str) -> Result<VpeEngine, VpeError> {
    let registry = GuardRegistry::builder()
        .with_builtins()
        .build()?;

    let engine = VpeEngine::builder()
        .with_registry(registry)
        .build()?;

    engine.register_schema_json(schema_json)?;
    engine.register_process_json(law_json)?;

    Ok(engine)
}
```
## Domain Models
```rust
#[derive(Debug, Clone)]
struct LoanRecord {
    loan_id: String,
    applicant_id: String,
    current_state: String,
    anchor_event_id: String,
    amount: i64,
    credit_score: i32,
}

#[derive(Debug, Clone)]
struct EvaluateLoanHttpRequest {
    trace_id: String,
    member_tier: String,
    requested_at_epoch_secs: i64,
}
```

## Context Adapter
```rust
use serde_json::Value;

fn build_context(record: &LoanRecord, req: &EvaluateLoanHttpRequest) -> ContextMap {
    let mut ctx = ContextMap::new();
    ctx.insert("rec.loan_id".into(), Value::String(record.loan_id.clone()));
    ctx.insert("rec.applicant_id".into(), Value::String(record.applicant_id.clone()));
    ctx.insert("rec.amount".into(), Value::from(record.amount));
    ctx.insert("ext.member_tier".into(), Value::String(req.member_tier.clone()));
    ctx.insert("ext.credit_score".into(), Value::from(record.credit_score));
    ctx.insert("sys.now".into(), Value::from(req.requested_at_epoch_secs));
    ctx
}
```

## Repository Contracts
```rust
trait LoanRepo {
    fn load_record(&self, loan_id: &str) -> Result<LoanRecord, String>;
    fn load_chronicle(&self, loan_id: &str, manifest: &StateManifest) -> Result<ChronicleView, String>;
    fn commit_turn(
        &self,
        loan_id: &str,
        expected_anchor_event_id: &str,
        verdict: &VpeVerdict,
    ) -> Result<(), String>;
}

trait EffectDispatcher {
    fn dispatch_all(&self, effects: &[VpeEffect]) -> Result<(), String>;
}
```

## Handler
```rust
fn evaluate_loan(
    engine: &VpeEngine,
    repo: &dyn LoanRepo,
    dispatcher: &dyn EffectDispatcher,
    loan_id: &str,
    req: EvaluateLoanHttpRequest,
) -> Result<VpeVerdict, String> {
    let process = ProcessRef::new("Lending", "LoanApproval", "2.1.0");

    let record = repo.load_record(loan_id)?;
    let manifest = engine.manifest(&process, &record.current_state)
        .map_err(|e| e.to_string())?;

    let chronicle = repo.load_chronicle(loan_id, &manifest)?;
    let context = build_context(&record, &req);

    let verdict = engine.execute(VpeRequest {
        process,
        trace_id: req.trace_id.clone(),
        now: req.requested_at_epoch_secs,
        current_state: record.current_state.clone(),
        action: "Evaluate".into(),
        context,
        chronicle,
    }).map_err(|e| e.to_string())?;

    repo.commit_turn(loan_id, &record.anchor_event_id, &verdict)?;
    dispatcher.dispatch_all(&verdict.effects)?;

    Ok(verdict)
}
```
---

# Example 2: Event-Sourced Service

## Projection
```rust
#[derive(Debug, Clone)]
struct OrderProjection {
    order_id: String,
    current_state: String,
    anchor_event_id: String,
    total_amount: i64,
    currency: String,
}
```
## Stores
```rust
trait ProjectionStore {
    fn load_projection(&self, order_id: &str) -> Result<OrderProjection, String>;
}

trait EventStore {
    fn load_for_vpe(
        &self,
        order_id: &str,
        manifest: &StateManifest,
    ) -> Result<ChronicleView, String>;

    fn append_with_expected_anchor(
        &self,
        order_id: &str,
        expected_anchor_event_id: &str,
        events: &[PlannedEvent],
    ) -> Result<(), String>;
}
```
## Command
```rust
#[derive(Debug, Clone)]
struct SubmitPaymentCommand {
    trace_id: String,
    timestamp: i64,
    action: String,
}

## Context Adapter

use serde_json::Value;

fn build_order_context(
    projection: &OrderProjection,
    command: &SubmitPaymentCommand,
) -> ContextMap {
    let mut ctx = ContextMap::new();
    ctx.insert("rec.order_id".into(), Value::String(projection.order_id.clone()));
    ctx.insert("rec.total_amount".into(), Value::from(projection.total_amount));
    ctx.insert("rec.currency".into(), Value::String(projection.currency.clone()));
    ctx.insert("sys.now".into(), Value::from(command.timestamp));
    ctx
}
```

## Handler

```rust
fn handle_submit_payment(
    engine: &VpeEngine,
    projection_store: &dyn ProjectionStore,
    event_store: &dyn EventStore,
    order_id: &str,
    command: SubmitPaymentCommand,
) -> Result<VpeVerdict, String> {
    let process = ProcessRef::new("OrderManagement", "PaymentFlow", "2.0.0");

    let projection = projection_store.load_projection(order_id)?;
    let manifest = engine.manifest(&process, &projection.current_state)
        .map_err(|e| e.to_string())?;

    let chronicle = event_store.load_for_vpe(order_id, &manifest)?;
    let context = build_order_context(&projection, &command);

    let verdict = engine.execute(VpeRequest {
        process,
        trace_id: command.trace_id.clone(),
        now: command.timestamp,
        current_state: projection.current_state.clone(),
        action: command.action.clone(),
        context,
        chronicle,
    }).map_err(|e| e.to_string())?;

    event_store.append_with_expected_anchor(
        order_id,
        &projection.anchor_event_id,
        &verdict.emitted_events,
    )?;

    Ok(verdict)
}
```

---

# Example 3: Workflow / Orchestration Bridge

## Workflow Task
```rust
use serde_json::Value;

#[derive(Debug, Clone)]
struct WorkflowTask {
    trace_id: String,
    effect_type: String,
    target: Option<String>,
    action: Option<String>,
    params: serde_json::Map<String, Value>,
}

trait WorkflowRuntime {
    fn enqueue(&self, task: WorkflowTask) -> Result<(), String>;
}
```
## Initial Turn
```rust
fn submit_payment_for_processing(
    engine: &VpeEngine,
    workflow: &dyn WorkflowRuntime,
    process: ProcessRef,
    current_state: String,
    trace_id: String,
    now: i64,
    chronicle: ChronicleView,
    mut context: ContextMap,
) -> Result<VpeVerdict, String> {
    context.insert("sys.now".into(), Value::from(now));

    let verdict = engine.execute(VpeRequest {
        process,
        trace_id: trace_id.clone(),
        now,
        current_state,
        action: "SubmitPayment".into(),
        context,
        chronicle,
    }).map_err(|e| e.to_string())?;

    for effect in &verdict.effects {
        workflow.enqueue(WorkflowTask {
            trace_id: verdict.trace_id.clone(),
            effect_type: effect.effect_type.clone(),
            target: effect.target.clone(),
            action: effect.action.clone(),
            params: effect.params.clone(),
        })?;
    }

    Ok(verdict)
}
```
## Callback Turn
```rust
#[derive(Debug, Clone)]
struct GatewayCallback {
    trace_id: String,
    timestamp: i64,
    success: bool,
    gateway_reference: String,
}

fn handle_gateway_callback(
    engine: &VpeEngine,
    process: ProcessRef,
    current_state: String,
    chronicle: ChronicleView,
    callback: GatewayCallback,
) -> Result<VpeVerdict, String> {
    let mut context = ContextMap::new();
    context.insert("sys.now".into(), Value::from(callback.timestamp));
    context.insert(
        "ext.gateway_reference".into(),
        Value::String(callback.gateway_reference),
    );

    let action = if callback.success {
        "GATEWAY_SUCCESS"
    } else {
        "GATEWAY_FAILURE"
    };

    engine.execute(VpeRequest {
        process,
        trace_id: callback.trace_id,
        now: callback.timestamp,
        current_state,
        action: action.into(),
        context,
        chronicle,
    }).map_err(|e| e.to_string())
}
```
---

# Summary

Across all examples, the pattern is consistent:

1. Ask for manifest
2. Load minimal history
3. Build context
4. Execute VPE
5. Persist result
6. Dispatch effects

This consistency is intentional.

It is the foundation for making VPE feel natural and reusable across:
- web applications
- event-sourced systems
- workflow engines
- distributed orchestration systems