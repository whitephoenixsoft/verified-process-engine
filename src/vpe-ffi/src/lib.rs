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
