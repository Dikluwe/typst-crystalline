# Prompt L0 — entities/module
Hash do Código: 97a29ce1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/module.rs`
**References:** ADR-0017,ADR-0023.

## Module contract

Module(Arc<ModuleInner>) is an evaluated file, with O(1) shared clone.
Preserve public new(name:impl Into<String>,scope:Scope)->Self,
name(&self)->&str,scope(&self)->&Scope,
realization_introspector(&self)->Option<&TagIntrospector>,
document_info(&self)->&DocumentInfo and set_document_info(&mut self,DocumentInfo).
Use String names/domain FileId. The original omitted-Content stage is obsolete.

ModuleInner privately carries name:String,scope:Scope,content:Option<Content>,
introspection_content:Option<Content>,
realization_introspector:Option<Arc<TagIntrospector>>,
bib_styles:HashMap<u64,Arc<IndependentStyle>>,document_info:DocumentInfo,
and the private kind below. Visual content retains associated provenance;
introspection_content retains realized semantic graph with consumed origins/
products so query finds originals transformed by show. CSL styles resolve at
eval; document metadata transports eval→pipeline→PDF Info.

## Single evaluation and memoized index: evidence

module.rs:39,97-104 already carried introspection_content; old eval/mod.rs:872-889
filled it by a second execution. Ratified single realization supersedes that
mechanism: never choose trees by cardinality or reevaluate to fill a carrier.
At HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b in the single-evaluation tree,
realize/mod.rs:197-200 built TagIntrospector, eval/mod.rs:888-896 discarded it,
and paged.rs:242,html.rs:102,query_helpers.rs:478,519 rebuilt initial indexes.
`lab/results/construction/diagnostics/dupla-avaliacao-implementacao-2026-10-05.md`
and receipts preserve provenance. This proves redundant work, not measured gain.
Content equality/hash project Realized to visual and cannot key origins/
Locations/provenance; acceptance concerns queries,identities,causal order,
counters,contexts and geometry.

The snapshot field initially is None. Its crate-private setter
set_realization_introspector(&mut self,TagIntrospector) requires the same unique
Arc as existing mutators, using the canonical entity type. Snapshot identity
is its evaluated Module, not text/raw addresses/files/Worlds/global caches.
Arc preserves lifetime and O(1) Module clone; no unsafe/I/O/clock/globals or
changed Content/Module equality/hash.

When set_content or set_introspection_content actually mutates a unique inner,
invalidate the index before publishing content. Shared clones stay immutable.
Publish snapshot only after all content adjustments/setters. Unrealized
modules keep None and callers use prior paths. Snapshot contains no final
pages/positions,later-attempt callback results,coordinator-injected bibliography
styles or later expanded content. Coordinators clone before mutating attempt
stores, never mutate the stored Arc; metadata/bib_styles changes do not make
it geometric and per-caller injections remain on independent copies.

`lab/history/construction/records/00_nucleo/materialization/memoizacao-indice-2026-10-05.md:3`
records human gate approval 2026-10-05 for Module L0 SHA256
b1499e45b2c79e61ceb0b4e6f079caf639ae610ffa09a37bab53c057edd89e54.
This supersedes the pending getter gate within that bounded package.

## Intrinsic builtin identity: evidence and contract

html-feature-identity-2026-10-17/{before,before-probe}.json.gz under
lab/results/construction/diagnostics preserves the same HEAD/uncommitted tree,
UTC/status/diffstat/full sources/L0/hashes,2123 verified prior hashes,argv/
fixtures/outputs/binary SHA. Historical pin9f2b6e8715237cb086899a42873660fe744622e8,
binary7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
fields.rs:39-52 gated format but not std; identity.rs:84-101 inferred module
identity from names/bindings. Correct-import probes exposed std.html bypasses
through direct/alias/closure/reexport and false gates on ordinary global.typ/
format.typ with similarly named bindings. The first `let m = import` probe
was invalid because import returns none; archived candidate-probe-syntax gives
no evidence. Upstream module.rs:138-154 reads binding guards and code.rs:347-368
anchors diagnostics to the field; language observables do not require copied
BindingGuard/address mechanics. The field-access owner forbids binding comparison.

Private ModuleKind::{Ordinary,Std,Format} and private kind mark identity.
Public new always creates Ordinary, even named global/format with identical
bindings. Crate-private stdlib(scope)/format(scope) factories fix those names;
crate-private is_std/is_format expose predicates. Preserve Debug/Eq/Clone,
setters/content/cache/metadata/bibliography and public opacity/signatures.
Measure Module/ModuleInner sizes without promising gain. The pinned nucleus
binds identity; ADR-0127 permits internal parity correction, no public/default/
phase change.

## Causal gate continuation

candidate-1/validation-green.log had four passing controls then genuine
argument-order failure: std.html(panic(...)) evaluated panic before gate.
before-set-probe.json.gz records bilateral provenance at the same tree:
std.html/format.html/std.asset/format.bundle calls evaluated args first;
set targets/aliases erased target errors and ignored args; missing variable/
field also passed, while false condition correctly short-circuited. Active
HTML alias-set ignored panic locally but ran it upstream. collection_methods.rs:36
evaluated before authentication; set.rs:44 used ok() and 326 checked literal
HTML syntax. These are causal effects/diagnostics, not mechanical differences.

The fields owner centralizes internal eval
check_module_field_feature(&Value,&str,Span,Features)->SourceResult<()> before
lookup; other owners never duplicate rules/diagnostics. Keep Module's prior
responsibility, no fallback/suppression/public change. L0 first, new source
freeze and full-suite rerun; changed sources cannot reuse old gates.

## Acceptance and limits

new("my-file",scope with x) preserves name/binding; clone is consistent;
new("empty",empty scope) remains empty. Show-derived snapshot includes original
and generated content; either content setter invalidates it; later attempts
cannot contaminate clones. Fields/labels/causal order match fresh reconstruction.
Revalidate PDF/HTML/query/bundle; bundle keeps per-document seeds/resets and
cannot substitute the root index. Measure paired time/memory against preserved
single-evaluation executable with source/hash/tree/time provenance; no universal
benefit or layout/cell memoization claim.

Builtin gate RED→GREEN covers homonyms,aliases,closures,imports in eight profiles,
exact message/hints/span/causal precedence and bilateral call/set effects with/
without feature. Distinguish disabled-feature rejection from active-feature
MATCH/Preserved credit. Preserve prior cross-file std equality divergence.
Build/full workspace/strict all-target Clippy+Rustdoc/fmt/strict-reverse
V1/V5/V15/V26/global differential diagnostics without new warnings/errors;
only six-owner reseal after valid graph. Heavy jobs sequential,
MemoryMax 12GiB/High 10GiB/swap 0/jobs 1. The original gate package excluded
publication/commit/release and did not certify global parity.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/module.md` preserves the original evidence and chronology.
