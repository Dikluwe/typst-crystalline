# Paridade Produção — P715 — Desestruturação, atribuição por desestruturação e atribuição simples/composta

**Data:** 2026-07-11
**Passo:** sem materialização prévia — instrução directa do utilizador
("avança para a desestruturação e corrija o bug"), sonda feita
directamente contra `lab/typst-original` per protocolo (ADR-0108).
**Hash do commit (implementação):** `7a901edd4`.
**HEAD base:** `1879b4610` (fim de P714).
**Estado:** FECHADO — `let (a, b) = ...`, `(a, b) = ...` (atribuição) e
`x = v`/`x += v`/etc. implementados com paridade ao mecanismo do
vanilla. ADR-0114 em vigor — mecanismo central (ligação de variáveis).

---

## 1. Sonda

### 1.1 Alcance mais amplo do que o relatado

O bloqueio reportado (`"destructuring assignment is not yet
implemented"`, `Expr::DestructAssignment` em `eval/mod.rs:906-909`, um
stub deliberado) revelou, ao medir, **dois problemas anteriores e mais
fundamentais**:

1. **`let (a, b) = ...` já estava errado** (`bindings.rs`, antes desta
   correcção): `eval_let` usava `pattern.bindings().into_iter().next()`
   — ligava **só o primeiro** ident **ao valor inteiro** (não ao
   elemento correspondente), e nunca definia os restantes. Medido:
   ```
   #let (a, b) = (1, 2)
   #a  → error (ou o Array inteiro, dependendo da leitura)
   #b  → error: unknown variable: b
   ```
2. **Atribuição simples (`x = 5`) não tinha braço nenhum** em
   `eval_binary_op` — `#{ x = 2 }` dava `"cannot apply Assign to int
   and int"`. As duas formas (atribuição simples e atribuição por
   desestruturação) partilham a mesma necessidade de raiz: mutar um
   binding **já existente**, distinto de `define` (que cria sempre um
   novo binding no scope actual).

Dado que a atribuição por desestruturação (`(a,b) = expr`) precisa
exactamente do mesmo mecanismo de mutação que a atribuição simples
(`x = 5`), implementar só uma sem a outra seria incompleto — corrigi
as duas juntas, mais o `let`-destructuring quebrado (mesmo mecanismo
de fundo, `Pattern` + `Value`).

### 1.2 Mecanismo vanilla confirmado (`typst-eval/binding.rs`, `access.rs`)

Um único `destructure_impl` genérico, parametrizado por uma função `f`
que decide o que fazer com cada folha (par ident-expr + valor):
- `destructure()` (para `let`) — `f` só aceita `Expr::Ident`, define um
  binding novo.
- `DestructAssignment::eval` — `f` usa `Access` (`access.rs:14-27`,
  mutação via referência mutável), suportando `Ident`, `Parenthesized`,
  `FieldAccess` (campo de dict), `FuncCall` (só métodos "accessor" tipo
  `.at()`).

Suporta array (posicionais + `..sink`, tamanho do sink calculado, não
iterativo) e dict (`ident` shorthand = `key: key`; `key: pattern`
renomeia/aninha; `..sink` recolhe chaves não usadas num novo dict),
recursivo (padrões aninhados). Mensagens de erro exactas medidas:
`"cannot destructure {ty}"`, `"cannot destructure named pattern from
an array"`, `"cannot destructure unnamed pattern from dictionary"`,
`"{quantifier} elements to destructure"` com hint `"the provided array
has a length of {len}, but the pattern expects {expected}"`, `"cannot
mutate a temporary value"`.

### Critério de fecho da sonda

- [x] Alcance real confirmado (2 bugs anteriores, não 1) — `let`
      destructuring quebrado + atribuição simples ausente.
- [x] Mecanismo vanilla confirmado, `file:line`, incluindo mensagens
      de erro exactas e o hint de aridade.
- [x] Scope-out do `Access` genérico (`FieldAccess`/`FuncCall` como
      alvo) identificado e justificado (ver §4).

---

## 2. Implementação

### `entities/scope.rs` + `rules/scopes.rs` — nova capacidade de mutação

- `Binding::value_mut(&mut self) -> &mut Value`, `Scope::get_mut(&mut
  self, name) -> Option<&mut Value>`.
- `Scopes::get_mut(&mut self, name) -> Option<&mut Value>` — pesquisa
  `top` → `scopes` (mesma ordem de `get`); **não** pesquisa `captured`
  nem `base` (scope-out medido, ver §4).

### `rules/eval/bindings.rs` — mecanismo de desestruturação partilhado

- `destructure_pattern`/`destructure_array`/`destructure_dict` — mirror
  exacto do `destructure_impl`/`destructure_array`/`destructure_dict`
  vanilla, incluindo mensagens de erro e o hint de aridade
  (`wrong_number_of_elements`).
- `destructure_let` — a `f` de `let` (só `Expr::Ident`, `scopes.define`).
  `eval_let` reescrito para a chamar em vez do `.next()` quebrado; a
  nomeação pós-hoc de closures (`#let f = (n) => ...` → recursão)
  preservada, mas restrita ao caso `Pattern::Normal(Expr::Ident(_))`
  (o vanilla também não a faz em `destructure()` genérico).
- `eval_destruct_assignment` (nova) — a `f` de atribuição: `Ident` →
  `scopes.get_mut`; qualquer outra folha → erro (scope-out, §4).
- `eval_assign` (nova) — atribuição simples/composta a um `Expr::Ident`;
  `+=`/`-=`/`*=`/`/=` lêem o valor actual via `scopes.get`, aplicam
  `eval_binary_op` com o operador subjacente, escrevem via
  `scopes.get_mut`.

