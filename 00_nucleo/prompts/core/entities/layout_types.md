# Prompt L0 — layout_types
Hash do Código: bf2fce1f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1. **Exclusive consumer:** `01_core/src/entities/layout_types.rs`.
This owner defines pure layout data: coordinates, text styles, frame items, pages and the paginated document. It performs no I/O, font parsing, rendering or compiler imports. Each adapter/shaper/exporter owns its implementation under its own canonical L0; shared invariants belong only to the pinned nuclei above.

## Evidence before decisions

The automatic-margin formula is a language rule, documented at historical upstream `e0e8ca4d`, `typst-library/src/layout/page.rs:127–131`, and implemented at `typst-layout/src/pages/run.rs:121`: `min(width, height) * (2.5 / 21)`. The asymmetric-margin probe (180pt page, left 12/right 38/top 10/bottom 10) put RTL content at 168 rather than vanilla 142, exposing loss of side identity in a scalar margin. Formula tagging probes distinguished `/Formula` with Alt from no structure tree and from `--no-pdf-tags`; geometry alone cannot preserve explicit-linebreak or paragraph-break cause. Shape probes showed even-odd fill rule and Gradient/Tiling lost before L3 when fill was converted to Color.

Math-size evidence at historical `style.rs:343–363` showed Display matrix cells should stay 11pt on Display→Text, while unconditional script scaling produced 7.7pt, a shorter grid and fewer assembly parts. The faux-bold audit of 2026-08-05 measured 1541/2074 text blocks (74.30%) using `2 Tr` with 0.073/0.051pt despite no genuine bold in the 30-section fixture; the values came from math weight 450. Historical vanilla's math weight 450 is retained; its lack of faux-bold is not a requirement to copy internal font-selection mechanics. These are historical measurements, not new runs or current completeness claims.

Discretionary-hyphen evidence is in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/{initial.json,before/}`: dirty HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, exact changed files/UTC/source/binary hashes, Tekt `1ca83fd69c72643e53d54dcbdad9488e0738cb2d37d71057a17e61ffa3b03717`, historical vanilla commit `9f2b6e8715237cb086899a42873660fe744622e8` and binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Eight en/pt × simple/optimized × justification controls retain boxes within 0.015pt after visual-only SHY→'-' normalization, but 30 discretionary occurrences extract U+002D instead of U+00AD. Ordinary pdftotext removes some line-end hyphens, so retain both extractions and compare unnormalized codepoints. Paragraph_hyphen.rs:208–214 and break_selection.rs:51–58 inserted ordinary '-'; word_layout.rs:137–144 and shaper/traversal.rs:72–75 lost cause. L1 lacks the face needed to fabricate shaped state; export's one identity per GID cannot distinguish literal and discretionary occurrences of the same glyph. Pinned vanilla inline/line.rs:173–179 and shaping.rs:583–635 preserve SHY while drawing '-'. Compound/authorial-SHY controls had additional selection differences; they do not certify UAX14 or unbroken SHY.

Text-separator source→PDF evidence at `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` uses that dirty baseline and pinned oracle: `A B C` and `#text("A B C")` extracted ABC instead of spaced words; `A#("B")C` extracted ABC in both. Eval retained Space/SpaceFromNewline, but space.rs:9 and text.rs:229 advanced without causal data. Export grouping and plain_text's invented joins cannot recover lexical spaces from arbitrary glyph gaps. This reproduction is with pdftotext, not every reader.

Authorial-SHY evidence in `lab/results/construction/diagnostics/shy-autoral-2026-09-28/{initial.json,before/resultados.json}` preserves state/commands/binaries and 30 bilateral probes: wide repre<U+00AD>sentation extracted two SHY versus one and the space control six versus one. Shaping.rs:313–325 preserved a SHY cluster but fonts.rs:318–328/builder fonts.rs:523–535 bound its substring to a reused space GID. This is occurrence-identity loss, not a new break-selection decision; narrow cases still differ in selection.

Bidi evidence in `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{initial.json.gz,probe.json.gz}` records dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/state/sources/binaries and 13 CLI inputs against historical 9f2b6e871. rtl_auto_mixed places ABC at 124.020510 versus 152.949220; ltr_numeric reverses 123/456 and places Arabic incorrectly. Layout_bidi.rs:73–82,227–240 and shaper/bidi.rs:34 re-resolve isolated lines/strings, losing paragraph context. Upstream inline/mod.rs:189,232 and collect.rs:156–178 preserve shared direction/explicit embeddings. The independently built unicode-bidi 0.3.18 probe shows that 456789 is one LTR run in its paragraph but becomes reversed runs in isolation even with the same RTL base. Dir alone is insufficient; this probe is not candidate geometry certification.

Shaped-face evidence in `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.rs,probe-build.txt,probe.txt,bilateral.json}` distinguishes DejaVu Sans regular and Oblique: shaping selected regular GIDs [5256,5260,5277,5288,5341] for emphasized مرحبا, while style-based export selected Oblique (five missing GIDs and no م coverage). Shaping.rs:254–255,358–362 retains only family; pipeline resources.rs:182–196,262–265 and export text/render/SVG select again. The earlier bidi raster confirms wrong ink. Historical paridade-producao-p534.md:79–80 documents the family approximation, not approval of GID corruption. Face identity is integrity needed for language morphology; neither matching PDF bytes nor vanilla font choices in general are required.

