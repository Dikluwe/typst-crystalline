# L0 — compiler/stdlib/structural/math/lr
Hash do Código: b60689b8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/lr.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own `math.lr(..body, size: 100%)` and its bare bridge. Accept zero or more positional values via the shared content cast, interleave canonical comma text, preserve MathSequence morphology, and retain explicit `size` presence even at 100%. Shared size validation is owned by casts. Keep diagnostics, order and static native identity.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.

## Type and Symbol function-cast classification

### Evidence before decision

type-kind-probe.json and more-cast-probe.json under
`lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17` record a
candidate passing 5/6 tests but failing kind:int. Historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`, foundations/value.rs:647–658 accepts
Type.constructor or Symbol.func as Func; content/element.rs:210–214 requires
to_element; symbol.rs:133–137, math/accent.rs:189–214 and math/lr.rs:144–198
classify symbolic conversion. CLI exercises 37 types; Location without a
constructor was checked in pinned source. type-kind-classes.json preserves all
38 classifications. A Symbol body/caption becomes SymbolElem upstream, while
local Content lacks that variant and current markup retains Text. That carrier/
morphology requires public evolution SY1; preserving text does not close it.

### Exclusive responsibility

Add only pub(crate) is_callable_lr_symbol(&str)->bool. A one-codepoint string in the complete pinned DELIMS start set, plus ceil/floor, is callable; empty/multiple-codepoint/unlisted-closing strings are not. lr-symbol-starts.json is the measurement. This classifies casts, creates no symbolic calls and changes neither math.lr, size, body nor its algorithm.

### Acceptance and limits

L0 first; functional Type/Symbol/textual-value RED before code, GREEN and full
batch revalidation under 12 GiB/zero swap/jobs1. ADR-0127 continuous flow adds no
public Rust contract, phase or product default. Reseal only affected owners after
V15/V26; preserve math tests and original classifiers.
