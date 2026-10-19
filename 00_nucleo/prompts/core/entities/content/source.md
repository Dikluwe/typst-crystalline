# L0 — entities/content/source
Hash do Código: b1627754

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/projection-exhaustiveness.toml sha256:c7269dd3cf9d8084731457973737db5df87a3d428b80ab5701593deaecdea87e
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/source.rs`

## Evidence and ratified ownership

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz`
preserves source/HEAD/UTC/tree/binaries: alias/spread lost content spans;
upstream foundations/content/mod.rs:109-114 assigns origin only when detached,
and model/table.rs:436-437 copies it to implicit Cell. Human `confirmo` on
2026-10-07 ratified SourceContent and Content::Source in contract C2.

Pure crate-private helpers assign first origin, unwrap only Source for
classification and read Source or existing Table/Cell spans. Never unwrap
Styled/ShowRecipe or execute content. Detached spans invent no origin and
reference spans never overwrite constructor origin. Casts, eval/layout,
identity/equality/morphology/traversal are owned elsewhere.

Acceptance requires alias/spread/explicit Cell/synthetic-origin RED→GREEN,
language/hash equivalence, clone/maps and exactly one traversal. Verify first
origin survives clone/map, body callback runs once, language equality/Hash
ignore origin while ObservationEq preserves causal distinction, and detached
content remains unwrapped. These transport tests add no API. Require builds,
suites/linter and preserved source without global resealing.

## Evidence before explicit Content projections

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b had an uncommitted tree.
`lab/results/construction/diagnostics/tekt-v16-content-views-2026-10-17/before.json.gz`
records UTC/status/diffstat/source/L0 and 2129 checked hashes. contexts-before.json
maps six V16 sites by AST/line; projection-plan.json records Content's120
variants and per-function complements. before-probe-result.json preserves
17280 symbolic pattern/guard controls: the future Content mutant compiles at
all six sites (zero selected E0004). This measures domain-protection mechanics,
not parity or AI readability. Opaque model payloads cannot replace real tests.

before-field-probe.json uses the current rlib/field-control.rs with hash/argv/
UTC/source freeze:48 of 84 reads differed for Source/Located/ShowRecipe/Realized
inside Styled or nested Styled. Outside Styled all four already preserved
fields. fields.rs:10-13 recognized only Equation/Styled and its guard rejected
carriers before delegation. source-origin already requires transparency and
the fields owner already requires body/visual delegation for the other three.
This is language field invariance, not upstream Rust structure: these internal
carriers have no own pinned syntax. No intent is inferred from the defect.
Refuters are lost precedence/absence/none, spurious non-equation fields,
consulting original instead of visual, or recipe execution. Historical language
pin remains9f2b6e8715237cb086899a42873660fe744622e8; this metamorphic measurement
is not a new bilateral comparison.

## Exclusive projection

source_span retains Source, TableCell and Table arms; the other 117 variants
explicitly return Span::detached. Do not change source_body, first-origin
assignment or span reach through other carriers.

Nominal complements follow declaration order and include variants not already
covered unconditionally; keep Styled if its guard can reject. Literal-field
or guarded arms retain unknown-field cases; variants delegated for every f
are excluded from the complement. String domains remain open. The pinned
projection-exhaustiveness nucleus shares this rule; ownership stays exclusive
and source-origin remains binding.

## Acceptance and limits for projections

L0 precedes tests/code; ADR-0127 continuous flow without new public field/
signature/trait, compatibility, default or phase. Require functional fields
RED, not a broken probe harness. After repair all 84 real carrier controls
agree, including style precedence and non-equations; Realized reads visual,
not original. Adding a variant only in the disposable structural model must
change zero to six selected E0004. Preserve classification across 17280 cases
except 96 helper cases now recognizing four carriers; exercise every V19
complement alternative separately. This is not payload equivalence: replay
old bodies, run real regressions and preserve unchanged algorithms. Do not
add a production matrix merely mirroring matches.

Remove exactly six V16, add the six prescribed uniform nominal V19, preserve
other diagnostics by source/rule/severity/message, and add no V20/suppression.
Validate V15/V26 before resealing five owner/consumer pairs. Require build,
focused/full workspace tests, Clippy all-targets -D warnings and
-D unfulfilled_lint_expectations, strict Rustdoc, fmt, strict/reverse lineage,
replay, source freeze and diff --check. Heavy jobs sequential, MemoryMax 12GiB,
High 10GiB, swap 0, jobs 1. The original implementation package excluded
commit/push/merge/release; it did not close other language work, pending
contracts or the global objective.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/content/source.md` preserves the original evidence and chronology.
