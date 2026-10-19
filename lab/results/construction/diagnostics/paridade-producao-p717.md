# Paridade Produção — P717 — Métodos mutantes (`push`, `pop`, `insert`, `remove`)

**Data:** 2026-07-12
**Passo:** `00_nucleo/materialization/typst-passo-717.md`
**Hash do commit (implementação):** `c570d53e5`.
**HEAD base:** `89712433b` (fim de P716).
**Estado:** FECHADO — os quatro métodos mutantes implementados com paridade
ao mecanismo do vanilla (`is_mutating_method`/`call_method_mut` +
`maybe_resolve_mutating`), sobre a fundação `access()` de P716, incluindo
os casos de erro medidos. Duas divergências antigas (P466/P501, dict
não-mutante) fechadas.

---

## 1. Sonda

### 1.1 Lista completa de mutating methods (vanilla)

`lab/typst-original/crates/typst-eval/src/methods.rs:9-16`:

- `is_mutating_method` = `push` | `pop` | `insert` | `remove` — **quatro**,
  não há outros.
- `is_dict_mutating_method` = `insert` | `remove` — dict não tem
  `push`/`pop`.
- `call_method_mut` (`methods.rs:24-63`): Array suporta os quatro; Dict só
  `insert`/`remove`. `pop`/`remove` **devolvem** o elemento removido;
  `push`/`insert` devolvem none. `args.finish()` corre **depois** da
  mutação.

### 1.2 Uso real em `cetz` (0.5.2) — sem questão de scope-out

Grep a `src/*.typ` + `src/**/*.typ`: `.push(` **63×**, `.insert(` **26×**,
`.pop(` **1×**, `.remove(` **1×** (maiores consumidores: `draw/shapes.typ`
26, `bezier.typ` 11, `draw/grouping.typ` 10, `styles.typ` 7). Os quatro
têm consumidor — implementados todos; nenhuma decisão de scope-out por
falta de uso foi necessária.

### 1.3 Despacho no vanilla (`call.rs`)

`FuncCall::eval` (`call.rs:33-43`) + `maybe_resolve_mutating`
(`call.rs:189-212`): método mutante → **args avaliados primeiro**
(`call.rs:196-198`, o `access()` toma o empréstimo mutável do `Vm`),
depois `access()` do target (o mesmo trait de P716 — temporários erram
`cannot mutate a temporary value` antes de qualquer resolução). Com o
local: Dict + `push`/`pop` → cai para a resolução normal, que termina em
``type dictionary has no method `push` `` (dicts deliberadamente não
resolvem campos como métodos, `call.rs:233-238`); Array/Dict →
`call_method_mut`; outros tipos → cai para a resolução normal com clone
(módulo com função `insert` continua a funcionar).

### 1.4 Comportamento medido (vanilla)

Binário `lab/typst-original/target/release/typst`, repositório em
`89712433b`. Documento do passo (`/tmp/p717-mut.typ`):

```
push(4)        → (1, 2, 3, 4)
pop()          → bloco devolve 3; resta (1, 2)
insert(1, 99)  → (1, 99, 2, 3)
remove(1)      → bloco devolve 2; resta (1, 3)
```

Casos adicionais (fontes: `array.rs:225-274`, `dict.rs:241-251`):

