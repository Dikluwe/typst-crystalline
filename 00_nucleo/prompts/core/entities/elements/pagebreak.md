# Prompt L0 — `entities/elements/pagebreak` — `PagebreakElem`
Hash do Código: e846ce46

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/pagebreak.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

Historical content.rs:1788,1614 distinguish empty text from a nonempty flow
command. The archived correction on 2026-08-13 explicitly resolves the former
Hash contradiction in favor of derive: pagebreak.rs:20 and parity.rs:26
already derived Hash. This is not a new algorithm choice.

## Contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PagebreakElem {
    pub weak:          bool,
    pub weak_explicit: bool,
    pub to:            Option<Parity>,
}
```

Preserve Content::Pagebreak(Arc<PagebreakElem>) and
`Content::pagebreak(weak: bool, to: Option<Parity>)` using entities::parity.
weak_explicit distinguishes omission from explicit false in repr. Existing
constructor true is explicit and false omitted. Evaluation uses
pagebreak_with_weak_presence(weak, weak_explicit, to); layout reads weak,
with to unchanged. All three fields participate in equality and derived Hash.
Parity's canonical Hash is an existing direct dependency, owned by
`00_nucleo/prompts/core/entities/parity.md`. Do not replace it with Debug hash:
Copy+Eq enums without floats use canonical derive; the HSpace/VSpace float
caveat does not apply. This owner does not re-own Parity's implementation.
Plain text is empty, is_empty always false, and both maps are terminal.
Field/kind/payload default None, non-locatable. Verify nonempty flow identity,
empty text, terminal maps, all-field equality and derived Hash.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/pagebreak.md`. Historical observations are not a new parity certification.