Numeric evidence at `lab/results/construction/diagnostics/geometria-numerica-ci-2026-10-13/before.json.gz` records dirty 06848cdcb5450f2c555626bf116bc7acd82bd82b, source/config/compiler/libm/UTC/diffstat. On this host 4968 cases show 69 trigonometry/atan2 differences at most 1 ULP, no class/zero-sign change; powers/hypot agree. This is not a universal or new vanilla-parity proof. Pre-shaping metadata evidence in pre-shaping-ci-2026-10-13/before.json.gz and cfg-test-ci inventory identifies 89 deprecated core diagnostics, while L1 emits Text and L3 shapes it; fallback/real-font tests require that permanent division.

Paint-root evidence in `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` retains dirty 06848cdcb5450f2c555626bf116bc7acd82bd82b/UTC/diffstat/binaries and Parent/Solid controls through containers/Place/Rotate. layout_types.rs:531–539 did not distinguish technical groups from roots. Tiling-nativo-2026-10-14/before.json.gz and preparation renders show Shape still carried Paint/Stroke and dense-open/stroke-open differed in all three targets, although prepare_cell already prepared once per occurrence. Geometry/text/paint are language observables; envelope/count/preparation mechanics alone prove no parity.

V16 evidence in `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/{before.json.gz,classification.json}` records the same dirty HEAD, exact state/config/hashes/UTC and AST spans/patterns/guards/bodies/complements for 70 cases across 26 owners. F051 is visit_item at layout_types.rs:1392. Exhaustiveness mechanics can change without new language semantics or inferred upstream intent.