| Snippet | Vanilla |
|---|---|
| `#let x = a.pop()` | `x=3`, `a=(1, 2)` — retorno como expressão |
| `a.insert(-1, 9)` em `(1,2,3)` | `(1, 2, 9, 3)` — negativo conta do fim |
| `a.insert(2, 9)` em `(1,2)` | `(1, 2, 9)` — `locate` com `end_ok=true` |
| `a.insert(4, 9)` (len 3) | `array index out of bounds (index: 4, len: 3)` — **sem** sufixo |
| `a.remove(5)` (len 3) | mesma mensagem **com** sufixo `and no default value was specified` |
| `a.remove(5, default: 9)` | devolve 9, array **intacto** |
| `a.remove(0, bad: 1)` | `unexpected argument: bad` |
| `a.pop(1)` | `unexpected argument` |
| `a.push()` / `a.insert(1)` | `missing argument: value` |
| `a.insert("x", 9)` | `expected integer, found string` |
| `d.insert("b", 2)` | cria a chave, devolve none |
| `d.insert(5, 2)` | `expected string, found integer` |
| `d.remove("x")` | `dictionary does not contain key "x"` — **sem hint** (≠ `at_mut` P716) |
| `d.remove("x", default: 7)` | devolve 7 |
| `d.push(2)` | ``type dictionary has no method `push` `` |
| `s.push("c")` (s: str) | ``type string has no method `push` `` |
| `(1, 2).push(3)` | `cannot mutate a temporary value` |

### 1.5 `access()` de P716 reaproveitável? — Sim, sem mudanças

`bindings.rs` `fn access` resolve o alvo directamente; os métodos mutantes
precisam apenas do despacho (`try_eval_mutating_method`) e do
`call_method_mut`. Nenhuma alteração a `access()`/`Scopes::get_mut`.
Tamanho real do passo: **S**, como a hipótese do passo previa.

### 1.6 Estado do cristalino antes

- `cetz` bloqueava em `campo desconhecido em array: 'push'` (P716 §3.3).
- `try_dispatch_collection_method` (`stdlib/collections.rs:66,68`) já
  tinha `dict.insert`/`dict.remove` **não-mutantes** (era P466/P501 —
  recebem o dict por valor/clone; a variável original não mudava), com a
  divergência documentada no próprio teste `p466_dict_remove`.

---

## 2. Implementação

L0 actualizado primeiro: `00_nucleo/prompts/engine/eval.md` §P717 (hashes
de linhagem via `crystalline-lint --fix-hashes`). Testes escritos antes do
código: 25 testes `p717_*`, 23 a falhar no estado P716.

- **`bindings.rs`**: `is_mutating_method`/`is_dict_mutating_method`
  (listas exactas), `try_eval_mutating_method` (mirror de
  `maybe_resolve_mutating`: args primeiro, `access()` do target, despacho)
  e `call_method_mut` (mirror; reutiliza `expect_positional`/
  `finish_args`/`long_type_name` de P716). Dict + `push`/`pop` e tipos
  escalares/str → erro verbatim ``type {ty} has no method `{method}` ``
  (mesmo observável do fall-through vanilla); `Module`/`Func`/`Type`/
  `Symbol`/`Content` → fall-through para a cadeia normal (campos destes
  tipos podem resolver para função). **Divergência medida e aceite**
  (registada no L0): o fall-through re-avalia target e args (o vanilla
  passa os já avaliados) — dupla avaliação de efeitos só nesse caminho,
  sem consumidor em `cetz`.
- **`closures.rs` `eval_func_call`**: intercepção **antes** do bloco P466
  (que avalia o target como valor — clone; mutação exige o local).
- **Divergências antigas fechadas**: `p466_dict_remove` e
  `p501_dict_insert_len` asseriam o comportamento não-mutante antigo —
  actualizados para a paridade vanilla (o original **é** mutado;
  `insert` devolve none). Os despachos não-mutantes de
  `collections.rs:66,68` ficam agora sombreados no caminho de chamada
  (a intercepção corre antes); continuam alcançáveis só via field access
  sem chamada (`eval_field_access`, P509) — comportamento desse caminho
  inalterado.

---

## 3. Validação

Estado da medição: working tree com exactamente as alterações deste passo,
commitado de seguida como `c570d53e5` (o diff do commit é o estado
medido). Re-verificação pós-commit em §3.4.

### 3.1 Documento do passo vs vanilla

