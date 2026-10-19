# Prompt L0 — `entities/numbering` — typed numbering and realized views
Hash do Código: bec78c90

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/numbering.rs`
**Governance:** ADR-0024, ADR-0107, ADR-0108, ADR-0127.

## Evidence before specification

At historical upstream `a51e02804`, model/numbering.rs:99–128 defines
Pattern/Func; layout/page.rs:290–311 documents visible current+total versus
unary reference callbacks; pages/run.rs:155–168 treats every function as
total-dependent; layout/introspect.rs:122–125 preserves raw Numbering per page.
Recorded bilateral probes show: binary callback on two pages → V1/2,V2/2;
variadic callback with explicit footer and page ref → R1; page-numbering query
preserves closure repr `(..) => ..`; logical counter starting 7 over three pages
→ N7/9,N8/9,N9/9 with first reference N7; lexical callback/pattern runs →
X1/2,II; unary visible callback errors `unexpected argument`, ternary errors
`missing argument: c`; integer return 42 displays 42, none is empty; explicit
footer suppresses binary marginal callback but unary reference still gives
AUTO1. These are historical probes, not fresh current-pin certification.

## Owned contract

```rust
pub enum Numbering {
    Pattern(EcoString),
    Func(Func),
}
```

Pure language data, O(1) clone, no I/O. Pattern preserves the original string;
parsing/formatting belongs to
`00_nucleo/prompts/core/compiler/stdlib/numbering.md`. Func preserves its
captured closure; equality/hash follow existing Func identity, never rendered
results or bytes. A delta's outer None means omitted/retain active value;
Some(None) disables numbering; Some(Some(Numbering)) installs a pattern or
function. Resolved page/config snapshots use Option<Numbering>, not a delta.

## Realization boundary

Never execute callbacks in this entity. The Engine fixpoint creates separate
visible `[current,total]` and reference `[current]` views per page. Convert
returns through normal language content semantics: Content unchanged, Str as
text, None as empty, others through ordinary markup display. Errors retain
the numbering-setting span. Raw Numbering::Func remains available even though
margin/reference consume realized content with different arities.

## Acceptance and historical gate

Preserve every recorded callback/arity/logical-counter/view/return/error case
above. The original public-entity/field change required explicit ADR-0127
approval before implementation. The header of
`lab/history/construction/records/00_nucleo/materialization/typst-passo-1158.md`
records the approved contract and phase; the later record reports completed enum/casts/
deltas/snapshots/typed Introspector and then callback views with
layout→Engine→relayout. Preserve that succession without turning implementation
status into a fresh approval for further public or phase changes. This owner
implements only the typed carrier; realization stays with its compiler owners.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/numbering.md`. Historical observations are not a new parity certification.
