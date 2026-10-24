# L0 — compiler/stdlib/tests/layout_containers
Hash do Código: e3f4b2ce

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
