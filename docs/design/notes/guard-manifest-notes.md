## Epic

Dynamic Guard Manifest

## Problem 

The host cannot call VPE without hard coding each process state prerequisite into its logic before the state is executed.

## Why it matters 

- harder to use the SDK without it
- only slightly less work than coding the logic themselves if not available 
- More templating without it 
- forces the host to still map out the law in code, hard coding it. This takes away the benefit of using a JSON law. 

## Definitions

- context: the environment supplied by the host to support executing the process state 
- context requirements: prerequisites needed by the state(s) To be able to make proper decisions. 
- guard: function specified in a state transition To conditionally direct the state to another state.
- chronicle: the history preceding the executing state 
- anchor: the last entry in the history 
- context field: values needed for the state transition to make a decision 
- schema: the definition of what valid context field are available

## Invariants 

- A manifest for the same guards and parameters must be determistic.
- Context fields must be defined in the schema.
- A guard must define their own contact requirements.
- A guard must accept a context when evaluating it's parameters.
- Context requirements must be available to the guard during runtime.

## Constraints 

-  Context requirements are specified by the guards
- A manifest generated for one state will return the context requirements for the guards of that state. 
- A manifest generated during compilation will return the context requirements for all states. 
- Context requirements must be available in the context, chronicle, or the anchor.
- A guard may only depend on declared context fields, It's anchor,  and declared chronicle during runtime 
- Context requirements can be context fields, a slice of History, or the anchor.

## Model

1. The host you defines the context fields in the schema 
2. The host defines the law and uses guards for state transitions.
3. Optionally, the host can define custom guards. Each guard must define its own context requirements.
4. The context Fields will be checked during compile time for validity 
5. After compilation a fool a manifest will be generated with all of the contacts requirements for the law. 
6. During runtime, the host will call the manifest to see what context requirements are required before executing The process. 
7. The host will retrieve the required history and context fields recommended from the manifest along with the anchor and supply it as part of the context.
8. The host executes the process with the command and context.

## Edge cases 

- A law without any guard will return an empty manifest after compilation
- A state without any guards will return an empty manifest during runtime execution. 
- with All with multiple occurrences of the same contact requirements will only one return one instance in the manifest. 
- guard does not report any context requirements. 
- context requirement too big for memory 

## What it will not handle 
- Runtime validation 
- Context mutability 
- guard business logic 
- manifest file format
- State transitions














