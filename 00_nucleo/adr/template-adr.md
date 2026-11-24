# ADR-NNNN — Decision title

**Status:** PROPOSED
**Date:** YYYY-MM-DD
**Decision owners:** [name or role]

## Context

Describe the measured pressure, requirement, or constraint that makes a
cross-cutting architectural decision necessary. Cite repository evidence by
canonical path and line when the decision depends on the current tree.

Identify the existing primary ADR that governs the question. Extend it when the
new decision fits its scope. Open a separate ADR for an independent
cross-component choice, global constraint, technology adoption or layer
boundary. Record component-local changes in the owning L0 and apply the
ADR-0127 approval gate when required.

## Decision

State the decision precisely enough that implementation and review are
unambiguous. Separate normative requirements from historical explanation.

## Affected L0 prompts

List every L0 prompt that must be revised because of this decision. Use the
full repository-relative path including `00_nucleo/prompts/`, as required
by ADR-0129. Do not use a basename, a prompts-root-relative abbreviation,
or an absolute filesystem path as its identity.

| Prompt | Required change |
|---|---|
| `00_nucleo/prompts/core/compiler/layout/mod.md` | Example only: replace with the actual existing affected prompt and its contract change. |

## Consequences

### Positive

- Describe the structural benefit.

### Negative

- Describe introduced cost or complexity.

### Neutral

- Describe trade-offs without a direct integrity impact.

## Alternatives considered

| Alternative | Advantages | Reasons rejected |
|---|---|---|
| Option A | — | — |
| Option B | — | — |

## Compatibility and supersession

State whether this ADR supersedes, amends, or is constrained by another ADR.
Use ADR identifiers as the stable reference; paths are secondary.

## Verification

Define the evidence that demonstrates compliance. When a number is used to
close or classify work, record the exact commit or the uncommitted working-tree
state and observation time as required by ADR-0132.

## References

- Add repository evidence, issues, standards, or primary documentation.
