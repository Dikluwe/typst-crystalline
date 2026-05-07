# P205A — Diagnóstico de F3 (cláusulas C1–C11)

**Data**: 2026-05-07.
**Pré-condição**: auditoria empírica P205A em
`typst-passo-205A-auditoria-f3.md` (A1–A14 completos).
**Output**: cláusulas C1–C11 instanciadas com valores
concretos + plano `*B+` sem ramos + decisão sobre
ADR-0074.

---

## §1 C1 — Escopo de F3

**Decisão**: **Mínimo** — 1–2 sub-stores trackable em
`LayouterRuntimeState` (`positions` + opcionalmente
`label_pages`).

Justificação (per A8 + A12 + A13 + A14):

1. **Categoria B é restrita** (3 sub-fields, todos em
   `runtime`). Não há "21 fields ortogonais elegíveis";
   o universo real é 3.
2. **A14** registou que benefício performance de F3
   minimal é arquitectural, não claramente mensurável;
   F3 completo (post-layout sealing global) magnitude
   XL com benefício potencial real mas custo elevado.
3. **A11** identificou `positions` como candidato com
   ganho semântico real (P204D deixou
   `position_of` retornando `None`; F3 minimal
   completaria essa pendência).
4. **A12** identificou `label_pages` como candidato
   secundário (sealing já implícito entre iterações
   fixpoint); migração trivial.
5. **`known_page_numbers`** é set externamente entre
   iterações — já é "sealed" por construção; tracking
   é trivial mas redundante (consumer já tem snapshot).
6. **`is_readonly`** é Categoria A (flag interno);
   excluído.

**Escopo concreto fixado**:

- Sub-store **`positions`** trackable post-layout
  (sealing point: fim de cada iteração fixpoint OU fim
  de `pub fn layout` se single-pass).
- Sub-store **`label_pages`** trackable post-iteração
  (avaliar magnitude vs benefício durante `*B`).

**Não inclui**:

- Reorganização Categoria D (`page_config`, `locator`,
  `current_location`).
- Consolidação Categoria A (cursor, cell_*, chain — são
  refactors ortogonais sugeridos para P210/P211).
- Restruturação do Layouter monolítico para emular
  arquitectura vanilla (Engine + N Layouters
  especializados) — magnitude XL+, fora-de-escopo.

---

## §2 C2 — Modelo de tracking

**Decisão**: **Híbrido** — sealing post-iteração por
sub-store.

Justificação (per A8 + A9 + A11):

1. **Single-pass puro** é impossível para Categoria B
   por paradox mutability/imutabilidade (A8).
2. **Post-layout vanilla-like** exigiria construção de
   `PagedIntrospector` análogo cristalino — magnitude
   M+ a L (refactor cross-modular). Vanilla **divergiu
   intencionalmente** (per P204D + P203A C5).
3. **Híbrido sealing post-iteração** mantém populated
   single-pass durante a iteração (sem mudança de
   pipeline), mas seal os sub-stores no fim de cada
   iteração para tracking na iteração seguinte (e em
   queries pós-layout).

**Mecanismo concreto**:

- Layouter populates `runtime.positions` durante a
  iteração (sem alteração).
- `pub fn layout` fim → sealing: extrair `positions`
  para sub-store immutável `SealedPositions` que é
  tracked.
- Iteração N+1 (fixpoint) lê via Tracked se necessário.
- Queries pós-layout (consumers `position_of`) acedem
  via Tracked sealed.

**Convergência intencional cristalino vs vanilla**:

- **Vanilla**: post-layout sealing global
  (`PagedIntrospector::new(&pages)` único call).
- **Cristalino F3 híbrido**: sealing por sub-store, no
  fim de iteração ou `layout()`.

C2 fixa **uma** alternativa: **Híbrido sealing
post-iteração**.

---

## §3 C3 — Mecanismo de tracking

**Decisão**: **Padrão A literal** — `#[comemo::track]`
em trait `PositionStore` (ou similar) implementada por
struct sealed.

Justificação (per A6 + A8):

1. **Vanilla usa Padrão A literal** em todos os 8
   sub-stores tracked auditados (Sink, Route, Traced,
   Locator, Context, World, Introspector,
   LateLinkResolver). Padrão B3 não é usado para
   nenhum tipo infraestrutural.
