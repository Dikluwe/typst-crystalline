# L0 — Term list carrier
Hash do Código: f60a6513

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/terms.rs`

## Contract

`TermsElem` carries consecutive items of one kind: term list, bullet list or numbered
list. Besides `items` it holds, privately to the core, the kind, the lexical named
arguments and how many trailing entries of `items` are labels prepared by realization.
`with_items` rebuilds the element keeping all of that, and every traversal uses it.
`children` are the items without the prepared labels. Plain text excludes the labels.

## Acceptance

Traversal preserves kind, named arguments and labels; equality distinguishes kinds.

## Evidence

Approved on 2026-11-24 (“Aprovo A–G”): `00_nucleo/diagnostics/lists-package-2026-11-24.pt-BR.md`. The previous text and its
measurements are in commit `f2c737125`.
