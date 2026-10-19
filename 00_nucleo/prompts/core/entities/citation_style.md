# Prompt L0 — `entities/citation_style`
Hash do Código: f2e2438d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/citation_style.rs`
**References:** ADR-0037, ADR-0054, ADR-0107.

## Evidence and scope

CitationStyle is a crystalline fallback construct for numeric/author-date/
alphabetic citation systems, orthogonal to CitationForm's rendering form.
Upstream has no CitationStyle enum: historical e0e8ca4d uses
Derived<CslSource,CslStyle> (bibliography.rs:163) and CitationForm
(cite.rs:133-147). Its bibliography default is fixed IEEE, not unspecified:
bibliography.rs:159-163 selects ArchivedStyle::InstituteOfElectricalAndElectronicsEngineers;
lines 79-95 describe IEEE's Engineering/IT use.

The 2026-08-13 comparison without explicit bibliography style observed
upstream `A [1] B [1] C [2] D [1]` versus local
`A [1] B ibid. C [2] D [1] Doe, op. cit.`; explicit "ieee" produced matching
bytes in that probe. The local Numeric fallback is not filling an upstream
vacuum. Applying the product's IEEE default is a separate bibliography-layout
issue; this entity cleanup neither changes it nor certifies current parity.

## Contract

CitationStyle derives Debug/Clone/Copy/PartialEq/Eq/Hash with AuthorDate,
Numeric and Alphabetic. Default is Numeric. Examples are (Kirsch,1973),
[1]/[2] and [Kir73], respectively; numeric order follows first appearance in
the bounded original integration. as_str(&self)->&'static str yields
"author-date", "numeric", "alphabetic" exactly. Distinct variants differ.

The historical integration description names optional CiteElem style, strict
case-sensitive style-string parsing and layout unwrap_or_default. The CiteElem
owner lists a different bounded carrier; retain this cross-owner integration
reconciliation, rather than adding a field from this prose alone.
Additional CSL-specific variants, automatic bibliography.style propagation
and full serialization/deserialization remain deferred.

## Equivalent Default: evidence and acceptance

Clippy identified the manual Default at citation_style.rs:22. The nongeneric
type introduces no public bounds through derive. Source/owners/spans/UTC/
status/diffstat at HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b with uncommitted
tree are in
`lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz`.
Use derive(Default) with #[default] Numeric, preserving the existing trait
and value. Do not change product bibliography style or claim new CSL parity.
This internal ADR-0127 correction changes no public signature/field/variant,
product default or phase. Preserve tests/expectations; validate workspace
all-targets, core suite/test inventory, build, Rustdoc, Clippy and lineage.
Acceptance includes Numeric default and all three exact as_str mappings.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/citation_style.md`. Historical observations are not a new parity certification.
