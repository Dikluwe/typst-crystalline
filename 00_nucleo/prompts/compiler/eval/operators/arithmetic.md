# Prompt L0 — `compiler/eval/operators/arithmetic` — aritmética, unários e lógica booleana
Hash do Código: 082d2ebb

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/operators/arithmetic.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/operators.md`
**ADRs**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0028/ADR-0029 (tipos tipográficos)

---

## Contexto

Braços aritméticos do dispatcher de operadores. Toda a semântica é medida
contra o vanilla (`foundations/ops.rs`, `layout/length.rs`, `layout/rel.rs`,
`typst-utils/scalar.rs`) — as referências `file:line` abaixo são a medição,
não decoração.

## Instrução

### Gate de divisão por zero (pré-match)

Antes do `match`, `BinOp::Div` com divisor zero erra `"cannot divide by zero"`
para: `Int(0)`, `Float(0.0)`, `Decimal(0)`, `Length` zero, `Relative` com
`abs` zero e `rel == 0.0`, `Ratio(0.0)`, `Angle(0.0)` (paridade do `is_zero()`
genérico do vanilla, `foundations/ops.rs:344-359`).

### Tabela de aritmética binária

| Combinação | Resultado | Paridade vanilla |
|---|---|---|
| `Int ± Int`, `Int * Int` | `Int`, checked — overflow → `"number too large"` | `ops.rs` (`checked_*`) |
| `Int / Int` | `Float` (não truncamento: `5/2 = 2.5`) | `ops.rs` |
| `Float` op `Float`, `Int ↔ Float` mistos | `Float` (IEEE propagado silenciosamente, sem guarda NaN/Inf) | `ops.rs` |
| `Decimal op Decimal` (`+` `-` `*` `/`) | `Decimal` homogéneo (via `rust_decimal`, 28 dígitos) | `foundations/decimal.rs` |
| `Decimal op Int`, `Int op Decimal` (`+` `-` `*` `/`) | `Int` é convertido exatamente por `Decimal::from(i64)`; resultado `Decimal`; operações checked, falha → `"value is too large"` | `ops.rs:102-111,183-192,227-236,301-310`; P1219 |
| `Duration ± Duration` | `Duration` checked em nanos — overflow → erro | — |
| `Duration * Int|Float` (ambas as ordens) | `Duration` | — |
| `Duration / Int|Float` | `Duration`; divisor zero → erro | — |
| `Duration / Duration` | `Float` (rácio de nanos); divisor zero → erro | — |
| `Str + Str` | concatenação | `ops.rs` |
| `Str * Int` / `Int * Str` | repetição; `n < 0` → `"number must be at least zero"`; overflow de bytes → `"cannot repeat this string {n} times"` | `str.rs:92-99`, `ops.rs:272-273` |
| `Symbol + Symbol`, `Str ↔ Symbol` | `Str` (concatenação dos grapheme clusters integrais) | `ops.rs::add`:129-138 |
| `Content + Content`, `Content ↔ Str`, `Content ↔ Symbol` | `Content` sequência (`Content::sequence`/`Content::text`) | `ops.rs::add` — o vanilla coage via `TextElem::packed`; o cristalino usa `Content::text`, mesmo observável (ADR-0107) |
| `Array + Array` | concatenação ordenada, sem dedup | `array.rs:1203-1216` |
| `Dict + Dict` | merge — direita vence em colisão, posição da primeira ocorrência preservada (`IndexMap::extend`) | `dict.rs:388-404` |
| `Array * Int` / `Int * Array` | repetição `cycle().take(len*n)`; `n < 0` → `"number must be at least zero"`; overflow → `"cannot repeat this array {n} times"` | `array.rs:140-147`, `ops.rs:274-275` |
| `Length ± Length` | `Length` (componentes abs/em separadas) | `ops.rs:194,196` |
| `Length * Int|Float`, `Length / Int|Float` (todas as ordens) | `Length`, escala uniforme, NaN → 0 por componente via `sanitize_length_nan`; inf propaga-se | `ops.rs:238-243,312-314`; `scalar.rs:30-32` |
| `Length / Length` | `Float` — só se ambos `abs` zero (rácio de `em`) ou ambos `em` zero (rácio de `abs`); misto → `"cannot divide these two lengths"` | `length.rs:64-72` (`try_div`) |
| `Length + Color` (ambas as ordens) | `Stroke` sólido com a espessura da parte absoluta | stroke syntax vanilla |
| `Length + Gradient` (ambas as ordens) | `Stroke` com `Paint::Gradient` e a espessura da parte absoluta | P1229: necessário para gradient no papel stroke; medido nas fixtures S1–S8 do vanilla ratificado |
| `Relative ± Relative`, `Relative ± Length` | `Relative` | `rel.rs` |
| `Relative * Int|Float`, `Relative / Int|Float` | `Relative`, escala | `rel.rs` |
| `Relative / Relative` | `Float` — `rel` ambos zero → rácio de `abs` (regra de `Length/Length`); `abs` ambos zero → `rel/rel`; misto → `"cannot divide these two relative lengths"` | `rel.rs:128-137` |
| `Relative + Color` (ambas as ordens) | `Stroke` (espessura = parte absoluta) | stroke syntax vanilla |
| `Ratio ± Ratio` | `Ratio` | `ops.rs` |
| `Ratio + Length` (ambas as ordens), `Ratio - Length`, `Length - Ratio` | `Relative` | medido: `type(50% + 1pt) == relative` |
| `Ratio * Int|Float` (ambas as ordens), `Ratio / Int|Float` | `Ratio` | `ops.rs` |
| `Ratio * Fraction` (ambas as ordens) | `Fraction` | medido: `100% * 2fr` = `2fr` |
| `Ratio / Ratio` | `Float` | `ops.rs:323` |
| `Fraction + Fraction` | `Fraction` | `ops.rs:127` |
| `Angle - Angle` | `Angle` | `ops.rs:194` |
| `Angle * Int|Float`, `Angle / Int|Float` (ambas as ordens) | `Angle` (via radianos) | `ops.rs:241-246,317-319` |
| `Angle / Angle` | `Float` (rácio de radianos) | `ops.rs:317-319` |
| `Align + Align` | merge para `Align2D`; conflito de eixo → `"cannot add two horizontal alignments"` / `"cannot add two vertical alignments"` / `"cannot add two 2D alignments"` | semântica vanilla |
| `Bool and Bool`, `Bool or Bool` | `Bool` (sem short-circuit aqui — esse vive no dispatcher central, ver hub) | `ops.rs` |

### Operadores unários (`eval_unary_op`)

- `Neg`: `Int` (checked → `"number too large"`), `Float`, `Decimal`,
  `Length` (nega componentes), `Relative`, `Angle` (via radianos), `Ratio`,
  `Fraction`, `Duration` — paridade `foundations/ops.rs:80-84`.
- `Pos`: `Int`, `Float`, `Decimal`, `Length`, `Angle`, `Ratio`, `Relative` e
  `Fraction` (identidade), conforme `foundations/ops.rs:45-53`.
- `Not`: `Bool`.
- Fronteira unária conforme `error_formatting.md`: spelling da linguagem,
  nomes longos e distinções `unary '+'`/`'+'` e `unary '-'`/`'-'` medidas.

### `sanitize_length_nan`

NaN → `0.0` por componente (`abs`, `em`) de `Length`; inf não é tocado.
Paridade do efeito observável de `Scalar::new` no vanilla
(`typst-utils/src/scalar.rs:30-32`): medido `repr(1pt * float.nan)` = `0pt`,
`repr(1pt * float.inf)` = `float.inf * 1pt`. Aplica-se nos braços
`Length * Int|Float` e `Length / Int|Float` — deliberadamente **no** braço do
eval, não em `Length::mul`/`Length::div` (disciplina um-bug-por-passo).

## Restrições Estruturais

- L1 puro: zero I/O, zero estado; só `Value` in, `Value` out (ver invariante 1 do hub).
- **Coerção Decimal limitada a Int**: as oito combinações `Decimal ↔ Int`
  convertem o inteiro exatamente, nunca por `f64`. Isso não autoriza
  `Decimal ↔ Float` nem uma torre numérica genérica.
- Scope-outs medidos (não reabrir sem medição nova): divisões mistas
  `Length ↔ Relative` e `Ratio ↔ Relative` (`ops.rs:315,324,328-329`);
  `Length * Ratio` / `Ratio * Length` (`ops.rs:240,243`); `Dict * Int`
  (inexistente no vanilla — medido); `%` como remainder e `//` (não existem
  no parser).

