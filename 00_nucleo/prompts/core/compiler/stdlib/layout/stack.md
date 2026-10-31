# L0 — stdlib/layout/stack — stack
Hash do Código: 1dbbc16e

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/stack.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contains this family's logic in `01_core/src/compiler/stdlib/layout.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not a benchmark or new bilateral vanilla parity evidence. Historical decisions, measurements and pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/layout.md`.

## Language contract

stack preserves direction, spacing and child order, including validation and content-element construction.

Extraction preserves signatures, function identity, syntax, morphology, errors and validation precedence. The hub and registry continue to forward calls statically without duplicating algorithms. Preserve public contracts, default behavior and pipeline phase.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Parity is at the language level under ADR-0107/0108; mechanics may diverge. Preserve exclusive ownership under ADR-0129/0130.

## Stack image frame correction — 2026-10-27

### Measurement before decision

`lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/before.json` records HEAD212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diff/stat/status and every received productive/L0 input, UTC and executable identities. before/{receipt.json,geometry.json}, observe-before.log and effects-before/{receipt.json,query-absolute.json} are fresh bilateral measurements against ratified64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1/SHA22897f93; received product17f39776 includes the completed image/h and JPEG work. Stack3 image tops14.834/22.072/29.310 versus vanilla5/25/45; Stack16 drift181.596pt; Stack20 is one page versus19+1 images on two pages. RTL/BTT source-order placement is wrong; mixed text/image, hidden and clipped children also fail. Pure horizontal text remains qualified. Negative spacing is rejected only by Tekt's P156I gate. Context metadata puts child19 on page1 versus2. Source stack.rs:115–119 returns zero descent for every child without text, discarding a computed bottom; :144 adds an outer text baseline, :181 uses the legacy-origin route, :219 advances by that zero descent plus text top. sub_frame.rs:315–344 distinguishes geometric from baseline origins, and :497–521 returns a cursor-based fallback except for previously owned cases. Pinned upstream stack.rs:274–300 accumulates completed frame sizes and :303–366 composes their geometry. These are observed language geometry/pages/accepted arguments; no upstream intent is inferred and matching Rust/PDF bytes is not acceptance.

### Classification and scope

Private parity correction, continuous ADR-0127: no public API/entity field, trait method, dependency, new product mode, phase or compatibility break. Existing signatures and source identities remain. The old ascent/descent fallback and reverse-direction limitations are superseded only for this measured frame-composition correction. Existing Ordinary/Footnote/Repeat subframe routes and all received JPEG/image/h/notes/line work remain preservation controls. No new page dispatcher, source replay, callback cache or semantic reconstruction from FrameItem is authorized. Use the existing region transition and static source-child protocol. Individual child-internal multipage text, fractional stack spacings and general alignment are not certified by these finite image controls.

### Exclusive constructor responsibility

Remove only the obsolete negative-Length rejection after successful typed spacing extraction. Retain the exact signed Length and source child order/direction. Defaults, type/name/direction/child errors and their precedence remain unchanged. This adds parity for an existing typed parameter, not a new flag, signature or eval/layout phase move.

### Acceptance

Require functional RED→GREEN before implementation, source-current bilateral PDF/SVG<=0.001pt, all four directions, spacing/default/negative, target clipping, mixed/no-paint children, nesting and exact-fit/19+1/14+6 pagination, text adjacency and h boundaries. Preserve real Context/page/label/counter source semantics and all current positive image/h, A–H/R1–R13/notes/relative-line controls. Repeat all12 established technical gates, unfiltered lint with zero warning/error, frozen provenance and restricted lineage reseal. Recheck JPEG acceptance/complete decode/dedup/time/RSS and R6 size/memory gates with no competing builds. Bound the known invalid oversized oracle case explicitly; never hide a timeout/signal as a qualified rejection. Historical self-sealed stdout limitations remain explicit; do not rewrite old manifests. No commit/push.
