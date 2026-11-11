# L0 — Bounded selective acquisition of font catalog metadata
Hash do Código: fddc2a6e

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fontdb/selective.rs`
**State:** ratified canonical owner.

## Measurement before decision

`lab/results/construction/diagnostics/catalogo-seletivo-2026-10-05.md` records the rejected full-post variant and measured prefix experiment (metadata38.10→29.00ms, old coverage94.33→88.04ms), with primary parser anchors and full provenance. No current CLI gain follows from those numbers. `lab/results/construction/diagnostics/catalogo-seletivo-preparacao-2026-10-07/audit.json` records 1, 140 faces/max20 sfnt records/max259, 927 projected bytes and259, 959 requested bytes excluding ≤16 extra TTC header/index bytes, all historic1, 086 assets unchanged and metadata extractor body unchanged. HEAD/dirty bytes/UTC and asset SHA accompany every count. Existing interval coverage remains current, so combination must be freshly measured.

ttf-parser0.25.1 lib.rs:1102–1111,1230–1233 requires head/hhea/maxp; 1433–1482 style/weight/width read OS2, monospace post. post.rs:334–379 validates32-byte header/version and34+2×count for v2, not glyph-name tail. `03_infra/src/fonts.rs:339–405` consults names/OS2/post and delegates coverage to its current owner. fontdb0.23.0 lib.rs:721–727,900–920 exposes fresh File reads but retained SharedFile bytes; helper must never substitute a path read for SharedFile's memory.

`00_nucleo/diagnostics/read-at-pipeline-2026-11-04.md` records the frozen working-tree experiment: 70 pairs preserved extracted text, page geometry and font names; the positional backend removed 11,180 physical seeks without reducing metadata observations. CPU decreased in 35/35 CLI pairs and 34/35 watch-startup pairs, with dispersion exceeding 3% in four of ten groups. These observations support testing an internal backend substitution, not a universal precise speedup or a change to warm-watch freshness.

## Classification and decision

Observable selectable font metadata, exact coverage, family/fallback and rendering preserve language behavior (ADR0107/0108). Buffer/table order, bytes read, mapping and DTO equality are mechanism controls. No claim of upstream intent. Hypothesis of lower acquisition cost is refuted by semantic/corpus divergences or absence of repeatable combined benefit. This helper is not a final-font reader, parser replacement, discovery owner or cache. The default acquisition path requires ADR-0127 §2.2/§5 package ratification; the approval recorded below satisfies that prerequisite.

## Private contract

Own a private `acquire(path:&Path,index:u32)->Option<FontInfo>` facade accessible to its parent only. None always declines optimization, never instructs dropping a font; parent falls back under `00_nucleo/prompts/infra/fontdb.md`. Accept only regular Source::File inputs provided by parent; Unix identity required; unsupported platforms decline. No new public types/APIs, unsafe, mutable globals, feature flags or I/O in L1.

Open one File and capture its metadata. For eligible Unix files, use a private positional backend over `FileExt::read_at`, exposing logical Read+Seek/read_exact to the unchanged generic projector. Read advances the logical cursor by the actual returned length only; errors, including Interrupted, leave it unchanged. read_exact retries Interrupted, completes short reads and rejects premature EOF or other errors. Empty reads do not call the backend. Cursor addition and signed seeks are checked; rejected seeks preserve the cursor. SeekFrom::End uses the captured size, and the projector uses Start only; do not generalize this adapter to readers requiring live external size. Independent readers must not share a logical cursor or modify the File's physical cursor. No backend return may exceed the requested slice length.

Read sfnt/TTC with checked arithmetic. Supported sfnt signatures are0x00010000/OTTO/true and TTC v1/v2; original face index/array/directory/table offsets must be valid. Collection offsets are absolute to original bytes. Unsupported signatures/layouts/indices decline; do not fabricate validity or broaden accepted input. Keep the private generic reader for deterministic fault/cap tests without exposing it publicly. Do not retain files, projected bytes or identities across acquisitions, add cross-face sharing, or change the existing fallback, metadata/coverage derivation or placement of the post-derivation identity guards.

Budget before every operation: at most4, 096 sfnt records, 8MiB complete projected buffer and16MiB cumulative requested bytes (including headers, directory, prefixes/rereads). Validate offset+full declared length against actual original length. Bound all allocations and use try_reserve; any overflow, budget/error/reserve failure declines. No attacker-controlled full-file allocation or eager fallback introduced. These caps cause fallback, never truncation of needed metadata; audit demonstrates all measured faces within them but does not promise host/font universality.

Preserve complete sfnt directory order and duplicate records. Copy head, hhea, maxp, name, OS/2, cmap completely. Omitted records keep their positions with offset/length zero. For post, first validate full declared bounds; retain min(length,32) outside v2; v2 with count available retains min(length,34+2×count). Truncated version/header/count/index arrays remain invalid or cause decline; no fabricated version/pitch/indices. Typed ttf-parser determines validity. Rebase copied offsets checked into sfnt-at-zero buffer; if required parsing/metadata fails, decline. No glyph-name payload, outline/shape table or variable instantiation is derived from this buffer.

Use current `font_info_from_bytes(buffer,0)` and current `extract_coverage` on a typed Face of projected buffer; do not duplicate name decoding, exceptions, style inference, interval collection or normalization. Return owned neutral FontInfo only. Projected bytes/Face/handles must not escape to FontSlot, World, shaper/export, or caches. No metadata-only mode/flag: system catalog still requires exact eager coverage.

Before returning Some, verify handle and current path identify the initial regular file: dev/ino, len, mtime/mtime_nsec, ctime/ctime_nsec. Unsupported identity/metadata failure/path replacement/changed length or timestamps decline. This is best-effort change detection, not atomic snapshot guarantee under concurrent writers; unstable sources cannot certify measurements. Do not mutate files/metadata or retain handles across acquisitions. No SharedFile/Binary/embedded path substitution; no sorting/dedup; no altered final mmap/loading failure policy.

## Tests and acceptance

Positional transport tests cover short reads, repeated Interrupted, early EOF, other errors, empty buffers, excessive backend lengths, cursor/seek overflow and negative seeks, physical File cursor independence and independent logical readers. Mutation checks must reject incorrect offset, requested-length advancement, advancement on error, ineffective seeks, fabricated EOF success and loss of Interrupted retry. Revalidate complete CLI and watch startup against a frozen pre-change baseline with paired CPU/wall/RSS and text/page geometry/font controls; isolated projector timing does not establish product benefit. No warm-watch acceleration is implied.

Inline tests precede implementation and demonstrate runtime RED for valid projection/membership/limits/omitted payload; no compile-only RED or performance threshold. Reuse existing licensed fixtures and independent expected name/style/pitch/coverage controls, plus original metadata callback as an additional reference. Include TTF/OTF/variable, original TTC indices/absolute offsets, post valid/invalid versions/prefix/indices/tails, optional/required bounds, duplicate order, corrupt headers/counts/overflow, exact budget edges and early failure before excess allocation/read. Reader injection tests Read/Seek faults deterministically; identity predicate tests detect modified/replaced files. Parent integration tests prove fallback and retained SharedFile behavior, complete ordering and paired slot/book indices.

Final acceptance is language selection/fallback/metadatas and equivalent text/font/page/geometry/PDF/errors, not a demand for Rust or PDF byte equality with vanilla. Per-face equality/book/slots and raster against previous are additional controls. Measure combined acquisition with current interval coverage, full CLI and process RSS against frozen previous and the ratified vanilla oracle. The original package used historical pin `9f2b6e8715237cb086899a42873660fe744622e8`; its measurements do not certify the current resynced pin. Record source/HEAD/dirty/UTC/argv/asset/bin hashes and all samples, caps/fallback diagnostics outside productive APIs. Three rounds, quiet/monitor/stable hashes, repeatability<3% for precise performance claims. No gain promise from historical experiment; no loss of unsupported input through fast-path rejection. Scope-only seals; build/tests/lint without errors and owners without V5/V6/V15/V26; preserve received tree and negative results, no commit.

Approval: the owner confirmed `catalogo-seletivo-2026-10-07` with `confirmo` on 2026-10-07, manifesto SHA-256 `045ef18d7a16143e640669cf5923db1a213eb677b6c278d4ee4a0bf542973c2e`. This satisfies the earlier ratification prerequisite for this package; it grants no other scope.

## Single reservation of the projected buffer

### Evidence and authorization

The received `selective.rs:202/239` reserves output for the directory and again
for every selected table. The completed C23 comparison is retained at
`/home/dikluwe/.codex/visualizations/2026/11/06/01a1b22c-8a18-7c32-af10-ecc4c2846e4d/c23-single-reserve-lab/README.md`,
with the exact received dirty state, paired samples and independent tests.
It supports fewer requested bytes/reallocations, a small scenario-dependent CPU
benefit and no general reduction of heap peak or RSS. The owner explicitly
authorized integration after that comparison. This is an internal organization
change under ADR-0127; no public API, default, dependency or pipeline change.

### Organization and preserved boundaries

After the existing header/TTC/directory reads, construct a fallibly reserved plan
bounded by the validated record count. Retain selected records in original order,
including duplicates and zero-length records; omitted records retain their place
with zero offset/length and acquire no new payload validation. Store each selected
record's directory position, source offset and chosen length. Validate complete
declared bounds before shortening post. Read each existing post prefix once during
planning and retain its current shortening decision for emission.

Checked header + directory + selected lengths must fit MAX_BUFFER before reserving
output. Track the original logical prefix/payload budget during planning and reject
before the associated read/reservation if MAX_READ would be exceeded; keep Reader's
actual checks unchanged. Reuse the original header as output and call its
try_reserve_exact once for the final additional size. Append directory and payloads
without further output growth, retaining the existing per-table read buffers and
checked rebased offsets. Drop the plan before returning. Never reserve by full file
size, persist a plan or introduce cross-face sharing.

All earlier caps, signatures, indices, fallible failure paths, identity fields and
post-derivation guards remain. Stable inputs preserve projected bytes and requested
counts. Prefix reads now precede payload emission: this is read-order mechanics,
not a stronger snapshot guarantee. Failure remains conservative None; no change to
parent fallback, retained SharedFile bytes, Binary omission, exact eager system
coverage or final lazy font loading. Outline semantics are outside this change.

### Durable regression boundary

Keep existing acquisition/transport tests. A private cfg(test) generic
project_for_test forwarding facade may expose project only to its parent test
module; no such facade exists in normal builds. Declare cfg(test) child
single_reserve_tests, whose sole owner is
`00_nucleo/prompts/infra/fontdb/selective/single_reserve_tests.md`.
The separate allocation executable is owned by
`00_nucleo/prompts/infra/tests/projected_buffer_reserve.md`; its allocator does not
replace the library test executable's existing image allocator. Test sources must
use repository fixtures and temporary directories, never lab absolute paths.
Preserve semantic controls and demonstrate the single-reservation assertion RED
on the received implementation, GREEN after integration. Directed bounds/post/
offset/count mutations must be rejected. Build, focused regressions, output
qualification and strict lineage must pass, preserving all received changes.
