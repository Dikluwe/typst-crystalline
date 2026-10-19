# L0 — compiler/eval/syntax_diagnostics
Hash do Código: 40dbd115

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/source-validation.toml sha256:727a433beef7710d70729176c3b5a5f851b7e98be320c7594b86e5b269d8d49d

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/syntax_diagnostics.rs`


## Imported syntax validation and inherited context

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/import-syntax-2026-10-17/before.json.gz`
retains UTC/status/git diff HEAD --stat/sources/L0/hashes; 2125 hashes from the
prior closure were checked. before-probe.json.gz preserves bilateral fixtures,
argv/outputs/binaries; before-context-probe measures eight profiles and a real
bundle. Historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
modules.rs:108 enters eval_markup without validating root; include repeats at
:324. mod.rs:649/778 validates main entrypoints, distinguishing warnings.
Reserved/mixed/nested panic with exit 101 in ast/mod.rs:94 while vanilla reports
syntax errors at the real source. Missing strings/expressions may be accepted;
a prefix panic executes before syntax error; module warnings are lost. Upstream
lib.rs:71–82,128–137 validates errors_and_warnings before eval; import.rs:36–47
registers a file trace. Do not infer intent from mechanics: acceptance is no
crash/undue effect and causal diagnostics with spans/hints/traces.

modules.rs:104–106 inherits only full_error: active Html/A11yExtras/Bundle fail
imports as disabled, and Bundle documents in modules fail under Paged context.
before-context-probe includes a bundle exporting child.html in vanilla and
failing in Tekt. This is context propagation, not a new target. The earlier
format:format.html attempt is archived in context-format-function-attempt;
that separate contract discrepancy is not target evidence. The valid fixture
uses .html/auto inference.

## Exclusive contract

Provide static private helper
`pub(super) fn validate_syntax(root: &SyntaxNode, emit_warning: impl FnMut(SourceDiagnostic)) -> SourceResult<()>`.
Enumerate root.errors in order and classify all five current SyntaxErrorKind:
NoTextWithinStars→Strong and NoTextWithinUnderscores→Emph warnings;
Other/InvalidHexNumber/InvalidUnicodeCodepoint errors. Normalize a warning span
exactly as the existing main block: LinkedNode::find of the empty Error,
expected parent kind and len 2, otherwise original span. Build
SourceDiagnostic::warning/error with original message and every hint in order.
Emit warnings through the callback, collect all errors and return Err when any
exist, otherwise Ok. No productive catch_unwind, typed AST, I/O, global context
or message reinterpretation. This is the sole owner of common translation/
classification.

ADR-0127 internal parity flow requires L0 before code. Preserve public Rust
contracts/defaults/flags, SourceDiagnostic/Tracepoint/World/Sink types and
eval/layout phase. Helper/closure remain static/private; the pinned
source-validation nucleus shares policy across actual owners.

## Acceptance and limits

Functional RED before implementation, no productive catch; tests may use
catch_unwind only to turn the old crash into a readable failure. GREEN requires
all errors before effects, nonfatal warnings/stable dedup, imported-source
provenance and nested traces. Mandatory controls: in-memory World packages,
RootedPath include, every feature profile and Bundle document. Preserve main/
expression/valid-file controls and the frozen corpus; a disabled-profile
rejection is not credit for an enabled feature. Probes retain whole stderr and
HTML without byte normalization to claim parity. Preexisting markup/diagnostic
differences do not silently become equivalent. Parser, include morphology,
document.format and unsupported import categories remain separate work unless
actually corrected/measured in this scope.

Validate ownership/V26 before restricted resealing; build/workspace tests,
all-targets Clippy -D warnings -D unfulfilled_lint_expectations, strict Rustdoc,
fmt, V1/V5/V15/V26/reverse lineage and differential SARIF without new errors/
warnings. The uniform V19 note for three error kinds may move to this owner;
preserve exhaustive match and justification. Heavy commands sequentially under
12 GiB/High 10 GiB/swap0/jobs1. No commit/push/merge/release or lint suppression.
