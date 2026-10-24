# L0 — introspect/extract_payload
Hash do Código: 172640c2


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Measured Source and OutlineEntry alignment

### Source evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/tekt-v16-introspect-origins-2026-10-16/before.json.gz`
retains UTC/status/git diff HEAD --stat/full sources/hashes. Value has 39 variants,
Content 120. eval/mod.rs:956 has latent None fallback T027; layout/realized.rs:14–45
has implicit predicates/other binding T158/T160, :77–130 implicit-false if-let
chains. extract_payload.rs:18–98 does not normalize Source while locatable.rs:21
classifies its body. The pinned source-origin nucleus requires transparency and
one occurrence. Hypothesis requiring precode RED: Source(Heading) returns None
and Located(Source(Heading)) loses preparation proof. traversal.rs:79 already
handles Source transparently. No inferred upstream intent or fresh vanilla
parity; the first audit deliberately did not close OutlineEntry.

### Later OutlineEntry evidence and supersession

`lab/results/construction/diagnostics/tekt-v16-introspect-walk-2026-10-16/before.json.gz`
retains the same HEAD/uncommitted state, UTC/exact diffstat/source hashes/binaries.
measurement-before.json/measurement-fields-before.json at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`: direct entry appears once, synthesized
once, mixed twice; Tekt direct query loses it (0 vs1), mixed loses direct (1 vs2),
and direct body() errors `cannot outline heading`. outline_entry.rs:61 accepts
only raw Heading in body; :99 supplies NativeElement; extract_payload.rs:148
drops OutlineEntry; walk.rs:805 treats it terminal; outline_entries.rs:38–61
synthesizes post-walk. map_content visits element/fill (traversal.rs:221), used
by realized.rs:139–281 to certify origins: test a Located referenced Heading
without confusing it with a rendered descendant. Pinned outline.rs:472–481
declares OutlineEntry Locatable. This measures language semantics/morphology/
errors, not Rust/byte/PDF mechanics. Wrong query/fields/body, phantom headings,
duplicates or synthesis/order/label regressions refute the correction.

The later decision supersedes only this owner's earlier pending OutlineEntry
alignment caveat. Historical investigation remains in the archived original;
Dynamic still depends on its own kind/payload boundary. No universal proof of
all variants follows from scoped alignment.

## Exclusive extraction contract

Preserve pure `pub fn extract_payload(content: &Content) -> Option<ElementPayload>`.
Same modeled input gives the same result independently of counters/walk context/
Locator/Engine. Domain cloning is allowed; no I/O, descendant traversal or
callbacks. Native payload formulas/fields belong to each entity's L0, not a
second table here. Lexical numbering/supplement/alt capture belongs to
`00_nucleo/prompts/core/compiler/introspect.md` after extraction.

Borrow-normalize content.source_body() before the static match. Delegate native
to_payload for Strong/Emph/Heading/Figure/Cite/Metadata/State/StateUpdate/
StateDisplay/CounterDisplayCallback/Outline/OutlineEntry/Bibliography/Equation/
Table/Footnote/CounterUpdate/ContextBlock. Dynamic uses existing dyn_to_payload,
not new dynamic dispatch. Located delegates to body; ShowRecipe/Realized have no
own payload and walk handles provenance/body separately. Other current variants
return None through the explicit nominal complement, not an open wildcard.
Source in that complement is unreachable after normalization, never a producer.
Preserve current signatures and behavior beyond already-required transparency.

The later alignment adds exactly Content::OutlineEntry(e)=>e.to_payload(),
removing OutlineEntry from None. The entity already supplies NativeElement;
add no payload type/Location special case or rendered descendant from element.
Replace only the prior discrepancy test with direct/Source Some(NativeElement);
other expectations remain. Keep post-walk synthesis and measure no duplicates;
refuted controls require refinement before any additional exception.

Do not infer payloads from bold/italic flags or descend through Styled; normal
walk finds child occurrences. Check native delegation, own empty/nested Strong/
Emph and None for Text/Empty/Space/SpaceFromNewline/Sequence/Styled. New locatable
producers require coordinated classifier/extractor/causal-traversal review under
`00_nucleo/prompts/core/compiler/introspect/locatable.md`. Update stale comments
about universal nonlocatability/open evolution; preserve explicit nominal barriers.

## Acceptance and limits

Source controls cover producer/nonproducer wrappers, Located/Source, no duplicate
collection, fallbacks/predicates. Confirm transparency RED before implementation;
existing preservation/single_evaluation_ controls pass before/after. Real workspace
copies with separate Value/Content sentinels retain evidence of absent E0004
barriers before and require E0004 at six selected points after, with sources/
other errors recorded. Restricted resealing only. Validate build/full workspace,
all-targets Clippy warnings/unfulfilled expectations as errors, fmt, strict
Rustdoc, V1/V5/V15/V26/reverse and SARIF removing only selected V16, allowing only
prescribed equivalent V19. Uniform groups share bodies without duplication/
suppression; structural barriers are not proof of execution of every alternative
or a defect in every fallback.

For OutlineEntry, L0 first and local tests/real functional RED before code.
Preserve old controls/CLI measurement; repeat direct/synthesized/mixed body()/field
fixtures at language level. Separate copied Content sentinel must add E0004 at
W1–W6 after, not before, without other errors/stubs. Keep logs/inputs. Reseal only
five affected pairs after valid V15/V26. Require workspace build/fmt/all-targets
Clippy -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings, full
workspace suite, strict/reverse lineage, diff and differential SARIF: six V16
removed (239→233), six new informational V19; extractor complement loses one
alternative and two needs_fresh_index groups move OutlineEntry false→true.
No new V20 or other diagnostic change after line mapping. V19 counts alternatives,
not execution coverage. No suppression/global configuration change. Under
12 GiB/zero swap/one job, heavy commands sequentially; record state/UTC/hashes.
ADR-0127 internal parity flow adds no API/field/trait/default/phase/compatibility.
No publication/commit/push. Other V16/general objectives remain open.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Keep pure Figure delegation to Element::to_payload. Migrate expectations to typed kind and supplement, distinguishing legacy custom names from actual element identity and numbering-independent caption presence. Preserve all other extraction/locatability policies.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.


## Approved reference contribution integration

Measured existing boundary before decision: `01_core/src/compiler/introspect/extract_payload.rs:18 and :94` retains the old generic transport or legacy assertion behavior. Source inspection establishes ownership; the independently frozen actual L3 INNER_PREPARATION RED establishes the language omission.

The human approved frozen proposal SHA1578cbea7378b739a7c1b0809d3437c0165c2de6d288e40b2db75c9ce7e38ff2, recorded in `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/reference-phase-approval.json`, including the separate no-child clarification. Actual L3 explicit/callback INNER_PREPARATION regressions failed on the unchanged pre-implementation source (`reference-phase-red-frozen`), authorizing RED-to-GREEN for this exact phase. Existing unrelated obligations remain.

Return no semantic payload for the exact classified generated reference ContextBlock. Ordinary source ContextBlock payload construction is unchanged. The owning walker registers internal producer Location separately, keeping the locatable/payload invariant and preventing fabricated source Context query membership.

No additional public contract beyond the exact three approved transport functions, new target inference, second arena, shared result/effect cache, fake observation, weakened diagnostic or unrelated generic visitor change is authorized. Preserve five-attempt stabilization, canonical paths and scoped lineage; validate reached and hidden effects, forward/repeated references, aliases, missing/unnumbered/non-Figure/bibliography/page controls and mixed target behavior.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
