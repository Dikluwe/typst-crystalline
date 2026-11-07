# L0 — 03_infra/src/svg_image.rs
Hash do Código: d6e37ce8


**Status:** Approved by owner 2026-11-01; approval at `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`.
**Exclusive consumer:** `03_infra/src/svg_image.rs`.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896

## Measurement before decision

See `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md` and oracle.json/tekt-before.json, with fixed commit/tree/binary and timestamps. Existing integer raster sizing and missing resource origin cannot faithfully transport measured SVG behavior. Public DTO/backend representation is an implementation choice; source semantics, geometry and visible paint are acceptance observables.

## Contract

Own the private World-aware adapter and SVG preparation. Delegate all raster metadata methods to ImageSizeImageSizer. Decode bounded gzip/SVGZ, validate XML/root/positive finite size and SVG namespaces, preserve CSS 96dpi/viewBox/units/fractions. Resolve relative resources from SvgOrigin using injected World/root/package restrictions, never a fabricated byte-image filename or unrestricted OS path. Use current compile font catalog and effective family chain at each occurrence; resolve fonts/resources and outline text into a self-contained SVG tree, return only PreparedSvg. Return errors/warnings, never silently synthesize successful empty paint. Cache only per compilation with raw data, origin and effective font context in identity; span diagnostics remain per occurrence. No foreign types in public contracts. Limit recursion/decompression/output allocations and detect resource cycles. Preserve valid intentionally empty SVG.

## Acceptance

L0 activation and owner/hash/pins precede module behavioral RED then GREEN. See proposal.md matrix and RAM/build/lint gates. Do not claim implementation or assign a code hash before this module exists.


## Compatible private backend

Use exact svg2pdf 0.13.0, its usvg 0.45.1 and resvg 0.45.1; L3-only pdf-writer 0.12.1 supplies chunk renumbering/xref offsets for integration. Private selection follows inspected crate manifests and APIs, not a claim of parity from version identity. Do not use filesystem/default href resolvers; XML/gzip bounds and World resolvers are mandatory.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Private SVG preparation uses bounded gzip/XML, injected World/font database, ordered families and self-contained render serialization.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Ordered SVG-declared family fallback must precede effective Typst context families and then allowed global fallback. Measurement: pinned embedded c0ae970 fonts contain U+0531 in DejaVu Sans Mono and NewComputerModern10 but not Libertinus Serif regular; usvg text/layout.rs:194 caches font selections before shaping chunks, so sharing a primary face ID cannot identify different span family chains. Test U+0531 and keep each font specification distinct internally without public contract changes.


## Font fallback identity after measured RED

`declared-font-red-2.log` in the 2026-11-01 SVG receipt reproduces U+0531 choosing context NewComputerModern10 before SVG-declared DejaVu Sans Mono. CSS Fonts defines family order by glyph coverage ([W3C CSS Fonts 3 §3.1](https://www.w3.org/TR/css3-fonts/#font-family-prop)); SVG uses that font selection policy. This is language styling, not a requirement to copy usvg's identity mechanics. Its text/layout.rs preselects font specs before shaping; duplicate primary IDs lose the ordered chain. In the private per-tree database, assign a distinct face ID to each selected specification (shared immutable face data), retain its ordered SVG/context family plan keyed by that ID, and consult it before allowed global fallback. No World IO, source mutation, public signature or render-style baking into source; separate span plans and aliased occurrences must remain distinct.


## Fallback face variant refinement

The same U+0531 RED remained after preserving family identity: iterating every face of a family may select Bold before Regular. Embedded c0ae970 DejaVu Sans Mono has regular/bold/oblique faces covering U+0531; direct CSS normal selection uses Regular. Store style/weight/stretch alongside each ordered per-spec plan, query the closest matching face for each declared/context family, and test coverage on that face. Only then use permitted global fallback ranked by style/stretch/weight; do not let database insertion order change the requested SVG text variant. Preserve the geometric refuter and its tolerance.


## Font demand refinement before code

`font-load-before.json` preserves the pilot binary/source receipt and per-command UTC; font-sc/JP/alias requests spend roughly 13–21 seconds between command records while simple vector paths do not acquire the catalog. This is an infrastructure-demand observation, not language parity or an isolated benchmark. Source inspection confirms fonts() eagerly asks World.font for every catalog entry before selecting one family. Replace that eager acquisition with per-tree lazy loading: consult FontBook family/variant/coverage metadata, request only the selected World font slot, preserve per-spec family/variant plans, then consult coverage-ranked global candidates only when allowed and needed. Cache loaded slot-to-face IDs only in that preparation tree; World retains its normal byte cache. Preserve all glyph/geometry/source refuters and test that unused font providers are not demanded. No public schema, phase or source change.


Coverage refinement: current `03_infra/src/fonts.rs:425` initializes Coverage::new() and resolves actual coverage lazily elsewhere. An empty metadata set is therefore unknown, not evidence that the face lacks the character. Declared/context and global fallback must try actual face cmap when metadata is empty; nonempty coverage may exclude unavailable characters. Do not change the shared font metadata contract in this task.


## SVG XML prolog parity

Pinned usvg parser/mod.rs explicitly parses with allow_dtd=true before converting the tree. The preparer's preliminary default Document::parse would reject ordinary SVG 1.1 PUBLIC DOCTYPE prologs used by exporters. Admit that XML prolog with the same parser option and convert the already-validated document through Tree::from_xmltree, avoiding a second parse. External DOCTYPE declarations do not fetch filesystem/network resources. Add a RED for a valid SVG 1.1 external DOCTYPE and verify paint plus zero resource-port reads, then retain malformed/UTF8/gzip/size/cycle controls. No public, phase or mode change.

## Private options initialization

`03_infra/src/svg_image.rs:225-228` initializes default usvg options and then
assigns dpi and font_family. Initialize these two fields in the options literal
with the remaining fields defaulted; preserve 96dpi, ordered effective family
selection, subsequent injected resolvers and all resource/font behavior. This
is an internal Clippy correction, with no schema, phase or output change.
