# Diagnóstico — Eliminação `CounterStateLegacy` (P190A)

**Data**: 2026-05-04
**Marco arquitectural relevante**: M5 universal completo (P200B); início de M6.
**Magnitude diagnóstico**: S-M (mais que passos anteriores — inventário extenso).
**Estado empírico**: confirmado por leitura directa de `01_core/src/`.

---

## §1 Validação estado actual

### §1.1 Workspace baseline

- Tests: **1.869 verdes**.
- Linter: **zero violations**.
- M5 universal completo (per P200 consolidado §10).
- Hash L0 introspect: `7a3ba2b7`.
- Hash código introspect: `8e0128e4`.

### §1.2 `CounterStateLegacy` ficheiro empírico

`01_core/src/entities/counter_state_legacy.rs` — **330 linhas** (per F1 estimativa confirmada).

---

## §2 Inventário 14+2 campos × consumer × cobertura Introspector

### §2.1 Campos privados (2)

| # | Campo | Type | Acessado via | Cobertura Introspector |
|---|-------|------|--------------|------------------------|
| 1 | `hierarchical` | `HashMap<String, Vec<usize>>` | métodos `step_hierarchical`, `format_hierarchical` | ✅ `intr.counters` (`apply_hierarchical_at`, `formatted_counter_at`) |
| 2 | `flat` | `HashMap<String, usize>` | métodos `step_flat`, `update_flat`, `get_flat`, `display_value` | ✅ `intr.counters` (`apply_at`, `flat_counter_at`) |

**Cobertura**: 100% via `CounterRegistry` (P184B + P184C + P198C).

### §2.2 Campos públicos (14)

| # | Campo | Type | Cobertura Introspector | Categoria | Walk arm muta? |
|---|-------|------|------------------------|-----------|---------------|
| 1 | `numbering_active` | `HashMap<String, bool>` | ✅ `intr.state` (StateRegistry P171/P182C) | Numbering active | Sim — SetHeadingNumbering, SetEquationNumbering, **Equation walk arm para gate** |
| 2 | `resolved_labels` | `HashMap<Label, String>` | ✅ `intr.resolved_labels` (P193B + P195D + P196B) | Labels & TOC | Sim — Heading + Labelled |
| 3 | `headings_for_toc` | `Vec<(Label, Content, usize)>` | ✅ `intr.headings_for_toc` (P200B) | Labels & TOC | Sim — Heading mut 4 |
| 4 | `auto_label_counter` | `usize` | ⚠️ **sem sub-store dedicado** — counter de sintetização label sequencial; pode ser local ao walk | Labels & TOC | Sim — Heading mut 2 |
| 5 | `label_pages` | `HashMap<Label, usize>` | ❌ **sem sub-store** — populated por Layouter (page tracking durante render); **não é responsabilidade de Introspector pre-pass** | Page tracking | Não (mutado por Layouter) |
| 6 | `known_page_numbers` | `HashMap<Label, usize>` | ❌ **sem sub-store** — populated por Layouter Pass 3 (final) per DEBT-12 | Page tracking | Não (mutado por Layouter) |
| 7 | `has_outline` | `bool` | ✅ `intr.kind_index.contains_key(&ElementKind::Outline)` (P189B substituição) | Document metadata | Não (P189B removeu mutação) |
| 8 | `is_readonly` | `bool` | ❌ **sem sub-store** — flag Layouter runtime (DEBT-13 outline freeze); **não é state introspecção** | Document metadata (Layouter runtime) | Não (Layouter set/unset durante outline render) |
| 9 | `figure_numbers` | `HashMap<String, Vec<usize>>` | ✅ via `intr.counters` chave `figure:{kind}` (P184B + figure_number_at_index P184C) | Figures | Sim — Figure walk arm |
| 10 | `figure_label_numbers` | `HashMap<Label, usize>` | ✅ `intr.figure_label_numbers` (P168) | Figures | Sim — Labelled walk arm |
| 11 | `local_figure_counters` | `HashMap<String, usize>` | ⚠️ **walk-internal** — usado por compute_figure helper; sem consumer downstream (per P197B §6) | Figures | Sim — Figure walk arm |
| 12 | `lang` | `Option<Lang>` | ❌ **sem sub-store** — config field (set via `Document.set_lang`); não é state derivado de Content | Document metadata | Não (set externamente) |
| 13 | `bib_entries` | `Vec<BibEntry>` | ✅ `intr.bib_store` (BibStore P181E) | Bibliography | Não (P181H restaurou puro) |
| 14 | `bib_numbers` | `HashMap<String, u32>` | ✅ `intr.bib_store` (P181E `assign_number`) | Bibliography | Não (P181H restaurou puro) |

