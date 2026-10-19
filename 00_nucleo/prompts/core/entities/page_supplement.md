# L0 — Page supplement domain
Hash do Código: bcba2be0

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

RefForm belongs to `00_nucleo/prompts/core/entities/elements/ref.md`, evaluation to
`00_nucleo/prompts/core/compiler/eval/mod.md`, page-run coordination to
`00_nucleo/prompts/core/compiler/layout/page_run.md`, metadata storage to
`00_nucleo/prompts/core/entities/page_store.md`, reference layout to
`00_nucleo/prompts/core/compiler/layout/references.md`, and fixpoint coordination to
`00_nucleo/prompts/core/compiler/layout/mod.md`.
Former cross-owner reference obligations remain handoff constraints in the
diagnostic; sanitation does not authorize changing them. No new constructor,
callback execution, I/O, mutable global state or compatibility change is introduced.
Verify localized Auto, None and explicit empty Content without collapsing omission.

## Equivalent derived Default

### Evidence before specification

The former manual Default at `01_core/src/entities/page_supplement.rs:17`
was identified by Clippy. At HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b` with uncommitted working tree,
`lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz`
records sources/owners/spans/UTC/status/diffstat. The non-generic type gains
no parameter bounds and keeps the same value. This is implementation evidence,
not new parity or performance measurement.

### Contract and acceptance

Use derive(Default) with #[default] on Auto, replacing only the manual
implementation. Keep localization, resolution, None and explicit content
unchanged; old manual examples specify the same required value. No public
signature/field/variant, product default or phase change. Preserve all tests
and expectations; validate complete workspace targets, core suite, test
inventory, build, Rustdoc, Clippy, and lineage under continuous ADR-0127 flow.
