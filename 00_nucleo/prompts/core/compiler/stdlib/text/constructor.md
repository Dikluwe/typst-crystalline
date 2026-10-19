# L0 — stdlib/text/constructor
Hash do Código: 262c4afc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:41e38095ab33ab1710ba11f43d6a5ca50def5a703f77a0001a066b72964c4165
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:c8570ec5f4f32c0805587230cafc796f35e5ee7c9f8e634e13d807db2afb6e00

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/constructor.rs`

## Evidence before specification


`01_core/src/compiler/stdlib/text/constructor.rs:36`, `01_core/src/compiler/stdlib/text/constructor.rs:190`, `01_core/src/compiler/stdlib/text/constructor.rs:198` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.



Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

Hyphenation policy 52 controls, hardcoded overhang 20, configured costs 66 and widow/orphan 78+24 source/state/initial/before receipts are preserved in correcao-politica-hifenizacao/correcao-hardcode-paragrafo/correcao-custos-paragrafo/correcao-viuvas-orfas-2026-09-28 diagnostics. They distinguish ignored versus typed policy, shared exterior costs from partial run styles and certified regional protection. first-policy-green.log has 5policy tests passing but 4long-type errors: local helper needed integer versus global type_mismatch int; existing length helper already uses language names. after-first-resultados/proveniencia refutes refusing glyph-neutral cost channels while integral Styled factoring already occurs before Par. No intent inferred.

TS1 `lab/results/construction/diagnostics/text-argumentos-2026-10-17/` before/morph-before/audit preserve dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/status/stat/source/L0/binary/argv and historical pin. Named em/affine/nested-size and six typed positional permutations expose early size collapse/body extraction. Human approval manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba` covers full size/test integration under 12GiB/zero swap, no release/merge/push; equality/RG1 remains independent.

## Exclusive current constructor contract


Own native_text and its private length/weight/style/lang/font/edge validators. TS1 supersedes the old initial-color restriction: body accepts Content|Str|None, typed positional size/fill search spans the argument list, and named properties take priority without consuming positional values of the same type. Missing body is `missing argument: body`; remaining positional arguments fail `unexpected argument`.

Accept the current VANILLA_TEXT_SET_PROPS closed set, with the same validation and transport as the set-rule owner. Known but uncaptured fields retain the existing accepted-without-effect scope limitation; unknown names, including bold/italic, fail `unexpected argument: {name}`. Preserve implemented transports: size Length -> Size(Length), fill Color -> Fill (None fails the paint cast; Gradient/Tiling transport remains separate), weight raw Int or symbolic Str -> Weight(u16), style normal|italic|oblique -> text.style, tracking Length -> Tracking, lang BCP-47 Str -> Lang, font Str|Array[Str]|Dict -> FontList, top-edge/bottom-edge named metric or Length -> their custom values, horizontal Dir -> text.dir, variations Dict -> folded text.variations through FontVariations::from_value.

Font accepts one family, a nonempty string array, named dict with required family and optional variant/weight/style, or legacy family-keyed variants/variant arrays. Empty fallback fails `font fallback list must not be empty`. Top-edge metrics are ascender/cap-height/x-height/baseline/bounds; bottom-edge baseline/descender/bounds. Invalid metrics use edge_cast_error. Vertical ttb/btt fails `text direction must be horizontal`. Preserve `unknown font weight name: {name}` and `unknown font style name: {name}`.

Accumulate Styles and return Content::Styled, or direct body when empty. Reuse the set-rule error helpers (type_mismatch, expected_length_error, edge_cast_error), not duplicated literals. Validators remain private, only native_text is reexported. No World/context/file reads. This unit owns constructor validation; set-rule dispatch and shared pinned properties remain with their actual owners.



TS1 typed positional search chooses first castable Size Length and Color anywhere, unless that named property is present; named presence takes precedence without consuming corresponding positional type. Keep Args occurrence/spans and named validation. Remaining body Content/Str/None→Empty, required; reject leftover/body-named with measured pinned message/hint, no argument/body reevaluation or font/region read. Full Size Length remains typed through Styles. Gradient/Tiling retain historical partial rejection/message/span, not bogus expected-gradient/found-gradient or fake solid support. Their explicit V19 group shares missing textual Paint-carrier policy; None/other invalid types use measured Paint error. Fill/render carrier is separate pending work.

hyphenate Bool|Auto transports text.hyphenate without body/other-style changes, using last named occurrence value_span or synthetic args.span. Its dedicated local long-name cast helper boolean/integer/string supersedes only initial type_mismatch reuse for this property; do not change global helper/type_name/public API. overhang Bool uses same custom/error channel as set owner and last overhang value_span, no new TextStyle field. costs uses shared parser and last named value_span/synthetic args.span; apply components without clearing inherited fields/body and no Sink warning invention.

Only text.costs.hyphenation/runt/widow/orphan enter preparer's glyph-neutral Styled whitelist. Shared exterior paragraph cost is read after temporary preparer push restores, not per partial fragment; complete Style factoring remains existing realization-before-Par, no new word boundary/space or phase. Defaults retained. Regional widow/orphan is validated/transferred here, selection/admission only canonical `00_nucleo/prompts/core/compiler/layout/paragraph_pagination.md` certificate, preserving refused geometries and default/strategy rules. No arbitrary variable-width/partial-context expansion.

## Acceptance and limits

Preserve all constructor forms/recognized validation/unimplemented known-field explicit limitations, errors/hints/spans/body/fields/native identities and public signatures/default/phase/compatibility. Meaningful RED→GREEN tests before code for hyphenate type/transport, overhang/body, all Ratio costs/empty/invalid/fold/error priorities and typed size/cast permutations/signed-zero/negative/restoration/Raw/Heading/math/capture/show/duplicate spans. Runtime regional tests certify only owner-approved subset; constructor success alone is no effect/geometry proof. Valid 1:1/pins before restricted reseal/full suites/build/lint/Clippy/Rustdoc/fmt/reverse lineage. Existing Fill Gradient/Tiling/other accepted-without-effect fields/equality debts remain separately inventoried, never erased by full-size completion.