### `rules/eval/mod.rs` — fiação

- `Expr::Binary` ganha um braço-guarda ANTES do dispatch genérico:
  se `op` é `Assign`/`AddAssign`/`SubAssign`/`MulAssign`/`DivAssign`,
  chama `eval_assign` (não avalia `lhs` como valor — precisa do nome).
- `Expr::DestructAssignment` deixa de ser stub — chama
  `eval_destruct_assignment`.

### Testes (21 novos, todos no crate `typst-core`)

- `entities/scope.rs` (2): `get_mut` muta binding existente / devolve
  `None` se ausente.
- `rules/scopes.rs` (4): muta em `top`, atravessa âmbitos pai
  (mutação sobrevive à saída do scope filho), ausente devolve `None`,
  **não** pesquisa `captured`.
- `eval/tests.rs` (15): `let`-destructuring (array simples,
  placeholder, spread, aninhado, dict shorthand+named, dict spread,
  aridade errada com hint, valor não-destructurável — 8); atribuição
  por desestruturação (muta existentes — reprodução exacta do padrão
  `cetz`, variável inexistente erra — 2); atribuição simples/composta
  (simples, `+=`/`-=`/`*=`/`/=`, variável inexistente erra, alvo
  não-ident erra, mutação atravessa fronteira de bloco `{ }` — 5).

---

## 3. Validação

Reprodução manual (release build), contra o vanilla — todos os casos
idênticos:

```
#let (a, b) = (1, 2)                         → 1 2   (idêntico)
#let (project: p, onto: (x, y)) = (...)      → hi 3 4 (idêntico, aninhado)
#{ (ctx, p) = (10, 20) }  (já definidos)      → 10 20  (idêntico)
#{ x = 5 }; #{ x += 10 }                      → 5 15   (idêntico)
#let (first, ..rest) = (1,2,3,4)             → 1 (2,3,4) (idêntico)
#let (_, b) = (1, 2)                          → 2      (idêntico)
#let (a, b, c) = (1, 2)                      → Err "not enough elements
                                                to destructure" + hint
                                                idêntico (só a posição
                                                do span diverge — o
                                                vanilla aponta para o
                                                padrão completo, o
                                                cristalino para o
                                                `let`; mecânica, não
                                                língua)
#{ (nope, x) = (1, 2) }                       → Err "unknown variable:
                                                nope" (idêntico)
```

- `cargo test --workspace` → **3841** (typst-core: 3820 + 21 novos,
  todos os 21 no mesmo crate) + **630** (typst-infra, inalterado) +
  inalterado nos restantes, 0 failed.
- `crystalline-lint .` → 0 violations; `--fix-hashes .` realinhou 10
  ficheiros (`eval.md` mudou de conteúdo); `bindings.rs` manteve o
  hash antigo para `rules/eval.md` (segundo `@prompt` do ficheiro,
  `field-access.md`, parece afectar o realinhamento automático) — sem
  violação reportada por `crystalline-lint .`, consistente com
  comportamento já observado noutros ficheiros multi-prompt nesta
  cadeia (P711 `pipeline.md`).

### Reprodução `cetz`

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({ import cetz.draw: *; line((0,0),(2,1)); circle((0,0)) })
```

**Bloqueio anterior resolvido** — `destructuring assignment` deixou de
ser a causa. **Novo bloqueio**: `error: cannot mutate a temporary
value` — confirmado como o scope-out medido (§4): `cetz` usa
`arr.at(i) = valor` (`hobby.typ:51,52,56,58,138-242` — mutação de
elemento de array via `.at()`) e `dict.campo = valor`
(`drawable.typ:36` — `drawable.segments = drawable.segments.map(...)`).
Tempo de compilação **~52.9s**, mesma ordem de grandeza, sem
regressão.

### Próximo passo sugerido (não iniciado, candidato P716)

Implementar o `Access` genérico do vanilla (`typst-eval/access.rs`)
para `FieldAccess` (mutação de campo de dict) e `FuncCall` acessor
(`.at()` em array/dict) como alvos de atribuição/desestruturação —
mecanismo real de referência mutável, não substituição do valor
completo. Confirmar primeiro no vanilla a lista exacta de "accessor
methods" reconhecidos (`is_accessor_method`) antes de implementar.

---

## 4. Não corrigido aqui — scope-out medido

- **`Access` genérico** (`FieldAccess`/`FuncCall` como alvo de
  atribuição) — confirmado como o próximo bloqueio real de `cetz`
  (`hobby.typ`, `drawable.typ`). Candidato a P716 (ver acima).
- **Mutação de variáveis `captured`** (scope de definição de uma
  closure) — `Scopes::get_mut` não pesquisa `captured` nem `base`;
  sem consumidor medido (o padrão real do `cetz`, `(ctx, p) =
  resolve(ctx, p)`, muta variáveis locais na mesma pilha léxica, não
  capturadas de um closure exterior).

## 5. Critério de fecho do passo

- [x] Sonda completa: alcance real (2 bugs, não 1) confirmado,
      mecanismo vanilla medido com mensagens de erro exactas.
- [x] Corrigido: `let`-destructuring (array, dict, aninhado, spread,
      placeholder), atribuição por desestruturação, atribuição
      simples/composta — todos testados com múltiplos casos.
- [x] Sem regressão em `cargo test --workspace` (3841 vs 3820 antes).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — bloqueio de desestruturação resolvido, novo
      bloqueio identificado (`Access` genérico), sem regressão de
      tempo.
- [x] Relatório com resultado exacto (este ficheiro), scope-out
      medido registado (§4).
