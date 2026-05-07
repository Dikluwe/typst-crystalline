# P205D — Inventário interno (`label_pages` trackable, condicional)

**Data**: 2026-05-07.
**Spec**: `00_nucleo/materialization/typst-passo-205D.md`.
**Output 1 de 3** (inventário interno; ver também
`materialization/typst-passo-205D-relatorio.md`).
**Caminho fixado**: **B (adiar)**.

---

## §1 C1 — Inventário empírico (6 sub-secções)

Etiquetas: `CONFIRMADO` quando achado coincide com a
hipótese implícita da spec; `AJUSTE NECESSÁRIO` quando
divergência empírica obriga a recalibrar.

### §1.1 C1.1 — Consumers de `label_pages`

**Status**: `AJUSTE NECESSÁRIO` (vs. hipótese spec
"label_pages é lido durante layout").

Greps empíricos em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`:

- **Writes** (production): 1 call-site —
  `01_core/src/rules/layout/references.rs:30`
  (`layouter.runtime.label_pages.insert(label.clone(),
  page);`). Ocorre em arm `Labelled` após
  `layout_content(target)` para registar página final
  pós-quebra.
- **Reads** (production): **ZERO directos** sobre
  `runtime.label_pages`. O que parecia ser leitura é
  sempre via `doc.extracted_label_pages` (move pós-finish,
  ver §1.6).
- **Reads** sobre `doc.extracted_label_pages`
  (production):
  - `01_core/src/rules/layout/mod.rs:1575` —
    convergência fixpoint (`if doc.extracted_label_pages
    == known_page_numbers { return doc; }`). Operação
    HashMap equality, **não tracked**.
  - `01_core/src/rules/layout/mod.rs:1580` —
    `known_page_numbers = doc.extracted_label_pages.
    clone();` para próxima iteração.
- **Reads** em tests: 4 sítios (`tests.rs:1287`,
  `tests.rs:1304`, `tests.rs:1349`, `tests.rs:1412`)
  inspeccionam `doc.extracted_label_pages` para asserções
  de pipeline.
- **Discriminação importante**: `outline.rs:48`
  (`layouter.runtime.known_page_numbers.get(&label)`)
  **NÃO lê `label_pages`** — lê o snapshot da iteração
  anterior do fixpoint, que é write-target separado em
  `mod.rs:1569` (`l.runtime.known_page_numbers =
  known_page_numbers.clone();`). Os dois fields têm
  semântica complementar (current-write vs.
  previous-snapshot) per separação leitura/escrita
  declarada em `mod.rs:1546-1547`.

### §1.2 C1.2 — Trait `Introspector` e label_pages

**Status**: `CONFIRMADO`.

- **Trait actual cristalino**: NÃO tem
  `label_to_page(label) -> Option<NonZeroUsize>` ou
  similar. Tem:
  - `query_by_label(&Label) -> Option<Location>` (label
    → location).
  - `position_of(Location) -> Option<Position>` (P205C
    impl real; location → page+point via
    `Position.page`).
- **Trait vanilla**:
  `lab/typst-original/crates/typst-library/src/
  introspection/introspector.rs`. Métodos
  page/label-relevantes:
  - `fn page(&self, location: Location) -> Option<NonZeroUsize>`
    (linha 65) — page para location.
  - `fn pages(&self, location: Location) -> Option<NonZeroUsize>`
    (linha 62).
  - `fn page_numbering(&self, location)` (linha 71).
  - `fn page_supplement(&self, location)` (linha 74).
  - `fn label_count(&self, label: Label) -> usize`
    (linha 50).
  - `fn query_label(&self, label: Label) -> StrResult<&Content>`
    (não no impl trait listado, mas em método
    associado).
- **Vanilla NÃO tem `label_to_page` ou similar
  directamente**. A rota label→page em vanilla é
  `query_label(label) -> Content` → extrair Location →
  `position(location).page` (Position.page é
  `NonZeroUsize`). Cristalino tem rota equivalente:
  `query_by_label(label) -> Location` →
  `position_of(location)?.page` (P205C activo via
  injecção).

### §1.3 C1.3 — Pipeline pré vs pós-layout para label_pages

**Status**: `CONFIRMADO`.

- **Pre-layout**: ausente. `label_pages` só é populated
  durante o layout (resultado de quebras de página,
  conhecido apenas após walking through Content).
- **Durante layout**: populated single-pass via
  `layouter.runtime.label_pages.insert(label, page)` em
  `references.rs:30`, dentro do arm
  `Content::Labelled` (após `layout_content(target)`).
- **Pós-layout**: disponível via `doc.extracted_label_pages`
  (move semântico em `Layouter::finish` — ver §1.6).

### §1.4 C1.4 — Consumers actuais de `extracted_label_pages`

**Status**: `AJUSTE NECESSÁRIO` (vs. hipótese spec
"PDF export, PDF outline, outros call sites empíricos").

- **Convergência fixpoint**: 2 call-sites em
  `mod.rs:1575,1580`. HashMap equality
  comparison + clone para próxima iteração. **Não
  tracked**; operação single-shot por iteração (max 5
  iterações via `MAX_ITERATIONS`).
- **PDF export**: NÃO existe ainda. `02_shell/`,
  `03_infra/`, `04_wiring/` não referenciam
  `extracted_label_pages` (grep retorna zero).
- **PDF outline / TOC**: NÃO usa `extracted_label_pages`.
  `outline.rs` lê `runtime.known_page_numbers` (snapshot
  anterior do fixpoint), não o field do `PagedDocument`.
- **Tests**: 4 tests inspeccionam
  `extracted_label_pages` (basic populate, label
  presence, fixpoint convergence). Não há outros
  consumers.

### §1.5 C1.5 — Benefício potencial de tracking

**Status**: `AJUSTE NECESSÁRIO` (vs. hipótese spec
"queries label_pages talvez frequentes pós-layout").

Análise qualitativa empírica:

- **Frequência de queries de `label_pages` em
  produção**: ZERO (fora da convergência fixpoint que é
  HashMap equality não-tracked).
- **Cache hits via comemo**: zero — não há queries
  repetidas que pudessem reutilizar cache.
- **Vanilla aplica tracking em label→page directamente?**
  NÃO. Vanilla rota label→page via `query_label` +
  `position` — ambos são tracked, **mas o tracking é
  sobre Content/Position**, não sobre um `LabelPages`
  store dedicado. Per §1.2.
- **Cristalino já tem rota Tracked equivalente**:
  `query_by_label` (tracked via trait `Introspector`
  com `#[comemo::track]` em P204B) +
  `Introspector::position_of` (impl real activa via
  `inject_positions` per P205C; delega a
  `SealedPositions::position_of` que é `#[comemo::track]
  impl` per P205B). Cache hits agregam-se nesse caminho
  sem necessidade de sub-store dedicado.
