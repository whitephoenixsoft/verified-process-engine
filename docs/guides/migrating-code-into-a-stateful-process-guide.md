# Migrating Code into a Stateful Process

## Purpose

This guide explains how to recognize when ordinary code is secretly a process, and how to migrate that code into an explicit stateful process.

The goal is not to make code more complicated. The goal is to make hidden process structure visible, enforceable, replayable, and explainable.

This is especially relevant for VPE, where processes may need explicit states, transitions, guards, effects, chronicle entries, and versioned laws.

---

## Core Idea

Ordinary code often treats an operation as one atomic action.

A stateful process treats that same operation as a lawful sequence of allowed movements.

Example:

Atomic action:

    approve_item()

Stateful process:

    Draft -> UnderReview -> Approved -> Committed -> Closed

The important question is not:

> Can this be modeled as a state machine?

Almost anything can.

The better question is:

> Does this operation need governance, or does it only need memory?

---

## Key Distinction

A change log records movement.

A state machine governs movement.

An event records an atomic action.

A state machine governs an interruptible action.

---

## When Code Is Hiding a Process

Code may need to become a stateful process when the operation can:

- Pause
- Resume
- Branch
- Be rejected
- Be approved
- Be reviewed
- Be blocked
- Be clarified
- Be superseded
- Be migrated
- Be replayed
- Require human or host intervention
- Produce multiple legitimate outcomes

If none of those are true, a simple function or event may be enough.

---

## Atomic Code Example

A simple implementation may look like this:

    fn approve_item(item: Item) -> Result<Resolution, Error> {
        validate_item(&item)?;
        check_conflicts(&item)?;
        let resolution = create_resolution(item)?;
        save_resolution(&resolution)?;
        close_source_item(&resolution)?;
        Ok(resolution)
    }

This looks like one action:

    approve item

But it may contain a hidden process:

    validate
    check conflicts
    create resolution
    save resolution
    close source

If every step happens immediately and failure simply aborts the operation, this may be fine.

But if any step can pause, branch, require intervention, or become meaningful later, the code is probably hiding a process.

---

## Migration Rule

When migrating code into a stateful process, classify the code into four categories:

| Existing Code Element | VPE Concept |
|---|---|
| Decision point | Transition or guard |
| Status update | State |
| Side effect | Effect |
| Record/log | Chronicle event |

Example:

    validate_item(&item)?;

May become:

    guard: item_is_valid

Example:

    check_conflicts(&item)?;

May become:

    if no conflicts -> ReadyForApproval
    if conflicts -> BlockedByConflict
    if ambiguous -> NeedsClarification

Example:

    save_resolution(&resolution)?;

May become:

    effect: emit ResolutionCreated commit

---

## Step 1: Identify the Object Moving Through the Process

Do not start by naming states.

Start by asking:

> What object is moving through the process?

Examples:

    CDS item
    candidate
    review request
    resolution draft
    conflict
    migration attempt
    law version
    signal
    vote
    manifest change

The states should describe the condition of that object.

Good states:

    Draft
    SelectedForReview
    MappedToArea
    BlockedByConflict
    ResolutionCandidate
    Approved
    Committed
    Closed
    Rejected
    NeedsClarification

Weak states:

    Checking
    Thinking
    Handling
    Processing
    DoingReview

Verbs are often transitions.

Nouns or adjectives are often states.

---

## Step 2: Identify the Boundary

A process usually exists because something is trying to cross a boundary.

Examples:

    CDS item -> Resolution
    Draft law -> Active law
    Candidate -> Approved candidate
    Conflict -> Resolved conflict
    Signal -> Escalated review item
    Old law version -> Migrated law version

The boundary matters because it tells you why governance is needed.

For Charter, many important boundaries are legitimacy-sensitive.

That means the system must not allow an item to cross casually.

---

## Step 3: Find the Intervention Points

A state is often a safe stopping place.

Look for places where a human, AI host, validator, reviewer, rule, or external system may intervene.

Ask:

    Where could this process safely pause?
    Where might a human need to inspect it?
    Where might another system need to resume it?
    Where might we need to explain why it stopped?
    Where can the next legal move change?

Those are likely states.

Example:

    SelectedForReview
    NeedsClarification
    BlockedByConflict
    PendingApproval
    SimulationDiverged
    MigrationRequired

---

## Step 4: Convert Actions into Transitions

Old method names often become transitions.

Example:

    approve(candidate)
    reject(candidate)
    request_clarification(candidate)
    commit(candidate)

