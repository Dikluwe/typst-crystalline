# PDF memory and executable size — R6, 2026-10-24

Status: bounded resource reduction accepted; complete source-bound validation and
A–H/R1–R5 preservation. This is a diagnostic, not an L0 specification.
Owners: `00_nucleo/prompts/wiring/main.md` and
`00_nucleo/prompts/core/entities/font_book.md`.

## Exact state and accepted change

HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, branch `codex/tekt-mature-layout`, **uncommitted working tree**.
The [received state](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/baseline.json), full source archive and immutable R5
binary copies bind the baseline to R5's completion. The [final build](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/final-build/receipt.json)
freezes all 2162 source/config/L0 inputs, exact dirty `git diff HEAD --stat`,
status, UTC, compiler and recipe. Only Cargo.toml and the two owning L0/consumer pairs differ from received R5. The [closure](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/completion.json)
seals the exact change list/patch, source-change timestamps and all evidence.
Coverage stores its immutable canonical runs in private `Arc<[u32]>` backing;
clone shares them instead of copying. Public metadata/selection contracts remain.
One genuine runtime RED→GREEN resource regression is retained.

Accepted recipe: `opt-level = "s"`, ThinLTO and one codegen unit. Symbols,
features, dependencies, CLI modes and compiler phases remain. Internal reversible
compiler optimization uses ADR-0127 continuous flow; L0 and ownership/Nucleus
validation preceded each experiment. Original product symbols are retained;
normalization strips disposable copies of both product and oracle only.

Final release SHA-256 `6ccf6b10b1dfb7d4443dda2ce9dd4d82837264e8fec2fdd1aec7762ba5f61d86`; debug `0ff7d7dc689a71f5caea518f7fc6f8a676251fc1112f070f69ba4e6a60c9caaa`.
Oracle remains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Actual executable/source/recipe bindings are in the
[binding receipt](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/artifact-source-bindings.json); observation-time host maps
are never substituted for an earlier immutable binary's actual build inputs.
Final build interval: 2026-10-24T12:13:54.686270+00:00 through 2026-10-24T12:13:56.684098+00:00.

## Resource tests and paired measurements

The independent [resource test](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/resources.py) was fixed in L0 before the
experiments: normalized size <=95% received and <=pinned oracle; geometric mean
of seven peak-RSS median ratios <=97%; no workload increase above 3%; before/current
round-median spread <=3% on every workload. These are resource criteria, not
language-parity definitions. The [received artifact is actual RED](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/red-resources/receipt.json);
the [accepted final artifact is GREEN](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/final-resources/receipt.json).
The thresholds and original language tolerances were not relaxed. Selection also
requires math elapsed time below the validated z+shared control.

| Role | Original bytes | Normalized bytes |
|---|---:|---:|
| before | 54,693,720 | 50,032,048 |
| current | 50,613,888 | 42,700,720 |
| vanilla | 52,271,128 | 52,271,120 |
| recipe_only | 50,612,416 | 42,700,248 |
| z_recipe_only | 50,867,080 | 40,680,016 |
| z_shared | 50,867,344 | 40,680,232 |

Original executable reduction: 7.46%.
Normalized reduction: 14.65%. Geometric mean RSS reduction:
5.22% in this local window. MiB = 1,048,576 bytes.

| PDF | Received MiB | Final MiB | Oracle MiB | RSS reduction |
|---|---:|---:|---:|---:|
| empty | 27.29 | 26.05 | 26.62 | 4.51% |
| hello | 31.35 | 29.55 | 30.69 | 5.74% |
| prose | 35.91 | 36.29 | 34.09 | -1.06% |
| math | 37.82 | 34.66 | 33.66 | 8.36% |
| table | 34.06 | 30.92 | 32.86 | 9.22% |
| multipage | 31.74 | 30.24 | 35.03 | 4.73% |
| context | 32.60 | 31.07 | 34.31 | 4.67% |

Window 2026-10-24T12:17:51.802600+00:00 through 2026-10-24T12:18:59.588404+00:00: three rounds × three randomized
fresh processes per role/workload, two warmups, GNU time maxRSS, no competing
build. All samples/commands, fixture hashes, source/binary/worker hashes and
elapsed times remain in the receipt. Recipe-only "s" is retained as a same-window
control. Allocation requests, mapped sections and peak RSS are separate measures;
smaller .text is an observed mechanism candidate, not a universal causal claim.
The same-recipe s control yields a current/recipe-only geometric RSS ratio
0.96608, supporting the local benefit of shared coverage separately
from recipe. Timing medians are short-process observations rounded by GNU time;
some costs remain higher than received R5. The accepted s recipe has lower math
elapsed cost than the fully validated z+shared alternative, a separately tested
selection criterion. No runtime-speed or universal memory superiority is claimed.