- **Conclusão de benefício**: ZERO observável. Materializar
  `SealedLabelPages` seria duplicação de informação já
  tracked.

### §1.6 C1.6 — Aliasing entre `runtime.label_pages` e `doc.extracted_label_pages`

**Status**: `CONFIRMADO`.

- **Move semântico**, não clone. `Layouter::finish` em
  `mod.rs:1183`: `doc.extracted_label_pages =
  self.runtime.label_pages;`.
- Após `finish`, `runtime` é consumido (`self` é
  `Layouter` movido). Não há divergência possível —
  são literalmente o mesmo HashMap em pontos
  diferentes do pipeline.
- **Paralelo literal a P205B** —
  `doc.extracted_positions =
  SealedPositions::from_runtime(self.runtime.positions);`
  (`mod.rs:1187-1189`). Mesma forma estrutural; em
  P205B, o move passa pelo construtor `from_runtime`
  para envolver no newtype tracked. Em cristalino actual
  para `label_pages`, o move é directo porque o tipo
  destino é o mesmo `HashMap<Label, usize>`.

---

## §2 C2 — Caminho fixado: **B (adiar)**

Decisão honesta com base em dados empíricos C1:

| Critério (per spec C2) | Resultado | Caminho |
|------------------------|-----------|---------|
| C1.5 mostra **benefício real** + C1.4 identifica consumer concreto | **NÃO** — C1.5 zero benefício; C1.4 só convergência fixpoint não-tracked | A descartado |
| C1.5 mostra **zero benefício observável** + consumers actuais usam `doc.extracted_label_pages` directamente sem fricção | **SIM** — tudo coincide com este critério | **B fixado** |

### Justificação consolidada (5 pontos)

1. **Zero consumers de produção de `label_pages` ou
   `extracted_label_pages` para além da convergência
   fixpoint** (que é operação HashMap equality
   single-shot, não tracked — não beneficiaria de
   `#[comemo::track]`).
2. **Vanilla NÃO trackeia label_pages directamente** —
   rota label→page é via `query_label` + `position`
   (ambos location-based). Cristalino já tem essa rota
   activa: `query_by_label` (P204B tracked) +
   `position_of` (P205C real via P205B
   `SealedPositions`). Sub-store dedicado seria
   over-engineering.
3. **`SealedLabelPages` seria duplicação** de
   informação já tracked por `SealedPositions`
   (location → page via `Position.page`) + label
   registry (label → location via `LabelRegistry`).
   Cache hits agregam-se no caminho tracked existente.
4. **Risco anti-padrão (per spec §8)** — inflar P205D
   por simetria com P205B/P205C é exactamente o
   anti-padrão que a spec anteviu. ADR-0074 declara
   P205D **condicional**, não fixa materialização.
5. **F3 minimal já está completo** estruturalmente via
   P205B (sealing infrastructure + `SealedPositions`)
   + P205C (`position_of` impl real activa via
   `inject_positions`). Pendência ADR-0073 §C6a fechada.

### Hipótese mais provável da spec confirmada

A spec §8 anteviu textualmente: "Hipótese mais provável:
C1.5 mostra zero benefício empírico observável [...]
Caminho B (adiar) é resultado provável." A auditoria
confirma essa hipótese empíricamente.

