fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. SETUP: The Logic Registry
    // We tell the engine what "Equals" means.
    let mut registry = GuardRegistry::new();
    registry.register("Equals", |params| {
        let path = params["path"].as_str().ok_or("Missing path")?.to_string();
        let value = params["value"].clone();
        Ok(Box::new(EqualsGuard { path, expected: value }))
    });

    // 2. GOVERNANCE: The Domain Schema
    // We define what an "Order" is allowed to look like.
    let mut order_schema = DomainSchema::new();
    order_schema.add_field("rec", "order.total", DataType::Number);
    order_schema.add_field("rec", "order.status", DataType::String);

    // 3. INITIALIZE: The Engine Facade
    let engine = VpeEngine::new(Arc::new(registry));
    engine.register_schema("OrderManagement", order_schema);

    // 4. REGISTRATION: The "Law" (JSON)
    // This is typically loaded from a file or DB.
    let process_json = r#"{
        "domain": "OrderManagement",
        "version": "1.0.0",
        "initial_state": "Draft",
        "states": [
            {
                "name": "Draft",
                "transitions": [
                    {
                        "action": "Submit",
                        "to": "Review",
                        "guards": [
                            { "type": "Equals", "path": "rec.order.status", "value": "Valid" },
                            { "type": "Equals", "path": "sys.is_maintenance", "value": false }
                        ]
                    }
                ]
            },
            { "name": "Review" }
        ]
    }"#;

    engine.register_process(process_json)?;

    // 5. EXECUTION: The Request
    // This data usually comes from your .NET/Go Host.
    let mut context = HashMap::new();
    context.insert("rec.order.status".to_string(), Value::String("Valid".into()));
    context.insert("sys.is_maintenance".to_string(), Value::Bool(false));

    let history = vec![]; // No history needed for this simple guard

    // 6. THE CALL: Get the Verdict
    let verdict = engine.execute(
        "OrderManagement", // Domain
        "1.0.0",           // Version
        "Draft",           // Current State
        "Submit",          // Action
        context,
        history
    )?;

    // 7. RESULT: Handle the outcome
    println!("Transition Successful!");
    println!("New State: {}", verdict.next_state_name);
    println!("Triggered Effects: {:?}", verdict.effects);

    Ok(())
}
