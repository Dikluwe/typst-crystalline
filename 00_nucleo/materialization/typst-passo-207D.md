# Passo 207D — Page-aware trait methods

**Série**: 207 (sub-passo `D`).
**Marco**: M9c (Bloco II trait page-aware + Bloco VIII
infraestrutura parcial).
**Tipo**: implementação cross-modular (sub-store novo
ou extensão + 4 trait methods).
**Magnitude**: M-L (~5-6h) — refina-se com C1
diagnóstico.
**Pré-condição**: P207C concluído; trait 22 métodos;
`LabelRegistry` multi-label; tests 1885 verdes; 0
violations; ADR-0076 PROPOSTO anotado §P207C ✅ MATERIALIZADO.
**Output**: 1 ficheiro (relatório curto).

---

## §1 Trabalho

Materializar 4 trait methods page-aware (itens 9, 10,
12, 13 P207A) + infraestrutura subjacente (decisão de
forma fixada em C1 com base em consumers reais).

Reuso de dados P207A + P207B + P207C:

- Trait com 22 métodos (`introspector.rs`; hash actual
  `12aab176`).
- Pattern de injecção `inject_positions` consolidado
  em P205C — `TagIntrospector` ganhou
  `pub fn inject_positions(&mut self, sealed:
  SealedPositions)`.
- `SealedPositions` em
  `01_core/src/entities/sealed_positions.rs` (hash
  `89baeda9`) — `position_of(loc) -> Option<Position>`
  onde `Position { page: NonZeroUsize, point: Point }`.
- `LayouterRuntimeState` em
  `01_core/src/rules/layout/layouter_runtime_state.rs`
  — populated single-pass; contém `positions`,
  `label_pages`, etc.
- `CountingIntrospector` wrapper L3 — regra empírica
  P207B §5: cada trait method novo propaga.

---

## §2 Cláusulas (5)

### C1 — Diagnóstico breve: decisão de infraestrutura

Antes de tocar código, inventário focado em **3
sub-secções**:

1. **Consumers reais dos 4 métodos page-aware**:
   - `pages(loc)`, `page(loc)`: grep production em
     `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/`
     + stdlib (esperado: zero, per P205D D3; consumer
     real desbloqueia com `here()` em P208).
   - `page_numbering(loc)`: idem; consumer principal
     seria `outline()` (P211).
   - `page_supplement(loc)`: idem; consumer principal
     seria refs com supplement override.
2. **Dados page-level disponíveis empiricamente**:
   - `LayouterRuntimeState` actual: lista fields
     literais; identificar quais carregam page
     numbering e page supplement.
   - Vanilla `PagedIntrospector::page_numbering` em
     `lab/typst-original/crates/typst-library/src/introspection/introspector.rs`:
     que dados consume? Resolvido em runtime ou
     pre-computed?
3. **Pattern subjacente**: comparar 3 opções:
   - **Opção 1 — estender `LayouterRuntimeState`**:
     adicionar `Vec<PageMeta>` ou `HashMap<NonZeroUsize,
     PageMeta>` durante layout; sealed pós-`finish`
     análogo a P205B mas para page-level.
   - **Opção 2 — `PageStore` sub-store dedicado**
     (paralelo a `SealedPositions`): novo
     `01_core/src/entities/page_store.rs`; novo
     `inject_pages` em `TagIntrospector`.
   - **Opção 3 — estender `SealedPositions`**:
     `Position` ganha `numbering` + `supplement`
     fields; `position_of` retorna richer struct;
     trait methods page-aware delegam.

Decisão em C2 com base em C1.1 + C1.2 + C1.3.

Critério literal:

- Se C1.1 mostrar **zero consumers reais** (esperado):
  é decisão estrutural antecipada para P208/P211.
  Forma menos invasiva é preferida.
- Se vanilla resolver page_numbering/page_supplement em
  **runtime via current_page state** (não pre-computed),
  cristalino pode seguir mesmo pattern via
  `LayouterRuntimeState` — Opção 1.
- Se vanilla pre-computa em `PagedIntrospector::new()`,
  cristalino deveria seal análogo — Opção 2.
- Opção 3 (mistura concerns) só se Position naturally
  carregar essa informação.

### C2 — Fixar opção de infraestrutura

Com base em C1.3, fixar **uma** opção. Justificar com:

- Pattern vanilla observado.
- Consumers reais (zero ou não).
- Reuso de pattern P205B/C (sealing + inject) vs nova
  abordagem.
- Custo estimado por opção.

C2 fixa **uma**. Se opções 1 e 2 forem ambas viáveis
sem diferenciador claro, **Opção 2 preferida** por
reuso literal de pattern P205B/C consolidado.

### C3 — Materializar infraestrutura + 4 trait methods

**L0 primeiro**:

Edições conforme C2:

- (Opção 2) Novo L0 prompt
  `00_nucleo/prompts/entities/page_store.md` análogo
  a `sealed_positions.md`.
