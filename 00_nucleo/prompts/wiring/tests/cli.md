# L0 — wiring/tests/cli
Hash do Código: c299a97e

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/cli.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/tests/cli.rs:38` — `temp_typ`.
- `04_wiring/tests/cli.rs:46` — `temp_pdf`.
- `04_wiring/tests/cli.rs:54` — `cleanup`.
- `04_wiring/tests/cli.rs:60` — `WatchedChild`.
- `04_wiring/tests/cli.rs:63` — `assert_alive`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/cli.rs`.

## Owned contract

- Own the external CARGO_BIN_EXE_typst integration suite only. Observe exit status, stdout/stderr and artifacts, never import private main modules or implement parsing/compilation/formatting.
- Cover commands/aliases/help/rejections, root/font/input/certificate/document id, eval/query/fonts/info/completions, formatted cross-file/plugin spans, transactional init, formats/features/casts and semantic HTML morphology.
- Success with no warning preserves empty stderr; file output leaves stdout empty. Errors precede experimental format warnings, followed by domain warnings; warning-only compilation retains success; semantic errors and missing physical query input exit 1; other argument/I/O errors 2. Check format signatures and content/DOM rather than complete byte parity.
- Use isolated temporary directories and bounded processes. Watch guards kill and wait on every path; each relevant mutation occurs once. Exercise initial publish, asset change, irrelevant-file filtering, temporary error with last artifact retained and recovery while the child remains alive. Sleeps/retries do not prove internal causal ordering; ArmedWatch integration owns arm/publish/wait.
- Nushell completions run twice deterministically with nonempty script/module/exported command structure and empty stderr in isolated HOME/XDG, producing no files. Preserve Bash/invalid-shell controls and public w alias; accept only the existing singular/plural help annotation spellings.
- Preserve canonical repr fields/order and semantic HTML feature/target separation, void/nested/escaped/whitespace cases. Default PNG must finish within memory/time bounds and produce A4 IHDR 1191 by 1684 at 144 PPI, never pass by OOM or absent output.
- Bundle cases cover binary assets, nested mixed-format documents, normalized duplicates, invalid paths, loose content and feature absence. Controlled watch/HTTP tests cover no_serve/no_reload/port, 200/301/404, MIME, raw asset bytes, response-only HTML reload script, SSE and last-snapshot preservation. Leave no daemon or child process.

## Verification and boundary

Run `cargo test -p typst-wiring --test cli -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Evidence and approval boundary

The full chronological record is archived at
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/wiring/tests/cli.md`.
Diagnostic paths below are under `lab/results/construction/diagnostics/` unless
fully qualified. Their historical oracle is
`9f2b6e8715237cb086899a42873660fe744622e8`; they retain the recorded state/time/
binary provenance and do not certify the current resynced pin.

- outer-sealed.log records cli_sucesso_com_warning and
  disciplina_warnings_antes_de_errors failing because hyphenate had already
  become supported. warning-control.typ in typst-unsealed emits
  `no text within stars` for `**`, preserving content and exit 0. Loopback
  restrictions in two watch tests are sandbox limitations, not discarded fixtures.
- `diagnostic-paths-2026-10-06/before.json` observes matching source/span for an
  unknown in a file outside cwd but absolute Tekt versus relative-with-.. vanilla
  paths. The broken control has primary `┌─ lib.typ:1:21` and a separate
  `while calling broken at main.typ:2:1` trace, without another ┌─; green.log
  preserves the initial wrong trace expectation. measurement.json.gz refutes
  attributing an eval expression to its document: oracle uses `<input-expression>`.
- `diagnostic-paths-2026-10-06/identity-next-audit.json` retains commands/sources/
  UTC/HEAD/changed tree/Tekt and vanilla hashes. Document source was lost while
  expression received the document's name. `compiler/eval/mod.rs:609–615` uses
  world.main() for Source/base, wiring/eval.rs:66–95 repeats that id for primary,
  world/cache.rs:58–78 registers/recycles ids; upstream eval.rs:159–179 and
  world.rs:176–190 distinguish expression/stdin/absent input. Integer equality/
  upstream global interning is mechanics; displayed origin/source/position and
  relative resolution are language observables. Human `confirmo` on 2026-10-06
  approved the identity gate, receipt
  `lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json`.
- `empty-markup-warning-2026-10-06/before.json` records empty **/__ caret width;
  this is independent of source identity and compile/query warning order.
