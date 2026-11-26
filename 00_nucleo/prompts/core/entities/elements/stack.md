# Prompt L0 — `entities/elements/stack` — `StackElem`
Hash do Código: 9ffa83cf

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/stack.rs`
**Architecture:** ADR-0105. The trait contract is owned by
`00_nucleo/prompts/core/entities/elements/mod.md`.

## Evidence before specification

Original extraction anchors: `content.rs:1681`, `content.rs:1850`, `content.rs:2023`, `content.rs:2192`, `content.rs:2463`.
These historical coordinates support the ownership boundary; they are not
fresh current-pin language-parity measurements.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct StackElem {
    pub children: Arc<[Content]>,
    pub dir:      Dir,
    pub spacing:  Option<Length>,
}
```

## Behavior and acceptance

Preserve `Content::Stack(Arc<StackElem>)` and
`Content::stack(children: Vec<Content>, dir, spacing)`, converting children to
`Arc<[Content]>` for O(1) clone (ADR-0026). PartialEq compares all three fields.
Hash remains manual through Debug because spacing contains f64; retain the
known signed-zero caveat (`-0.0` versus `0.0`).
Plain text concatenates children in order. Emptiness requires every child to
be empty. Both maps recurse through children, preserve dir/spacing, and
`map_content` collects SourceResult. Field/kind/payload use defaults;
non-locatable. Verify ordering, all-child emptiness, maps, and equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/stack.md`. Historical observations are not a new parity certification.

## Layout family package

`given` records which declarable properties the call gave.

Approved on 2026-11-26 (“Aprovo A–E”): `00_nucleo/diagnostics/layout-package-2026-11-26.pt-BR.md`.
