# L0 — stdlib/shapes/frame
Hash do Código: 5cc36a1e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/set/declared-properties.toml sha256:b4b40ae19e8c22e62a22af00459e257fa4a6c9d8c518d101cd0798cbf4f97744

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/shapes/frame.rs`
**Governance:** ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

Own the admission, normalization and construction shared by rect, square, circle and
ellipse, and the pure projection of declared properties onto every shape.

- One optional positional body: content, string, symbol or none. Anything left over
  fails with `unexpected argument` at its span. A body is never discarded.
- `circle.radius` becomes equal width and height, `square.size` the same; with either
  of them, width or height is `unexpected argument: NAME`. An auto radius or size
  leaves width and height free.
- width and height admit auto, relative length and, for height, fraction, plus the
  existing numeric extension. inset and outset admit relative lengths or a dictionary
  of sides; the default inset is 5pt. stroke admits auto, none, every stroke form and,
  for rect and square, a dictionary of sides. An auto or absent stroke is 1pt black
  without fill and nothing with fill, decided after declarations are applied.
- The lexical named arguments are kept with the element. Declarations fill what they
  omit: stroke folds by side and component, inset and outset by side, anything else
  is taken whole. An explicit auto stroke restores the default. Declared values are
  admitted only if this same construction accepts them; a declared fraction has no
  effect yet.
- With a body the result is a Block whose outline is the shape, so every traversal of
  a Block reaches the body once. Without a body it stays a leaf Shape.

Line, polygon and curve keep their own construction; only their omitted paint, and for
line its extent, is projected.

## Acceptance

RED→GREEN for each clause through the CLI and unit controls for normalization, folding
and leftovers. No shape loses its body.

## Evidence

Approved on 2026-11-24 (“Aprovo A–F”): `00_nucleo/diagnostics/shapes-package-2026-11-24.pt-BR.md`.
