# Prompt L0 — StyleChain
Hash do Código: f7bcdf66

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:c8570ec5f4f32c0805587230cafc796f35e5ee7c9f8e634e13d807db2afb6e00

**Layer:** L1, pure RAM; no system I/O. **Exclusive consumer:** `01_core/src/entities/style_chain.rs`.

## Evidence before the current contract

Historical style audit2026-05-15 in `lab/results/construction/diagnostics/diagnostico-text-fase-a-passo-266.md` verified real StyleDelta/resolver/consumer wiring. The original inert annotations on weight/tracking/leading/lang were already obsolete: actual consumers included PDF Tc/cursor advance, line-height leading, language hyphenation/quote localization, and faux-bold/PDF2Tr. Its13/13 structural entries,100% subsystem coverage,four promoted consumers and aggregate≈86% Text coverage describe that snapshot,not current universal language parity. Additional subscript/superscript/highlight and baseline_offset were already in the bridge. Revision chronology and old ADR references remain in the archived original; current ADR0107/0108/0129/0131 govern interpretation.

Historical upstream `e0e8ca4d`,text/mod.rs:180–182 declares FontList([Libertinus Serif]),:141–144 documents font-path→system→embedded priority. Embedded-family evidence is owned by `00_nucleo/prompts/infra/embedded_fonts.md`. On2026-08-13,pdffonts for one-line Ola mundo/no set text showed oracle `/usr/local/bin/typst`(e0e8ca4d)SGDIWK+LibertinusSerif-Regular-Identity-H versus local release at HEAD4f64e4e69/docs-only dirty tree AAAAAA+LibertinusSerif-Regular;both CIDType0C/embedded/subset. Family,face and embedding matched;subset tag/Identity-H spelling differed mechanically. This corrects historical exact-byte claims,not a new measurement against currentbd5ebec285505889e8ff1d4f68804ab147a699f3. Earlier FreeSerif default was replaced after accent decomposition/CFF subset defects,then Liberation Serif,then embedded Libertinus Serif;only the final default/fallback below remains active. Provenance:lab/results/construction/diagnostics/paridade-producao-p753.md and archived owner.

style_chain.rs:779–784 called collapse solely for bold_from_strong/italic_from_emph,copying custom fields unnecessarily. table-local-scaling-followup-2026-10-09/stacks-after.json.gz,releasea01e3854b67abf7ec7902e14e0a695a3ea425a91ad13aab464247aade98b8491,records801collapse allocation samples under layout_styled/local-set-1600,64requests/sample. semantic-red-gate.json.gz records1067883requests in restored binary at HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b/uncommitted tree/UTC. Direct-bit resolution saving clones is a refutable inference,not assumed gain;allocation/precedence/bilateral output+time controls may reject it.

performance-c2-2026-10-07/table-local-regression.json/three r*-table-local samples freeze source/argv/UTC/state/releaseac889581/historical9f2b6e871:font grows10%,second columnx60.880→38.999/overlap;table-local-c2r1-2026-10-07/red.log10pt/Auto gap fail while explicit11pt passes. Upstream eval/code.rs:32–43 applies declaration styles only;local expression_dispatch:132 collapses defaults(size11/boldfalse/italicfalse at style_chain:304–315),tracks:328 misses standalone-complete Cell,measurement lacks Cell visual. Joint cause is an inference refuted if source/font/size/position still differs after lexical isolation/complete visual measurement. This owner only supplies lexical-prefix extraction,not table measurement.

style-font-capacity-2026-10-10/before.json.gz preserves HEAD/tree/files/diffstat/sources/baselinebb138eb8d2f8a5762ed693bfe2dda526c092780a47e0dc4c5e8e4f7af35b81a8. Earlier style-font-resolution-reuse memo was discarded:less requests gave no consistent speedup/higher RSS;no such cache exists here. style-scope-transfer-2026-10-10/stacks-current.json.gz has400inclusive periodic480-byte vector samples under font/local-set1600;capacity-probe.json measures FontFamily120bytes,filter_map/collect onefamily len1/capacity4/buffer480. Synthetic probe validates type/iterator,not time/exclusive cost. style_chain:641–653 has zero iterator lower bound;exact effective-string reservation benefit remains refutable by requested bytes,RSS/time,no upstream-intent/PDF-byte requirement.

