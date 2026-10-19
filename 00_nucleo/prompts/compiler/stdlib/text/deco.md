# L0 — stdlib/text/deco
Hash do Código: 19f869cc

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/deco.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/text/deco.rs:32`, `01_core/src/compiler/stdlib/text/deco.rs:38`, `01_core/src/compiler/stdlib/text/deco.rs:161` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own native_underline/strike/overline, their private DecoKind/build_decoration parser, highlight and its local default color helper. Require one Content|Str body. Lines accept stroke: typed Color|none, offset/extent: Length|Int|Float|none (bare numbers are points), emitting their corresponding Content carriers. Default render offsets remain +0.10em underline, -0.25em strike, -0.80em overline; rendering belongs to layout, not this constructor. Rich Stroke remains outside this specified line parser. evade/background retain educational scope-out diagnostics, including their literal historical P284/ADR references as message text, not as current architectural ownership. Other named errors preserve `{fn}(): argumento nomeado inesperado '{name}'`.

highlight accepts fill: typed Color|none (default RGBA 255,242,54,255; none disables), radius/extent: Length|none, and emits Content::highlight_full with its style carriers. Gradient fill and math-mode scope limitations remain; radius/extent are supported, not historical scope-outs.

Only Value::Color is a color value here: a string such as "red" is not a color alias; named color constants work after evaluation. Preserve typed rejection diagnostics, none/defaults, geometry and carriers. Reuse the existing shapes color coercion helper; do not add a replacement CSS table or fallback. Do not render, read World/context or introduce global state. Keep private helpers private.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
