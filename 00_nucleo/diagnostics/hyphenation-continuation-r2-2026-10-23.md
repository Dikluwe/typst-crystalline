# Explicit-hyphenation prose continuation — R2 — 2026-10-23

The two reported R2 cases now match pinned vanilla in complete text, physical
line/page membership and every PDF bounding-box coordinate at the unchanged
0.001 pt limit. The correction admits ASCII colon in the existing terminal
punctuation certificate of a uniform hyphenated word. Existing global selection
and widow/orphan admission then operate normally; their algorithms and costs
are unchanged. Previously accepted A–H and R1 controls remain preserved.

## State, provenance and L0 gate

- HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **uncommitted working tree**. The
  [baseline](../../lab/results/hyphenation-continuation-r2-2026-10-23/baseline.json)
  records the full dirty stat/status, UTC and exact received source/config/L0 map;
  [actual source texts](../../lab/results/hyphenation-continuation-r2-2026-10-23/baseline-source.json.gz)
  include untracked productive files. The map exactly matches R1's completion.
- Only `01_core/src/compiler/layout/paragraph_hyphen.rs` and its exclusive owner,
  `00_nucleo/prompts/core/compiler/layout/paragraph_hyphen.md`, differ from that
  received productive/config/L0 state. No existing change was reset or committed.
  [Audit](../../lab/results/hyphenation-continuation-r2-2026-10-23/audit.json)
  retains every changed input; all 1853 font/reference assets are unchanged.
- Final release: `2026-10-24T01:12:21.641268+00:00` through `2026-10-24T01:19:03.493001+00:00`, SHA-256
  `2f2a9d4ebd4b6bf0d1a8abf595ce7a8d8964fa8845ca186f4365fa38bbc1616b`. Its
  [build receipt](../../lab/results/hyphenation-continuation-r2-2026-10-23/final-build/receipt.json)
  and [source archive](../../lab/results/hyphenation-continuation-r2-2026-10-23/final-build/source.json.gz)
  bind the exact dirty state, recipe, toolchain and sources. Frozen inputs do not
  change during validation, build or measurement. Later diagnostic edits have
  their own completion receipt and do not alter productive inputs.
- Fixed oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
  `cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
  `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. No target update or version-string substitution.
- Heavy jobs ran serially, one Cargo job, MemoryMax 12 GiB, MemoryHigh 8 GiB,
  zero swap. Commands, workers, source/fixture/binary hashes and UTC remain in
  the evidence. Measurements below refer to the source-bound final release.

## Measurement before decision

[Fresh before probes](../../lab/results/hyphenation-continuation-r2-2026-10-23/before/receipt.json)
reproduce both debts on frozen R1. Replacing only `Paragraph:` with `Paragraph.`
qualifies complete text and all PDF coordinates; bold and nonhyphenated lexical
controls also qualify. The initial `paragraph_hyphen.rs:40–56` rejects colon in
the suffix after its alphabetic prefix. `paragraph_prepare.rs:256` propagates
that refusal to the complete paragraph, bypassing global hyphen candidates and
the already-certified regional grouping. Pinned `inline/linebreak.rs:780–786`
derives hyphenation from alphabetic segments while retaining punctuation.

Classification: language-visible words, discretionary-hyphen identity and
continuation require parity; private candidate storage and implementation steps
are mechanics. ADR-0127 continuous flow applies: L0 was updated and sealed first;
no public entity, trait, signature, mode/default path, compatibility or pipeline
phase changed. Production adds only `:` to the existing suffix predicate.
It retains punctuation in the final measured slice, computes character-edge
penalties only from the alphabetic prefix, and introduces no cut at colon.
No page compensation, hardcoded fixture, export substitution or tolerance change.

## RED→GREEN and full validation

[Actual RED](../../lab/results/hyphenation-continuation-r2-2026-10-23/red-tests/receipt.json):
exit 101, two behavioral failures and one passing refusal control. Candidate
preparation refused a colon-terminated Unicode word; source-to-layout admission
gave 3+1 lines instead of 2+2. The zero-cost control retained 3+1.
[GREEN](../../lab/results/hyphenation-continuation-r2-2026-10-23/green-tests/receipt.json)
passes all three tests. Tests retain UTF-8-safe cuts, unchanged edge counts,
nonadditive slice measurement, exact reassembled source punctuation, chosen
U+00AD replacement identity, Simple/Optimized and absent/explicit-zero tracking.
Unsupported internal/numeric/standalone words and mixed policies stay refused.
Full lint passed before the expensive validation; no suppression or fixture
refresh was required.

