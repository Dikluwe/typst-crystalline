# L0 — compiler/eval/repr
Hash do Código: 1b2eeb83

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/repr.rs`

Após a atomização de 2026-09-30, este consumer é a fachada pública estática e abriga os testes históricos. Os Prompts L0 individuais em `00_nucleo/prompts/compiler/eval/repr/` possuem Value, medidas, pintura, matemática, Content, Selector e formatação de listas. As cláusulas abaixo permanecem contexto normativo desses owners; a extração não altera a morfologia pública.
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/repr.rs:28` — `pub fn repr_value`.
- `01_core/src/compiler/eval/repr.rs:202` — `fn repr_duration`.
- `01_core/src/compiler/eval/repr.rs:242` — `fn repr_relative`.
- `01_core/src/compiler/eval/repr.rs:252` — `fn repr_math_lr_size`.
- `01_core/src/compiler/eval/repr.rs:265` — `fn round_with_precision`.
- `01_core/src/compiler/eval/repr.rs:276` — `fn format_float_with_unit`.
- `01_core/src/compiler/eval/repr.rs:283` — `fn format_float_component`.
- `01_core/src/compiler/eval/repr.rs:287` — `fn format_float_rounded`.
- `01_core/src/compiler/eval/repr.rs:303` — `fn repr_length`.
- `01_core/src/compiler/eval/repr.rs:315` — `fn repr_ratio`.
- `01_core/src/compiler/eval/repr.rs:321` — `fn repr_angle`.
- `01_core/src/compiler/eval/repr.rs:330` — `fn repr_color`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/args.rs:455-459`.
- `foundations/content/mod.rs:625–638`.
- `foundations/content/mod.rs:626-640`.
- `foundations/func.rs:460-470`.
- `foundations/module.rs:174-179`.
- `introspection/counter.rs:519-523,557-565`.
- `introspection/state.rs:370-374`.
- `lab/typst-original/crates/typst-library/src/foundations/array.rs:1188-1199`.
- `lab/typst-original/crates/typst-library/src/foundations/content/mod.rs:626-640`.
- `lab/typst-original/crates/typst-library/src/foundations/context.rs:65-83`.
- `lab/typst-original/crates/typst-library/src/foundations/selector.rs:309-321`.
- `lab/typst-original/crates/typst-library/src/math/op.rs:49-54`.
- `math/mod.rs:141-149`.
- `typst-library/src/text/space.rs:10-45`.
- `visualize/color.rs:2300-2303,2320-2323,1399-1415`.
- `visualize/stroke.rs:306-320`.

## Contract

Own exhaustive, deterministic public morphology for Value, Content and Selector,
including escaping, short/multiline formatting and recursive constructor fields.
Do not mutate payload, evaluate functions/context, derive language equality from
repr, use Debug, reconstruct erased syntax or claim general round-trip capability.

## Value and container projection

Preserve scalar and domain forms, negative zero/nonfinite spelling, quoted string
escaping, decimal/version/duration/color/gradient/tiling/bytes and all existing
variant formatters. Symbols retain every Unicode scalar, including variation
selectors/ZWJ, and their variant morphology. A valid literal label uses <name>;
other labels use label(<quoted repr>). Empty dictionary is (:), array is ().
Named modules use exactly <module NAME> from their stored name; aliases preserve
that name. Never identify stdlib from bindings/path or relabel an ordinary std.typ.
Functions retain native public names and anonymous closure/With `(..) => ..`;
unnamed native behavior stays unchanged, and Func::name() is not changed here.

State prints state(<key repr>, <init repr>) without losing either value.
Counter Page prints counter(page), string keys are quoted, selector keys recurse.
Location remains opaque location(..). A ContextBlock always prints context(),
independent of body/function/id/span; do not inspect its contents or use ellipses.

Array repr keeps the first min(len,40) items, then literal
`.. (<len-40> items omitted)` when needed, including plural 'items' for one.
This is repr-only elision: no payload truncation or limit in generic helpers,
Args, dictionary fields, Content sequences or constructor lists. Real nested arrays
apply their own elision. Tuples retain singleton comma; Args do not add that comma.
The canonical list formatter joins ', ' horizontally and uses multiline on an
already-multiline child or above the existing 50-byte width boundary. Reindent
all child lines by two spaces per level, use item/field trailing commas and close
at the container indentation. Historical ASCII calibration is evidence for that
scope, not proof of every non-ASCII vanilla width convention.

Args Some occurrence carrier prints every positional/named value in causal order,
including repeated names; no elision or metadata. Args None retains legacy
named-first projection. All textual serialization fallbacks deliberately reuse
this same morphology, including CBOR string fallbacks; they do not become a
structured serializer or promise byte preservation across morphology corrections.

LocatedContent Some formats the complete frozen snapshot in stored order excluding
label, recursing on field values and using the canonical element constructor name.
Never print Location, Arc, set_fields or baked styles. Raw/None keep their current
content projection. Equality compares fields under its separate owner, not this
string. Do not hardcode example Heading bodies, supplement or defaults.

For a Stroke with no explicit cap/join/dash/miter-limit and the default resolved
pair, both automatic paint/thickness print stroke(); paint alone prints paint repr,
thickness alone prints length and both print <thickness> + <paint>. Preserve complex
parenthesized strokes and internal non-default fallbacks. Presence comes from
StrokeFields, never guessed from resolved value. The constructor's positional
round-trip limitations remain outside this formatter contract.
SpotColorant prints color.spot(name-or-none, fallback-repr); a spot Color composes
that colorant repr and canonical .tint(ratio), never projects its fallback as the
color itself. to-hex is a separate explicit operation.
Relative prints both ratio and absolute components even when absolute is zero,
because Ratio is the separate pure-percent variant; preserve canonical units.

## Content and mathematical morphology

SpaceFromNewline and Space both print [ ]; origin stays internal. Value repr does
not normalize realized whitespace. Boxed with width_fraction prints
box(width: <fraction>fr, body: <body>); other boxes retain their form. HtmlElem uses
tag/attrs/body in that order, with canonical full short or structural multiline
form, ordered attributes and escaping, not whitespace-normalized equivalence.
OutlineEntry prints entry with its current morphological fields; a realized
snapshot prints level/element/fill in order.

Use causal presence, never default-value comparison, for optional public fields:

- attach prints base and slots t,b,tl,bl,tr,br in canonical order. Omitted slot
  is absent, ExplicitNone prints none, Present(Empty) prints []. Every Present
  delegates to the canonical math-field formatter. Canonical binom recognition
  remains prior to its internal line=false fraction carrier and preserves lower
  children in order.
- root always prints radicand; an explicit index prints index first, with none
  distinct from an omitted field. limits prints body then inline only when its
  explicit bit is true (including true); scripts prints only body.
- op always prints text then limits, including native/simple/user operators.
  No nominal special cases for lim/csc/dim/dif. MathClassOverride always prints
  class(class: <string>, body: <body>) in that order. MathStyled with any explicit
  math axis (variant/size/cramped/bold/italic, even default/Plain) retains
  styled(child: <body>, ..), not a flattened child or a nominal kind constructor.
  Mono/script values carrying those axes use this same projection.
- cancel prints body then explicit length/inverted/cross/angle/stroke/background
  in that order. Callback uses public repr and stroke the canonical formatter.
  underline prints underline(body: ...); flush prints flush().
- vec prints explicit delim/align/gap then children tuple, keeping zero/singleton
  and relative gap. Only direct MathIdent/MathText children project through the
  escaped textual content formatter as [text]; stored variants remain unchanged.
  Other/structured children use their own repr, without recursive bracket injection
  or double-wrapping Text/strong/emph. Inner lr/frac formatters retain ownership.
- line=true frac prints num/denom then optional explicit style. Parenthesis-origin
  bits never print; binom does not acquire a style field.
- accent prints base and escaped normalized accent, then explicit size/dotless.
  Do not print derived position or realized dotless state.
- cases prints explicit delim/reverse/gap then children tuple. Missing delimiters
  project none, not an internal sentinel; layout splitting at & does not rewrite
  stored morphology.
- mat prints explicit delim/align/augment/row-gap/column-gap then rectangular rows
  tuple of tuples, retaining Empty padding, signed offsets and stroke auto/custom.
  Explicit gap projects to both axes, never its own gap field. A single vertical
  auto-stroke augment uses integer form, others the canonical dictionary.
- lr prints explicit size before body and preserves empty/sequence/nested lr;
  mid prints body. Do not infer either identity from a MathDelimited or a glyph.
- overbrace/overbracket/overparen/overshell and their under counterparts print
  each own constructor with body then annotation only when causally explicit,
  including none. Mathematical overline prints overline(body: ...), not textual
  overline[...] or a substituted underline. primes prints primes(count: n) for
  every count, never a derived glyph/string. stretch prints body then explicit size.

Cascades/derived geometry never manufacture morphological fields. Received curly
quote glyphs cannot recover a lost SmartQuote token; print actual received content,
not a nominal/glyph heuristic. Unknown structured-child parity remains Unknown
rather than being masked by a vec-specific projection.

## Selector projection and acceptance

Element prints the canonical function name followed by .where and one ordered
named group; an empty group prints (:). Retain recursive values, escaping and
multiline formatting; do not flatten to Kind, reorder fields or chain Where calls.
Before/After print <base>.before(<end>) or <base>.after(<start>), adding
`, inclusive: false` only when false. Other existing selector variants preserve
their existing forms. Counter and textual fallbacks reuse these formatters.

Verify presence-equal-to-default, omitted/none/empty distinctions, nested multiline,
escaping, complete underlying array data, field order, constructor identities and
fallback propagation against independently sourced expectations. Non-ASCII width,
general complex-Content parity and unavailable bindings are not presumed successes.
No payload, hash, public equality, signature, eval/layout phase or export behavior
is authorized to change to repair repr.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: 00_nucleo/diagnosticos/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Adaptar somente construção Value Stroke nos testes históricos pelo From existente; repr permanece igual e a fachada pública não muda. Não alterar formatter filho ou reescrever expectativas.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.

Ratificação Memória 5: o dono confirmou o manifesto SHA-256
6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237
com “confirmo” em 2026-10-08. O gate ADR-0127 deste pacote está satisfeito;
suas cláusulas propostas passam a vigorar nesta materialização.
