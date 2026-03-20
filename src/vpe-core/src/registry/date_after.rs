/// Checks if a record field is after a specific point in time
pub struct DateAfterGuard {
    pub path: String,       // e.g., "rec.trial_end_date"
    pub compare_to: String, // e.g., "sys.now"
}

impl Guard for DateAfterGuard {
    fn check(&self, context: &ContextMap, _history: &[VpeEvent]) -> bool {
        let field_val = context.get(&self.path).and_then(|v| v.as_u64()).unwrap_or(0);
        let now_val = context.get(&self.compare_to).and_then(|v| v.as_u64()).unwrap_or(0);

        // Logic: Is the record's date already in the past?
        field_val < now_val
    }

    fn get_requirements(&self) -> Vec<HistoryRequirement> {
        // No history needed, just current context
        vec![]
    }
}
