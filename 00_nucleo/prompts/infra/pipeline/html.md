# L0 — infra/pipeline/html
Hash do Código: 2772de8b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/html.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c
```

## Current evidence

- `03_infra/src/pipeline/html.rs:64` — `resolve_html_pretty`.
- `03_infra/src/pipeline/html.rs:71` — `compile_to_html_string_with_features_and_serialization_and_pretty_override`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/html.rs`.

## Owned contract

- Own the semantic HTML compilation facade, HTML option resolution and intrinsic `html.frame` preparation/position collection. The external evaluation target remains Html; its feature gate precedes evaluation and is never inferred from output extension or serialization mode.
- Evaluate, collect document format options, delegate contextual preparation, resolve intrinsic frame islands and export through the selected `HtmlSerializationMode`. Do not paginate the whole HTML document or create a global PagedDocument/PageStore.
- Resolve pretty once as `pretty_override.unwrap_or(options.html.pretty)`. Wrappers without a decision pass None; the explicit boolean wrapper passes Some. Deliver the same resolved decision to external HTML and inline SVG frames; inline frames do not consult `format.svg`.
- Crystalline/Vanilla modes affect encoding only, not features, content, context or diagnostics. No CLI/env access belongs here.
- Traverse frames in occurrence order with effective styles and invoke the intrinsic L1 paged facade locally. Nested frames collapse into the outer island; distinct occurrences retain distinct causal identities. Layout/render failure aborts rather than dropping a frame or emitting a placeholder.
- For actually observed HTML `location.position()` reads, supply intrinsic positions to selective stabilization before terminal marker projection. `location.page()` stays none: an intrinsic Page is never a public HTML page.
- Missing ids, collisions, ambiguous topology or unmatched attempts fail closed. General query-derived Location identity outside an island remains outside this contract; do not associate it by ordinal.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
