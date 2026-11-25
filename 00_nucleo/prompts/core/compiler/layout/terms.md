# L0 — terms
Hash do Código: a0f87f3d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/terms.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Contract

Own the paged layout of the list container (`TermsElem`), for bullet lists, numbered
lists and term lists. The container is block-level.

- The origin is the line start of the active region, never the page margin.
- A tight container that immediately follows a paragraph, without a paragraph break,
  is separated from it by its spacing or the paragraph leading. Every other outer gap
  is the ordinary block spacing. Between items the gap is `spacing`, else leading when
  tight and paragraph spacing when wide.
- Properties are the lexical ones, then the declared ones, then the defaults:
  indent 0, body-indent 0.5em, marker-align and number-align end, hanging-indent 2em.
- The label column is as wide as the widest label and never wider than the region.
  Each body is a breakable block inset by indent, label column and body-indent on the
  start side, so it begins beside its label. Without a vertical alignment the label
  shares the first baseline of the body; whichever stands lower decides the line.
  With one, the label is placed on the item frame. Outer alignment moves bodies only.
- A bullet marker is chosen by nesting level, cycling through the list of markers.
- Numbers start at `start`, else 1, or the number of items when reversed. An explicit
  item number resets the count, which then goes up or down and never below zero. A
  pattern uses the counting symbol of the level; with `full` all parent numbers.
- A term is strong, followed by the separator and the description; continuation lines
  hang by hanging-indent after indent.
- Labels prepared by realization are read from the container. Source children follow
  their typed plan edges `TermsItem(index)`, `ListItemBody`, `EnumItemBody`,
  `TermItemTerm` and `TermItemDescription`; generated labels stay unplanned.

## Acceptance

RED→GREEN through the CLI for nesting, containers, number alignment, heading and shape
bodies, spacing, reversed, full and terms. Page breaks, columns, RTL, notes, counters
and contexts inside items keep one effect each.

## Evidence

Approved on 2026-11-24 (“Aprovo A–G”): `00_nucleo/diagnostics/lists-package-2026-11-24.pt-BR.md`. The previous text and its
measurements are in commit `f2c737125`.