### §2.3 Categorias resumidas

| Categoria | Campos | Cobertura |
|-----------|--------|-----------|
| **Counters core** (privados) | `hierarchical`, `flat` | ✅ 100% via CounterRegistry |
| **Numbering active** | `numbering_active` | ✅ via StateRegistry |
| **Labels & TOC** | `resolved_labels`, `headings_for_toc`, `auto_label_counter` | ✅ 2/3 (auto_label_counter é local-walk) |
| **Page tracking** | `label_pages`, `known_page_numbers` | ❌ 0/2 (Layouter runtime — não pre-pass) |
| **Document metadata** | `has_outline`, `is_readonly`, `lang` | ⚠️ has_outline ✅; is_readonly + lang fora escopo Introspector |
| **Figures** | `figure_numbers`, `figure_label_numbers`, `local_figure_counters` | ✅ 2/3 (local_figure_counters walk-internal) |
| **Bibliography** | `bib_entries`, `bib_numbers` | ✅ via BibStore |

**Achado crítico**: nem todos os 16 campos têm cobertura Introspector — alguns são genuinamente Layouter-runtime (`label_pages`, `known_page_numbers`, `is_readonly`) ou config (`lang`). **Eliminação total `CounterStateLegacy` exige ou migrar estes campos para outro lugar (Layouter struct dedicada) ou aceitar struct reduzida persistente**.

---

## §3 Inventário 8 métodos `CounterStateLegacy`

| # | Método | Função | Substituível por | Caller(s) |
|---|--------|--------|------------------|-----------|
| 1 | `new()` | Construtor `Default` | n/a (eliminado com struct) | Layouter init, walk init |
| 2 | `is_numbering_active(key) -> bool` | Lê `numbering_active.get(key)` | `intr.is_numbering_active(...)` (P185B) | walk arm Equation, compute_heading_auto_toc, Layouter equation/heading/figure |
| 3 | `step_hierarchical(key, level)` | Avança hierarchical counter | `intr.counters.apply_hierarchical_at(...)` (interno from_tags) | walk arm Heading, walk arm CounterUpdate |
| 4 | `format_hierarchical(key) -> Option<String>` | Formata counter como "1.2.3" | `intr.formatted_counter(...)` (P170) ou `formatted_counter_at(...)` (P185B) | compute_heading_auto_toc, Layouter heading |
| 5 | `step_flat(key)` | Avança flat counter | `intr.counters.apply_at(Step, ...)` (interno from_tags) | walk arm CounterUpdate, walk arm Equation |
| 6 | `update_flat(key, value)` | Update flat counter | `intr.counters.apply_at(Update(val), ...)` | walk arm CounterUpdate |
| 7 | `get_flat(key) -> usize` | Lê flat counter | `intr.flat_counter_at(...)` (P185B) | compute_labelled, Layouter equation |
| 8 | `display_value(kind) -> String` | Lê valor para CounterDisplay | wrapper sobre `formatted_counter` ou similar | materialize_time CounterDisplay arm |

**Cobertura**: 100% dos métodos têm equivalente Introspector via 19 trait methods existentes (após P200B = 20 métodos).

---

## §4 Inventário 4 helpers `compute_*` família ADR-0069

