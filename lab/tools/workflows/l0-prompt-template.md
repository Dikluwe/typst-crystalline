# L0 Prompt: [Component Name]

**Layer:** L[n] — [Core | Shell | Infrastructure | Wiring]
**Created:** YYYY-MM-DD
**Exclusive consumer:** `[exact productive consumer path]`
**Generated files:** [files this prompt originated, if applicable]
**Technique** *(optional; use only when the mechanism has a named computer-science technique; do not force one into every prompt)*: [technique/area] — [one sentence or reference to the standard concept]. Example: “Term rewriting system — termination by cycle detection, not by a maximum depth.” This field can make the prompt useful as study material as well as a specification.

The prompt’s canonical path is the owner path used by lineage metadata. A materializable prompt owns exactly one productive consumer; one consumer has exactly one owner. Put shared invariants in Tekt nuclei, pinned by their consuming prompts. A nucleus is not an owner and does not authorize code by itself.

---

## Context

Describe relevant existing system behavior, the domain problem this work addresses, and the components with which the new or changed component interacts.

## Structural Constraints

- State the target layer and its implications (for example, L1 has zero I/O).
- Identify interfaces to implement and link existing source files.
- State what the component must not do.
- List dependencies forbidden in the target layer.
- Preserve existing public API, behavior, and phase unless a separately authorized change says otherwise.

## Instruction

Specify the required result precisely enough that a later implementation produces structurally equivalent behavior. Separate owned responsibilities from work owned by other prompts. State scope-outs and unresolved proposals with their existing status; do not silently promote a proposal to an accepted rule or delete an obligation without authority.

## Verification Criteria

Describe deterministic observations that establish whether the instruction was satisfied. Write and run tests before implementation where applicable: establish the intended failing result (RED), then implement and verify the passing result (GREEN). Keep test authoring and implementation as distinct stages; do not require simultaneous authorship.

```text
Given [precise precondition]
When [deterministic action]
Then [expected resulting state]
```

Include relevant edge cases and error behavior. Record evidence and provenance when a decision depends on a measurement.

## Expected Result

- List files created or changed, including implementation and tests where applicable.
- State what each file exposes (functions, types, or interfaces).
- Explain how to verify structural constraints and the requested behavior.
- State the required validation commands and resource limits from current repository guidance.

## Revision History

| Date | Reason | Affected files |
|---|---|---|
| YYYY-MM-DD | Initial creation | [file] |