`/tmp/p717-mut.typ` → valores **idênticos** ao vanilla:
`(1, 2, 3, 4)` / `3 (1, 2)` / `(1, 99, 2, 3)` / `2 (1, 3)`.
(O `diff` do `pdftotext` só difere em linhas em branco de espaçamento de
parágrafo — divergência de render pré-existente, alheia ao eval.)

### 3.2 Suites

- `cargo test --workspace` → **3891 passed, 0 failed** no `typst-core`
  (3866 em P716 + 25 novos), restantes crates verdes.
- `crystalline-lint .` → **0 violations**.

### 3.3 Reprodução `cetz` — campos fixos de progresso

Documento do passo (`/tmp/p717-cetz.typ`):

- **Exit code:** 1. **Tempo:** ver §3.4 (medição pós-commit).
- **Bloqueio de P716 resolvido** — `campo desconhecido em array: 'push'`
  desapareceu.
- **Próximo bloqueio, com `file:line`:** `error: not enough elements to
  destructure` (hint: `length of 1, but the pattern expects 2`). Causa
  isolada por redução: **spread em literal de array é ignorado pelo
  cristalino** — `#let x = (1, ..(2, 3))` dá `(1)` (len 1) no cristalino
  vs `(1, 2, 3)` (len 3) no vanilla. Consumidor em `cetz`:
  `coordinate.typ:440` — `return (ctx, ..result)` em
  `coordinate.resolve()`, cujo retorno alimenta desestruturações
  `let (ctx, p) = resolve(...)` (ex.: `draw/shapes.typ:76`). O array de
  retorno fica com 1 elemento (`ctx`) e a desestruturação de 2 falha.
  Nota: `eval_args` (`closures.rs:60`) também ignora `Arg::Spread`
  ("fronteira deliberada") — o mesmo tema em chamadas. Candidato a P718:
  spread em literais de array/dict (e possivelmente em args de chamada).

### 3.4 Re-verificação pós-commit (proveniência)

Executada no commit `c570d53e5`, working tree limpa:

- `/tmp/p717-mut.typ` → mesmos valores: `(1, 2, 3, 4)` / `3 (1, 2)` /
  `(1, 99, 2, 3)` / `2 (1, 3)`.
- `cetz` → mesmo bloqueio (`not enough elements to destructure`, length
  1 vs 2), tempo real 52,2s (52,4s pré-commit; P716 reportou 53,0s —
  sem alteração significativa, o documento continua a falhar na fase de
  eval, antes de qualquer layout).

---

## 4. Gate das ADRs (critério do passo)

Grep às ADRs em vigor por `Access`/`get_mut`/mutação: as únicas
ocorrências são incidentais — `AccessDenied` (enum de erro de I/O,
ADR-0005) e a discussão de mutação CoW de `EcoVec` no `Sink` (ADR-0035,
ortogonal: trata de estado interno de L1, não de mutação de bindings da
linguagem; consistente com este passo — mutação in-place de `Value`
dentro do eval não é estado global). ADR-0107/0108/0109 não mencionam o
mecanismo. **Nada contradiz o decidido.**

---

## 5. Critério de fecho do passo

- [x] Sonda completa — lista confirmada (`push`/`pop`/`insert`/`remove`,
  `methods.rs:9-16`); uso em `cetz` medido (§1.2, os quatro com
  consumidor — custo de scope-out não aplicável); comportamento medido
  (§1.4); `access()` de P716 confirmado reaproveitável sem mudanças
  (§1.5).
- [x] Implementado e testado (25 testes novos), mensagens verbatim do
  vanilla; mecanismo P716 sem regressão (teste dedicado).
- [x] Sem regressão — `cargo test --workspace` verde (3891/0; dois testes
  antigos que documentavam a divergência não-mutante actualizados para a
  paridade).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — campos fixos registados (tempo, exit code,
  próximo bloqueio com `file:line`: `coordinate.typ:440`, spread em
  literal de array).
- [x] Grep às ADRs pelos termos centrais — nada contradiz (§4).
- [x] Relatório com hash do commit.
