# VPE FFI Acceptance Tests
Version: Canonical v1

## Engine Lifecycle

AT-001: Engine initializes successfully  
AT-002: Engine frees without crash  
AT-003: Null free is safe  

## Registration

AT-010: Valid schema registers  
AT-011: Valid process registers  
AT-012: Invalid process fails cleanly  
AT-013: Unknown guard fails cleanly  

## Execution

AT-020: Execution is deterministic  
AT-021: Missing anchor fails  
AT-022: State desync fails  
AT-023: No transition returns error  
AT-024: Effects returned correctly  
AT-025: AUTO_TICK executes correctly  

## Migration

AT-030: Valid lift succeeds  
AT-031: No rule fails cleanly  
AT-032: Invalid transform rejected  

## Simulation

AT-040: Seamless replay detected  
AT-041: Divergence detected  
AT-042: Incompatibility detected  

## Safety

AT-050: Invalid UTF-8 handled  
AT-051: Invalid JSON handled  
AT-052: Result freed safely  
AT-053: Double free is undefined (host responsibility)  
AT-054: Null free safe  

## End-to-End

AT-100: Full execution cycle succeeds  
- register schema  
- register process  
- execute  
- commit result  
- repeat deterministically