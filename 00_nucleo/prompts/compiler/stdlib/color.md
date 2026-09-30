# L0 — stdlib/color
Hash do Código: 5adad83e

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/color.rs`

Após a atomização, este consumer possui o despacho estático do tipo color, o mapa pinado e os testes históricos. Os Prompts L0 individuais em `00_nucleo/prompts/compiler/stdlib/color/` possuem paleta, validação de argumentos, operações de luz, mix, transformações, inspeção, alpha e Spot. As cláusulas abaixo permanecem contexto normativo, sem nova semântica.

## Evidence and classification

`01_core/src/compiler/stdlib/color.rs:31` (fn err_typed<T>msg: impl Into<String>) -> SourceResult<T> {); `01_core/src/compiler/stdlib/color.rs:52` (pub fn color_type_fieldfield: &str) -> Option<Value> {); `01_core/src/compiler/stdlib/color.rs:94` (const COLOR_MAP_DATA: &str = include_str!"color_maps.hex");) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own color_type_field and the pinned color map; child prompts own predefined_color_bindings, process/spot operation adapters and validation. The global registry and entity-side color mathematics remain separate. Preserve the existing static/bound paths and native short names.

Qualified constructors rgb/luma/oklab/oklch/linear-rgb/cmyk/hsl/hsv delegate to their foundation native owners; no new bare hsl/hsv/linear-rgb aliases. Preserve the predefined namespace palette and native CMYK representation, not an sRGB approximation of publicly observed color space/components. Existing cyan/magenta global compatibility extras remain distinct from color namespace membership.

Preserve lighten/darken, mix, negate, rotate, saturate/desaturate, to-hex, transparentize/opacify, components and space operations, including their ratio/angle/type casts, alpha and channel semantics. Unknown members remain errors. map preserves the exact predefined map/control data; do not replace it by interpolation or a shortened palette because of an old extraction error.

mix accepts a variadic nonempty sequence of Colors or (Color, Float|Ratio weight) pairs. Malformed pairs fail `expected a color or color-weight pair`; a nonpositive total fails `sum of weights must be positive`; hue-based spaces reject more than two colors. auto uses Oklab for process colors. The bound receiver is the first color of weight one. The crystalline weight extension is accepted only for exactly two bare colors, using 1-weight and weight. Preserve space validation and ColorSpaceError messages/hints.

color.spot is the SpotColorant callable type. Its name is String or None (including the special string all), with a mandatory process Color fallback; nested spot fallback is rejected. Preserve the spot type/repr and missing/type diagnostics. tint exists as a static self-first function and bound call syntax, not an independent bound field value. Require a Ratio in [0%,100%], preserve cast/range spans and positional-name/extra diagnostics; no Float/String coercion. No public fallback method.

Spot Color operations preserve colorant identity: lighten clamps t-t*factor, darken clamps t+(1-t)*factor, negate gives 1-t. components without alpha exposes tint; space returns the SpotColorant; to-hex uses the tinted fallback; rgb is explicit process conversion. Default mix preserves spot only for the same complete colorant, otherwise Oklab. An explicit spot space requires compatible colorants. Do not invent hue/alpha/saturation behavior when the current operation diagnoses unsupported spot color.

The older no-tint scope-out is superseded by the later tint contract, not retained as a competing rule. This unit's language support is not proof of exporter Separation output or general color parity.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
