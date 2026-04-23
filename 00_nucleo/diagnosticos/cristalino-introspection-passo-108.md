# Passo 108.B — Inventário do cristalino (introspecção)

**Data**: 2026-04-23
**Propósito**: identificar o que já existe em `01_core/` com nomes
ou semântica próxima do `Introspection` do vanilla; registar API,
relações, estado (stub / parcial / completo) e DEBTs vivos.

---

## Parte 1 — O que já existe

### `CounterState` (`entities/counter_state.rs:30`)

- **Estado**: **implementação funcional, divergente do vanilla**.
- **Linhas**: ~290 (com testes). ~14 campos, ~10 métodos.
- **Role**: agrega tudo aquilo que o vanilla distribui por
  `Counter`, `CounterKey`, `Location`, `labels`, `queries`,
  `Introspector::query_count_before`, etc., numa única struct plana.
- **Campos**:

| Campo | Tipo | Propósito |
|-------|------|-----------|
| `hierarchical` | `HashMap<String, Vec<usize>>` | Contadores hierárquicos (heading). |
| `flat` | `HashMap<String, usize>` | Contadores planos (equation, figure, …). |
| `numbering_active` | `HashMap<String, bool>` | Flags `#set heading(numbering: ...)`. |
| `resolved_labels` | `HashMap<Label, String>` | Texto resolvido por label ("Secção 1.1"). |
| `headings_for_toc` | `Vec<(Label, Content, usize)>` | Catálogo para TOC; Content preserva formatação. |
| `auto_label_counter` | `usize` | Gerador de IDs `auto-toc-N`. |
| `label_pages` | `HashMap<Label, usize>` | Escrito por `references.rs`; label → página. |
| `known_page_numbers` | `HashMap<Label, usize>` | Lido por `outline.rs`; injectado entre iterações. |
| `has_outline` | `bool` | Sinal para o fixpoint. |
| `is_readonly` | `bool` | Bloqueia step/update na renderização da TOC. |
| `figure_numbers` | `HashMap<String, Vec<usize>>` | Números 1-based por kind de figura. |
| `figure_label_numbers` | `HashMap<Label, usize>` | Label → número de figura. |
| `local_figure_counters` | `HashMap<String, usize>` | Auxiliar interno da introspecção. |

- **Métodos públicos**:
  - `new() / default()`
  - `is_numbering_active(key)`
  - `step_hierarchical(key, level)` — respeita `is_readonly`.
  - `format_hierarchical(key) -> Option<String>` ("1.2.3").
  - `step_flat(key)` / `update_flat(key, value)` / `get_flat(key)`.
  - `display_value(kind)` — unifica leitura hierárquico/plano.
- **Enum apoio**: `CounterAction { Step, Update(usize) }`.
- **Relação com vanilla**: é **fusão** de `Counter + Location +
  labels + queries` num único struct; sem `Selector`, sem
  `Tracked`, sem convergência.
