# PDF memory and executable size — batch G, 2026-10-23

Status: accepted release recipe and complete validation. Existing work is preserved.
This diagnostic records evidence; the owning specification is
[the binary L0](../prompts/wiring/main.md).

## Exact state and decision

HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, branch `codex/tekt-mature-layout`, **uncommitted working tree**.
The received inputs match F's validated state. The final source/L0/config map has
2162 files and differs only in `Cargo.toml`,
`00_nucleo/prompts/wiring/main.md` and the lineage header of
`04_wiring/src/main.rs`; executable Rust logic is unchanged. The precise dirty
`git diff HEAD --stat`, status, hashes and changed-file list are in
[validated evidence](../../lab/results/pdf-resources-2026-10-23/validated-evidence.json).
The [three-file patch](../../lab/results/pdf-resources-2026-10-23/final-change-vs-dirty-baseline.patch)
is relative to the received dirty tree, not HEAD.

Source-bound release builds and full source archives are retained under
[raw evidence](../../lab/results/pdf-resources-2026-10-23/).
The final build ran 2026-10-23T22:05:54.494204+00:00 through 2026-10-23T22:10:12.984055+00:00.
Accepted binary SHA-256: `e83cb46d8e00bbaa5492af8caed82ddaa8efde275538d6d7abe3ba3df513976b`. It is byte-identical to
the same-source recipe control. Oracle SHA-256:
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`, commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`. No moving target/resync was used.

The received workspace lacked release LTO/codegen overrides. The pinned reference
uses ThinLTO and one codegen unit. A same-source experiment qualified output
before allocation profiling and showed lower size/RSS with those two settings.
The accepted workspace recipe is `lto = "thin"`, `codegen-units = 1`; existing
optimization level, symbols, features, dependencies, output defaults and public
contracts stay. L0 was updated/sealed first; ADR-0127 continuous flow applies to
this internal build optimization. No release was published.

## Size and RSS

MiB below means bytes/1,048,576. Both executable copies receive the same
`strip --strip-all` normalization; original executables retain their bytes.

| Executable | Original MiB | Normalized MiB |
|---|---:|---:|
| Received F | 61.84 | 52.92 |
| Same-source recipe control | 52.13 | 47.69 |
| Accepted final | 52.13 | 47.69 |
| Pinned oracle | 49.85 | 49.85 |

The accepted raw size falls 15.70%
and normalized size falls 9.88%.
The normalized-size artifact test is RED for the received binary and GREEN for
the accepted binary against the unchanged pinned oracle cap. This is an artifact
cost criterion, not a language-parity definition. Production symbols remain.
The same-source section receipt shows `.text` 29,869,093→26,750,021 bytes and
`.eh_frame` 2,661,932→1,735,580 bytes; build recipe matters, and these numbers do
not establish an architectural advantage.

The paired window was 2026-10-23T22:10:54.174235+00:00 through 2026-10-23T22:11:30.248964+00:00:
three rounds × three fresh processes per binary/workload, two warmups each,
randomized within each replicate, default affinity and no competing build.
Uninstrumented `/usr/bin/time` peak RSS is reported as the median in MiB.
All round-median variations meet the recorded 3% repeatability condition.
Individual samples, commands, environment, worker/input/binary hashes and controls
remain in the [paired receipt](../../lab/results/pdf-resources-2026-10-23/paired-final/receipt.json).

| PDF | Received F | Recipe control | Accepted final | Oracle | F→final reduction |
|---|---:|---:|---:|---:|---:|
| empty | 30.83 | 27.10 | 27.18 | 26.76 | 11.85% |
| hello | 37.61 | 33.37 | 33.45 | 30.52 | 11.06% |
| prose | 42.33 | 38.28 | 38.21 | 33.91 | 9.72% |
| math | 41.12 | 37.57 | 37.68 | 33.30 | 8.36% |
| table | 39.34 | 36.12 | 36.21 | 32.71 | 7.96% |
| multipage | 38.04 | 34.05 | 34.08 | 35.59 | 10.41% |
| context | 38.63 | 34.69 | 34.64 | 34.16 | 10.32% |

All seven measured PDF workloads improve against the received release. RSS is
still higher than the oracle on six of seven controls; no leak fix, universal memory
advantage, watch/cache benefit or runtime-speed claim is made. Recipe and final
are identical executables, so their small RSS differences are measurement
variation, not a loader gain.

## Measured and rejected reader prototypes

Source-bound allocation stacks localized large requests to font-catalog metadata
projection and full-file reads used solely for a collection count. Requested
allocation bytes are neither live heap nor peak RSS. The explicit World preload
has no CLI caller; it was not optimized or credited. The previously ratified
single-font copying decision was preserved.

Two L0-first prototypes read table payloads directly into reserved final storage
and read only the twelve-byte collection header. Two actual runtime RED tests
then passed GREEN; 85 focused tests passed with one existing ignored test.
The first test compilation's missing type annotation is retained separately and
receives no RED credit. Despite lower allocation requests, the same-recipe
package demonstrated no additional CLI peak-RSS reduction; paired math/table
samples even showed increases. The
[rejection receipt](../../lab/results/pdf-resources-2026-10-23/reader-prototypes-rejected.json)
keeps every sample, bootstrap method, frozen source and decision.

Both reader consumers and both owning L0 files were restored **exactly** to the
received state. Their experimental tests are archived, not shipped or counted
as retained regressions. The simpler, better measured recipe is the accepted
change. No new cache, mutable global, L1 I/O or public API was introduced.

## Output and validation gates

- All seven original PDFs pass the unchanged complete per-page word sequence and
  page/word-box gate, tolerance 0.001pt; maximum observed delta is 0.000020pt.
- All 40 A/C/D/F preservation controls retain pages, lines, words and geometry;
  all also pass the pinned-oracle geometry gate.
- All 210 E symbol observations retain the received raw outcomes. Existing E
  extension/field/render limits remain recorded; this is not 210 new parity claims.
- Three complete font inventories and all 1853 hashed font/reference
  assets remain unchanged. No snapshot update or relaxed assertion was needed.
- All pages of seven controls at72dpi and four first-page controls at600dpi retain
  zero changed pixels against the received release. Table's first-page oracle
  mismatch remains 1,548 pixels at600dpi before and after, the separate F border
  painting boundary, not a passing oracle raster gate.
- All 12 frozen workspace gates pass: Python CI tests, layout, strict and reverse
  lineage, build, domain/infra tests, wiring tests, doctests, all-target build,
  Clippy (`-D warnings`, `-D unfulfilled_lint_expectations`), fmt and Rustdoc
  (`-D warnings`). Rust totals: 9,849 passed, zero failed,
  7 existing ignored tests; no experimental tests receive final credit.

Heavy Cargo jobs were sequential, offline/locked, one job, configured 12GiB max,
8GiB high and zero swap. The final release monitor ended with143 while its service
remained active; Cargo subsequently completed with exit0 and unchanged inputs.
That transport interruption is retained separately, not described as a clean
initial monitor run. Persistent logs were used for final observations/validation.
Full `crystalline-lint .`, final diff checks and closure hashes are recorded in
the completion receipt. Batch H remains open.