All historical pins and numbers above retain their original state limits and do not certify current target `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Source observations are not automatically intent. Preserve provenance sufficient to reproduce every number used for a decision.

## Core coordinates, frames and current traversal

`Pt` wraps f64 typographic points: Pt+Pt and Pt*f64 are supported; Pt+f64 is not implemented. `Point` and `Size` contain two Pt coordinates/dimensions. The simpler Pt representation deliberately differs from upstream Abs(Scalar); positions embedded in frame items differ from upstream lazy (Point,FrameItem) tuples. Early Vec<Frame> document stubs are historical, superseded by the Page/PagedDocument contracts below rather than an instruction to revert.

Keep the closed FrameItem family: Text, TextShaped, Line, Glyph, Image, Shape, Group, Link, Semantic, TextReplacement and TextSeparator. Preserve every existing field; this vocabulary does not create a new variant. Frame is a page canvas. `Frame::plain_text()` is an approximate test/diagnostic join (historically Text joined with spaces); `PagedDocument::plain_text()` joins pages with newline. It is not a lexical parity oracle. Text/TextShaped extract original text, TextReplacement extracts its replacement once, separator carriers are skipped to avoid double spaces, and semantic wrappers recurse as specified. Historical ignore-Group helper behavior is not permission to lose content through newer transparent wrappers; semantic page traversal sees body only, while visual exporters traverse all page layers. Images/lines/glyphs/shapes add no textual leaf content.

FrameVisitor keeps its signature, callbacks/defaults, order and recursion. Resource traversal descends through visual children; no new visitor trait method is authorized by the transport packages. F051 removes only the final `_` debug_assert arm and its unreachable-pattern suppression: the eleven variants are already covered. The trait description must require a compile-time decision when the enum grows, rather than promise hypothetical debug fallback. Preserve callbacks and traversal, and explicitly test all eleven variants including wrapper children. This supersedes only the old fallback policy, not historical restrictions on trait methods/signatures. No match→if-let rewrite, suppression or threshold change may conceal V16.

## Text and shaped identity

`FrameItem::Text { pos, text, style, bidi }` is permanent pre-shaping L1 data. L3 converts it to TextShaped when a face is loaded; Text remains the Type1/unloaded-font fallback. The old deprecated annotation (including the intermediate semver correction) is historical and is removed, along with local allow(deprecated) used solely to compensate for it. Preserve the permanent role and existing exporter preference for shaped text; do not make L1 fabricate glyphs or move shaping out of L3. No carrier field, algorithm, default or phase changes through that metadata correction. A deny(deprecated) construction/read control must fail before and pass afterward; compare Rust bodies/data excluding comments and explicitly removed metadata, and retain fallback/shaping tests with and without fonts.

`ShapedGlyph` carries glyph_id:u16, x_advance:i32, x_offset:i32, y_offset:i32, cluster:u32 and char_code:char. `TextShaped` carries pos, glyphs:Vec<ShapedGlyph>, style, original text:EcoString, units_per_em:u16, bidi and font as specified below. Original text supports fallback/CMap and diagnostic extraction. L3 try_shape supplies `rb_face.units_per_em().max(1) as u16`; shaped PDF TJ uses `-(x_advance / upm * 1000)`. Exhaustive matches decide explicitly on TextShaped, using `..` only where a field is irrelevant.

Add only to TextShaped `font: Option<crate::entities::world_types::Font>`. Some is a clone of the loaded source face that produced glyphs and units_per_em, including productive cache hits and variable fonts. The entity already uses Arc; no mmap/read/OpenType parsing/World reference is introduced here. Persist no catalog slot, path, family name or World pointer as identity. It remains valid in another World/after the session with the same source bytes. Current normalized face index 0 is supported; new collection/nonzero-index support is outside scope. None preserves manual/legacy resolution; Text/TextSeparator get no font field and math Glyph remains outside this correction. Add no unsafe or global state.

Use source-face bytes plus effective merged axes, explicit coordinates winning by tag. Style remains semantically honest: do not change italic/bold/weight/semantic_emph/semantic_strong to force face selection. A coincident family/variant is not a valid GID domain. Clone/translation/reconciliation/fragmentation/wrappers preserve font; merging shaped items requires compatible face and axes in addition to prior criteria. Never infer the face from char_code/cluster/GID/later coverage. Subset/instance bytes do not replace origin identity.

L3 owners retain existing public entrypoints and FontKey, adapt internal bindings to original face+axes, and use the same association for collection/subset/CMap/PDF/outlines/SVG/PNG. PNG/SVG instantiate once per face/request before render with original resource indices separate from instance bytes. A local table lasts one export, keeps supplied-resource order and appends missing faces; distribute multifont items once before per-face collection, never share global GIDs/CMaps across faces. Private functions may receive face/bindings explicitly. A newly required public signature needs its own L0/gate; approval of this field alone cannot authorize it.

The shaped-font gate is explicit: `lab/history/construction/records/00_nucleo/materialization/identidade-fonte-shaped-aprovacao-2026-10-05.json`, human “Confirmo o L0; prossiga com implementação e medição”, recorded `2026-10-06T01:41:37.921093+00:00`, approved SHA `f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`. Apply the pinned shaped-face nucleus only through owning L0s. The implementation snapshot/inventory is in identidade-fonte-shaped-implementacao-2026-10-05/.

## TextStyle and math morphology

From StyleChain, a missing font supplies at least FontList("Helvetica"); the historical ≥95% shaping goal is not a current measurement. `variations: Option<FontVariations>` defaults None and transports explicit axes through StyleChain resolution and text merge (`text.variations` plus inheritance) to L3. L3 merges them with FontVariant-derived axes under `00_nucleo/prompts/infra/font_variant.md`.

`math: bool` defaults false and is set once at layout_equation entry, inherited through all math sub-layouts. Attach/frac/root/text sub-layouts adjust size/italic without overwriting font/math. The flag signals math fallback independently of whether a body face happens to contain a MATH table. StyleChain conversion explicitly uses false; text merges inherit layouter.style.math.

`math_script: bool` defaults false, becomes true at MathAttach's script_style and follows descendants so compute_gaps can suppress spacing_between in script context. StyleChain conversion is false; text merge inherits the layouter flag. The later discrete level does not silently remove this existing signal.

`MathSize { Display, Text, Script, ScriptScript }` defaults Text in TextStyle. Equation entry selects Display for block, Text for inline. Scale factors remain in MathConstants and are absolute to the base: Display→Text×1, Text→Script×script_percent_scale_down, Script→ScriptScript×(script_script_percent_scale_down/script_percent_scale_down), ScriptScript→ScriptScript×1. Attach scripts descend Display/Text→Script and Script/ScriptScript→ScriptScript. Frac/root/underover/matrix/cases keep the level honest during their respective descents; matrix/cases also correct the former unconditional scale. Do not conflate the discrete level with a new independent scale source.

`cramped: bool` defaults false. Force it for attach bottom-left/bottom-right/sub (not superscript, which inherits), frac denominator (numerator inherits), root radicand/index, and accent base (the initially modeled accent was only above). Attach compute_script_shifts reads the ambient cramped flag, not its internal script_style, and selects superscript_shift_up versus superscript_shift_up_cramped only when a superscript exists; do not change other formula terms. The old observation that math overline/underline were absent was a historical scope finding. Later callable owners exist; their own cramped/layout contracts decide subsequent behavior, rather than this history authorizing an implicit correction.

`math_text_item: bool` defaults false, true only for direct Content::Text with TextItem morphology in math. Valid true state requires math=true. It is internal provenance, never user configurable and never enters StyleDelta/StyleChain/parser/eval. Text/TextShaped and style merges preserve it without a new FrameItem field. MathIdent/MathText/numbers/GlyphFragment/stretch forms/promoted MathClassOverride remain false. True preserves math family/fallback/size/shaping but excludes singular-glyph italic correction and OpenType ssty in shaping and metric fallback, even Script/ScriptScript. False preserves applicable IC and .st/.sts behavior. Add no character list, nominal heuristic or redundant field.

`semantic_strong`/`semantic_emph` default false and derive exclusively from from_strong/from_emph provenance, not manual render weight/italic. They are not interchangeable with bold/italic. `faux_bold_stroke_pt(k)` returns 0 when weight.unwrap_or(400)<600; at/above 600 retain `((w - 400)/300) * size * k`. Thus math 450/medium 500 use real font selection, while genuine 700 without a loaded bold face retains existing 2Tr outline behavior (e.g.0.44pt at 11pt for the historical k). Type1 and verbose emitters keep consuming this helper. The accepted mechanical divergence does not alter the preexisting real-bold selection work.

## Glyphs, lines and paint carriers

Glyph carries pos, glyph_id, x_advance, size, style:TextStyle and base_char:char. style chooses the correct effective font context; base_char is the original requested stretch character, not a guessed Unicode reverse mapping for a variant/assembly GID. Both are needed: unlike shaped style, Glyph.style.font is not rewritten to the resolved face, so early resource selection must re-resolve with base_char plus math/size/variant context. Four stretchy/assembly construction sites already have both; all offset/translation/slicing/footnote/math reconstruction paths preserve them unchanged. No font bytes enter L1.

Line carries start,end,thickness,color:Option<Color>,cap:LineCap. Some color sets stroke color; None preserves prior black default. Decoration inheritance remains explicit stroke, then current text fill, then black (`stroke.or(self.style.fill)`). Cap survives translation/slicing/groups/reconstruction without recalculation. Legacy lines use Butt; matrix automatic augment uses Square and custom augment preserves its stroke cap. This public cap expansion is explicitly confirmed after preseal `d75cf13cd99cdd7a72e3a67e1aa04bfccbf2b4ca` in `lab/history/construction/records/00_nucleo/materialization/typst-passo-1431.md`, following the matrix gate confirmed on 2026-09-17. Its render promise is raster/PDF only; SVG/other targets accept and ignore the field for compilable exhaustiveness without changed output. Do not extend this into a cross-target cap claim.

Shape carries pos,kind,width,height,fill,stroke,fill_rule,parent_bbox_at_emit and its existing geometry. Fill-rule survives from ShapeElem; old emitters without an exposed rule supply NonZero, and exporters must not derive it from segment orientation. Earlier Color→Paint approval (2026-08-26) required whole Gradient/Tiling transport and allowed only the specified exporter fallback; conic SVG equivalence still needs a validated error budget, and unmodeled color-space conversion cannot silently become equivalence.

The later approved prepared-paint contract supersedes only Shape's old declarative Paint/Stroke requirement: fill is `Option<FramePaint>` and stroke `Option<FrameStroke>`. Preserve all other variants, PageFill, TextStyle, FrameVisitor and plain_text. Paint cells are not semantic page children. Apply the pinned tiling-frame-transport nucleus; no total From<Paint>, carrier PartialEq, solid fallback to obtain a green build, raised old limit or transfer of layout to L3. Validate fill/stroke/resource/clipping/affine Parent transport, native targets and Solid/Gradient/page-semantic controls. Preparation alone does not prove render.

Group carries pos,matrix,clip_mask,inner_width,inner_height,items and `paint_root: bool`: true establishes a local root of those inner dimensions, false inherits. Technical groups must not reset Parent paint. Clone/reconstruction/translation preserve it, with existing traversal and geometry. Box background/body and Block clipping retain their own owner-defined behavior; general measured Rotate/Box geometry differences remain open.

The human approved only T1–T3 in manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`, receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. The initial paint-root-only stage was explicitly incomplete; the later Shape migration is authorized but T1c/T2/T3 close only after integration and verification. T4/publication remain outside this package. A partial/broken intermediate build is not production certification.

