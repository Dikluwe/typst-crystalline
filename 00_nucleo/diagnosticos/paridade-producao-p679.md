# P679 — `#import` de ficheiros locais (nível 5a de P678)

**Passo:** 679
**Data:** 2026-07-10
**Foco:** implementar `eval_module_import` para caminhos relativos — pré-requisito absoluto (P-α de P678) para pacotes `@preview` e útil por si só.
**Tipo:** Implementação (L1), com sonda de paridade contra o vanilla local.
**Commit base:** `320d607ae — P678: adiciona hash do commit ao relatório`
**ADR-0108 em vigor** (medir antes de decidir). **ADR-0107 em vigor** (paridade com a linguagem, não com a mecânica).

---

## 1. Medição na fonte vanilla (`lab/typst-original`)

Fonte: `crates/typst-eval/src/import.rs` (0.15.0, `969087ec`).

- `import.rs:22` — o `source` é avaliado (`source_expr.eval(vm)?`); string literal → `Value::Str`.
- `import.rs:33-39` — `Value::Str(path) => import(...)`: resolve o caminho e **substitui** `source` pelo `Value::Module` avaliado (`replaced_source = true`).
- `import.rs:60-75` — `as nome`: liga o módulo sob `new_name`.
- `import.rs:78-106` — bare import (`imports() == None`, sem `as`): liga sob `bare_name()` (file_stem). Caminho dinâmico → `"dynamic import requires an explicit name"` (linha 96); file_stem não-identificador → `"module name would not be a valid identifier"` (linha 100).
- `import.rs:108-111` — `Imports::Wildcard`: itera `scope.iter()` do módulo e liga cada binding.
- `import.rs:113-121` — `Imports::Items`: lookup no scope do módulo; se falta, `error!(component.span(), "unresolved import")` (linha 121).
- `import.rs:77` — `let scope = source.scope().unwrap();`: um `Value::Module` expõe o seu `Scope`, base para `#mod.campo` (field access).

**Classificação (ADR-0108):** a forma `#import "f.typ": ...` e o acesso `#mod.campo` são **sintaxe/morfologia** da linguagem (paridade); as mensagens de erro (`unresolved import`, `cyclic import`) são **observáveis mecânicos** (paridade ao nível do texto do erro); o algoritmo interno (`import`/`import_file`, `Tracepoint`, `VirtualRoot`) é **mecânica** e diverge de propósito (P329).

---

## 2. Sonda de paridade (cristalino vs vanilla)

Binários: vanilla `lab/typst-original/target/release/typst` (`typst 0.15.0 (969087ec)`); cristalino `target/debug/typst` (build do working tree de P679). Texto extraído com `pdftotext -layout`, whitespace normalizado. Ficheiro importado `p679-utils.typ`:

```typst
#let saudacao(nome) = "Olá, " + nome + "!"
#let PI = 3.14159
```

| # | Forma | Resultado (vanilla = cristalino) |
|---|---|---|
| 1 | `#import "u.typ": saudacao` → `#saudacao("Mundo")` | `Olá, Mundo!` ✅ |
| 2 | `#import "u.typ": *` → `#saudacao("Mundo") #PI` | `Olá, Mundo! 3.14159` ✅ |
| 3 | `#import "u.typ": saudacao as ola` → `#ola("Mundo")` | `Olá, Mundo!` ✅ |
| 4 | `#import "u.typ"` → `#u.saudacao("Mundo")` | `Olá, Mundo!` ✅ |
| 5 | `#import "u.typ" as u` → `#u.saudacao("Mundo")` | `Olá, Mundo!` ✅ |
| 6 | stdlib visível no importado: `#let nums = range(3)` no ficheiro | `(0, 1, 2)` ✅ |
| 7 | `#include "inc.typ"` (regressão) | conteúdo incluído ✅ |

**PASS = 7 / 7** — texto idêntico ao vanilla nas sete formas.

Comportamentos confirmados pelo vanilla e replicados:
- O markup solto do ficheiro importado **não** aparece no documento importador (só os bindings contam).
- O ficheiro importado vê a stdlib (`range(3) → (0, 1, 2)`).

---

## 3. Implementação

### 3.1 `eval_module_import` (`01_core/src/rules/eval/modules.rs`)

Substitui o stub (`"import não implementado nesta versão do cristalino"`). Assinatura alinhada a `eval_module_include`: `(import, scopes, ctx, engine)`. O dispatcher em `mod.rs:744` passou a chamar `modules::eval_module_import(i, scopes, ctx, engine)`.

