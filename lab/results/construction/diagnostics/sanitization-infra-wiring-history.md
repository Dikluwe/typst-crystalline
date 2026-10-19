# Infrastructure and wiring documentary sanitation: history and unresolved evidence

## Scope and provenance

This diagnostic belongs only to the documentary batch covering every prompt in
`00_nucleo/prompts/infra/` except the entire `infra/export/` subtree, and every
prompt in `00_nucleo/prompts/wiring/`. The root prompt
`00_nucleo/prompts/wiring.md` is outside that scope. `infra/export-fixtures.md`
is inside scope because it is a sibling file, not part of the excluded directory.
This report does not claim global sanitation or current upstream parity.

The repository instructions and applicable ADRs were read before editing. The
current consumers were inspected before consolidating their contracts. Evidence
anchors in the prompts identify local source, not a newly run language comparison.
The existing working-tree edits, including prior English child specifications,
benchmark work, code and existing history archives, were preserved. Concurrent
out-of-scope prompt edits were detected and left untouched by this task; the
verification receipt below records them without attributing them to this batch. No productive source, test, ADR, nucleus, fixture or existing diagnostic
was edited. No formatting or hash-reseal command was run.

- Repository HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`.
- Baseline: uncommitted working tree, status captured at `2026-09-27T00:40:09.030758+00:00`.
- The complete initial changed-file inventory is retained below, including
  untracked files; it must not be confused with changes made by this batch.
- Each edited prompt's initial SHA-256 and exclusive consumer are recorded in
  the inventory below. The original `Hash do Código` values, existing consumer
  headers and nucleus pins remain unchanged; missing metadata was not invented.

The current upstream reference remains the repository's pinned
`9f2b6e8715237cb086899a42873660fe744622e8`. Older citations to `e0e8ca4d`,
`586e1bd43` or `a51e02804`, and version-only assertions, are historical evidence
for those snapshots only. No installed binary's version string was used to
certify current identity, and no new upstream measurement was made.

## Preserved contracts and ownership

| Family | Preserved contract and normative owner |
| --- | --- |
| World | SystemWorld constructor/state/input/plugin composition remains in the hub. Source/byte caching and dependencies, font builders/coverage, rooted paths/bibliography, package roots and World trait delegation each remain in their own child. Source errors are memoized; unsuccessful byte reads remain retryable. |
| Fonts | Slots and metadata/collection extraction belong to fonts; system discovery and eager system coverage belong to fontdb; bundled-byte classification belongs to embedded_fonts; insertion order and missing-coverage cache belong to world/fonts. Catalog ordering belongs to fallback_fonts. |
| Metrics | The facade owns state/lifetimes/constructors/Clone/reexports. basic, caches, fallback, math and resolution own the algorithms. Preserve signed glyph ink, variation keys, MATH percentages/design units, present zero, assemblies, kern and the canonical ssty predicate. |
| Shaping | The facade declares/reexports children. traversal owns item replacement/passthrough; bidi owns input run coordinates; selection owns candidate/run choice; shaping owns sub-run text and local UTF-8 clusters, variations/tracking/ssty; reconciliation owns line shifts. The paged coordinator owns phase placement. |
| Images | image-sizer owns inspection of supplied bytes. World paths own byte acquisition. L1 image layout owns sizing/orientation interpretation. The paged coordinator invokes validation. The excluded export/images facade and ICC/JPEG/PNG/scan/XObject children remain normative owners of PDF payload processing; they were read only. |
| Pipeline | The hub owns eval adapters, context walkers, public facades and timings. Children own paged, semantic HTML, bundle, format-specific orchestration, font resources and selective stabilization. Preserve callback/numbering budgets, warning/error flow, features/target separation and typed format-option precedence. |
| I/O and wiring | L3 owns font/filesystem/network/plugin operations and returns neutral results/raw diagnostics. L4 composes those operations and existing publication/status/stdout/stderr behavior; L2 owns CLI parsing, formatting and serialization. No new L4 compiler, resolver, formatter or watch algorithm is introduced. |
| Tests | Each suite owns its observations/harness only. Preserve temporary isolation, bounded process behavior, unknown/fail-closed results and language/format observables. Test prompts do not acquire ownership of productive code. |

Hub duplication was removed or replaced by canonical routing: shaper no longer
restates Text replacement/fallback; embedded_fonts and fontdb no longer own
SystemWorld builders/order or CLI defaults; fonts no longer owns the world's
coverage/preload algorithm; font_variant no longer owns pipeline/font-key/export
signatures; layout_bidi no longer owns L1 paragraph alignment. Font resource
collection and image processing are not conflated. The existing fine-grained
pipeline, metric, World and plugin owners were kept rather than recombined.

The p1427 test contract now owns only its retained controls; decoded repr/style
and causal parenthesis observation are owned by p1428. The watch capability test
has one current method allowlist instead of contradictory initial/amended lists.

## References corrected with a proved destination

| Removed or misleading reference | Current verified destination |
| --- | --- |
| `03_infra/src/font_variant_instancer.py`, described as include_str input | `03_infra/src/font_variant.rs:130`, inline `INSTANCER_SCRIPT`; the separate file does not exist. |
| `03_infra/src/integration_tests.rs::p307b_snapshot`, future harness/include_bytes template | `03_infra/src/p307b_snapshot_tests.rs`, whose header declares `infra/export-fixtures.md`; it reads references at runtime. |
| `03_infra/src/export.rs` as the active monolith | `03_infra/src/export/mod.rs` and existing excluded owners; old monolith coordinates remain archival, not rewritten as equivalent current coordinates. |
| `build_font_book`, unconditional slots plus conditional metadata | `03_infra/src/fonts.rs:892`, `pair_slots_with_book`, with paired insertion. |
| Planned with_system_fonts constructor signature and world.rs builder bodies | `03_infra/src/world/fonts.rs`, current builders on an existing World; no API addition. |
| world.rs as the bibliography/path implementation | `03_infra/src/world/paths.rs`, with the unchanged pure BibTeX parser owner in L1. |
| Historical latest-version entry in SystemWorld | `03_infra/src/project_init.rs:96,133` for omitted template versions, and `package_downloader.rs:132` for preview index fetching. Their declared owners remain shell/init and infra/package_downloader. |
| Raw `NewComputerModern10` / `NewComputerModernMath` as current catalog families | The fonts metadata exception table supplies `New Computer Modern` / `New Computer Modern Math`; embedded grouping and fallback use those names. |
| `typst-adr-0098-...md` placeholder | `00_nucleo/adr/typst-adr-0098-single-source-of-truth-invariante-anti-bug.md`. Historical rationale is retained here, not as a duplicate exporter contract. |
| Developer checkout `/home/dikluwe/Documentos/Antigravity/typst-crystalline` in snapshot regeneration | Harness resolution through CARGO_MANIFEST_DIR and the existing repository fixtures. |
| Bare `shell/watch.md`, `wiring.md`, infra/export and math-owner prompt paths | Canonical `00_nucleo/prompts/...` paths. The watch cycle now routes specifically to wiring/watch; root wiring remains outside scope. |
| Unqualified `sec_22.typ` witness | `.typ/sec_22.typ`, verified by the consumer's include_str path. |
| Facade-only `font_metrics::tests` verification filter for child algorithms | `font_metrics::` filter, covering the existing family modules without changing tests. |

The following persistent references were checked and kept: p307b sources and
reference PDFs, tiny.jpg, NotoSans-Regular.ttf and its OFL license, the p839/p840
metadata fixtures, and NewCMMath-Book.otf. NotoSans-Regular.ttf retains SHA-256
`2ec33f84606cbaa0a1a944488e14f97faf2f6a25ecdd8354f5358f06da13c7d9`.
`Cargo.lock` still pins typst-assets to
`c0ae970f64413cf8e39cecd56d15ee50de42eaf4`.

Existing test-controlled `/dev/shm`/TMPDIR and isolated HOME/XDG requirements
were retained. Their environmental capability requirements are not a request
to recreate historical temporary files or silently select a different fallback.

## History moved out of the specifications

### Font loading, metadata and variation work

The old fontdb narrative deferred discovery under ADR-0020, then activated it in
P515. P674 reused fontdb bytes; P839 required paired slot/book insertion;
P937/P938 moved from block coverage to exact coverage and alternated eager/lazy
policies. The current split is source-specific: fontdb extracts system coverage
eagerly, while metadata pairing can leave coverage empty for the World cache.
Old startup performance estimates and execution totals are not current evidence.

P838 corrected collection-absolute table offsets and OS/2 serif metadata. P839
changed name/style extraction and replaced build_font_book; its reported wrong
font/size symptoms explain why paired insertion matters, without establishing
current output measurements. The old source coordinates `info.rs:131-138`,
`:168-203`, `:206-267`, `typst-kit/src/fonts.rs:172-189` and
`exceptions.rs:46-342` are historical upstream coordinates, not newly verified
against the current pin. The metadata table and local fixtures remain the
inspectable current evidence.

P753/P754 introduced bundled fonts and separated text from math/code. P784
measured raw nameID 1 values in the pinned assets: Libertinus Serif has spaces,
NewComputerModern10/Math do not. P840's exception table later supplied documented
names with spaces. The raw-name comparison became stale. The old P1031 citations
from `e0e8ca4d` (`typst-kit/src/fonts.rs:129-142`, text/mod.rs:180-182) support the
historical bundled-font/default-family decision. The separate quoted upstream
discovery-priority comment does not prove the crystalline insertion order or an
unconditional project override; current composition belongs to world/fonts.

P558 kept FreeSerif last after a CFF accent/subsetting limitation. P783/P784/P840
corrected math-family spelling. These explain catalog choices; the actual order
is owned by fallback_fonts, and neither this history nor a family name decides
math semantics.

P525 applied variation axes in shaping while PDF embedding could retain the
default instance. P527/P528 recorded visible weight mismatch; P529/P530 motivated
static instancing after subsetting. The old estimated Python runtime, broad
cross-file patch plan and always-fallback-with-warning rule were removed. The
current inline subprocess and each caller's actual error/fallback policy govern.
P836 established explicit-axis override by tag. Its upstream
`text/font/mod.rs:113-120` citation is preserved as historical evidence, not a
fresh measurement.

### Bidirectional layout and shaping

P562's simple reversal, P565/P567 paragraph fusion and P569 extraction cleanup
were superseded in their cross-line behavior. The old Arabic witnesses were
`الكتاب 42 على الطاولة` and `معلومات قيمة.`. The latter was reported with joined
words in pdftotext but spaced words in mutool; the extractor observations are
historical, not reproduced here. Whitespace/suffix cleanup and real metrics
remain in the current consumer.

P1140.12 used explicit linebreak markers as a reflow barrier. P1140.13 recorded
working tree `ca28f4ab74ae66985cdc66805c16c2ddc8f08366` at
`2026-08-24T12:15:59-03:00` and rejected byte-width/fixed-slack/vertical-distance
heuristics. P1140.14, at `45b547073d7686cdd5d3e3030c82de3e22ec395f`, concluded that
emitted frames cannot reconstruct separators, break opportunities or full region
limits. L1 wrapping therefore remains sovereign and L3 never fuses baselines.

The surviving investigation documents are
`00_nucleo/diagnosticos/typst-p1140.12-rtl-shaped-gaps.md`,
`typst-p1140.13-reflow-rtl-sem-calibracao.md` and
`typst-p1140.14-limites-regiao-reflow-rtl.md` in that same diagnostics directory.
The old hub coordinates shaper.rs:508 and pipeline.rs:346-374 do not identify the
current child implementations. No vertical-writing roadmap or L1 paragraph
alignment plan remains normative in layout_bidi.

Current shaping source inspection corrected documentation only: emitted clusters
are local to TextShaped.text, while BidiRun.byte_start refers to original input;
TextItem carries no explicit ssty request (`None`), not a fabricated Some(0).
Single-face glyph ink can retain a negative bottom extent, unlike the clamped
text union. Metric math fallback insertion checks readable faces and slot
identity; it does not add a MATH-table filter at that insertion point.

### Images and controlled snapshots

The old image-sizer P1031 citation from `e0e8ca4d`,
`visualize/image/raster.rs:363-375`, described determine_dpi and a positive-density
filter. The prompt explicitly left positivity as a point to confirm. Current
PNG/EXIF parsers have no matching uniform filter; this was not silently fixed by
documentary editing. Orientation metadata remains distinct from byte transforms.

P307a/P307b replaced a monolithic textual export proxy with a controlled binary
snapshot corpus. Initial V7 orphan/future-harness status, coverage/LOC tables,
old md5 totals, execution chronology and obsolete command templates were removed
from L0. P601 fixed epoch; the harness now explicitly fixes Compact/Disabled tags.
Those controls are test-specific, not production defaults.

P833 replaced a malformed JPEG fixture lacking a chroma quantization table.
P1483 intentionally regenerated only gradient references after enabling real
Pattern/Shading fills; the other references' manifest entries were corrected
without regenerating their bytes. The operational record remains in
`03_infra/fixtures/p307b/MANIFEST.md`; its historical qpdf/determinism claims were
not rerun in this sanitation. The original rationale remains in
`00_nucleo/diagnosticos/diagnostico-export-passo-307a.md`.

### Queries, measurements, bundle and public observers

Query helpers began as structural-summary support in P206C; the old future-only
CLI prohibition became stale after P1137-I-001. P1285 widened located retrieval
beyond headings. Its source inspection on
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88` found the then heading-only limitation
in L2 serialization, not L3 retrieval. This history does not restore a heading
filter or a Vec<Content> API.