## Images, links and semantic envelopes

Image carries top-left pos, Arc<Vec<u8>> raw data, physical width/height in points, intrinsic pixel dimensions for PDF XObject /Width,/Height, EXIF orientation 1–8 for its cm transform, and `alt: Option<EcoString>`. None differs from Some(""); preserve alt through layout/shaping without changing geometry/image bytes. Link carries rendered items, LinkTarget, pos and clickable size. Url uses PDF /Link URI action; Destination(Label) uses GoTo/D and named Dests. Preserve ordinary body rendering. Link color/underline decoration remains outside this contract pending its own decoration carrier.

Semantic is transparent visual data with kind, placement Inline/Block, alt and child items. Formula preserves empty Some("") versus absent/none None. Bounds/plain_text/visitors/shaping/visual export recurse; no intrinsic ink/measure. Actual MCIDs/ParentTree/StructTreeRoot belong to L3; ActualText or a content-stream comment is not tagging. ExplicitLinebreakBoundary is transparent/no alt, one positioned empty Text at the closed line's baseline, no text/ink; it prevents reflow across an explicit linebreak. ParbreakBoundary is distinct, Block placement, one empty Text at the terminating baseline and no glyph/space/height/accessibility content. Consumers may share a reflow-barrier action without erasing the line-versus-paragraph distinction. Baseline distances or a 1.5×height threshold cannot recover the lost cause.

Semantic table vocabulary carries Table summary, Row, HeaderCell(level/scope/rowspan/colspan), DataCell(rowspan/colspan), THead and TBody, permitting direct TR under Table for explicitly mixed rows. Repeated headers across pages preserve one logical identity with physical per-page exporter links; level is transient relational data, not an emitted numeric attribute. Preserve transparency and child traversal through bounds/plain_text/shaping/transforms/layers. L1 allocates neither object IDs nor MCIDs and writes no PDF. Detailed table/a 11y owners retain their own approval/effect boundaries, including any unresolved summary-gate receipt; this vocabulary is not an independent ratification.