text-argumentos-2026-10-17/before.json.gz,before-measurement.json.gz,morph-before-measurement.json.gz,audit.json preserve HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/UTC/status/diffstat/source/L0/binaries/argv/historical9f2b6e8715237cb086899a42873660fe744622e8. named-em0versus24pt,affine3versus27,nested18→0versus36;six Size/Color/body orders rejected locally/accepted upstream,positional18pt ignored12versus18. Pinned text/mod.rs:290,303,924–930,1136–1142/args:178–247 typed search/named priority/restbody/affine fold versus local em loss at style_chain:513–517/constructor:123/top-wins collapse:394–395,419–422. Restoration/Raw/heading/math/capture-show/duplicates-spans/zero-negative/metrics refute incomplete correction. Upstream text2em==text24pt compares Styled child(content:779–782):equality defect needs its own correction,full Length is no equality closure.

Human approved manifest771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba/two public contracts,S1–S16,12GiB/swap0,no release/merge/push,seal after implementation/valid graph;RG1 independent. This narrowly supersedes scalar-size/top-wins-size historical clauses;other properties retain their rules.

## Persistent style representation and resolution

StyleChain is an immutable linked list of style deltas. A set rule affects subsequent content only;children inherit unspecified properties. It replaces the former flat bold/italic/size representation. Preserve public StyleDelta field types:

```rust
pub struct StyleDelta {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub size: Option<Length>,
    pub fill: Option<Color>,
    pub heading_level: Option<u8>,
    pub weight: Option<u16>,
    pub tracking: Option<Length>,
    pub leading: Option<Length>,
    pub lang: Option<Lang>,
    pub font: Option<FontList>,
    pub subscript: Option<bool>,
    pub superscript: Option<bool>,
    pub highlight: Option<Option<Color>>,
}
pub struct StyleChain(Option<Arc<StyleNode>>);
```

The listing preserves the original named field contract;later existing custom/provenance channels remain governed below rather than silently removed. StyleDelta::empty defines nothing;default_chain sets bold=false,italic=false,typed size11pt. push is O(1) with shared ancestor;clone is O(1),cloning top Arc only. For ordinary properties access traverses inner→outer,first defined wins,then accessor default. collapse is read-only over every node and retains Option presence/custom first occurrence per key. Show-set captured above StyleChain::empty contains only actual declarations,no default_chain values. Size is the explicit affine-fold exception,not ordinary top-wins.

StyleDelta.size retains full declared abs+em Length;size() remains resolved f64 points. Fold typed/custom text.size lexically,typed wins in same node. collapse and local_delta_since preserve composed affine function without default completion when ancestor excluded. Diff/bridge read full Length;Pt appears only in resolved render. A captured prefix containing only custom text.size remains custom,never promoted to semantic typed presence;with a typed Size declaration the composed result stays typed. This preserves constructor presence/set's historical morphological transparency,no PartialEq Styled/equality fix implied. Across nodes contributions fold;other fields top-win. Test different-ancestor roundtrip,excluded base,explicit Size equal to default.

## Bridge, fonts and causal provenance

From<&StyleChain> for TextStyle bridges to flat resolved layout/export style. If no font defined,set Some(Libertinus Serif),which typst-assets provides with New Computer Modern/New Computer Modern Math/DejaVu Sans Mono via embedded-font owner. If unavailable(e.g.empty configured FontBook),shaper keeps DEFAULT_FALLBACK_FONTS_SERIF order Liberation Serif,DejaVu Serif,Bitstream Vera Serif,FreeSerif last,to avoid known accent defect. Family parity does not demand subset-name equality.

Bridge always sets math=false:equation layout entry sets true afterward. It always sets math_script=false:math attach's script_style construction sets true. It sets math_text_item=false:only mathematical layout's direct Content::Text emission marks TextItem. These are entry/construction provenance,not style-chain/configurable properties;no new StyleDelta/parser/eval channels. All other fields resolve normally. Canonical owners:`00_nucleo/prompts/core/entities/layout_types.md`,`00_nucleo/prompts/core/compiler/layout/equation.md`,`00_nucleo/prompts/core/compiler/math/layout/attach.md`.

