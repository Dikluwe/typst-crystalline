# L0 — compiler/eval/rules/set/text — Set text and lexical transport
Hash do Código: f87ab12f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:41e38095ab33ab1710ba11f43d6a5ca50def5a703f77a0001a066b72964c4165
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:dc3d1eff8516cab7b246430e867fd906289164509c4533e071be543f33b210b6
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:c8570ec5f4f32c0805587230cafc796f35e5ee7c9f8e634e13d807db2afb6e00

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set/text.rs`

## Extraction evidence and contract

set.rs dispatcher after historical P1499 extraction keeps text arm contiguous.
L1 baseline in
`lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz`
and suite after that extraction passed6401 tests/two ignored. Structural evidence,
not benchmark/fresh vanilla comparison.

Preserve property schemas/type validation/defaults/precedence/scope-out warnings
and custom text.* transport transparent to morphology. text.hyphenate accepts
bool/auto; costs/lexical policy follow pinned nuclei. Dispatcher calls statically
without arm reorder. Keep errors/spans/argument evaluation/value identity/style
absence-presence/execution phase. Historical decisions/measurements remain in
`00_nucleo/prompts/core/compiler/eval/rules.md`. L1/crystalline-lint without
regression; language ADR-0107/0108,1:1 ownership/pins ADR-0129/0130.

## Ratified complete text size

### Evidence before decision

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`,
before-measurement.json.gz, morph-before-measurement.json.gz and audit.json
preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC, status/diffstat, sources/L0/binaries/argv and historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. named-em measures A at 0pt versus
24pt; affine 3pt versus 27pt; nested over 18pt measures 0pt versus 36pt. All six
Size/Color/body permutations are accepted by the pin and rejected by Tekt.
Positional set 18pt is ignored (12 versus 18pt). Pinned text/mod.rs:290,303,
924–930,1136–1142 and args.rs:178–247 establish typed search, named preference,
remaining body and composition. style_chain.rs:513–517 and constructor.rs:123
discard em; collapse at style_chain.rs:394–395,419–422 uses top-wins where size
needs a fold. Size/scope is observable language; Rust representation is an
internal choice. Infer no historical intent. Restoration, Raw, heading, math,
capture/show, duplicates/spans, zero/negative or metric failures can refute it.
The equality probe refuted comparing declared values: the pin returns true for
text(size:2em)[A] == text(size:24pt)[A]. Pinned StyledElem::eq at
content/mod.rs:779–782 compares child. That equality defect requires independent
correction; a complete size type does not close that axis.

### Ratification and limits

The human approved the complete-size package and its two public contracts in
manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba`.
Require implementation and S1–S16 tests under 12 GiB/zero swap; no release/merge/
push. Reseal only after implementation and a valid graph. RG1 remains independent.

### Exclusive property selection and unresolved Paint boundary

Set text no longer ignores positional Size Length/Color: typed selection/named
preference, validate leftovers with once-only evaluation/order/causal occurrences.
Transport text.size as Length without resolving em. StyleChain owns fold, not
this dispatcher. Keep capture/lexicality/other properties.

Explicitly classify all three Arg variants when selecting font property's AST.
Uniform V19 arguments lacking matching named font expression use evaluated
occurrence without reevaluating unrelated positional/spread/named args. Validate
named solid fill; None/other nonpaints use Paint cast error. Gradient/Tiling and
legacy font validators are independent work. Preserve earlier partial Gradient/
Tiling transport as explicit uniform V19 no-effect group here until Paint work;
this scope-out is no invalid cast. None/nonpaints now fail. Solid color or
contradictory message does not close Paint.

Initial complete-size SARIF has two new V17 here (AST lookup/font-array validation).
Split exhaustive variant selection/conditions into arm body, no suppression,
preserving diagnostics/evaluation. Residual Pos/Spread V19 is uniform: no named
font AST expression, use evaluated occurrence.

## Shared selection-nucleus pin evidence

Measurements/shape-RTL refinement in
`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18`
precede new shared selection pin. This owner receives only pin; executable code/
API/phase/other responsibilities remain unchanged in that refinement.