P1307-R5's old proposal text recorded baseline
`b303f1f15b610e09872b567027e0d806387fde8c`, independent evidence SHA-256
`82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8` and upstream
`a51e02804`, with old query_helpers.rs:459,511-523 and cli.rs:758 coordinates.
The current consumer returns Vec<Value> and preserves located snapshots. The
proposal's old pending-approval label is archived; source presence does not by
itself prove that historical approval or identify the missing independent
artifact. This batch neither reapproves nor rolls back productive APIs.

P1140.19 made PageRun transparent to approximate walkers; P1292 and P1402 added
neutral math/flow handling and OutlineEntry descent. The P1436-P1451 amendments
mixed approved/proposed/materialized labels; their terminal projections are now
consolidated from the current exhaustive matches without claiming approval of
any archived proposal. P1494 recorded SpaceFromNewline neutrality on
`6da06e849d75972651a342ad6aad337313451c79`; language normalization remains L1.

Measurements started with P204G's eviction/call counters rather than nonexistent
cumulative comemo hooks. Historical method totals and workspace test counts were
removed. P844 added counter_final_values; P1140.4 delegated equation content;
P1158 retained typed page_numbering; P1493 delegated local HTML position without
adding a counter. Current fixed-order counter observations are not cache hit/miss
statistics. Eviction is already called by watch; the future-watch narrative is
not current authority.

P1484 at `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08` replaced the unfeatured asset
placeholder test with its feature-gate diagnostic. P1485 added active bundle
corpus and directory watch publication. The old proposed/open status was removed
from the current test contract. Historical probe provenance remains in
`00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md` and
`00_nucleo/diagnosticos/typst-p1485-integracao-gate2.md`.

The p1297 rustdoc failure compared the old file-only allowlist with the current
`abandon, abandon_directory, publish, publish_directory` surface
(watch.rs:81-115 at that old working tree). The current exact allowlist preserves
private capability state and a single public arm function.

P1454 corrected the frame-rect public red expectation to `#ff4136`; `#ff0000`
was a CSS assumption that would have justified an incorrect SVG-only rewrite.
P1466 changed fixture strings to public color values and transferred p1427 repr
and causal-parenthesis cases to p1428. The removed names were
`p1427_reprs_preservam_estilo_explicito_e_omissao` and
`p1427_parenteses_causais_so_aparecem_no_horizontal`. The retained controls and
p1428 observations remain separate, necessary suites. Bilateral observations in
these historical notes are not fresh measurements against the current pin.

### Pre-existing archives and already-English families

Earlier working-tree edits already supplied English children and the archival
family `00_nucleo/diagnosticos/l0-infra-wiring-history/`. Its README/manifest and
`00_nucleo/diagnosticos/l0-infra-wiring-sanitization-2026-09-26.md` were preserved
byte-for-byte. The present batch moved repetitive historical pointers and cleanup
execution boilerplate out of scoped L0s into this single new diagnostic; it did
not create, edit or reseal those earlier archives.

Those archives retain the prior pipeline stabilization/gate narratives, callback
and numbering evolution, plugin protocol investigation, command/format observer
chronology and superseded default descriptions. Their historical source seals,
old pins and claimed validation receipts remain attributable to their own captured
working tree. They do not certify the current tree, approve pending proposals or
own productive modules. Current English contracts, their child routing and typed
format/feature/tagging boundaries were preserved after consumer inspection.

## Unresolved references and implementation/lineage gaps

| Item | Preserved information and next investigation boundary |
| --- | --- |
| Versionless resolution owner | `infra/package_version_resolution.md` has legacy Hash do Código `caeadbbd`, but no Rust @prompt consumer. project_init declares shell/init; index fetching declares package_downloader. The reference is explicit about this gap. Resolving ADR-0129 ownership requires an authorized cross-owner change outside this batch, not a fictitious owner or deletion of the evidence. |
| Proposed remote index cache | Old package_version_resolution specified at most one index fetch per SystemWorld. Current latest_version performs a request per call, and initialization owns versionless resolution. No proved cache replacement exists; decide whether the old proposal is retired or requires separate work. No cache guarantee was added. |
| p1426 seal | The initial p1426 prompt lacks Hash do Código. The test and owner relation remain; no seal/header was fabricated. Investigate the accepted lineage before any scoped reseal. |
| BibTeX escaped quote | Old L0 promised the next unescaped quote; parse_quoted currently terminates at any quote (`bibtex.rs:231`). Preserve the discrepancy for a dedicated parser-contract review, not a silent code fix or localization change. |
| BibTeX signed number | Old grammar allowed leading minus; parse_value dispatches minus but parse_number only advances digits (`bibtex.rs:246`), and year parses u32. No negative-number support is certified. |
| Approximate list/enum singleton | Old summary text promised a root singleton item count of one. Current count_list_groups/count_enum_groups recurse into its body (`query_helpers.rs:246,260`); the promise is not generally satisfied. Preserve current projection and investigate separately. |
| Zero image density | PNG pHYs/EXIF can return zero; JFIF requires positive density (`image_sizer.rs:32,50,96,365`). L1 calculate_dimensions consumes supplied DPI without a uniform positivity filter (`layout/image.rs:61`). The old upstream positive-filter citation is not an implemented guarantee. No geometry change was made. |
| Optional DPI files | `/tmp/p773-dpi300.png` and `/tmp/p773-dpi300.jpg` were absent during inspection. The existing optional test skips them. No persistent replacement is proved; inspect the original P773 generator if an external real-file probe is needed. Inline metadata fixtures remain available. |
| Concurrent-download temp cleanup | DirectoryNotEmpty accepts the winner, then the guard is forgotten (`package_downloader.rs:253-265`); the comment about cleaning the losing temp is not an implemented action. Preserve the race policy but do not claim complete cleanup. A dedicated L3 review is needed. |
| Historical P763/P764 probes | The old prompts expressly lacked the original command/output artifact. P1031 supplied source citations from e0e8ca4d. The separate import probe recorded `/usr/local/bin/typst` version string and crystalline HEAD `4f64e4e69` on 2026-08-13, but lacks complete binary/dirty-tree provenance for current proof. No missing artifact is replaced speculatively. |
| Historical CFF corpus | font_variant named P523/P529 corpus/instancing evidence without a proved current corpus path or complete measurement receipt. Keep the identifiers for investigation; no invented fixture path or runtime claim is normative. |
| R5 independent artifact | The SHA-256 and baseline above identify the claimed independent record, but the original L0 did not provide a resolvable artifact path. Recover it before treating the claim as reproducible approval/parity evidence. |
| Excluded/out-of-scope owners | shell/init and root wiring retain their own historical/broad ownership text. Concurrent edits were observed in the infra/export subtree. This task never wrote those files or assessed the other writer's final state. This batch does not sanitize those documents or claim their global consistency. |
| Prompt/source hash drift | Documentary edits change prompt bytes while source headers and Hash do Código remain untouched by instruction. Read-only V5 diagnostics therefore remain. Resolving them requires separately authorized scoped lineage work; no global repair was run. |

## Documentary verification

- Canonical current prompt/file/fixture paths resolve; unresolved historical
  paths are explicitly identified above rather than silently discarded or renamed.
- The scoped specifications contain English normative prose. Literal Portuguese
  diagnostic messages, identifiers/filenames, `Hash do Código` and `Núcleos Tekt`
  remain where preserving the contract/parser requires them.
- Original code-seal metadata and shared-nucleus path/hash pairs are unchanged.
- Byte-digest comparison against the captured baseline verifies protected code,
  tests, fixtures, benchmarks, ADRs, nuclei and the previously existing diagnostics.
  Other prompt changes appeared concurrently, including the excluded export
  subtree and compiler/introspection/columns. They were left untouched; the
  exceptions and digest receipt below prevent a false byte-identity claim.
- Read-only `crystalline-lint --checks v5,v15,v26 --format sarif --fail-on warning .`
  reports V5 drift and no V15/V26 findings. Its failure is not waived or represented
  as zero global violations. The independently discovered orphan/missing-seal
  issues above are retained even when those selected checks do not report them.
- No behavior tests, cargo build, snapshot update or upstream probe was run for
  these documentary edits. Verification commands in L0 are instructions, not
  execution claims. No line-count metric was produced.

## Edited-file inventory

The table lists changes relative to this task's captured working tree, not HEAD.
Initial SHA-256 values identify the exact input prompts, including existing edits.
The new diagnostic is `00_nucleo/diagnosticos/sanitization-infra-wiring-history.md`.

