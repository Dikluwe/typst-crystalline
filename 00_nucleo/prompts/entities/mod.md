# L0 — Domain entity facade
Hash do Código: fdc4e847

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/mod.rs`

## Responsibility

The consumer declares the static L1 entity modules, the inline `frame_visitor`
reexport facade (`walk_frame_items`, `FrameVisitor` from `layout_types`) and
the private `p1431_line_cap_tests` module under `#[cfg(test)]`.

Preserve its public module surface, including `font_variations` and
`document_format_options`. Each child owns its domain behavior. The facade
must not duplicate defaults, parsing, document extraction or export precedence.
Format carriers belong to `00_nucleo/prompts/entities/document_format_options.md`;
font axes belong to `00_nucleo/prompts/entities/font_variations.md`.

Verify static declarations and test-only gating. No new reexports, removal of
modules or business logic is authorized by this documentary cleanup.
