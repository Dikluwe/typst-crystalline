# Prompt L0 — CitationForm
Hash do Código: 86a8960c

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/citation_form.rs`
**References:** ADR-0034/0037/0054/0064 case A/0065 criterion 2.

## Evidence before bounded contract

Historical upstream e0e8ca4d model/cite.rs:133-147 defines exactly five forms:
Normal (active-style default), Prose (sentence-friendly), Full (complete
bibliography-entry information), Author and Year. The original claim of
additional style-specific enum variants was unsupported. cite.rs:85 defaults
to Some(Normal); lines 75-76 specify that form:none includes a work in the
bibliography without displaying it. CiteElem's Option can carry absence,
but the historical None↔Normal layout fallback differs from that observable.
Retain this integration gap; a type declaration alone does not prove rendering.

## Domain contract

The bounded crystalline enum has Normal,Prose,Author,Year, deriving
Debug/Clone/Copy/PartialEq/Eq/Hash. Default is Normal. as_str(&self)->&'static str
returns "normal","prose","author","year" respectively. The dedicated enum
provides type safety and exhaustive matching rather than Option<String>.
Hash supports CiteElem's Option<CitationForm> and is safe for these unit,
nonfloating variants. Preserve all variants and exact strings.

The original fallback rendering was Normal or missing-key→[key],
Prose→Author (Year), Author→Author, Year→Year, with same-document bibliography
lookup through CounterState::bib_entries. Native cite parses optional named
form strings strictly/case-sensitively and rejects invalid strings with a
message listing valid forms. These rendering/parser mechanisms have separate
owners; entity cleanup does not restore old placeholders over later CSL work.

## Pending scope and reconciliation

Full is the specifically measured missing upstream form; broader CSL forms
and real rendering were historically deferred under ADR-0054/hayagriva ADR-0062.
Current rendering closure must be checked in its owner, not inferred from this
historical deferral. The original ADR-0064 flattening of Smart<Option<...>>
to Option and unwrap_or_default requires reconciliation with explicit none's
hidden-but-included observable. Adding enum variants is not automatically
Rust-compatible for exhaustive consumers, despite the older claim.

## Equivalent Default: evidence and acceptance

At citation_form.rs:44 Clippy identified manual Default, whose existing value
is Normal. The nongeneric type adds no public bounds through derive. Sources/
owners/spans/UTC/status/diffstat at HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
uncommitted tree, are in
`lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz`.
Use derive(Default) with #[default] Normal, preserving the existing trait/value,
all variants/as_str, public signatures/fields, product default and phase.
ADR-0127 internal continuous flow; no new parity/performance claim.
Check all four as_str values, Normal default, differing variants unequal,
and derived Hash. Preserve tests/expectations; validate workspace all-targets,
core suite/test inventory, build, Rustdoc, Clippy and lineage.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/citation_form.md`. Historical observations are not a new parity certification.
