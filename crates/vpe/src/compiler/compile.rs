use crate::compiler::compiled::CompiledProcess;
use crate::compiler::digest::compute_digest;
use crate::compiler::manifest::build_manifests;
use crate::compiler::source::LawSource;
use crate::compiler::validate::validate_law;
use crate::error::VpeError;
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
pub struct ValidationReport {
    pub process: ProcessRef,
    pub warnings: Vec<String>,
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

    pub fn validate(
        &self,
        schema: &DomainSchema,
        law: &LawSource,
    ) -> Result<ValidationReport, VpeError> {
        let warnings = validate_law(schema, law, &self.registry)?;
        let process = ProcessRef::new(&law.domain, &law.process, &law.version);

        Ok(ValidationReport { process, warnings })
    }

    pub fn compile(
        &self,
        schema: &DomainSchema,
        law: &LawSource,
    ) -> Result<CompilationResult, VpeError> {
        let validation = self.validate(schema, law)?;

        let process_ref = ProcessRef::new(&law.domain, &law.process, &law.version);
        let manifests = build_manifests(law);
        let digest = compute_digest(law)?;

        let process = CompiledProcess::new(process_ref.clone(), digest.clone(), manifests.clone());
        let report = RegistrationReport {
            process: process_ref,
            digest,
            manifests,
            warnings: validation.warnings,
        };

        Ok(CompilationResult { process, report })
    }
}