| Helper | Localização | Campos legacy lidos | Substituível por |
|--------|-------------|---------------------|------------------|
| `compute_labelled` (P195D) | `introspect.rs:325-368` | `state.flat["equation"]` (via `state.get_flat`); `state.figure_numbers["{kind}"]` (via `state.figure_numbers.get`); `state.lang` | `intr.flat_counter_at("equation", loc)` (P185B); `intr.counters.value_at_index("figure:{kind}", idx)` (P184C); `state.lang` permanece (config field) |
| `compute_heading_auto_toc` (P196B) | `introspect.rs:380-394` | `state.is_numbering_active("heading")`; `state.format_hierarchical("heading")` | `intr.is_numbering_active("numbering_active:heading")` (P185B); `intr.formatted_counter("heading")` (P170) |
| `compute_figure` (P197B) | `introspect.rs:413-427` | `state.local_figure_counters.get(kind)` | **walk-internal** (per P197B §6); pode ser eliminado se walk arm Figure for purificado |
| `compute_heading_for_toc` (P200B) | `introspect.rs:431-453` | `state.auto_label_counter` | **walk-internal** — auto_label_counter é local; pode ser eliminado se walk arm Heading purificado |

**Achado**: 2 helpers (`compute_figure`, `compute_heading_for_toc`) leem campos walk-internal — eliminação directa pós-purificação walk arm. 2 helpers (`compute_labelled`, `compute_heading_auto_toc`) leem state cobertos por Introspector — migração para `intr.X_at(...)` location-aware.

---

## §5 Inventário walk arms × mutações legacy

| Walk arm | # mutações | Linhas | Estado pós-M5 |
|----------|-----------|--------|---------------|
| Heading | 4 | 507, 509, 514, 520 | Tags emitidas (P196B + P200B); mutações preservadas como write paralelo |
| Figure | 2 | 606, 609 | Tags via extract_payload + from_tags (P184B + P197B); mutações preservadas |
| SetHeadingNumbering | 1 | 624 | Tag via extract_payload + from_tags (P182C + P198B); mutação preservada |
| SetEquationNumbering | 1 | 645 | Tag via extract_payload + from_tags (P199B); mutação preservada |
| CounterUpdate | 3 | 650-661 | Tag via extract_payload + from_tags (P198C); mutações preservadas |
| Labelled | 2 | 639, 642 | Tag pós-recursão (P195D); mutações preservadas |
| Equation | 1 | 580 | Tag via extract_payload (P186C/D/E); mutação preservada (gate ao counter step) |

**Total**: ~14 mutações legacy a eliminar em walk arms.

**Cláusula gate substancial**: walk arms ainda referenciados por `compute_*` helpers (que leem state). Eliminação simultânea ou sequenciamento cuidado obrigatório.

---

## §6 Inventário Layouter consumers `self.counter.X`

`grep -rnE "self\.counter\." 01_core/src/rules/layout/` retornou **10 ocorrências** em 2 ficheiros:

| Ficheiro:linha | Acesso | Substitutível por |
|----------------|--------|-------------------|
| `equation.rs:33` | `self.counter.is_numbering_active("equation")` (fallback) | `self.introspector.is_numbering_active_at("numbering_active:equation", loc)` |
| `equation.rs:35` | `self.counter.step_flat("equation")` (mutação Layouter) | n/a — Layouter mutação a eliminar (deriva counter de Tag emit no walk) |
| `equation.rs:109` | `self.counter.get_flat("equation")` (read) | `self.introspector.flat_counter_at("equation", loc)` |
| `mod.rs:328` | `self.counter.step_hierarchical("heading", *level)` (mutação) | n/a — Layouter mutação a eliminar |
| `mod.rs:343` | `self.counter.is_numbering_active("heading")` (fallback) | `self.introspector.is_numbering_active_at(...)` |
| `mod.rs:356` | `self.counter.format_hierarchical("heading")` (fallback) | `self.introspector.formatted_counter_at("heading", loc)` |
| `mod.rs:499` | `self.counter.figure_numbers.get(...)` (fallback) | `self.introspector.figure_number_at_index(...)` |
| `mod.rs:665` | `self.counter.bib_entries.iter().find(...)` (fallback) | `self.introspector.bib_store.lookup(...)` |
| `mod.rs:673` | `self.counter.bib_numbers.get(...)` (fallback) | `self.introspector.bib_store.assigned_number(...)` |
| `mod.rs:1136` | `doc.extracted_label_pages = self.counter.label_pages` (page tracking) | **manter** ou migrar para sub-store dedicado se necessário |