2. **M8 cristalino adoptou Padrão A literal** para
   trait `Introspector` — paridade vanilla. Coerência
   arquitectónica favorece Padrão A para F3.
3. **Categoria B fields são tipos concretos** (HashMap
   sealed); padrão A com trait dedicado expõe queries
   relevantes (e.g. `position_of(loc) -> Option<Position>`)
   sem expor o HashMap interno.
4. **Padrão B3** só faria sentido se houvesse múltiplas
   implementações (cristalino + paged + html). Não há
   plano para múltiplas — Padrão B3 seria over-engineering.

**Mecanismo concreto**:

```text
pub trait PositionStore: Send + Sync { ... }   // L1

#[comemo::track]
pub trait PositionStore { ... }
```

Ou `#[comemo::track] impl SealedPositions { ... }` se
struct concreta basta.

C3 fixa **uma** alternativa: **Padrão A literal**.

---

## §4 C4 — Sealing point

**Decisão**: **Fim de cada iteração fixpoint** (após
`l.finish()`).

Justificação (per A3 + A10 + A11):

1. **A10** mostrou que loop fixpoint corre N iterações;
   cada uma constrói `Layouter::new` fresh. Sealing por
   iteração é natural.
2. **A11** identificou que `positions` é populated
   durante a iteração; sealing no fim da iteração é
   inevitável se queries cross-iteration são desejadas.
3. **`pub fn layout`** end-point é aceitável para o
   short-circuit case (sem TOC; per `mod.rs:1524`); para
   o fixpoint case, sealing por iteração é melhor
   granularidade.

**Mecanismo concreto**:

- `Layouter::finish()` retorna `(PagedDocument,
  SealedPositions)` ou similar.
- Loop fixpoint extrai sealed sub-stores, opcionalmente
  passa Tracked para iteração seguinte (P204C-style).
- Ao fim do loop, último sealed é exposto em
  `PagedDocument` ou retornado separadamente para
  queries pós-layout.

C4 fixa **uma**: sealing após `l.finish()` por iteração.

---

## §5 C5 — Compatibilidade com fixpoint

**Decisão**: **Coexistência** — F3 sub-stores tracked
em paralelo com hash convergence.

Justificação (per A10):

1. Hash convergence (`extracted_label_pages ==
   known_page_numbers`) é semântica do fim do fixpoint.
2. Tracking acelera queries dentro de cada iteração
   (cache hits) e queries cross-iteration via sealed
   sub-stores.
3. Substituição (tracking-based fixpoint convergence)
   exigiria refactor profundo da semântica do loop —
   fora-de-escopo.
4. Cristalino mantém divergência intencional
   (hash-based) face a vanilla
   (`comemo::Constraint::validate`-based — implícito).

C5 não tem ramos. Fixada: **Coexistência**.

---

## §6 C6 — Position e F3

**Decisão**: **Position trackable** — sub-store sealed
pós-iteração; `TagIntrospector::position_of` ganha impl
real via sub-store sincronizado.

Justificação (per A11 + C1 + C4):

1. C1 inclui `positions` como sub-store F3 minimal.
2. P204D deixou `position_of` retornando `None` (per
   §C6a do diagnóstico P204D); F3 minimal **fecha essa
   pendência**.
3. Padrão C6a (consumers acedem `layouter.runtime.positions`
   directamente) é alternativa back-up se F3 não
   prosseguir.

**Mecanismo concreto**:

- `Layouter::finish` produz `SealedPositions`
  (HashMap immutável wrapped).
- `PagedDocument` ganha campo
  `pub positions: Arc<SealedPositions>` (ou similar).
- Trait `Introspector` ganha consumer que acede via
  `SealedPositions` quando disponível
  (`PagedIntrospector` ou `TagIntrospector`
  enriquecido).
- Consumers existentes (`layouter.runtime.positions`)
  permanecem por compatibilidade.

C6 fixa **uma**: **Position trackable via sub-store
sealed**.

---

## §7 C7 — Lacunas residuais e F3

**Decisão**: F3 não abre lacunas novas; mantém
divergência intencional cristalino vs vanilla
(arquitectura assimétrica per A5).

Justificação:

1. P204H confirmou zero lacunas formalmente catalogadas.
2. F3 minimal (C1) endereça pendência específica
   (`position_of` real) sem criar nova categoria.
3. Categoria D fields ambíguos (`page_config`,
   `locator`, `current_location`) **não são abordados**
   por F3. Permanecem ambíguos; podem ser sub-passos
   futuros (P210+).
4. Divergência arquitectónica `P205A.div-1` (cristalino
   monolítico vs vanilla decomposto) é registada como
   **decisão de design**, não lacuna.

**Lacuna potencial registada** (não bloqueante):

- Consumers de `position_of` ainda usam dual path
  (Introspector + `layouter.runtime.positions`); F3
  unifica para Introspector path. Migração de consumers
  é trabalho de sub-passos `*B+`.

---

## §8 C8 — Magnitude agregada

**Output** (não pré-fixada).

| Decisão | Contribuição |
|---------|-------------|
| C1 = Mínimo (positions + label_pages) | M (2 sub-stores) |
| C2 = Híbrido sealing post-iteração | S (mecanismo simples) |
| C3 = Padrão A literal | S (paridade M8) |
| C4 = Fim de cada iteração | S |
| C5 = Coexistência | nulo (não muda fixpoint) |
| C6 = Position trackable | S (fecha pendência P204D) |
| C7 = Sem lacunas novas | nulo |

**Magnitude agregada estimada**: **M** (cross-modular
mas escopo focado; ~3-5 sub-passos de magnitude S-M
cada).

Sub-passos prováveis (per C10):

- B — sealing infrastructure + `SealedPositions`.
- C — `position_of` impl real + consumers migrate.
- D — `label_pages` trackable + sealing.
- E — measurements + tests + ADR ACEITE.

Magnitude por sub-passo: S-M cada. Total: M agregado.

---

## §9 C9 — ADR-0074 PROPOSTO?

**Decisão**: **Sim, criar ADR-0074 PROPOSTO**.

Justificação:

1. **F3 é decisão arquitectural com alternativas reais**
   (C1 escopo mínimo/médio/completo; C2 single-pass /
   post-layout / híbrido; C6 trackable / runtime).
2. **F3 estende padrão M8** mas com **divergência
   significativa** (vanilla não tem Layouter
   monolítico — F3 é solução cristalina específica,
   não paridade literal).
3. **ADR dedicada permite cross-reference** com
   ADR-0073 (M8) e documentar a divergência intencional
   cristalino vs vanilla.
4. **Padrão emergente**: cada marco arquitectónico
   maior tem ADR dedicada (ADR-0072 para M7, ADR-0073
   para M8). F3 segue esse padrão.

**Não, documentação inline** seria suficiente apenas
se F3 fosse pura paridade literal (Padrão A em sub-stores
existentes); mas A5+A6 mostraram que F3 é divergência —
exige ADR dedicada.

C9 = **Afirmativa**. ADR-0074 é Output 4.

---

## §10 C10 — Sub-passos `*B+` (plano sem ramos)

Plano fixado com base em C1–C9:

### P205B — Sealing infrastructure + SealedPositions

**Magnitude**: S–M.

- Definir `pub struct SealedPositions(Arc<HashMap<Location, Position>>)` em L1.
- Implementar `#[comemo::track] impl SealedPositions { fn position_of(&self, loc: Location) -> Option<Position>; }`.
- `Layouter::finish` retorna tuple `(PagedDocument, SealedPositions)` ou anexa ao `PagedDocument`.
- Tests: 2-3 sentinelas (struct existe; track aplicado; finish retorna).

### P205C — `position_of` impl real + consumer migration

**Magnitude**: S–M.

- `TagIntrospector` (ou novo `PagedTagIntrospector`)
  consome `SealedPositions` para impl `position_of`
  retornando `Some(Position)` real (em vez de `None`).
- Consumers do dual path migrate para
  `Introspector::position_of` exclusivamente.
- Tests: cobertura E2E (consumer recebe Position
  correcta).

### P205D — `label_pages` trackable

**Magnitude**: S.

- `pub struct SealedLabelPages(Arc<HashMap<Label, usize>>)`.
- Análogo a P205B; sealing após cada iteração fixpoint.
- Consumer (outline.rs?) migra para Tracked se houver
  benefício; senão, mantém runtime path com sub-store
  paralelo.