May become:

    approve:
      UnderReview -> Approved

    reject:
      UnderReview -> Rejected

    request_clarification:
      UnderReview -> NeedsClarification

    commit:
      Approved -> Committed

A transition should represent lawful movement from one state to another.

---

## Step 5: Convert Conditions into Guards

Old `if` checks often become guards.

Example:

    if !has_required_reviewer(candidate) {
        return Err("missing reviewer");
    }

May become:

    transition: approve
    from: UnderReview
    to: Approved
    guard: has_required_reviewer

A guard answers:

> Is this transition allowed right now?

Guard failure means the movement is not legitimate.

---

## Step 6: Convert Side Effects into Effects

Old writes, saves, messages, or commits usually become effects.

Examples:

    effect: emit ResolutionCreated commit
    effect: close CDS item
    effect: notify reviewer
    effect: append chronicle entry
    effect: create migration report
    effect: mark previous resolution superseded

Effects should happen because a transition occurred.

The state machine governs the movement.

The effects apply the consequences.

---

## Step 7: Convert Logs into Chronicle Events

Old log entries may become chronicle events.

Example:

    ReviewStarted
    ValidationPassed
    ConflictDetected
    ClarificationRequested
    ApprovalGranted
    ResolutionCommitted
    SourceItemClosed

A chronicle should make the process replayable and explainable.

The chronicle should answer:

    What happened?
    When did it happen?
    What state was the item in?
    What transition was attempted?
    Which guards passed or failed?
    What effects were emitted?
    Why was the process blocked, rejected, or allowed?

---

## Step 8: Decide Whether Failures Are Errors or States

Not every failure should be treated the same way.

Some failures are meaningful process outcomes.

Other failures are technical errors.

### Process outcomes

These may deserve states:

| Old Failure | Possible Process State |
|---|---|
| Conflict found | BlockedByConflict |
| Missing information | NeedsClarification |
| Waiting for approval | PendingReview |
| Simulation mismatch | SimulationDiverged |
| Migration unavailable | Incompatible |
| Duplicate candidate | MergeRequired |
| Ambiguous reference | RejectedAmbiguous |

### Technical errors

These should usually remain errors:

    database unavailable
    serialization failed
    network timeout
    permission service unavailable
    disk full
    unexpected panic

Rule:

> If the failure is a meaningful domain or process outcome, consider making it a state.  
> If the failure is infrastructure malfunction, keep it as an error.

---

## Guard Failure vs Transition Failure

A guard failure means:

    The transition was not allowed.

A transition failure means:

    The transition was allowed, but execution failed.

Example transition:

    Candidate -> Approved

Guard failure:

    Cannot approve because required reviewer is missing.

Transition failure:

    Approval was valid, but writing the commit failed.

These should not be modeled the same way.

The first belongs to process law.

The second belongs to runtime reliability.

---

## The Migration Ladder

Do not jump straight from ordinary code to a full state machine.

Use a ladder.

### Stage 1: Plain Function

    approve_item()

Everything happens in one call.

Good for simple atomic operations.

---

### Stage 2: Function with Explicit Steps

    validate
    detect conflicts
    approve
    commit
    close

Still code, but the internal stages are visible.

---

### Stage 3: Function Emits Events

    ReviewStarted
    ValidationPassed
    ConflictCheckPassed
    ApprovalGranted
    ResolutionCreated
    SourceClosed

Now there is a chronicle.

---

### Stage 4: Events Derive Status

    Current status: Approved

The system can reconstruct where the item is.

---

### Stage 5: Explicit States and Transitions

    Draft -> UnderReview -> Approved -> Committed

Now the process is governed.

---

### Stage 6: Versioned Process Law

    ReviewProcess v1
    ReviewProcess v2
    Migration/lift between them

Now the process is VPE-native.

---

## Example: Migrating a Charter Review Process

Original rough code:

    fn reconcile_cds_item(item: CdsItem, area: Area) -> Result<Resolution, Error> {
        ensure_selected(item)?;
        ensure_area_valid(area)?;
        ensure_no_blocking_conflict(item)?;
        let resolution = derive_resolution(item, area)?;
        commit_resolution(resolution)?;
        close_cds_item(item)?;
        Ok(resolution)
    }

Hidden process:

    CDS item starts as open.
    It is selected for reconciliation.
    It is mapped to an Area.
    It is checked for conflicts.
    It becomes a resolution candidate.
    It is approved.
    A derived resolution is committed.
    The original CDS item is closed.

Possible states:

    Open
    SelectedForReview
    MappedToArea
    BlockedByConflict
    ResolutionCandidate
    Approved
    Committed
    Closed
    Rejected
    NeedsClarification

