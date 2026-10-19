# L0 — stdlib/math_style — mathematical style constructors
Hash do Código: 4bc93843

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/math_style.rs`

## Evidence before specification

Exclusive L1 consumer `01_core/src/compiler/stdlib/math_style.rs`, behavior split by math-style domain with shared stdlib conventions at `00_nucleo/prompts/core/compiler/stdlib/mod.md`; MathStyled composition belongs to math layout owners. Historical 12-function completion predates Display/Inline/cramped additions and is not current whole-language certification. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

Measured bb("R") emitted unstylable Content::Text while bb(R) produced ℝ: MathText conversion reaches existing apply_math_style arms without layout change. Missing frak() measured `missing argument: body`, unlike historical silent Empty/broken PDF. Bilateral 2026-08-31 oraclea51e02804 refutes old Portuguese extra/named/cramped diagnostics and confirms innermost setter wins same glyph axis, orthogonal flags/sizing compose. Historical byte-identical example was supporting snapshot evidence, not acceptance by byte mechanics.

Measured qualified math.bold/cal/display/italic/sans/scr/sscript/upright missing in Tekt against 2026-09-21 oracle 586e1bd43fae; human approval after causal 445cf0c81 confirms only that projection. Implementation belongs to `00_nucleo/prompts/core/compiler/stdlib/structural/math.md`, with unchanged 14 functions here; historical core 6067 pass is that snapshot, not fresh execution.

## Exclusive constructor contract

Fourteen native_bb/bold/cal/display/frak/inline/italic/mono/sans/scr/script/serif/sscript/upright accept exactly one Content|Str body, Str→Content::MathText, return Value::Content(MathStyled{kind,bold,italic,body,cramped}). italic uses native_math_italic identity. Dedicated pure L1 common helper wraps without resolving composed style.

| Function | `kind` | `bold` | `italic` | `cramped` |
|---|---|---|---|---|
| `bb` | `Some(DoubleStruck)` | None | None | None |
| `bold` | None | `Some(true)` | None | None |
| `cal` | `Some(Chancery)` | None | None | None |
| `display` | `Some(Display)` | None | None | `Some(false)` / named |
| `frak` | `Some(Fraktur)` | None | None | None |
| `inline` | `Some(Inline)` | None | None | `Some(false)` / named |
| `italic` | None | None | `Some(true)` | None |
| `mono` | `Some(Monospace)` | None | None | None |
| `sans` | `Some(SansSerif)` | None | None | None |
| `scr` | `Some(Roundhand)` | None | None | None |
| `script` | `Some(Script)` | None | None | `Some(true)` / named |
| `serif` | `Some(Plain)` | None | None | None |
| `sscript` | `Some(SScript)` | None | None | `Some(true)` / named |
| `upright` | None | None | `Some(false)` | None |

Bold/Italic/Upright kind None are orthogonal over inherited glyph variant (bold(bb(x)) keeps DoubleStruck); Serif Plain forces glyph variant. Display/Inline remain size variants MathStyleKind::Display/Inline factor 1.0, a recorded mechanical approximation to upstream EquationElem MathSize::Display/Text while full math layout remains incomplete; do not confuse it with complete equation-size parity.

Register globals in canonical eval root scope using original Func::native names/pointers. Qualified eight above are same functions/arguments/fields/content, no namespace wrappers; function equality/identity behavior stays with its owner. Bb/Frak/Inline/Mono/Script/Serif are outside that measured projection slice, not removed/denied.

One private wrap_math_style(args,name,kind:Option<MathStyleKind>,bold:Option<bool>,italic:Option<bool>,default_cramped:Option<bool>,allow_cramped_named:bool)->SourceResult<Value>. Only Script/SScript/Display/Inline permit cramped Bool, defaults true/true/false/false respectively; all others reject named. Explicitly select first invalid key in existing order, no fictitious loop/reorder/extra acceptance.

Exact common public diagnostics: missing `missing argument: body`; extra positional `unexpected argument`; unconvertible body `expected content, found <tipo vanilla>`; unknown named `unexpected argument: <nome>`; nonbool cramped `expected boolean, found <tipo vanilla>`. Canonical long type names include Int integer/Str string/Bool boolean. Mono/Script use provided Args call span for absent/invalid/extra/named/cramped errors, other identities keep their span policy. For only Mono/Script and exactly named body, preserve diagnostic `the argument body is positional` with separate hint `try removing body:`; other unknown names unchanged, Script cramped remains valid. Keep body/kind/cramped/default/evaluation order/morphology/layout and shared helper, no widened identity exception.

Composition is MathLayouter's responsibility: inner same-axis setter wins, orthogonal properties compose; size wrapper Inline does not erase glyph variant and cramped stays subtree-local. Complete Greek/digit coverage remains outside this Latin-prioritized scope.

## Acceptance and limits

Test direct Content/Str MathText wrapping, nested axis precedence/orthogonal flags, absent/extra/unknown/wrong-type body, four cramped defaults/overrides/types, global-qualified pointer identities and exact span/hint restrictions. Preserve public signatures/pipeline/1:1 ownership/hash lineage/build/full suite/lint. No declared historical completion count substitutes current scope, and unmeasured qualified routes/full math rendering remain excluded.