- Tests: 2 sentinelas.

### P205E — Encerramento + ADR ACEITE

**Magnitude**: S documental (análogo a P204H).

- Auditoria das condições de validação ADR-0074.
- Forma de fecho (estruturalmente fechado / completo).
- ADR-0074 PROPOSTO → ACEITE.
- Blueprint anotado [P205].
- Relatório consolidado da série P205.

**Total**: 4 sub-passos `*B+` (P205B–E) + P205A
diagnóstico = 5 sub-passos na série P205.

**Magnitude total**: M (S+S+S+S = M agregado).

C10 fixa **plano sem ramos**.

---

## §11 C11 — Sem cláusulas condicionais

C1–C10 fixadas com valores concretos:

- C1 = Mínimo (positions + label_pages).
- C2 = Híbrido sealing post-iteração.
- C3 = Padrão A literal.
- C4 = Fim de cada iteração fixpoint.
- C5 = Coexistência com hash fixpoint.
- C6 = Position trackable via sealed sub-store.
- C7 = Sem lacunas novas; divergência registada.
- C8 = Magnitude M agregada.
- C9 = ADR-0074 PROPOSTO sim.
- C10 = 4 sub-passos B-E plano fixo.

---

## §12 Decisões durante a leitura

### D1 — Vanilla não tem Layouter monolítico (`P205A.div-1`)

A5 confirmou empíricamente. Implicação directa: F3 não é
"paridade vanilla literal" como M8 foi. F3 é solução
cristalina específica; ADR-0074 deve documentar a
divergência intencional.

### D2 — Categoria B é apenas 3 sub-fields

A2 mostrou que dos 21 fields ortogonais, apenas o `runtime`
field contém Categoria B (3 sub-fields). Os restantes 18
são A/C/D. F3 escopo "mínimo" é praticamente o universo
elegível — não há "F3 completo" plausível sem expandir
a definição de Categoria B ou refactor profundo da
arquitectura cristalina.

### D3 — Position concrete (P204D) ainda incompleto

P204D materializou tipo `Position` + `runtime.positions`
populated, mas `TagIntrospector::position_of` retorna
sempre `None`. F3 minimal **fecha essa pendência**
estruturalmente — `position_of` ganha impl real via
sub-store sealed.

### D4 — Loop fixpoint hash-based mantém-se

A10 confirmou que hash convergence
(`extracted_label_pages == known_page_numbers`) é
ortogonal a tracking. F3 + fixpoint coexistem; nenhum
substitui o outro. Cristalino mantém divergência
intencional vs vanilla (que usa `comemo::Constraint`
implicitamente).

### D5 — Magnitude F3 é M (não L como M8)

C8 totaliza M. F3 é mais focado que M8 (que envolveu 7
sub-passos B-H + P204A diagnóstico = 8 sub-passos com
magnitude L cross-modular). F3 é 4 sub-passos B-E +
P205A = 5 sub-passos com magnitude M agregada.

### D6 — Categorias D ficam para sub-passos futuros (P210+)

F3 não toca `page_config`, `locator`,
`current_location`. Esses são sub-passos ortogonais
candidatos a P210+ (consolidação cell_*, cursor as
Point, etc.).

---

## §13 Cross-references

- **Auditoria empírica**:
  `00_nucleo/diagnosticos/typst-passo-205A-auditoria-f3.md`.
- **Spec**:
  `00_nucleo/materialization/typst-passo-205A.md`.
- **Snapshot 2026-05-05** §5 (21 fields ortogonais
  pós-P190I) — referência cumprida.
- **ADR-0073** (M8 ACEITE 2026-05-07) — referência para
  Padrão A literal e arquitectura M8.
- **P204D** — referência para Position concrete
  (pendência fechada por F3 minimal).
- **P190C** — referência para `LayouterRuntimeState`
  pattern (struct dedicada).
- **Vanilla `PagedIntrospector::new`**:
  `lab/typst-original/crates/typst-layout/src/introspect.rs:38`
  — referência para post-layout sealing pattern.
- **Vanilla `Engine`**:
  `lab/typst-original/crates/typst-library/src/engine.rs:19`
  — referência para arquitectura decomposta.
