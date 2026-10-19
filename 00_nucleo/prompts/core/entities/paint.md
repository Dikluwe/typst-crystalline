# Prompt L0 — `entities/paint`
Hash do Código: 41cc159a

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/paint.rs`
**Governance:** ADR-0086, ADR-0083, ADR-0039.

## Evidence and specified subset

The original Paint introduction compared upstream Solid/Gradient/Tiling with
a deliberately Solid-only wrapper for Stroke.paint. Its complete chronology
and reserved expansion comments remain in the archive. This owner has no
recorded later normative expansion in that original text; actual richer
consumers do not automatically authorize rewriting its contract.

## Contract retained from the owner

The declared subset is Paint::Solid(Color), deriving Debug/Clone/Copy/PartialEq.
`solid(c: Color) -> Self` wraps c, `to_color(&self) -> Color` returns the solid
color without panic, and From<Color> returns Solid. No external dependencies.
Gradient/Tiling were comments reserving possible consumer-driven expansion,
not unit placeholder variants. The specified facade is public module paint and
`pub use paint::Paint`, owned by `00_nucleo/prompts/core/entities/mod.md`.

The original preserves TextStyle.fill: Option<Color>, Style::Fill(Color), and
native color constructors returning Value::Color. It introduces no Value::Paint;
Paint::Solid is an internal wrapper. Stroke.paint is the affected boundary,
owned by `00_nucleo/prompts/core/entities/geometry.md`; the Color owner is
`00_nucleo/prompts/core/entities/color.md`.

## Pending reconciliation and acceptance

Later Gradient/Tiling and text-paint requirements elsewhere require explicit
owner reconciliation with this Solid-only contract. Do not remove those
features or silently infer a replacement contract from implementation. The
original scope excludes panicking unwrap_solid (replaced by total to_color
within Solid-only), relative(), and as_decoration(); text gradients were a
future candidate. These limits are preserved as unresolved owner coverage.
Verify `Paint::solid(c).to_color() == c`, `Paint::from(c) == Paint::Solid(c)`,
PartialEq/Copy and no external dependencies within the declared subset.
This cleanup does not certify compatibility of that old subset with every
current consumer.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/paint.md`. Historical observations are not a new parity certification.
