# L0 — entities/content/identity
Hash do Código: db961f0d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:dced4bcb1e45b09423bb3b9739af4da3cd440d500d6a620e08e13219236d1735
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:8388731d50379499612641a70a85695c0e7139ec605289c29161e12086abd78a
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:6345ad9c825988872fb53b94c574f61e85a7c21074597400c33bd13ae6a3f68d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/identity.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Base identity contract

Own `pub fn elem_name(&self)->&'static str` and depth-bounded structural Debug for every closed Content variant. Use nominal exhaustive static matches without wildcard, dynamic dispatch, registry or property map. Adding a variant forces explicit diagnostic-name classification at compile time. Preserve existing names/output/payload formatting except the measured language-name corrections below. Space and SpaceFromNewline both identify as `space`: causal origin is not public element identity.

Internal carrier names are transparent; Debug projects lexical/visual content without making carriers language elements. Never infer occurrence identity from Debug. Keep callers compiling, identity/field-access controls and core library tests green, with valid ownership/nuclei/direct+reverse lineage after restricted resealing.

## Semantic SmartQuote evidence and approval


The historical smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz collection records pin `9f2b6e8715237cb086899a42873660fe744622e8`, binaries/HEAD/tree/diffstat/UTC/sources/argv/cwd/environment/output. PDF uses extracted text, HTML DOM/text/diagnostics, not PDF bytes. Eval converted markup quotes into Text, losing double/enabled and retaining English glyphs when reused under German/French/disabled styles. Primes6'/2-inch were misread as quotes. Native-function show/public-name corrections already passed separately; empty constructor has no fields upstream, explicit double:false is known while omission is Unset, and enabled:false remains SmartQuote. New set-double/show-filter positives distinguish presence. HTML literal worked through Text while programmatic SmartQuote rejected; literal titles render quotes but script title expects string and rejects Content before export in both binaries.

Historical markup_stream.rs:79–128 chooses glyph during eval; layout smartquote toggles two flags without preceding text; stdlib smartquote.rs:112–117 erases disabled elements; observation/elements.rs:242–250 compares current fields. Upstream eval/markup.rs:141–146 creates explicit-double elements, inline/collect.rs:196–211 resolves style/context and text/smartquote.rs:119–158 handles prime/apostrophe/nesting.

Human approval on 2026-10-06 covers double bool→Option<bool>, enabled:Option<bool>, pure shared helper/context and glyph selection moving eval→target composition. Keep identity/fields during eval. Paged and semantic HTML use their own target composition (ADR-0128), sharing one pure L1 contextual selector with per-stream/compilation state. No I/O/global/vtable/property registry/upstream memoization copying, new mode/flag/unsafe/watch API/math phase. HTML escaping remains separately owned. Stack representation is mechanics; fields/known-versus-omitted/glyph/style are acceptance. Other proposals/ADRs remain unchanged.


Change only SmartQuote's public diagnostic elem_name to `smartquote`, as the measured .double behavior requires. Keep its Content variant, exhaustive signature and all unrelated names. Rust Debug may retain the historical internal form; it is neither JSON nor a language parity criterion.

## Matrix and Cell diagnostic evidence

`lab/results/construction/diagnostics/matrix-morfologia-2026-10-06/before.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diff/source/binaries/cases at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Raw/query mat lacked rows, getters/methods lost fields and func-equality failed. Upstream matrix.rs:91–221 declares fields/rows; local math_matrix.rs:45–52 retains rows/explicit but get_field defaults, content_methods.rs:151–191 omits mat and cli.rs:1154–1348 falls back to body. This is language morphology/semantics, not required Rust structure/PDF bytes or inferred intent. Change MathMatrix elem_name from `math.matrix` to `mat`; Rust Debug remains `math.matrix`.

`lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz` freezes the same HEAD/tree provenance plus argv/cwd/results. Upstream table.rs:734–778 defines body/properties; local table_cell.rs:62–109 lacks getter, identity.rs:107 uses table.cell, content_methods.rs:226–254 loses native function and rules/show.rs:300–320 omits it. Body/fields/func-eq/explicit-show refute parity. Change TableCell elem_name to `cell`; Debug remains `table.cell`.

presence-refuters.json shows x:auto versus omitted collapse and lost fill:none/stroke:none. Auto is valid for x/y/fill/align/inset/breakable, invalid for colspan/rowspan/stroke; none is valid only for fill/stroke. The public presence gate was confirmed on 2026-10-07 in `lab/history/construction/records/00_nucleo/materialization/table-cell-presenca-aprovacao-2026-10-07.json`. Presence introduces no new element identity: retain cell and mechanical Debug. Option None alone proves neither omission nor explicit none.

Require measured fields/has/at/func/JSON/query/padding/presence RED→GREEN, span rejection/clone/map/observation, build/lint and D05/D06/query plus D08A/D05/D07/new-refuter collections with exact provenance. These internal corrections add no public Rust type/signature/phase. Matrix selectors/render/general Styled/unmeasured elements and positioned/style/padding/show Cell realization are not certified by name repair.

## Prepared table evidence and approval

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diffstat/status/sources/binaries/argv, historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` and Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Upstream grid resolve.rs:204–278 resolves fields,1500–1530 ordered conflicts,1665–1703 empty cells and 1945–1988 breakable. Local grid_placement.rs:91–112 prioritizes explicit cells and 262–270 only knows GridCell; realize/mod.rs:160–200 visits bottom-up and table.rs:127–152 strips wrappers without attributes. Probes show zero cell effects when Table is replaced, zero hidden-heading effects and aliases `LOCAL a`, `OUTER LOCAL a`, `NEW a`. Unknown fields, hidden effects or wrong order refute the hypothesis. Cache/pagination equivalence is not inferred, and Rust mechanics/bytes are not the acceptance target.