Final [validation](../../lab/results/hyphenation-continuation-r2-2026-10-23/validation-complete/validation-commands.json):
**12/12 gates**, **9861 Rust tests passed, 7 ignored**,
34 Python CI tests passed. Workspace build/tests, wiring, doctests, all targets,
strict/reverse lineage, Clippy with warnings denied, formatting, Rustdoc with
warnings denied, full crystalline-lint and diff check pass. Final source is
unchanged across those gates and all runtime observations.

## Bilateral results and preservation

The [final PDF corpus](../../lab/results/hyphenation-continuation-r2-2026-10-23/after/receipt.json)
contains 24 inputs, with raw words including discretionary hyphens; no text or
coordinate is normalized away. Source-bound [debug controls](../../lab/results/hyphenation-continuation-r2-2026-10-23/after-debug/receipt.json)
qualify against the release at the same 0.001 pt limit.

| Original R2 input | Before pages | Current pages | Vanilla pages | Maximum bbox delta pt |
|---|---:|---:|---:|---:|
| hyphenation | 6 | 5 | 5 | 5.999999999062311e-06 |
| zero-tracking-hyphenation | 6 | 5 | 5 | 5.999999999062311e-06 |

Complete line/page words match in 21 of 24 inputs. Full bbox geometry qualifies
in 17: both originals, absent/zero tracking,
Simple/Optimized without justification, Unicode/adjacent fragments, period/bold
refuters, false/auto lexical policy and four widow-cost reductions. Four justified
variants match continuation but retain the separate optical-margin gap below;
three deliberately unsupported word contexts retain the existing divergence.

[Preservation replays](../../lab/results/hyphenation-continuation-r2-2026-10-23/replays.json):
60 A complete page/word matches in release/debug/oracle; 28 B init observations;
40 C/D/F full line/page words and all bbox coordinates at 0.001 pt; 210 E raw
outcomes preserved including existing errors/extensions; 98 H stdout controls;
all ten R1 cases preserve words/page membership and bounds. The old legacy-symbol
checker remains the previously recorded policy debt, not a new all-green raw
comparison. Other prior rendering and reflection limits are not recertified away.

## Explicit remaining limits and refutation controls

The four justified variants retain up to 2.797267 pt of bbox difference,
despite matching full line/page words. This is **not** claimed as a new geometry
success. The [14 boundary probes](../../lab/results/hyphenation-continuation-r2-2026-10-23/boundary/receipt.json)
show the same mismatch before/after for period-only justified controls, with
preserved before/current bounds at 0.001 pt. Disabling `text.overhang` qualifies both those
period controls and the newly admitted colon controls. Pinned
`inline/line.rs:677–715` applies 0.8 of the glyph advance to period/comma and
0.3 to colon/semicolon. Current `paragraph_hyphen.rs` transports only the already
specified chosen-hyphen optical margin; other punctuation was already an explicit
L0 scope-out. These source points and refuters support a pre-existing punctuation
overhang limit, independently of this preparation fix.

`internal`, `numeric` and `standalone` colon cases still take the refused path;
they preserve before/current language and geometry but fail bilateral continuation.
Nonzero tracking ±0.2 pt likewise preserves its pre-existing fallback and mismatch.
These are not successful parity controls. Arbitrary UAX14, mixed intra-word
styles, shaping-safe nonzero-tracking hyphen cuts and broader regional contexts
remain outside this correction. R1 note geometry/layout and R3–R7 retain their
recorded open boundaries.

## Resource preservation

Fresh [RSS measurements](../../lab/results/hyphenation-continuation-r2-2026-10-23/memory/receipt.json)
use three randomized rounds × three fresh processes per role/workload, two
warmups and GNU time maxRSS, without competing builds. All seven meet the
unchanged 3% round-repeatability bound and current/before-R2 median cap of 1.03.
All seven original PDF output geometries qualify at 0.001 pt. This is retention
of accepted G, not a new causal memory or latency improvement claim.

| PDF | Before R2 KiB | Current KiB | Vanilla KiB | Preservation |
|---|---:|---:|---:|---|
| empty | 27688 | 27896 | 27044 | pass |
| hello | 34364 | 34324 | 30964 | pass |
| prose | 39012 | 38960 | 34708 | pass |
| math | 38236 | 38280 | 34112 | pass |
| table | 36740 | 36800 | 33596 | pass |
| multipage | 34776 | 34876 | 36112 | pass |
| context | 35096 | 35312 | 34816 | pass |

Normalized executable: 50009520 bytes versus pinned oracle
52271120, passing the original G cap. Only disposable copies were stripped.
