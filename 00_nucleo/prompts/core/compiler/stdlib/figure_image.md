# Prompt L0 — `stdlib/figure_image` — imagens
Hash do Código: c3a8e6b7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:b57c37587932a65caedce0b0df5691463ad336b90126a978ea88a7c369432738

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/figure_image.rs`

## Evidence before specification

Image and existing native_figure share this exclusive consumer; a historical missing-dedicated-Figure-spec drift note does not excuse omission of its complete current clauses below. Shared stdlib conventions remain `00_nucleo/prompts/core/compiler/stdlib/mod.md`. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

Image format/payload/source-page probes distinguish SVG/PDF sources deliberately unsupported, signature validation in eval versus deep decode in L3 and detached decode diagnostics. The old claim Args lacks individual origins is historical, superseded by later causal occurrence transport; this source's remaining detached paths remain debts, not permission to remove new origins. `lab/history/construction/records/00_nucleo/materialization/typst-passo-1141.md:4` explicitly confirms Path|Str gate 2026-08-24; pinned foundations/path 15-29,62-81 grounds sandboxed read_path rather than old str-only read_bytes.

Caption 2026-09-21/586e1bd43fae probes and pinned figure 511-591 distinguish Align top/bottom from wrongly specified Str, omission/explicit fields and unsynthesized values. Human approval after causal 3ac60a578 covers reduced qualified constructor/entity plus Figure caption cast; integration corrected both implementation/two tests after direct oracle refuted shared bad L0 position-string assumption. Historical core 6129 pass is that bounded snapshot, not universal synthesis proof. Image alt human-confirmed 2026-09-23, fixture SHA `90475d9f6bd2b6964495861203c480af064b7273c14dc0feb492f79444dbd7d4`, PDF/UA106/0,623/0 and pinned image 165-181 distinguish missing alt field/type versus rendering.

tekt-v16-figure-casts-2026-10-17/before/before-expanded-measurement preserves dirty HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/UTC/stat/source/L0/config/binaries/130 pairs. Silent Figure body Empty/caption-type-name/kind-inference fallbacks differ; pinned body→caption→kind validation wins over named order. type-kind-probe/more-cast-probe/type-kind-classes measures 38Types(37CLI+Location source), pinned value 647-658/element 210-214/symbol 133-137/math accent 189-214/lr 144-198: callable Type/Symbol is not necessarily Element. Symbol Content carrier gap is real SY1 debt. caption-shape-expanded-before separates observed source from actual binary build source for 7new+8prior cases; None/Str/Symbol coercions already fit existing Content entity, while explicit separator Auto presence doesn't. V17 T241/T242 at 174/177 has source/L0/AST/CI28guards 21owners evidence, no runtime/AI-benefit claim.

## Exclusive current image contract

native_image(path,width?,height?,fit?,page?,alt?) reads only through injected World abstraction, no direct filesystem/global state. First source Value::Path|Str: rooted helper resolves Str once in caller context, preserves resolved Path, reads World::read_path. Use normalized vpath for extension/loaded bytes for signature, preserve portable virtual identity only, never physical PathBuf/sandbox bypass. Keep dimensions/fit default cover with contain|cover|stretch.

page optional positive Int is validated no-op while PDF source unsupported;0/negative exact `number must be positive`, wrong `expected integer, found {tipo}`. alt optional Str→Option<EcoString>, absent remains observable for later PDF/UA-specific validation, never paint/text/size/fit/MIME/read behavior; other targets unchanged. Entity public-field gate was separately confirmed, no arbitrary new metadata.

After loading reject case-insensitive svg/svgz exact `SVG images are not supported yet`, pdf `PDF images are not supported yet`, regardless signature. Unknown detector→`unknown image format`. PNG/JPEG/GIF/WebP known signature proceeds without full eval decode; L3 validate_document_images before export performs deep validation and fails `failed to decode image ({detalhe})`, detached existing nuance rather than silent omission. Dedicated format detector/entity owner shared with exporter. PDF-source/hayro/Form-XObject/other decode dependencies are consciously scoped out, no new L1 heavy/I/O logic here. Existing detached argument/file errors remain independently inventoried, not complete causal-span parity.

## Exclusive current Figure and caption contract

native_figure body accepts Content/LocatedContent/Str/None(Empty)/Symbol(graphemeText provisional), missing exact `missing argument: body`, unsupported `expected content, found {vanilla_type_name}`. Caption None means absence, Content/LocatedContent/Str/Symbol supported, invalid `expected content or none, found {tipo}`; FigureCaption body extraction retained. Kind Str keeps name/Auto keeps historical auto; Func only existing native_element_fields or Func::element_name recognized Element, no name comparison/constructor execution. Arbitrary Func/With→`expected element`, other invalid including None→`expected function, string, or auto, found {tipo}`. Valid Element Func still takes historical inference path pending typed-kind public evolution; no false equivalence to String kind. infer_kind_from_body V16/recursion/phase remains open, no fabricated neutral-container leaves to silence lint.

Type kind with constructor→`expected element`, no constructor→`type {nome longo} does not have a constructor`; exhaustive measured Type classification, Bool name boolean/Relative name relative length. Callable accent/LR Symbol→`expected element`, noncallable→`symbol {valor} is not callable`; delegate accent/LR classification private bridges, no copied tables/new public Func/trait. Symbol body/caption preserves whole graphemeText as current markup, not Empty/error and not complete SymbolElem morphology/reflection (SY1).

Validate body→caption→kind once, preserve Args/occurrences/extras/unknown-name order. Type errors use first body/last caption-kind corresponding value_span, then occurrence.span then args.span, never Value equality/string search. Missing keeps detached existing policy for caller origin. Shared private figure_argument_value_span extracted from already-tested closure serves Figure/caption without reevaluation or unrelated order changes.

native_figure_caption(body,position:,separator:) creates existing Content::FigureCaption(FigureCaptionElem{position,separator,body}). Body/Separator pure cast Content/LocatedContent/Str/None/Symbol; None→known Empty including explicit separator:none, Symbol→provisional Text. Required body/named-body/extra/type diagnostics measured: `missing argument: body`, `expected content, found {tipo}`, ``the argument `body` is positional`` with inherited measured hint. Position only Align2D{h:None,v:Some(Top|Bottom)}, no horizontal component/String; preserve exact ``expected `top` or `bottom`, found left``/`expected alignment, found string` and invalid-alignment repr. Position/Separator omitted remains None/unset, don't materialize defaults. Field order explicit position,separator,body; declared presence preserved in original value. Figure currently extracts only caption.body; stored position/separator have no visual effect.

Do not synthesize kind/supplement/numbering/counter/figure_location, realize numbering/supplement text, resolve locale default separator, or apply explicit caption position/separator visually in this reduced slice. Unsynthesized getter errors differ upstream, not language parity claimed from missing fields. SeparatorAuto versus omission cannot fit Option<Content>; FG1 proposal required before sentinel/public change. Typed-kind/synthesis/recursion FG1 and SymbolElem/reflection SY1 remain follow-ups at tekt-v16-figure-casts-2026-10-17/follow-ups.md, not executable authority.

## Acceptance and limits

Preserve image loading/format/signature/deep decode errors/page validated-no-op/alt presence/type/source sandbox controls and existing geometry. Figure functional RED before code for missing-invalid body/caption/kind/causal precedence/extra order, actual aliases/homonyms/Type/Symbol cases and grapheme survival; repeat 130 pairs classifying intended fixes/controls/prior diagnostics without claiming JSON/fields/reflection completeness. Caption Align top-bottom versus String/left, None/Str/Symbol/Located coercions/known EmptySeparator/span,7new+8prior controls and exact independent RED→GREEN. Source/public carrier graph audited before proposals; no defaults/synthesis invented at encoding or construction.

V17 structural rewrite preserves accepted domain/error/short-circuit/effects with no suppression/config hiding; existing functional controls before/after, family RED→zeroV17. L0/validV15-V26 before restricted owners reseal/full workspace/build/strict all-target Clippy+unfulfilled/Rustdoc four indexes/fmt/strict-reverse lineage/diff/SARIF, sequential 12GiB/High 10GiB/swap 0/jobs1, no release/merge/commit/push; RG1 independent. No general ICC/morphology/render/phase certification or new public default/signature from this internal repair.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Stop constructor-time infer_kind_from_body entirely. Native Figure validates body→caption→kind→supplement→numbering→unknown/excess arguments, independent of named source order. Preserve last named value_span→occurrence.span→Args.span and positional origins. Kind omission is None, explicit auto Some(None), string Some(Some(Name)), and actual recognized element Func Some(Some(Element)) retaining the original Func identity. Use typed native_element_fields registry or actual FuncRepr::Element, never name alone; preserve Type/Symbol/With errors. No inference/callback executes during construction.

Supplement accepts Auto, None, Content, LocatedContent, Str/Symbol via Content cast, and Func with its argument span. Invalid supplement errors `expected content, function, none, or auto, found {type}`. Numbering accepts Str/Func/None, transports the exact supplied Value in existing figure.numbering Styled chain; omission stays absent. Invalid numbering errors `expected string, function, or none, found {type}`. Explicit constructor values override inherited settings; resolved fields start None. Preserve body/caption cast semantics, unknown/excess diagnostics, all independent figure.caption and image loading/decoding contracts including unsupported SVG/PDF. New kind inference/preparation belongs to the occurrence coordinator, not this owner.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.

## Located supplement alias preservation

Actual `reference-matrix-product-2-main/receipt.json`, case
prepared-alias-two-references, records queried Figure/count/distinct-Location
values [[4,[3],[1],4]] while the pinned oracle reference-acceptance/oracle receipt
records [[2,[1],[1],2]]. The constructor's supplement cast calls
caption_content_value at figure_image.rs:39, where LocatedContent(snapshot,actual
Location) was projected to snapshot.content() alone. That erases an intentionally
prepared alias before two reference uses; source parent/derived re-location then
creates extra automatic actions. This is a measured alias-contract defect, not
permission to deduplicate by value, regenerate a Location, or skip counter actions.

For LocatedContent accepted by this existing content cast, retain its existing
content unchanged inside the existing Content::Located carrier with the supplied
actual Location. Figure query snapshots already contain the prepared Figure
entity fields (their separate authoritative fields slot is None in this measured
path). Preserve that complete underlying entity, source wrappers and resolved
kind/supplement state; do not invent a generic snapshot-field reification API or
fabricate extra fields. Other Value casts and native_figure's separate body and
caption branches remain unchanged. Existing figure.caption helper callers retain
the same content and identity instead of losing an accepted Located alias.

Add a focused native Figure constructor control using a fully prepared Figure
LocatedContent as its supplement: require the exact original Location and complete
body/caption/kind/lexical/resolved fields, without executing any callback. The real
repeated-reference matrix remains the language-level RED→GREEN requirement. This
is an internal correction within the approved alias/carrier contract, with no new
public signature, entity field, phase, inference or generic Func/Content equality.
