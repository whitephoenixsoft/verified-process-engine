use crate::types::{ProcessRef, StateManifest};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CompiledProcess {
    process_ref: ProcessRef,
    digest: String,
    manifests: HashMap<String, StateManifest>,
}

impl CompiledProcess {
    pub fn new(
        process_ref: ProcessRef,
        digest: String,
        manifests: HashMap<String, StateManifest>,
    ) -> Self {
        Self {
            process_ref,
            digest,
            manifests,
        }
    }

    pub fn process_ref(&self) -> &ProcessRef {
        &self.process_ref
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn manifest(&self, state: &str) -> Option<&StateManifest> {
        self.manifests.get(state)
    }
}
