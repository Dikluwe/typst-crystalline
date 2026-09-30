# Layout-risk reproduction

## Scope and evidence rules

Diagnosis and reproduction only, following repository instructions and active
ADR-0107/0108/0109/0121/0127/0129/0130. The starting hypotheses come from
sanitization-integration-review.md, sanitization-layout-history.md and
sanitization-introspection-columns-history.md in this directory. Their historical
observations are not fresh parity evidence. Content, morphology, numbering and
visible geometry are language observables; queue representation and slicing
algorithms are mechanics. Vanilla's execution mechanism is not prescribed.

All new files belong to this report or
`00_nucleo/diagnosticos/layout-risk-reproduction/`. This investigation did not
edit productive code/tests, L0s, ADRs, nuclei or lineage headers. No global
formatting, hash reseal or productive test-suite execution was performed.

## Provenance and reproduction

Measurement HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`.
The working tree was already dirty; HEAD alone does not identify measured inputs.
Each `outputs/<fixture>/<binary>.json` records exact command, UTC start/finish,
exit, stdout/stderr, fixture/executable hashes and state receipts.
Immutable `receipts/state-*.json` files contain full porcelain status, per-file
SHA-256 and controlled environment. Associated binary git diffs against HEAD
have recorded hashes. This identifies staged, unstaged and untracked inputs,
without pretending the committed tree represents the session.

- Crystalline: `target/debug/typst`, SHA-256
  `5ec96631940f3f02be377c67af7b798337176e1c9e8c7bfd9a63675580b4dd12`.
  `cargo build --locked --bin typst` succeeded between
  `2026-09-27T01:59:00.221999Z` and `2026-09-27T01:59:04.920168Z`.
  This was a Cargo-validated incremental build, not a claimed clean rebuild.
  Productive/documentary snapshots before/after that build were identical;
  see `receipts/build.json`. Auxiliary version: `typst 0.15.1 (6ef8b698)`.
- Vanilla: `lab/typst-original/target/release/typst`, SHA-256
  `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
  Bytes were verified before use against the ratified P1489 receipt.
  Pin: `9f2b6e8715237cb086899a42873660fe744622e8`; tree:
  `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, freshly resolved by git.
  `00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md` records the locked
  quarantined-workspace build with explicit TYPST_COMMIT_SHA and one Cargo job.
  Version text alone was not provenance. Neither the root release binary nor
  /usr/local/bin/typst served as oracle.

External documentary edits occurred during execution. Individual command
boundaries and immutable receipts preserve those states; batch receipts marked
`stable: false` do not hide the changes. The final evidence audit found no changes
in 01_core/02_shell/03_infra/04_wiring since the validated CLI build. Exact external
L0 paths are listed in `receipts/evidence-index.json:changed_since_build`.
Selected decisive commands have individual before/after snapshots. The function
numbering control had an external documentary edit during its crystalline run;
its binary and productive inputs remained unchanged. No external edit was reverted.

The evidence index records receipt/input asset hashes, DejaVu font-file hashes
and tool versions; its integrity check passed. Compilation used LC_ALL=C, TZ=UTC,
NO_COLOR=1 and `/usr/share/fonts/truetype/dejavu` explicitly. State receipts record
the controlled environment. The page-column footnote fixture intentionally avoids
a second text-style wrapper and uses default font behavior; the explicit-container
reproduction independently uses DejaVu Sans.

Run from the repository root:

```sh
python3 00_nucleo/diagnosticos/layout-risk-reproduction/create_asset.py
python3 00_nucleo/diagnosticos/layout-risk-reproduction/run.py --build
python3 -u 00_nucleo/diagnosticos/layout-risk-reproduction/run.py
python3 00_nucleo/diagnosticos/layout-risk-reproduction/slicing_probe.py
python3 00_nucleo/diagnosticos/layout-risk-reproduction/visual_review.py
python3 00_nucleo/diagnosticos/layout-risk-reproduction/audit_evidence.py
```

Use `--only <fixture-stem> ...` for isolation. A batch exits nonzero on a changed
source/documentary snapshot but retains individual result receipts; that runner
exit is not a product failure. Initial scouting used an unsupported vanilla
--color option and unsuitable rect stroke arguments. Those attempts are not
parity evidence. Final accepted fixtures/commands omit those arguments.

Text extraction used pdftotext -bbox. Colored bounds use mutool at 72 dpi and
measure red/blue pixel ink, not layout API bounds. Raw/footnote review renders
used 144 dpi and were visually inspected. Conclusions do not depend on PDF byte
identity or missing text extraction alone. The rendering workflow did not
authorize polishing or fixing diagnostic product outputs.

## Results

| Bounded hypothesis | Classification | Decisive fixture/harness |
| --- | --- | --- |
| Deferred note lost at explicit segmented-container exit | Confirmed | footnote-container-deferred.typ |
| Deferred note lost at last segmented page column | Confirmed | footnote-page-last-deferred.typ |
| Image cover clip lost during recomposition | Confirmed | clip-align.typ, clip-float.typ |
| Tail slicing discards clip_rect | Confirmed mechanically; language consequence inconclusive | slicing_probe.rs |
| Pattern page reference displays physical rather than logical number | Confirmed | page-reference.typ |
| Raw whitespace not preserved | Confirmed | raw-minimal.typ, raw-spaces.typ |
| Floating offsets have no effect | Confirmed | float-offset.typ |
| Column-scoped float ignores its column boundary in this case | Confirmed | float-scope-column.typ |
| Initial weak breaks behave as strong breaks | Confirmed | pagebreak-weak.typ, colbreak-weak.typ |

Universal loss claims are refuted by controls: fitting notes survive; directly
emitted images keep their crop; function-numbered references use logical values;
parent-scoped placement matches this vanilla control. These do not refute the
narrowed failures. No source hypothesis alone is presented as a reproduced defect.

## 1. Pending footnotes

Read L0s: `00_nucleo/prompts/compiler/columns.md`,
`00_nucleo/prompts/compiler/footnote_overflow_columns.md` and the layout hub.
Consumers: `01_core/src/compiler/layout/columns.rs`, `footnote_flush.rs`.

Measurement: footnote-container-deferred emits LEFT, MARK and the marker in
crystalline, but no note body, with exit 0 and empty stderr. Vanilla emits
NOTE_SENTINEL. Both render one page. Visual inspection confirms complete body
absence. Removing colbreak in footnote-container-flow-control preserves the
body bilaterally. The note has a 30pt block height on a 300pt page: it is not an
inherently oversized note requiring the defensive escape.

footnote-page-last-deferred independently confirms the last-column case:
crystalline emits one page with the marker and no body; vanilla retains the
body on a successor page. The complete crystalline output and vanilla
continuation were inspected. Earlier small/fitting-note scouting retained bodies.
footnote-last-segment has additional style wrapping and does not isolate direct
segmentation; its page-count difference is not claimed as a diagnosed defect.

Causal path: columns.rs:192 inspects direct separators through one outer Styled;
:407 starts segmented siblings, :435 saves the exterior queue. Page columns
flush at :528; explicit containers flush once at :580. footnote_flush.rs:147-163
defers a body that cannot fit remaining space and preserves it at :383.
columns.rs:595 then replaces that pending queue with saved_outer_footnotes,
discarding the child remainder before the hub can recover it. Runtime output
confirms the archived inference for both input forms.

Language obligation: recover the body or warn; fitting/deferred notes must not
silently disappear. Exact vanilla footer placement, queue storage and page totals
are not universal requirements. Empty stderr proves silence for these probes,
not failure of all oversized-note warning paths.

Boundary owner: `00_nucleo/prompts/compiler/columns.md`; the flush owner already
retains its remainder contract. Next necessary test: an outer pending note plus
multiple fitting/deferred inner notes across the final sibling, checking bodies,
order, numbering and warnings for both page_columns values, RTL and fitting
controls. No productive test was added.

## 2. Image clipping

Read L0s: `00_nucleo/prompts/compiler/layout-image.md`,
`00_nucleo/prompts/compiler/layout/helpers.md`,
`00_nucleo/prompts/compiler/layout/placement.md`,
`00_nucleo/prompts/compiler/layout/place.md`,
`00_nucleo/prompts/compiler/layout/slicing.md` and the coordinate nucleus.

The generated wide.png is 80 by 40 pixels, red/blue. A 40pt-square cover target
requires a 40pt crop, not the full 80pt transformed width. At 72 dpi:

| Fixture | Crystalline colored bounds | Vanilla colored bounds |
| --- | --- | --- |
| clip-control | [30,30,70,70] | [30,30,70,70] |
| clip-align | [70,30,150,70] | [100,30,140,70] |
| clip-float | [10,30,90,70] | [30,30,70,70] |

Inspected images show exposed image halves, confirming lost cropping rather
than merely different alignment. image.rs:222-239 creates the target clip;
Align recomposition uses helpers.rs:143, whose Image reconstruction at :196
sets clip_rect to None. Float emission independently does so at place.rs:442-460.
Preserving the visible target crop is the language obligation; its metadata
representation is mechanics.

Confirmed owners: helpers for Align recomposition and Place for float emission.
Next tests: nonzero x/y translation with translated crop, nested semantic/link/
group paths, contain/no-clip controls, orientation and top/bottom floats.
Keeping an untranslated rectangle alone is not sufficient.

The external Rust harness includes unchanged productive slicing.rs through a
path module and links the recorded Cargo-built core rlib. Its receipt records
commands and source/rlib/harness hashes. Output:

```text
head y=5 clip=Some(Rect { x: Pt(30.0), y: Pt(5.0), w: Pt(40.0), h: Pt(40.0) })
tail y=10 clip=None
```

slicing.rs:39 partitions by starting y; :84/:136 rebuilds the tail Image without
its clip. Mechanical loss is confirmed. Vanilla has no equivalent private API
to compare. A grid/table overflow with a clipped cover image in its tail remains
necessary to prove the language consequence and discriminate other translation
paths. That pagination consequence is **inconclusive**, not a reproduced defect.
Next test owner: slicing plus its caller integration, not the decoder.

## 3. Logical page references

Read L0s: `00_nucleo/prompts/compiler/layout_references.md`,
`00_nucleo/prompts/entities/page_store.md`,
`00_nucleo/prompts/compiler/introspect.md`,
`00_nucleo/prompts/infra/pipeline/paged.md`.

page-reference-control displays page 1 bilaterally. After counter(page).update(7),
page-reference displays “page 1” in crystalline versus “page 7” in vanilla;
both visible footers show 7. The function-numbering control displays “page 7”
bilaterally, isolating the Pattern branch rather than all introspection.

references.rs:30 captures physical destination identity, appropriately. Pattern
at :106-107 formats that physical page_number; Func consumes the unary realized
PageStore view. paged.rs:712 realizes logical numbering and :686 binds logical
snapshots. Physical location/link identity must not be replaced with logical
display numbering; link clicking was not measured by these probes.

Owner: `00_nucleo/prompts/compiler/layout_references.md`. Affected obligation:
display the target's logical number consistently with its unary reference view.
The sanitized owner already marks this older promise unresolved; this diagnosis
does not rewrite it. Next test: Pattern/Func with non-one starts, multiple pages,
forward/backward targets, lexical numbering changes/resets, physical destinations,
supplement precedence and missing-numbering diagnostics.

## 4. Raw whitespace

Read L0s: `00_nucleo/prompts/compiler/layout/raw_highlight.md`,
`00_nucleo/prompts/entities/elements/raw.md`; consumer raw.rs.

raw-minimal uses raw("A    B", block:true, lang:"txt"). Crystalline renders AB;
vanilla retains the gap. raw-spaces adds a single-space control and a tab/empty
line under an unknown syntax. Crystalline renders AB, AB, ABC; inspected vanilla
output retains different gaps and the vertical separation of C. Bbox reading
order is not used as the whitespace proof.

raw.rs:168/:174 uses split_whitespace for highlighted/fallback lines, :181 for
the whole unknown-syntax string. Separators disappear before layout_word and
are not reconstructed there. The entity retains the original text. Exact raw
spacing/line structure is the language obligation, not syntect or token storage.
The plain path is reproduced; exhaustive highlighted-syntax parity is not claimed.

Owner: `00_nucleo/prompts/compiler/layout/raw_highlight.md`. Next test: plain and
recognized highlighted raw with leading/repeated/trailing spaces, tab stops,
empty lines/terminal newline, block/inline forms and style restoration.

## 5. Float offsets and scope

Read L0s: Place/placement owners, `00_nucleo/prompts/entities/elements/place.md`,
`00_nucleo/prompts/compiler/stdlib/layout.md`; consumers place.rs/stdlib/layout.rs.

| Fixture | Crystalline colored bounds | Vanilla colored bounds |
| --- | --- | --- |
| float-base | [15,15,45,35] | [15,15,45,35] |
| float-offset (dx 17pt, dy 11pt) | [15,15,45,35] | [32,26,62,46] |
| float-scope-column | [195,15,225,35] | [86,15,116,35] |
| float-scope-parent | [195,15,225,35] | [195,15,225,35] |

These 72-dpi observations confirm ignored offsets and missing column/parent
distinction in this case, not universal Parent failure. stdlib/layout.rs:113-133
transports the inputs. place.rs:69 queues DeferredFloat without them, :84-86
discards them and :306 emits using region/alignment only. Scope measurements do
not certify every upstream column lifecycle. Non-float delegation at :115 retains
the inputs; these probes do not diagnose its path as defective.

Owner: `00_nucleo/prompts/compiler/layout/place.md`. Language obligation: accepted
float offsets affect visible geometry and scopes select their respective regions.
The current L0 explicitly records unsupported fields: this is a confirmed
language gap, not proof of breaching a promise of already-implemented support.
Next test: isolate each offset, negative values, top/bottom floats, alignments,
actual column progress, non-float controls and independent clearance/flow checks.

## 6. Weak breaks

Read L0s: pagebreak/colbreak layout and entity owners, columns and stdlib/layout.
Constructors preserve weak/weak_explicit at stdlib/layout.rs:2254/:2848.

At empty page start, pagebreak-weak produces two pages in crystalline and one
in vanilla; strong produces two bilaterally. At two-column start, colbreak-weak
places TARGET at xMin 124.8 versus 15.0, though both have one page. Strong places
it in the second column bilaterally (124.8 / 124.2); the small gutter-related
coordinate discrepancy is not classified here. Page counts alone miss weak
column-break failure.

pagebreak.rs:16 does not inspect weak; colbreak.rs:16 ignores the element.
columns.rs:208 also consumes inspected separators without testing weak. Transport
therefore succeeds but collapse is absent. The exact column route depends on
wrappers; adapter and segmented splitter both need coverage. weak_explicit is
repr/identity presence, not a second layout policy.

Owners: `00_nucleo/prompts/compiler/layout/pagebreak.md`,
`00_nucleo/prompts/compiler/layout/colbreak.md`, and
`00_nucleo/prompts/compiler/columns.md` for consumed segmented separators.
Obligation: weak initial breaks must not create the empty region deliberately
created by a strong break. Current owners already record missing support.
Next test: empty/nonempty, consecutive/trailing weak/strong breaks, parity and
page-run boundaries, segmented/continuous columns, omission/explicit-false
transport. No new weak policy is authorized by this report.

## Handoff and limits

Confirmed cases need separate owner-approved L0/test/implementation cycles.
No correction or public-contract decision is made here. Slicing's end-to-end
language impact remains open, as do the next tests identified above. Scouting
page-count/text-mapping discrepancies outside the isolated hypotheses were not
promoted to reproduced defects. This is not global sanitization completion.

Artifacts comprise fixtures/scripts, build/execution/state receipts, PDFs,
extracted bbox data and review PNGs in the assigned diagnostic directory.
