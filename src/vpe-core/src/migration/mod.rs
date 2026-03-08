pub struct MigrationEngine;

pub struct MigrationRule {
    pub from_state: String,
    pub to_state: String,
    pub migration_guards: Vec<Box<dyn Guard>>,
    pub transforms: Vec<TransformOp>,
}

impl MigrationEngine {
    pub fn lift(
        current_state: &str,
        context: &ContextMap,
        history: &[VpeEvent],
        migration_rules: &[MigrationRule]
    ) -> Result<(String, ContextMap), String> {
        // 1. Find all rules that apply to our current state
        for rule in migration_rules.iter().filter(|r| r.from_state == current_state) {
            
            // 2. Run Migration Guards
            // These check if the record "qualifies" for this specific path into V2
            let passes = rule.migration_guards.iter().all(|g| g.check(context, history));

            if passes {
                // 3. Apply Transformations
                let new_context = Self::transform(context.clone(), &rule.transforms)?;
                
                // 4. Return the new "Landing State" and the reshaped context
                return Ok((rule.to_state.clone(), new_context));
            }
        }

        Err(format!("Migration Failed: No valid migration path found for state '{}'", current_state))
    }
}

    pub fn needs_lift(record_version: &str, target_version: &str) -> bool {
        // Simple string comparison or SemVer logic
        record_version != target_version
    }

    pub fn transform(
        mut context: ContextMap, 
        rules: &[TransformOp]
    ) -> Result<ContextMap, String> {
        for op in rules {
            match op {
                TransformOp::Move { from, to } => {
                    if let Some(val) = context.remove(from) {
                        context.insert(to.clone(), val);
                    }
                },
                TransformOp::Set { target, value } => {
                    context.insert(target.clone(), Value::from_json(value.clone()));
                },
                TransformOp::Map { target, from, mapping } => {
                    if let Some(val) = context.get(from) {
                        let key = val.to_string(); // Convert Value to string for lookup
                        if let Some(new_val) = mapping.get(&key) {
                            context.insert(target.clone(), Value::from_json(new_val.clone()));
                        }
                    }
                }
            }
        }
        Ok(context)
    }

    // During Migration Rule Registration
    fn validate_migration_rule(rule: &RawMigrationRule) -> Result<(), String> {
        for op in &rule.transforms {
            match op {
                RawOp::Move { from, to } => {
                    compiler.validate_identifier_structure(from)?;
                    compiler.validate_identifier_structure(to)?;
                    compiler.validate_write_path(to, domain)?; // Cannot write to sys.*
                },
                // ... etc
            }
        }
        Ok(())
    }

}
