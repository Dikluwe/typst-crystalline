# L0 — infra/shaper/shaping
Hash do Código: d26779af

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/shaping.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
```

## Current evidence

- `03_infra/src/shaper/shaping.rs:21` — `ssty_level_for_subrun`.
- `03_infra/src/shaper/shaping.rs:38` — `shaped_width`.
- `03_infra/src/shaper/shaping.rs:106` — `try_shape`.
- `03_infra/src/shaper/shaping.rs:366` — `byte_idx_to_char`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/shaping.rs`.

`00_nucleo/diagnosticos/infra-performance-followup/language-control.json` shows
that explicit ja/zh/ko settings do not affect Tekt glyph selection while a same-face
rustybuzz control does. Both local buffer sites omit language configuration.
Pinned vanilla configures language at
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980`.
Language selection is a style semantic input; glyph IDs are discriminating render
controls, not proof of full language parity. The cache structure is mechanics.
The repository Noto Sans fixture supplies independent ru/sr local substitutions
with different advances; controls must cover measurement and emission separately.

## Owned contract

- Own shaped_width, try_shape, UTF-8 cluster mapping, rustybuzz setup, variations, tracking, ssty and TextShaped materialization. Use sibling bidi/selection/cache helpers rather than duplicate their policies.
- Clone the cached base face before applying axes. Derived variant axes are merged with explicit style variations, explicit values winning by tag; preserve nonvariable/unknown-axis no-op behavior.
- Configure each width-measurement and emission rustybuzz buffer with the explicit TextStyle.lang when present, using its normalized language code. Leave language unset when absent, preserving the existing inferred/default behavior. Pass the same optional explicit language identity to the run-cache owner. Set direction per bidi run; preserve language, offsets, glyph ids/advances and clamped positive units_per_em. Each emitted TextShaped.text is the sub-run text and glyph clusters are UTF-8 byte offsets relative to that text, with replacement character fallback for an invalid mapping. BidiRun.byte_start is a separate original-input coordinate owned by the bidi child. Whitespace-only input takes the unchanged-Text path; traversal owns the rule against reshaping TextShaped.
- Measure multiline width as the maximum paragraph width. Emission resets x for each paragraph and advances y by the first primary's resolved top/bottom edges plus style leading (default 0.65em). This does not repair L1 vertical flow/pagination for an embedded newline.
- Preserve the existing distinct measurement/emission feature and tracking paths rather than harmonize them speculatively. In emission calculate the rounded i32 tracking increment once per candidate from resolved tracking points, positive units_per_em and style.size, using the existing conversion formula. Pass that same increment to the cache key and add it only before a following different cluster, never inside a cluster or after the last glyph of a sub-run; negative tracking remains valid.
- Math mode is decided by style.math. Consume the canonical ssty_eligible_text from font_metrics; request levels 1/2 only for eligible Script/ScriptScript outside math_text_item. TextItem keeps math family/target features and every non-ssty property but no explicit ssty feature (`None`), also represented as absence in its cache key.
- Preserve forced math script behavior independently of requesting ssty and existing default OpenType features. No family/codepoint/fixture exception, speculative feature change, vertical shaping or nonzero face-index support is introduced.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
