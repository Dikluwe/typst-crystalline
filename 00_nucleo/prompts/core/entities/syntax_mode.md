# Prompt L0 — `entities/syntax_mode`
Hash do Código: 235d96f4

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/syntax_mode.rs`

## Evidence and responsibility

The recorded upstream reference is
`lab/typst-original/crates/typst-syntax/src/set.rs`. Lexer/parser use distinct
lexical rules by context; the mode is a state marker, not an algorithm owner.
Markup is top-level; parsing explicitly enters/leaves Code around #/{/} and
Math around dollar delimiters. Exact transition parsing belongs to the parser.
The old prose called `*` italic while its own example used *bold* and _italic_;
this inconsistent example is historical, not a new lexical rule.

## Contract

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SyntaxMode {
    Markup,
    Math,
    Code,
}
```

Pure L1, no I/O, global state or external dependencies. All three variants
are unit state markers with Debug/Copy/Clone/Eq/PartialEq/Hash and no fields.

## Acceptance

Copying Markup leaves the original usable and equal. Markup, Math, and Code
are pairwise distinct. Debug emits exactly Markup, Math, or Code. Keep local
tests and lineage pointing to this sole owner.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/syntax_mode.md`. Historical observations are not a new parity certification.
