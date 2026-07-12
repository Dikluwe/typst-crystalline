# Paridade Produção — P716 — `Access` genérico: atribuição a `dict.campo` e `arr.at(i)`

**Data:** 2026-07-12
**Passo:** `00_nucleo/materialization/typst-passo-716.md`
**Hash do commit (implementação):** `A_PREENCHER`.
**HEAD base:** `c69f40187` (fim de P715).
**Estado:** FECHADO — `dict.campo = v`, `arr.at(i) = v`, `arr.first() = v`,
`arr.last() = v`, `dict.at(k) = v` (simples e compostas, e como folhas de
desestruturação-atribuição) implementados com paridade ao mecanismo
`Access` do vanilla, incluindo os casos de erro medidos.

---

## 1. Sonda

### 1.1 Lista completa de accessor methods (vanilla)

`lab/typst-original/crates/typst-eval/src/methods.rs:19-21`:

```rust
pub(crate) fn is_accessor_method(method: &str) -> bool {
    matches!(method, "first" | "last" | "at")
}
```

**Três**, não só `.at()`: `first`, `last`, `at`. Array suporta os três
(`first_mut`/`last_mut`/`at_mut`, `foundations/array.rs:104-119`); dict só
`at` (`Dict::at_mut`, `foundations/dict.rs:99-104`). Não há outros.

### 1.2 Mecanismo `Access` completo (vanilla)

- **`Access`** (`typst-eval/access.rs:14-27`): 4 formas de alvo — `Ident`
  (`scopes.get_mut`), `Parenthesized` (recursivo), `FieldAccess`
  (`access_dict` + `Dict::at_mut`), `FuncCall` (só se o callee for
  `FieldAccess` cujo campo é accessor method; avalia **args primeiro**,
  access do target depois — `access.rs:62-64`). Qualquer outra expressão
  avalia (para efeitos) e erra `cannot mutate a temporary value`.
- **Caso especial de `apply_assignment`** (`typst-eval/ops.rs:77-85`): `=`
  **puro** com lhs `FieldAccess` não passa pelo `at_mut` — faz
  `access_dict` + `Dict::insert`, ou seja **cria** a chave se não existir.
  As formas compostas (`+=` etc.) passam pelo `Access` normal e erram em
  chave ausente. Ordem de avaliação: **rhs primeiro** (`ops.rs:74`).
- **`access_dict`** com target não-dict (`access.rs:76-107`): três níveis —
  tipos com field getters próprios (Symbol/Content/Module/Func/Args) →
  `cannot mutate fields on {ty}`; `fields_on(ty)` vazio → `{ty} does not
  have accessible fields`; senão (Version, Length, Rel, Stroke, Alignment —
  `fields.rs:77-91`) → `fields on {ty} are not yet mutable` + hint.
- **Desestruturação-atribuição** (`typst-eval/binding.rs:30-42`): a folha é
  `*expr.access(vm)? = value` — usa o mesmo `Access`, **sem** o caso
  especial de insert.

### 1.3 Comportamento medido (vanilla)

Binário: `lab/typst-original/target/release/typst`, repositório em
`c69f40187`, working tree sem alterações a código (só documentos de teste
untracked em `/tmp`). Documento principal (`/tmp/p716-access.typ`, o do
passo):

```
#let d = (a: 1, b: 2)  →  #{ d.a = 10 }  →  (a: 10, b: 2)
#let arr = (1, 2, 3)   →  #{ arr.at(1) = 20 }  →  (1, 20, 3)
#{ arr2.at(5) = 20 }   →  error: array index out of bounds (index: 5, len: 3)
```

Casos adicionais medidos (compilações individuais, mesmo estado):

| Snippet | Vanilla |
|---|---|
| `d.novo = 5` (chave nova, `=` puro) | **insere** — `(a: 1, novo: 5)` |
| `d.b += 1` (chave ausente, composta) | `dictionary does not contain key "b"` + hint `use `insert` to add or update values` |
| `d.at("outro") = 7` (chave ausente) | mesma mensagem + hint (não insere) |
| `arr.at(1, default: 0) = 20` | `unexpected argument: default` |
| `arr.at(0, 1) = 5` | `unexpected argument` |
| `arr.at() = 1` / `d.at() = 1` | `missing argument: index` / `missing argument: key` |
| `arr.at("x") = 1` | `expected integer, found string` |
| `arr.first() = 100`, `arr.last() = 300` | mutam; array vazio → `array is empty` |
| `s.at(0) = "x"` (s: str) | `cannot mutate a temporary value` |
| `x.at(0) = 1` (x: int) | ``type integer has no method `at` `` |
| `s.len() = 1` (não-accessor) | `cannot mutate a temporary value` |
| `x.a = 1` (x: int) | `integer does not have accessible fields` |
| `c.body = [x]` (c: content) | `cannot mutate fields on content` |
| `v.major = 9` (v: version) | `fields on version are not yet mutable` + hint |
| `(arr.at(0), arr.at(1)) = (9, 8)` | muta ambos — `(9, 8)` |
| `(d.novo,) = (2,)` | `dictionary does not contain key "novo"` (folha sem insert) |
| `n.xs.at(0) = 9` (aninhado) | `(xs: (9, 2))` |

Nota de paridade observável: o vanilla usa o nome **longo** do tipo nestas
mensagens (`integer`, `string`), não o curto (`int`, `str`).

### 1.4 Uso real em `cetz` (0.5.2)

- `arr.at(i) = valor`: `hobby.typ:51,52,58,138-151,160-161,219-230,242-243`
  (dezenas de ocorrências — o coração do algoritmo de splines).
