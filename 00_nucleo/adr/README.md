# Architecture decisions

Start with the [reading order](ORDER.md). It lists the cross-component rules, specific decisions still in force, pending proposals and predecessors. A superseded record retains its original text at its stable path. Follow its **Current authority** or supersession notice for new work. The [historical path map](ARCHIVE.md) covers decisions moved before the consolidation.

## Choosing where to record a change

| What changes | Where to record it |
|---|---|
| A component contract, including an extension covered by an existing decision | Update the owning L0 first. Apply the [approval gate](adr-0127-l0-gate-stop-vs-continuous-flow.md) to the actual change. |
| A declarative invariant used by several owners | Pin a Tekt Nucleus from each owner under the [L0 ownership rules](adr-0129-l0-owners-and-shared-tekt-nuclei.md). |
| A measurement, comparison, inspection or construction result | Follow the [evidence workflow](adr-0132-evidence-for-maintained-changes.md). Keep the measured revision and the limits of the result. |
| A new durable architectural choice across components, or a change to one | Amend the governing ADR when its scope fits. Use the [template](template-adr.md) for an independent decision. |

## Status and identity

Keep numbered files in this directory. Use `adr-NNNN-short-english-title.md` and preserve each assigned identifier and filename through status changes. ADR-0026 and ADR-0026-R1 identify separate records. Leave unused numbers and archived identifiers as recorded in the index.

The primary metadata field uses `**Status:** VALUE` on its own line. Keep approval dates, qualifications and prior status separately.

| Value | Meaning |
|---|---|
| PROPOSED | Awaiting adoption. |
| ACCEPTED | An architectural rule in force. |
| IMPLEMENTED | An adopted technical decision has been materialized. |
| DEFERRED | Intentionally postponed. |
| SUPERSEDED BY ADR-NNNN | The named decision governs the subject. An owning L0 can supply the local contract. |
| REVOKED | Withdrawn without a successor. |

## Maintaining a decision

Write new decisions and maintained translations in English. Include the owner, date, measured context, decision, alternatives, consequences, compatibility, affected L0s, and verification. Identify each L0 by its full repository-relative path under [ADR-0129](adr-0129-l0-owners-and-shared-tekt-nuclei.md).

When consolidating, trace each continuing obligation and exception to its successor or owning L0. Keep unresolved public contracts and proposals visible with their status. Preserve approvals, hashes, dates, commands, observable error strings and machine-parsed literals. Archive run-specific evidence with its original revision. A later measurement can supersede an observation without rewriting the earlier receipt.

Update the reading order and operational guides with each change. Check that every numbered file is indexed once, every successor is active and the destinations resolve in a clean checkout. Run `git diff --check` and the repository’s lineage checks after changes to L0.
