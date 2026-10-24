# L0 — compiler/eval/rules/textual/space
Hash do Código: d7d13730



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/space.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec


## Measurement before decision

At the working tree captured in `lab/results/construction/diagnostics/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual.rs:335-463` contains space origin, neighbor visibility, wrapper text, normalization and Unicode classification. The `p1494` filter passed 17 tests on that state. This is a structural baseline, not a timing or new vanilla measurement.

## Decision and behavior

Own `normalize_markup_spaces` and its helpers in this descendant module. Before literal/regex show on realized flow/root, collapse adjacent spaces to one occurrence whose newline origin is the OR of its components. Discard only an inferred newline occurrence when adjacent internal textual characters satisfy the pure pinned Unicode nonspacing classification (Han/Bopomofo/Hiragana/Katakana/Yi and measured ideographic punctuation, not Korean or ambiguous curly quotes). Use immutable data and the authorized icu_properties dependency. Keep explicit space between supporting texts when no component has newline. Explicit Linebreak is destructive and consumes adjacent spaces, including explicit ones, but remains Linebreak. Parbreak and actual nontext content remain causal boundaries. A textless style wrapper does not hide adjacent characters for normalization. Do not use global trim/split_whitespace, document language or target-specific space heuristics.

The coordinator calls this algorithm only when its existing `normalize_spaces` flag is true. Preserve indices by replacing consumed slots with Empty. The Unicode control and styled-space tests are owned here. No phase, public signature or default changes.

## Acceptance

The existing Unicode/space tests and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.


## Carrier-transparent normalization boundary

Expose normalization internally for insertion finalization after element/text
transformation, preserving Unicode policy. Space origin projects internal carriers;
provenance markers without visual information interrupt neither previous_visible
nor next_visible neighbor search, while retaining origin in graph and paragraph
boundaries. Supportive text projects realized carriers without changing discard
policy.

## Ratified Source transparency

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz retains HEAD/
working tree/UTC/sources/argv/binaries at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized
operations provide transparency; Source cannot leak as observable element. Human
`confirmo`,2026-10-07, final approval record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md is preapproval history. Existing ShowRecipe.body transparent
delegation also accepts Source.body, without separate-origin execution/published
carrier/Location. Preserve this operation's classification/preparation/serialization.
Sole owner/static dispatch, no I/O/new signature/trait/flag/eval-layout phase.
Require C2/C/macro tests/recollections/build/lineage before certification.

## Borrowed normalization with explicit change tracking

### Evidence before decision

snapshot-normalization-before.json.gz in
`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09`
retains UTC/sources. space.rs:66–114 reconstructs every group's first space;
space_with_origin:24–35 keeps Styled/canonical Source origin but removes Located/
Realized and normalizes redundant/detached Source. Inference: one shared group
decision allows unchanged-sequence reuse without a duplicate Unicode policy.

### Exclusive scanner and consumers

Provide crate-private normalize_markup_spaces_if_changed on borrowed slice,
returning None unchanged, Vec only from first change. Share scanner/application
with in-place normalize_markup_spaces. Every group sees earlier normalized state.
Change means discard, collapsing multiple effective spaces, adding newline origin,
or normalizing carrier; intermediate Empty alone is not change. Space/
SpaceFromNewline/recursive Styled may retain form; Source only with non-detached
span and no direct canonical origin in body. Located/Realized keep historical
transformation. Never decide through Content equality/hash/address. Preserve
Unicode/destructive Linebreak/neighbors/Empty slots/styles/provenance; in-place
consumer allocates no Vec. RED→GREEN reuse/mixed groups/Unicode/carriers/evolving
neighbors plus realize-owner suite/qualification. No public/flag/phase change.

### Timing refutation and revised responsibility

measurements-normalization{-refuter,}-first.json.gz retains sources/binaries/
UTC/HEAD/dirty/argv. First candidate consistently slower in no-fill-1600 and
table-text-1600: normalize_markup_spaces also certified canonical form before
applying each group. Refutable inference: restrict certification to borrowed
consumer to reduce cost outside normalize_insertions.

Shared scanner determines groups/policy; in-place applies each as before. Only
borrowed consumer certifies canonical form/decides output materialization. Keep
shared Unicode/updated state between groups. Repeat allocation/time controls
that refuted first variant before acceptance.

