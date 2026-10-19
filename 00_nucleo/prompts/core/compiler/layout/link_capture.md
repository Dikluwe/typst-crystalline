# L0 — Private link capture scopes
Hash do Código: 6cf52551

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/link_capture.rs`

## Evidence before specification

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

`links-captura-2026-10-14/architecture.sarif` records V2: the new module lacks its own tests, while flow tests live in the Link owner.

## Ownership and contract

Own a private destination/lexical-scope registry and neutral open/close boundaries encoded through existing FrameItem variants (Semantic Artifact Other Inline, own NUL prefix, one zero Rect Shape without fill/stroke). Recognize exactly this format. The registry allocates no Location and executes no closure. Scope start/end append independent boundaries to line and committed buffer; line closure segments both before content transfer and reopens continuations afterwards. Region closure segments the committed buffer. Subframes have local pairs, and outer buffers retain their own pairs.

Resolve the tree after fixups and before document publication: consume boundaries, preserve every frame's order/region and construct FrameItem::Link for each nonempty fragment. Do not confuse membership with geometric intervals or collect old buffers by length. Descend through groups/wrappers; bounds use the helper owned by `00_nucleo/prompts/core/compiler/layout/link.md` in each vector's coordinate system. Equal inherited membership inside/outside a group needs no redundant nested same-destination links; preserve distinct destinations. Metadata-only segments create no clickable area. Do not remove positions, text identity or semantic wrappers.

Continue capture across line/page/column changes, preserve membership in deferred frames, exclude unrelated marginals/note/float and respect rollback. The public facade and phase remain layout. Do not hide capture loss by discarding text or fabricating a rectangle on the last page. This is internal ADR-0127 work without a new API/default/phase.

## Acceptance and limits

Locally test preservation of unlinked neighbors, fragment bounds, no area for an empty scope and preservation of a visible artifact whose alt resembles the private marker. These controls do not replace full-flow pagination, subframe and rollback tests.