- `compile-stdin-2026-10-06/before.json.gz` retains HEAD/tree/UTC/sources/commands/
  binary SHA-256. Physical file `-` with #unknown_physical competed with separate
  stdin; Tekt opened the file, oracle read stdin. compile.rs:223–270 always used
  SystemWorld::new; world.rs:102–106 already has for_stdin without physical read.
  Upstream args.rs:164 documents `-`; world.rs:222–242 validates root/selects
  stdin identity and :282–290 acquires bytes. Documented syntax establishes
  intent; FileId/lazy acquisition are mechanics. World-selection causality is
  refuted if only composition changes fail the language/diagnostic/resource controls.
  green.log retains stdout residual RED at compile.rs:487–492; after.json.gz
  root_error refutes unconditional `<stdin>` under a child root, requiring
  `project/<stdin>:1:1` for --root project with parent cwd.
- `plano-debitos-2026-10-06/before.json.gz` observes compile/c `-` without output
  writing -.pdf while oracle requires OUTPUT/exit 2. Upstream args.rs:351–355
  explicitly requires it; shell cli.rs:922–935 derived it. Implicit `-` remains an
  established Tekt extension and is not itself a vanilla parity probe. final.json.gz
  records wrong initial `Usage: compile [OPTIONS] [INPUT] [OUTPUT]` presentation.
- `plano-debitos-2026-10-06/math-before.json.gz` retains pin/HEAD/tree/UTC/sources/
  commands/SHA: explicit equation is locatable but query(math.equation), alias
  and selector(math.equation) were rejected. selector.rs:269–283 recognizes a
  static native table without equation; structural/math.rs:97–103 already registers
  its constructor and the index already supports Equation/ElementKind::Equation.
- Warning/query controls in plano-debitos before.json.gz preserve raw stderr and
  refute warnings→errors and generic hints. Human `confirmo` on 2026-10-06 approved
  the amendment and replacement hint; candidates/hashes:
  `lab/history/construction/records/00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json`.
  HTML controls require error before experimental before empty-strong warning;
  successful output retains experimental before empty-strong.
- `query-residuos-2026-10-06/before.json.gz` retains HEAD/tree/UTC/pin/binary hashes/
  sources/commands/stdout/stderr/status. query-stdin-callback wrongly succeeded;
  generated metadata and nested error disappeared; query-selector-quotes failed
  at the historical selector limitation before document error; where-level was
  rejected versus [2]. Invalid-document/invalid-selector establishes compilation
  precedence; valid-document/invalid/text-selector establishes retrieval error
  without deprecation. Existing contextual eval references already match using
  full paged compilation. Upstream query.rs:34–66 compiles then retrieves;
  :86–105 evaluates in default Scope then casts LocatableSelector. Local
  query_helpers.rs:488–522 consulted only Module, pipeline.rs:182–214 already used
  final-introspector compilation. The compiled-query API/path was approved with
  `confirmo` on 2026-10-06; receipt query-compilada-aprovacao-2026-10-06.json.
- `query-compilada-2026-10-06/refined-debug.json.gz` compares query-where-body
  and independent eval-where-body-reference: both retain complete Tekt body,
  while vanilla preserved smartquote(double:false) and Tekt had already converted
  it to Text("’") at markup_stream.rs:125. This refutes attributing quote loss to
  new query field transport; it remains separately measured construction debt.
- `smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz`
  retains pin/binary SHA-256/HEAD/tree/diffstat/UTC/sources/argv/cwd/environment/
  stdout/stderr. PDF uses extracted text and HTML DOM/text/diagnostics. Eval
  converted markup quotes into Text, losing double/enabled; variables retained
  English glyphs under German/French/disabled use; primes 6'/2" became quotes.
  Separate native-show/public-name repairs already passed, but markup remained
  non-SmartQuote. Vanilla distinguishes empty fields, omitted double Unset,
  explicit double:false and enabled:false preserving SmartQuote. Positive set
  double:false/filtered-show controls supplement literal HTML Text success and
  programmatic SmartQuote rejection. Literal title quotes render; script requires
  string and rejects Content before export in both binaries. Local source anchors:
  markup_stream.rs:79–128, layout smartquote.rs's two toggles without preceding
  text, stdlib/text/smartquote.rs:112–117 removing disabled elements, observation/
  elements.rs:242–250. Vanilla markup.rs:141–146 sets explicit double,
  inline/collect.rs:196–211 resolves style/context, text/smartquote.rs:119–158
  distinguishes primes/apostrophes/nesting. Human approval on 2026-10-06 ratified
  double bool→Option<bool>, enabled:Option<bool>, pure helper/context and
  eval→render glyph choice. Only conflicting quote clauses are superseded;
  other ADRs/nuclei/proposals remain intact.

## Warning, source and path acceptance

Use `**` only to replace the obsolete hyphenate warning fixtures, retaining
stdout/artifact/stderr/order assertions. Never restore a warning for a valid
property or weaken the sentinel. The historical warning-order sentinel
`disciplina_warnings_antes_de_errors` now checks the approved measured order,
including both diagnostics/content/status/artifact; the old order is superseded.