Possible transitions:

    select_for_review:
      Open -> SelectedForReview

    map_to_area:
      SelectedForReview -> MappedToArea

    detect_conflict:
      MappedToArea -> BlockedByConflict

    clear_conflict:
      BlockedByConflict -> MappedToArea

    derive_candidate:
      MappedToArea -> ResolutionCandidate

    approve:
      ResolutionCandidate -> Approved

    commit:
      Approved -> Committed

    close_source:
      Committed -> Closed

    reject:
      SelectedForReview | MappedToArea | ResolutionCandidate -> Rejected

    request_clarification:
      SelectedForReview | MappedToArea | ResolutionCandidate -> NeedsClarification

This is no longer just a change log.

It is a lawful process.

---

## How to Know the Granularity Is Right

A state may be too small if:

    Nobody cares if the system pauses there.
    No different action becomes legal there.
    No audit question depends on it.
    It only exists because the implementation has a function there.

A state may be too large if:

    Too many different things can happen inside it.
    You need flags to explain what subcondition it is in.
    Users ask “where exactly is it stuck?”
    Different reviewers need to act at different moments.

A major smell:

    state: UnderReview
    flags:
      validated: bool
      conflict_checked: bool
      area_mapped: bool
      approved: bool
      committed: bool

This may mean `UnderReview` is hiding multiple states.

---

## The Most Important Conversion Question

Do not ask first:

> What states do I need?

Ask:

> What must be prevented?

For Charter and VPE, examples include:

    Prevent unreviewed CDS items from becoming Resolutions.
    Prevent ambiguous references from silently expanding.
    Prevent conflicted candidates from being committed.
    Prevent partial approval from appearing legitimate.
    Prevent derived items from being reprocessed.
    Prevent old process versions from corrupting new laws.

Each prevention rule usually implies guards, states, transitions, or chronicle requirements.

---

## VPE Process Formula

A VPE process can be described as:

    Process = object + states + transitions + guards + effects + chronicle + version

Where:

    object = what is moving
    states = where it may safely wait
    transitions = allowed movements
    guards = conditions that must be true
    effects = external changes caused by movement
    chronicle = replayable record
    version = law identity over time

---

## Process Extraction Worksheet

Use this worksheet when migrating ordinary code into a stateful process.

    1. What object is moving?
    2. What boundary is it trying to cross?
    3. What makes the boundary legitimacy-sensitive?
    4. What are the possible stable states?
    5. What are the allowed transitions?
    6. What guards must pass?
    7. What effects happen after transition?
    8. What events must be recorded?
    9. What failures are meaningful states?
    10. What failures are technical errors?
    11. Where can a human or host intervene?
    12. What must be replayable later?
    13. What must be prevented?
    14. What process version is this?
    15. What happens if the process law changes later?

---

## Strong State Naming Guidance

Prefer states that describe the condition of the object:

    Draft
    SelectedForReview
    PendingApproval
    NeedsClarification
    BlockedByConflict
    ResolutionCandidate
    Approved
    Committed
    Closed
    Rejected
    Deprecated
    Superseded

Avoid vague activity states:

    Processing
    Handling
    Checking
    DoingReview
    Thinking
    Working
    MaybeDone

A strong state should help answer:

    What is allowed next?
    Why is the item stuck?
    Who may intervene?
    Is it legitimate yet?
    Can it be resumed?
    Can it be replayed?

---

## Charter/VPE Application

For Charter, VPE is especially useful around legitimacy gateways.

Examples:

    CDS item -> Reconciliation Review -> Resolution
    Resolution -> Supersession Review -> New Resolution
    Conflict -> Deliberation -> Resolution
    Signal -> Escalation -> Review Item
    Law Version -> Migration -> Lifted Law Version
    Candidate -> Group Decision -> Legitimate Commit

VPE should not decide the meaning of Charter concepts.

Instead:

    Charter defines what matters.
    VPE defines what may happen next.
    CCS records what happened.
    CQL asks what happened.
    CGL explains or assists with interpretation.

---

## Final Summary

Migrating code into a stateful process means:

    Stop treating the operation as one function.
    Identify the object crossing a boundary.
    Find the intervention points.
    Promote meaningful pauses into states.
    Turn actions into transitions.
    Turn conditions into guards.
    Turn writes into effects.
    Turn logs into chronicle events.
    Keep technical failures separate from process outcomes.
    Version the process law when its rules change.

The goal is not ceremony.

The goal is lawful movement.

