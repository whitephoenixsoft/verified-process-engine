
use crate::registry::Guard;
use crate::types::{ProcessRef, StateManifest, VpeEffect};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CompiledProcess {
    process_ref: ProcessRef,
    digest: String,
    manifests: HashMap<String, StateManifest>,
    initial_state_idx: usize,
    nodes: Vec<Node>,
    state_index: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub transitions: Vec<Edge>,
}

#[derive(Debug)]
pub struct Edge {
    pub action: String,
    pub target_idx: usize,
    pub priority: u32,
    pub guards: Vec<Box<dyn Guard>>,
    pub effects: Vec<VpeEffect>,
}

impl CompiledProcess {
    pub fn new(
        process_ref: ProcessRef,
        digest: String,
        manifests: HashMap<String, StateManifest>,
        initial_state_idx: usize,
        nodes: Vec<Node>, 
            state_index: HashMap<String, usize>,
    ) -> Self {
        Self {
            process_ref,
            digest,
            manifests,
            initial_state_idx,
            nodes,
                state_index,
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

    pub fn initial_state_idx(&self) -> usize {
        self.initial_state_idx
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
    
    pub fn state_index(&self, state: &str) -> Option<usize> {
        self.state_index.get(state).copied()
    }
}