Path controls use isolated cwd/files: absolute/relative main input, imported error
and trace, query/contextual transient eval, cwd descendants and space/Unicode
names. Assert status, actual source/position/hints without path projection,
weakened prior canaries or candidate-helper-derived expectations. Full bilateral
equality belongs to reproducible collection. Trace checks retain second file/line/
call without demanding primary-label syntax for trace lines. Query hints deliberately
use input.display(), including a supplied absolute path; inspect diagnostic paths
without prohibiting that deliberate hint text.

The physical-path repair originally excluded transient naming, compile stdin and
warning order; retain its adverse expression case as a separate recorded DIFF,
resolved only through their independently approved contracts below. Eval --in
regressions observe actual document excerpt/position, document warning plus
expression error without swapped sources, invalid `<input-expression>`, stdin
`<stdin>`, imported error/trace/document callback, relative read with cwd outside
input's directory and Unicode source. Never project names/spans. Require two-
character carets for ** and __ with valid and later-failing expressions; retain
document/expression paths/hints/status. Identity alone did not close that caret
debt; these explicit negative controls do. Caret repair does not certify unrelated
compile/query warning order.

## Compile stdin and output acceptance boundary

Productive obligations belong to `00_nucleo/prompts/wiring/compile.md` and
`00_nucleo/prompts/shell/cli.md`. Observe exactly input `-` selecting stdin once,
root validation/canonicalization, partial BrokenPipe acceptance, UTF-8 validation,
missing-root exit 1/body `root directory not found (searched at ...)`, invalid-UTF8
exit 1/body `file is not valid UTF-8` plus extensionless hint, other acquisition
exit 2 and one status-before-diagnostic callback. Preserve font/input/CA/plugin
configuration, no physical document file, supplied-root imports/read and causal
stdin/import sources across format/timing/publication/warnings/phases.

Black-box controls distinguish physical `-`, PDF file/stdout, SVG/PNG/HTML,
inputs/import/read with explicit root, errors/traces, ** warning and invalid UTF-8.
Use language/diagnostic observations, never complete PDF/SVG bytes/internal ids.
Output `-` sends unchanged exported bytes to stdout for physical or stdin input,
without overwriting physical `-`; require PDF signature/status/stderr. Bundle
rejects stdout before World, and watch rejects it. Root controls retain `<stdin>`
for cwd root and `project/<stdin>` for child root, including traces, without a
primary source overriding the synthetic path.

The initial stdin slice left required-output parsing incomplete; its separate
current L2 contract rejects `-` without positional/flag output before format/output/
RunIntent or resource acquisition, exit 2/body
`the following required arguments were not provided:\n  <OUTPUT>`.
Cover c/compile/implicit, pdf/svg/html/bundle, empty stdout/no -.pdf/no bundle tree;
accept positional/flag/stdout and flag precedence in both orders, preserving
physical-input derived output. Public signatures/flags/helpers stay unchanged.
Error presentation clones the existing subcommand with bin_name `typst compile`
and required input/output only on the error tree. Require `Usage: typst compile`
and `<INPUT> <OUTPUT>`, preserving parse/help/completion tree/accepted forms.
Do not invent oracle-only creation-timestamp or claim whole-stderr identity across
different flag surfaces. Preserve raw stderr. Continuous ADR-0127 parity repair
adds no public/default/phase contract; the old warning-order constraint is
superseded only by the approved order.

## Selector and query acceptance boundary

The existing native equation constructor is recognized by the owning static table
using fn_addr_eq. Structural hub exposes only an internal pub(super) bridge to
math::equation::native_math_equation within stdlib; no broader module/API/signature
visibility, Func-name/string identity, executed constructor/World/make_math_module
binding discovery. Tests cover parse_selector_arg/native_selector, renamed alias
to the real native function and a distinct function named math.equation. Observe
number/order/fields of existing equations. Do not wrap bare math in Equation,
change phase, promote all functions, certify matrix morphology or merge table.cell.
.where remains with element_where and its separately approved scope.

Query clean/warning/error/warning+error controls cover hints with field/one,
identifier/special field, alias, quotes/Unicode selector, cardinality error and
stdin error/trace/import; collect raw stdout/stderr/status against the pin without
projecting divergences. Pure L2 controls cover the public hint/escapes/no I/O.
Cell need not become locatable: oracle rejects it, and show/selector are distinct.
Missing physical query inputs (relative, ./relative, absolute) require empty stdout,
exit 1 and complete input-file-not-found stderr, no deprecation, preserving original
cwd/argument and cleanup. Compile's missing-file expectation does not change.

The compiled-query boundary is the complete paged pipeline/final introspector,
compile before selector, no file export/PDF extraction, one ordinary execution,
no isolated expansion/pre-context index/parallel stabilization. Typed enum separates
compilation and retrieval, never message/span/substring heuristics. Wiring composes
without a selector/realization algorithm or new flags/targets.