- **Divergência intencional**: documentada no header do ficheiro
  ("Cristalino diverge do Typst original aqui: o original resolve
  contadores em duas passagens com `comemo` ... Esta implementação
  usa uma única passagem...").

### `rules/introspect.rs` (689 linhas)

- **Estado**: **implementação funcional** mas em L1 directo, sem
  `Location`, sem `Tag`, sem multi-pass.
- **API pública**:
  - `pub fn introspect(content: &Content) -> CounterState`:
    percorre `Content` uma vez, popula `CounterState`.
- **Helpers privados**:
  - `walk(content, state)`: efeitos colaterais em `state` na ordem
    de travessia do Layouter (contadores, labels, figure_numbers,
    has_outline).
  - `materialize_time(content, state) -> Content`: congela AST
    substituindo `CounterDisplay` por `Content::text(display_value)`
    no momento da introspecção (DEBT-18 resolvido).
- **Call sites**:
  - `03_infra` (L3 orquestrador): chama `introspect()` antes de
    `layout()`. Passa o state resultante para `layout()` como
    estado inicial (só `resolved_labels`).
  - `layout()` L1 — usa fixpoint interno com `known_page_numbers`
    injectado (DEBT-17 resolvido).
- **Relação com vanilla**:
  - Combina responsabilidades de: `Introspector::build`,
    `CounterState` (sequences), `references` (labels), `outline`
    (has_outline), parte do `Locator`.
  - **Não tem**: `Tag` discovery (trabalha com Content directo),
    `comemo` tracking, multi-pass fixpoint completo (tem sim
    fixpoint de páginas dentro de `layout()`).

### `Content::Heading`, `SetHeadingNumbering`, `SetFigureNumbering`,
### `CounterUpdate`, `CounterDisplay`, `Outline`, `Labelled`, `Ref`

Todas variantes de `Content` (enum fechado, ADR-0026):

- **`Content::Heading { level, body }`** — heading numerado pelo
  contador hierárquico.
- **`Content::SetHeadingNumbering { active }`** — efeito de
  `#set heading(numbering: "1.1")`.
- **`Content::SetFigureNumbering { pattern }`** — efeito de
  `#set figure(numbering: "1")`.
- **`Content::CounterUpdate { key, action }`** — efeito de
  `counter(x).step()` ou `counter(x).update(n)`.
- **`Content::CounterDisplay { kind }`** — placeholder substituído
  por texto durante `materialize_time` ou `layout`.
- **`Content::Outline`** — placeholder da TOC.
- **`Content::Labelled { target, label }`** — associação retroactiva
  `elemento <label>`.
- **`Content::Ref { target: Label }`** — referência `@label`.
- **Relação com vanilla**: no vanilla, todos estes seriam elementos
  com `Location` (`Locatable`), descobertos via `Tag` durante layout
  e consumidos via `Introspector::query`. No cristalino, são
  variantes do enum `Content` consumidas directamente por
  `introspect::walk` e `layout`.

### `bindings::eval_counter_method` (`rules/eval/bindings.rs:93`)

- `extract_counter_key(expr)` detecta `counter("key")` como
  callee; `eval_counter_method(key, method, args, ...)` produz
  `Content::CounterUpdate`/`CounterDisplay` consoante o método
  (`step`, `update`, `get`, `display`).
- **Estado**: **parcial** — `step/update` produzem nós reais;
  `get/display` caem no fallback `Content::CounterDisplay { kind }`
  (lê o valor actual, não é "pergunta em tempo de introspecção").
- **Relação com vanilla**: no vanilla, `counter.get()` dispara
  `engine.introspect::<CounterAtIntrospection>(...)` que resolve
  via `Introspector::query_count_before` com acesso à `Location`.
  No cristalino, `Content::CounterDisplay` é resolvido lazy no
  walk seguinte.

### Outros sinais de introspecção em L1

Grep por `Location|Counter|Query` em `01_core/src/` encontra 8
ficheiros:

- `entities/counter_state.rs` — já coberto.
- `rules/introspect.rs` — já coberto.
- `rules/layout/mod.rs` — consome `CounterState`; fixpoint.
- `rules/layout/counters.rs` — arm `CounterUpdate`/`Display`.
- `rules/layout/references.rs` — arm `Labelled`/`Ref`.
- `rules/layout/outline.rs` — arm `Outline`.
- `rules/layout/tests.rs` — testes (inclui variantes da máquina).
- `rules/eval/bindings.rs` — `extract_counter_key` / counter method.
- `rules/eval/mod.rs` — dispatcher (`Expr::Ref`, `Expr::Label`).
- `entities/world_types.rs` — **não** tem Introspector/Location.

**Nenhum ficheiro** chamado `location.rs`, `introspector.rs`,
`query.rs`, `tag.rs`, `locator.rs`, `state.rs` existe em L1. O
conceito "Introspection como subsistema" está implementado como
`walk` funcional sobre `CounterState`.

---

## Parte 2 — DEBTs relacionados

### DEBT-10 — Contadores em duas passagens — **RESOLVIDO (Passo 62)**

- Resolvido: `introspect()` é a primeira passagem; `layout()`
  recebe `CounterState` com `resolved_labels` populado; forward
  refs funcionam.
- **Estado actual**: encerrado. `rules/introspect.rs` existe e é
  consumido por L3.

### DEBT-14 — `#set figure(numbering: ...)` — **ENCERRADO (Passo 75)**
### DEBT-15 — `kind` hardcoded em `Content::Figure` — **ENCERRADO (Passo 75)**

- Histórico.

### DEBT-17 — Fixpoint da TOC — **RESOLVIDO (Passo 65)**

- Resolvido: fixpoint interno no `layout()` com
  `known_page_numbers` injectado.

### DEBT-18 — Contexto temporal na TOC — **RESOLVIDO (Passo 66)**

- Resolvido via `materialize_time`.

### DEBT-45 — `check_*_depth` não chamados — EM ABERTO

- Relação **indirecta**: quando Introspection completa for
  materializada, `check_layout_depth`/`check_call_depth` ganham
  contextos novos. Hoje, estão livres de dependência de
  Introspection.

### DEBT-1 — StyleChain (resíduos)

- Campo "Propriedades adicionais... `counter(heading).at(here())`"
  referenciado. Sem `here()` e `Location`, este tipo de expressão
  não pode ser avaliada. **Relação**: Introspection destrava parte
  desta pendência.

### DEBTs ainda abertos que tocam Introspection

Grep + leitura rápida do DEBT.md identifica:

- **DEBT-2** — ~closures com captura~ (não relacionado).
- **DEBT-8 / DEBT-9 / DEBT-33 / DEBT-34d / DEBT-34e / DEBT-35b /
  DEBT-42 / DEBT-43 / DEBT-45 / DEBT-50** — (maioria não
  relacionada com Introspection; ver 108.C para o subconjunto
  que Introspection destravaria).

### DEBTs **novos** que Introspection completa criaria

- Para `counter(x).at(here())`: requer `here()` function + `Location`
  + introspector acessível a partir do eval.
- Para `query(heading)`: requer `Selector` materializado +
  `Introspector::query`.
- Para `@label` resolvido em tempo de eval (não em tempo de
  layout): requer `Location` identificável antes do layout.

---

## Parte 3 — Consumidores no cristalino

`rules/layout/` consome `CounterState` nos seguintes pontos:

- **`layout/mod.rs`** (orquestrador): 22 referências. Gere
  `Layouter` com `CounterState` injectado; roda fixpoint quando
  `has_outline`.
- **`layout/counters.rs`** (arm `CounterUpdate` / `CounterDisplay`):
  8 referências.
- **`layout/references.rs`** (arm `Labelled` / `Ref`): 3
  referências. Escreve `label_pages` durante o layout.
- **`layout/outline.rs`** (arm `Outline`): 5 referências. Lê
  `known_page_numbers` e `headings_for_toc`.
- **`layout/tests.rs`**: 90 referências (testes exercitam todas as
  variantes).

`rules/introspect.rs` é o **produtor** central; tudo o resto é
consumidor.

### Divergências face ao vanilla (sem "correcção" em análise)

1. **Single-pass vs multi-pass**: o cristalino faz uma única
   passagem `introspect()`, depois `layout()` com fixpoint limitado
   a páginas (`known_page_numbers`). O vanilla faz **6 iterações**
   de eval+layout+introspect com detecção de fixpoint por hash.
2. **Sem `Location`**: o cristalino identifica elementos pela
   combinação `(Label, Content)` ou por chave textual. O vanilla
   usa `Location(u128)` gerada por `Locator` em layout.
3. **Sem `Selector`**: o cristalino não suporta queries
   estruturadas (`query(heading.where(level: 1))`). `show rule`
   tem `Selector::NodeKind(...)` mas vive em `show.rs`, não é
   `Introspector::query`.
4. **Sem `comemo` em Introspection**: o `CounterState` é um
   `HashMap` plano; cada pass reconstrói do zero. O vanilla faz
   memoização com `comemo` em `sequence()` de contador.
5. **`CounterDisplay` materializado**: o cristalino resolve em
   `materialize_time` ou em `layout`; o vanilla faz lookup via
   engine.introspect.
6. **Elementos introspectáveis são variantes de Content**: não há
   `Locatable` trait. Adicionar um novo elemento introspectável
   requer nova variante + braço em `introspect::walk` + braço em
   `layout/`.

---

## Observações

1. **Escala do cristalino actual**: ~690 linhas em `introspect.rs`
   + ~290 em `counter_state.rs` = **980 linhas** de máquina funcional.
   É substancialmente mais do que "stub" e faz parte do caminho
   crítico do pipeline L1 → L3.
2. **Divergência é de design, não é "work in progress"**: ADR-0033
   estabelece paridade funcional, não estrutural. Materializar
   `Introspection` no cristalino não é transcrever o vanilla — é
   decidir quais conceitos vanilla resolvem problemas reais do
   cristalino, e absorvê-los.
3. **Existe valor para o utilizador sem Introspection completa**:
   `counter(heading).step()`, `heading`s numerados, figuras com
   caption, TOC — tudo já funciona com a máquina actual.
4. **O que não funciona hoje**: `counter(x).at(loc)`, `query(...)`,
   `locate(x)`, `here()`. Estes são os candidatos naturais a primeira
   materialização de Introspection verdadeira.
5. **Assumir que "Introspection não existe" é incorrecto**. Existe
   uma versão simplificada, funcional, documentada como divergência
   intencional. A decisão 108.D é **qual componente do vanilla traz
   valor incremental** face ao que já existe.
