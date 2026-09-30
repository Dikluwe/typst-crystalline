# L0 — Font variation helpers and static instancing
Hash do Código: f4ea81ad

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_variant.rs`

## Current evidence

- `03_infra/src/font_variant.rs:27` — `text_style_to_font_variant`.
- `03_infra/src/font_variant.rs:45` — derived axes.
- `03_infra/src/font_variant.rs:76` and `:87` — explicit variation merging.
- `03_infra/src/font_variant.rs:130`, `:174`, `:199` — inline Python script, availability probe and instancing.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own conversion between the existing L1 `TextStyle`/`FontVariant` and L3 OpenType axes, variable-font detection, the subprocess availability probe and static instancing. The script is embedded as `INSTANCER_SCRIPT` in this Rust consumer; there is no separate Python-file consumer.

An explicit weight wins over `bold`; otherwise use bold or regular weight. `italic` selects Italic, otherwise Normal; stretch remains Normal. Derived axes include `wght` only for weight other than 400 and `ital=1` for Italic. No `wdth`, oblique-angle or `slnt` mapping is introduced. `axis_variations_for_text_style` and `merge_explicit_variations` replace a derived value by the explicit value with the same tag and append new explicit tags.

`is_variable_font` parses face zero and detects `fvar`; invalid bytes return false. `instantiate_variable_font` returns unchanged copied bytes for empty variations, a nonvariable font or no matching font axes. Otherwise filter requests to available axes and invoke the embedded script with binary stdin/stdout. Remove `GPOS`/`GSUB`/`GDEF`, fix every unrequested axis at its default and use `fontTools.varLib.instancer.instantiateVariableFont` to produce static bytes. Failed process setup, input writing or completion returns `None`; failed exit preserves the existing stderr message. The caller owns whether failure is fatal or a fallback.

Use `TYPST_CRYSTALLINE_PYTHON` when configured, otherwise `python3` from PATH. `variable_font_instancer_available` probes `import fontTools` without forwarding subprocess output. Python/fontTools are runtime dependencies of the active instancing path.

`00_nucleo/prompts/infra/pipeline/resources.md` owns early resource validation; the PDF builder's font owner owns subsetting and the point at which it invokes instancing. Font-key identity and exporter dispatch are not owned here. Parser and rustybuzz types remain in L3.

## Verification boundary

Existing inline tests distinguish derived axes, explicit overrides, additional axes and explicit-only requests. Resource/export tests verify instancing integration under their own owners; this prompt does not impose an unconditional default-font fallback on them.