| PDF | Received seconds | Final s seconds | z+shared seconds |
|---|---:|---:|---:|
| empty | 0.10 | 0.10 | 0.14 |
| hello | 0.10 | 0.10 | 0.14 |
| prose | 0.12 | 0.13 | 0.18 |
| math | 0.13 | 0.17 | 0.25 |
| table | 0.11 | 0.11 | 0.16 |
| multipage | 0.10 | 0.11 | 0.15 |
| context | 0.10 | 0.11 | 0.15 |

## Rejected experiments and historical guard

The ["s" recipe-only control](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/candidate-resources/receipt.json) reduces
normalized bytes but fails the predeclared 3% aggregate RSS gate; its multipage
received round also exceeds 3% variation. The [single-font transfer refinement](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/candidate2-resources/receipt.json)
still fails aggregate RSS and gives no demonstrated additional aggregate benefit.
Its new buffer test genuinely fails then passes in the archived source states,
but neither that test nor the experimental algorithm is shipped or counted in
final tests. The [rejection receipt](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/single-transfer-rejected.json) restores
both pipeline/pdf source and owner exactly to the received state. Prior Oct10
single-font amplification observations remain historical evidence.

The [validated z+shared alternative](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/z-cycle/audit.json) passes all full
language/resource gates, but is superseded by the lower-CPU-cost passing s+shared
variant. Its entire cycle is retained verbatim; the
[relocation manifest](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/z-cycle-relocation.json) resolves earlier receipt paths
and hashes, and observer revisions match their original recorded SHA-256 values.

The [recipe-only z control](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/candidate3-resources/receipt.json) also fails
aggregate RSS (ratio 0.97259). The accepted shared-coverage candidate retains
that control in the same final measurement window, in addition to the s recipe.
Public constructors, exact membership/order, canonical encoding, equality/hash,
font metadata and selection remain covered by independent existing tests.

[Historical stress replays](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/legacy-resources/receipt.json) cover local-set-400,
local-set-1600 and variants-1600 with nine fresh processes/role in three rounds,
current source-bound binaries, exact fixture hashes and complete before/current
word/page geometry at 0.001 pt. Their RSS and elapsed tradeoffs remain explicit,
without relabeling the old measurement or inferring a global performance claim.

| Stress control | RSS final/received | Received seconds | Final seconds |
|---|---:|---:|---:|
| local-set-400 | 0.9558 | 0.11 | 0.12 |
| local-set-1600 | 0.9618 | 0.17 | 0.18 |
| variants-1600 | 0.9940 | 0.56 | 0.74 |

## Preservation and complete validation

All 12 frozen full-validation gates pass: Python CI, layout, strict/reverse lineage,
build, domain/infra tests, wiring tests, doctests, all-target build, Clippy with
`-D warnings -D unfulfilled_lint_expectations`, fmt and Rustdoc with `-D warnings`
and four indexes. Rust: 9878 passed, zero failed, 7 existing ignored.
All final orchestration stages, full crystalline-lint and diff checks pass.
Heavy Cargo is serial, offline/locked, Rust 1.92, one job, effective MemoryMax 12 GiB,
MemoryHigh 8 GiB and MemorySwapMax 0. Resource receipts retain the effective caps.
Candidate3 and candidate4 dispatch automatic-review timeouts recovered on their permitted retry;
neither was a compiler/resource-test result and no process ran in either first attempt.

Three complete font inventories preserve received stdout and diagnostics in
debug and release. The [audit](../../lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/audit.json) verifies all 1853 existing assets and every received
input outside the five owned files. Replays preserve A 60 complete page/word
controls, B init, C/D/F 40 strict geometry cases, E 210 raw observations, H 98 stdout
controls, R1 markers, R2 continuation/boundaries, R3 all 47 original/additional
render controls and R4 all 210 reflection/conversion controls plus 14 rendering
comparisons. R5 PDF/SVG painting at 72/600 dpi remains identical to the accepted R5
state in debug/release/compact, including all residual observations; four prepared
gradient/tiling controls and refused inputs retain their recorded outcomes.
Native PNG's 11 pixels/max delta 1 and font-name/encoding PUBLIC_FORMAT difference
remain recorded differences. No comparator, corpus or tolerance was weakened.

## Remaining scope

RSS remains workload/platform dependent. Medians above pinned vanilla in this
window: prose, math. Cases without a median RSS reduction versus received R5:
prose; their small changes stay within the fixed preservation gate.
See the per-case table rather than a universal superiority claim. This bounded R6 reduction does not
close font-catalog allocation costs, leak/watch/cache questions, unmeasured
resource workloads or runtime-speed tradeoffs. Existing R1–R5 language/render
limits and R7's stale legacy-symbol carrier policy remain open exactly as in R5.
