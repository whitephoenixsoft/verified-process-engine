use thiserror::Error;

#[derive(Debug, Error)]
pub enum VpeError {
    #[error(transparent)]
    Schema(#[from] SchemaError),

    #[error(transparent)]
    Compile(#[from] CompileError),

    #[error(transparent)]
    Runtime(#[from] RuntimeError),

    #[error("process not found: {0}")]
    ProcessNotFound(String),

    #[error("unsupported: {0}")]
    Unsupported(String),
}

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("invalid schema: {0}")]
    Invalid(String),
}

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("invalid law: {0}")]
    InvalidLaw(String),

    #[error("unknown guard type: {0}")]
    UnknownGuardType(String),

    #[error("type mismatch: {0}")]
    TypeMismatch(String),

    #[error("unresolved reference: {0}")]
    UnresolvedReference(String),

    #[error("duplicate state: {0}")]
    DuplicateState(String),

    #[error("initial state not found: {0}")]
    InitialStateNotFound(String),

    #[error("unknown transition target state: {0}")]
    UnknownTargetState(String),
    
    #[error("empty target not allowed")]
    EmptyTargetState,
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("anchor missing")]
    AnchorMissing,

    #[error("desync: expected state '{expected}', got '{provided}'")]
    Desync { expected: String, provided: String },

    #[error("no transition found from state '{state}' for action '{action}'")]
    NoTransitionFound { state: String, action: String },
    
    #[error("missing required context field: {field}")]
    MissingContextField { field: String },
    
    #[error("maximum auto-transition depth exceeded")]
    AutoTransitionLimitExceeded,

    #[error("unknown state: {0}")]
    UnknownState(String),
}