Fluxo:
1. `import.source()` tem de ser `Expr::Str` (caminho literal); caso contrário → `import: caminho deve ser uma string literal`.
2. Caminho a começar por `@` → `import de pacotes (@preview/...) ainda não é suportado pelo cristalino` (scope-out; pacotes são P-β/P-γ de P678).
3. Resolução: `engine.world.include_source(engine.current_file, &path)` (caminho relativo ao ficheiro actual; regista o ficheiro). Erro propaga a mensagem do world (L0 §P679 ponto 3).
4. Ciclo: `engine.route.contains(src_id)` → `ciclo de importação detectado: ficheiro ... já está na cadeia de avaliação activa` (mesma família de `#include`).
5. `module_name = import.bare_name()` (file_stem); se inválido → `module name would not be a valid identifier`.
6. Avalia o ficheiro num **módulo isolado** (`eval_imported_file`) → `Module::new(module_name, module_scope)`.
7. Liga bindings no escopo do chamador conforme `imports()`:
   - `None` → módulo sob `new_name` (`as`) ou `bare_name` (`Value::Module`).
   - `Wildcard` → cada `(name, value)` de `module.scope().iter()`.
   - `Items` → `module.scope().get(orig).cloned()`; se `None` → `unresolved import: \`{orig}\`` (span do item); senão liga sob `bound_name`.
8. Retorna `Value::None` (em markup é descartado; os efeitos são os bindings).

### 3.2 `eval_imported_file` (helper em `modules.rs`)

Avalia o ficheiro importado num módulo isolado, espelhando o `run_pass` do eval principal (Passo 109, ADR-0044) mas numa única passagem (conteúdo descartado):
- Scope base próprio: stdlib (`super::make_stdlib()`) + cores predefinidas + `text`.
- `Engine` local com `StyleChain`/`show_rules`/`active_guards` próprios (**não** partilhados com o importador — `#set`/`#show` do importado não vazam); `route = Route::extend(engine.route).with_id(src_id)`; `current_file = src_id`; `sink` reborrowed (warnings do importado chegam ao caller).
- `EvalContext` local (herda `full_error`; `apply_show_rules = false`).
- `eval_markup(source.root(), ...)`; se `ctx.flow` (`#return`/`#break` solto) → `flow.forbidden()`.
- `module_scopes.exit()` → `Module::new(name, scope)`.

### 3.3 Field access em `Value::Module` (`01_core/src/rules/eval/bindings.rs`)

Novo armo em `eval_field_access` (pré-requisito das formas 4 e 5 — sem ele, `#u.campo` dava `field access não suportado em module`):

```rust
Value::Module(m) => m.scope().get(field.as_str()).cloned().ok_or_else(|| {
    vec![SourceDiagnostic::error(access.span(),
        format!("módulo '{}' não tem campo '{}'", m.name(), field))]
}),
```

O valor obtido é tipicamente `Value::Func`, que o dispatcher de chamada (`apply_func`) já trata — logo `#u.saudacao("Mundo")` funciona sem mais alterações.

### 3.4 Prompts L0 actualizados

- `00_nucleo/prompts/rules/eval.md` — nova secção **§P679** (medição file:line, classificação, semântica, critérios). Hash `cce90241` → `a660f985` (propagado por `crystalline-lint --fix-hashes` aos 10 ficheiros que referenciam `eval.md`).
- `00_nucleo/prompts/rules/eval/field-access.md` — nova secção **§11 Field Access `Module` (P679)**. Hash `c822a5ed` → `d935c8b5` em `bindings.rs`.

---

## 4. Erros (cristalino)

| Caso | Cristalino | Vanilla (referência) |
|---|---|---|
| Ciclo `a → b → a` | `ciclo de importação detectado: ficheiro FileId(1) já está na cadeia de avaliação activa` | `error: cyclic import` (com trace) |
| Ficheiro inexistente | `include: ficheiro não encontrado: …/nao-existe.typ` (mensagem do world) | `error: file not found (searched at …)` |
| Item inexistente | `unresolved import: \`naoexiste\`` (span do item) | `error: unresolved import` |
| Campo inexistente em módulo | `módulo 'p679-utils' não tem campo 'inexistente'` | `error: module does not contain …` |

