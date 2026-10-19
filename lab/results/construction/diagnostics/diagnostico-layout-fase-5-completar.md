# Diagnóstico Layout Fase 5 — "completar Layout" (Tudo A+B+C+D)

**Data**: 2026-05-13.
**Passo**: P226 (primeiro passo Fase 5 Layout candidata
diagnóstico).
**Marco**: abertura série β "completar Layout" cumulativa
per decisão humana literal P225 §8 "completar Layout" =
Tudo A+B+C+D.
**ADR validadora**: ADR-0079 PROPOSTO (Layout Fase 5
roadmap).
**ADR meta complementar**: ADR-0080 PROPOSTO ("L0 minimal
para refactors aditivos pós-M9c" N=7).

---

## §1 Contexto

Layout pós-P225 está em **estado terminal estructural
reconhecido oficialmente** (§3.0terdecies P225):
- Cobertura per metodologia §A.9: **89% real** (paridade
  visual Opção γ refrescada).
- Distribuição §A.5: `12/4/2/0/0 = 18` (zero ausentes
  preservado).
- Fase 3 fechada P221 (DEBT-56 ENCERRADA); Fase 4 candidata
  fechada P225 (DEBT-37 §"Divergência" + DEBT-34e fechadas;
  DEBT-34d preservado per `P224.div-1`).
- ADR-0061 IMPLEMENTADO + ADR-0078 IMPLEMENTADO + ADR-0066
  PROPOSTO.
- Tests workspace: **2039 verdes**.

**Decisão humana literal P225 §8 + P226 pré-spec**:
"completar Layout" = Tudo A+B+C+D, incluindo:
- Refinos cosméticos (A).
- Refinos algorítmicos isolados (B).
- Reabertura de decisões arquitecturais (C).
- Reabertura ADR-0066 + arquitectura single-pass via
  runtime queries (D).

Este diagnóstico cobre 4 categorias com **13-14 sub-passos
identificados** mas **NÃO reservados** per política P158.

---

## §2 Categoria A — Cosméticos (sem reabrir decisões)

Atributos visuais não-estruturais; refinos paridade
P156G+H+I scope-outs Block/Boxed/Stack:

### A.1 — `stroke` Grid + Table inheritance
- **Mecânica**: paridade vanilla `GridStroke` — atributo
  uniforme Grid-level (4 sides ou per-side via Sides<...>).
- **Layouter**: emite `FrameItem::Line` por cell border
  (4 linhas por cell se single-cell; com gutter ajusta
  geometry).
- **Stdlib**: `native_grid` aceita `stroke: ?` named.
- **Inherits Table**: P157A `Content::Table` delegate
  passa parameter ou preserva default None.
- **Magnitude**: S+ a M (~1-2h).
- **L0**: Opção γ paridade ADR-0080 (refactor aditivo).
- **Dependências**: nenhuma (categoria A independente).

### A.2 — `fill` Grid + Table
- **Mecânica**: paridade vanilla `GridFill` — colour de
  fundo Grid-level.
- **Layouter**: emite `FrameItem::Shape::Rect` com fill
  per cell (background).
- **Stdlib**: `native_grid` aceita `fill: ?` named.
- **Magnitude**: S+ a M (~1-2h).
- **Dependências**: nenhuma; ortogonal a A.1.

### A.3 — `stroke`/`fill` em GridCell per-cell
- **Mecânica**: paridade vanilla per-cell precedence.
  GridCell `stroke`/`fill` substitui Grid-level se Some.
- **Refactor**: adicionar 2 fields a `Content::GridCell`
  (`stroke: Option<...>`, `fill: Option<...>`); arms
  cascata em 5 sítios.
- **Magnitude**: M (~2-3h; precedence rules + cascade).
- **Dependências**: **A.1 + A.2** (precedência GridCell
  vs Grid-level).

### A.4 — Block/Boxed `outset` + `radius` + `clip` (refinos P156G+H scope-outs)
- **Mecânica**: paridade vanilla `Block`/`Box` cosméticos
  (P156G+H deixaram 6 atributos scope-out).
- **Magnitude**: M (~2-3h cada; cumulativo M-L).
- **Dependências**: nenhuma; ortogonal.

### A.5 — Place per-cell alignment override
- **Mecânica**: Place dentro de Grid com `align: ?` específico.
- **Magnitude**: S+ (~1h).
- **Dependências**: nenhuma.

**Total Categoria A**: **5 sub-passos** identificados
não-reservados; magnitude cumulativa **M-L (~6-9h)**.

**Ordem sugerida**: A.1 → A.2 → A.3 (per-cell depende
Grid-level); A.4, A.5 ortogonais.

---

## §3 Categoria B — Algorítmicos isolados (sem reabrir decisões)

Refinos algorítmicos sem reabrir Opção B P219 ou
ADR-0066:

### B.1 — DEBT-34d Auto track sizing
- **Mecânica**: "Auto não encolhe antes de matar fr" —
  refactor passo 3 do `layout_grid` (algoritmo Auto vs
  Fraction negociação). Implementar min-content e
  max-content para Auto.
- **Fecha DEBT-34d** preservado per `P224.div-1`.
- **Magnitude**: M (~2-3h; algorítmico isolado em L1).
- **Dependências**: nenhuma (categoria B independente).
- **L0**: Opção γ (refactor algorítmico interno).

### B.2 — Consumer geometric integration P224.C
- **Mecânica**: `place_cells` algorítmico (P224.C 264 LOC)
  → Layouter geometric. Integrar `PlacedCell` com
  `layout_grid` para iteração geometric real (em vez de
  ordem linear actual P82).
- **Refactor**: `layout_grid` chama `place_cells` antes
  de iterar; itera `Vec<PlacedCell>` em ordem; renderiza
  cada body na célula `(row, col)` ocupando
  `colspan × rowspan`.
- **Magnitude**: M (~2-3h).
- **Dependências**: nenhuma estrita; complementa P224.C
  (integration consumer geometric).

### B.3 — Per-cell GridCell atributos (`align`/`inset`/`breakable`)
- **Mecânica**: paridade P157B subset estendido —
  adicionar 3 fields a `Content::GridCell`.
- **Layouter**: aplica per-cell override em Layouter Grid.
- **Magnitude**: M (~2-3h).
- **Dependências**: **B.2** (per-cell precisa integration
  geometric).

**Total Categoria B**: **3 sub-passos**; magnitude
cumulativa **M+ a L (~6-9h)**.

**Ordem sugerida**: B.1 → B.2 → B.3 (sequencial; B.2
desbloqueia B.3).

---

## §4 Categoria C — Estruturais reabrindo decisões (maior risco)

Reabertura de decisões arquitecturais maiores; **alta
complexidade**; magnitudes L+ a XL:

### C.1 — Place `float` real (flow contorna)
- **Mecânica**: paridade vanilla — Place com `float: true`
  flutua para topo/fundo página/coluna; conteúdo fluído
  contorna. Multi-region flow real **ou parcial** (flow
  secundário topo/fundo página).
- **Reabertura**: Opção B P219 graded — semantic real
  adiada per ADR-0054 (precedente N=5 cumulativo
  weak/breakable/float/repeat). C.1 promove `float`
  semantic adiada → real.
- **Magnitude**: **L+ (~5-8h)** — refactor multi-pass
  layout ou flow secundário.
- **Dependências**: pode beneficiar C.2 (multi-region
  facilita flow real) mas implementável standalone com
  flow secundário simples.
- **L0**: Opção α (mudança não-aditiva; reabertura
  decisão arquitectural).

### C.2 — Opção A multi-region completa (columns/colbreak real flow)
- **Mecânica**: paridade vanilla — `columns(n)` flow real
  entre N colunas (não single-region graded P219); `colbreak`
  salta para próxima coluna (não pagebreak downgrade β P220).
- **Reabertura mais complexa**:
  - **Reabre P216B** `Regions { current: Region }` minimal
    → multi-region `Regions { current, backlog, last }`
    completo.
  - **Reabre DEBT-56 ENCERRADA P221** — não literalmente
    (DEBT-56 preservada CLOSED); criar **DEBT-56b novo**
    para "refino Opção A multi-region pós-fecho DEBT-56".
- **Magnitude**: **L+ a XL (~10-20h)** — refactor
  multi-region completo + algoritmo flow.
- **Dependências**: C.1 pode usar Opção A se materializada
  (sinergias arquitecturais).
- **L0**: Opção α obrigatória (reabertura decisão maior;
  ADR-0078 pode precisar anotação ou superseder por
  ADR-0079 categoria C.2).

**Total Categoria C**: **2 sub-passos**; magnitude
cumulativa **L+ a XL (~15-28h)**.

**Ordem sugerida**: **C.1 OU C.2 caso-a-caso** —
ortogonais; C.1 standalone com flow secundário simples vs
C.1 acoplado a C.2 (multi-region facilita).

**Nota arquitectural sobre DEBT-56 reabertura**:
- DEBT-56 P221 ENCERRADA literal (CLOSED via materialização
  Opção B graded; critério 5/5 cumprido).
- C.2 materialização Opção A real **não reabre DEBT-56**;
  cria DEBT-56b novo.
- Pattern emergente "fecho de DEBT preservado literal +
  criação de novo DEBT para refino pós-fecho" N=1
  candidato.

---

## §5 Categoria D — Runtime queries (reabertura ADR-0066)

Reabre **ADR-0066 PROPOSTO** + arquitectura single-pass:

### D.1 — `state(key, init)` runtime mutable
- **Mecânica**: primeira feature runtime queries genuína.
  Mutable state cross-document evaluation.
- **Refactor**: extender `introspect.rs` para suportar
  state queries via fixpoint simples (2 iterações se state
  convergir).
- **Promoção ADR-0066 PROPOSTO → IMPLEMENTADO** ocorre
  após D.1 (3 condições §"Plano promoção" satisfeitas:
  state materializada; pipeline introspect extendido
  2-pass; tests E2E observable).
- **Magnitude**: M (~2-3h).
- **L0**: Opção α (mudança não-aditiva; reabertura
  arquitectura).

### D.2 — `metadata(value)` attaching
- **Mecânica**: anota Content arbitrário com metadata
  Value; query via D.4.
- **Magnitude**: S+ (~1-2h).
- **Dependências**: D.1 (pipeline 2-pass).

### D.3 — `here()` / `locate()` location-aware
- **Mecânica**: callbacks executados com Location actual
  (página + posição).
- **Magnitude**: M (~2-3h).
- **Dependências**: D.1 (pipeline 2-pass).

### D.4 — `query(target)` runtime introspection
- **Mecânica**: query Content por target (label, type,
  metadata) retornando Array<Content>.
- **Magnitude**: M+ (~3-4h).
- **Dependências**: D.1 + D.2 (state + metadata).

### D.5 — `position(target)` location-aware
- **Mecânica**: localização absoluta de target.
- **Magnitude**: S+ (~1-2h).
- **Dependências**: D.3 + D.4.

### D.6 — Cross-document cite refs (Bloco C cross-módulo continuação)
- **Mecânica**: cross-references entre documents.
- **Magnitude**: L+ (~5-8h; depende multi-document
  pipeline).
- **Dependências**: ortogonal Categoria D (não bloqueia
  D.1-D.5).

**Total Categoria D**: **5-6 sub-passos**; magnitude
cumulativa **L+ a XL (~10-18h)**.

**Ordem obrigatória**: **D.1 → {D.2, D.3} → D.4 → D.5**;
D.6 ortogonal.

**Cobertura Introspection esperada pós-D.5**: 17% → ~50%
(per ADR-0066 §"Subset minimal" estimativa).

---

## §6 Roadmap total + matriz dependências cumulativa

| Categoria | Sub-passos | Magnitude | Reabertura | Ordem sugerida |
|-----------|------------|-----------|------------|----------------|
| **A** Cosméticos | A.1+A.2+A.3+A.4+A.5 | M-L (~6-9h) | não | A.1→A.2→A.3; A.4, A.5 ortogonais |
| **B** Algorítmicos | B.1+B.2+B.3 | M+ a L (~6-9h) | não | B.1→B.2→B.3 |
| **C** Estruturais | C.1+C.2 | L+ a XL (~15-28h) | **sim** (Opção B P219; P216B + DEBT-56b) | C.1 ⊥ C.2 |
| **D** Runtime | D.1+D.2+D.3+D.4+D.5+D.6 | L+ a XL (~10-18h) | **sim** (ADR-0066 PROPOSTO → IMPLEMENTADO) | D.1→{D.2,D.3}→D.4→D.5; D.6 ⊥ |
| **Total** | **~13-15 sub-passos** | **L+ a XL (~37-64h)** | sim (2-3 reaberturas) | caso-a-caso |

**Dependências cross-categoria**:
- C.2 sinergia com B.2 (multi-region + geometric integration).
- C.1 standalone OU acoplado a C.2.
- D.6 ortogonal a A+B+C (multi-document).

---

## §7 Reaberturas arquiteturais (registo explícito)

### Reabertura 1 — Opção B P219 graded (Categoria C.1)
- **Decisão original P219**: column flow Opção B graded
  (single-region width reduzida).
- **C.1 materialização**: Place `float` real flow contorna
  (multi-pass layout ou flow secundário).
- **Nota arquitectural**: Opção B P219 preservada literal
  para column flow; C.1 introduz flow ortogonal (Place
  float ≠ column flow). Sem conflito directo.

### Reabertura 2 — P216B `Regions { current }` minimal (Categoria C.2)
- **Decisão original P216B**: `Regions` minimal
  (`{ current: Region }`); `backlog`/`last` diferidos.
- **C.2 materialização**: `Regions { current, backlog,
  last }` completo (multi-region real).
- **Nota arquitectural**: P216B preservada literal como
  baseline histórica; C.2 extende para Opção A real.

### Reabertura 3 — DEBT-56 ENCERRADA (Categoria C.2 derivada)
- **DEBT-56 P221**: ENCERRADA literal (CLOSED via
  materialização Opção B graded; critério 5/5 cumprido).
- **C.2 materialização**: introduz **DEBT-56b novo**
  para "refino Opção A multi-region pós-fecho DEBT-56".
- **Nota arquitectural**: DEBT-56 preservada CLOSED;
  DEBT-56b novo aberto se/quando C.2 materializar
  (decisão diferida ao próprio C.2).

### Reabertura 4 — ADR-0066 PROPOSTO (Categoria D)
- **Estado actual P226**: ADR-0066 PROPOSTO (Bloco C
  cross-módulo primeira materialização parcial via P222
  measure stdlib).
- **D.1 materialização**: state runtime mutable; 3
  condições §"Plano promoção" satisfeitas.
- **Promoção ADR-0066 PROPOSTO → IMPLEMENTADO** ocorre
  formalmente após D.1.
- **Nota arquitectural**: arquitectura single-pass
  pré-existente preservada para D.6 e refinos não-runtime;
  D.1-D.5 introduzem 2-pass para runtime features.

---

## §8 Trade-offs cumulativos

- **Magnitude total**: ~37-64h cumulativos (L+ a XL).
- **Cobertura Layout pós-completar**: 89% → **100%
  literal** (todas 18 entradas §A.5 → impl puro ou impl⁺).
- **Cobertura Introspection bonus**: 17% → ~50%
  (Categoria D D.1-D.5).
- **Reaberturas arquiteturais**: 3-4 explícitas (Opção B
  P219 + P216B + DEBT-56b + ADR-0066).
- **Risco arquitectural**: alto em C + D; baixo em A + B.
- **L0 actualização**: Opção γ para A + B (paridade
  ADR-0080); Opção α obrigatória para C + D (reaberturas
  decisões arquitecturais).

---

## §9 Decisão humana caso-a-caso

P226 **NÃO reserva** sub-passos per política P158. Decisão
humana fixa ordem materialização. Sugestões:

### Cenário "baixo risco primeiro" (A → B → C → D):
- Validar pattern ADR-0080 Opção γ + extender cobertura
  Layout via cosméticos.
- Validar B.1 Auto track sizing fecha DEBT-34d preservado.
- Materializar C apenas após confiança em A+B.
- D último (reabre arquitectura single-pass).

### Cenário "alto valor primeiro" (D → C → A → B):
- Categoria D desbloqueia ADR-0066 IMPLEMENTADO +
  cobertura Introspection +33pp.
- Categoria C amplia cobertura semantic real (Place float
  + columns/colbreak real).
- A + B refinos finais.

### Cenário "selectivo":
- Scope-out parcial formal de C ou D se decisão humana
  posterior. ADR-0079 §"Critério de promoção" suporta:
  PROPOSTO → IMPLEMENTADO se A+B materializadas + C+D
  scope-out formal.

**Decisão humana fica em aberto literal** pós-P226.

---

## §10 Referências

- **ADR-0079** PROPOSTO (Layout Fase 5 roadmap; validação
  formal).
- **ADR-0080** PROPOSTO (L0 minimal para refactors
  aditivos pós-M9c N=7).
- **ADR-0066** PROPOSTO (Introspection runtime adiada;
  Categoria D promoção).
- **ADR-0078** IMPLEMENTADO (Column flow Opção B graded;
  Categoria C.2 não reabre literalmente — preserva +
  DEBT-56b novo).
- **ADR-0061** IMPLEMENTADO (Layout Fases 1+2+3+4
  candidata; preservada).
- **ADR-0054** Perfil graded (A+B preservam; C+D promovem
  além graded).
- **P156B** + **P215** — precedentes diagnósticos amplos
  Layout (paridade estrutural para P226).
- **DEBT-34d** preservado per `P224.div-1` (Categoria B.1
  fecha).
- **DEBT-37 §"Divergência"** fechada P223 (Categoria C
  não reabre).
- **DEBT-56** preservada CLOSED P221 (Categoria C.2
  introduz DEBT-56b novo).

---

## §11 Estado pós-P226

- **Diagnóstico amplo Fase 5** materializado em ficheiro
  dedicado.
- **ADR-0079 PROPOSTO** + **ADR-0080 PROPOSTO** criados.
- **13-15 sub-passos** identificados não-reservados.
- **3-4 reaberturas arquiteturais** registadas explicitamente.
- **Política "sem novas reservas" P158** preservada
  literal.
- **Trajectória aberta**: decisão humana caso-a-caso fixa
  ordem materialização A/B/C/D conforme prioridade.