SemanticKind::Label is the closed inline transparent envelope for outline-prefix PdfMarkerTag::Label. Preserve prefix items without adding text/measurement/Location. Its productive effect is tagged PDF; it is neither a named-destination LabelElem nor Artifact. Outline's canonical owners retain the separately recorded prefix/page approvals and their bounded scope.

## Text replacement and separators

`TextReplacement { text: EcoString, items: Vec<FrameItem> }` carries extractable text for one complete occurrence and already positioned visual children. It has no independent position/transform/font/width/height. Bounds/metrics/translation/clipping/reconciliation act on children exactly once; resources/shaping inspect visual children while text adds no glyph/advance/measurement. Initial automatic hyphenation uses ordered Text/TextShaped children; a math Large operator may use positioned Glyph parts. Do not nest TextReplacement.

A selected global discretionary hyphen draws '-' but carries U+00AD; incremental layout may wrap the already measured whole prefix and replace it with lexical prefix+U+00AD to preserve kerning. A private explicit cause comes from selection, never inferred from suffix '-', position or GID. Literal '-' remains ordinary U+002D Text. This envelope adds no candidate/penalty/SHY-recognition policy. The later internal authorial-SHY correction expands identity only to already shaped runs with an actual U+00AD cause: preserve original text through occurrence replacement, TextShaped children, no nesting/new advance/candidate. Explicit authorial-SHY break selection and fallback without a face remain outside that correction.

Shaping/fallback process children while preserving the envelope; merging/reflow must neither merge it into external text nor erase it. Group/Link/Semantic/page layers retain occurrence order. Resource visitors recurse without a new public trait method; extractors use replacement text once rather than extracting children again. plain_text remains approximate. SVG visual export uses children; semantic HTML retains its separate pipeline.

PDF uses occurrence ActualText independent of tagging and Compact/Verbose, UTF-16BE with BOM and balanced BDC/EMC at top/local levels. Children retain visual GIDs/advances/ToUnicode, including literal '-' sharing a discretionary GID; Type1/embedded fonts preserve identity too. ActualText does not fabricate MCID/structure tree/formula Alt/accessibility certification, and Semantic.alt is not a replacement mechanism. The L3 replacement handler has its own 1:1 owner and static exhaustive dispatch. The owner confirmed “confirmo” after proposal SHA `b744f80e801eceddefadc76e79cf343ba19174d65ad84f53a2c4b9ccaebc1870`; authorization/UTC are in the identity batch authorization.json. Apply the pinned text-replacement nucleus.

`TextSeparator { pos: Point, advance: Pt, style: TextStyle, origin: TextSeparatorOrigin, bidi }` represents each retained U+0020. Public `TextSeparatorOrigin { Markup, Newline, Literal }` distinguishes normalized Space, SpaceFromNewline and each literal Text-adapter space. pos starts its causal baseline advance, advance is the physical assigned width, style retains font/style at that point. This is data, with no render method/compiler import. It creates no ink, line/baseline/ascent/descent/height or shape/image font; do not add its width to the cursor twice. Preserve leading-space suppression and wrapping/justification. Text and Space emit the same carrier class while retaining their local advance/flush/origin rules.

Translation/rebase/direction/shaping, Group/Link/Semantic and suffix rollback preserve causal occurrence order. Carrier addition is append-only; never mutate a prefix already certified by a length checkpoint. Do not infer identity by equal-content lookup or any positive glyph gap. L3 emits the extractable space with the prepared font/pos/advance and its resource owner resolves/subsets/maps it. Existing visitor text hook transports the carrier with no new trait method; plain_text skips it to avoid duplicate approximate spacing. The owner explicitly authorized this public variant+origin enum with “Autorizar o contrato proposto” on 2026-09-28, recorded in materialization provenance. Every affected consumer still requires its own current L0 before code.

## Optional bidi context

Only Text, TextSeparator and TextShaped gain `bidi: Option<crate::entities::bidi_context::BidiSpan>`. Entity validation belongs exclusively to `00_nucleo/prompts/core/entities/bidi_context.md`. Add no TextStyle field, FrameItem/SemanticKind variant, visitor method or FontMetrics trait method. Preserve separator origin and all other fields; the later Text metadata correction independently supersedes the old requirement to retain deprecated. Uncertified producers, math and legacy/manual frames use None. Typst/CLI defaults and phases remain unchanged. Debug/pattern/struct-literal migration is a real public Rust contract change, not automatic source compatibility.

The option is logical context only, with no own text/controls/ink/width/font/baseline/space. Clones/translations/envelopes/visitors/diagnostics retain it without extracting auxiliary text. Shaper Text→TextShaped preserves it; splits produce validated subspans using the entity's logic, not a duplicate validator. Plain text/PDF read leaf text and separator origin, never bidi input as an extra text source. Ratification is “confirmo”, manifest `1ae7513eb3af426754909954aef165c97257a9f62d3911fbb111e2002fac4043`, receipt `lab/history/construction/records/00_nucleo/materialization/bidi-contexto-aprovacao-2026-10-05.json`. Frozen implementation before.json.gz preserves the approved versions; later code hashes do not replace receipt evidence. Optional-carrier transport does not by itself certify broad paragraph/bidi/shaping geometry beyond each owner’s tested slice.

