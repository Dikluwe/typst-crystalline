# P772n — L0 + Implementação: `cannot_mutate_constant` (mutação silenciosa de nomes da stdlib)

> **Passo:** 772n
> **Data:** 2026-07-16
> **Commit-base:** `61b7edee78fdae9b020e458f5989f638cbf04096` (HEAD, mesmo de
> P772k/l/m).
> **Medido/implementado em:** 2026-07-16T22:38–23:30Z.
> **Dependência:** P772l §2.3 (achado original).

---

## 1. Sonda — mecanismo exacto do vanilla

```
$ grep -n "cannot_mutate_constant\|BindingKind\|fn bind\b" \
    lab/typst-original/crates/typst-library/src/foundations/scope.rs
```

**Achado que corrige a hipótese do prompt do passo.** A hipótese inicial
("`Binding` ganha `kind: BindingKind` com `Normal`/`Const`") está **errada**.
O vanilla não marca bindings individuais como constantes. O mecanismo é
**estrutural**:

```rust
// Scopes::get_mut (foundations/scope.rs:63-70)
pub fn get_mut(&mut self, var: &str) -> HintedStrResult<&mut Binding> {
    std::iter::once(&mut self.top)
        .chain(&mut self.scopes.iter_mut().rev())
        .find_map(|scope| scope.get_mut(var))
        .ok_or_else(|| {
            match self.base.and_then(|base| base.global.scope().get(var)) {
                Some(_) => cannot_mutate_constant(var),
                _ if var == "std" => cannot_mutate_constant(var),
                _ => unknown_variable(var),
            }
        })
}
```

`get_mut` só pesquisa `top`/`scopes` — **nunca** `base`. A stdlib vive
inteiramente em `base` (nunca em `top`/`scopes`), por isso é imutável **por
construção**, sem qualquer flag por-binding. A escolha entre as duas
mensagens de erro é só uma verificação extra no caminho de falha: existe em
`base.global`? É `"std"`? → `cannot_mutate_constant`. Senão →
`unknown_variable`.

`BindingKind`/`Capturer` (o par que P772l §2.2 identificou) existe só para
**outro** caso: variável capturada por closure/`context`. Não está envolvido
aqui.

### Mensagem exacta (medida palavra por palavra)

```
#{ calc = 5 }   → error: cannot mutate a constant: calc     (exit 1)
#{ std = 5 }    → error: cannot mutate a constant: std      (exit 1)
#{ image = 5 }  → error: cannot mutate a constant: image    (exit 1)
#{ table = 5 }  → error: cannot mutate a constant: table    (exit 1)
```

Mensagem **uniforme**, sem variação por contexto (ao contrário do par
`Capturer::Function`/`Capturer::Context` de P772l §2.2).

### Sombra local continua mutável

```
#let calc = 5
#{ calc = 10 }
#calc
```

Vanilla: exit 0, sem erro. `#let` local cria um binding **normal** em
`top`, que `get_mut` encontra **antes** de sequer olhar para `base` —
confirma que a protecção é por camada de âmbito, não por nome.

---

## 2. Decisão de âmbito (registada antes de implementar)

| Achado | Incluído neste passo? | Razão |
|---|---|---|
| `cannot_mutate_constant` (prioridade) | **Sim** | Objectivo do passo. |
| §2.2 — mensagem correcta para `Capturer` (`unknown variable` → `variables from outside the function/context are read-only`) | **Não** | Mecanismo **diferente** do usado aqui (tag de "por que motivo o scope foi capturado" em `Scopes`, não a separação `base`/`top` que resolve `cannot_mutate_constant`). Medir e implementar os dois em conjunto arrisca sub-medir um deles; mantém-se o achado registado em P772l, candidato a passo próprio e curto — a infra-estrutura de erro (`is_constant`-style) já está pronta como precedente. |
| §2.6 — avisos de depreciação (`Deprecation`) | **Não** | Severidade menor (warning, não altera o documento), o próprio passo já assinalava "pode ficar para depois sem risco"; precisa de dados de depreciação por símbolo que a stdlib do cristalino ainda não carrega. |

---

## 3. L0 — escrito antes do código

Actualizados antes de qualquer alteração em `01_core/`:

