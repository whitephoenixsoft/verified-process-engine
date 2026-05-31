## Epic

Dynamic Guard Manifest

## Problem 

The host lacks a discoverable, deterministic contract for required execution inputs.

## Why it matters 

- harder to use the SDK without it
- only slightly less work than coding the logic themselves if not available 
- More templating without it 
- forces the host to still map out the law in code, hard coding it. This takes away the benefit of using a JSON law. 

## Definitions

- Manifest: the listing of dependencies needed to process a law of a state. It consists of required Context fields and history slices.
- Context: the environment supplied by the host to support executing the process state. It is used by the guard. 
-  Context sources:
    - Context fields
    - Chronicle 
    - Anchor 
- Context requirements: part of the context. The prerequisites needed by the state(s) to be able to make proper decisions. They contain history slices and context fields.
- Guard: a deterministic function that evaluates context and returns a transition decision. Guards are referenced by state transitions. 
- Chronicle: the history preceding the executing state. It is used by the guard.
- Anchor: the last entry in the history.
- Context field: the value from the context needed to make a decision.
- Schema: the definition of the context fields.

## Invariants 

- The manifest must deterministically represent the complete set of context requirements for a given law or state.
- All context requirements referenced by the manifest must be resolvable from the defined context sources.
- Guard evaluation must be deterministic given the same resolved context inputs.
- All required context inputs must be resolved prior to guard evaluation.

## Constraints 

-  Context requirements are specified by the guards
-  A guard must define their own contact requirements.
- A guard must accept a context when evaluating it's parameters.
- Context fields must be defined in the schema.
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