RED before implementation covers the three original residues, valid metadata
callback/nested error, .where(level), .where(body with apostrophe/Unicode), invalid
document plus invalid selector, valid document plus invalid/text selector, label
and math.equation. Compilation error precedes selector and receives warnings/
deprecation; retrieval error gets neither. Observe stdout/status/stderr and Content
morphology, not Rust equality/PDF bytes. Retain earlier stdout/cardinality/field/
path/import/argv controls; never accept lost fields/Source or an unexecuted program.
The original body-filter acceptance required correct cardinality/full sequence/
adjacent text and identity with eval --in while separately retaining quote debt;
the approved quote contract below supersedes only that debt's quote restriction.

## Semantic quote acceptance boundary

The owning domains preserve SmartQuote identity/fields in eval and resolve glyphs
only at each target's rendering composition. HTML stays semantic without paginated
layout (ADR-0128); one pure L1 context helper has per-flow/compilation state, no I/O/
globals/vtable/property registry/upstream memoization copy. Stack/state is Tekt
mechanics; acceptance is language/known-versus-omitted fields/glyph/style. Escaping
stays with HTML owners. No modes/flags/unsafe/watch API/math-phase additions.

Bilateral CLI tests cover query/eval body with true/false double, empty/present
fields, getter Unknown versus false, language/disabled reuse, show smartquote/alias/
.where(double), PDF/HTML/default/title and primes/wrapping. Keep the full original
23/23 query matrix. Invalid script/style and recursive show-text controls are
rejections, not positive quote witnesses. Do not use PDF bytes for parity or
change fonts/SOURCE_DATE_EPOCH for convenience. Preserve exclusive owner/pins and
RED→GREEN; no unrelated proposal is approved by this quote gate.

## Harness boundary

### Evidence

`lab/results/construction/diagnostics/tekt-v12-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC,
sources/L0/configuration/lock/hashes/status/diffstat. Its 2077 hashes match the
final FrameItem checkpoint: the prior suite ran 8622 tests with zero failures
and seven ignored at that same state. gate-baseline-result.json records real RED
for 19 V12 before L0/code changes. items.json contains complete AST items and
classification.json records responsibility per unit. These are historical
measurements, not new parity or upstream-intent claims.

The linter identifies top-level enums/inherent impls in L4, including test support.
Scenarios, language fixtures and final product assertions stay in L4; independent
subprocess/filesystem/observation helpers belong to L3 test support. Do not move
the whole suite, hide declarations inside an L4 module to exploit shallow parsing,
exclude tests from lint, convert impls solely to evade the rule or change severity.

### Exclusive responsibility

Use `03_infra/tests/support/wiring/cli.rs`, owned exclusively by
`00_nucleo/prompts/infra/tests/support/wiring/cli.md`, for WatchedChild, their existing impls and Drop.
Keep all scenarios and final observations in this L4 consumer. Import only
WatchedChild through a harness module with static #[path] to L3;
never use that module in the production library.

Recorded pre-transfer item anchors (historical, not current declaration locations):
04_wiring/tests/cli.rs:60 struct_item WatchedChild; 04_wiring/tests/cli.rs:62 impl_item WatchedChild; 04_wiring/tests/cli.rs:76 impl_item WatchedChild.

### Acceptance and limits

This internal ADR-0127 harness separation does not put support into the L3
production library or CLI. pub(super) grants access only inside the test crate.
No public product API/dependency/flag/default/phase changes. The separately
ratified Published contract grants no other contracts. No new derives,
suppressions, hooks, cleanup policy, tolerance or verification fallback.

Preserve moved item bodies except minimum visibility for the parent test module;
Checks fields remain private where present. L3 owns its std imports, with no
use super::* or dependency on L4 logic. BIN, subprocess invocation/options,
scenarios/expectations and #[test] remain in L4; support does not read
CARGO_BIN_EXE_typst. Retain legitimate per-harness differences in prefixes,
messages, creation atomicity and cleanup.

Check restricted transfer replay, identical test functions/attributes/expectations,
valid 1:1 graph, no support in the productive library, V12 GREEN zero and complete
SARIF without new warnings. The original batch required workspace suite with
explicit /dev/shm TMPDIR and historical vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
That historical receipt does not certify the current pin; subsequent runs use the
ratified current oracle after independent recipe/source/tree/SHA verification.
Preserve all expectations/tolerances. Validate cargo build, workspace/all-target
Clippy -D warnings/-D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings/four
indexes, strict V1/V5/V15/V26/reverse lineage and diff --check. L0 first; resealing
limited to authorized owners/code. Heavy jobs sequential under MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1; freeze sources/L0/configuration during commands.
No import-syntax mirror tests, unmeasured RAM/AI-gain claims or release publication.