## Page configuration and document snapshots

PageConfig transports width/height in points, per-side margins, typed numbering, columns and the page geometry/canvas/running/supplement data defined by canonical owners `00_nucleo/prompts/core/entities/page_geometry.md`, `00_nucleo/prompts/core/entities/page_canvas.md`, `00_nucleo/prompts/core/entities/page_running.md` and `00_nucleo/prompts/core/entities/page_supplement.md`. Page dimensions are trim size; bleed is separate physical data. Geometry owns PageBinding and folded margin specs, resolving physical per-page snapshots and applying flipped exactly once to the delta. Canvas owns folded bleed/PageFill/background/foreground; Page keeps physical bleed/fill and separate background/body/foreground vectors. Text/semantic visitors inspect body only; visual export uses all three.

`PageMarginSpec` has public left/right/top/bottom Option<f64> (None means auto for that side); resolved `PageMargins` has four f64 sides. PageConfig.margin is PageMargins, replacing the old scalar and margin_is_auto. The old flag's semantics survive per side: default auto recalculates from effective dimensions on SetPage size changes, explicit fixed sides remain fixed until explicitly changed, omitted margin preserves its specification, uniform explicit margin sets all sides. Do not silently choose one side through a scalar alias. Default sides use auto_margin, formula `min(width,height) * 2.5 / 21`, with normatively derived Paper::A4 dimensions (no baked 595.28/841.89 or point fallbacks); when both dimensions are infinite use normative A4. The historical A4≈70.87pt and 200pt-small-side≈23.81pt examples illustrate the formula, not fixed defaults. Page stores physical margins, not Paper. The original “future canvas properties absent” staging note is superseded only by their later explicit owner contracts.

Running matter fields are number_align, header, header_ascent, footer, footer_descent, defaults center+bottom, Auto,30%,Auto,30%; marginal content is a separate visual layer. PageConfig stores PageSupplement; Page stores resolved Content, with validated supplement/pages/numberings vectors aligned 1:1. PageConfig.numbering and Page.numbering are `Option<entities::numbering::Numbering>` rather than EcoString; None means disabled and preserves callable Numbering semantics. The numbering public/phase contract was approved in the historical numbering gate; the Page source-span carrier for callback error remains separately specified, not implicitly added by page-label work.

`Page.logical_number: usize` is the final validated logical number independent of physical/export-selected index. L1 constructors use physical number before a realized store exists; L3 replaces it 1:1 from final PageStore before returning the document. Direct-export Page construction states its logical number explicitly. Preserve plain_text/canvas/visual content/counters/selection; do not add the separately excluded numbering source span. The initial proposal was ratified: `lab/results/construction/diagnostics/typst-p1486-page-labels.md:164`, implementation addendum 2026-09-24 11:18:44UTC, dirty HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, explicitly records the owner's public-field confirmation. Its paired 1+3 probe produced reader labels 10,12, refuting physical-index sufficiency. Painted page-number differences remained outside this correction. A mismatch between final realized store and Page snapshot refutes closure; do not claim full numbering parity from reader-label tests.

PagedDocument retains extracted_label_pages:HashMap<Label,usize> (1-based), extracted_label_positions:HashMap<Label,Point> at insertion for PDF Dests, figure/table page-number vectors in counted document order carried through fixpoint, extracted_headings:Vec<(Label,Option<String>,Content,usize)> (auto-label,number,body,level) copied from headings_for_toc for PDF Outlines, and document_info:DocumentInfo copied from Module for PDF Info. Empty collections initialize in new and finish/pipeline populate them without changing layout() signature.

`format_options: DocumentFormatOptions` defaults through Default, preserving pretty=false, then L3 replaces it only after successful pure extraction from final content. It contains no CLI flag/serialized bytes/exporter handle. `00_nucleo/prompts/core/entities/document_format_options.md` owns the type; this owner only transports it. Page/PageConfig/frame geometry/plain_text are unchanged by that addition, explicitly approved/materialized 2026-09-19.

`document_language: Option<Lang>` initializes None. Some is the first effective document language before emission, independent of painted glyphs; with None L3 applies only the measured PDF/UA default en, never a fixed dc:language inference. The no-text-fr fixture disproves glyph-based detection. This field, semantic_strong/emph and Image.alt were explicitly confirmed 2026-09-23. Historical veraPDF1.30.2 fixture 102 pass/4fail versus 106/0 is evidence for the slice, not universal PDF/UA certification. No new mode/trait/eval-layout phase/dynamic dispatch follows.

## Proposed attachment and artifact carrier: approval provenance gap

The archived public proposal adds `PagedDocument.attachments: Vec<Arc<PdfAttachElem>>` in document order, collected from invisible layout markers, with no visual FrameItem and ignored by plain_text. It also adds `SemanticKind::Artifact(ArtifactKind)`, preserving visual children with alt None, no structural-tree participation/MCID and normal child resource traversal. Formula cannot substitute for Artifact, because that would emit Formula/MCID incorrectly. Eval produces markers, layout collects/wraps and export consumes PagedDocument with no new phase. PdfTags::Enabled remains default; Disabled preserves visuals without tags.

