# Review: physical page occurrences for SetPage running content

**Decision requested:** approve the exact new public API and Paged phase/order described in the frozen [full specification](agents/core/proposal.md), [Rust declarations](agents/core/api-declarations-final.json) and [finite owner domains](agents/core/candidate-owner-domains.json). Implementation remains pending this decision and subsequent owner-first L0/RED→GREEN gates.

| Frozen artifact | SHA-256 |
|---|---|
| Full proposal | `f509e593d67ec2c2ca19f9c32f2d561a9149d4db903e5b12f11d76dbf9cbdaa2` |
| Exact public declarations | `4baaab61b3c3b247593433d18ab78d22ba5e2c275acab6e747c43dc0ef559196` |
| Owner domains | `a32eca2a829a350b1826bdd48a822b0383e70ac863fab4dee2fec5f6be9a6306` |

The [independent final review](agents/contracts/proposal-final-independent-review.md), bound by its [machine-readable receipt](agents/contracts/proposal-final-independent-review.json), finds the frozen API/phase proposal reviewable without a concrete contract, provenance or compatibility blocker. It does not claim implementation success. The [per-site classification](agents/parity/descendant-site-classification.md) supplies the finite source/generated/measurement/deferred catalogue and the required owner-specific proofs.

## Measured cause and result required

The [baseline report](README.md) records the original Figure 1/header + Figure 1/body/final 1 failure, and fresh pinned-oracle evidence requiring Figure 1/header + Figure 2/body/final 2. Repeated headers and footers require separate physical occurrences and causal actions. A configured template superseded before any page stays inert. Contextual headers need actual page identity; ambient styles follow each physical page (20pt then 30pt), while explicit inner styles remain effective. A repeated label must yield the observed duplicate-label error.

SetPage currently transports an opaque raw template until layout. Preparing every installed delta would execute dormant templates, and treating one installation as one occurrence would lose repetitions. Native PageRun has lexical boundaries/restoration and cannot replace progressive SetPage. These observations support the proposed physical admission boundary; they do not require upstream frame-introspector mechanics.

## Exact architectural decision

Add five public functions and their explicitly declared protocol types:

| Function | Responsibility |
|---|---|
| `prepare_page_running_body` | Build an owned Content/source-plan pair under the shared occurrence arena. |
| `layout_with_page_running` | Discover actual physical requests and replay sealed contributions through pure L1 layout. |
| `realize_page_running` | Prepare a requested insertion using its actual effective environment. |
| `bind_page_running_content` | Rebind the existing Context Session's expanded contribution into a new validated immutable resolution. |
| `introspect_with_page_running` | Incrementally construct the source-derived, target-aware global index from admitted order, including Bundle document starts. |

The channel covers Background, Header, Footer and Foreground. Discovery, retained realization, source-derived incremental construction and validated replay join the existing Paged candidate transaction. Actual page order is background → header → body → footer → foreground. Pending work cannot escape as a complete document. Renderers remain callback-free. Counter/state/query/label order, original/visual membership, source parents, geometry and diagnostic transactions share the same admitted occurrence view.

An opaque typed source plan transports body packets by explicit child edges, including no-paint facts and replay/deferred paths. It keeps original Content intact. Immutable request authority distinguishes source/capture generation from physical keys; aliases are reserved without executing dormant templates. Matching includes the measured effective page environment. The original signatures, entity fields, enum variants and legacy layout façades are preserved. HTML keeps its separate semantic target; no direct Func marginal mode or new flag is added.

The finite inspected scope includes 56 direct-call owners and 118 sites plus the explicitly named optimized, generated-wrapper, deferred, math, checkpoint and orchestration domains. Each actual edit requires its current owning L0 first, a narrower change manifest and meaningful tests. Unproven packet coverage prevents Complete. This approves no unknown public interface, generic equality change, global frame introspection, new solver or unrelated column/baseline policy. Any subsequently discovered public/phase/compatibility extension returns to ADR-0127 review.

## Gates and limits

The implementation must demonstrate actual RED→GREEN for the frozen sources and causal acceptance families, preserve historical F/G receipts and all warning/error expectations, then repeat tests, builds, strict Clippy/Rustdoc/fmt, canonical paths, forward/reverse lineage, ownership/Nucleus DAG/pins and complete diagnostic comparison. Scoped hashes follow the validated 1:1 owner graph. Existing uncorrected position/HTML-warning/cast-message observations remain individually visible; baseline validation success is not language acceptance.

All twelve existing validation jobs pass on the unchanged 2142 productive inputs at HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`. Full pinned lint has zero errors and warnings and unchanged V19/V20 note objects. See [the receipts](validation-baseline/validation-commands.json) and [complete lint comparison](lint-delta.json).

## Why confirmation is required

[ADR-0127 §2, items 1 and 3](../../../00_nucleo/adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) requires confirmation for a new public contract and movement of logic between pipeline phases. This proposal adds exact public L1 interfaces used by L3 and a physical-discovery/retained-realization/source-construction boundary. Previous Figure/Ref approvals explicitly did not cover static SetPage admission. The ordinary parity/internal flow does not waive these two gates.