| Edited prompt | Exclusive current consumer / exception | Initial prompt SHA-256 |
| --- | --- | --- |
| `00_nucleo/prompts/infra/bibtex.md` | `01_core/src/compiler/eval/bibtex.rs` | `ea159a6afa18677b179aed98cb208a88fa317d92514b5bfbd67d114d376d0caf` |
| `00_nucleo/prompts/infra/embedded_fonts.md` | `03_infra/src/embedded_fonts.rs` | `ad2392ff028cb6c4e0ec34d69738eac6ec3057d77dc4837f02cfa3f8273c6a3d` |
| `00_nucleo/prompts/infra/export-fixtures.md` | `03_infra/src/p307b_snapshot_tests.rs` | `4777f13afc209cde393b6bba71ed65af58de6d7a4cef5d60e6606658f5928f23` |
| `00_nucleo/prompts/infra/fallback_fonts.md` | `03_infra/src/fallback_fonts.rs` | `f905f7a29b4fa514d43ad539074d52b052c80ca701342a257af3a480faf18497` |
| `00_nucleo/prompts/infra/font_metrics/basic.md` | `03_infra/src/font_metrics/basic.rs` | `88f3c1e0d410f88ba80c1016746e4854e514b1458132996584907192cd5fde7b` |
| `00_nucleo/prompts/infra/font_metrics/caches.md` | `03_infra/src/font_metrics/caches.rs` | `02a4d9d56d496a9fd2cbf9464c84382c58fe255db57900397dfdc47f314270cb` |
| `00_nucleo/prompts/infra/font_metrics/fallback.md` | `03_infra/src/font_metrics/fallback.rs` | `003942d27780e282d1fcfc7467372d5203d6263b6edb8d1d8165cb5c75927116` |
| `00_nucleo/prompts/infra/font_metrics/math.md` | `03_infra/src/font_metrics/math.rs` | `4cd987819b9ea3d08085629c2f48d2842ac46fffd2f9644c0b63d88cb585e731` |
| `00_nucleo/prompts/infra/font_metrics/resolution.md` | `03_infra/src/font_metrics/resolution.rs` | `cbc9de7d3aaf649cc7582ca92c9844dc71b570482269f595c4b5de9d581cfc03` |
| `00_nucleo/prompts/infra/font_metrics.md` | `03_infra/src/font_metrics.rs` | `7196090e5be164f326c4099b5f3bcb245ffa6dbe8790d2b21885648841c04932` |
| `00_nucleo/prompts/infra/font_variant.md` | `03_infra/src/font_variant.rs` | `024489f98d0b639da44d9b761f99a5ad82f8ab2267d1741195db65f8d464cc91` |
| `00_nucleo/prompts/infra/fontdb.md` | `03_infra/src/fontdb.rs` | `c0d315145cbc26eb89393f91884764eda97aa7b4a654c12933cacee596a61076` |
| `00_nucleo/prompts/infra/fonts.md` | `03_infra/src/fonts.rs` | `af1f91f46e5517735af734dc92fa5988cae8b27a2098b03ab821e013122a13f1` |
| `00_nucleo/prompts/infra/image-sizer.md` | `03_infra/src/image_sizer.rs` | `270f13728612a835f82b47c928d1fc60d0bbfdff4d29d88fc4b9a80f92137396` |
| `00_nucleo/prompts/infra/integration_tests.md` | `03_infra/src/integration_tests.rs` | `029088dfcd13468a5fbef5f541094ce2be86adf1e10784d9960e9a8e5796320b` |
| `00_nucleo/prompts/infra/layout.md` | `03_infra/src/layout.rs` | `57ba2b693e5dd1d0a7cdce3f0c79d1f931540291285d69820966d4825a5a6c5c` |
| `00_nucleo/prompts/infra/layout_bidi.md` | `03_infra/src/layout_bidi.rs` | `d9afd26ce5a7c3846852aac2c10c11bf701a2f369fc6eba6d4eb3b372c75a1ad` |
| `00_nucleo/prompts/infra/measurements.md` | `03_infra/src/measurements.rs` | `cbba2ecad709cc7631965d529e220fbe48e688afcc1148b851a85055ed92712c` |
| `00_nucleo/prompts/infra/package_downloader.md` | `03_infra/src/package_downloader.rs` | `d87873d85bbc2ee1b6c809e3d328e632d97c70f68496ad90fbe87ff12977615f` |
| `00_nucleo/prompts/infra/package_version_resolution.md` | No declared exclusive consumer; pending | `abbaaa0b5601850c40245b403402c41150a37bb4f8cec573102b52dbd71a195e` |
| `00_nucleo/prompts/infra/pipeline/bundle.md` | `03_infra/src/pipeline/bundle.rs` | `e4581b4dd23c77275715a333990b30986ffb886f38f682b3031a93a0809603f5` |
| `00_nucleo/prompts/infra/pipeline/context_stabilization.md` | `03_infra/src/pipeline/context_stabilization.rs` | `7f7e9161b5ac1c4be1773a2b3d5c666445dc4d2f0ccd05c6778f1f48e659e895` |
| `00_nucleo/prompts/infra/pipeline/html.md` | `03_infra/src/pipeline/html.rs` | `12d0bedafeb9ac40b416577cf15f1f0d247a52ef7d2f122c7c55e930014cef50` |
| `00_nucleo/prompts/infra/pipeline/paged.md` | `03_infra/src/pipeline/paged.rs` | `ab3199eecd695a15a14c3fb269e34fbc6b90ec5a7bf7881656e441edcc9c6815` |
| `00_nucleo/prompts/infra/pipeline/pdf.md` | `03_infra/src/pipeline/pdf.rs` | `4da2333b2a1281ba04ab37151fa39883c4187f8b80cf9267ce71447740d47a38` |
| `00_nucleo/prompts/infra/pipeline/raster.md` | `03_infra/src/pipeline/raster.rs` | `4bf33b8ff1aa070b8d10f6c6f22614bc4a152bcb426c428ef9c31e68a2fb86bc` |
| `00_nucleo/prompts/infra/pipeline/resources.md` | `03_infra/src/pipeline/resources.rs` | `223ed00bdb559fb4e427b2dbd81a056d4cdb1c22fb6a73740673f7f2b7cb27c7` |
| `00_nucleo/prompts/infra/pipeline/svg.md` | `03_infra/src/pipeline/svg.rs` | `d8a6da72a699461357d5171a1f4a3daaf1db2ef19f08bbdec7fcd8ad7ed992f8` |
| `00_nucleo/prompts/infra/pipeline/tests.md` | `03_infra/src/pipeline/tests.rs` | `f38553bb5010509b9d1f391dbeedec774f7bdd7539e91c7ba4a3a46c80ddb5a0` |
| `00_nucleo/prompts/infra/pipeline.md` | `03_infra/src/pipeline.rs` | `4e58165188e8cc559ca3721f0940fc1ae2da768393d80941fdb57627bd8c5aed` |
| `00_nucleo/prompts/infra/plugin_host/execution.md` | `03_infra/src/plugin_host/execution.rs` | `5152136389cc3e53ecdc76f44fc68791672c96a521ed8fc5244e3cc34352b151` |
| `00_nucleo/prompts/infra/plugin_host/loading.md` | `03_infra/src/plugin_host/loading.rs` | `bd54aad2ace4d4f87ec26994fa978f2083590b05bda7a9c7b39934cb71a3166a` |
| `00_nucleo/prompts/infra/plugin_host/snapshot.md` | `03_infra/src/plugin_host/snapshot.rs` | `8013a3a14110921b740eae6b8c7c00638ec66bc0bb03073bf936252b1bfad11e` |
| `00_nucleo/prompts/infra/plugin_host/tests.md` | `03_infra/src/plugin_host/tests.rs` | `06cb132264f738f3b3097e967ba5c767dd783111518158d17efdb7267ea0b29e` |
| `00_nucleo/prompts/infra/plugin_host/typst_env.md` | `03_infra/src/plugin_host/typst_env.rs` | `14e8722d058797344d557c30e0d4d5d3231add0c3572432aee4939b91ea5aa08` |
| `00_nucleo/prompts/infra/plugin_host.md` | `03_infra/src/plugin_host.rs` | `ab3fb997b628cbb2289ea76a994a6e95b3473e270bff90df8b328b83451626fe` |
| `00_nucleo/prompts/infra/query-helpers.md` | `03_infra/src/query_helpers.rs` | `60122f5f792515171d39aa1c2553d1bfbaf6498e81aba121748eb4c391092533` |
| `00_nucleo/prompts/infra/runtime_info.md` | `03_infra/src/runtime_info.rs` | `6350b184157098a17f535f6733f5fa8822f1f72e0196a9626107b02f5be5a939` |
| `00_nucleo/prompts/infra/shaper/bidi.md` | `03_infra/src/shaper/bidi.rs` | `d542c9b93f9aad1c2c26b50ee2efbb466812705c6338f82935f68d0fe8916eea` |
| `00_nucleo/prompts/infra/shaper/cache.md` | `03_infra/src/shaper/cache.rs` | `07f387d0b390a0f8f965be75a9c35975fd29e5c233760ccaf1e48dfded0af892` |
| `00_nucleo/prompts/infra/shaper/font_selection.md` | `03_infra/src/shaper/font_selection.rs` | `e02df8392cabd0211e9f00f66fde86fb10a0a77d87ea8c5ddd6e2d8c1dac748c` |
| `00_nucleo/prompts/infra/shaper/reconciliation.md` | `03_infra/src/shaper/reconciliation.rs` | `4f2af7a8ce0ef8aa46114d8cedae3cac4a3a4a0460c7c55c7e3b24a734ddecbf` |
| `00_nucleo/prompts/infra/shaper/shaping.md` | `03_infra/src/shaper/shaping.rs` | `87339d72ccca4bc98b94cf2bbef52b6fd723b4f5df5e3f1bfa942ce378d322e2` |
| `00_nucleo/prompts/infra/shaper/tests.md` | `03_infra/src/shaper/tests.rs` | `2e4f446f7452c1b3df5dfbc65c17f6f7fe5b7e184064b036ed67cea1501db24f` |
| `00_nucleo/prompts/infra/shaper/traversal.md` | `03_infra/src/shaper/traversal.rs` | `9fc2575538c79629ff90f0029f65ce128ae5a62c404ee8e5abf157c4b35eeb59` |
| `00_nucleo/prompts/infra/shaper.md` | `03_infra/src/shaper.rs` | `654c1b5e0ab8fb6bba557b075caf9db7946f8bab3fbd9b0f9fec80fc21744ab9` |
| `00_nucleo/prompts/infra/system-world/cache.md` | `03_infra/src/world/cache.rs` | `635e554a40330c14e49972a1f81a016d92a0e53c35999ddbbdba449864a1e22e` |
| `00_nucleo/prompts/infra/system-world/fonts.md` | `03_infra/src/world/fonts.rs` | `6650ef8fd00cb099272081ff59c8b955a46e0b01a3e3b8133b867701266cfa10` |
| `00_nucleo/prompts/infra/system-world/packages.md` | `03_infra/src/world/packages.rs` | `5076f270c5b7083cd541f7a7cafa054acc197854050fdb9adc242ccf8afabc81` |
| `00_nucleo/prompts/infra/system-world/paths.md` | `03_infra/src/world/paths.rs` | `ca2cbd0010af3208e193c1fbec9f5b7c1f0210780ac66ade68fc61965773e2a5` |
| `00_nucleo/prompts/infra/system-world/tests.md` | `03_infra/src/world/tests.rs` | `98ad93aa961a5fb7fd6ce81c19132b1101ca67680dc9bbb792003e44da844254` |
| `00_nucleo/prompts/infra/system-world/world_impl.md` | `03_infra/src/world/world_impl.rs` | `07e435ac6e2ecfb67551ab570f467f4302b4ce62891464cd5544b7ad9f5c6c13` |
| `00_nucleo/prompts/infra/system-world.md` | `03_infra/src/world.rs` | `ecc69137ff7de2b16d4603e24e797a48008201db642d9df933fbf89ab3f05706` |
| `00_nucleo/prompts/infra/tests/p1295_watch_contract.md` | `03_infra/tests/p1295_watch_contract.rs` | `a996f8a00644bf239a008e11de1e9628f906c7079f7ff63c9c8bffa562fabfab` |
| `00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md` | `03_infra/tests/p1297_watch_capability_contract.rs` | `2afa13c8a5c46e4fea1dc1cb274b464029d17dd3e495e2bd70abce1cdeb8516b` |
| `00_nucleo/prompts/infra/tests/p1424-tests.md` | `03_infra/tests/p1424_html_frame.rs` | `675f3bca3e3111aa80d070d5baa74e561d505cb2aa90a1ab96fdd790c1a2f25d` |
| `00_nucleo/prompts/infra/tests/p1425-tests.md` | `03_infra/tests/p1425_math_limits.rs` | `a0bccd93159682b49dc7a2ef7afb476333d12ce3dc2a6237b1859e63c9633347` |
| `00_nucleo/prompts/infra/tests/p1426-tests.md` | `03_infra/tests/p1426_math_root.rs` | `3ca78f475e0d1f0c4015aab053d14a4d5f493f8519bced7b9ffd5263b860f6cf` |
| `00_nucleo/prompts/infra/tests/p1427-tests.md` | `03_infra/tests/p1427_math_frac.rs` | `b0bcafa0950cc7bfb8f10a1eb1d384f7acb095d08ca83157597b59e674b24934` |
| `00_nucleo/prompts/infra/tests/p1428-tests.md` | `03_infra/tests/p1428_math_frac.rs` | `af31471028c601e8e0e48cc88b70ed451eb33afc71bd0373091c0a74be56d53e` |
| `00_nucleo/prompts/wiring/compile.md` | `04_wiring/src/compile.rs` | `896cb220176f7a0597a24a01c0cb9e7e3788b94fbad3902e274909f8898ddaab` |
| `00_nucleo/prompts/wiring/completions.md` | `04_wiring/src/completions.rs` | `43cea1b3fe94a3cd2ee13f9036b031b8320b929cba55022c47fd719059ef8344` |
| `00_nucleo/prompts/wiring/diagnostics.md` | `04_wiring/src/diagnostics.rs` | `3e2684865d79c9bd6bef674574d45526b405a9acf5fb68e2feeae4fc47606e67` |
| `00_nucleo/prompts/wiring/eval.md` | `04_wiring/src/eval.rs` | `3087cb29731618ba08baeb4e2bda493df254626abc04a3fdd1dc65deaedc14aa` |
| `00_nucleo/prompts/wiring/eviction.md` | `04_wiring/src/eviction.rs` | `cf7bbd8f102faeceb58490f55bf1a5d5804f3b22b32326795244a02b27b89432` |
| `00_nucleo/prompts/wiring/fonts.md` | `04_wiring/src/fonts.rs` | `ed6e255859770a1e6b8af68b45277de5ffc96acc8a8e187b50e69b220b90885f` |
| `00_nucleo/prompts/wiring/info.md` | `04_wiring/src/info.rs` | `f12e405d00226e7325c6e45f74556ae1306120a4ae20362afed5d7bf7e8ab993` |
| `00_nucleo/prompts/wiring/init.md` | `04_wiring/src/init.rs` | `9835643633fe683aa292d9dd4ee9f79f8bb7f92b9c4697b22fc7235b2b3580e2` |
| `00_nucleo/prompts/wiring/query.md` | `04_wiring/src/query.rs` | `be95b9d4bfeb9bd25538812c77b4f494548cbf3c476c5bd5570cd62e2c9c3d3c` |
| `00_nucleo/prompts/wiring/tests/cli.md` | `04_wiring/tests/cli.rs` | `bc4c78a62f87c2c4e309c2b74681e4221b4f61e51a638af38b40cdb263e03b24` |
| `00_nucleo/prompts/wiring/tests/crystalline_lint.md` | `04_wiring/tests/crystalline_lint.rs` | `ed504ab16cd6ca580049c33e2c2a82fec64c9d89a4d6ae1586f231a33b7311fe` |
| `00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md` | `04_wiring/tests/p1289_float_is_infinite.rs` | `23c9871e0eacaa30d995c1ac06e8d4044286b7702262485b3cf8e8a80b8ad634` |
| `00_nucleo/prompts/wiring/tests/p1291_callback_runtime.md` | `04_wiring/tests/p1291_callback_runtime.rs` | `695863240e9f6fc89edaf6fff4a87a9682424a7933e4d35db08dee96d52f4aac` |
| `00_nucleo/prompts/wiring/tests/p1292_contract.md` | `04_wiring/tests/p1292_contract.rs` | `8505e3867063c5929c7f00c099cecbe449d16b44e40cdc04d9bc4514b7ca34db` |
| `00_nucleo/prompts/wiring/tests/p1293_contract.md` | `04_wiring/tests/p1293_contract.rs` | `b57276d876615be9a3ab24d99cfb522e177121d9468dc14206fb2c9ed4edb6fa` |
| `00_nucleo/prompts/wiring/tests/p1462-format-svg.md` | `04_wiring/tests/p1462_format_svg.rs` | `c0c7212f20497c96ffc56f70e16788461cfea71bcb73bf882ad5524e95005c11` |
| `00_nucleo/prompts/wiring/tests/p1463-format-png.md` | `04_wiring/tests/p1463_format_png.rs` | `58eb7a6c7556e67b75a20a1f02c69bf2e69496dec53bf354ca91467db8cb79b2` |
| `00_nucleo/prompts/wiring/tests/p1464-format-pdf.md` | `04_wiring/tests/p1464_format_pdf.rs` | `95018c99208831cef7c4f1e9482d78e6b854d4f7185011db8c20fc6b98fc5996` |
| `00_nucleo/prompts/wiring/tests/p1465-format-html.md` | `04_wiring/tests/p1465_format_html.rs` | `f62fc1dbde6916282e6cdf26aa4a584e046f74a84857cd721b426a44fc0329da` |
| `00_nucleo/prompts/wiring/tests/p1467-format-pdf-tagged-pretty.md` | `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs` | `243b4b6aec0b6094c863c2546a5acb00d9524e1a169a6ba9cb4b7df43e278612` |
| `00_nucleo/prompts/wiring/tests/p1468-format-pdf-pages.md` | `04_wiring/tests/p1468_format_pdf_pages.rs` | `74489bd63972135255ad5c597d9646b657c419b11daa383f8bb6c4bb865c49dd` |
| `00_nucleo/prompts/wiring/tests/p1469-format-pdf-standard.md` | `04_wiring/tests/p1469_format_pdf_standard.rs` | `525efa10b2551d029e06ba45283dc937118461af63a0b62173946143f50af4b7` |
| `00_nucleo/prompts/wiring/watch.md` | `04_wiring/src/watch.rs` | `ca94b02e1e519d41e6030ba8fe2dacad318d3c8f5a613041e66c83740e5f6979` |

