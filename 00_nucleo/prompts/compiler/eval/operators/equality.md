# Prompt L0 — `compiler/eval/operators/equality` — igualdade e pertença
Hash do Código: d9f6ab2d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/operators/equality.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/operators.md`
**ADRs**: ADR-0025 (dois sistemas de igualdade), ADR-0107 (paridade língua — igualdade morfológica de content)

---

## Contexto

O `==` da **linguagem** Typst não é o `PartialEq` do Rust (ADR-0025): coage
`Int ↔ Float` (medido: `1 == 1.0` → `true`), compara `Content`
**morfologicamente** (texto/markup/estilo semântico, ignorando estilo de
render — via `Content::morph_canon`), e propaga a coerção a elementos
aninhados de arrays/dicts. O `derive(PartialEq)` de `Value` fica para o Rust
(IndexMap, testes, estruturas de dados).

## Instrução

### Braços dedicados de `Eq`/`Neq` (antes do braço genérico)

- `Int ↔ Float` cruzado: coerção para `f64` e comparação.
- `Content == Content`: `a.morph_canon() == b.morph_canon()` — a forma
  canónica é comparada com o `==` estrutural; o estilo de render assado é
  ignorado.
- `Version == Version`: comparação directa sobre todos os componentes
  (zero-pad; não existem `pre`/`build` em Typst).
- `Ratio ↔ Relative`: igualdade quando a parte absoluta do `Relative` é zero
  e `|rel − ratio| < 1e-9`.
- Braço genérico: `(Eq, a, b) → values_eq(&a, &b)`; `Neq` é a negação.

### `values_eq` — igualdade da linguagem, recursiva

Paridade com `Value::eq` do vanilla (que **é** `ops::equal`,
`foundations/value.rs:295-299`):

- `Int ↔ Float`: coerção, em qualquer profundidade (medido:
  `(1,2) == (1.0,2.0)` → `true`; `(a: 1) == (a: 1.0)` → `true`).
- `Array`/`Dict`: elemento a elemento com `values_eq` (mesmo comprimento).
- `Length ↔ Relative`: igual quando o `Relative` tem parte relativa zero
  (`ops.rs:458-460`; medido: `10pt == (10pt + 0%)` → `true`).
- `Ratio ↔ Relative`: parte absoluta zero + tolerância `1e-9`.
- `Content`: morfológico (`morph_canon`), também em posição aninhada.
- Resto: delega no `PartialEq` derivado.

### `value_eq` e a pertença `in` / `not in`

`value_eq` delega em `values_eq` e serve os braços de pertença (medido:
`1 in (1.0, 2.0)` → `true`; `(1,) in ((1.0,), (2,))` → `true`).

| `lhs in rhs` | Resultado | Medição vanilla |
|---|---|---|
| `Str in Dict` | `Bool` — a chave existe (`contains_key`) | `"a" in (a:1,b:2)` → `true` |
| `Str in Str` | `Bool` — substring | `"ell" in "hello"` → `true` |
| `any in Array` | `Bool` — `values_eq` elemento a elemento | `(1,2) in ((1,2),(3,4))` → `true` |
| `not in` | negação lógica das combinações acima | `1 not in (1,2,3)` → `false` |
| combinação sem braço (ex.: `Int in Str`) | **erro** de fronteira | vanilla: `"cannot apply 'in' to integer and string"` |

**Divergência registada**: o texto da mensagem de erro de `in` com tipos
incompatíveis diverge do vanilla (o cristalino emite o formato genérico de
fronteira); o observável "é um erro de tipo" é preservado (classe aceite,
ADR-0107).

## Restrições Estruturais

- L1 puro (ver hub); nenhum braço toca `EvalContext`/`Scope`.
- Não tocar o `derive(PartialEq)` de `Value` — os dois sistemas coexistem
  por decisão (ADR-0025).

## Critérios de Verificação

```
eval_binary_op(Eq, Int(1), Float(1.0))          == Bool(true)
eval_binary_op(Neq, Int(1), Float(1.0))         == Bool(false)
eval_binary_op(Eq, Content(it.body), Content([a]))  // casa morfologicamente
eval_binary_op(Eq, Ratio(50%), Relative(0pt + 50%)) == Bool(true)
eval_binary_op(Eq, Length(10pt), Relative(10pt + 0%)) == Bool(true)

// coerção recursiva
eval_binary_op(Eq, Array[1,2], Array[1.0,2.0])  == Bool(true)
eval_binary_op(Eq, Dict{a:1}, Dict{a:1.0})      == Bool(true)

// pertença
eval_binary_op(In, Str("a"), Dict{a:1,b:2})     == Bool(true)
eval_binary_op(In, Str("z"), Dict{a:1,b:2})     == Bool(false)
eval_binary_op(In, Str("ell"), Str("hello"))    == Bool(true)
eval_binary_op(In, Array[1,2], Array[Array[1,2],Array[3,4]]) == Bool(true)
eval_binary_op(In, Int(1), Array[Float(1.0)])   == Bool(true)
eval_binary_op(NotIn, Int(5), Array[1,2,3])     == Bool(true)
eval_binary_op(In, Int(1), Str("hello"))        == Err (tipos incompatíveis)
```

## Resultado Esperado

- Braços `Eq`/`Neq`/`In`/`NotIn` e os helpers `values_eq`/`value_eq`
  conforme especificado; testes unitários no ficheiro e E2E no eval.