The public field/enum proposal remains bounded by ADR-0127 confirmation. No explicit post-gate attach/artifact receipt was located in this review of the owner and historical `lab/history/construction/records/00_nucleo/materialization/typst-passo-1286.md`; retain the full specified contract and approval-provenance gap, without inferring ratification from code/later callers. This gap is not an instruction to remove the listed current behavior. The later V16 maintenance cannot materialize a historical pending proposal.

## Pure numeric conversions

Angle remains radians internally. Existing to_rad returns that float, to_deg its degree equivalent; angle.rad/angle.deg reuse these causes, distinct from Rust Angle::rad/deg constructors. Historical HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, layout_types.rs:1277–1290 and upstream angle.rs:142–151 support this. Preserve finite values, signed zeros, infinities, representation/signatures/arithmetic/construction; create no duplicate module/formula. A measured precision mismatch requires correcting the cause, not arbitrary repr rounding. Angle NaN retains its already authorized conditional resolution; do not alter arithmetic to fabricate a bilateral receiver.

TransformMatrix::rotate uses pinned pure libm 0.2.11 cos/sin; skew uses tan. Preserve Y-down signs/coefficient positions/concatenation/signatures, zero translation and current nonfinite policy. No new transform representation or suppression. This internal continuous-flow correction preserves geometry/language tests, compares the measured SVG examples before/after with declared differences, and does not certify every platform/case.

## Acceptance and remaining limits

Keep Pt arithmetic/compile-negative controls, frame/document joins, per-side automatic/fixed margins and canonical page-owner boundaries. For every transport change, update each 1:1 owner before its implementation, establish meaningful RED→GREEN, preserve existing geometry/text/effects/diagnostics and restrict resealing to valid V15/V26 ownership. Do not infer rendered acceptance from helper fixtures, item counts, build success or Rust equality.

TextReplacement requires global/incremental SHY extraction and coexistence with literal '-' even sharing font/GID; test simple/optimized, justification, overhang, styles/fonts, groups/links, Type1/embedded, Compact/Verbose and tags on/off, visitor/translation/fallback behavior, comparing geometry separately with unchanged breaks/advances. Authorial-SHY identity requires causal shaped-run tests; general authorial break selection/compound policy and no-face fallback stay open.

Separators require source→PDF RED/GREEN for `A B C`, `A  B   C`, `#text("A B C")`, with `A#("B")C == ABC`. Cover styles/fonts/newline/line-region boundaries/justification/tracking/kerning/RTL/callbacks/suffix rollback/groups/links and visible geometry/diagnostics/text retention. Invented-carrier helper tests cannot replace the full source route. Bidi optional-field migration retains certified-versus-None producer boundaries and validated splits; it does not extend the certificate to untested consumers.

Face identity requires regular/italic controlled catalogs, Arabic/Latin emphasis, semantic markers, cache miss/hit, two faces with identical family/variant but different coverage/GIDs, cross-World slots, legacy None fallback and variable-font derived/explicit override axes, including two requests in one document and instantiation. Static-face tests alone do not certify VF. Exercise wrappers/page layers, inspect paired PDF ink/text/geometry and effective-face SVG/PNG, retaining Latin regular/italic, Arabic regular, strong, math and absent-resource controls. Copy no vanilla defect and promise no speed gain.

Prepared paint requires complete integrated fill/stroke/resources/clipping/Parent affine/native-target/Solid/Gradient/page semantics and its pinned invariants; initial metadata-only or prepared-cell success is incomplete. Public shape fallback/conic/color-space restrictions remain. Page labels test final snapshot alignment and distinguish reader labels from painted numbering; attachment/artifact/summary provenance gaps remain explicit.

V16 maintenance preserves test fixtures/expected outputs/tolerances/counts/Unknown failure, all callbacks/effects/metadata/clipping/font/position/paint/width and informative V19 alternative notes. One shared arm's execution is not coverage of every nominal alternative. Before code run new FrameVisitor/workspace controls unchanged and establish AST RED of 70 warnings; afterward require no covered-family V16/new warning and retain external V16 debt. Disposable sentinel-enum probes for cargo check -p typst-core measure absent E0004 before/present after only at compiled productive-core sites; do not extrapolate to cfg(test) or blocked L3. Replay patterns/guards/bodies, unchanged enum/signatures and real tests/build, never stubs.

Revalidate full workspace/build/all targets, core/shaper/fallback tests, strict all-target Clippy with -D warnings and -D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings/four HTML indexes, strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze source/L0/config during commands, record preexisting warnings and exact state/fonts/binaries/UTC for CLI samples. Heavy commands run sequentially, MemoryMax 12GiB, MemorySwapMax 0, CARGO_BUILD_JOBS1. Internal metadata/numeric/exhaustiveness work uses ADR-0127 continuous flow without new public contract/default/compatibility/phase; public changes require their specific bounded receipts. Structural cleanup does not establish global parity, memory gain or measured AI benefit.

## Historical provenance

