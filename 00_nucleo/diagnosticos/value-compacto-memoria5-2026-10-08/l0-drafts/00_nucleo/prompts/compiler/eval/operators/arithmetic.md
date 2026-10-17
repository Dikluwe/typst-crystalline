# L0 — compiler/eval/operators/arithmetic
Hash do Código: 2971fc48

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/arithmetic.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/arithmetic.rs:27` — `fn apply_binary`.
- `01_core/src/compiler/eval/operators/arithmetic.rs:518` — `fn sanitize_length_nan`.
- `01_core/src/compiler/eval/operators/arithmetic.rs:538` — `fn eval_unary_op`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/args.rs:468-482`.
- `foundations/ops.rs:344-359`.
- `foundations/ops.rs:45-53`.
- `foundations/ops.rs:80-84`.
- `lab/typst-original/crates/typst-library/src/foundations/ops.rs:94–95`.
- `typst-utils/src/scalar.rs:30-32`.

## Contract

Own Add/Sub/Mul/Div/And/Or, unary evaluation and sanitize_length_nan.
Before division dispatch, reject zero Int/Float/Decimal/Length/Relative/Ratio/Angle
with `cannot divide by zero`; retain Duration's own zero checks. A Relative
divisor is zero only when both components are zero. Keep this precedence even
when the dividend would otherwise be incompatible.

Preserve the supported binary domains and result types:

- Int addition/subtraction/multiplication are checked (`number too large`);
  Int division returns Float, not a truncated integer. Float and mixed Int/Float
  arithmetic propagates IEEE nonfinite values without a general NaN/Inf guard.
- Decimal arithmetic is homogeneous or mixed with Int converted exactly via
  Decimal::from(i64), never f64; checked failure is `value is too large`.
  This does not enable Decimal/Float coercion or a generic numeric tower.
- Duration addition/subtraction is checked in nanoseconds; integer/float scaling
  and division retain the current checked conversions, errors and nonfinite
  guards; Duration/Duration returns the nanosecond ratio as Float.
- Str+Str concatenates; Symbol+Symbol and Str/Symbol mixed addition returns Str
  preserving full graphemes (variation selectors and ZWJ included).
  Content with Content/Str/Symbol returns the existing content sequence.
- Array+Array concatenates without deduplication. Dict+Dict keeps first-key
  position and right-hand values. String/array repetition accepts Int on either
  side; negative repeats fail `number must be at least zero`; size overflow fails
  `cannot repeat this string {n} times` or `cannot repeat this array {n} times`.
- Length addition/subtraction keeps abs/em components separate. Scalar scaling
  and division sanitize each NaN component to zero, but preserve infinity;
  do not move this eval-specific behavior into the domain Length operators.
  Length/Length succeeds only on compatible pure axes, otherwise
  `cannot divide these two lengths`.
- Length+Color or Length+Gradient in either order constructs Stroke using the
  absolute thickness and the corresponding paint. Relative+Color in either
  order retains its current absolute-thickness stroke construction.
- Relative addition/subtraction with Relative/Length and scalar scaling/division
  preserve both components. Relative/Relative returns a Float only for compatible
  pure absolute or relative axes; otherwise
  `cannot divide these two relative lengths`.
- Ratio arithmetic and scalar scaling/division preserve Ratio; Ratio/Ratio
  returns Float. Ratio with Length in the supported + and - orders produces
  Relative. Ratio*Fraction in either order produces Fraction; Fraction+Fraction
  produces Fraction.
- Angle subtraction, scalar scaling/division and Angle/Angle preserve the current
  radians-based results. Alignment addition merges compatible axes to Align2D;
  preserve `cannot add two horizontal alignments`,
  `cannot add two vertical alignments` and `cannot add two 2D alignments`.
- Bool And/Or operate on already evaluated boolean operands; short-circuit is
  coordinated by 00_nucleo/prompts/compiler/eval.md, not here.

Do not infer reverse division or additional pairings from the word 'scaling'.
Unsupported pairs keep the canonical mismatch diagnostic. Mixed Length/Relative
and Ratio/Relative division, Length*Ratio, Dict*Int, remainder and // are not
enabled by this contract.

Args+Args Add delegates exclusively to
00_nucleo/prompts/compiler/eval/operators/join.md. Args+None and None+Args return
the same Args, preserving aggregate span, occurrences and views; do not reconstruct
through the detached-span Args join. Do not route every join-compatible pair into Add.

Unary Neg retains Int checked overflow and the supported Float/Decimal/Length/
Relative/Angle/Ratio/Fraction/Duration results. Pos is identity on the current
Int/Float/Decimal/Length/Angle/Ratio/Relative/Fraction domains; Not requires Bool.
Mismatch wording delegates to error_formatting. Tests distinguish checked
overflow, zero precedence, IEEE behavior, repetition, grapheme preservation,
component compatibility, paint construction and Args metadata conservation.

## D08B3-C — marcas fornecidas de stroke Length + Color

Medição anterior à decisão: table-cell-styles-d08b3c-2026-10-07/physical-complete.json.gz / stroke-with-body-recipe contém literal red+2pt, render black 1pt Tekt contra red 2pt vanilla, apesar da correção do consumer de bordas. Coleção identifica HEAD/working tree/fontes/binaries/UTC/argv/PDF/SVG. arithmetic.rs:336–344 usa ..Stroke::default() sem provided bits; realizer fold_stroke pode substituir esses componentes pelo default Table. Fonte pinada foundations/ops.rs produz Stroke::from_pair com os dois componentes Custom; geometry stroke model preserva ausência de outros componentes. É cor/espessura/getter da linguagem, não obrigação de igualdade Rust.

Decisão interna contínua ADR-0127: nos braços existentes Length + Color e Color + Length marcar somente specified.paint e specified.thickness como true. Preservar valores, casts, espessura absoluta, overhang, assinatura, outros braços e componentes não fornecidos. Não ampliar Gradient/Relative ou resolução em neste recorte. Teste RED→GREEN de ambas as ordens antes do fallback Table, vetor físico com cor/espessura corretas e suíte geral; sem API/fase/modo/default ou compatibilidade nova. Rich strokes/relative/em já fora de certificação continuam incompletos.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: 00_nucleo/diagnosticos/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Os três braços já existentes que constroem Stroke usam a conversão From<Stroke> preservada. Não mudar operações, provided bits, casts ou defaults e não ampliar Gradient/Relative nesta adaptação.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.