## Critérios de Verificação

```
eval_binary_op(Add, Int(1), Int(2))            == Int(3)
eval_binary_op(Div, Int(5), Int(2))            == Float(2.5)
eval_binary_op(Add, Int(i64::MAX), Int(1))     == Err("number too large")
eval_binary_op(Div, _, Int(0))                 == Err("cannot divide by zero")

// Decimal homogéneo e coerção exata com Int
eval_binary_op(Add, Decimal(1.5), Decimal(2.5)) == Decimal(4.0)
eval_binary_op(Div, Decimal(10), Decimal(3))    == Decimal(3.333…)
eval_binary_op(Div, Decimal(1), Decimal(0))     == Err("cannot divide by zero")
eval_binary_op(Add, Decimal(1.5), Int(2))       == Decimal(3.5)
eval_binary_op(Sub, Int(2), Decimal(5.5))       == Decimal(-3.5)
eval_binary_op(Mul, Int(3), Decimal(1.5))       == Decimal(4.5)
eval_binary_op(Div, Decimal(7.5), Int(2))       == Decimal(3.750)
eval_binary_op(Div, Int(2), Decimal(0.5))       == Decimal(4.0)
eval_binary_op(Add, Decimal(MAX), Int(1))       == Err("value is too large")
eval_unary_op(Neg, Decimal(1.5))                == Decimal(-1.5)

// repetição
eval_binary_op(Mul, Array[0], Int(3))           == Array[0,0,0]
eval_binary_op(Mul, Int(3), Array[0])           == Array[0,0,0]
eval_binary_op(Mul, Array[1,2], Int(-1))        == Err("number must be at least zero")
eval_binary_op(Mul, Str("ab"), Int(2))          == Str("abab")
eval_binary_op(Mul, Dict{:}, Int(2))            == Err (fronteira — Dict * Int não existe no vanilla)

// Str/Symbol/Content cruzados em Add
eval_binary_op(Add, Str("⟨"), Content(x))       == Content("⟨x")
eval_binary_op(Add, Content(x), Str("|"))       == Content("x|")
eval_binary_op(Add, Symbol(a), Symbol(b))       == Str(ab)
eval_binary_op(Add, Str(a), Symbol(b))          == Str(ab)
eval_binary_op(Add, Str("a"), Symbol("♥️"))     == Str("a♥️")
eval_binary_op(Add, Symbol("👩‍💻"), Str("!"))   == Str("👩‍💻!")

// Length/Relative/Ratio
eval_binary_op(Sub, Length(2em), Length(5em))   == Length(-3em)
eval_binary_op(Div, Length(2cm), Length(1cm))   == Float(2.0)
eval_binary_op(Div, Length(10pt+1em), Length(5pt)) == Err("cannot divide these two lengths")
eval_binary_op(Mul, Length(1pt), Float(NaN))    == Length(0pt)   // NaN → 0
eval_binary_op(Mul, Length(1pt), Float(inf))    == Length(inf pt) // inf propaga-se
eval_binary_op(Div, Relative(50%), Relative(25%)) == Float(2.0)
eval_binary_op(Add, Ratio(50%), Length(1pt))    == Relative(50% + 1pt)
eval_binary_op(Mul, Ratio(100%), Fraction(2fr)) == Fraction(2fr)
eval_binary_op(Add, Length(4pt), Gradient(g)) == Stroke(paint: Gradient(g), thickness: 4pt)
eval_binary_op(Add, Gradient(g), Length(4pt)) == Stroke(paint: Gradient(g), thickness: 4pt)

// Angle/Fraction/Align/unários
eval_binary_op(Sub, Angle(90deg), Angle(45deg)) == Angle(45deg)
eval_binary_op(Div, Angle(30deg), Angle(30deg)) == Float(1.0)
eval_binary_op(Add, Fraction(1fr), Fraction(2fr)) == Fraction(3fr)
eval_binary_op(Add, Align(center), Align(bottom)) == Align2D(h:center, v:bottom)
eval_binary_op(Add, Align(left), Align(right))  == Err("cannot add two horizontal alignments")
eval_unary_op(Neg, Angle(15deg))                == Angle(-15deg)
eval_unary_op(Neg, Duration(3s))                == Duration(-3s)
```