The complete original owner, including initial stubs, superseded margin/deprecation stages and bounded proposals, is archived at `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/layout_types.md`. Temporary construction records remain under `lab/history/construction/records/00_nucleo/materialization/`. Their numbered history preserves provenance, not the organization of this enduring owner contract.


## R8 — resolved native superscript carrier (PUBLIC CONTRACT APPROVED)

Status: **APPROVED FOR MATERIALIZATION**. The human approved exactly
TextStyle::superscript_native: Option<bool>, default None, from the saved draft
SHA-256 4b14829d1eb38475d1985a9707a6e6ce40166196dfe524afe512d99b9cabf4d0.
The explicit answer and UTC are recorded in the R8 native-carrier-approval.json.
The original FontMetrics method approval remains separate; no other public
addition is authorized. All tests-first, reseal and complete validation gates apply.

### Measurement before proposal

The R8 evidence root is
`lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/`.
`candidate5-source.json`/`.json.gz`, candidate-debug5 and candidate5/receipt.json
bind the exact dirty source, UTC, inputs and binary. All ten original note cases
pass complete physical PDF word/page boxes at0.001pt; svg-candidate5/receipt.json
also passes glyph origins and separator geometry. The six font/size controls are
recorded separately in fonts-candidate3/receipt.json. These are finite controls,
not universal text-shaping certification.

`numbering-candidate5/receipt.json` contains twelve real additional note probes.
For Nimbus Sans11pt, numbering `1 Z`, the call digit is x48.127–50.3248 while the
fixed oracle is x48.127–51.7966:1.4718pt disagreement. Its entry digit is
x29.35–31.21813 instead of x29.35–32.46916. At17pt the maximum call disagreement is
2.2746pt. DejaVu/Liberation and the refused native `(1) Z` control pass the same
unchanged tolerance. The fixed oracle remains commit
bd5ebec285505889e8ff1d4f68804ab147a699f3, tree
cac836c07e4dec2a89219370428292393a3216b5, binary
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.

`text/style.rs::for_text` receives the approved method's native=false for the
complete marker, sizes/shifts synthetically, but retains only geometric values.
`text.rs`/word_layout split the marker into words. `shaper/shaping.rs` sees the
partial `1`, whose face supports sups, and applies a native glyph again. The
semantic superscript flag cannot encode this separate choice: synthetic text is
still superscript. Baseline/size values, MATH flags, font names, private strings
or invented source identity must not become hidden feature-choice channels.
`red-cache-multiline.json` independently records warm/cold glyph/advance
inconsistency and its authorized cache correction; that cache fix does not
transport a discarded layout decision.

### Exact proposed public addition

Add only this field to the existing public TextStyle:

```rust
pub superscript_native: Option<bool>,
```

This is resolved render metadata, separate from semantic `superscript`. It is
not a Typst source property and is never added to StyleDelta, StyleChain, parser,
eval, CLI or a new mode. Default and StyleChain conversion initialize None.

- None: preserve automatic face/run coverage detection for unresolved/manual
  styles. Ordinary and mathematical text remain None.
- Some(false): L1 resolved synthetic script sizing/offset. Width measurement and
  painting must not reactivate sups on a later word/subrun, even when that shorter
  text alone is covered.
- Some(true): L1 selected native sups, with the approved outer-size/zero-offset
  policy. L3 requests the feature while preserving actual face/variation/complete
  run verification. No second scaling or shift.

Preserve `superscript` as true for both synthetic/native superscript and retain
all semantic strong/emph/math provenance. Clones, effective-style merges,
fragments, Text/TextShaped/Glyph style transport and reconciliation preserve the
resolved choice. A new source style starts unresolved; do not synthesize a
choice from glyph IDs, later coverage or coordinates.

This owner declares only the pure data/default/conversion contract. Actual
caller population belongs to core/compiler/layout/text/style.md; advance/width
and shaping consumption belong to the existing infra font_metrics and shaper
owners; their cache keys must include effective feature choice where it affects
results. They retain their own1:1 L0s and require conditional L0 updates before
materialization. The already-approved FontMetrics signature stays unchanged.
No other entity/FrameItem field, enum variant, trait method, public signature,
dependency, global cache, source replay or pipeline phase is proposed.

### Gate, compatibility and acceptance

This is a public entity-field addition, and Rust callers that spell every
TextStyle field without `..Default` must include the new field. ADR-0127 STOP
therefore applies; the earlier trait-method approval does not cover this field.
The exact saved addition and hash are now explicitly approved as recorded above.

After approval: tests first must show false surviving complete-marker→word→width
and paint, true retaining native glyphs, None retaining automatic behavior,
borrowed provider fidelity, cache separation and semantic superscript identity.
Require the twelve numbering/font/size refuters to pass0.001pt PDF/SVG; preserve
the original ten note cases, six font controls, literal brackets, source/counter/
callback once, nesting, split/reentry/column/rollback and refused controls. Run
complete workspace/build/tests/Clippy/fmt/Rustdoc, strict and reverse lineage,
full lint and A–H/R1–R7 preservation. Keep arbitrary mixed-face shaping, opaque
carrier migration and literal-newline flow limits explicitly measured; this
field alone does not certify those broader scopes.
