use std::fs;
use vpe::compiler::source::LawSource;
use vpe::prelude::*;

pub fn run(schema_path: &str, law_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let schema: DomainSchema = serde_json::from_str(&fs::read_to_string(schema_path)?)?;
    let law: LawSource = serde_json::from_str(&fs::read_to_string(law_path)?)?;

    let registry = GuardRegistry::builder().with_builtins().build()?;
    let compiler = VpeCompiler::with_registry(registry);
    let result = compiler.compile(&schema, &law)?;

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "success": true,
            "data": {
                "process": result.report.process,
                "digest": result.report.digest,
                "manifests": result.report.manifests,
            },
            "warnings": result.report.warnings,
            "errors": [],
        }))?
    );
    Ok(())
}
