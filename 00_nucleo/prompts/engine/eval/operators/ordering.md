# Prompt L0 — `engine/eval/operators/ordering` — ordenação de valores
Hash do Código: 8e797556

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/eval/operators/ordering.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/engine/eval/operators.md`
**ADRs**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir)

---

## Contexto

A ordenação da linguagem (`<` `<=` `>` `>=`) é paridade com `ops::compare`
do vanilla (`foundations/ops.rs:471-500`). Os pares incomparáveis erram —
o observável "é erro" é paridade; o **texto** da mensagem (`cannot compare
{a} and {b}`, nomes longos) é especificado no nó `error_formatting.md`.

## Instrução

### Braços específicos (antes do combinado — a ordem do match é semântica)

`Lt`/`Leq`/`Gt`/`Geq` para: `Int`, `Float`, `Int ↔ Float` (coerção para
`f64`, como no vanilla), `Decimal` (homogéneo), `Duration` (homogéneo),
`Version` (lexicográfica zero-pad sobre os componentes).

### Braço combinado → `value_cmp`

`(op @ (Lt|Leq|Gt|Geq), a, b)` delega em `value_cmp(&a, &b)`;
`Some(ord)` → o boolean correspondente; `None` → erro
`"cannot compare {a} and {b}"` (formato verbatim do vanilla,
`foundations/ops.rs:500`).

### `value_cmp` — tabela de comparação

| Combinação | Semântica | Fonte vanilla |
|---|---|---|
| `Bool` | `false < true` | `ops.rs:473` |
| `Int`, `Float`, `Int ↔ Float` | ordem numérica (`partial_cmp`) | `ops.rs` |
| `Decimal` | ordem numérica homogénea | — |
| `Str` | lexicográfica | `ops.rs:483` |
| `Version` | `partial_cmp` zero-pad | — |
| `Duration` | ordem por nanos | — |
| `Angle` | ordem por radianos | `ops.rs:480` |
| `Ratio` | ordem do valor | `ops.rs:477` |
| `Length` | `length_partial_cmp` | `length.rs:195-204` |
| `Relative` | `rel_partial_cmp` | `rel.rs:193-202` |
| `Length ↔ Relative` | só se a parte relativa do `Relative` for zero (guard) | `ops.rs:491-494` |
| `Array` | `cmp_arrays` | `ops.rs:514-530` |
| qualquer outro par | `None` → erro | — |

### `cmp_arrays`

Lexicográfica recursiva com `value_cmp` elemento a elemento; prefixo igual →
o mais curto é menor; elementos incomparáveis propagam `None` (o chamador
erra). Paridade `try_cmp_arrays` (`foundations/ops.rs:514-530`).

### `length_partial_cmp` e `rel_partial_cmp`

- `Length`: comparável só medido numa única componente — ambos `em` zero →
  rácio de `abs`; ambos `abs` zero → `em`; misto → `None`
  (`layout/length.rs:195-204`).
- `Rel<Length>`: `rel` ambos zero → compara `abs`; `abs` ambos zero →
  compara `rel`; misto → `None` (`layout/rel.rs:193-202`; medido:
  `(10pt + 50%) < (20pt + 50%)` → erro nos dois binários).

## Restrições Estruturais

- L1 puro (ver hub).
- Scope-out medido: ordenação `Ratio ↔ Relative` (`ops.rs:492,494`) — sem
  consumidor medido; não reabrir sem medição nova.

## Critérios de Verificação

```
eval_binary_op(Lt, Str("a"), Str("b"))          == Bool(true)
eval_binary_op(Lt, Bool(false), Bool(true))     == Bool(true)
eval_binary_op(Lt, Array[1,2], Array[1,3])      == Bool(true)
eval_binary_op(Lt, Array[1,2], Array[1,2,0])    == Bool(true)  // prefixo igual → mais curto menor
eval_binary_op(Lt, Length(1cm), Length(2cm))    == Bool(true)
eval_binary_op(Lt, Relative(50%), Relative(60%)) == Bool(true)
eval_binary_op(Lt, Length(10pt), Relative(20pt + 0%)) == Bool(true)
eval_binary_op(Lt, Length(10pt), Relative(10pt + 1%)) == Err("cannot compare length and relative length")
eval_binary_op(Lt, Relative(10pt+50%), Relative(20pt+50%)) == Err (incomparável)
eval_binary_op(Lt, Angle(30deg), Angle(45deg))  == Bool(true)
eval_binary_op(Lt, Decimal(1.0), Decimal(2.0))  == Bool(true)
eval_binary_op(Geq, Version(1,2), Version(1,10)) == Bool(false) // zero-pad: 2 < 10
eval_binary_op(Lt, Dir(LTR), Int(2))            == Err("cannot compare direction and integer")
```

## Resultado Esperado

- Os braços de ordenação e os quatro helpers conforme a tabela; testes
  unitários no ficheiro; zero regressão na suite do eval.