A stateful process is justified when the system must know not only what happened, but whether what happened was allowed.

---

## State Machine Benefit Checklist

Use this checklist to decide whether a piece of code should remain a simple action/event or be migrated into a stateful process.

| Question | Without State Machine | With State Machine | State Machine Benefit? |
|---|---:|---:|---:|
| Can the operation be treated as one atomic step? | Yes | Yes, but may be unnecessary | No |
| Can the operation pause and resume later? | Hard / informal | Explicitly supported | Yes |
| Can a human, host, or reviewer intervene? | Usually ad hoc | Modeled as states/transitions | Yes |
| Can the operation branch into different valid outcomes? | Hidden in conditionals | Explicit transition paths | Yes |
| Can an action be approved, rejected, or blocked? | Usually custom logic | First-class process outcomes | Yes |
| Can invalid sequences happen? | Possible unless manually guarded | Prevented by transition rules | Yes |
| Does the current phase change what actions are allowed? | Hard to see | Clear from current state | Yes |
| Do you need to explain why something is stuck? | Requires digging through logs | State gives direct explanation | Yes |
| Do you need deterministic replay? | Possible, but harder | Natural fit with chronicle/events | Yes |
| Do you need auditability? | Logs may help | State + transition history is stronger | Yes |
| Do you need to prove the process was followed? | Difficult | Transition history can prove it | Yes |
| Do you need guard checks before movement? | Manual checks in code | Guards are part of process law | Yes |
| Do you need different failure outcomes? | Often mixed with errors | Process outcomes can become states | Yes |
| Do you need to separate technical failure from process rejection? | Often blurred | Clearly separated | Yes |
| Do you need migration between process versions? | Usually difficult | Versioned laws can support lift/migration | Yes |
| Do you need simulation before committing? | Custom implementation | Can be modeled as process state/path | Yes |
| Do you need to prevent partial completion from looking complete? | Risky | Explicit incomplete/intermediate states | Yes |
| Do you need to support multiple actors? | Often scattered across code | Actor permissions can attach to transitions | Yes |
| Do you need to make allowed next actions visible? | Hard to infer | Derived from current state | Yes |
| Is the operation simple, local, and immediately completed? | Good fit | May be overkill | No |
| Is the workflow still unstable or experimental? | Easier to change | May cause premature rigidity | No / Maybe |
| Are the states mostly implementation steps? | Simpler as code | State machine may add noise | No |
| Is no one expected to inspect or resume the middle? | Fine as function | Little benefit | No |
| Would the state names be vague, like `Processing` or `Handling`? | Better as code/logs | Weak state model | No |
| Does the process cross a legitimacy boundary? | Risky | Strong fit | Yes |
| Does the process produce a legitimate artifact? | Possible, but informal | Strong fit | Yes |
| Does the process need to reject ambiguity? | Manual validation | Explicit blocked/rejected outcome | Yes |
| Does the process need to prevent silent expansion or silent continuation? | Harder | Strong fit | Yes |


---

## Quick Decision Rule

Use a state machine when the answer is **yes** to several of these:

| Question | Yes/No |
|---|---|
| Can this process pause? |  |
| Can it resume later? |  |
| Can someone intervene? |  |
| Can it be approved or rejected? |  |
| Can it be blocked? |  |
| Can it branch into multiple valid outcomes? |  |
| Can invalid ordering cause harm? |  |
| Does the current phase change what is allowed? |  |
| Does it cross a legitimacy boundary? |  |
| Does it create or modify a legitimate artifact? |  |
| Does it require guards before movement? |  |
| Does it need a replayable chronicle? |  |
| Does it need audit/proof that the process was followed? |  |
| Does it need migration/version handling? |  |
| Would a simple log be insufficient to explain what happened? |  |

If most answers are **no**, keep it as a function, event, or change log.

If several answers are **yes**, especially around pause/resume, intervention, legitimacy, replay, or invalid ordering, it likely deserves a state machine.

---

## VPE Fit Checklist

| VPE Question | Yes/No |
|---|---|
| Is there an object moving through a process? |  |
| Is there a boundary it must cross? |  |
| Is the boundary legitimacy-sensitive? |  |
| Are there allowed and forbidden transitions? |  |
| Are there guards that must pass? |  |
| Are there effects after successful movement? |  |
| Are there meaningful process failures? |  |
| Are there technical failures that must stay separate? |  |
| Should the process be replayable? |  |
| Should the process be versioned? |  |
| Could future laws require migration/lift? |  |

If this table is mostly **yes**, the code is not just code anymore. It is a VPE process candidate.