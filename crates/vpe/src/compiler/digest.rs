use crate::compiler::source::LawSource;
use crate::error::VpeError;
use sha2::{Digest, Sha256};

pub fn compute_digest(law: &LawSource) -> Result<String, VpeError> {
    let json = serde_json::to_vec(law)
        .map_err(|e| VpeError::Unsupported(format!("digest serialization failed: {e}")))?;
    let mut hasher = Sha256::new();
    hasher.update(json);
    Ok(format!("{:x}", hasher.finalize()))
}
