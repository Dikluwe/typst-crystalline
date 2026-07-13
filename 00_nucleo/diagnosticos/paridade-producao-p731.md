# Relatório P731 — Namespaces embutidos (`calc`, `sys`, `math`, `sym`) como `Value::Module`

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-731.md`
**ADRs em vigor:** ADR-0107 (paridade é com a linguagem), ADR-0108 (medir antes de decidir).
**Commit:** `64ff1b49ff25ed57d249233a0f978326fef81b21`
**Proveniência das medições (regra de proveniência):** commit base `61368ca3dc3dfc842e5a7249ee3bbb8bdcb3e8ef` ("P730: preenche hash do commit no relatório"), branch `Tekt`, working tree com as alterações deste passo (`git diff HEAD --stat`: 19 ficheiros — `00_nucleo/prompts/rules/eval.md`, `00_nucleo/prompts/rules/stdlib/{structural,sym,sys}.md`, `01_core/src/rules/eval/{bibliography,closures,control_flow,flow,markup,math,mod,modules,rules,tests}.rs`, `01_core/src/rules/stdlib/{calc,mod,structural,sym,sys}.rs`; os 7 ficheiros `eval/*.rs` não listados na implementação têm só o header `@prompt-hash` sincronizado `696f25e1` → `a3904d9a`). Medições vanilla: `lab/typst-original/target/release/typst`; medições cristalino: `./target/release/typst` (release build de 2026-07-13T22:20Z; compilação cetz medida 22:21:03–22:21:47Z).

---

## Sonda ampla — medições antes de decidir (ADR-0108)

P730 registou o bloqueio: `#import calc: min, max` → erro "import: a fonte tem de ser um caminho string ou um módulo, recebeu dictionary" (`eval/modules.rs:187` exige `Value::Module`), usado pelo cetz em `aabb.typ:18` (corpo de função, caminho do bounds de `line`).

### Tipo de cada namespace embutido — vanilla vs cristalino (medido)

| Namespace | Vanilla | Cristalino (antes) | Acção |
|---|---|---|---|
| `calc` | `module` | `dictionary` | **corrigido neste passo** |
| `sys` | `module` | `dictionary` | **corrigido neste passo** |
| `math` | `module` | `dictionary` | **corrigido neste passo** |
| `sym` | `module` | `dictionary` | **corrigido neste passo** |
| `std` | `module` | `module` | já correcto (P709) |
| `emoji` | `module` | unknown variable | achado registado (fora de scope) |
| `pdf` | `module` | unknown variable | achado registado (fora de scope) |
| `html` | unknown (feature off no binário medido) | unknown variable | paridade acidental |
| `color` | `type` | `dictionary` | achado registado (vanilla trata como tipo, não módulo) |
| `gradient` | `type` | `dictionary` | achado registado (idem) |
| `counter` | `type` | `function` | achado registado |
| `state` | `type` | `function` | achado registado |
| `text` | `function` | `function` | paridade |
| `str` | `type` | `type` | paridade |

Os quatro namespaces que o vanilla expõe como `module` e o cristalino expunha como `dictionary` são exactamente os corrigidos. `emoji`/`pdf` (ausentes), `color`/`gradient` (vanilla: `type`) e `counter`/`state` (vanilla: `type`) ficam registados em `achados-adiados-cetz.md` — nenhum é consumidor do cetz.

### Localização exacta da construção (cristalino)

- Scope global em `01_core/src/rules/eval/mod.rs:1436-1447`: `sym` via `build_sym_dict()` (`stdlib/sym.rs:108`), `calc` via `make_calc_module()` (`stdlib/calc.rs:48`), `math` via `make_math_module()` (`stdlib/structural.rs:2073`), `sys` via `make_sys_module()` (`stdlib/sys.rs:32`) — todas devolviam `Value::Dict(IndexMap)`.
- Field access em `Value::Module` **já existia** (P679, `eval/bindings.rs:1425-1430`: `m.scope().get(field)`) — sem alteração necessária.
- Consumidor runtime localizado: `lookup_math_op` em `eval/math.rs:44` (acesso por campo sobre o módulo `math`). `#set math.equation(numbering:)` é tratado sintacticamente (`rules.rs:678+`, `text_str` no AST) — não toca o valor runtime.

### Critério de fecho da sonda

- [x] Lista completa de namespaces embutidos confirmada, com o tipo real no vanilla para cada um (tabela acima).
- [x] Estado actual do cristalino confirmado, namespace a namespace.
- [x] Localização exacta de onde cada um é construído (file:line acima).

## L0 (Prompts)

- `00_nucleo/prompts/rules/eval.md` — §Scope global: `calc`/`math`/`sym`/`sys` são `Value::Module` (P731); `color`/`gradient` permanecem `Dict` com nota do achado (vanilla expõe como `type`); §P694: `sys` é `Module`.
- `00_nucleo/prompts/rules/stdlib/sym.md` — representação `Module`; rename `build_sym_dict` → `build_sym_module`.
- `00_nucleo/prompts/rules/stdlib/sys.md` — parágrafo da divergência de repr reescrito para `Module`.
- `00_nucleo/prompts/rules/stdlib/structural.md` — `make_math_module` devolve `Value::Module`.

`crystalline-lint --fix-hashes .` → headers sincronizados (`@prompt-hash` `696f25e1` → `a3904d9a` nos ficheiros vinculados a `eval.md`); `crystalline-lint .` → **0 violations**.

## Implementação

Conversão de `Value::Dict(IndexMap)` para `Value::Module` em quatro builders, mantendo o conteúdo (mesmos nomes e valores) — só muda o contentor:

1. `stdlib/calc.rs` — `make_calc_module()`: o `IndexMap` interno é convertido no fim em `Scope` e embrulhado em `Module::new("calc", scope)`.
2. `stdlib/sym.rs` — `build_sym_dict` **renomeada** `build_sym_module` (nome passa a dizer a verdade sobre o tipo devolvido); imports `IndexMap`/`FxBuildHasher` removidos; 2 testes do ficheiro actualizados.
3. `stdlib/sys.rs` — `make_sys_module()` constrói `Scope` directamente; 2 testes actualizados.
4. `stdlib/structural.rs` — `make_math_module()` converte no fim (mesmo padrão de `calc`).
5. `eval/math.rs` — `lookup_math_op` passa de `Dict::get` para `m.scope().get` (único consumidor runtime do módulo `math`).
6. `eval/mod.rs:1436` + import (linha 1084) e `stdlib/mod.rs:134` — re-export renomeado para `build_sym_module`.
7. Testes antigos que faziam pattern-match em `Value::Dict` sobre estes módulos actualizados para `Value::Module` em `stdlib/mod.rs` (`lookup_math`, `p299_math_module_total_42_operadores` — 43 entradas via `m.scope().len()`, `p471_sym_dict_*` ×2, blocos de testes `calc` ×4 com `dict.iter().map(|(_, b)| b.value())`).

O mecanismo de `#import` (`eval/modules.rs:187`) **não foi tocado** — já rejeitava `Dict` correctamente (P679); a correcção é na origem da representação.

## Validação

- Fail-first confirmado: `cargo test -p typst-core p731` antes da implementação → **3 failed / 2 passed** (os 2 que passavam cobriam ausência de regressão de field access e de `math`, que dependiam de caminhos ainda funcionais).
- Depois: **5 passed, 0 failed** (`p731_type_namespaces_sao_module`, `p731_import_calc_items`, `p731_import_calc_wildcard` (pi), `p731_field_access_sem_regressao`, `p731_math_module_sem_regressao`, secção P731 em `eval/tests.rs`).
- `cargo test --workspace` — **4722 passed, 0 failed** (4027 + 631 + 33 + 2 + 27 + 2; 8 ignored pré-existentes). Pré-P731: 4717; +5 = os novos testes do passo. Atenção redobrada à regressão silenciosa (o passo muda representação usada em toda a stdlib): zero falhas, nenhum teste antigo precisou de ajuste de lógica — só de interface (pattern `Dict` → `Module`).
- `crystalline-lint .` — **0 violations** (`--fix-hashes`: "Nothing to fix" após a sincronização da fase L0).
- E2E do passo (`/tmp/p731-cristalino-tipos.typ`): `#type(calc)` … `#type(std)` → **`module module module module module`** — idêntico ao vanilla.
- E2E import (`/tmp/p731-import.typ`, conteúdo exacto do passo): `#import calc: min, max` + `#min(1, 2)` + `#max(1, 2)` → saída contém **`1` e `2`** (min e max correctos; pdftotext cola as duas linhas como `12`).

### cetz — reprodução final: **a cadeia fecha**

Com a reprodução exacta do passo (`#import "@preview/cetz:0.5.2"` + canvas com `import cetz.draw: *`, `line((0, 0), (2, 1))`, `circle((0, 0))`):

| | Vanilla | Cristalino |
|---|---|---|
| Compilação | exit 0 | **exit 0** (44 s) |
| Pixels não-brancos (150 dpi) | 1451 | **1508** |
| Diff (>8) | — | **9264 / 6530142 bytes (0.1419%)** — nível de anti-aliasing |

Verificação visual dos dois PNG: desenhos idênticos — círculo na origem atravessado pela linha `(0,0)→(2,1)`, mesma posição e proporções. Em P727 o cristalino renderizava só o círculo (1100 px não-brancos); agora renderiza linha **e** círculo, com pixels a mais (1508 vs 1451) apenas por anti-aliasing da rasterização.

**O diff de pixels aproxima-se de zero (0.14%): a cadeia P678-731 fecha de vez.**

## Resumo da cadeia P678-731 (FECHADA)

- **54 passos** (P678 a P731), 32 deles com relatório de paridade de produção em `00_nucleo/diagnosticos/` (P700–P731).
- Ponto de partida: P700 estabeleceu a validação real com `#import "@preview/cetz:0.5.2"` e `cetz_core.wasm`; o caso mínimo do passo (`line` + `circle` num canvas) só foi compilar e renderizar com paridade visual no fim da cadeia.
- Categorias de bugs corrigidos (relatórios P700–P731):
  - **Carregamento e representação de módulos:** P700 (validação cetz/wasm), P709 (módulo `std`), P731 (namespaces `calc`/`sys`/`math`/`sym` como `Module`).
  - **Funções e chamadas:** P702 (`Func::with`), P708 (binding keyword-only), P715 (desestruturação e atribuição), P724 (destructuring em parâmetro de closure).
  - **Operadores:** P706 (`in`/`not in`), P713 (`Length / Length`), P720 (`array + array`, merge de dicts), P722 (`Array * Int`), P725 (`Length * Int` + saneamento NaN).
  - **Métodos stdlib:** P701 (`cbor`), P703 (`rgb` hex), P704 (`range(step:)`), P705 (`luma`), P707 (métodos de `Arguments`), P710 (`to-absolute`), P714 (`array.at`), P717 (métodos mutantes), P730 (`array.slice`).
  - **Semântica de avaliação e fluxo:** P711 (`context` herda `StyleChain`), P712 (`measure()` real), P716 (`Access` genérico), P718 (spread), P719 (`for` sobre `Dict`), P723 (aridade do `for`), P728 (short-circuit `and`/`or` + `join` em code block), P729 (`join` entre iterações).
  - **Repr e render:** P721 (`repr` da linguagem), P726 (`fill/stroke: none`), P727 (fallback de stroke default em `curve`).
- Achados que permanecem abertos (não-bloqueantes para o cetz): ver `achados-adiados-cetz.md` — namespaces `emoji`/`pdf` ausentes; `color`/`gradient` e `counter`/`state` com tipo divergente do vanilla; e mais cinco itens de baixa prioridade sem consumidor no cetz.

## Campos fixos cetz (estado após P731 — FINAL)

- `line` + `circle` no canvas com `import cetz.draw: *`: **compila (exit 0) e renderiza ambos** — diff 0.1419% (anti-aliasing).
- `circle` sozinho: compila (mantido desde P727).
- `#import cetz.draw: *` top-level: compila (mantido).

## Critério de fecho do passo

- [x] Sonda ampla completa, todos os namespaces confirmados (tabela vanilla vs cristalino).
- [x] Implementado e testado, incluindo `#import` (items e wildcard) e field access directo sem regressão.
- [x] Sem regressão em `cargo test --workspace` (4722 passed, 0 failed), com atenção redobrada dado o alcance — nenhuma falha.
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — **diff de pixels final registado: 0.1419% (anti-aliasing); cadeia P678-731 fechada**.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p731.md`, com hash do commit.
- [x] Item marcado como fechado em `achados-adiados-cetz.md` (e três novos achados registados: `emoji`/`pdf`, `color`/`gradient`, `counter`/`state`).
- [x] Resumo completo da cadeia: 54 passos, categorias acima.