StyleChain::variations()->Option<FontVariations> reads eval-validated custom text.variations Value::Dict inner→outer and folds with FontVariations::fold(inner wins tag,outer tags survive),matching historical text/mod.rs:846–850 #[fold]. Bridge consumes TextStyle.variations. No typed StyleDelta variations field;constructor/set converge on sole custom channel,as other text.<field> transport.

Morphological bold_from_strong/italic_from_emph already distinguish *strong*/_emph_ from set text(bold/italic:true);historical corpus QDF Strong/Em nodes and2026-09-23 approval in layout_types authorize semantic_strong/semantic_emph. Bridge walks inner→outer and takes first definition of each bit,stopping when both found/end;absent=false,Some(false) overrides outertrue. Do not call collapse in bridge solely for provenance;public collapse still complete for show-set. Do not change bold/italic render or infer semantics from weight/appearance. Representation/API/default/phase unchanged;require same language/raster and measured local-scope allocation/time reduction.

## Lexical-prefix and custom-font helpers

Crate-private local_delta_since returns only nodes pushed after a proven shared ancestor,or None if ancestor not in chain. Internal Arc identity proves lexical boundary only,never exposed to callers/telemetry or replaced by value comparison. Explicit size11/boldfalse survives even equal to inherited default. Same-chain ancestor→empty delta;no ancestor/default copy. Preserve existing representation/public API/purity/O(1)clone;size uses affine rule above,other deltas top-win. Test shared ancestor,equal-default declaration,nonancestor. Internal parity continuous flow,L0 first/RED→GREEN/scoped lineage/builds/suites/lint/physical+timing recollection.

For text.font custom Array,count Value::Str entries,reserve exactly that many FontFamily slots,fill in existing order/normalization via FontFamily::new. Zero strings allocates no buffer/returns None and stops inheritance as before. Typed delta.font precedes custom within node;custom first occurrence applies;nonArray ignored. Do not change legacy dict/regex,defaults,StyleDelta/public APIs/phase/cache. Two local Array scans only;push/nodes/chain unchanged. External RED→GREEN requires strictly fewer requested bytes/font controls with no allocation-count growth;separate mechanics from language. Preserve order/mixed-empty-invalid/precedence/typed regex/original declarations/text-markup-query-font-position-page-raster suites. Paired equivalent argv time/RSS without interposer and short/no-consumed-declaration controls;repeatable regression or changed observables refutes/requires exact prior source+L0 restoration. Fewer cumulative bytes proves no lower resident memory.

## Verification, instrumentation and scope

Test default11pt/boldfalse/italicfalse,push propagation/inherited None,O(1)clone,correct bridge/defaultfont,full13named resolver coverage and existing sub/super/highlight/baseline_offset transport. Historic coverage counts are archived evidence,not fixed modern acceptance totals. StyleChain exposes no allocation identity or p1339_observation-conditioned method;persistent storage/defaults/Arc/resolution/productive APIs remain telemetry-independent. No I/O or global mutable state. Font coverage discovery remains deferred to its owning layer;this owner does not certify all text/font equality/render behavior.

Archive: `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/style_chain.md`.

## Raw causal storage traversal

Measured 2026-10-19 in the uncommitted Figure/Grid working tree: private
`StyleNode` at style_chain.rs:275 stores exactly `delta` and `parent`;
ShowRecipeContent captures a StyleChain, while observation/content.rs:38
unconditionally returns Unproven for recipes. Integrated RED8 preserves the
actual own-hidden Figure query failure and identifies both captured body and
supplement as Unproven. Raw storage, rather than a resolved cascade, is needed
to prove the complete causal representation.

Provide only a crate-private borrowed `raw_deltas` iterator, head to parent,
with complete StyleChain/StyleNode destructuring. Yield every stored delta,
including empty and shadowed declarations, without allocation, flattening,
resolution, callback execution, or Arc identity shortcuts. Empty chains yield
no items. The iterator exposes no public allocation identity or telemetry API;
ordinary style resolution and public contracts remain unchanged. Test borrowed
order, empty-node presence, and empty-chain termination. The observation owner
compares the returned deltas and retains opaque Unproven values.

## R8 approved resolved carrier bridge
Measurement: src/entities/style_chain.rs:827–855 constructs the full resolved TextStyle. The approved layout_types.md R8 contract adds superscript_native: Option<bool>. From<&StyleChain> must initialize None: this is resolved render provenance, never a new StyleDelta property. No public StyleChain signature changes.