## Nominal barriers and transparent whitespace boundaries

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
`lab/results/construction/diagnostics/tekt-v16-whitespace-2026-10-17/before.json.gz`
retains sources/L0/config/hashes/UTC/status/exact diffstat. contexts.json AST/spans:
11 emitted V16/one latent identity forwarding in classified_content.
measurement-before.log measures five differences in seven Source pairs.
measurement-family-before.log/source-family-before-analysis.json measure28 pairs,
20 differences: five left/right break/Empty-crossing cases fail equally under
Source/Located/Realized.visual/Styled. Parbreak/Text boundaries/support remain.
Real in-crate algorithm in disposable workspace, not adapted private-helper copy
or universal parity proof. First external probe failed E0624, preserved without
functional RED credit.

### Current boundary projection

Shared scanner classifies intermediate Empty/destructive Linebreak through
private borrowed space_boundary_content recursively projecting Source.body,
Located.body, Realized.visual, Styled.body. Other Content stop through nominal
complement. Never inspect Realized.original/execute ShowRecipe/change text grouping.
Projection only decides boundaries; original nodes/origins/styles/positions stay
in graph. Do not consume break marker, collapse Parbreak/Text/math or generalize
to is_empty. Preserve Unicode/newline-OR/canonical_space/historical Located-
Realized space transformation. ShowRecipe keeps helper-specific current role,
no new scanner traversal. Four projection arms plus other116 Content return own
reference; V19 means stop projection, not discard content.

| Case/source | Current uniform policy | Excluded unguarded complement variants |
| --- | --- | --- |
| W02 space.rs:19 space_origin | Only Space/SpaceFromNewline and existing carrier projection have origin; others None. | Realized, Source, Located, Space, SpaceFromNewline, Styled |
| W03 :32 space_with_origin | Source/Styled keep reconstruction; rest use aggregated newline to make SpaceFromNewline/Space. Keep had_newline guard and both bodies. | Source, Styled |
| W04 :33 space_with_origin | Same as W03: these are two arms in one match. | Source, Styled |
| W05 :61 supportive_text | Only existing text/decorations/carriers provide Unicode neighbors; never convert nontext to support. | Realized, Located, Source, ShowRecipe, Text, Styled, Strong, Emph, Underline, Strike, Overline, SmallCaps |
| W06 :76 canonical_space | Only existing canonical forms skip reconstruction; Located/Realized retain historical transformation. | Space, SpaceFromNewline, Styled, Source |

None of those exclusions has a separately guarded variant; retain all guarded
variants in fallback and keep had_newline.

### Acceptance and limits

L0 before code; existing-invariant internal continuous ADR-0127, no new field/
signature/trait/API/default/phase/dependency/flag/compatibility. Complement removes
only unconditional Content arms. Uniform per-case V19 stays visible, not proof
every alternative executed. No allow/expect/@/if-let hiding decision. HTML keeps
absorbing Boundary arms and nominally closes three complement cells; no change
to preformatted/raw/title/ruby/tag tables/SmartQuote.

Add owner regressions in three modules: both normalizers, single/nested Source,
four carriers at breaks/Empty, Parbreak/Text/MathIdent controls, span/style,
Realized.visual versus original, positive/negative matching projections and3×3
inline/top-level Boundary/EmptyInline/Supportive matrices. Relax no old test.
Real RED before code/GREEN after. Future Content/SpaceSupport probes compile
cfg(test): real enum in core; disposable infra copy unlocks only out-of-target
E0004 dependencies using recorded synthetic never-executed panic arms. Never
repair selected site inside probe. Distinguish blocked target compilation from
its actual exhaustiveness. Record12 final matches/13 arms including new helper;
two space_with_origin arms are one compiler match, not two errors.

Require full workspace build/suite/fmt/all-targets Clippy -D warnings/-D
unfulfilled_lint_expectations/Rustdoc -D warnings/four indexes/strict
V1/V5/V15/V26/reverse/replay/full differential SARIF. Expected V16 152→141,
V19 only prescribed groups, no V20 increase. MemoryMax 12 GiB/zero swap/one job,
heavy tasks sequential. No commit/push/merge/release/new CI pin. Global objective/
math-stretch-composite remain open.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