## Resultado Esperado

- Os braços aritméticos/booleanos de `eval_binary_op`, `eval_unary_op` e
  `sanitize_length_nan` conforme a tabela; testes unitários no próprio
  ficheiro cobrindo as linhas acima; zero regressão na suite do eval.

## P1307-R4 — encaminhamento público de Args + Args

### Medição anterior à decisão

No baseline R4 SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree capturado,
`01_core/src/compiler/eval/operators/mod.rs:40-41` encaminha Add a este nó.
Seu match em `arithmetic.rs:54-134` aceita Array/Dict, mas não Args. O focal
independente `args.join-duplicates`, encerrado em
`2026-09-07T17:37:05.017565+00:00`, observa sucesso no vanilla ratificado
`a51e02804` e erro `cannot add arguments and arguments` no baseline.
Fonte upstream `lab/typst-original/crates/typst-library/src/foundations/ops.rs`
delega o par Args ao Add de `foundations/args.rs:468-482`. Recibo e texto
prévio deste L0 estão em `00_nucleo/diagnosticos/p1307-r4-contract-refinement.json`.

### Decisão de roteamento da semântica já aprovada

Adicionar somente o braço `(BinOp::Add, Value::Args, Value::Args)`, delegando
a `join` do owner `compiler/eval/operators/join.md`. Esse owner mantém
exclusivamente o algoritmo de remoção dos named LHS presentes RHS e
concatenação causal; não repetir a lógica aqui. Match permanece fechado e
estático, sem nova API/trait, avaliação ou fase. Não generalizar para todos
os pares aceitos por join: Args+None ficou fora de R4 e é reaberto por P1308 abaixo.