**Achado crítico**: Layouter contém **mutações próprias** ao counter (`equation.rs:35`, `mod.rs:328`) — não apenas reads. Eliminação requer ou:
- Migrar mutações Layouter para read-only via Introspector (mais limpo).
- Manter Layouter `counter` como struct local-Layouter durante render (subset de campos).

---

## §7 Inventário tests dependentes

`grep -rE "use.*CounterStateLegacy|: CounterStateLegacy|CounterStateLegacy::"` em `01_core/src/`:
- **counter_state_legacy.rs próprio**: 17 tests internos (preservar como tests do struct legado).
- **layout/counters.rs:7**: import + uso interno do helper.
- **introspect.rs:83**: `walk` fn requer state legacy.
- **introspect.rs:1146**: tests internos.

`grep -nE "layouter\.counter\." 01_core/src/` (tests externos via Layouter):
- Referenciado em testes Layouter (~15+ tests).

**Estimativa**: ~50-80 tests dependentes directa ou indirectamente. Adaptação necessária quando fields forem eliminados.

---

## §8 Decisões cláusula 1–9

### Cláusula 1 — Estratégia de eliminação

- **Decisão**: **β — incremental por categoria**.
- 6 categorias confirmadas em §2.3.
- Magnitude: 6-8 sub-passos × M; total agregado L cross-modular.
- Big-bang (γ) descartado — risco substancial em 16 fields + 8 methods + 4 helpers + Layouter migration.
- Per-field (α) descartado — sub-passos demasiado pequenos; perde coerência arquitectural.

### Cláusula 2 — Ordem das categorias (dependências cruzadas)

- **Decisão** (ordem de execução):
  1. **Bibliography** (`bib_entries`, `bib_numbers`) — caminho Introspector activo desde P181E; **sem mutação walk** (P181H restaurou puro); **sem helper compute_* lê**; menor risco.
  2. **Page tracking** (`label_pages`, `known_page_numbers`) — Layouter-only fields; **mover para struct Layouter dedicada** (ex: `LayouterRuntime` ou similar); sem migração Introspector.
  3. **Document metadata** (`has_outline`, `is_readonly`, `lang`) — `has_outline` substituível por `intr.kind_index`; `is_readonly` + `lang` movem para struct Layouter.
  4. **Numbering active** — substituível por `intr.is_numbering_active_at`; eliminar mutações em walk arm SetHeadingNumbering, SetEquationNumbering, Equation.
  5. **Counters core** + helpers `compute_labelled` + `compute_heading_auto_toc` migration — substituir leituras por `intr.flat_counter_at` / `formatted_counter_at` location-aware.
  6. **Labels & TOC** — substituir leituras por `intr.resolved_labels`/`headings_for_toc`/`figure_label_numbers`; eliminar `compute_heading_for_toc` (walk-internal); manter `auto_label_counter` como local walk fn (não state).
  7. **Figures** — substituir consumer C3 fallback; eliminar `compute_figure` (walk-internal); eliminar `local_figure_counters`.
  8. **Walk arms purification** — eliminar mutações legacy em todos arms.
  9. **Layouter migration final** — substituir todas as 10 ocorrências `self.counter.X` por Introspector path; eliminar field `counter` do Layouter.
  10. **Eliminação struct + L0 cleanup**.

### Cláusula 3 — Forma final do struct

- **Decisão**: **α — eliminação total** com excepção pragmática.
- `CounterStateLegacy` deixa de existir como struct unificada.
- Campos genuinamente Layouter-runtime (`label_pages`, `known_page_numbers`, `is_readonly`, `lang`) movem para struct Layouter dedicada (ex: `LayouterRuntimeState`).
- Restantes 12 campos têm cobertura Introspector — eliminados.
- 8 métodos: 7 substituídos por trait methods Introspector; 1 (`new()`) eliminado com struct.

### Cláusula 4 — API pública

- **Decisão**: **API interna** — `pub mod counter_state_legacy;` em `entities/mod.rs:32` é apenas dentro do crate `typst-core`.
- Sem re-export em `lib.rs`.
- Crystalline tem `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/` — verificar se outros crates dependem.
- **Eliminação livre** — não é breaking change para consumers externos.

### Cláusula 5 — 4 helpers `compute_*` família ADR-0069

