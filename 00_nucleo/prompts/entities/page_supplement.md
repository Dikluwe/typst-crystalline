# L0 — Page supplement domain
Hash do Código: 0d79ef0e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/page_supplement.rs`

## Evidence before specification

Historical upstream `a51e02804` anchors are
`typst-library/src/layout/page.rs:315-329`,
`typst-layout/src/pages/run.rs:145-149,233-235`,
`typst-layout/src/document.rs:98-103`,
`typst-layout/src/introspect.rs:29-56,122-129`, and the translation key `page`.
`translations/en.txt:8`, `pt.txt:8`, `es.txt:8`, `fr.txt:8`, `de.txt:8`
provide page, página, página, page and Seite, respectively.
These are snapshot-specific evidence, not a new current-pin measurement.

## Owned contract

`PageSupplement` preserves Auto, None and explicit Content, including empty
Content. Default is Auto. None resolves to empty Content, explicit Content is
preserved, and Auto resolves the page name in the effective language. The
enclosing delta distinguishes omission, which retains the previous value.
Resolved supplement is per-page metadata, not itself a visual item.
Auto must reuse the L1 localization policy and the upstream catalog entry for
every language already supported by that owner. Unsupported entries fall back
to English under the existing policy. Do not invent regional heuristics,
calibrate translations from output or read files at runtime. The consumer's
restricted table is a discrepancy, not permission to weaken this obligation.

## Delegation

RefForm belongs to `00_nucleo/prompts/entities/elements/ref.md`, evaluation to
`00_nucleo/prompts/compiler/eval.md`, page-run coordination to
`00_nucleo/prompts/compiler/layout/page_run.md`, metadata storage to
`00_nucleo/prompts/entities/page_store.md`, reference layout to
`00_nucleo/prompts/compiler/layout_references.md`, and fixpoint coordination to
`00_nucleo/prompts/compiler/layout.md`.
Former cross-owner reference obligations remain handoff constraints in the
diagnostic; sanitation does not authorize changing them. No new constructor,
callback execution, I/O, mutable global state or compatibility change is introduced.
Verify localized Auto, None and explicit empty Content without collapsing omission.