A detecção de ciclo reutiliza `Route::contains` (ADR-0033/0036), já usada por `#include`. A mensagem de ciclo é da família do cristalino (não byte-idêntica ao vanilla) — é **observável mecânico**; a **linguagem** (detectar ciclo e errar, não panicar nem loopar) é paridade.

---

## 5. Testes

Em `01_core/src/rules/eval/tests.rs`:
- Corrigido `eval_import_retorna_err_sem_panic` (verificava o stub) → substituído por `import_ficheiro_ausente_retorna_err_sem_panic` (ficheiro não registado → `include_source` falha → `Err` limpo, sem panic).
- Adicionado `ImportMockWorld` (mapa path→`Source` com `FileId` estável; `include_source` por clone) e 8 testes novos: `import_item_unico`, `import_wildcard`, `import_rename_item`, `import_bare_modulo_field_access`, `import_as_modulo_field_access`, `import_stdlib_visivel_no_ficheiro_importado`, `import_item_inexistente_retorna_unresolved_import`, `import_ficheiro_ausente_retorna_err_sem_panic`. O teste pré-existente `import_cycle_detectado_retorna_err_sem_panic` continua a passar.

Validação:
- `cargo test --workspace` → **verde** (`3657` passed no core lib; `0 failed` no workspace).
- `crystalline-lint .` → **No violations found**.

---

## 6. Limitações e débitos

- **Pacotes `@preview/...` fora do scope** — devolvem erro claro. São P-β/P-γ/P-δ de P678 (resolução de pacote, cache, download). `PackageSpec` já existe em L1; a resolução é trabalho posterior.
- **Registry de elementos de utilizador não propagado ao módulo importado** — `eval_imported_file` constrói o scope base com stdlib + cores + `text`, mas não com `registry.names()` (o `Engine` não carrega o registry). Em produção o registry está vazio até pacotes registarem elementos; ficheiros locais com `#let`/`#fn` não são afectados. Propagá-lo exigiria adicionar `registry` ao `Engine` ou à assinatura — débito, fora do scope de P679.
- **Mensagem de ficheiro inexistente é a do world** (`include: ficheiro não encontrado` em produção via `03_infra`, `ficheiro não encontrado` no `ImportMockWorld`). O L0 §P679 ponto 3 decide propagar a mensagem do world; diferenciá-la de `#include` exigiria uma variante de `include_source` ou re-escrita em L1 — débito de mensagem, não de linguagem.
- **Mensagem de ciclo não é byte-idêntica ao vanilla** — usa a família do cristalino (partilhada com `#include`). Paridade ao nível da linguagem (detecta e erra), não do texto.

---

## 7. Critério de fecho do passo

- [x] `eval_module_import` implementado (5 formas) — §3.1.
- [x] Módulo isolado (scope/engine/ctx próprios; markup descartado; stdlib visível) — §3.2.
- [x] Field access em `Value::Module` (formas bare/`as`) — §3.3.
- [x] Ciclo, ficheiro inexistente, item inexistente, campo inexistente → erros limpos (sem panic) — §4.
- [x] `#include` sem regressão — §2 (forma 7).
- [x] Paridade medida contra o vanilla local — §2 (PASS 7/7).
- [x] Testes automatizados (8 novos + 1 corrigido) e `cargo test --workspace` verde — §5.
- [x] Prompts L0 actualizados e `crystalline-lint .` limpo — §3.4.
- [x] Pacotes `@preview` explicitamente fora do scope (erro claro), com encaminhamento para P-β..δ de P678 — §6.

---

## 8. Proveniência das medições

- **Commit base:** `320d607ae — P678: adiciona hash do commit ao relatório`.
- **Hora da validação:** 2026-07-10T15:21:07Z (working tree = trabalho de P679; código cristalino = commit "Hash do commit" abaixo).
- **Vanilla usado:** `lab/typst-original/target/release/typst` — `typst 0.15.0 (969087ec)`.
- **Cristalino usado:** `target/debug/typst` (build do working tree de P679).
- **Ficheiros alterados (`git diff HEAD --stat`):** ver commit abaixo — `00_nucleo/prompts/rules/eval.md`, `00_nucleo/prompts/rules/eval/field-access.md`, `01_core/src/rules/eval/{modules,mod,bindings,tests}.rs`, e actualização de `@prompt-hash` em `01_core/src/rules/eval/{bibliography,closures,control_flow,flow,markup,math,rules}.rs`.

---

## Hash do commit

`52651c8fc — P679: implementa #import de ficheiros locais (nível 5a de P678)`
