# L0 — compiler/eval
Hash do Código: ce5c6327

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/mod.rs:81` — `pub fn realize_outlines`.
- `01_core/src/compiler/eval/mod.rs:92` — `pub fn repr_stroke_value`.
- `01_core/src/compiler/eval/mod.rs:100` — `pub fn repr_value_for_serialization`.
- `01_core/src/compiler/eval/mod.rs:110` — `pub fn pdf_standards_conflict_diagnostic`.
- `01_core/src/compiler/eval/mod.rs:252` — `enum ContextReadRequest`.
- `01_core/src/compiler/eval/mod.rs:331` — `struct ContextRead {`.
- `01_core/src/compiler/eval/mod.rs:345` — `struct ContextReads {`.
- `01_core/src/compiler/eval/mod.rs:738` — `pub struct EvalContext {`.
- `01_core/src/compiler/eval/mod.rs:905` — `pub enum EvalTarget {`.
- `01_core/src/compiler/eval/mod.rs:915` — `pub fn eval`.
- `01_core/src/compiler/eval/mod.rs:928` — `pub fn eval_expression`.
- `01_core/src/compiler/eval/mod.rs:940` — `struct ExpressionWorld<'a> {`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/value.rs:210–214`.
- `lab/typst-original/crates/typst-library/src/foundations/value.rs:343-362`.
- `lab/typst-original/crates/typst-library/src/lib.rs:374`.
- `lab/typst-original/crates/typst-library/src/lib.rs:397-401`.
- `layout/grid/mod.rs:422-438,574-677,767-768`.
- `model/table.rs:289-305,494-613,732-733`.
- `typst-bundle/src/lib.rs:67-76`.
- `typst-eval/src/call.rs:638`.
- `typst-eval/src/code.rs:148`.
- `typst-library/src/lib.rs:430`.
- `typst-realize/src/lib.rs:1261-1325`.

## Contract

Own EvalContext, public evaluation entrypoints, base-scope construction, the
two document passes, markup-stream coordination and the exhaustive Expr
dispatcher. Retain explicit Engine/Scopes/context state and the pinned core and
feature-gate invariants. No child behavior becomes hub-owned merely because
the hub calls or reexports it.

## Entrypoints and scope

Keep eval, eval_expression, eval_expression_with_features,
eval_expression_with_features_and_introspector, eval_with_full_error,
eval_with_full_error_and_target and eval_with_full_error_target_and_features with
their existing signatures, return values, sink behavior and defaults. Existing
default target is Paged; full_error defaults false and Features default empty.
Target Paged/Html/Bundle is explicit and does not enable a feature. Eval receives
resolved configuration, never reads environment or imports infra.

Acquire World inputs once for document evaluation and use the same values in both
passes. Build a fresh read-only Library base containing the composed stdlib,
predefined colors, text and supplied element-registry constructors. Local scopes
remain mutable/shadowable. The lexical binding std holds an independent clone of
the unshadowed stdlib in a Module publicly named global; do not create a global
binding called global. Ordinary local std bindings can shadow it normally.
Private make_stdlib wrappers delegate composition to
00_nucleo/prompts/compiler/stdlib/registry/mod.md and its family owners. Global
registration, namespace membership, color aliases, native PDF/HTML/asset identity
and compatibility math globals remain normative there, not copied into this hub.

Expression evaluation builds a numbered parse_code Source from world.main() and
the complete expression. The private per-invocation World overlay returns that
identical Source for its id and delegates every other World service/id unchanged,
including defaults, inputs, plugin/path/package/font/date services. Do not mutate
the caller's World or recreate its resources. The injected-introspector sibling
clones the supplied snapshot before evaluation; the ordinary feature-aware
entrypoint delegates with an empty snapshot and preserves its default behavior.
Keep parser diagnostics, scopes, route, sink, limits and feature transport.

A produced Func with detached private origin may acquire the producer-expression
span. Never overwrite a known closure/alias/With/factory/contextual origin, change
other Values, execute a function or make origin part of public equality. A Span
without its Source is insufficient for a resolvable call trace.

## Document coordination and dispatch

Propagate syntax errors with hints; markup empty-stars/underscores warnings remain
nonfatal sink diagnostics. Preserve the existing empty-source warning literal.
Run independent fresh-scope/context document passes with identical base inputs:
first disable show application and retain original introspection content; then
enable show application and return the official rendered content/module scope,
bibliography styles and document metadata. Keep existing warning deduplication
and page-column wrapping. Original and rendered content are distinct carriers,
not interchangeable fixpoint inputs. A residual document FlowEvent is forbidden
at its causal span. Do not shift introspection to a different phase here.

Expr dispatch remains static and exhaustive. Preserve literal/unit construction
(Percent -> Ratio), lexical lookup, arrays/dictionaries/spreads, blocks, contextual
content and first-error order. Code blocks accumulate through operators::join.
Boolean And/Or evaluate the left operand first, type-check as currently required,
and do not evaluate a short-circuited right operand. Assignment, calls, closures,
control flow, imports, markup and math delegate to their owners below. Retain the
first flow event until the responsible loop/function consumer handles it.

Markup captures SpaceFromNewline only when the whitespace token contains newline,
otherwise Space. Comments/trivia do not erase adjacent origin; Parbreak/Linebreak
remain distinct. Defer eager per-leaf textual show to the rules owner with the
causal lexical style/rule snapshot, so later declarations do not retroactively
affect earlier groups. A ContentBlock held as a value retains raw whitespace
until insertion; local textual-show matching preserves unmatched separators.
Normalization and realized-group matching are specified only by
00_nucleo/prompts/compiler/eval/rules/textual.md. Smartquote markup reads enabled,
alternative and canonical quotes from styles and delegates to the shared pure
quote resolver; it still emits Text and does not migrate to layout.

## Contextual observation coordination

Keep a private empty-per-evaluation log of the operations actually observed,
independent of introspector state. Store the effective request/arguments, span,
current file, location, in-context state, target/features, immutable styles/rules/
guards and complete result or error. Do not reconstruct requests from AST,
diagnostic strings, equal Values or a final global stack. Preserve query/locate/
here, state/counter (including label resolution and legacy requests), Location
and outline helper requests as their typed existing operations.

A counter read whose selector tree contains Element selects the block before
location/label/callback resolution, including failing reads. Traverse Where,
And/Or, Within and both Before/After branches without flattening them. Once
selected, every already-observed introspective entry participates in validation.
has_filtered_counter_reads remains a pure getter. The doc-hidden public
has_location_position_reads getter reports actual LocationPosition requests even
if their result is None/error; page, page-numbering/query/state/counter do not
select that demand. L3 owns the retry/fixpoint choice, not this getter.

context_reads_valid_for replays only recorded requests against the candidate
TagIntrospector with captured causal context and current supplied Engine resources,
delegating operation semantics to their owners. Do not reexecute the block body,
callbacks never demanded or append new observations while replaying. The private
relation comes exclusively from
00_nucleo/prompts/compiler/eval/observation.md: Same certifies preservation;
Different/Unproven, success/error transitions or changed full diagnostics invalidate
reuse. Mechanism failures propagate SourceDiagnostic; a stable observed error
does not turn the body into success.

context_nonconvergence_diagnostics uses the orchestrator's real ordered candidate
history and final-attempt requests projected onto it; it does not claim that the
body ran those requests in every round or invent details for opaque/alien reads.
OutlineEntryPage and OutlineEntryIndented remain distinct causal reads, not
counter/state substitutes; indented carries the actual entry/prefix/inner/gap and
private parent styles, while absent parent fails before registration.

## Delegation boundary

- 00_nucleo/prompts/compiler/eval/bindings.md owns the reexport hub and its children.
- 00_nucleo/prompts/compiler/eval/call_dispatch.md owns argument/call routing/traces;
  00_nucleo/prompts/compiler/eval/closures.md owns closure capture/application.
- 00_nucleo/prompts/compiler/eval/control_flow.md owns loops/conditionals;
  00_nucleo/prompts/compiler/eval/flow.md owns event representation/forbidden text.
- 00_nucleo/prompts/compiler/eval/operators.md delegates pure operator semantics.
- 00_nucleo/prompts/compiler/eval/modules.md owns imports/includes;
  00_nucleo/prompts/compiler/eval/markup.md owns specialized markup adapters;
  00_nucleo/prompts/compiler/eval/math.md owns mathematical syntax adapters.
- 00_nucleo/prompts/compiler/eval/rules.md owns set/show/realization, with
  00_nucleo/prompts/compiler/eval/selector_matching.md and
  00_nucleo/prompts/compiler/eval/show_rule_termination.md as independent owners.
- 00_nucleo/prompts/compiler/eval/font_dict.md, bibliography.md, table.md and cast.md
  own their exclusive helpers (their full canonical paths are in the owner map).
- 00_nucleo/prompts/compiler/eval/repr.md owns morphology formatting;
  00_nucleo/prompts/compiler/eval/observation.md and its children own internal
  observation comparison, distinct from both public equality and public repr.
- 00_nucleo/prompts/compiler/eval/tests.md and the independent test prompts own
  their exclusive test consumers, not this productive module.

The pure public repr_stroke_value and repr_value_for_serialization facades delegate
to repr without Debug, I/O or deciding structured serialization classes. Keep their
signatures and caller-owned classification. Public realize_outlines and
pdf_standards_conflict_diagnostic facades delegate to rules; they do not transfer
rule or PDF-schema ownership to this hub.

Preserve public contracts, defaults, compatibility and phases. Any ambiguous
change affecting these requires ADR-0127 direction before implementation.

### Refinamento de precedência medido antes da implementação

No estado precedence-state.json, regression-acceptance.json registra 191 verificações passando e duas falhas no controle show-par-sets-false. O Tekt emite extraordi-/interna- onde vanilla conserva palavras sem hifenização. show-par-full-tree.log mostra SetPage e o escopo global dentro de um Par sintetizado; a recipe fica exterior ao text.hyphenate true. eval/mod.rs:1532-1560 possui a proveniência exata dos escopos #set; rules.rs:660-727 apagou essa distinção ao tratar Styled sempre como inline. Não inferir origem pelo conteúdo do Styles.

Decisão interna: o fold dos escopos #set em eval_markup transporta marcador privado eval.flow.scope=true no Styles existente. Antes de formar o primeiro parágrafo material, realize_flow reconhece esse marcador e mantém o escopo exterior à realização dos seus parágrafos, preservando os carriers prévios SetPage/Empty/Space/SpaceFromNewline em ordem, sem os embrulhar em Par. Após conteúdo inline material o escopo permanece inline dentro do parágrafo corrente; wrappers de constructors/estilos sem marcador permanecem inline. Nenhuma entidade, assinatura, comportamento válido por defeito ou fase pública muda. Pin no núcleo compartilhado paragraph-composition documenta a distinção; não recorrer a heurística por chave de estilo. O consumer do hub só produz o marcador no fold; preparação e reconhecimento são do owner rules.
