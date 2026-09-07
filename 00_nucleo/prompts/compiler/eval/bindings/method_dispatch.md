# Prompt L0 — `compiler/eval/bindings/method_dispatch` — métodos mutantes e de acesso
Hash do Código: 2edbbc98

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/bindings/method_dispatch.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/bindings.md`
**ADRs**: ADR-0107 (paridade língua)
**Técnica**: classificação sintáctica antes do despacho — o nome do método decide, *antes* de avaliar, se a chamada precisa do lugar mutável ou só do valor; sem isto, `a.push(1)` avaliaria `a` como cópia.

---

## Contexto

Este nó despacha os métodos que operam **sobre o lugar** e não sobre uma cópia:
os mutantes (`push`, `pop`, `insert`, `remove`) e os de acesso (`at`, `first`,
`last`), mais as tabelas que classificam cada nome de método.

Corresponde a `typst-eval/src/methods.rs` (104 linhas), função a função:
`is_mutating_method`, `is_dict_mutating_method`, `is_accessor_method`,
`call_method_mut`, `call_method_access` têm os mesmos nomes dos dois lados.

## Restrições Estruturais

- L1 puro. `call_method_access` e `call_method_mut` **não recebem
  `Engine`/`EvalContext`** — operam sobre `&mut Value` e `Args` já avaliados.
  Só `try_eval_mutating_method` recebe contexto, e apenas para avaliar os
  argumentos e resolver o lugar via `super::access::access`.

## Instrução

### Tabelas de classificação

| Função | Verdadeiro para |
|---|---|
| `is_mutating_method` | `push`, `pop`, `insert`, `remove` |
| `is_dict_mutating_method` | `insert`, `remove` |
| `is_accessor_method` | `first`, `last`, `at` |
| `has_readonly_method` | `(Str\|Bytes, first\|last\|at)`, `(Content, func\|has\|at\|fields\|location)`, `(Version\|Args, at)` |

`has_readonly_method` existe para a **mensagem** de erro: distingue "este tipo
não tem esse método" de "este método existe mas o valor é temporário" —
`"cannot mutate a temporary value"`.

### `try_eval_mutating_method(fa, args_node, span, scopes, ctx, engine)`

1. Avalia os argumentos (`call_dispatch::eval_args`).
2. Resolve o lugar (`access`).
3. Despacha para `call_method_mut`.

Devolve `Ok(None)` quando o método não é mutante — o chamador segue o caminho
normal.

### `call_method_mut(&mut Value, method, args, span)`

`Array`: `push`, `pop`, `insert(index, value)`, `remove(index)`.
`Dict`: `insert(key, value)`, `remove(key)`.
Qualquer outro par (tipo, método): erro, com a distinção de `has_readonly_method`.

### `call_method_access(&mut Value, method, args, span) -> &mut Value`

Só `Array` e `Dict`; devolve referência mutável ao elemento (`at`, `first`,
`last`). Para outros tipos, erro imediato.

### Helpers de argumentos

`expect_positional(args, span, what)` — extrai um posicional obrigatório;
`finish_args(args, span)` — erro se sobrarem argumentos por consumir. Ambos
usados também por `field_access`.

## Critérios de Verificação

```
#let a = (1,); a.push(2)            → a == (1, 2)
#let a = (1, 2); a.pop()            → devolve 2, a == (1,)
#let a = (1, 3); a.insert(1, 2)     → a == (1, 2, 3)
#let d = (a: 1); d.insert("b", 2)   → d == (a: 1, b: 2)
#let d = (a: 1); d.remove("a")      → devolve 1, d == (:)
"abc".push("d")                     → Err "cannot mutate a temporary value"
#let n = 1; n.push(2)               → Err (método inexistente para integer)
#let a = (1,); a.push()             → Err (argumento posicional em falta)
#let a = (1,); a.push(1, 2)         → Err (argumentos a mais)
```

## Resultado Esperado

- `is_mutating_method` e `try_eval_mutating_method` visíveis em `eval`;
  `call_method_access`, `is_accessor_method`, `expect_positional`,
  `finish_args` visíveis dentro de `bindings`; o resto privado ao nó.

## P1307-R3 — consumo coerente de Args

### Medição anterior à decisão

`01_core/src/compiler/eval/bindings/method_dispatch.rs:46` remove o primeiro
positional; `:336,388` remove named default. Os helpers continuam a consultar
items/named depois dessas operações. Snapshot adicional R3 SHA-256
`592497d1e786241b9ba121479c0c55dbad370a70732e130b4f7ac3bd709d4405`
registra HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree P1306
e diff/stat completos antes deste amendment. A inserção futura de um carrier
Some torna a mutação exclusiva da view incoerente; isso decorre do fluxo de
dados local, não de medição de novo comportamento vanilla.

### Migração, pendente de aprovação de Args

`expect_positional` deve consumir por `Args::remove_positional(0)` depois
do mesmo guard de missing; default usa `Args::remove_named("default")`,
que retira suas ocorrências e devolve o último valor como a view legada.
Não alterar assinatura dos helpers, referências mutáveis ao valor, ordem
de acesso/finish, mensagens, âncoras agregadas ou prioridade de erros.
Some mantém apenas ocorrências não consumidas; None conserva sua política
legada. Não deixar metadata stale, converter Some em None ou validar casts
de cada named como se estes métodos fossem os novos encoders.

O helper compartilhado com field_access permanece deste owner, não se copia.
Aceitação cobre mutantes/acessores array/dict e métodos de Content via helper,
missing/extra/default com Some/None e controle do lugar mutável. É inferência
que substituir apenas o consumo basta; outra escrita em Args ou alteração
de observáveis exige rever a obrigação antes de código. Gate ADR-0127 do
carrier permanece obrigatório, sem implementação nesta rodada documental.