- `00_nucleo/prompts/world-types.md` — `Library` deixa de ser stub opaco
  `()`; ganha `global: Scope`. Tabela "Destino de cada tipo" e "Nota sobre
  Library e FontBook" actualizadas; histórico de revisões.
- `00_nucleo/prompts/engine/scopes.md` — `Scopes::get` deixa de ter `base`
  como stub (consulta real); nova secção "Protecção de bindings
  constantes (P772n)"; novo método `is_constant` na interface pública;
  critérios de verificação estendidos.
- `00_nucleo/prompts/engine/eval.md` — braço `Ident` de `access()`
  actualizado; nova secção `§P772n` com o mecanismo, a correcção e os
  critérios de verificação.

---

## 4. Implementação

### 4.1 `Library` materializada (`01_core/src/entities/world_types.rs`)

`Library(())` → `Library { pub global: Scope }`. `Library::new()`
mantido (produz `global` vazio) — preserva ~30 mocks de `World` em testes
não relacionados com eval/stdlib (`grep Library::new` confirmou todos os
call sites antes de mudar a assinatura — nenhum quebrado, já que
`new()` continua zero-arg). `Library::with_global(scope)` é o construtor
novo, só usado no bootstrap real.

Nota: `Library` perdeu `#[derive(PartialEq, Eq, Hash)]` (tinha quando era
`Library(())`) — `Scope` não implementa esses traits e nada no workspace
dependia deles (confirmado pelo build limpo).

### 4.2 `Scopes` (`01_core/src/engine/scopes.rs`)

- `get()`: `base` deixa de ser stub (`let _ = self.base; None`) — consulta
  real `self.base.and_then(|base| base.global.get(name))`, como último
  recurso.
- `is_constant(name) -> bool` (novo): true sse `name` só é alcançável via
  `base` — não em `top`, `scopes`, nem `captured`.
- `snapshot()`: passou a incluir `base` na captura eager (ver §4.4 —
  achado durante a implementação, não previsto na sonda).

### 4.3 Bootstrap do avaliador

`01_core/src/engine/eval/mod.rs` (`run_pass`) e
`01_core/src/engine/eval/modules.rs` (`eval_imported_file`): deixam de
fazer `scopes.define(name, ...)` para cada item de stdlib/cores/`std`/
`text`/elementos de utilizador (o que os achatava em `scopes.top`,
indistinguíveis de bindings normais — causa raiz de P772l §2.3). Em vez
disso: constroem um `Scope` local (`global`), embrulham-no em
`Library::with_global(global)`, e chamam `Scopes::new(Some(&library))`.
O `scopes.enter()` que existia só para abrir um `top` fresco para o corpo
do documento deixou de ser necessário (o `top` já nasce vazio) — `exit()`
no fim continua a funcionar sem alteração (`scopes.pop().unwrap_or_default()`
quando não há frame empurrado).

### 4.4 `access()` (`01_core/src/engine/eval/bindings.rs`)

Braço `Expr::Ident` (mutação): quando `get_mut` falha,
`scopes.is_constant(name)` decide entre `"cannot mutate a constant:
{name}"` e `"unknown variable: {name}"`.

### 4.5 Achado durante a implementação, não previsto na sonda: `snapshot()` precisava de incluir `base`

Primeira versão do código (base real em `get`/`get_mut`, sem tocar
`snapshot()`) fez **11 testes falharem** — todos por `"unknown variable:
upper"` ou equivalente, dentro de closures/show-rules
(`#show heading: it => upper(it.body)`). Causa: `Scopes::with_parent`
(usado em cada chamada de closure) cria `Scopes<'static>` com `base:
None` — não pode carregar o lifetime de `&'a Library`, que não é
`'static`. Antes desta correcção, a stdlib "escapava" para dentro das
closures só porque vivia em `scopes.scopes[0]` (achatada, alcançável por
`snapshot()`, que já incluía `scopes`). Ao mover a stdlib para `base`,
`snapshot()` deixou de a ver — closures perderam acesso à stdlib.

**Correcção**: `snapshot()` passa a incluir `base.global` (com prioridade
mais baixa, antes de `captured`/`scopes`/`top`, replicando a ordem de
`get`). Isto não reabre P772l §2.3: `get_mut` continua a nunca pesquisar
`captured`, por isso mutar um nome de stdlib capturado por uma closure
continua a falhar — só que com a mensagem "unknown variable" (o caso
"captured", P772l §2.2, deliberadamente fora do âmbito deste passo) em
vez de "cannot mutate a constant". Registo consistente com a regra de
proveniência do `CLAUDE.md`: a medição (11 falhas, `cargo test
--workspace`, mesmo commit) revelou algo que a sonda inicial não
antecipou — corrigido e re-medido antes de fechar o passo.