- (Opção 1) Edição L0
  `00_nucleo/prompts/rules/layout/layouter_runtime_state.md`.
- (Opção 3) Edição L0
  `00_nucleo/prompts/entities/sealed_positions.md` +
  `position.md` para enriched fields.

Edição L0 `00_nucleo/prompts/entities/introspector.md`:
- 4 entradas novas: `pages`, `page`, `page_numbering`,
  `page_supplement`. Trait passa 22 → **26** métodos.
- Semântica: cada um retorna `Option<T>` (None se
  Location não-locatable ou pre-injecção).
- Paralelo vanilla: `PagedIntrospector` (single-target;
  cristalino tem único `TagIntrospector` — single-target
  trivial).

**L1 depois** (conforme C2):

- Opção 1: campo novo em `LayouterRuntimeState`; sealing
  análogo P205B-style em `Layouter::finish`.
- Opção 2: `01_core/src/entities/page_store.rs` (novo);
  field em `PagedDocument`; método `from_runtime`; 4
  accessors.
- Opção 3: `Position` extension; `SealedPositions`
  enriched.

`introspector.rs`:
- 4 trait methods novos + impl em `TagIntrospector`.
- Field novo em `TagIntrospector` (per C2 — `PageStore`
  ou referência runtime).
- Método de injecção (per C2 — `inject_pages` ou
  extensão de `inject_positions`).

### C4 — Propagação a `CountingIntrospector`

Per regra empírica P207B §5:

`03_infra/src/measurements.rs`:
- `INTROSPECTOR_METHODS`: 22 → **26**; 4 entries
  (pages, page, page_numbering, page_supplement).
- `CALL_COUNTERS`: 22 → **26**.
- impl 4 métodos delegando com `record_call(22)` /
  `record_call(23)` / `record_call(24)` / `record_call(25)`.
- Sentinel `p204g_introspector_call_counts_existe`: 22
  → 26.
- L0 `measurements.md`: descrição "26 métodos".

### C5 — Tests + verificação final

Tests dedicados (~8-12):

- 4 unit tests pre-injecção retornam None (1 por
  método).
- 4 unit tests pós-injecção retornam Some(valor)
  esperado.
- 1-2 E2E pipeline completo (layout → seal → inject →
  query page-aware via trait).
- 1-2 tests específicos da infraestrutura escolhida
  (struct/sub-store).

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 1893+ verdes (1885 + 8+); 0 violations.

Anotar ADR-0076 §P207D: `✅ MATERIALIZADO {data}` +
sumário opção fixada em C2.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-207D-relatorio.md`.

Estrutura paralela a P207B/P207C (~3-5 KB) com 1
secção extra "Opção de infraestrutura fixada" antes
das alterações de código.

---

## §4 Não-objectivos

- `here()` / `locate()` (P208).
- Selector enum extensions (P209).
- Outline configurável (P211).
- `query_count_before` (Q4 deferred).
- `figure_label_numbers` refactor — out-of-scope (per
  P207C §5 risco 2).
- Page-level rendering ou layout — page-aware é
  metadata, não rendering.

---

## §5 Riscos a evitar

1. **Inflar com Opção 3 sem benefício**: misturar
   point-level (Position) com page-level (numbering,
   supplement) em `SealedPositions` é tentação por
   minimização de structs mas mistura concerns
   (P205B fez Position concrete focado em coordenadas;
   numbering+supplement são page-meta).
2. **Materializar sem dados disponíveis**: se C1.2
   mostrar que `LayouterRuntimeState` actualmente não
   captura page_numbering ou page_supplement durante
   layout, materializar trait methods que retornam
   sempre None pre-`here()` consumers seria stub
   inflacionado. Mitigação: documentar empíricamente o
   que está disponível; trait method pode retornar None
   stub se infra ainda incompleta — caller chama
   `inject_pages` ou similar com dados parciais.
3. **Quebrar pattern de injecção consolidado**: opção 1
   altera `LayouterRuntimeState` mas pattern existente é
   sealing pós-finish via novo sub-store. Opção 1 pode
   ser viável mas exige justificação extra.
4. **Esquecer propagação a `CountingIntrospector`**: 4
   métodos novos exigem 4 entries + 4 slots + 4 impls +
   sentinel update. P207B §5 regra empírica torna isto
   mecânico mas obrigatório.

---

## §6 Nota sobre ambição P207D

Trait passa de 22 → 26 métodos num único sub-passo. É
o sub-passo mais largo de M9c em número de trait
extensions. Pode ser desdobrado em P207D + P207E (2
métodos cada) se C1 revelar que infraestrutura para
page_numbering / page_supplement é significativamente
distinta da page/pages. Decisão dentro de P207D se
empírico justificar.

Se desdobrado:
- P207D: `pages`, `page` + infra base.
- P207E: `page_numbering`, `page_supplement` + extensão
  da infra para metadata page-level.

Caso contrário, P207D faz os 4. P207E reservado para
outro Bloco II item ou avança para P208.
