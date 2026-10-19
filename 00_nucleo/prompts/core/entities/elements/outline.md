# Prompt L0 — `entities/elements/outline` — `OutlineElem`
Hash do Código: 382e6921

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/outline.rs`
**Architecture:** ADR-0105 model D; user element, outside DEBT-58.

## Contract

Content::Outline holds Arc<OutlineElem>. OutlineTarget derives Debug/Clone/
Copy/PartialEq/Eq/Hash/Default, with default Headings plus Figures and Tables.
Targets select heading TOC, Introspector::figures_for_lof and tables_for_lot.
OutlineIndent derives Debug/Clone/PartialEq/Hash with Auto,Bool(bool),
Length(Length),Function(Func). Historical upstream 0.14.2 accepted length/
function/auto; Bool preserves crystalline compatibility. Auto/Bool(true)
activate heading indentation, Bool(false) disables it. Length/Function are
accepted/stored; specific rendering remains a separately scoped ADR-0054 debt.

OutlineElem derives Debug/Clone/PartialEq/Hash, not Eq (Content lacks Eq),
and has public title:Option<Content>, depth:usize, indent:OutlineIndent,
target:OutlineTarget, plus the private presence state specified below.
Depth/indent apply to headings. Custom title is preserved; omitted title uses
target defaults. The original literals "Índice", "List of Figures", "List of
Tables" and later localized-default evidence are historical product behavior,
not permission to force a new language or erase localization.

Content::outline() uses outline_with(None,3,Auto) with Headings;
outline_with(title,depth,indent) preserves those inputs. lof(title)/lot(title)
use Figures/Tables, depth 1 and Bool(false). with_target(title,depth,indent,target)
returns exactly those four fields. The 2026-08-13 source check at outline.rs:87-94
corrected an invalid two-argument example/raw bool: preserve the four-argument
signature and typed OutlineIndent.

plain_text is empty; is_empty defaults false. Both maps recurse into title
and preserve depth/indent/target. get_field defaults None; element_kind is
Outline, to_payload Outline. Keep locatable/queryable payload-based consumers.
Equality/hash compare all public fields and private presence state.

## Explicit title: evidence and ratification

At baseline fe3209458, omitted title and title:none collapsed into Option and
both generated Contents; measured upstream omission localizes the default,
explicit none suppresses title. Human approval on 2026-09-15 authorizes private
supplied-title state without a new Typst-reflectable field. Omitted resolves
default, explicit none has no title, supplied Content survives. Clone/hash/
equality/map_content retain presence; internal constructors explicitly choose
one of three forms. This approval supersedes the archived PROPOSED heading.

## Explicit indent: evidence and contract

Baseline2ff1a23d4 could not distinguish default auto from explicit indent:auto,
needed to apply a set rule only to omission. Preserve private supplied-indent
state without changing existing public signature or adding a Typst field.
Clone/hash/equality/both maps and causal realization identity preserve it.
An internal operation may replace omitted indent with inherited style and
mark it resolved; explicit values, including auto, are never overwritten.
Acceptance must preserve these presence distinctions, defaults, locatability
and recursive title transformations.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/outline.md`. Historical observations are not a new parity certification.
