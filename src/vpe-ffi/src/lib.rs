use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn vpe_init() -> *mut VpeEngine {
    // Create the engine on the heap and "forget" it so Rust doesn't drop it
    let registry = Arc::new(GuardRegistry::new()); // Standard setup
    let engine = Box::new(VpeEngine::new(registry));
    
    Box::into_raw(engine)
}

#[no_mangle]
pub extern "C" fn vpe_register_process(
    engine_ptr: *mut VpeEngine, 
    json_ptr: *const c_char
) -> bool {
    let engine = unsafe { &mut *engine_ptr };
    let json_str = unsafe { CStr::from_ptr(json_ptr).to_string_lossy() };

    match engine.register_process(&json_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[repr(C)]
pub struct FfiVerdict {
    pub next_state_name: *mut c_char,
    pub effect_json: *mut c_char, // Simplified effects as a JSON blob
}

#[no_mangle]
pub extern "C" fn vpe_execute(
    engine_ptr: *mut VpeEngine,
    domain: *const c_char,
    current_state: *const c_char,
    action: *const c_char,
    context_json: *const c_char,
    history_json: *const c_char
) -> *mut FfiVerdict {
    let engine = unsafe { &*engine_ptr };
    
    // 1. Marshall strings from C to Rust
    let d_str = unsafe { CStr::from_ptr(domain).to_str().unwrap() };
    let s_str = unsafe { CStr::from_ptr(current_state).to_str().unwrap() };
    let a_str = unsafe { CStr::from_ptr(action).to_str().unwrap() };
    
    // 2. Deserializing Context/History from JSON strings passed by .NET
    let context: ContextMap = serde_json::from_str(unsafe { CStr::from_ptr(context_json).to_str().unwrap() }).unwrap();
    let history: Vec<VpeEvent> = serde_json::from_str(unsafe { CStr::from_ptr(history_json).to_str().unwrap() }).unwrap();

    // 3. The Core Logic Call
    match engine.execute(d_str, "latest", s_str, a_str, context, history) {
        Ok(v) => {
            let res = Box::new(FfiVerdict {
                next_state_name: CString::new(v.next_state_name).unwrap().into_raw(),
                effect_json: CString::new(serde_json::to_string(&v.effects).unwrap()).unwrap().into_raw(),
            });
            Box::into_raw(res)
        },
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn vpe_free_verdict(ptr: *mut FfiVerdict) {
    if ptr.is_null() { return; }
    unsafe {
        let verdict = Box::from_raw(ptr);
        // Retake ownership of the internal strings so they get dropped too
        let _ = CString::from_raw(verdict.next_state_name);
        let _ = CString::from_raw(verdict.effect_json);
    }
}

#[repr(C)]
pub struct FfiMigrationResult {
    pub success: bool,
    pub new_state_name: *mut c_char,
    pub new_context_json: *mut c_char,
    pub error_message: *mut c_char,
}

#[no_mangle]
pub extern "C" fn vpe_needs_lift(
    current_version: *const c_char, 
    target_version: *const c_char
) -> bool {
    let c_v = unsafe { CStr::from_ptr(current_version).to_string_lossy() };
    let t_v = unsafe { CStr::from_ptr(target_version).to_string_lossy() };
    
    MigrationEngine::needs_lift(&c_v, &t_v)
}

#[no_mangle]
pub extern "C" fn vpe_lift(
    engine_ptr: *mut VpeEngine,
    domain: *const c_char,
    current_state: *const c_char,
    context_json: *const c_char,
    history_json: *const c_char
) -> *mut FfiMigrationResult {
    let engine = unsafe { &*engine_ptr };
    let d_str = unsafe { CStr::from_ptr(domain).to_str().unwrap() };
    let s_str = unsafe { CStr::from_ptr(current_state).to_str().unwrap() };
    
    // Deserialize context and history
    let context: ContextMap = serde_json::from_str(unsafe { CStr::from_ptr(context_json).to_str().unwrap() }).unwrap();
    let history: Vec<VpeEvent> = serde_json::from_str(unsafe { CStr::from_ptr(history_json).to_str().unwrap() }).unwrap();

    // Perform the lift
    match engine.migration_engine.lift(s_str, &context, &history, engine.get_migration_rules(d_str)) {
        Ok((new_state, new_ctx)) => {
            let ctx_json = serde_json::to_string(&new_ctx).unwrap();
            Box::into_raw(Box::new(FfiMigrationResult {
                success: true,
                new_state_name: CString::new(new_state).unwrap().into_raw(),
                new_context_json: CString::new(ctx_json).unwrap().into_raw(),
                error_message: std::ptr::null_mut(),
            }))
        },
        Err(e) => Box::into_raw(Box::new(FfiMigrationResult {
            success: false,
            new_state_name: std::ptr::null_mut(),
            new_context_json: std::ptr::null_mut(),
            error_message: CString::new(e).unwrap().into_raw(),
        })),
    }
}

