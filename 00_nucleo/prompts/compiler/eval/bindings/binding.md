# Prompt L0 — `compiler/eval/bindings/binding` — `#let`, desestruturação e atribuição
Hash do Código: f5915c2b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/bindings/binding.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/bindings.md`
**ADRs**: ADR-0107 (paridade língua), ADR-0044 (`Engine<'_>`)
**Técnica**: *pattern matching* estrutural — a desestruturação percorre o padrão e o valor em paralelo, com um sumidouro (`sink`) para o spread; o mesmo esqueleto serve `#let` e `=`.

---

## Contexto

Este nó liga **nomes a valores**: `#let`, a desestruturação de padrões
(array/dict) que `#let` e a atribuição por desestruturação partilham, e a
atribuição simples `=`.

Corresponde a `typst-eval/src/binding.rs` do vanilla (209 linhas), quase
função a função: `destructure` ↔ `destructure_let`, `destructure_impl` ↔
`destructure_pattern`, `destructure_array`, `destructure_dict`,
`wrong_number_of_elements`. A divergência é mecânica (o vanilla usa
`impl Eval for ast::LetBinding`; aqui são free functions — ADR-0107).

## Restrições Estruturais

- L1 puro; `Engine<'_>`/`EvalContext`/`Scopes` são parâmetros.
- Não despacha métodos nem resolve campos — delega o lugar mutável a
  `super::access::{access, access_dict}`.

## Instrução

### `eval_let(binding, scopes, ctx, engine)`

| `LetBindingKind` | Comportamento |
|---|---|
| `Normal(pattern)` | avalia o init (ou `None` se ausente) e desestrutura conforme o padrão |
| `Closure(ident)` | avalia o init e liga ao nome da closure |

### Padrões suportados (`destructure_pattern`)

| `Pattern` | Comportamento |
|---|---|
| `Normal(expr)` | liga o valor ao lugar resolvido pelo expr |
| `Placeholder` | descarta o valor (`_`) |
| `Parenthesized(p)` | recorre sobre `p` |
| `Destructuring(d)` | despacha para `destructure_array` ou `destructure_dict` conforme o valor |

### Itens de desestruturação (`DestructuringItem`)

`Pattern` (posicional), `Named` (por chave), `Spread` (sumidouro — recolhe o
resto; no máximo um por padrão).

### Erros

`wrong_number_of_elements` — mensagem verbatim do vanilla quando o número de
elementos do valor não bate com o do padrão, distinguindo o caso com e sem
spread.

### `eval_assign` e `eval_destruct_assignment`

`eval_assign` cobre `=` e os compostos (`+=`, `-=`, …, via
`crate::compiler::eval::operators::eval_binary_op`);
`eval_destruct_assignment` cobre `(a, b) = expr` (P715), reusando
`destructure_pattern` sobre lugares já existentes em vez de criar ligações.

## Critérios de Verificação

```
#let (a, b) = (1, 2)                → a == 1, b == 2
#let (a, ..rest) = (1, 2, 3)        → a == 1, rest == (2, 3)
#let (a: x, b: y) = (a: 1, b: 2)    → x == 1, y == 2
#let (a, b) = (1,)                  → Err (wrong number of elements, verbatim)
#let (_, b) = (1, 2)                → b == 2, sem ligação para `_`
#let f(n) = n + 1                   → Closure ligada a `f`
(a, b) = (b, a)                     → troca (P715)
x += 1                              → equivalente a x = x + 1
```

## Resultado Esperado

- `eval_let`, `destructure_let`, `eval_destruct_assignment`, `eval_assign`
  visíveis em `eval`; `destructure_pattern`/`_array`/`_dict` e
  `wrong_number_of_elements` privados ao nó.