---

## §3 (Caminho A) — N/A

Caminho A não fixado; secção omitida per spec C13
(execução do caminho fixado).

Para registo: se hipoteticamente Caminho A fosse fixado
no futuro (mudança de requirements), o trabalho
mínimo seria:

- `01_core/src/entities/sealed_label_pages.rs` (newtype
  `pub struct SealedLabelPages { label_pages:
  HashMap<Label, usize> }` + `#[comemo::track] impl
  { fn page_of(&self, label: Label) -> Option<usize> }`).
- L0 prompt em `00_nucleo/prompts/entities/
  sealed-label-pages.md`.
- `01_core/src/entities/mod.rs` — adicionar `pub mod
  sealed_label_pages;`.
- `Layouter::finish` — substituir `doc.
  extracted_label_pages = self.runtime.label_pages;`
  por construção via `from_runtime`.
- `TagIntrospector` — adicionar `pub label_pages:
  SealedLabelPages` + `pub fn inject_label_pages`.
- Tests dedicados (3-4 unit + sentinelas).

P205D fica deferido; não há **prazo** para Caminho A
ser revisitado — só faria sentido se aparecer consumer
de produção (ex: PDF outline usando label→page query
intensivamente, com pattern de cache hit observável).

---

## §4 Decisões durante a leitura

### D1 — Distinguir `runtime.label_pages` vs `runtime.known_page_numbers`

Antes da auditoria, parecia plausível que `outline.rs`
lesse `label_pages` (write-target durante layout). Grep
revelou que `outline.rs:48` lê `known_page_numbers` —
campo separado que recebe snapshot da iteração anterior
do fixpoint via `mod.rs:1569`. Os dois fields têm
semântica complementar. Esta distinção é central para
C1.1 mostrar zero consumers reais de `label_pages`
durante layout.

### D2 — Vanilla `Introspector` trait não tem `label_to_page`

Confirmado por grep em `lab/typst-original/crates/
typst-library/src/introspection/introspector.rs`.
Métodos page-relevantes são todos location-based. A rota
canónica vanilla label→page é multi-step
(`query_label` + `position`), não single-step.
Implicação: simetria com vanilla **não justifica**
materializar P205D — vanilla também não trackeia
label_pages directamente.

### D3 — Tipo do field é `usize`, não `NonZeroUsize`

A spec §3 C3 menciona `SealedLabelPages(HashMap<Label,
NonZeroUsize>)` como forma estrutural sugerida. Mas o
tipo actual em `runtime.label_pages` e
`doc.extracted_label_pages` é `HashMap<Label, usize>`
(per `layouter_runtime_state.rs:43` e
`layout_types.rs:429`). Vanilla usa `NonZeroUsize` (per
`fn page(...) -> Option<NonZeroUsize>`); cristalino
diverge. Não é trabalho de P205D harmonizar — Caminho A
hipotético usaria o tipo cristalino actual (`usize`)
para evitar refactor cascata. Caminho B torna esta
discussão irrelevante.

### D4 — `from_runtime` precedente em P205B torna Caminho A trivial

`SealedPositions::from_runtime(self.runtime.positions)`
é o padrão consolidado: move o HashMap interno para o
newtype. Caminho A para `SealedLabelPages` seria
literal-paralelo. Implementação total seria ~30-40 min
(spec §9 estimou correctamente). **Mas trivialidade ≠
necessidade** — risco anti-padrão §8.

### D5 — Convergência fixpoint não-tracked é insensível a sealing

`mod.rs:1575` (`if doc.extracted_label_pages ==
known_page_numbers`) é HashMap equality nativa. Se
`extracted_label_pages` fosse `SealedLabelPages`, teria
de wrap/unwrap ou expor método de comparison. Caminho A
**adicionaria fricção** sobre o consumer principal sem
benefício correspondente. Confirma B como decisão
correcta.

### D6 — `inject_label_pages` exigiria padrão paralelo a P205C

Caminho A precisaria não só do sealed sub-store mas
também do método `inject_label_pages` em
`TagIntrospector` para activar. Sem injecção, o
sub-store fica vazio post-construção. P205C demonstrou
que injecção exige caller pós-layout consciente —
adicionar segundo ponto de injecção sem consumer real
multiplicaria pontos de fricção arquitectural.

---

## §5 Resumo — métricas previstas

| Métrica | Valor |
|---------|-------|
| Caminho fixado | **B (adiar)** |
| Tests workspace antes | 1860 |
| Tests workspace depois | **1860** (sem alteração) |
| Tests P205D novos | 0 |
| Linter violations antes | 0 |
| Linter violations depois | 0 |
| Ficheiros novos (código) | 0 |
| Ficheiros modificados (código) | 0 |
| Ficheiros novos (docs) | 3 (esta diagnose + relatório + spec já existente) |
| Ficheiros modificados (docs) | 1 (ADR-0074 §P205D anotada) |
| LOC novas | 0 código |
| Cargo deps adicionados | 0 |
