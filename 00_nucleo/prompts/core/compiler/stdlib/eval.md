# L0 — stdlib/eval
Hash do Código: a357932d

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/eval.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/eval.rs:54` (pub fn native_eval); `01_core/src/compiler/stdlib/eval.rs:189` (fn eval_synthetic_root) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own native_eval re-evaluation using the existing NativeWithEngine ABI. Require one String source. mode defaults to code and accepts only code/markup/math; scope is a Dictionary. Preserve `missing argument: source`, `expected string, found TYPE`, `unexpected argument`, `unexpected argument: NAME`, `expected dictionary, found TYPE`, and `expected "markup", "math", or "code"` with the current long-type suffix for invalid types.

Delegate parsing to `00_nucleo/prompts/core/compiler/parse/mod.md` using parse_anchored and the supplied source anchor. Preserve real syntax messages/hints and semantic diagnostics; never replace them with a generic parse failure. A detached anchor remains detached. Argument-origin transport belongs to the call dispatcher; missing/spread/named/extra anchor limitations must remain explicit rather than fabricated from text.

Create fresh Scopes from the caller's base stdlib only; caller lexical variables are invisible. Add scope dictionary bindings in a confined scope and evaluate with a local Engine so let/set/show do not leak. Math results are inline Equation content, not a new render phase. Code returns its evaluated Value and markup returns Content.

Preserve the fresh-scope contract even when archived probes used caller-visible variables. file is not a supported argument. No filesystem, layout/render, new Value kind or additional public ABI is introduced.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