The original D08B3-A gate was satisfied by `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`. It covers the specified five public fields, two carriers, facade and logical phase before Cell show, without authorizing later extensions.

## Prepared and source-carrier identity

Keep public cell/table names and bounded mechanical Debug. Prepared carriers create neither ElementKind/NodeKind nor identity by name.

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/tree/UTC/source/argv/binaries at the historical pin. Existing ShowRecipe/Located/Realized branches provide the corresponding transparent operation. The exact Source contract remains `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`; human approval `confirmo` on 2026-10-07 is recorded in the final `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`. Source delegates to body without exposing span as language identity or executing recipes; segmentation retains origin where applicable. Keep exclusive ownership/static dispatch, no I/O/new signature/trait/flag/eval-layout phase. Require C2/C/macro collections/builds/lineage.

## Prepared table acceptance and incomplete scope

Require RED→GREEN, suites/build/lint, restricted preservation and the original D08A/B2, D05/D06/D07/query collections. Accept language, effects and actual geometry, never byte equality or whitespace normalization to close render defects. The isolated-cell package separately completes raw-cell-no-placement/outside-fields defaults and fresh show-cell-reconstruct generation; the first prepared-table package alone is explicitly incomplete.

Generic Table defaults/fields (`table-root-equality`), function styles, rich paints/strokes/insets, lines/gutters, multipage spans and PDF/UA remain outside certification. Do not change legacy extra casts, syntax, flags or global nonlocatable policy through this slice.

## Typed-transform identity

The transformacoes-2026-10-14/results.json.gz corpus and supplementary controls measure lost identity/presence in matrix/body; pre-migration source/L0 are in `lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14/base-files.json.gz`, hashes/lines in consumers.json. Approved manifest `960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d` authorizes typed representation/layout resolution. Name Transform by operation as move/rotate/scale/skew; Matrix stays transform. Never infer native identity from matrix values. Debug carries operation/body. Require RED→GREEN and dependent-path revalidation; build alone does not close X1–X5.

## Constructor Heading identity

`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/target-controls.json.gz` shows heading-raw-constructor-func regressing to styled because identity.rs:26 fixes that name for every Styled. Use only the nominal constructor_heading projection ratified by review SHA-256 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c` to return heading for that marked shape. External/unmarked Styled stays styled. Do not generally unwrap, change structural Debug/signatures or add API beyond that ratified facade. L0 precedes code and func/fields/errors/opaque-Styled controls.

## Empty and Sequence reflection

`lab/results/construction/diagnostics/sequence-reflection-2026-10-17/{before,before-sources}.json.gz` freezes HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree/UTC/diffstat/source/binary hashes at current pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Upstream content/mod.rs:724–728 requires Sequence children and 538–585 projects has/at/fields. empty-fields/empty-func/empty-children and sequence-fields/sequence-children/sequence-single/sequence-for refute local absent fields/empty name; fields.rs:317–322, identity.rs:24 and content_methods.rs:242 identify causes. Empty text stays Text and ordinary Styled remains opaque; Rust representation need not match upstream.

Name Empty `sequence`, like Sequence, for func and field errors, superseding only its historical `empty` name. Keep signature, other identities and Rust Debug. This reflection slice explicitly leaves Sequence construction and selector/where/show/counter admission to its separately measured successor, including argument/identity/limit refuters; generic invalid-method diagnostics are also outside scope. Do not claim total func/Sequence parity.

Use continuous ADR-0127 with no fields/types/traits/signatures/phase/product mode added. Require L0 first, RED→GREEN/regressions/workspace/build/strict Clippy+Rustdoc, valid ownership/nuclei/strict+reverse lineage and no increased V16. V15/V26 must pass before restricted resealing. Heavy validation is sequential under MemoryMax 12 GiB/High 8 GiB/swap 0/jobs 1; record peak without inferring gain.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/content/identity.md` preserves the original evidence and chronology.