## Concurrent out-of-scope verification receipt

At `2026-09-27T00:56:20.938057+00:00`, the following files differed from the initial protected
snapshot. Every entry is outside the task's edit scope. No mutation command in
this task targeted these paths, and no attempt was made to restore their initial
bytes. These differences show that the workspace was changing concurrently;
identifying the other writer is not necessary to preserve its work.

Protected productive source/test/fixture trees, benchmark files, ADRs, nuclei and
previously existing diagnostics matched their captured digests at this check.
A new diagnostic `00_nucleo/diagnosticos/sanitization-introspection-columns-history.md`
also appeared outside this task; it was not edited.

| Outside-scope path | Captured SHA-256 | Observed SHA-256 |
| --- | --- | --- |
| `00_nucleo/prompts/infra/export/bitmap_glyphs.md` | `42f453bf65eb2a0ed904abe71f6c0de4cee5a34ee4695aa54e021996e5693c69` | `33d8ead7d017d73943d4acf2d297d6d521830d9971927799e02709952f16b794` |
| `00_nucleo/prompts/infra/export/font_subset.md` | `ba5b1e5448ee2415221641a83e133899416be6b8729c2d8f00375c2632dca8f5` | `2054c234b5f887bd53ccf91b54f066bc22e91b5d0789d94550d003d24d81af09` |
| `00_nucleo/prompts/infra/export/fonts.md` | `4dc7735f438c913ba97112a4415fb7eab2507b3a127679afeccf06ac41ebb733` | `0a27b2c6cf521f7b6a0702a550ae26f324dc9f35989dce124cbfc8986d3b0b58` |
| `00_nucleo/prompts/infra/export/oracle.md` | `27e5cbb28c482b8f9b0d787a309590a530739c6bd3c4db1f5d3bb64eefdc1c6a` | `2d86e8e115d623c1146243fe0848a9fc5fd3183e07b0fe6e7cdfc312837fa1e2` |
| `00_nucleo/prompts/infra/export/pdf_defaults.md` | `85019da8d71aec43c026fb9a563d5bcf130b7c831fce874c2626addbe7e687cb` | `05d57b5690275e359a37fca00ec6aef67b5dbe3356e8a61c1fa3ef956c5a24a9` |
| `00_nucleo/prompts/infra/export/p1405_pdf_label_tests.md` | `df70eff74072c0b2bdd1cfd3d2f4bad51f9383a861818c5049514dcd57f22b36` | `7af6429b10f62776bdc72a2da2fdca6568924380f5e00dfb782f3e6b372839a6` |
| `00_nucleo/prompts/infra/export/render.md` | `fa1cf3e233badb60b6354f60b945a5ff7759581216ea3b22c1500173faa941b1` | `ff8ee598cac873282ae948190a85b29054a1a4fd96339a508f37ba4a913eb330` |
| `00_nucleo/prompts/infra/export/svg.md` | `62fffa4673dfe8ee45a10e3e5ded3e7e8905ff930194c9d6698a7253e9aecd0f` | `dedeb6528dae13d57fe6a76781e099018aab7aded405ec849a3319ed5809899d` |
| `00_nucleo/prompts/infra/export/p1250_svg_integrated.md` | `b7c8f6b7348104d18f841fa0c8f27bfa82b8ec588b8d4309c27cc64c9ef64eab` | `f344cedd52da09b6d7ba8ebcdc86803d0346a600d3b0484bdfc9a3e612014192` |
| `00_nucleo/prompts/infra/export/p1431-line-cap-tests.md` | `559159efadb15663daa761b2e6add104b4fa9e47549c0460eaca46a60fe95d39` | `c5ef17830897152106aadf73edd4cc5d08c840f2218e89d8c357e16b49fb80a6` |
| `00_nucleo/prompts/infra/export/mod.md` | `4a7c63f0650719b64d94d3f54b32f60aaafca2b8d8465eaded79254ac4b1b981` | `2b2c2e3f0d4019a2f7c841d3996fd512ee052c4a5188e6c16b7080dcc4a10438` |
| `00_nucleo/prompts/infra/export/images.md` | `c1ae6252bf94bcacd6e5bb0436cdb8d95218f4ed398e1240358a84d37db5ea6d` | `931f74944620928057cb0179daff171260c3162166b31a56bc50254dfc52d0c6` |
| `00_nucleo/prompts/infra/export/builder.md` | `8eba9a6984fc68380774ad035fb4474b608ce3168b4c00a35e03e7015e039810` | `72cc01c7e1f929ff8b455533a75d2a9c5540e6109b0648e33ffdd7c8d50d6d5d` |
| `00_nucleo/prompts/infra/export/html.md` | `2c2947c45dd795f3adc23559944cacec7a4c4b7f9fd2d89f1c9ef71d9cee14d2` | `879a31befb32d9700ad30c8f00ea25df61ff88a152bff335cd74753312fe433e` |
| `00_nucleo/prompts/infra/export/stream.md` | `38ea47584088c4a7afebba9b717522c4b8a9b6aa35d35c5c238c93ccf15dd290` | `8e50262c7195a1ce3825575026551caa455c86a5e10a1a010fbc274a9fc654a0` |
| `00_nucleo/prompts/infra/export/gradients/cmyk.md` | `d516ceb01765e3fc483a2312255773e73a2e98f816dc2f6a88d5804e089762a9` | `985c7845d88bb0ec10e4b398c3103451ae5ba3264f21dbbd963bd3aeed33737c` |
| `00_nucleo/prompts/infra/export/gradients/linear.md` | `8cdeddcbd3c2bb3f0f27cbaf7b202ae6d9a0d56893a21f51c6849ad252399d83` | `ebaea68ac7e3ba3d5fb918506815d9550b7ac1bd3b1daa8713071829a1921026` |
| `00_nucleo/prompts/infra/export/gradients/radial.md` | `ac277ab3189c5fb487a15b23f0fb3941f4e8f75782a0617158ed35ec6f90acaa` | `f57a15e459f492ac12ac9e293ad89342a785c40d9b024e1a24c70ae07a5c0512` |
| `00_nucleo/prompts/infra/export/gradients/relative.md` | `22d99c8180503fa81cc521d3ad6fc9dab30692ce97781ec4ada63d79e928a500` | `823b953dab76118ef9de74eea003182eeca2044e34efc94bab7cc3db66e70190` |
| `00_nucleo/prompts/infra/export/gradients/adaptive.md` | `1b91c358c7d78d2c8964dd0ed20b87bfab3126961699b3c3aaecd082cd6758a7` | `338e779fb8255e63a7b1ace891a17615e4ded79e77323392e82b23a4ac660c7c` |
| `00_nucleo/prompts/infra/export/gradients/conic.md` | `fd3d54c5aef0b4c5c99ebed6ef734e67b8c76e33dccc60dd23f28383eb3245f8` | `07bacb7114ac53084236f3553a5186380ada319072025661ece5552e619f18fe` |
| `00_nucleo/prompts/infra/export/gradients/function_dict.md` | `2cc3e2eaeaf297c97fbf12ba4871c49acd63fd1e40a4b33276ecf44b8f13e88e` | `afaf58631eb1e14bf8c2778d5f0ddd737d2585df2161dcbd10eaf266fc58850f` |
| `00_nucleo/prompts/infra/export/gradients/mod.md` | `bd7ef42488673c377b394a00217b6fb7372fab0177028555c2e698813b968c6e` | `1f97715d12ab580b9e4a751956318fc5204366af238e613640e1ca8a1fa1968f` |
| `00_nucleo/prompts/infra/export/tests/p1475_pdfa_conformance_core_tests.md` | `3e0c5be35d78e743bce6b1dcbb8f4bd2023fb0efb7784e801bf3070c6c5e0421` | `393aecc0c6370240028fc219ef952c00377c320a5beba593cf15ceb877de05cd` |
| `00_nucleo/prompts/infra/export/tests/p1476_pdfa_attachments_af_tests.md` | `1a1782ae407314bf95302eb56c6f6ca5a9656e16f5c4a1f8dbfc3018f825555d` | `d43faff6c4503b819d307a6bdb0cbd5e209203e3b025587a01afed5eb793157b` |
| `00_nucleo/prompts/infra/export/tests/p1477_pdfa_icc_version_and_cidset_tests.md` | `b7ad5f4577c8ef782ace66bc08783dd028ff2ea929e1e0a616af7b15ac6f4924` | `7c8d0b8f4df2cc0763dd3af9775e8d4015cbed81dc3d56cce143d4dac48f38f1` |
| `00_nucleo/prompts/infra/export/tests/p1478_pdfa4f_empty_embedded_files_tests.md` | `27717b6d3a42317a97b81e01e2b89f0012e0112ce2af2c2373b1f076d3715132` | `98ab6cb1fec75f4083579e0e126d6d9bf0382db7f92635fd4f551abdb66ea9d6` |
| `00_nucleo/prompts/infra/export/tests/p1480_pdfua_identification_tests.md` | `e5a019124c234cc47a79b63ba859a7388d42112d2f9ab73e8e2baca1ffd5ac1f` | `f4818ad07b488799c801a59365876c7c07c54da427b27bd6ab30be0242fa43fb` |
| `00_nucleo/prompts/infra/export/tests/p1481_pdfua_structure_tests.md` | `68e44a12a3fcc04df650189fe23b9608dfc321425d1fb924c4b9e4860a6b26be` | `1642feb57ed2fe049f8f42e5fa89c9c9d5c6ff04f3e60d9777860d845e75bb33` |
| `00_nucleo/prompts/infra/export/tests/p1483_pdf_spot_tests.md` | `4f53d5796df55387ea680c2609ddc0f318f18c41693d39fbfd85229bea0437cd` | `e39db5b1a3b3d92d99a9b2bb74bea6880e5acd817be18b3fe871aa8366db39a7` |
| `00_nucleo/prompts/infra/export/tests/mod.md` | `82c2dda07ef28f5f7d81fba9085c3bc2b8d5c55390464e02addde41ce8856852` | `1abd4f161fd3f9a6aee83df25efc94b3b3f8b0ef8778692ea7b7ee558537680a` |
| `00_nucleo/prompts/infra/export/tests/p1486_pdf_page_labels_tests.md` | `b007cd16c0c69ef116475897c6b0d1cbd68122c69e8520ae78c4a382edabfb17` | `7331b251a830104db405ac174c9f99fcf4cab2a6e6597356aa476b8969c791e6` |
| `00_nucleo/prompts/infra/export/builder/fonts.md` | `48afe1736e8f8e25939e5ed507486f20142f6a8a58f10075579d30cafaa44179` | `30f9b869ce5f336b151bc5933de449b5af7466c39e57a3aa7d7c78b6bd70ff94` |
| `00_nucleo/prompts/infra/export/builder/resources.md` | `0a9e8ffb4ba71bbdf49c06b143fd051ac4bd0c87d4e4ee07863cee37471bfe58` | `276a88cd659835335d9c68c8cb44d3674e38b0e6491d44b9ed4591d96938c3a9` |
| `00_nucleo/prompts/infra/export/builder/streams.md` | `f2c2a9e864a4c69898c51dd2989bfdd0a1028a71a4f66e10aa891730982fb96d` | `dd4b0c51044a82ea9be6f01d6cf91f012ff15055ab95abf215436940f1ea7d09` |
| `00_nucleo/prompts/infra/export/builder/metadata.md` | `52cf11542fa3ac605e542ca6a7e277c4006e476230becb5880fe5e85779303be` | `a431a5f6f0e09414be713c0b48315d05884ade697c2a293e4a503351f517287f` |
| `00_nucleo/prompts/infra/export/builder/navigation.md` | `eec45f5e9414d832de000d2e8ddd461df852a6a6d831d215997275acecdbe088` | `c27c695840516a2a76b7fd4afe5a08425be6c8f1f2e178b80ff4fbe2a8145d06` |
| `00_nucleo/prompts/infra/export/builder/serialization.md` | `b77f14524666665b2c8bdf43d200064ca5732800881e68ca461d54316fc3ecbd` | `dc0b69e97603f908872bd4b23cdc2e294a011f310648cffc2777140ea99bfff6` |
| `00_nucleo/prompts/infra/export/stream/draw.md` | `3efe7394c159da5ef7189fe7b02815ac897dcc6b48a2d6608b55c306753e54bc` | `beacbb267ff27f8cf7f730dc2ad6499d69bdc01415adf8d807170f02e268ef12` |
| `00_nucleo/prompts/infra/export/stream/page.md` | `46d04737ce338b499fbd4ddb5b151f0a2dc108187f31ffee515f12a017f5d08f` | `fe62d187a0e295a4a51a040c034aa41097cb6b3d46cce0e7513df23d6a37245f` |
| `00_nucleo/prompts/infra/export/stream/shape.md` | `f550bf1f9156eb08639f4ec940989c88e0d38522438765b6c30ebe57e7cf8841` | `74ae1fcc1db3d60a5b55b0741a28620fa5a921bf8485d685b6ac81498853bd13` |
| `00_nucleo/prompts/infra/export/stream/text.md` | `f7ee3ff6cbee1cbac20bbd03a7fdcf62b3b26dabfc9b60818f675091299db18a` | `024f766491e5e8e5c4800ae947564079fad1ce56500a9fbc50c30bba788b0556` |
| `00_nucleo/prompts/infra/export/images/icc.md` | `3c6e96b691c96887ccfa70fcdccbca6a50d1af9440fee4fdd71ccacfec740f3e` | `8cbd2afde264efa149b6cd9e9063bd7a5733a10ee944e9f47d3d3bca09149c86` |
| `00_nucleo/prompts/infra/export/images/jpeg.md` | `85062889979aa0425627ebfbbe093b7361e865470b875392ff99428432dff802` | `ddc21b3950cb71edae76b0885575397dffa7cc736934a20a06d21783445d32b1` |
| `00_nucleo/prompts/infra/export/images/png.md` | `41046c35ec5f966aac4c9c0852e65f6f2ec9d070924dab5956e6b57613756d5c` | `38a87355708c0fac6e00cddffafe155afcfd61e2c047af133351e192ffcb7839` |
| `00_nucleo/prompts/infra/export/images/scan.md` | `996ff6a991c58d40472413a641ca7271147337595f57eb8d142f2507b197e1fb` | `b8be30261445fb5c7a7cb630ec34c85912a6413b2bd42415763f38be32373e25` |
| `00_nucleo/prompts/infra/export/images/xobjects.md` | `bf3bed401bc08e645599bd126117f7428fe42e5cbe823c66e81933e6f6b1825d` | `992f90e9534bb9703a288475c3d9d046a701804b86e1bd9c7e90287daa758be0` |
| `00_nucleo/prompts/infra/export/html/document.md` | `2bd53aa14e38e89dd344d3a3f9530bedc18ed5ec5fef223790717ac0b90e018d` | `44bb1632b19bcf9d5b29c98e12b4c0305c9dd5953ec69497ecc290e4dec14b8d` |
| `00_nucleo/prompts/infra/export/html/inline.md` | `a895c57f552726bfdf461952ebfff1acb66e9a5c6b89c118518d94d20c326714` | `c8dbe26a6b29a122617055ef6e42f9e15b9f773c5ab38d84e4a8bbbc35142356` |
| `00_nucleo/prompts/infra/export/html/table.md` | `9b7107e4f7d0c262736d382d233670207b86f58d81e642220a79e0b867fcddca` | `07b0957553b5b2610b848f3ec8e0476ac44b58f7532ec4640031a386ecef7aae` |
| `00_nucleo/prompts/infra/export/html/whitespace.md` | `88368f5f17b8a6cdbad64673291f1e0f7c3b1bdbd06c68da1ece532f44ef8ab6` | `2d8c263ed022ca66f210fdb1c9fe482433a00939a604263f4cc66102b6ef17ca` |
| `00_nucleo/prompts/infra/export/html/encoding.md` | `12a7c27f26fd8e7fa28e93b898aa7362fbae39cdc49c902b83c52d7c921d19e4` | `ef08cf92029b04b2d065b41f48ee074e6532cd5073a39851ecfafb821a0d8511` |
| `00_nucleo/prompts/infra/export/html/pretty.md` | `b197a40dabfaeabfb04fa081325e0a4a631848e67423ce8ecc03baba32f5a84b` | `0a2867112ce9f38e8acbe5a837d271c2b0cd012a2cc9f3877667f843efa07c5d` |
| `00_nucleo/prompts/infra/export/html/tags.md` | `f851eff1fde2adc41557ca62df2e3d25b368fcb0e7d7261f641e41df8f6a0a12` | `223b6a40e6b3eda12be22ec6c43bad80fa29e34fd471d34a0f4f69ae7406c63b` |
| `00_nucleo/prompts/infra/export/html/tests.md` | `a7ed00ed68208702078aee0d4de7d1621cc67e32f8d1f6faf42843a79d095816` | `ad4a2c22039c77e9cbad5de1030a0b141b5175b4e7cd7a68e6fcee5f9a3a424a` |
| `00_nucleo/prompts/infra/export/html/p1360_ab_tests.md` | `102d5b10a6270d73ec72c0addb1745a3ae303ad677d7a978a60e3ae79698c7d0` | `b2f156b1eae77230e43818ac10793f2f76b1e8ad41faa57d8bb370cadd72db85` |
| `00_nucleo/prompts/infra/export/svg/paints.md` | `c43b544e4b9e06bb44e832c696915c6445e3abc17b6186a0bb42cbc84ae54c4e` | `cab60aa238878262178a85665552333a908a9996409838336a5866744e28588f` |
| `00_nucleo/prompts/infra/export/svg/glyphs.md` | `6ba1724003e6286bf0deadb36ca4b55236d8c1eae137615ae2f639b5e36e0bda` | `af8dc2b191d417120a35ad80b812cc80ab591781a023703c083fe6eebd656f6d` |
| `00_nucleo/prompts/infra/export/svg/images.md` | `4b861c7828cffba6f627abb974ff558f1d2903bfb5860abbdeaf5bf0beb82b83` | `f4f1d21f1eba818b61300fd221661793bd07725a174fcf00ab5e283d66d0ac38` |
| `00_nucleo/prompts/infra/export/svg/geometry.md` | `bc63cfe4e52f508c27ccb65ed4e68204bd6fd5845e13d6d092434f3852e3363e` | `ca38d1ff37502fd12f65a150828f8939fee00bf9d240c8ed1821c333463ea704` |
| `00_nucleo/prompts/infra/export/svg/groups.md` | `1e4d4f61cafca71a91d4f92d51ed07545c133a71d87261afd13c82dbaf47112a` | `880e33a0d25ebebb4124d8781ca338f2df79ad3d1e2862466cad15c991ba733c` |
| `00_nucleo/prompts/infra/export/svg/navigation.md` | `0d235e046625070f79d110ca370b815f81b9ca949febc132c7753268282e2a5f` | `6f587b7094965c790560904c6f472d34cee42fcb29a91b7e71a1bc71b422c73b` |
| `00_nucleo/prompts/infra/export/svg/serialization.md` | `e2c4e78093ecc8fdc8c06f3a11c4a16649692f0532d178459a6f59e23241f464` | `614b2767131accde29c05f222aac30507d0e4fb75be72f18e7f7130a3aeb842b` |
| `00_nucleo/prompts/infra/export/svg/tests.md` | `d878d062b989601662c11c7be8c7424b6db8e09e28b4e3f6a0e150f2a555853b` | `3589b1f4bc6354d9c4eee398949fe0254774582e0dacfd61eeb08d564f89e352` |
| `00_nucleo/prompts/infra/export/svg/p1360_ab_tests.md` | `d103df35cdc4c299f6f5c017ca9f67bc15f05c61212247a5fa3ff495a0849319` | `e899c7773c0f85cfaede51c96d395af6bd98a92f00187fec7a59825fea87c1ce` |
| `00_nucleo/prompts/compiler/footnote_overflow_columns.md` | `c2b36956b2c61d0f663e2d81f4dec4c52400714236855b11b17242abf1252571` | `221a0624f7c5a15110be2771563488d5ef53ed429d2ad5e561ec707545a332c7` |
| `00_nucleo/prompts/compiler/introspect.md` | `a9ec81e1596e145a4b3592a7cbaf1f027c5db15a0a81ae65172ce77807c67e6e` | `ff2c8c9fd1192b47b35fe0d43539465792507734e5fe8d1394ddaa24c354d12f` |
| `00_nucleo/prompts/compiler/columns.md` | `441914e618e586463cf084fe309ff1779068037cf481c84f2318267d1b9815f7` | `69a3988894704c95fffe53ac3bfbe9ba3078ca61ba6e48ab52001ea05fc37919` |
| `00_nucleo/prompts/compiler/introspect/convergence.md` | `0334a735b2a632be3f9562f610958a6bcac40a13bada3757dbbee3f3d7e06a88` | `114a71a44bd8ca8f03ac3edc4577f3c914d50c9f5482785c7edd03363ab29915` |
| `00_nucleo/prompts/compiler/introspect/fixpoint.md` | `9a0fb90452c91a75fe1f091af514f782b2c16a3c6d3b0b95a52b137748b76b1b` | `29cc3ddc3154e031b1d0cb91a6ecd3f866085ff66e69f68d8755dfa01b7fe293` |
| `00_nucleo/prompts/compiler/introspect/labelled.md` | `8f2122b0adf8d5b1d6be95bfcf911d253be4a8ed535a3aa191e93862828fa1bf` | `d42eba2268848075ebca61128be43ad5d5051cd45f3a14281f1a4b6d22781d23` |
| `00_nucleo/prompts/compiler/introspect/extract_payload.md` | `c2ff13632be6ebc395634643097f4f13c134eab6c95080bdbb4a53fb69eaf301` | `2368d65a323814ac6cc3a7f89453d045571f7f6b908669bd645bafcbad7ccaf9` |
| `00_nucleo/prompts/compiler/introspect/heading.md` | `b731524d15e71111a7ced67627582cde5094cbeac598387b49d0f3c8c64758e4` | `33f8e6ce9fa62e503457fee7370a1bfafdaaa03e86fcd72a3e416c3c94b289e3` |
| `00_nucleo/prompts/compiler/introspect/from_tags.md` | `ce580c45284df1348226e6da3bbe3565e66fbec0f99f8aba6a83152de78a6eed` | `fe5f02b1787f93cb002c5d02ec7efc0ea10c409aac344fa42b4395b4d53f7236` |
| `00_nucleo/prompts/compiler/introspect/locatable.md` | `e3cbed3fe1c4180f384256228601c0b18a0231d39e92859ba83176103748fd29` | `56372bd7d1b48b5bbc542c9a312ecf77f731d29d3853de15cbbe06dec9542a1a` |

