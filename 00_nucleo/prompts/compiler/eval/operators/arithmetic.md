# Prompt L0 — `compiler/eval/operators/arithmetic` — aritmética, unários e lógica booleana
Hash do Código: 4be32591

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
| `Duration ± Duration` | `Duration` checked em nanos — overflow → erro | — |
| `Duration * Int|Float` (ambas as ordens) | `Duration` | — |
| `Duration / Int|Float` | `Duration`; divisor zero → erro | — |
| `Duration / Duration` | `Float` (rácio de nanos); divisor zero → erro | — |
| `Str + Str` | concatenação | `ops.rs` |
| `Str * Int` / `Int * Str` | repetição; `n < 0` → `"number must be at least zero"`; overflow de bytes → `"cannot repeat this string {n} times"` | `str.rs:92-99`, `ops.rs:272-273` |
| `Symbol + Symbol`, `Str ↔ Symbol` | `Str` (concatenação dos caracteres) | `ops.rs::add`:129-138 |
| `Content + Content`, `Content ↔ Str`, `Content ↔ Symbol` | `Content` sequência (`Content::sequence`/`Content::text`) | `ops.rs::add` — o vanilla coage via `TextElem::packed`; o cristalino usa `Content::text`, mesmo observável (ADR-0107) |
| `Array + Array` | concatenação ordenada, sem dedup | `array.rs:1203-1216` |
| `Dict + Dict` | merge — direita vence em colisão, posição da primeira ocorrência preservada (`IndexMap::extend`) | `dict.rs:388-404` |
| `Array * Int` / `Int * Array` | repetição `cycle().take(len*n)`; `n < 0` → `"number must be at least zero"`; overflow → `"cannot repeat this array {n} times"` | `array.rs:140-147`, `ops.rs:274-275` |
| `Length ± Length` | `Length` (componentes abs/em separadas) | `ops.rs:194,196` |
| `Length * Int|Float`, `Length / Int|Float` (todas as ordens) | `Length`, escala uniforme, NaN → 0 por componente via `sanitize_length_nan`; inf propaga-se | `ops.rs:238-243,312-314`; `scalar.rs:30-32` |
| `Length / Length` | `Float` — só se ambos `abs` zero (rácio de `em`) ou ambos `em` zero (rácio de `abs`); misto → `"cannot divide these two lengths"` | `length.rs:64-72` (`try_div`) |
| `Length + Color` (ambas as ordens) | `Stroke` sólido com a espessura da parte absoluta | stroke syntax vanilla |
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
- `Pos`: `Int`, `Float`, `Length` (identidade).
- `Not`: `Bool`.
- Fronteira unária: `"cannot apply {op:?} to {type_name()}"` — usa o nome
  **curto** do tipo (divergência de texto registada face aos nomes longos do
  vanilla; o nó `error_formatting.md` cobre a fronteira binária).

### `sanitize_length_nan`

NaN → `0.0` por componente (`abs`, `em`) de `Length`; inf não é tocado.
Paridade do efeito observável de `Scalar::new` no vanilla
(`typst-utils/src/scalar.rs:30-32`): medido `repr(1pt * float.nan)` = `0pt`,
`repr(1pt * float.inf)` = `float.inf * 1pt`. Aplica-se nos braços
`Length * Int|Float` e `Length / Int|Float` — deliberadamente **no** braço do
eval, não em `Length::mul`/`Length::div` (disciplina um-bug-por-passo).

## Restrições Estruturais

- L1 puro: zero I/O, zero estado; só `Value` in, `Value` out (ver invariante 1 do hub).
- **Sem coerção cruzada de `Decimal`**: misturas `Decimal + Int` etc. caem na
  fronteira genérica. Divergência registada face ao vanilla (que coage
  `Decimal ↔ Int`): sem consumidor medido; scope-out explícito.
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

// Decimal (homogéneo, sem coerção)
eval_binary_op(Add, Decimal(1.5), Decimal(2.5)) == Decimal(4.0)
eval_binary_op(Div, Decimal(10), Decimal(3))    == Decimal(3.333…)
eval_binary_op(Div, Decimal(1), Decimal(0))     == Err("cannot divide by zero")
eval_binary_op(Add, Decimal(1), Int(2))         == Err (fronteira)
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

// Length/Relative/Ratio
eval_binary_op(Sub, Length(2em), Length(5em))   == Length(-3em)
eval_binary_op(Div, Length(2cm), Length(1cm))   == Float(2.0)
eval_binary_op(Div, Length(10pt+1em), Length(5pt)) == Err("cannot divide these two lengths")
eval_binary_op(Mul, Length(1pt), Float(NaN))    == Length(0pt)   // NaN → 0
eval_binary_op(Mul, Length(1pt), Float(inf))    == Length(inf pt) // inf propaga-se
eval_binary_op(Div, Relative(50%), Relative(25%)) == Float(2.0)
eval_binary_op(Add, Ratio(50%), Length(1pt))    == Relative(50% + 1pt)
eval_binary_op(Mul, Ratio(100%), Fraction(2fr)) == Fraction(2fr)

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
