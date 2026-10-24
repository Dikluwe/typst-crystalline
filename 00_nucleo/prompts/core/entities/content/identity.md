# L0 — entities/content/identity
Hash do Código: 85cf346a


**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/identity.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/math/equation-constructor-origin.toml sha256:947456f05619642c93f67721453e4ad5577557679dbc422a20a48f397ae97ded
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:dced4bcb1e45b09423bb3b9739af4da3cd440d500d6a620e08e13219236d1735
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:8388731d50379499612641a70a85695c0e7139ec605289c29161e12086abd78a
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:6345ad9c825988872fb53b94c574f61e85a7c21074597400c33bd13ae6a3f68d


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

## Equation constructor identity

At HEAD `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`, this owner's
`identity.rs:13-31` recognizes heading transport but names option-bearing
equation construction styled. The producer at
`compiler/stdlib/structural/math/equation.rs:175-195` already transports explicit
options without changing the Equation body. Current-pin language receipts in
`lab/results/v16-corrections-2026-10-19/batch-e/agents/language/` and upstream
`lab/typst-original/crates/typst-library/src/math/equation.rs:62-131` distinguish
that constructor from an unrelated Styled wrapper. This is public identity
morphology; no Rust representation equality is required.

Recognize equation identity through a private nominal classifier only for a
valid `equation.constructor` Bool(true) marker whose body reaches Equation
through Source, ShowRecipe, Located, Realized.visual or another valid marked
constructor transport. An outer/unmarked Styled is opaque even if it carries
equation.* option keys; false/non-bool marker and a marked non-Equation body
are not constructors. Never execute recipes or callbacks. No public helper or
entity field/signature is added; preserve Heading, aliases, bare Equation and
all unrelated identities and structural Debug. The producer owns installing
the marker only on already nonempty explicit-option transport.

Tests must fail before the fix for valid marked identity and preserve ordinary
Styled, invalid markers, non-Equation bodies, carriers and Realized.visual
selection. Actual native constructor identity is also exercised by the producer
owner. Continuous ADR-0127 applies; keep owner-specific lineage and shared
nucleus pins consistent before compiling and validating.

## Carrier-transparent diagnostic depth

### Measurement

`lab/results/v16-corrections-2026-10-19/batch-g/hash-before/receipt.json`
(SHA-256 `786b466c42e89682d021248a005ecda102a93011280e4f3851d9d897947a3664`)
records the public Rust probe at HEAD
`4382891f0d5f0839669597cb5c90a98f24cfb8bd`, with an uncommitted working
tree bound to the complete input snapshot and successful candidate-build
receipt SHA-256
`395efdcf67420d63942b2d90bc4d906a8ffbc245aca8e18b640307d0e619951c`.
Compilation and execution both returned zero between
`2026-10-21T00:03:23.653097+00:00` and
`2026-10-21T00:03:24.489288+00:00`; inputs and the actual core artifact
remained unchanged. The receipt binds Rust 1.92.0, the probe, command arguments,
real workspace dependency artifact and its SHA-256. The identity, Rust Hash
and public content-hash owners and consumers were byte-identical to that HEAD.

Of 366 actual rows, 248 equal pairs had different public content hashes and
168 had different Rust hashes. Source/Located depth controls reproduce the
cause in existing Strong/Sequence and newly comparable Title/HtmlFrame/HtmlElem,
plus same-id Context and the two equivalent Space origins. At
`identity.rs:296-319`, Debug counts each transparent carrier before delegating
to its body or visual: a leaf below seven visible Strong nodes is exposed
without a carrier but becomes `...` with one Source. At
`content_hash.rs:25-29`, hashing the resulting Debug string makes this a
compatibility violation, rather than a language identity or occurrence change.
The nominal Rust Hash fallback has the same cause for nested carriers.

The independent nested-Value Float(-0.0/0.0) hypothesis was refuted by its
actual control: CounterDisplay Debug omits that payload, so its one equal pair
has no public or Rust hash violation. No observed gap is inferred for that
path. The finite corpus neither establishes nor weakens the global equal-value
hash requirement. Original source/L0 bytes before this refinement remain in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/core/identity-red-preparation/before.json.gz`.

### Contract

Debug shall project Source.body, ShowRecipe.body, Located.body and
Realized.visual by borrowed iterative descent before checking or incrementing
the visible-depth counter. These four invisible carriers consume no visible
level. Every other current Content alternative explicitly stops that descent;
the closed classification must reject an unreviewed future variant at compile
time. Do not clone nodes, recurse across the carrier chain, execute recipes or
closures, inspect Realized.original, resolve styles or alter causal data.

Keep MAX_DEPTH exactly eight, the existing depth counter and reset behavior,
and all nominal payload formatting. Seven visible Strong nodes still expose
their text; the eighth still truncates. This clause supersedes the earlier
output-preservation requirement only where invisible carriers consumed that
budget and broke established body/visual transparency and equal-value hashing.
It changes neither elem_name nor any other nominal Debug output. Source spans,
Locations and captured recipe state remain stored and untouched.

Both existing hash algorithms remain unchanged. The public hash owner permits
numeric results to change with separately specified Debug changes; that does
not exempt equal values from equal hashes. Do not introduce a weaker Hash law,
a new global, a public entity/signature, a product default, a pipeline phase or
an occurrence-identity projection. This is an internal diagnostic-formula
correction under ADR-0127 continuous flow, justified by the measured violation
and existing Source/carrier invisibility. It grants no unrelated Debug or
hashing rewrite and makes no universal parity claim.

### Acceptance and limits

Three owner-local functional regressions must fail before the correction and
pass afterward: one leaf at the visible boundary across all four carriers and
representative old/new equality families preserves Eq and Debug; public and
Rust hashes agree for selected equal pairs with deep carrier chains; visible
MAX_DEPTH remains effective while a 512-carrier chain selects only the visual
and retains original data without executing stored panic-if-called closures.
Keep the existing tests and payloads unchanged. Compiler-negative extension
checks must protect the actual closed carrier classifier and nominal formatter
independently of these functional tests. Repeat the source-bound public probe,
focused/full regression suites and structural/ownership/lineage gates with
unchanged inputs during each measurement. This bounded correction does not
certify every Content/Value equality pair or broader language parity.

## Historical provenance

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/content/identity.md` preserves the original evidence and chronology.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## R4 — direct public field diagnostics

Evidence precedes correction: before-final/receipt.json under
lab/results/math-fields-r4-2026-10-23 records pinned bd5ebec2, UTC, source
map, binary SHA and working-tree stat. Direct omitted vec.gap/cases.reverse
reports does-not-have-field locally, whereas vanilla distinguishes unset with
`field ... is not known at this point`; these are observable language errors.

Correct only MathVec and MathCases diagnostic elem_name entries to vec and cases respectively, as measured in absent-field oracle diagnostics. Preserve public Symbol identity and every other mapping/Debug representation.

This supersedes only conflicting lookup/name clauses above; continuous
ADR-0127 parity correction without public Rust signature/entity/trait, phase
or default changes. Tests first, RED→GREEN, existing suites and full lineage/lint.
