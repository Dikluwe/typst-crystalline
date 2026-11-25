# L0 — compiler/stdlib/structural/lists — list constructors
Hash do Código: 66f43bca

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/lists.rs`

## Contract

Own `list`, `enum`, `terms` and their items.

- `list` and `enum` return one container, including for zero or one item. Positionals
  are items or displayable content; an item of the same kind is kept, anything else
  becomes the body of a new one. `terms` takes items or two-element arrays, and the
  existing named pairs remain accepted as an extension.
- `list`: marker (string, symbol, content, non-empty array or function), marker-align,
  indent, body-indent, spacing (length or auto), tight. `enum`: numbering (string or
  function), start (auto or non-negative integer), full, reversed, number-align,
  indent, body-indent, spacing, tight. `terms`: separator, indent, hanging-indent,
  spacing, tight. An unknown name or type fails at its span. An empty marker array
  fails with `array must contain at least one marker`.
- A negative start or item number fails with `number must be at least zero`.
- Named arguments are kept lexically. `declare` admits a declared value only if the
  constructor accepts it, and stores it under `ELEMENT.PROP`; `property` reads the
  lexical value, else the declared one.

## Acceptance

RED→GREEN through the CLI for each form, error and declared property.

## Evidence

Approved on 2026-11-24 (“Aprovo A–G”): `00_nucleo/diagnostics/lists-package-2026-11-24.pt-BR.md`. The previous text and its
measurements are in commit `f2c737125`.
