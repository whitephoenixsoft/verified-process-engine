

pub struct SimulationReport {
    pub trace_id: String,
    pub is_seamless: bool,
    pub original_final_state: String,
    pub simulated_final_state: String,
    pub breaking_guards: Vec<String>, // Which guards failed that used to pass?
}

impl SimulationEngine {
    pub fn replay_history(
        new_dag: &VpeDag,
        history: &[VpeEvent],
        initial_context: ContextMap
    ) -> SimulationReport {
        let mut current_state_idx = new_dag.initial_state_idx;
        let mut current_context = initial_context;
        let mut seamless = true;
        let mut failures = Vec::new();

        for event in history {
            // Try to find the transition in the NEW law using the OLD event
            match VpeRuntime::evaluate(new_dag, current_state_idx, &event.action, &current_context, history) {
                Ok(verdict) => {
                    current_state_idx = verdict.next_state_idx;
                    // Apply context updates if the verdict has effects
                },
                Err(e) => {
                    seamless = false;
                    failures.push(e);
                    break; // Simulation "Stuck"
                }
            }
        }

        SimulationReport {
            trace_id: history.first().map(|e| e.trace_id.clone()).unwrap_or_default(),
            is_seamless: seamless,
            original_final_state: "??".to_string(), // Fetched from record
            simulated_final_state: new_dag.nodes[current_state_idx].name.clone(),
            breaking_guards: failures,
        }
    }
}
