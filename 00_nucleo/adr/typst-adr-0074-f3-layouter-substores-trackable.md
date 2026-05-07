# ⚖️ ADR-0074: F3 — Layouter sub-stores trackable (sealing post-iteração)

**Status**: **ACEITE** (final, P205E 2026-05-07).
**Validado**: P205A–E concluídos; 7/7 condições do plano
de validação CUMPRIDAS (P205D condicional cumprida via
deferral documentado per cond 3).
**Data**: 2026-05-07 (PROPOSTO P205A); 2026-05-07
(ACEITE P205E).
**Sub-passos**: P205A (PROPOSTO); P205B–C
(materialização); P205D (DEFERIDO empírico); P205E
(transição ACEITE).
**Diagnóstico prévio**:
- `00_nucleo/diagnosticos/typst-passo-205A-auditoria-f3.md` (P205A).
- `00_nucleo/diagnosticos/typst-passo-205A-diagnostico.md` (P205A).

---

## Validação P205A–E — ACEITE final

**Data**: 2026-05-07.
**Auditor**: P205E (per spec C1).

| # | Condição (per Plano de validação) | Estado | Evidência |
|---|-----------------------------------|--------|-----------|
| 1 | P205B materializado: `SealedPositions` + `#[comemo::track]` impl; `Layouter::finish` retorna sealed sub-store | **CUMPRIDA** | `01_core/src/entities/sealed_positions.rs` (hash `89baeda9`); `Layouter::finish` produz via `SealedPositions::from_runtime(self.runtime.positions)` em `mod.rs:1187-1189`; 4 tests novos (2 sentinelas + 2 unit) |
| 2 | P205C materializado: `TagIntrospector::position_of` retorna `Some(Position)` via consumer com sealed sub-store | **CUMPRIDA** | `TagIntrospector` campo `pub positions: SealedPositions` + método `pub fn inject_positions`; trait impl delega `self.positions.position_of(location)`; 4 tests novos (3 unit `p205c_*` em `introspector.rs::tests` + 1 E2E em `layout/tests.rs`) |
| 3 | P205D materializado: `SealedLabelPages` struct (se benefício se materializar; senão, decisão de não prosseguir documentada) | **CUMPRIDA** | P205D deferred via Caminho B per spec branch "decisão de não prosseguir documentada"; 6 sub-secções de inventário empírico em `00_nucleo/diagnosticos/typst-passo-205D-inventario.md`; relatório em `00_nucleo/materialization/typst-passo-205D-relatorio.md` |
| 4 | Tests workspace verdes (estimativa 1852 → 1862-1870; ∆+10 a +18) | **CUMPRIDA** | 1852 → 1860 verdes (∆+8 real). Critério literal "verdes" cumprido; estimativa numérica orientadora foi superestimada porque P205D deferred eliminou ~4-6 tests previstos. Sem regressão |
| 5 | `crystalline-lint .` 0 violations | **CUMPRIDA** | `✓ No violations found` confirmado pós-cada sub-passo (P205B/C/D + verificação final P205E) |
| 6 | Sealing point identificado e implementado: `Layouter::finish` produz sealed sub-stores reproduzivelmente | **CUMPRIDA** | `Layouter::finish` em `01_core/src/rules/layout/mod.rs:1187-1189` produz `extracted_positions` 1× por iteração; padrão `from_runtime` consolidado |
| 7 | Consumers de `layouter.runtime.positions` directamente migrados para `Introspector::position_of` (P205C) | **CUMPRIDA-vacuously** | P205C C1.1 confirmou zero consumers de produção (P204F SKIP `here-locate`); migração formal limitada a tests E2E novos (D3 P205C). Infraestrutura activa para futuros consumers |

**Forma de fecho**: **Completo** (per P205E C2).

