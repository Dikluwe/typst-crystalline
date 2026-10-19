# L0 — Public HTML frame integration tests
Hash do Código: 5507d76e

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/html_frame.rs`

## Current evidence

- `03_infra/tests/html_frame.rs` — public facade cases.
- `00_nucleo/prompts/infra/pipeline/html.md` — HTML orchestration.
- `00_nucleo/prompts/infra/export/html.md` and `00_nucleo/prompts/infra/export/svg.md` — excluded exporter owners observed by the test.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Exercise the public HTML facade. Disabled HTML feature fails; enabled feature exposes `repr(html.frame) == "frame"` and preserves the existing three signature diagnostics. Text produces one unescaped, block-level inline SVG root with positive intrinsic dimensions, no literal frame tag and no A4 canvas.

A 10pt by 5pt rect using public `red` preserves physical dimensions, viewBox `0 0 10 5` within 0.001pt and color `#ff4136`; CSS `#ff0000` is not a replacement. Nested frames produce one island with the simple island's geometry/paint. `html.elem("frame")` remains a literal tag; the route without frames preserves the existing DOM.

The facade rejects an HTML-only frame body unsupported by the paged island instead of succeeding with empty SVG. Direct export of an unresolved carrier fails closed, without omission, textual fallback or literal frame tag. IDs, attribute order, paths and SVG byte equality are not acceptance. Do not adapt expectations to the candidate implementation.

## Verification boundary

Coverage is limited to public html.frame behavior, not authorship, manifests, hashes or external reports.
