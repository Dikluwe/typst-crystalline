# ADR-0132 — Evidence for changes to the maintained compiler

**Status:** ACCEPTED
**Date:** 2026-11-19
**Decision owners:** Typst Crystalline maintainers

## Context

ADR-0114 §2 and ADR-0117 §§1–3 both require a separate A.0 artifact before a
materialization specification. ADR-0084 prescribes numbered audit phases and
coverage bands. ADR-0085 prescribes the corresponding construction filenames.
ADR-0065 requires a numbered inventory sub-step. ADR-0080 exempted certain
additive construction changes from L0 updates, including new public fields
and new modules, which now have explicit ownership and approval rules.
These procedures caught mistaken assumptions about missing features during
migration. The recent reference-layout integration used owning L0 prompts,
source anchors, failing tests, controlled measurements, and checks bound to
the tested tree. Its evidence is indexed in
`00_nucleo/diagnostics/validated-references-layout-2026-11-19.md`.

## Decision

Before deciding, inspect the current implementation, owning L0 and relevant
architectural decisions. Record a source anchor or observable result that
distinguishes the claimed problem from the current behavior. When upstream
behavior matters, use the pinned reference and classify the finding under
ADR-0107. Record the revision and reproducible observation according to
ADR-0108 and ADR-0121.

Use the smallest durable record that preserves the evidence and decision.
A local correction can cite an existing test, a failing focused test, or an
observed source path in its owning L0. A multi-component audit, uncertain
baseline, or performance comparison can use a diagnostic with linked raw
evidence. Preserve prior measurements and add new receipts when the tree or
input changes. The depth of verification follows the scope and risk of the
change.

Numbered construction steps, a separate A.0 file, a minimum number of
commands and B1/B2/B3 percentages are optional techniques. ADR-0127 continues
to govern confirmation. The owning L0, lineage and nucleus checks still
apply before materializing a productive change.

Use the LOC budgets and research formats in ADR-0094 when a current task
calls for them. A stop for a budget requires an explicit current limit.
Reuse an existing helper when it preserves its caller's contract.

## Affected L0 prompts

This decision changes the evidence workflow. Existing component obligations
and historical evidence in their owners keep their scope. Use canonical
owner paths for future changes under ADR-0130 and ADR-0131.

## Consequences

Current work can proceed from one evidence-backed L0 update. A broader
investigation still leaves a reproducible diagnostic. The reviewer must
decide whether the evidence exercises the actual behavior under change.

## Alternatives considered

Keeping the A.0 artifact as a universal gate preserves a uniform form
but makes small repairs duplicate evidence. Removing the evidence gate
loses the protection against stale assumptions that the construction
reports established.

## Compatibility and supersession

Supersedes ADR-0080, ADR-0114 and ADR-0117. For current work, supersedes the
prescribed sub-step, phase sequence, coverage bands and filename rules in
ADR-0065, ADR-0084 and ADR-0085. It clarifies the scope of the construction
recipes in ADR-0094. Preserve the original audit records and
their provenance. ADR-0107, ADR-0108, ADR-0119, ADR-0121, ADR-0127 and
ADR-0129 continue to apply.

## Verification

For each decision, a reviewer can recover the current owner, concrete
evidence and the validation used. For a public contract, default, phase
or compatibility change, the reviewer can also recover the ADR-0127
approval. Keep historical ADR identifiers and links resolvable.
