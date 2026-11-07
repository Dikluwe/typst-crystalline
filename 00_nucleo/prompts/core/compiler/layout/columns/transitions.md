# L0 — compiler/layout/columns/transitions
Hash do Código: dbb0df13

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/column-note-regions.toml sha256:ce40781bf12cee5ae49390769448b1ca10968af4eaa5a109399374beab99e619
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57

## Evidence before specification

At HEAD `f7195cdf6` (clean tree, 2026-09-30), the block is at `01_core/src/compiler/layout/columns.rs:705-832`. This observation establishes extraction boundaries, not performance or new bilateral parity.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. The pre-repair snapshot is `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

## Ownership and contract

Layer L1. Sole productive consumer: `01_core/src/compiler/layout/columns/transitions.rs`. Own physical operations for closing, merging, finishing and starting columns, preserving translation and fixups. Close committed segments before taking each column; open continuations in subsequent column/page buffers. Preserve line boundaries when a line continues, or reconstruct them when the buffer is reset. Do not change translation, physical order or fixups.

Preserve public contract, default behavior, pipeline phase, compatibility, purity and static dispatch. Link capture is an internal correction under ADR-0127 with no new API, default or phase.

## Acceptance and limits

The L1 suite, build and crystalline-lint must pass. Separation does not prove a performance gain.

Require RED→GREEN for breaks before/within the body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.


## Footnote column and lexical-numbering correction — 2026-11-01

### Measurement before decision

`lab/results/construction/diagnostics/footnote-columns-numbering-2026-11-01/{before.json.gz,baseline-build.json.gz,baseline-tekt.json.gz,baseline-vanilla.json.gz,general-before.json.gz}` binds HEAD3c6b0ce7, uncommitted tree/exact diffstat and source/L0 bytes, UTC, argv and executable SHA. Oracle commit64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17, binary22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34. Global/alias/local set is ignored at the call (H1/ref1 instead of Ha/refa); query numbering field is missing. Short standalone columns already match; long explicit columns use three product pages versus two oracle pages, and Box notes occupy the wrong column while main rows ignore note area. Existing pending queues and page_column_notes distinguish ownership, but refresh refuses columns, flow selects fixed segments on any colbreak and final flow guesses page bottom. Native physical text/coordinates and lexical values govern acceptance; buffer/match shape is mechanics. No intent is inferred from source inspection alone.

### Exclusive obligation

In corrected note flow, close per-column notes before translation but retain page-owned notes until merge/page close. Track the actual committed main-flow end before note placement; merge restores the deepest main-flow baseline/cursor and resets tracking for the next physical page. Preserve legacy non-note transitions and original link/line-number/source geometry.

This is internal convergence/restoration of existing language behavior under ADR-0127 continuous flow: no public entity/trait/signature, user mode/default, compatibility break or eval/layout phase change. Preserve the constructor's raw morphology and all #8935 identity, measurement and header replay corrections. Nucleus obligations are shared only through these full pins; each source retains its canonical sole owner. Require minimal real RED before production code, GREEN, prior14 #8935, footnote/subframe/column suites, bilingual source controls, full PDF extraction and SVG paint checks. Use only the task's exclusive /dev/shm target, one compilation job, 12GiB process limit, preserve concurrent work and do not commit/push. Global combined qualification remains separate. Any new public carrier or phase need stops at a concrete proposal for human approval.