---

## 5. Validação

### 5.1 Casos do passo

```
#{ calc = 5 }   → cannot mutate a constant: calc    (exit 1) ✓ idêntico ao vanilla
#{ std = 5 }    → cannot mutate a constant: std     (exit 1) ✓
#{ image = 5 }  → cannot mutate a constant: image   (exit 1) ✓
#{ table = 5 }  → cannot mutate a constant: table   (exit 1) ✓

#let x = 1; #{ x = 2 }; #x           → sem erro, "2"   ✓ não regride
#let calc = 5; #{ calc = 10 }; #calc → sem erro, "10"  ✓ paridade sombra
#{ zzz = 5 }                          → unknown variable: zzz ✓ não regride

#import "lib.typ" (lib.typ com #{ calc = 5 })
  → cannot mutate a constant: calc, no ficheiro importado ✓
```

### 5.2 Não-regressão de closures/stdlib

```
#let f(x) = calc.abs(x)
#f(-5)
```

→ `5`, sem erro (confirma que `snapshot()` restaura acesso da closure à
stdlib).

### 5.3 Suite completa

```
cargo test --workspace
  4176 passed (typst-core, +11 face a antes de P772n: 2 helpers de
  is_constant/base + 2 testes co-localizados em scopes.rs + regressão
  resolvida das 11 falhas transitórias de §4.5)
  644 + 33 + 2 + 29 + 2 + 0 restantes, 0 falhas
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente, não relacionado)
```

---

## 6. Decisão

`cannot_mutate_constant` fechado — mecanismo estrutural (via `Library`
materializada + `base` real em `Scopes`), não a flag `BindingKind::Const`
hipotética do prompt do passo. Isto significa que a "infra-estrutura de
schema" que o passo antecipava para §2.2/§2.6 **não foi criada** — esses
dois achados continuam a precisar do seu próprio mecanismo (`Capturer` é
sobre *por que* um scope foi capturado, não sobre onde a stdlib vive) e
do seu próprio L0. Registar isto explicitamente para não assumir, num
passo futuro, que "a mudança de schema já está pronta" — não está, porque
não foi essa a mudança feita.

---

## Critério de fecho do passo (`typst-passo-772n.md`)

- [x] Mensagem de erro do vanilla confirmada palavra por palavra,
      incluindo `std`/`image`/`table`, sem variação por contexto.
- [x] Decisão de âmbito registada (§2.2/§2.6 explicitamente não incluídos,
      com razão).
- [x] L0 escrito antes do código (`world-types.md`, `rules/scopes.md`,
      `rules/eval.md` §P772n).
- [x] `Binding` **não** ganhou novo campo — a sonda corrigiu a hipótese;
      em vez disso, `Library` (stub `()` desde a criação) foi
      materializada com `global: Scope`, e `Scopes.base` (já existente,
      sempre `None`) passou a ser usado a sério.
- [x] `#{ calc = 5 }` (e `image`, `table`, `std`) dá erro idêntico ao
      vanilla.
- [x] Bindings normais de utilizador continuam mutáveis, sem regressão
      (incluindo sombra local de nome de stdlib).
- [x] `cargo test --workspace` verde (4176 + 644 + 33 + 2 + 29 + 2, 0
      falhas — incluindo a correcção de 11 falhas transitórias
      encontradas e resolvidas durante este mesmo passo, §4.5).
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772n.md`.

---

## Próximo passo

§2.2 (`Capturer` — mensagem correcta para mutação de variável capturada
por closure/`context`) e §2.6 (avisos de depreciação) continuam abertos,
cada um precisando do seu próprio L0 (mecanismo distinto do usado aqui —
ver §6). Fora isso, prioridades por severidade continuam: imagem com
formato desconhecido omitida silenciosamente (P772k/P650), colapso de
espaço em fontes variáveis de peso alto (P772m §3), ou reconfirmação da
varredura da stdlib (P772e-style, segunda rodada).
