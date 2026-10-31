# L0 — compiler/stdlib/tests/layout_containers
Hash do Código: 79c13de9

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/layout_containers.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own only the layout_containers regression tests and their local fixtures/assertions. Use the private shared support fixtures. Preserve arguments, diagnostic messages, spans and language-level observations. Expectations must be independent of candidate output. Do not implement namespace behavior or transfer production contracts into this test consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Test quality maintenance

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/testes-ci-2026-10-13/before.json.gz`
preserves sources/hashes/UTC/status/diffstat. Input Clippy inventory is
matematica-ci-2026-10-13/clippy-inventory.json, retained in that batch's
evidence.json.gz. Under ADR-0108 this measurement precedes decision.
The inventory reports equality assertions against Bool literals and unused
pattern bindings. Observed fields include clip, breakable, sticky and overhang.

### Contract and acceptance

Express booleans with assert!/assert!(!...), preserving polarity/message;
rename unused bindings without removing the expression, execution or pattern.
Preserve every input/default/negative control. No productive-code change or new
lint exception. This internal test repair follows ADR-0127 continuous flow.
Quality RED is Clippy -D warnings; rerun cases and validate lineage. Tests without
a prior functional failure must not acquire an artificial functional RED→GREEN
claim.


## Measured Repeat test correction — 2026-10-22

### Evidence before decision

`lab/results/nested-fractions-repeat-2026-10-21/core-interim-red/receipt.json.gz`
preserves UTC `2026-10-22T01:40:02.778850+00:00`–`2026-10-22T01:42:27.009317+00:00`,
uncommitted HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, exact changed-file
stat, all frozen input hashes, full Cargo command/log and `changed_inputs=[]`.
The actual full-core run has 7810 passes, four assertion failures and five
preexisting ignored tests. These two old Repeat expectations are among its
explicit failures; no test deletion, ignore or tolerance change follows.

Bilateral `repeat-before/measurement.json.gz` in that result root (SHA-256
`e6d3c085c28062ad22fdb3b1575108d3a778842a50bf0e5c7fa8381b445e88e0`)
records UTC `2026-10-22T01:15:47.337768+00:00`–`2026-10-22T01:16:29.201630+00:00`,
the same HEAD, exact dirty state and inputs, baseline product SHA-256
`94208ba4526d7bda97d4519aa8eb12212bd13dc1f8932552f235940e73238485`,
and ratified vanilla commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Observable body content, typed payload, defaults and accepted arguments are
language requirements; the candidate's wrapper depth is mechanics under
ADR-0107. This correction follows ADR-0108 and ADR-0127 continuous flow.

### Scoped correction and acceptance

The immutable negative-gap source uses `gap:-1pt`; local baseline rejects it
with the historical P156J diagnostic, while pinned vanilla succeeds and
paints twelve X copies. Source evidence `typst-library/src/layout/repeat.rs:31–44`
declares a Length gap without a nonnegative caster; pinned
`typst-layout/src/repeat.rs:32–65` resolves signed gap in its general arithmetic.
The old `native_repeat_rejeita_gap_negativo` assertion at this consumer's
lines1028–1035 is therefore specifically refuted by a language measurement,
not relaxed to accept arbitrary successful candidate output.

Keep the exact original body `.` and gap `Length::pt(-1.0)` inputs. Rename
the test in English and require actual `Value::Content(Content::Repeat)`
with exact signed gap, unchanged default `justify=true` and original body
text `.`. This stronger complete-payload obligation replaces only the
refuted negative-gap rejection. Preserve every other bad-argument control
(missing body, integer body, non-Length gap, non-Bool justify and unknown
named argument), its polarity, source and diagnostic acceptance. No constructor
change or other fixture correction is authorized by this test owner.

## Stack image frame correction — 2026-10-27

### Measurement before decision

`lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/before.json` records HEAD212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diff/stat/status and every received productive/L0 input, UTC and executable identities. before/{receipt.json,geometry.json}, observe-before.log and effects-before/{receipt.json,query-absolute.json} are fresh bilateral measurements against ratified64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1/SHA22897f93; received product17f39776 includes the completed image/h and JPEG work. Stack3 image tops14.834/22.072/29.310 versus vanilla5/25/45; Stack16 drift181.596pt; Stack20 is one page versus19+1 images on two pages. RTL/BTT source-order placement is wrong; mixed text/image, hidden and clipped children also fail. Pure horizontal text remains qualified. Negative spacing is rejected only by Tekt's P156I gate. Context metadata puts child19 on page1 versus2. Source stack.rs:115–119 returns zero descent for every child without text, discarding a computed bottom; :144 adds an outer text baseline, :181 uses the legacy-origin route, :219 advances by that zero descent plus text top. sub_frame.rs:315–344 distinguishes geometric from baseline origins, and :497–521 returns a cursor-based fallback except for previously owned cases. Pinned upstream stack.rs:274–300 accumulates completed frame sizes and :303–366 composes their geometry. These are observed language geometry/pages/accepted arguments; no upstream intent is inferred and matching Rust/PDF bytes is not acceptance.

### Classification and scope

Private parity correction, continuous ADR-0127: no public API/entity field, trait method, dependency, new product mode, phase or compatibility break. Existing signatures and source identities remain. The old ascent/descent fallback and reverse-direction limitations are superseded only for this measured frame-composition correction. Existing Ordinary/Footnote/Repeat subframe routes and all received JPEG/image/h/notes/line work remain preservation controls. No new page dispatcher, source replay, callback cache or semantic reconstruction from FrameItem is authorized. Use the existing region transition and static source-child protocol. Individual child-internal multipage text, fractional stack spacings and general alignment are not certified by these finite image controls.

### Exclusive test responsibility

The existing native_stack_rejeita_spacing_negativo input Length::pt(-1) is preserved, but its P156I rejection expectation is explicitly refuted by the new pinned negative-spacing source. Replace it with strict successful Stack construction retaining exactly Some(Length::pt(-1)), default TTB and zero children. This is not a deleted/ignored case or looser assertion. Require this test to fail before removing the constructor guard; every other invalid/default case is retained unchanged.

### Acceptance

Require functional RED→GREEN before implementation, source-current bilateral PDF/SVG<=0.001pt, all four directions, spacing/default/negative, target clipping, mixed/no-paint children, nesting and exact-fit/19+1/14+6 pagination, text adjacency and h boundaries. Preserve real Context/page/label/counter source semantics and all current positive image/h, A–H/R1–R13/notes/relative-line controls. Repeat all12 established technical gates, unfiltered lint with zero warning/error, frozen provenance and restricted lineage reseal. Recheck JPEG acceptance/complete decode/dedup/time/RSS and R6 size/memory gates with no competing builds. Bound the known invalid oversized oracle case explicitly; never hide a timeout/signal as a qualified rejection. Historical self-sealed stdout limitations remain explicit; do not rewrite old manifests. No commit/push.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's original point-only Stroke test values/assignments
to Length::pt while preserving exact numeric values and all assertions. When
a prepared scalar is compared to its declarative source, explicitly resolve
that source with the fixture font; do not relax expected geometry or tolerance.
This test migration implements only the approved field representation change;
existing non-stroke obligations and prior approvals remain intact.
