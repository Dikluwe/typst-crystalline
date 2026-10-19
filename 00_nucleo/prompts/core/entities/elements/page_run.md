# Prompt L0 — `entities/elements/page_run` — `PageRunElem`
Hash do Código: 0439335e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/page_run.rs`
**References:** ADR-0026/0107/0108/0109/0127.

## Evidence before lexical scope contract

Historical upstream a51e02804 page.rs:504-523 implements page[...] as an initial
weak boundary, flush marker, body, final boundary and local style map.
Language measurements at HEAD 45b547073d7686cdd5d3e3030c82de3e22ec395f,
uncommitted tree,2026-08-24T13:43:00-03:00 (before these L0 edits:84 files,
775 insertions,508 deletions) established:
100×120 inside 200×200 restores 200×200; empty middle run keeps its 100×120 page;
an isolated empty run yields one page; consecutive 100×120/140×160 runs yield
those two pages without an extra blank; nested runs yield
180×180→100×120→180×180→240×240 (LIFO); explicit break in 100×120 creates two
run pages then returns to 200×200. Page configuration inside block/columns
reports `page configuration is not allowed inside of containers`.

## Ratified carrier and deltas

Content::PageRun(Arc<PageRunElem>) is one closed lexical container, preventing
unbalanced pairs and restoring nested scope through normal layout descent.
Separate PageRunStart/End and SetPage push/pop remain rejected because they
permit malformed sequences and mix lexical scope with set-rule progression.

The initial Debug/Clone/PartialEq carrier has width:Option<PageDimension>,
height:Option<PageDimension>, margin:Option<PageMarginSpec>, numbering,
columns:Option<usize>, and body:Content. Later contracts extend it with
optional paper/flipped/binding, lateral margin mode, bleed/fill/background/
foreground, six running-matter arguments (including header/footer tri-state)
and PageSupplement delta. Preserve omission/none and all eighteen established
deltas; do not bake defaults. Paper resolves axes and is not introspectable.
Binding/folded margins and canvas layers participate in the LIFO snapshot.
See `00_nucleo/prompts/core/entities/page_running.md` and
`00_nucleo/prompts/core/entities/page_supplement.md` for their typed domains.

The final numbering delta is Option<Option<Numbering>>, distinguishing omission,
disabling and installation, superseding the original Option<EcoString>/Some("")
encoding. Two consecutive page runs preserved callback/pattern lexically as
X1/2 and II. map_* clones Numbering without executing Func; to_set_page and
lexical application transport the same delta; defaults stay in layout.

## Element and constructors

plain_text delegates to body; is_empty always false, since even an empty run
keeps a page. Both maps recurse into body and present content layers while
preserving every configuration delta; equality/hash retain deltas and body.
No locatable payload or show-rule selectability follows from this container.
Do not expose properties ahead of their public reflection contract.

The internal helper takes typed deltas and body, with no property map/dyn/
string dispatch/numeric defaults/layout/state mutation. Ratified page(..,body)
and std.page(..,body) require body and fill existing deltas; omission remains
None. The initial constructor was deliberately incomplete; later bounded
extensions supersede its exclusions, not unrelated domains.

## Approval provenance and acceptance

`lab/history/construction/records/00_nucleo/materialization/typst-passo-1140.19.md` §15
records owner confirmation 2026-08-24 of the initial carrier;
typst-passo-1140.20.1.md:58 and typst-passo-1140.20.2.md:3,48 record extension
approvals. typst-passo-1140.26.md:145 records public-constructor approval on
2026-08-24. typst-passo-1157.md:380 records owner "Continue" on 2026-08-25 for
typed numbering; typst-passo-1158.md:6 confirms that contract/phase approval.
These receipts supersede the archived pending headings within their scope.

Require exhaustive Content matches, no configuration leakage to siblings,
semantic nonemptiness of empty body and nested runs without global IDs/state.
Preserve all measured page/boundary/LIFO/container-error controls. Public page
construction is authorized by its later receipt; it does not imply closure of
the original entire parity series or other excluded work.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/page_run.md`. Historical observations are not a new parity certification.
