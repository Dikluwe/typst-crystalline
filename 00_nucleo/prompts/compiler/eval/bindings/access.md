# Prompt L0 — `compiler/eval/bindings/access` — lugares mutáveis e erros de nome
Hash do Código: eb55d1ba

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/bindings/access.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/bindings.md`
**ADRs**: ADR-0107 (paridade língua), ADR-0044 (`Engine<'_>`)
**Técnica**: resolução de *l-value* — a mesma expressão que como r-value produz um valor, como l-value produz uma referência mutável ao lugar onde o valor vive.

---

## Contexto

Este nó resolve o **lugar mutável** por trás de uma expressão de acesso: dado
`a.b.c` ou `arr.at(i)` no lado esquerdo de uma atribuição ou de um método
mutante, devolve `&mut Value` para o sítio real, em vez de uma cópia.

Corresponde a `typst-eval/src/access.rs` (107 linhas). Divergência mecânica
deliberada: o vanilla usa um `trait Access` com uma impl por variante de AST;
aqui é uma free function com `match` exaustivo (ADR-0107 — a paridade é com a
semântica de l-value, não com a forma do despacho em Rust).

## Restrições Estruturais

- L1 puro; devolve `&'s mut Value` com o lifetime do `Scopes`.
- Não decide o que fazer com o lugar — quem escreve é `binding` (atribuição)
  ou `method_dispatch` (método mutante).

## Instrução

### `access(expr, scopes, ctx, engine) -> SourceResult<&mut Value>`

| Forma da expressão | Resolução |
|---|---|
| `Ident` | procura no stack de scopes; erro `unknown_variable` se ausente |
| `Parenthesized` | recorre sobre o interior |
| `FieldAccess` | resolve o alvo como dict via `access_dict`, depois a chave |
| `FuncCall` | só para métodos de acesso (`at`, `first`, `last`) — delega a `super::method_dispatch::call_method_access`, autorizado por `super::method_dispatch::is_accessor_method` |
| outra | erro — não é um lugar atribuível |

### `access_dict` — o alvo tem de ser dicionário

Resolve o alvo de um `FieldAccess` e exige `Value::Dict`; se for outro tipo,
a mensagem nomeia o tipo com `long_type_name`.

### Mensagens

- `unknown_variable(span, name)` — variável desconhecida. Distingue o caso da
  variável **capturada** por closure (P772q+r), cuja mensagem é diferente.
- `missing_key(span, key)` — chave ausente no dicionário.

### `long_type_name(value) -> &'static str`

Nome **longo** do tipo na língua, para mensagens de erro: `int → integer`,
`str → string`, `bool → boolean`; o resto delega em `type_name()`. É
`pub(crate)` por ser usado fora de `bindings` (`eval/mod.rs`,
`call_dispatch.rs`, `stdlib/*`).

## Critérios de Verificação

```
#let d = (a: 1); d.a = 2            → d == (a: 2)
#let a = (1, 2); a.at(0) = 9        → a == (9, 2)
#let d = (:); d.x = 1               → d == (x: 1)   // cria a chave
#let d = (a: 1); d.b                → Err (missing key)
x = 1  (x não ligado)               → Err (unknown variable)
#let n = 1; n.at(0) = 2             → Err (nomeia "integer", não "int")
```

## Resultado Esperado

- `access`/`access_dict`/`missing_key` visíveis dentro de `bindings`;
  `unknown_variable` visível em `eval`; `long_type_name` `pub(crate)`.