Justificação literal: ADR-0074 §"Decisão" declarou P205D
**condicional** ("opcional dependendo de benefício
observado em P205B/C"); plano de validação cond 3
aceitou explicitamente ambas as branches ("se benefício
se materializar; senão, decisão de não prosseguir
documentada"). P205D deferred com fundamento empírico
(zero consumers de produção; vanilla não trackeia
label_pages directamente; sub-store seria duplicação
de info já tracked por `SealedPositions` + label
registry) é **dentro do escopo declarado**, não
excepção.

Pendência **ADR-0073 §C6a fechada estruturalmente** por
F3:

- P204D §C6a deixou `TagIntrospector::position_of`
  retornando `None` como solução temporária.
- F3 minimal materializou: P205B (`SealedPositions`
  sealed sub-store) + P205C (`inject_positions` activa
  impl real). `Introspector::position_of` agora devolve
  `Some(Position)` real após injecção pós-layout.

ADR-0073 §C6a permanece textualmente como registo
histórico do estado intermédio; F3 fecha-a sem
alteração da ADR-0073.

ADR-0066 anotada cirúrgicamente (P205E C6 afirmativa)
com cross-reference a F3 para auditor futuro entender
chain-of-custody completo: introspection runtime adiada
→ M8 adoptou comemo → F3 fechou §C6a.

---

## Contexto

M8 estruturalmente fechado em P204H (2026-05-07). ADR-0073
ACEITE — `#[comemo::track]` aplicado ao trait
`Introspector` em paridade vanilla literal; Layouter
ganha lifetime `'a` + Tracked field (`introspector:
Tracked<'a, dyn Introspector + 'a>`); Position concrete
materializado mas `position_of` ainda retorna `None`
(per ADR-0073 §C6a; `TagIntrospector` é construído
pre-layout, sem acesso a Layouter runtime).

P204A diagnóstico §15 anteviu F3 como "refactor dos 21
fields ortogonais do Layouter (per snapshot 2026-05-05
§5; pós-P190I)". Snapshot consolidado §6 declarou que
"alguns dos 21 fields ortogonais do Layouter são
candidatos a migrar para sub-stores trackable se isto
reduzir aliasing entre estado de layout e estado de
introspecção".

P205A auditou empíricamente (A1–A14) e produziu
diagnóstico (C1–C11):

- **Apenas 3 sub-fields são Categoria B** (runtime
  introspecção): `runtime.label_pages`,
  `runtime.known_page_numbers`, `runtime.positions`. Os
  restantes 18 são Categoria A (runtime puro), C
  (config) ou D (ambígua).
- **Vanilla não tem Layouter monolítico** — `Engine<'a>`
  + N Layouters especializados (Composer, Distributor,
  Work, Collector, StackLayouter, GridLayouter). F3
  **não pode ser paridade vanilla literal** —
  arquitectura cristalino vs vanilla é
  fundamentalmente assimétrica (`P205A.div-1`).
- **Vanilla trackeia apenas post-sealing** —
  `PagedIntrospector::new(&pages)` constrói sub-stores
  immutáveis post-layout. Cristalino diverge
  intencionalmente (single-pass populates `runtime.positions`).
- **Categoria B fields populated single-pass** durante
  layout; tracking exige sealing point que cristalino
  não tem actualmente (`P205A.div-2`).

---

## Decisão

Cristalino adopta **F3 minimal** com **sealing
post-iteração** + **Padrão A literal** análogo a M8:

### Escopo (per P205A C1)

**Mínimo** — 2 sub-stores trackable:

1. `SealedPositions` (sub-store immutável de `runtime.positions`).
2. `SealedLabelPages` (sub-store immutável de
   `runtime.label_pages`; opcional dependendo de
   benefício observado em P205B/C).

**Não inclui**:

- Reorganização Categoria D (`page_config`, `locator`,
  `current_location`).
- Consolidação Categoria A (cursor as Point, cell_* as
  CellRect) — ortogonal; candidato a P210+.
- Restruturação do Layouter monolítico para emular
  arquitectura vanilla (Engine + N Layouters
  especializados) — magnitude XL+, fora-de-escopo.

### Modelo de tracking (per P205A C2)

**Híbrido sealing post-iteração**:

- Layouter populates `runtime.positions` durante a
  iteração (sem alteração).
- `Layouter::finish()` produz tuple `(PagedDocument,
  SealedPositions)` ou anexa ao `PagedDocument`.
- Iteração N+1 (fixpoint) ou queries pós-layout lêem
  via Tracked.

Mantém divergência intencional cristalino vs vanilla:

- **Vanilla**: post-layout sealing global
  (`PagedIntrospector::new(&pages)` único call).
- **Cristalino**: sealing por sub-store, no fim de
  iteração ou `layout()`.

### Mecanismo (per P205A C3)

**Padrão A literal** — `#[comemo::track]` em trait
ou impl directo de struct concreta:

```text
pub struct SealedPositions(Arc<HashMap<Location, Position>>);

#[comemo::track]
impl SealedPositions {
    pub fn position_of(&self, loc: Location) -> Option<Position>;
}
```

Coerência arquitectónica com M8 (que adoptou Padrão A
literal em trait `Introspector`).

### Sealing point (per P205A C4)

**Fim de cada iteração fixpoint** (após `l.finish()`).

Mecanismo:

- `Layouter::finish` retorna sealed sub-stores além de
  `PagedDocument`.
- Loop fixpoint extrai sealed; opcional Tracked entre
  iterações.
- Último sealed exposto em `PagedDocument` ou retornado
  para queries pós-layout.

### Compatibilidade com fixpoint (per P205A C5)

**Coexistência** — F3 sub-stores tracked em paralelo
com hash convergence
(`extracted_label_pages == known_page_numbers`).

F3 acelera queries dentro de cada iteração (cache
hits); hash convergence detecta fim do loop.
Substituição (tracking-based fixpoint convergence) seria
refactor profundo da semântica do loop —
fora-de-escopo.

### Position e F3 (per P205A C6)

**Position trackable** via sub-store sealed. F3 minimal
**fecha pendência P204D §C6a**: `TagIntrospector::position_of`
ganha impl real via consumer que acede `SealedPositions`.

Consumers existentes (`layouter.runtime.positions`)
permanecem por compatibilidade; migração futura para
`Introspector::position_of` exclusivo (P205C).

---

## Alternativas consideradas

### Alternativa B — F3 escopo médio (todos Categoria B sealable)

**Rejeitada**. Categoria B é apenas 3 sub-fields; "médio"
e "mínimo" coincidem na prática.

### Alternativa C — F3 escopo completo (post-layout vanilla-like)

**Rejeitada por desproporção**. Exigiria:

- Construir `PagedIntrospector` análogo cristalino
  (refactor cross-modular).
- Alterar pipeline de layout para separar populate de
  sealing globalmente.
- Magnitude XL+ com benefício potencial não
  demonstrado (sem benchmarks).

Cristalino divergiu intencionalmente em P204D para
single-pass; reverter exigiria justificação que A14 não
forneceu.

### Alternativa D — Padrão B3 (trait + blanket impl)

**Rejeitada**. Vanilla usa Padrão A literal em todos os
8 sub-stores tracked auditados. Cristalino M8 adoptou
Padrão A. Padrão B3 só faria sentido se houvesse
múltiplas implementações (cristalino + paged + html);
não há plano.

### Alternativa E — F3 single-pass tracking

**Rejeitada por impossibilidade técnica**. Fields
populated mutably durante layout não podem ser tracked
no mesmo período (paradox; A8). Sealing point é
inevitável.

### Alternativa F — Não adoptar F3 (manter Padrão C6a)

**Rejeitada**. P204D §C6a deixou
`TagIntrospector::position_of` retornando `None` como
solução temporária. F3 minimal **fecha essa pendência**
estruturalmente. Sem F3, consumers continuam com dual
path indefinidamente — fragmentação arquitectónica.

---

## Consequências

### Positivas

- **Fecha pendência P204D §C6a** —
  `Introspector::position_of` ganha impl real.
- **Cache hits** em queries `position_of(loc)`
  repetidas após sealing — proveitoso para documentos
  com muitas references.
- **Coerência arquitectónica com M8** — Padrão A
  literal aplicado consistentemente.
- **Sealing point explícito** — separação clara
  layout-time (mutable) vs query-time (Tracked).
- **Consumers podem migrar** para Introspector path
  exclusivo (eliminando dual path
  `layouter.runtime.positions` direct access).

### Negativas

- **Layouter::finish API muda** — retorna tuple ou anexa
  sealed sub-stores. Breaking change interno
  (consumers cristalino).
- **Arc + HashMap clone** no sealing — overhead pequeno
  por iteração.
- **Divergência intencional vs vanilla** explícita —
  documentação adicional; sem benchmark comparativo
  (per P205A A14 não-objectivo).
- **Categoria D fields ambíguos não endereçados** —
  permanecem ambíguos; resolução adiada para sub-passos
  futuros (P210+).

### Neutras

- **Loops fixpoint cristalinos preservados** —
  hash-based convergence mantém-se.
- **`label_pages` é trackable mas opcional** — decisão
  binária em P205D depende de benefício observado.
- **Vanilla `PagedIntrospector` permanece referência
  arquitectónica** mas não é reproduzido literalmente
  (divergência registada `P205A.div-1`).

---

## Plano de validação

ADR-0074 transita de `PROPOSTO` para `ACEITE` quando
todas estas condições forem verdadeiras (verificadas em
P205E):

1. **P205B materializado**: `SealedPositions` struct + `#[comemo::track]` impl;
   `Layouter::finish` retorna sealed sub-store.
2. **P205C materializado**: `TagIntrospector::position_of`
   retorna `Some(Position)` via consumer com sealed
   sub-store.
3. **P205D materializado**: `SealedLabelPages` struct (se
   benefício se materializar; senão, decisão de não
   prosseguir P205D documentada).
4. **Tests workspace verdes**: estimativa 1852 → 1862-1870
   (∆+10 a +18 tests novos).
5. **Crystalline-lint 0 violations**.
6. **Sealing point identificado e implementado** —
   `Layouter::finish` produz sealed sub-stores
   reproduzivelmente.
7. **Consumers de `layouter.runtime.positions`
   directamente migrados** para `Introspector::position_of`
   (P205C).

ADR transita para `REJEITADO` se durante materialização
for descoberto:

- Sealing impossível por algum field B (improvável dada
  a auditoria empírica P205A).
- Arc + HashMap overhead inaceitável (improvável; clone
  é O(n) único por iteração).
- Tests catastróficos (>5% regressão) — improvável;
  cristalino não tem benchmarks de baseline.

Se ADR for rejeitada, ADR-0073 §C6a (Padrão C6a)
mantém-se: consumers acedem `layouter.runtime.positions`
directamente; pendência P204D não é fechada.

---

## Plano de materialização

5 sub-passos (P205A–E):

### P205A — Diagnóstico-primeiro — ✅ MATERIALIZADO 2026-05-07

Magnitude M (real: ~30 min).

- Auditoria empírica A1–A14 com etiquetas e evidência.
- Diagnóstico C1–C11 com decisões fixadas.
- ADR-0074 PROPOSTO (este ficheiro).
- Plano `*B+` sem ramos (4 sub-passos).

### P205B — Sealing infrastructure + SealedPositions — ✅ MATERIALIZADO 2026-05-07

Magnitude S–M (real ~30 min).

- `pub struct SealedPositions(Arc<HashMap<Location, Position>>)` em L1.
- `#[comemo::track] impl SealedPositions { fn position_of(&self, loc: Location) -> Option<Position>; }`.
- `Layouter::finish` produz sealed sub-store.
- 2-3 sentinelas.

**Materialização**: `01_core/src/entities/sealed_positions.rs`
(L1; hash `89baeda9`). Newtype `SealedPositions { positions:
HashMap<Location, Position> }` (sem `Arc` — coerência com
pattern `BibStore`/`MetadataStore`; clone O(n) aceitável,
sealing 1× por iteração). Caminho B fixado em C2: campo
`pub extracted_positions: SealedPositions` em
`PagedDocument` com `Default::default()` retrocompatível.
2 sentinelas + 2 unit tests = 4 tests novos.
1852 → 1856 verdes; 0 violations. L0 prompt
`00_nucleo/prompts/entities/sealed-positions.md` (hash
`94c68ba8`).

### P205C — `position_of` impl real + consumer migration — ✅ MATERIALIZADO 2026-05-07

Magnitude S–M (real ~25 min).

- `TagIntrospector` (ou novo wrapper) consome
  `SealedPositions` para impl `position_of`.
- Consumers do dual path migrate.
- Tests E2E.

**Materialização**: Caminho A fixado em C2
(`TagIntrospector` enriquecido em vez de
`PagedTagIntrospector` wrapper — wrapper exigiria
delegar 19 métodos só para 1 especial; cristalino tem
única impl; per `P205A.div-1`). `TagIntrospector` ganha
campo `pub positions: SealedPositions` (default empty)
+ método `pub fn inject_positions(&mut self, sealed)`.
`Introspector::position_of` delega a
`self.positions.position_of(location)` —
**comportamento P204D §C6a preservado pre-injecção**
(devolve `None`); `Some(Position)` real após injecção.
Zero consumers de produção identificados em C1.1
(P204F SKIP `here-locate`); migração formal limitada
a tests E2E novos. 4 tests novos (3 unit em
`introspector.rs::tests` + 1 E2E em `layout/tests.rs`
exercendo pipeline completo layout → seal → inject →
query). 1856 → 1860 verdes; 0 violations.

### P205D — `label_pages` trackable (condicional) — ✅ DEFERIDO 2026-05-07

Magnitude real: S documental (~15 min).

- `pub struct SealedLabelPages(Arc<HashMap<Label, usize>>)`
  análogo a P205B — **NÃO materializado**.
- Consumer migra para Tracked se houver benefício;
  senão, mantém runtime path com sub-store paralelo.
- 2 sentinelas.

Decisão de prosseguir P205D fixa-se durante P205C com
base em benefício observado.

**Decisão fixada em P205D C2: Caminho B (adiar)**.

Fundamento empírico (P205D C1 inventário; 6 sub-secções):

1. **C1.1 — Zero consumers de produção** lêem
   `runtime.label_pages` directamente (`outline.rs:48`
   lê `runtime.known_page_numbers`, **distinto** —
   snapshot da iteração anterior do fixpoint, não o
   write-target durante layout).
2. **C1.4 — `doc.extracted_label_pages` consumido apenas
   por convergência fixpoint** (`layout/mod.rs:1575,1580`)
   via HashMap equality — operação não-tracked
   (single-shot por iteração).
3. **C1.5 — Vanilla NÃO trackeia label_pages
   directamente**. Trait vanilla `Introspector` tem
   `page(location)`, `pages(location)`,
   `page_numbering(location)`, `page_supplement(location)`
   — todos location-based, não label-based.
   Resolução label→page em vanilla: `query_label(label)
   -> Content` (extrai `Location`) + `position(location).
   page`. Cristalino **já tem essa rota** via
   `query_by_label(label) -> Option<Location>` +
   `Introspector::position_of(loc) -> Option<Position>`
   (impl real activada em P205C via `inject_positions`).
4. **C1.2 — Tracking de label_pages seria duplicação**
   de informação já tracked por
   `SealedPositions::position_of` (location → page via
   `Position.page`) + label registry (label →
   location). Cache hits agregar-se-iam no caminho
   tracked existente; sub-store dedicado não acrescenta
   capacidade observável.
5. **C1.6 — Aliasing confirmado** —
   `doc.extracted_label_pages = self.runtime.label_pages`
   (move semântico em `mod.rs:1183`); paralelo literal
   ao padrão P205B `extracted_positions::from_runtime`.

F3 minimal completo via P205B (sealing infrastructure +
`SealedPositions`) + P205C (`position_of` impl real via
injecção). Pendência ADR-0073 §C6a fechada
estruturalmente.

Risco evitado (per spec §8): inflar P205D por simetria
com P205B/P205C replicando `SealedLabelPages` quando a
infraestrutura é trivial e o benefício é zero — é
exactamente o anti-padrão que a spec anteviu.

Sem alterações de código. Sem ficheiros novos. Sem L0
novo. Sem tests novos. 1860 mantém-se. 0 violations
mantém-se.

### P205E — Encerramento + ADR ACEITE

Magnitude S documental.

- Auditoria das condições de validação ADR-0074.
- Forma de fecho.
- ADR-0074 PROPOSTO → ACEITE.
- Blueprint anotado [P205].
- Relatório consolidado da série P205.

---

## Cross-references

- **ADR-0073** (M8 ACEITE 2026-05-07) — `#[comemo::track]`
  em trait `Introspector`. F3 estende padrão para
  Layouter sub-stores com divergência intencional
  registada.
- **ADR-0066** (SUPERSEDED-BY 0073). F3 não toca
  ADR-0066.
- **ADR-0072** (M7 fixpoint estruturalmente fechado).
  Loops fixpoint preservados em F3.
- **P204D** — Position concrete; F3 fecha pendência
  §C6a.
- **P190C** — `LayouterRuntimeState` pattern (struct
  dedicada para Layouter-runtime). F3 estende com
  sealing.
- **P204A** §15 + snapshot consolidado §6 — referência
  para escopo "21 fields ortogonais"; F3 redefine
  escopo para 3 sub-fields Categoria B (per A2).
- **Vanilla `PagedIntrospector::new`**:
  `lab/typst-original/crates/typst-layout/src/introspect.rs:38`
  — referência arquitectónica (não paridade literal).
- **Vanilla `Engine`**:
  `lab/typst-original/crates/typst-library/src/engine.rs:19`
  — referência para arquitectura decomposta vanilla.
- **`P205A.div-1`** — Vanilla não tem Layouter
  monolítico; arquitectura cristalino vs vanilla
  assimétrica. F3 é solução cristalina específica.
- **`P205A.div-2`** — Categoria B fields populated
  single-pass; tracking exige sealing point
  (resolvido por C2 = híbrido sealing post-iteração).

---

## Pattern emergente

ADR-0074 aplica padrão consolidado pela série P204:

1. **Diagnóstico-primeiro de profundidade média** (14
   cláusulas A1–A14 cobrindo 5 blocos arquitecturais —
   menos que M8 16 cláusulas porque F3 escopo é
   menor).
2. **Decisões fixadas com base em empírico**, não
   herdadas.
3. **Padrão A literal** preferido (paridade com M8;
   vanilla também usa).
4. **Magnitude calibrada** — M agregado (S+S+S+S);
   menor que M8 L cross-modular.
5. **Divergência intencional vs vanilla registada**
   explicitamente (não escondida).

Pattern reaproveitável para futuros refactors
arquitectónicos cristalino-only (sem paridade vanilla
literal possível).
