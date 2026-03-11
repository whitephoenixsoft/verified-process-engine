pub struct TimeElapsedGuard {
    pub seconds: u64,
}

impl Guard for TimeElapsedGuard {
    fn check(&self, context: &ContextMap, history: &[VpeEvent]) -> bool {
        // 1. Get current time from the seeded 'sys' namespace
        let now = context.get("sys.now").and_then(|v| v.as_u64()).unwrap_or(0);
        
        // 2. Get the timestamp of the 'Anchor' (Last State Transition)
        let last_time = history.last().map(|e| e.timestamp).unwrap_or(0);

        // 3. If enough time has passed, the guard returns True
        (now - last_time) >= self.seconds
    }

    fn get_requirements(&self) -> Vec<HistoryRequirement> {
        vec![HistoryRequirement::LastTransition] // Only needs the Anchor!
    }
}