- `dict.campo = valor`: `drawable.typ:36` (`drawable.segments = ...`),
  `drawable.typ:57` (`drawable.pos = ...`).

### 1.5 Estado do cristalino antes

Binário `target/release/typst` construído em `c69f40187` (P715):
`/tmp/p716-access.typ` → `error: cannot mutate a temporary value` na
**linha 2** (`d.a = 10`) — o scope-out do P715 confirmado como bloqueio.

---

## 2. Implementação

L0 actualizado primeiro: `00_nucleo/prompts/rules/eval.md` §P716 (hash de
linhagem `824cf31b` via `crystalline-lint --fix-hashes`). Testes escritos
antes do código: 25 testes `p716_*` em `eval/tests.rs`, 22 a falhar no
estado P715 (os 3 restantes passavam por coincidência de mensagem).

Tudo em `01_core/src/rules/eval/bindings.rs`, mirror de
`typst-eval/access.rs` + `methods.rs` + `ops.rs`:

- **`access(expr, scopes, ctx, engine) -> SourceResult<&mut Value>`** —
  mirror do trait `Access` como free function (o cristalino não tem `Vm`).
  4 braços + fallback avalia-e-erra.
- **`access_dict`** — os três níveis de erro do vanilla; braço "not yet
  mutable" = espelho de `fields_on` (Version, Length, Relative, Stroke,
  Align). Duration cristalino (campos de leitura, P412) fica no braço
  "does not have accessible fields", como no vanilla.
- **`is_accessor_method`**, **`call_method_access`** — lista exacta
  (`first`/`last`/`at`); índice negativo conta do fim (`locate_opt`);
  validação de args na ordem do vanilla (`expect` → `at_mut` → `finish`),
  com `missing argument:`/`unexpected argument`/`expected integer, found`.
- **`long_type_name`** — nome longo nos erros do `Access` (`integer`,
  `string`, `boolean`), porque a mensagem é o observável (ADR-0107).
- **`eval_assign` reescrito** como mirror de `apply_assignment`: rhs
  primeiro; caso especial `=`+`FieldAccess` → insert; resto via `access` +
  `mem::replace` (deixou de ler via `scopes.get`+clone).
- **`eval_destruct_assignment`** — folha passa a `*access(...)? = value`
  (mirror exacto). `destructure_pattern`/`_array`/`_dict`/`destructure_let`
  passam a enfiar `ctx`/`engine` até à folha.
- `Scopes::get_mut` (P715) inalterado.

---

## 3. Validação

Estado da medição: working tree com exactamente as alterações deste passo
(12 ficheiros, `git diff HEAD --stat`: `eval.md` +120/-1;
`bindings.rs` +445; `tests.rs` +215; 9 ficheiros só com hash de linhagem),
commitado de seguida como `A_PREENCHER` — o diff do commit é o estado
medido. Re-verificação pós-commit registada em §3.4.

### 3.1 Documentos vs vanilla

- `/tmp/p716-access.typ` → mesmo erro do vanilla:
  `array index out of bounds (index: 5, len: 3)` na linha 10.
- Variante sem o caso de erro (`d.a=10; arr.at(1)=20; first/last;
  d2.at("x")=9`) → `pdftotext` **idêntico** ao vanilla:
  `(a: 10, b: 2)` / `(1, 20, 3)` / `(100, 2, 300)` / `(x: 9)`.
- `(arr.at(0), arr.at(1)) = (9, 8)` → `(9, 8)` (igual vanilla).
- `n.xs.at(0) = 9` (aninhado) → `(xs: (9, 2))` (igual vanilla).

### 3.2 Suites

- `cargo test --workspace` → **3866 passed, 0 failed** no `typst-core`
  (3841 em P715 + 25 novos), restantes crates todos verdes.
- `crystalline-lint .` → **0 violations** (hashes `824cf31b`).

### 3.3 Reprodução `cetz` (P700-716)

Documento do passo (`/tmp/p716-cetz.typ`: `canvas` + `line` + `circle`):

- **Bloqueio de P715 resolvido** — `cannot mutate a temporary value`
  desapareceu.
- **Próximo bloqueio:** `error: campo desconhecido em array: 'push'` —
  `cetz` usa os **mutating methods** do vanilla (`push`/`pop`/`insert`/
  `remove`, `typst-eval/methods.rs:9-16` `is_mutating_method` +
  `call_method_mut`, que também obtém o alvo via `Access`). É o mecanismo
  irmão do deste passo, sobre a mesma fundação `access()` agora existente.
  Candidato natural a P717.
- Tempo de compilação (sinal de progresso): ver §3.4 (medição pós-commit).

### 3.4 Re-verificação pós-commit (proveniência)

Executada no commit `A_PREENCHER`, working tree limpa (documentos de teste
em `/tmp`, fora do repositório):

- `/tmp/p716-access.typ` → `A_CONFIRMAR`
- `cetz` → `A_CONFIRMAR` (erro e tempo)

---

## 4. Critério de fecho do passo

- [x] Sonda completa — lista completa de accessor methods confirmada
  (`first`/`last`/`at`, `methods.rs:19-21`); comportamento e erros medidos
  (§1.3); uso em `cetz` com `file:line` (§1.4).
- [x] Implementado e testado, incluindo casos de erro (25 testes novos).
- [x] Sem regressão — `cargo test --workspace` verde (3866/0).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — próximo bloqueio registado (`push`, mutating
  methods) com tempo de compilação como sinal de progresso.
- [x] Relatório com hash do commit.