Verificar o operador real, duplicatas/colisões/spans e a comparação com With,
além dos controles Array/Dict e aritmética vigente. É implementação interna
do contrato Args+Args aprovado em P1307-R3, em fluxo contínuo ADR-0127;
a auditoria refutou a suficiência do owner join sozinho, não amplia a família
encode nem o contrato público de Args.

## P1308 — identidade None para Args em Add

### Medição anterior à decisão

Baseline `p1308-baseline.json` SHA-256
`62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`,
HEAD b303f1f15 mais working tree ali integralmente identificada.
O recibo público `p1307-r6-public-matrix-2.json`, SHA-256
`53737a33e636323f4e0ab50499bde3cbb0905fe50889cdc5d3d7c5d631f6e51c`,
mede erro em r4.args.join-none; a expectativa vanilla ratificada é identidade.
Fonte `lab/typst-original/crates/typst-library/src/foundations/ops.rs:94–95`
tem identidade None em ambas as ordens. O helper cristalino join já a possui;
o dispatcher Add deste owner ainda não aceita esses pares.

### Decisão autorizada

Adicionar `(Add, Args(a), None)` e `(Add, None, Args(a))` retornando o mesmo
Args, inclusive seu span agregado, occurrences, duplicatas e views. Não
reconstruir via Args+Args (esse join destaca span); não ampliar outros pares.
Args+Args continua delegado exclusivamente ao owner join. É semântica da
linguagem, não igualdade Rust; autorização P1308 resolve a exclusão R4.
Testar valor, ordem e origem através do operador público, ambos os lados,
Args vazio e erro posterior. Refuta a solução perder qualquer origem ou
alterar Array/Dict/outros operadores. Sem API pública ou mudança de fase.