- **Decisão**:
  - `compute_labelled` (P195D): **migrar** para Introspector path location-aware.
  - `compute_heading_auto_toc` (P196B): **migrar** para Introspector path location-aware.
  - `compute_figure` (P197B): **eliminar** (walk-internal; sem consumer downstream per P197B §6).
  - `compute_heading_for_toc` (P200B): **eliminar** (walk-internal; auto_label_counter é local).
- 2 helpers eliminados, 2 helpers migrados.

### Cláusula 6 — Layouter dependências

- **Decisão**: Layouter migra `counter` field para read-only via Introspector path completo. **Mutações próprias do Layouter (`equation.rs:35`, `mod.rs:328`) eliminadas** — counter já é populado pelo walk pre-pass; Layouter apenas lê.
- Field `counter` substituído por `runtime: LayouterRuntimeState` (subset com `label_pages`, `known_page_numbers`, `is_readonly`, `lang`).
- Magnitude significativa — 10 ocorrências `self.counter.X` migradas; Layouter struct refactored.

### Cláusula 7 — Walk arms

- **Decisão**: eliminar **14 mutações legacy** em walk arms migrados em M5.
- Walk torna-se **puro** — apenas emite Tags; não muta state.
- Sequenciamento crítico: cada arm purificado **após** consumer migration concluir.

### Cláusula 8 — Tests workspace

- **Decisão**: padrão pragmático auditor #1.
- Tests sentinela legacy (E1-E6 + E2-residuo + walk_arm_*_le_X_legacy) **preservar como histórico** (testes adaptados para usar Introspector path) ou **remover** se redundantes com tests pós-M6.
- Tests Layouter adaptados conforme `self.counter.X` for substituído.
- Tests internos de `CounterStateLegacy` removidos quando struct for eliminado.

### Cláusula 9 — Critério de fecho de M6

- **Decisão**:
  - `grep -rn "CounterStateLegacy" 01_core/src/` retorna **zero** (excepto histórico em comentários).
  - 4 helpers eliminados (2) ou migrados (2).
  - Walk arms puros (14 mutações eliminadas).
  - Layouter sem field `counter` (substituído por `runtime` ou similar).
  - Tests workspace verdes.
  - F1 fechado (`CounterStateLegacy` eliminado).
  - F3 parcialmente fechado (Layouter perde 1 field; resta lidar com outros 19 → 18).

---

## §9 ADR + DEBT avaliação

### ADR

- **Decisão**: **ADR-0070 PROPOSTO** será criado.
- Justificação: eliminação de struct de 16 fields + 8 methods + 4 helper migrations + Layouter refactor é decisão arquitectural maior. Replica precedente ADR-0068 (PROPOSTO em P185A; ACEITE depois). Pattern stylesheet é eliminação de write paralelo M5; não nova variante operacional ADR-0069.
- ADR-0070 estrutura (proposta):
  - **Título**: "Eliminação `CounterStateLegacy` — fim de M5 universal".
  - **Estado**: PROPOSTO (P190A); ACEITE quando P190 série fechar.
  - **Contexto**: M5 universal completo P200B; write paralelo M5 ainda activo; struct legado de 16 fields + 8 methods.
  - **Decisão**: eliminação total via 6 categorias incrementais; pattern stylesheet "eliminação write paralelo".
  - **Consequências**: F1 fecha; F3 parcialmente fecha; Layouter migrado para Introspector path completo; walk torna-se puro.
  - **Alternativas avaliadas**: incremental por field (descartada — sub-passos demasiado pequenos); big-bang (descartada — risco substancial); façade temporário (descartada — adia problema); rename (descartada — engana).

### DEBT

- **DEBT M6 documentação** (P200C §8) **fecha por execução em P190 série**.
- F1 (`CounterStateLegacy`) fecha após P190.
- F3 (Layouter 19 fields) parcialmente fecha — 1 field eliminado.
- Sem DEBT formal aberto.

---