This is a time-specific receipt, not a claim that later concurrent work stopped.

## Initial working-tree inventory

This complete captured status is historical provenance for this batch. A listed
path does not imply that it was edited by this task. Generated target artifacts
are not tracked documentary inputs. The capture preserves filenames and Git
state rather than deriving any line-count metric.

<details>
<summary>Initial git status --short --untracked-files=all</summary>

```text
 M 00_nucleo/adr/README.md
 D 00_nucleo/adr/adr-stub-vs-fallback.md
 M 00_nucleo/adr/template-adr.md
 M 00_nucleo/adr/typst-adr-0001-estrategia-migracao.md
 D 00_nucleo/adr/typst-adr-0002-hierarquia-contencao.md
 D 00_nucleo/adr/typst-adr-0003-comemo-contencao.md
 D 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md
 M 00_nucleo/adr/typst-adr-0005-packagespec-world.md
 M 00_nucleo/adr/typst-adr-0006-typst-timing.md
 D 00_nucleo/adr/typst-adr-0007-rustc-hash.md
 M 00_nucleo/adr/typst-adr-0008-defer-inline.md
 M 00_nucleo/adr/typst-adr-0009-math-class.md
 M 00_nucleo/adr/typst-adr-0010-unicode-ident.md
 M 00_nucleo/adr/typst-adr-0011-unicode-math-class.md
 M 00_nucleo/adr/typst-adr-0012-unicode-script.md
 M 00_nucleo/adr/typst-adr-0013-unicode-segmentation.md
 M 00_nucleo/adr/typst-adr-0014-unscanny.md
 M 00_nucleo/adr/typst-adr-0015-ecow.md
 M 00_nucleo/adr/typst-adr-0016-lazyhash.md
 M 00_nucleo/adr/typst-adr-0017-adiamento-eval-typst-library.md
 M 00_nucleo/adr/typst-adr-0018-rustc-hash.md
 M 00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md
 M 00_nucleo/adr/typst-adr-0020-fontdb.md
 M 00_nucleo/adr/typst-adr-0021-datetime.md
 M 00_nucleo/adr/typst-adr-0022-fontbook.md
 M 00_nucleo/adr/typst-adr-0023-indexmap.md
 M 00_nucleo/adr/typst-adr-0024-ecow-value.md
 M 00_nucleo/adr/typst-adr-0025-int-eq-float.md
 M 00_nucleo/adr/typst-adr-0026-R1-content-arc.md
 M 00_nucleo/adr/typst-adr-0026-content-divergencia.md
 M 00_nucleo/adr/typst-adr-0027-cidfont-subsetting.md
 D 00_nucleo/adr/typst-adr-0028-typographic-types.md
 M 00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md
 M 00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md
 M 00_nucleo/adr/typst-adr-0031-early-hashing-source.md
 M 00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md
 M 00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md
 M 00_nucleo/adr/typst-adr-0034-diagnostico-tipos-vanilla.md
 M 00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md
 M 00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md
 M 00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md
 M 00_nucleo/adr/typst-adr-0038-sistema-estilos-l1.md
 M 00_nucleo/adr/typst-adr-0039-frameitem-style.md
 M 00_nucleo/adr/typst-adr-0040-activacao-set-rule.md
 M 00_nucleo/adr/typst-adr-0041-activacao-show-rule.md
 M 00_nucleo/adr/typst-adr-0042-sink-materializado.md
 M 00_nucleo/adr/typst-adr-0043-canal-sink-saida.md
 M 00_nucleo/adr/typst-adr-0044-engine-agregador.md
 M 00_nucleo/adr/typst-adr-0045-formato-diagnosticos.md
 M 00_nucleo/adr/typst-adr-0046-cli-minima.md
 M 00_nucleo/adr/typst-adr-0047-cli-clap.md
 M 00_nucleo/adr/typst-adr-0048-diagnosticos-com-cor.md
 M 00_nucleo/adr/typst-adr-0049-cli-em-l2.md
 M 00_nucleo/adr/typst-adr-0050-formatter-em-l2.md
 M 00_nucleo/adr/typst-adr-0051-flags-funcionais.md
 M 00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md
 M 00_nucleo/adr/typst-adr-0053-font-tipo-composto.md
 M 00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md
 M 00_nucleo/adr/typst-adr-0055-font-consumer-cidfont.md
 M 00_nucleo/adr/typst-adr-0057-lang-hyphenation.md
 M 00_nucleo/adr/typst-adr-0058-value-type-simplificado.md
 M 00_nucleo/adr/typst-adr-0059-args-tipo-separado.md
 D 00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md
 D 00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md
 M 00_nucleo/adr/typst-adr-0062-hayagriva-bibliography-parsing.md
 M 00_nucleo/adr/typst-adr-0064-smart-para-option-default.md
 M 00_nucleo/adr/typst-adr-0065-inventariar-primeiro.md
 D 00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md
 M 00_nucleo/adr/typst-adr-0067-attribute-grammar-scoping.md
 M 00_nucleo/adr/typst-adr-0068-location-aware-layouter.md
 M 00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md
 M 00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md
 M 00_nucleo/adr/typst-adr-0071-walk-pipeline-redesign.md
 M 00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime-fechado.md
 M 00_nucleo/adr/typst-adr-0073-comemo-introspector.md
 M 00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md
 M 00_nucleo/adr/typst-adr-0075-vanilla-integration.md
 M 00_nucleo/adr/typst-adr-0076-introspector-completion.md
 M 00_nucleo/adr/typst-adr-0077-regex-l1.md
 M 00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md
 D 00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md
 M 00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md
 D 00_nucleo/adr/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md
 M 00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md
 M 00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md
 M 00_nucleo/adr/typst-adr-0084-auditoria-condicional.md
 M 00_nucleo/adr/typst-adr-0085-diagnostico-imutavel.md
 M 00_nucleo/adr/typst-adr-0086-paint-wrapper-solid-only.md
 M 00_nucleo/adr/typst-adr-0087-gradient-linear-only.md
 M 00_nucleo/adr/typst-adr-0088-gradient-radial-only.md
 M 00_nucleo/adr/typst-adr-0089-gradient-conic-only.md
 D 00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md
 M 00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md
 M 00_nucleo/adr/typst-adr-0092-conic-coons-patches-rgb-cmyk.md
 M 00_nucleo/adr/typst-adr-0093-meta-metodologia-evolucao-adrs.md
 M 00_nucleo/adr/typst-adr-0094-meta-operacional-specs.md
 M 00_nucleo/adr/typst-adr-0095-dedup-arc-as-ptr-resources.md
 D 00_nucleo/adr/typst-adr-0096-pattern-debt-37-campo-layouter-consumer-pending.md
 D 00_nucleo/adr/typst-adr-0097-scope-out-reconfirmado-por-fase-a.md
 M 00_nucleo/adr/typst-adr-0098-single-source-of-truth-invariante-anti-bug.md
 M 00_nucleo/adr/typst-adr-0099-activacao-posterior-feature-graded.md
 M 00_nucleo/adr/typst-adr-0100-coesao-por-dominio-l3.md
 M 00_nucleo/adr/typst-adr-0101-ieee754-restricao-stdlib.md
 M 00_nucleo/adr/typst-adr-0102-math-style-mechanism.md
 M 00_nucleo/adr/typst-adr-0103-math-style-composition.md
 M 00_nucleo/adr/typst-adr-0104-atomicidade-para-agentes.md
 M 00_nucleo/adr/typst-adr-0105-modelo-elemento-d-agora-f-destino.md
 M 00_nucleo/adr/typst-adr-0106-fronteira-de-extensao-e1.md
 M 00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md
 M 00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md
 M 00_nucleo/adr/typst-adr-0109-atomizacao.md
 M 00_nucleo/adr/typst-adr-0110-marcador-vanilla.md
 M 00_nucleo/adr/typst-adr-0111-crates-parsing-loading.md
 M 00_nucleo/adr/typst-adr-0112-rust-decimal-l1.md
 M 00_nucleo/adr/typst-adr-0113-stub-transparente-vs-fallback-software.md
 M 00_nucleo/adr/typst-adr-0114-sonda-antes-da-spec.md
 M 00_nucleo/adr/typst-adr-0115-infra-benchmark-scanner.md
 M 00_nucleo/adr/typst-adr-0116-excecao-get-unchecked-scanner.md
 M 00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md
 M 00_nucleo/adr/typst-adr-0118-runtime-state-context.md
 M 00_nucleo/adr/typst-adr-0119-disciplina-verificacao.md
 M 00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md
 M 00_nucleo/adr/typst-adr-0121-proveniencia-medicao.md
 M 00_nucleo/adr/typst-adr-0122-paridade-defeitos-testes.md
 M 00_nucleo/adr/typst-adr-0123-geometria-tipografica.md
 M 00_nucleo/adr/typst-adr-0124-checklist-sublayouts.md
 M 00_nucleo/adr/typst-adr-0125-implementacao-vs-linguagem.md
 M 00_nucleo/adr/typst-adr-0126-modo-verboso-primeiro.md
 M 00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md
 M 00_nucleo/adr/typst-adr-0128-html-target-semantico.md
 M 00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md
 M 00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md
 M 00_nucleo/prompts/compiler/eval.md
 M 00_nucleo/prompts/compiler/layout.md
 M 00_nucleo/prompts/compiler/layout/equation.md
 M 00_nucleo/prompts/compiler/layout/page_geometry.md
 M 00_nucleo/prompts/compiler/layout/parbreak.md
 M 00_nucleo/prompts/compiler/layout/placement.md
 M 00_nucleo/prompts/compiler/layout/tests.md
 M 00_nucleo/prompts/compiler/layout/word_segment.md
 M 00_nucleo/prompts/compiler/parse/parser.md
 M 00_nucleo/prompts/compiler/stdlib/_comum.md
 M 00_nucleo/prompts/entities/content.md
 M 00_nucleo/prompts/entities/content_hash.md
 M 00_nucleo/prompts/entities/syntax-node.md
 M 00_nucleo/prompts/infra/export/builder.md
 M 00_nucleo/prompts/infra/export/html.md
 M 00_nucleo/prompts/infra/export/images.md
 M 00_nucleo/prompts/infra/export/p1250_svg_integrated.md
 M 00_nucleo/prompts/infra/export/stream.md
 M 00_nucleo/prompts/infra/export/svg.md
 M 00_nucleo/prompts/infra/font_metrics.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
 M 00_nucleo/prompts/infra/plugin_host.md
 M 00_nucleo/prompts/infra/shaper.md
 M 00_nucleo/prompts/infra/system-world.md
 M 00_nucleo/prompts/wiring.md
 M 00_nucleo/prompts/wiring/eviction.md
 M 00_nucleo/prompts/wiring/tests/cli.md
 M 00_nucleo/prompts/wiring/tests/crystalline_lint.md
 M 00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md
 M 00_nucleo/prompts/wiring/tests/p1291_callback_runtime.md
 M 00_nucleo/prompts/wiring/tests/p1292_contract.md
 M 00_nucleo/prompts/wiring/tests/p1293_contract.md
 M 00_nucleo/prompts/wiring/tests/p1462-format-svg.md
 M 00_nucleo/prompts/wiring/tests/p1463-format-png.md
 M 00_nucleo/prompts/wiring/tests/p1464-format-pdf.md
 M 00_nucleo/prompts/wiring/tests/p1465-format-html.md
 M 00_nucleo/prompts/wiring/tests/p1467-format-pdf-tagged-pretty.md
 M 00_nucleo/prompts/wiring/tests/p1468-format-pdf-pages.md
 M 00_nucleo/prompts/wiring/tests/p1469-format-pdf-standard.md
 M 01_core/Cargo.toml
 M 01_core/src/compiler/eval/bibliography.rs
 M 01_core/src/compiler/eval/bindings/field_access.rs
 M 01_core/src/compiler/eval/bindings/method_dispatch.rs
 M 01_core/src/compiler/eval/bindings/value_methods.rs
 M 01_core/src/compiler/eval/call_dispatch.rs
 M 01_core/src/compiler/eval/control_flow.rs
 M 01_core/src/compiler/eval/font_dict.rs
 M 01_core/src/compiler/eval/math.rs
 M 01_core/src/compiler/eval/mod.rs
 M 01_core/src/compiler/eval/modules.rs
 M 01_core/src/compiler/eval/operators/arithmetic.rs
 M 01_core/src/compiler/eval/p1427_tests.rs
 M 01_core/src/compiler/eval/p1430_tests.rs
 M 01_core/src/compiler/eval/p1431_tests.rs
 M 01_core/src/compiler/eval/repr.rs
 M 01_core/src/compiler/eval/rules.rs
 M 01_core/src/compiler/eval/selector_matching.rs
 M 01_core/src/compiler/eval/tests.rs
 M 01_core/src/compiler/introspect.rs
 M 01_core/src/compiler/introspect/fixpoint.rs
 M 01_core/src/compiler/introspect/from_tags.rs
 M 01_core/src/compiler/introspect/locatable.rs
 M 01_core/src/compiler/layout/columns.rs
 M 01_core/src/compiler/layout/decorations.rs
 M 01_core/src/compiler/layout/equation.rs
 M 01_core/src/compiler/layout/format_options.rs
 M 01_core/src/compiler/layout/mod.rs
 M 01_core/src/compiler/layout/page_geometry.rs
 M 01_core/src/compiler/layout/parbreak.rs
 M 01_core/src/compiler/layout/placement.rs
 M 01_core/src/compiler/layout/sequence.rs
 M 01_core/src/compiler/layout/set_page.rs
 M 01_core/src/compiler/layout/tests.rs
 M 01_core/src/compiler/layout/word_segment.rs
 M 01_core/src/compiler/math/layout/accent.rs
 M 01_core/src/compiler/math/layout/attach.rs
 M 01_core/src/compiler/math/layout/mod.rs
 M 01_core/src/compiler/math/layout/p1430_tests.rs
 M 01_core/src/compiler/math/layout/p1431_tests.rs
 M 01_core/src/compiler/math/layout/spacing.rs
 M 01_core/src/compiler/math/layout/tests.rs
 M 01_core/src/compiler/parse/parser.rs
 M 01_core/src/compiler/stdlib/color.rs
 M 01_core/src/compiler/stdlib/counter.rs
 M 01_core/src/compiler/stdlib/format.rs
 M 01_core/src/compiler/stdlib/foundations/color.rs
 M 01_core/src/compiler/stdlib/gradients.rs
 M 01_core/src/compiler/stdlib/layout.rs
 M 01_core/src/compiler/stdlib/loading.rs
 M 01_core/src/compiler/stdlib/mod.rs
 M 01_core/src/compiler/stdlib/structural/document.rs
 M 01_core/src/compiler/stdlib/structural/math.rs
 M 01_core/src/compiler/stdlib/structural/mod.rs
 M 01_core/src/compiler/stdlib/structural/outline.rs
 M 01_core/src/compiler/stdlib/structural/p1429_tests.rs
 M 01_core/src/compiler/stdlib/structural/p1430_tests.rs
 M 01_core/src/compiler/stdlib/structural/par.rs
 M 01_core/src/compiler/stdlib/text/constructor.rs
 M 01_core/src/entities/color.rs
 M 01_core/src/entities/content.rs
 M 01_core/src/entities/content_hash.rs
 M 01_core/src/entities/elements/figure_caption.rs
 M 01_core/src/entities/elements/footnote_entry.rs
 M 01_core/src/entities/gradient.rs
 M 01_core/src/entities/locator.rs
 M 01_core/src/entities/style.rs
 M 01_core/src/entities/style_chain.rs
 M 01_core/src/entities/syntax_node.rs
 M 02_shell/src/cli.rs
 M 03_infra/src/export/builder.rs
 M 03_infra/src/export/html.rs
 M 03_infra/src/export/images.rs
 M 03_infra/src/export/stream.rs
 M 03_infra/src/export/svg.rs
 M 03_infra/src/export/tests/mod.rs
 M 03_infra/src/export/tests/p1475_pdfa_conformance_core_tests.rs
 M 03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs
 M 03_infra/src/export/tests/p1477_pdfa_icc_version_and_cidset_tests.rs
 M 03_infra/src/export/tests/p1478_pdfa4f_empty_embedded_files_tests.rs
 M 03_infra/src/font_metrics.rs
 M 03_infra/src/integration_tests.rs
 M 03_infra/src/pipeline.rs
 M 03_infra/src/pipeline/context_stabilization.rs
 M 03_infra/src/plugin_host.rs
 M 03_infra/src/query_helpers.rs
 M 03_infra/src/shaper.rs
 M 03_infra/src/watch.rs
 M 03_infra/src/world.rs
 M 03_infra/tests/p1250_svg_integrated.rs
 M 03_infra/tests/p1405_pdf_label.rs
 M 04_wiring/src/eviction.rs
 M 04_wiring/src/main.rs
 M 04_wiring/tests/cli.rs
 M 04_wiring/tests/crystalline_lint.rs
 M 04_wiring/tests/p1289_float_is_infinite.rs
 M 04_wiring/tests/p1291_callback_runtime.rs
 M 04_wiring/tests/p1292_contract.rs
 M 04_wiring/tests/p1293_contract.rs
 M 04_wiring/tests/p1462_format_svg.rs
 M 04_wiring/tests/p1463_format_png.rs
 M 04_wiring/tests/p1464_format_pdf.rs
 M 04_wiring/tests/p1465_format_html.rs
 M 04_wiring/tests/p1467_format_pdf_tagged_pretty.rs
 M 04_wiring/tests/p1468_format_pdf_pages.rs
 M 04_wiring/tests/p1469_format_pdf_standard.rs
 M Cargo.lock
 M benches/Cargo.toml
 M benches/scanner_bench.rs
?? 00_nucleo/adr/ORDER.md
?? 00_nucleo/diagnosticos/adr-0130-step-1060-history.md
?? 00_nucleo/diagnosticos/adr-archive/README.md
?? 00_nucleo/diagnosticos/adr-archive/adr-0113-draft-stub-vs-fallback-portuguese.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0004-passo1-descobertas.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0060-model-structural-roadmap.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0061-layout-fase-x-roadmap.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0079-layout-fase-5-roadmap.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0096-pattern-debt-37-campo-layouter-consumer-pending.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0097-scope-out-reconfirmado-por-fase-a.md
?? 00_nucleo/diagnosticos/adr-archive/redirects.tsv
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0002-hierarquia-contencao.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0003-comemo-contencao.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0007-rustc-hash.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0028-typographic-types.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0066-introspection-runtime-adiada.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md
?? 00_nucleo/diagnosticos/adr-cleanup-classification.md
?? 00_nucleo/diagnosticos/adr-governance-reconciliation.md
?? 00_nucleo/diagnosticos/adr-performance-quantity-analysis.md
?? 00_nucleo/diagnosticos/adr-reference-legacy-portuguese.md
?? 00_nucleo/diagnosticos/adr-template-legacy-portuguese.md
?? 00_nucleo/diagnosticos/benchmark-suite-review-state.json
?? 00_nucleo/diagnosticos/benchmark-suite-review.md
?? 00_nucleo/diagnosticos/content-microbenchmarks.md
?? 00_nucleo/diagnosticos/core-l0-sanitization-history.md
?? 00_nucleo/diagnosticos/core-l0-sanitization-manifest.json
?? 00_nucleo/diagnosticos/core-l0-sanitization.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/README.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/authorized-final-checks.json
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/authorized-reseal.json
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/fonts.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/metadata.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/navigation.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/resources.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/serialization.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/streams.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/final-checks.json
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/document.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/encoding.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/inline.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/p1360_ab_tests.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/pretty.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/table.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/tags.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/tests.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/whitespace.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/icc.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/jpeg.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/png.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/scan.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/xobjects.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/manifest.json
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/seal-plan.json
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream/draw.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream/page.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream/shape.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream/text.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/geometry.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/glyphs.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/groups.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/images.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/navigation.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/p1360_ab_tests.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/paints.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/serialization.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/tests.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/validation.md
?? 00_nucleo/diagnosticos/font-metrics-atomization.md
?? 00_nucleo/diagnosticos/font-metrics-l0-before-atomization.md
?? 00_nucleo/diagnosticos/html-atomization-manifest.json
?? 00_nucleo/diagnosticos/html-atomization.md
?? 00_nucleo/diagnosticos/images-atomization.md
?? 00_nucleo/diagnosticos/images-l0-before-atomization.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/README.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/basic.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/caches.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/fallback.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/math.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/resolution.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/bundle.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/context_stabilization.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/html.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/paged.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/pdf.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/raster.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/resources.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/svg.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/tests.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/execution.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/loading.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/snapshot.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/tests.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/typst_env.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/bidi.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/cache.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/font_selection.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/reconciliation.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/shaping.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/tests.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/traversal.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/cache.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/fonts.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/packages.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/paths.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/tests.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/world_impl.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/manifest.json
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/compile.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/completions.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/diagnostics.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/eval.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/eviction.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/fonts.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/info.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/init.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/query.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/cli.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/crystalline_lint.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1289_float_is_infinite.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1291_callback_runtime.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1292_contract.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1293_contract.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1462-format-svg.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1463-format-png.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1464-format-pdf.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1465-format-html.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1467-format-pdf-tagged-pretty.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1468-format-pdf-pages.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1469-format-pdf-standard.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/watch.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-sanitization-2026-09-26.md
?? 00_nucleo/diagnosticos/layout-final-atomization.md
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md
?? 00_nucleo/diagnosticos/observation-microbenchmarks.md
?? 00_nucleo/diagnosticos/p1505-content-representation-ab.md
?? 00_nucleo/diagnosticos/p1506-pdf-builder-atomizacao.md
?? 00_nucleo/diagnosticos/parser-atomization.md
?? 00_nucleo/diagnosticos/parser-l0-before-atomization.md
?? 00_nucleo/diagnosticos/pdf-builder-microbenchmarks.md
?? 00_nucleo/diagnosticos/pipeline-atomization.md
?? 00_nucleo/diagnosticos/pipeline-microbenchmarks-baseline.json
?? 00_nucleo/diagnosticos/pipeline-microbenchmarks.md
?? 00_nucleo/diagnosticos/plugin-host-atomization.md
?? 00_nucleo/diagnosticos/plugin-host-l0-before-atomization.md
?? 00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md
?? 00_nucleo/diagnosticos/stdlib-microbenchmarks.md
?? 00_nucleo/diagnosticos/stdlib-registry-atomization.md
?? 00_nucleo/diagnosticos/stdlib-tests-atomization.md
?? 00_nucleo/diagnosticos/stream-atomization.md
?? 00_nucleo/diagnosticos/svg-atomization.md
?? 00_nucleo/diagnosticos/svg-l0-before-atomization.md
?? 00_nucleo/diagnosticos/syntax-node-atomization.md
?? 00_nucleo/diagnosticos/tekt-english-reference-pilot.md
?? 00_nucleo/diagnosticos/wiring-main-atomization.md
?? 00_nucleo/diagnosticos/world-atomization-manifest.json
?? 00_nucleo/diagnosticos/world-atomization.md
?? 00_nucleo/materialization/tekt-english-reference-migration.md
?? 00_nucleo/prompts/_nuclei/eval/observation-relation.toml
?? 00_nucleo/prompts/compiler/eval/observation.md
?? 00_nucleo/prompts/compiler/eval/observation/content.md
?? 00_nucleo/prompts/compiler/eval/observation/elements.md
?? 00_nucleo/prompts/compiler/eval/observation/geometry.md
?? 00_nucleo/prompts/compiler/eval/observation/math.md
?? 00_nucleo/prompts/compiler/eval/observation/page.md
?? 00_nucleo/prompts/compiler/eval/observation/style.md
?? 00_nucleo/prompts/compiler/eval/observation/value.md
?? 00_nucleo/prompts/compiler/layout/align.md
?? 00_nucleo/prompts/compiler/layout/context_block.md
?? 00_nucleo/prompts/compiler/layout/counter_display.md
?? 00_nucleo/prompts/compiler/layout/counter_display_callback.md
?? 00_nucleo/prompts/compiler/layout/measurement.md
?? 00_nucleo/prompts/compiler/layout/outline_entry.md
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md
?? 00_nucleo/prompts/compiler/layout/space.md
?? 00_nucleo/prompts/compiler/layout/state_display.md
?? 00_nucleo/prompts/compiler/layout/style_scope.md
?? 00_nucleo/prompts/compiler/parse/parser/checkpoints.md
?? 00_nucleo/prompts/compiler/parse/parser/diagnostics.md
?? 00_nucleo/prompts/compiler/parse/parser/state.md
?? 00_nucleo/prompts/compiler/stdlib/registry/contextual.md
?? 00_nucleo/prompts/compiler/stdlib/registry/data.md
?? 00_nucleo/prompts/compiler/stdlib/registry/diagnostics.md
?? 00_nucleo/prompts/compiler/stdlib/registry/figure_image.md
?? 00_nucleo/prompts/compiler/stdlib/registry/formats.md
?? 00_nucleo/prompts/compiler/stdlib/registry/foundations.md
?? 00_nucleo/prompts/compiler/stdlib/registry/layout.md
?? 00_nucleo/prompts/compiler/stdlib/registry/math.md
?? 00_nucleo/prompts/compiler/stdlib/registry/microbenchmarks.md
?? 00_nucleo/prompts/compiler/stdlib/registry/mod.md
?? 00_nucleo/prompts/compiler/stdlib/registry/modules.md
?? 00_nucleo/prompts/compiler/stdlib/registry/shapes.md
?? 00_nucleo/prompts/compiler/stdlib/registry/structural.md
?? 00_nucleo/prompts/compiler/stdlib/registry/table_grid.md
?? 00_nucleo/prompts/compiler/stdlib/registry/tests.md
?? 00_nucleo/prompts/compiler/stdlib/registry/text.md
?? 00_nucleo/prompts/compiler/stdlib/tests/arguments.md
?? 00_nucleo/prompts/compiler/stdlib/tests/assertions.md
?? 00_nucleo/prompts/compiler/stdlib/tests/bibliography.md
?? 00_nucleo/prompts/compiler/stdlib/tests/calc.md
?? 00_nucleo/prompts/compiler/stdlib/tests/color.md
?? 00_nucleo/prompts/compiler/stdlib/tests/contextual.md
?? 00_nucleo/prompts/compiler/stdlib/tests/eval.md
?? 00_nucleo/prompts/compiler/stdlib/tests/figure_image.md
?? 00_nucleo/prompts/compiler/stdlib/tests/footnote.md
?? 00_nucleo/prompts/compiler/stdlib/tests/foundations.md
?? 00_nucleo/prompts/compiler/stdlib/tests/gradients.md
?? 00_nucleo/prompts/compiler/stdlib/tests/introspection.md
?? 00_nucleo/prompts/compiler/stdlib/tests/layout_containers.md
?? 00_nucleo/prompts/compiler/stdlib/tests/layout_context.md
?? 00_nucleo/prompts/compiler/stdlib/tests/layout_spacing.md
?? 00_nucleo/prompts/compiler/stdlib/tests/math.md
?? 00_nucleo/prompts/compiler/stdlib/tests/mod.md
?? 00_nucleo/prompts/compiler/stdlib/tests/shapes.md
?? 00_nucleo/prompts/compiler/stdlib/tests/stroke.md
?? 00_nucleo/prompts/compiler/stdlib/tests/support.md
?? 00_nucleo/prompts/compiler/stdlib/tests/symbols.md
?? 00_nucleo/prompts/compiler/stdlib/tests/table_grid.md
?? 00_nucleo/prompts/compiler/stdlib/tests/text.md
?? 00_nucleo/prompts/compiler/stdlib/tests/transforms.md
?? 00_nucleo/prompts/entities/content/construction.md
?? 00_nucleo/prompts/entities/content/emptiness.md
?? 00_nucleo/prompts/entities/content/equality.md
?? 00_nucleo/prompts/entities/content/fields.md
?? 00_nucleo/prompts/entities/content/identity.md
?? 00_nucleo/prompts/entities/content/morphology.md
?? 00_nucleo/prompts/entities/content/page-structure.md
?? 00_nucleo/prompts/entities/content/plain-text.md
?? 00_nucleo/prompts/entities/content/rust-hash.md
?? 00_nucleo/prompts/entities/content/sequence.md
?? 00_nucleo/prompts/entities/content/tests.md
?? 00_nucleo/prompts/entities/content/tests/morphology.md
?? 00_nucleo/prompts/entities/content/tests/projections.md
?? 00_nucleo/prompts/entities/content/tests/representation.md
?? 00_nucleo/prompts/entities/content/tests/sequence.md
?? 00_nucleo/prompts/entities/content/tests/traversal.md
?? 00_nucleo/prompts/entities/content/traversal.md
?? 00_nucleo/prompts/entities/syntax-node/errors.md
?? 00_nucleo/prompts/entities/syntax-node/linked.md
?? 00_nucleo/prompts/entities/syntax-node/representation.md
?? 00_nucleo/prompts/infra/export/builder/fonts.md
?? 00_nucleo/prompts/infra/export/builder/metadata.md
?? 00_nucleo/prompts/infra/export/builder/navigation.md
?? 00_nucleo/prompts/infra/export/builder/resources.md
?? 00_nucleo/prompts/infra/export/builder/serialization.md
?? 00_nucleo/prompts/infra/export/builder/streams.md
?? 00_nucleo/prompts/infra/export/html/document.md
?? 00_nucleo/prompts/infra/export/html/encoding.md
?? 00_nucleo/prompts/infra/export/html/inline.md
?? 00_nucleo/prompts/infra/export/html/p1360_ab_tests.md
?? 00_nucleo/prompts/infra/export/html/pretty.md
?? 00_nucleo/prompts/infra/export/html/table.md
?? 00_nucleo/prompts/infra/export/html/tags.md
?? 00_nucleo/prompts/infra/export/html/tests.md
?? 00_nucleo/prompts/infra/export/html/whitespace.md
?? 00_nucleo/prompts/infra/export/images/icc.md
?? 00_nucleo/prompts/infra/export/images/jpeg.md
?? 00_nucleo/prompts/infra/export/images/png.md
?? 00_nucleo/prompts/infra/export/images/scan.md
?? 00_nucleo/prompts/infra/export/images/xobjects.md
?? 00_nucleo/prompts/infra/export/stream/draw.md
?? 00_nucleo/prompts/infra/export/stream/page.md
?? 00_nucleo/prompts/infra/export/stream/shape.md
?? 00_nucleo/prompts/infra/export/stream/text.md
?? 00_nucleo/prompts/infra/export/svg/geometry.md
?? 00_nucleo/prompts/infra/export/svg/glyphs.md
?? 00_nucleo/prompts/infra/export/svg/groups.md
?? 00_nucleo/prompts/infra/export/svg/images.md
?? 00_nucleo/prompts/infra/export/svg/navigation.md
?? 00_nucleo/prompts/infra/export/svg/p1360_ab_tests.md
?? 00_nucleo/prompts/infra/export/svg/paints.md
?? 00_nucleo/prompts/infra/export/svg/serialization.md
?? 00_nucleo/prompts/infra/export/svg/tests.md
?? 00_nucleo/prompts/infra/font_metrics/basic.md
?? 00_nucleo/prompts/infra/font_metrics/caches.md
?? 00_nucleo/prompts/infra/font_metrics/fallback.md
?? 00_nucleo/prompts/infra/font_metrics/math.md
?? 00_nucleo/prompts/infra/font_metrics/resolution.md
?? 00_nucleo/prompts/infra/pipeline/bundle.md
?? 00_nucleo/prompts/infra/pipeline/html.md
?? 00_nucleo/prompts/infra/pipeline/paged.md
?? 00_nucleo/prompts/infra/pipeline/pdf.md
?? 00_nucleo/prompts/infra/pipeline/raster.md
?? 00_nucleo/prompts/infra/pipeline/resources.md
?? 00_nucleo/prompts/infra/pipeline/svg.md
?? 00_nucleo/prompts/infra/pipeline/tests.md
?? 00_nucleo/prompts/infra/plugin_host/execution.md
?? 00_nucleo/prompts/infra/plugin_host/loading.md
?? 00_nucleo/prompts/infra/plugin_host/snapshot.md
?? 00_nucleo/prompts/infra/plugin_host/tests.md
?? 00_nucleo/prompts/infra/plugin_host/typst_env.md
?? 00_nucleo/prompts/infra/shaper/bidi.md
?? 00_nucleo/prompts/infra/shaper/cache.md
?? 00_nucleo/prompts/infra/shaper/font_selection.md
?? 00_nucleo/prompts/infra/shaper/reconciliation.md
?? 00_nucleo/prompts/infra/shaper/shaping.md
?? 00_nucleo/prompts/infra/shaper/tests.md
?? 00_nucleo/prompts/infra/shaper/traversal.md
?? 00_nucleo/prompts/infra/system-world/cache.md
?? 00_nucleo/prompts/infra/system-world/fonts.md
?? 00_nucleo/prompts/infra/system-world/packages.md
?? 00_nucleo/prompts/infra/system-world/paths.md
?? 00_nucleo/prompts/infra/system-world/tests.md
?? 00_nucleo/prompts/infra/system-world/world_impl.md
?? 00_nucleo/prompts/wiring/compile.md
?? 00_nucleo/prompts/wiring/completions.md
?? 00_nucleo/prompts/wiring/diagnostics.md
?? 00_nucleo/prompts/wiring/eval.md
?? 00_nucleo/prompts/wiring/fonts.md
?? 00_nucleo/prompts/wiring/info.md
?? 00_nucleo/prompts/wiring/init.md
?? 00_nucleo/prompts/wiring/query.md
?? 00_nucleo/prompts/wiring/watch.md
?? 01_core/src/compiler/eval/observation/content.rs
?? 01_core/src/compiler/eval/observation/elements.rs
?? 01_core/src/compiler/eval/observation/geometry.rs
?? 01_core/src/compiler/eval/observation/math.rs
?? 01_core/src/compiler/eval/observation/mod.rs
?? 01_core/src/compiler/eval/observation/page.rs
?? 01_core/src/compiler/eval/observation/style.rs
?? 01_core/src/compiler/eval/observation/value.rs
?? 01_core/src/compiler/layout/align.rs
?? 01_core/src/compiler/layout/context_block.rs
?? 01_core/src/compiler/layout/counter_display.rs
?? 01_core/src/compiler/layout/counter_display_callback.rs
?? 01_core/src/compiler/layout/measurement.rs
?? 01_core/src/compiler/layout/outline_entry.rs
?? 01_core/src/compiler/layout/par_line_callbacks.rs
?? 01_core/src/compiler/layout/space.rs
?? 01_core/src/compiler/layout/state_display.rs
?? 01_core/src/compiler/layout/style_scope.rs
?? 01_core/src/compiler/parse/parser/checkpoints.rs
?? 01_core/src/compiler/parse/parser/diagnostics.rs
?? 01_core/src/compiler/parse/parser/state.rs
?? 01_core/src/compiler/stdlib/registry/contextual.rs
?? 01_core/src/compiler/stdlib/registry/data.rs
?? 01_core/src/compiler/stdlib/registry/diagnostics.rs
?? 01_core/src/compiler/stdlib/registry/figure_image.rs
?? 01_core/src/compiler/stdlib/registry/formats.rs
?? 01_core/src/compiler/stdlib/registry/foundations.rs
?? 01_core/src/compiler/stdlib/registry/layout.rs
?? 01_core/src/compiler/stdlib/registry/math.rs
?? 01_core/src/compiler/stdlib/registry/microbenchmarks.rs
?? 01_core/src/compiler/stdlib/registry/mod.rs
?? 01_core/src/compiler/stdlib/registry/modules.rs
?? 01_core/src/compiler/stdlib/registry/shapes.rs
?? 01_core/src/compiler/stdlib/registry/structural.rs
?? 01_core/src/compiler/stdlib/registry/table_grid.rs
?? 01_core/src/compiler/stdlib/registry/tests.rs
?? 01_core/src/compiler/stdlib/registry/text.rs
?? 01_core/src/compiler/stdlib/tests/arguments.rs
?? 01_core/src/compiler/stdlib/tests/assertions.rs
?? 01_core/src/compiler/stdlib/tests/bibliography.rs
?? 01_core/src/compiler/stdlib/tests/calc.rs
?? 01_core/src/compiler/stdlib/tests/color.rs
?? 01_core/src/compiler/stdlib/tests/contextual.rs
?? 01_core/src/compiler/stdlib/tests/eval.rs
?? 01_core/src/compiler/stdlib/tests/figure_image.rs
?? 01_core/src/compiler/stdlib/tests/footnote.rs
?? 01_core/src/compiler/stdlib/tests/foundations.rs
?? 01_core/src/compiler/stdlib/tests/gradients.rs
?? 01_core/src/compiler/stdlib/tests/introspection.rs
?? 01_core/src/compiler/stdlib/tests/layout_containers.rs
?? 01_core/src/compiler/stdlib/tests/layout_context.rs
?? 01_core/src/compiler/stdlib/tests/layout_spacing.rs
?? 01_core/src/compiler/stdlib/tests/math.rs
?? 01_core/src/compiler/stdlib/tests/mod.rs
?? 01_core/src/compiler/stdlib/tests/shapes.rs
?? 01_core/src/compiler/stdlib/tests/stroke.rs
?? 01_core/src/compiler/stdlib/tests/support.rs
?? 01_core/src/compiler/stdlib/tests/symbols.rs
?? 01_core/src/compiler/stdlib/tests/table_grid.rs
?? 01_core/src/compiler/stdlib/tests/text.rs
?? 01_core/src/compiler/stdlib/tests/transforms.rs
?? 01_core/src/entities/content/construction.rs
?? 01_core/src/entities/content/emptiness.rs
?? 01_core/src/entities/content/equality.rs
?? 01_core/src/entities/content/fields.rs
?? 01_core/src/entities/content/identity.rs
?? 01_core/src/entities/content/morphology.rs
?? 01_core/src/entities/content/page_structure.rs
?? 01_core/src/entities/content/plain_text.rs
?? 01_core/src/entities/content/rust_hash.rs
?? 01_core/src/entities/content/sequence.rs
?? 01_core/src/entities/content/tests/mod.rs
?? 01_core/src/entities/content/tests/morphology.rs
?? 01_core/src/entities/content/tests/projections.rs
?? 01_core/src/entities/content/tests/representation.rs
?? 01_core/src/entities/content/tests/sequence.rs
?? 01_core/src/entities/content/tests/traversal.rs
?? 01_core/src/entities/content/traversal.rs
?? 01_core/src/entities/syntax_node/errors.rs
?? 01_core/src/entities/syntax_node/linked.rs
?? 01_core/src/entities/syntax_node/representation.rs
?? 03_infra/src/export/builder/fonts.rs
?? 03_infra/src/export/builder/metadata.rs
?? 03_infra/src/export/builder/navigation.rs
?? 03_infra/src/export/builder/resources.rs
?? 03_infra/src/export/builder/serialization.rs
?? 03_infra/src/export/builder/streams.rs
?? 03_infra/src/export/html/document.rs
?? 03_infra/src/export/html/encoding.rs
?? 03_infra/src/export/html/inline.rs
?? 03_infra/src/export/html/p1360_ab_tests.rs
?? 03_infra/src/export/html/pretty.rs
?? 03_infra/src/export/html/table.rs
?? 03_infra/src/export/html/tags.rs
?? 03_infra/src/export/html/tests.rs
?? 03_infra/src/export/html/whitespace.rs
?? 03_infra/src/export/images/icc.rs
?? 03_infra/src/export/images/jpeg.rs
?? 03_infra/src/export/images/png.rs
?? 03_infra/src/export/images/scan.rs
?? 03_infra/src/export/images/xobjects.rs
?? 03_infra/src/export/stream/draw.rs
?? 03_infra/src/export/stream/page.rs
?? 03_infra/src/export/stream/shape.rs
?? 03_infra/src/export/stream/text.rs
?? 03_infra/src/export/svg/geometry.rs
?? 03_infra/src/export/svg/glyphs.rs
?? 03_infra/src/export/svg/groups.rs
?? 03_infra/src/export/svg/images.rs
?? 03_infra/src/export/svg/navigation.rs
?? 03_infra/src/export/svg/p1360_ab_tests.rs
?? 03_infra/src/export/svg/paints.rs
?? 03_infra/src/export/svg/serialization.rs
?? 03_infra/src/export/svg/tests.rs
?? 03_infra/src/font_metrics/basic.rs
?? 03_infra/src/font_metrics/caches.rs
?? 03_infra/src/font_metrics/fallback.rs
?? 03_infra/src/font_metrics/math.rs
?? 03_infra/src/font_metrics/resolution.rs
?? 03_infra/src/pipeline/bundle.rs
?? 03_infra/src/pipeline/html.rs
?? 03_infra/src/pipeline/paged.rs
?? 03_infra/src/pipeline/pdf.rs
?? 03_infra/src/pipeline/raster.rs
?? 03_infra/src/pipeline/resources.rs
?? 03_infra/src/pipeline/svg.rs
?? 03_infra/src/pipeline/tests.rs
?? 03_infra/src/plugin_host/execution.rs
?? 03_infra/src/plugin_host/loading.rs
?? 03_infra/src/plugin_host/snapshot.rs
?? 03_infra/src/plugin_host/tests.rs
?? 03_infra/src/plugin_host/typst_env.rs
?? 03_infra/src/shaper/bidi.rs
?? 03_infra/src/shaper/cache.rs
?? 03_infra/src/shaper/font_selection.rs
?? 03_infra/src/shaper/reconciliation.rs
?? 03_infra/src/shaper/shaping.rs
?? 03_infra/src/shaper/tests.rs
?? 03_infra/src/shaper/traversal.rs
?? 03_infra/src/world/cache.rs
?? 03_infra/src/world/fonts.rs
?? 03_infra/src/world/packages.rs
?? 03_infra/src/world/paths.rs
?? 03_infra/src/world/tests.rs
?? 03_infra/src/world/world_impl.rs
?? 04_wiring/src/compile.rs
?? 04_wiring/src/completions.rs
?? 04_wiring/src/diagnostics.rs
?? 04_wiring/src/eval.rs
?? 04_wiring/src/fonts.rs
?? 04_wiring/src/info.rs
?? 04_wiring/src/init.rs
?? 04_wiring/src/query.rs
?? 04_wiring/src/watch.rs
?? benches/README.md
?? benches/build.rs
?? benches/content/construction.rs
?? benches/content/coverage.rs
?? benches/content/fixtures.rs
?? benches/content/identity.rs
?? benches/content/morphology.rs
?? benches/content/page_structure.rs
?? benches/content/projections.rs
?? benches/content/representation.rs
?? benches/content/sequence.rs
?? benches/content/traversal.rs
?? benches/content_bench.rs
?? benches/layout/fixtures.rs
?? benches/layout/measurement.rs
?? benches/layout/style_scope.rs
?? benches/layout_bench.rs
?? benches/observation_bench.rs
?? benches/pdf_builder_bench.rs
?? benches/pipeline_bench.rs
?? benches/run_core_benchmark.py
?? benches/stdlib_bench.rs
?? benches/stream_bench.rs
?? benches/support/core_runner.rs
?? benches/support/profile.rs
?? benches/tests/test_benchmark_guards.py
```

</details>
