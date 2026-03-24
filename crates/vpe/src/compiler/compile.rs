use crate::compiler::compiled::CompiledProcess;
use crate::compiler::digest::compute_digest;
use crate::compiler::source::LawSource;
use crate::compiler::validate::validate_law;
use crate::error::{CompileError, VpeError};
use crate::registry::GuardRegistry;
use crate::schema::DomainSchema;
use crate::types::{ProcessRef, StateManifest};
use std::collections::HashMap;

pub struct VpeCompiler {
    registry: GuardRegistry,
}

pub struct CompilationResult {
    pub process: CompiledProcess,
    pub report: RegistrationReport,
}

#[derive(Debug, Clone)]
pub struct RegistrationReport {
    pub process: ProcessRef,
    pub digest: String,
    pub manifests: HashMap<String, StateManifest>,
    pub warnings: Vec<String>,
}

impl VpeCompiler {
    pub fn with_registry(registry: GuardRegistry) -> Self {
        Self { registry }
    }

    pub fn validate(&self, schema: &DomainSchema, law: &LawSource) -> Result<(), VpeError> {
        let _ = &self.registry;
        validate_law(schema, law)?;
        Ok(())
    }

    pub fn compile(
        &self,
        schema: &DomainSchema,
        law: &LawSource,
    ) -> Result<CompilationResult, VpeError> {
        self.validate(schema, law)?;

        let process_ref = ProcessRef::new(&law.domain, &law.process, &law.version);
        let manifests = HashMap::<String, StateManifest>::new();
        let digest = compute_digest(law)?;

        let process = CompiledProcess::new(process_ref.clone(), digest.clone(), manifests.clone());
        let report = RegistrationReport {
            process: process_ref,
            digest,
            manifests,
            warnings: vec![],
        };

        Ok(CompilationResult { process, report })
    }
}