## §10 Plano de sub-passos (β — incremental por categoria)

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| **P190B** | Categoria Bibliography (eliminar `bib_entries`, `bib_numbers` + Layouter migration mod.rs:665, 673) | M |
| **P190C** | Categoria Page tracking (mover `label_pages`, `known_page_numbers` para `LayouterRuntimeState` dedicada) | M |
| **P190D** | Categoria Document metadata (eliminar `has_outline` via `kind_index`; mover `is_readonly` + `lang` para `LayouterRuntimeState`) | M |
| **P190E** | Categoria Numbering active (eliminar `numbering_active`; migrar consumers Layouter equation.rs + mod.rs:343 + walk arm Equation gate; eliminar mutações walk arms SetHeading/SetEquation) | M |
| **P190F** | Categoria Counters core + helpers `compute_labelled` + `compute_heading_auto_toc` (migrar para Introspector path location-aware; eliminar `flat`, `hierarchical`; eliminar 6 dos 8 métodos) | M+ |
| **P190G** | Categoria Labels & TOC (eliminar `resolved_labels`, `headings_for_toc`, `auto_label_counter`; eliminar `compute_heading_for_toc`; migrar consumer outline) | M |
| **P190H** | Categoria Figures (eliminar `figure_numbers`, `figure_label_numbers`, `local_figure_counters`; eliminar `compute_figure`; migrar Layouter consumer mod.rs:499) | M |
| **P190I** | Walk arms purification — eliminar 14 mutações legacy; Layouter migration final; eliminar field `counter`; eliminar struct `CounterStateLegacy` + 8 métodos restantes; L0 cleanup; relatório consolidado P190 + ADR-0070 ACEITE | M+ |

**Total**: 8 sub-passos B-I × M = **L cross-modular agregado**.

---

## §11 Regra dos 2 eixos por campo

| Campo | Eixo 1 (consumer durante walk?) | Eixo 2 (sub-store activo?) | Ordem migração |
|-------|--------------------------------|----------------------------|----------------|
| bib_entries, bib_numbers | Não | ✅ BibStore | 1ª (mais fácil) |
| label_pages, known_page_numbers | Layouter durante render | ❌ Layouter-only | 2ª (mover, não eliminar) |
| has_outline | Não | ✅ kind_index | 3ª |
| is_readonly, lang | Layouter runtime | ❌ Layouter-only | 3ª (mover) |
| numbering_active | walk arm Equation; helpers | ✅ StateRegistry | 4ª (chained com helpers) |
| flat, hierarchical | helpers compute_* | ✅ CounterRegistry | 5ª (chained com helpers) |
| resolved_labels | walk arm Heading + Labelled | ✅ ResolvedLabelStore | 6ª |
| headings_for_toc | walk arm Heading; outline | ✅ headings_for_toc store | 6ª |
| auto_label_counter | walk arm Heading | ❌ walk-internal (local) | 6ª (eliminar) |
| figure_numbers | helper compute_labelled; Layouter | ✅ CounterRegistry chave figure:{kind} | 7ª |
| figure_label_numbers | walk arm Labelled | ✅ figure_label_numbers store | 7ª |
| local_figure_counters | helper compute_figure | ❌ walk-internal | 7ª (eliminar) |

---

## §12 Próximo sub-passo (P190B com escopo concreto)

**P190B — Categoria Bibliography**:

1. Confirmar empíricamente:
   - Walk arm Bibliography puro desde P181H (sem mutações legacy).
   - Consumer Layouter `mod.rs:665, 673` faz fallback `self.counter.bib_entries`/`bib_numbers`.
   - Trait `Introspector` tem `bib_store: BibStore` field; método para query.

2. Migrar Layouter consumer:
   - `mod.rs:665`: `self.counter.bib_entries.iter().find(...)` → `self.introspector.bib_store.lookup(...)` ou método equivalente.
   - `mod.rs:673`: `self.counter.bib_numbers.get(...)` → `self.introspector.bib_store.assigned_number(...)`.

3. Eliminar fields `bib_entries` + `bib_numbers` de `CounterStateLegacy`.

4. Adaptar tests Layouter dependentes.

5. L0 actualizado (`entities/counter_state_legacy.md` se existir; `rules/layout/*.md`).

6. Tests workspace verdes.

7. Hash actualizado via `crystalline-lint --fix-hashes`.

**Critério de fecho P190B**: tests workspace 1869 verdes; lint zero violations; struct CounterStateLegacy reduzido a 14 fields (era 16).

Magnitude **M** (mais simples categoria — caminho Introspector activo desde P181H; consumer migration directa).
