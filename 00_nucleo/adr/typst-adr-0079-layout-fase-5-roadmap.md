# ⚖️ ADR-0079: Layout Fase 5 roadmap — "completar Layout" (Tudo A+B+C+D)

**Status**: `PROPOSTO`
**Data**: 2026-05-13
**Autor**: Humano + IA
**Validado**: diagnóstico amplo P226
(`00_nucleo/diagnosticos/diagnostico-layout-fase-5-completar.md`);
decisão humana literal P225 §8 + P226 pré-spec "completar
Layout" escopo Tudo A+B+C+D.
**Diagnóstico prévio**:
[`diagnostico-layout-fase-5-completar.md`](../diagnosticos/diagnostico-layout-fase-5-completar.md)

---

## Contexto

Layout pós-P225 está em **estado terminal estructural
reconhecido oficialmente** (§3.0terdecies P225):
- Cobertura per metodologia §A.9: **89% real**.
- Distribuição §A.5: `12/4/2/0/0 = 18` (zero ausentes).
- ADR-0061 IMPLEMENTADO (Fases 1+2+3+4 candidata);
  ADR-0078 IMPLEMENTADO (column flow Opção B graded);
  ADR-0066 PROPOSTO (Introspection runtime adiada).
- DEBTs Layout: DEBT-56 ENCERRADA P221; DEBT-37
  §"Divergência" fechada P223; DEBT-34e ENCERRADO P224;
  **DEBT-34d preservado** per `P224.div-1`.

**Decisão humana literal P225 §8 + P226 pré-spec
pós-discussão**: "completar Layout" = **Tudo A+B+C+D**
incluindo reabertura ADR-0066 + arquitectura single-pass.

Diagnóstico amplo P226 identifica **13-15 sub-passos
cumulativos** cobrindo 4 categorias com magnitudes L+ a
XL cumulativas (~37-64h).

---

## Decisão

Materializar **13-15 sub-passos cumulativos** cobrindo 4
categorias A+B+C+D conforme diagnóstico amplo P226.
Roadmap identificado mas **NÃO reservado** per política
P158 — sub-passos materialização ficam abertos para
decisão humana caso-a-caso.

### Categorias e magnitudes:

| Categoria | Sub-passos | Magnitude | Reabertura |
|-----------|------------|-----------|------------|
| **A** Cosméticos | 5 (A.1-A.5) | M-L (~6-9h) | não |
| **B** Algorítmicos isolados | 3 (B.1-B.3) | M+ a L (~6-9h) | não |
| **C** Estruturais | 2 (C.1, C.2) | L+ a XL (~15-28h) | **sim** (C.1, C.2) |
| **D** Runtime queries | 5-6 (D.1-D.6) | L+ a XL (~10-18h) | **sim** (D categoria) |

---

## Reaberturas arquiteturais (registo explícito)

### Reabertura 1 — Opção B P219 graded (Categoria C.1)

- **Decisão original P219**: column flow Opção B graded
  (single-region width reduzida; pattern N=5 "Field
  armazenado semantic adiada").
- **C.1 materialização**: `Place float` real flow contorna
  (multi-pass layout ou flow secundário topo/fundo).
- **Nota**: Opção B P219 preservada literal para column
  flow; C.1 introduz flow ortogonal (Place float ≠ column
  flow). Sem conflito directo arquitectural.

### Reabertura 2 — P216B `Regions { current }` minimal (Categoria C.2)

- **Decisão original P216B**: `Regions` minimal
  (`{ current: Region }`); `backlog`/`last` diferidos.
- **C.2 materialização**: `Regions { current, backlog,
  last }` completo (multi-region real para columns/
  colbreak flow real).
- **Nota**: P216B preservada literal como baseline
  histórica; C.2 estende para Opção A real.

### Reabertura 3 — DEBT-56 ENCERRADA (Categoria C.2 derivada)

- **DEBT-56 P221**: ENCERRADA literal (CLOSED via
  materialização Opção B graded; critério 5/5 cumprido).
- **C.2 materialização**: introduz **DEBT-56b novo** para
  "refino Opção A multi-region pós-fecho DEBT-56".
- **Nota**: DEBT-56 preservada CLOSED literal; DEBT-56b
  novo aberto se/quando C.2 materializar (decisão diferida
  ao próprio C.2). Pattern emergente "fecho de DEBT
  preservado literal + criação de novo DEBT para refino
  pós-fecho" N=1 candidato.

### Reabertura 4 — ADR-0066 PROPOSTO (Categoria D)

- **Estado actual P226**: ADR-0066 PROPOSTO (Bloco C
  cross-módulo primeira materialização parcial via P222
  measure stdlib).
- **D.1 materialização**: `state(key, init)` runtime
  mutable; 3 condições §"Plano promoção" satisfeitas
  (state materializada; pipeline introspect extendido
  2-pass; tests E2E observable).
- **Promoção ADR-0066 PROPOSTO → IMPLEMENTADO** ocorre
  formalmente após D.1.
- **Nota**: arquitectura single-pass pré-existente
  preservada para D.6 e refinos não-runtime; D.1-D.5
  introduzem 2-pass para runtime features.

---

## Trade-off cumulativo

- **Magnitude cumulativa**: **L+ a XL (~37-64h em 13-15
  sub-passos)**.
- **Cobertura Layout pós-completar**: 89% → **100%
  literal** (todas 18 entradas §A.5 → impl puro ou impl⁺).
- **Cobertura Introspection bonus**: 17% → ~50%
  (Categoria D D.1-D.5).
- **Reaberturas arquiteturais**: **3-4 explícitas**
  (Opção B P219 + P216B + DEBT-56b + ADR-0066).
- **Risco arquitectural**: alto em C + D; baixo em A + B.
- **L0 actualização**: Opção γ paridade ADR-0080 para
  A + B (refactors aditivos); Opção α obrigatória para
  C + D (reaberturas decisões arquiteturais).

---

## Critério de promoção

ADR-0079 transita PROPOSTO → **IMPLEMENTADO** quando:

1. **Todos 13-15 sub-passos** identificados materializados.
2. **OU decisão humana de scope-out parcial formal**
   (e.g., categorias A+B materializadas; C+D scope-out
   formal por trade-off magnitude/risco; ADR-0079
   anotada com nota "Fase 5 mínima cumprida; C+D adiadas
   por scope-out formal").

ADR-0079 transita PROPOSTO → **REJEITADA** se:
- Decisão humana literal "abandonar completar Layout".

---

## Cross-references

- **ADR-0061** IMPLEMENTADO — Layout Fases 1+2+3+4
  candidata (preservada).
- **ADR-0078** IMPLEMENTADO — Column flow Opção B graded
  (não reabre literalmente; C.2 introduz DEBT-56b novo).
- **ADR-0066** PROPOSTO — Introspection runtime adiada
  (D reabre via promoção formal pós-D.1).
- **ADR-0080** PROPOSTO — L0 minimal para refactors
  aditivos pós-M9c N=7 (paridade A+B; Opção γ).
- **ADR-0054** — Perfil graded (A+B preservam; C+D
  promovem além graded).
- **P156B** + **P215** — precedentes diagnósticos amplos
  Layout.
- **P226** — diagnóstico amplo Fase 5 + ADR-0079 PROPOSTO
  + ADR-0080 PROPOSTO criados.
- **Diagnóstico amplo**:
  [`diagnostico-layout-fase-5-completar.md`](../diagnosticos/diagnostico-layout-fase-5-completar.md)
  — fonte de verdade matriz dependências + magnitudes.

---

## Próximos passos

Sub-passos identificados em diagnóstico amplo P226:

### Categoria A (cosméticos):
- A.1 stroke Grid + Table inheritance (S+ a M)
- A.2 fill Grid + Table (S+ a M)
- A.3 stroke/fill GridCell per-cell (M)
- A.4 Block/Boxed outset/radius/clip (M cada)
- A.5 Place per-cell alignment override (S+)

### Categoria B (algorítmicos isolados):
- B.1 DEBT-34d Auto track sizing fix (M)
- B.2 Consumer geometric integration P224.C (M)
- B.3 Per-cell GridCell atributos (M)

### Categoria C (estruturais reabrindo):
- C.1 Place float real flow contorna (L+)
- C.2 Opção A multi-region completa columns/colbreak (L+ a XL)

### Categoria D (runtime queries):
- D.1 state(key, init) runtime mutable (M) — desbloqueia
  promoção ADR-0066
- D.2 metadata(value) attaching (S+)
- D.3 here()/locate() location-aware (M)
- D.4 query(target) runtime introspection (M+)
- D.5 position(target) location-aware (S+)
- D.6 Cross-document cite refs (L+; ortogonal D)

Decisão humana fixa ordem materialização **caso-a-caso**.
Pattern P156C-J + P217-P220 + P222-P224 sugere ordem
"baixo risco → alto risco" (A → B → C → D); mas
dependências cross-passo podem alterar (D.1 desbloqueia
D.2-D.5; B.2 facilita C.2).

---

## Anotações futuras

ADR-0079 §"Aplicações cumulativas" será expandida em
cada sub-passo materializado (paridade ADR-0061 +
ADR-0078 que mantêm bloco anotações cumulativas
incremental por sub-passo). P226 é anotação inicial
PROPOSTO; sub-passos A+B+C+D materializarão e anotarão.

ADR-0079 §"Reservas pré-existentes" preservada vazia
(roadmap NÃO reservado per política P158; reservas
conceptuais identificadas mas não formalizadas como
ADRs ou DEBTs novos).

---

## Aplicações cumulativas (sub-passos materializados)

### P227 anotação — Categoria A sub-passo 1 (stroke Grid + Table)

**Data**: 2026-05-13.

**Primeiro sub-passo materialização Fase 5 Layout candidata**
— abertura formal pós-ADR-0079 PROPOSTO P226. Paridade
estrutural P217 (primeiro Fase 3) e P222 (primeiro Fase 4).

**A.1 materializado** (categoria A — cosméticos sem
reabrir decisões):
- **Grid +1 field** `stroke: Option<Stroke>` (8 → 9 fields
  cumulativos pós-P224).
- **Table +1 field** `stroke: Option<Stroke>` (3 → 4 fields;
  refino paralelo Grid).
- **`Value::Stroke(Stroke)` variant novo** — primeira adição
  ao enum Value pós-M9c (Value variants 54 → 55).
- **Helper `extract_stroke(val, fn, field)`** novo em
  `stdlib/layout.rs` (Opção β parsing Length/Color/Stroke
  shorthands paridade vanilla UX).
- **`native_stroke(paint:?, thickness:?)` constructor** em
  `stdlib/layout.rs` (~70 LOC; stdlib funcs 59 → 60).
- **`native_grid` + `native_table` accept `stroke:` named**
  via `extract_stroke`.
- **Renderização Opção β simplificada** em `layout_grid`:
  4 `FrameItem::Shape::Line` per cell border (top + bottom
  + left + right; sem deduplicação adjacentes; refino A.3
  candidato).

**6 decisões fixadas**:
- Decisão 1 — Opção α `Option<Stroke>` uniforme.
- Decisão 2 — Opção β parsing shorthands.
- Decisão 3 — `Value::Stroke` variant novo (audit C1
  confirmou ausência).
- Decisão 4 — `native_stroke` constructor paridade
  `native_rgb`.
- Decisão 5 — Opção β render simplificada.
- Decisão 6 — Table refino paralelo Grid.
- Decisão 7 — ADR-0080 NÃO promover EM VIGOR em P227
  (P228 candidato administrativo XS dedicado).

**Valida ADR-0080 PROPOSTO N=7 → 8** — primeira aplicação
real pós-formalização do pattern "L0 minimal para refactors
aditivos pós-M9c". L0 não tocado em P227. **Promoção
ADR-0080 PROPOSTO → EM VIGOR** candidato sólido P228
administrativo XS.

**Pattern emergente "refino aditivo paralelo entre variants
irmãos" N=1 inaugurado P227** (Grid + Table recebem mesmo
field paralelo; precedente futuro para A.2 fill + A.3
per-cell).

**Reuso patterns cumulativos**:
- `extract_length` reuso N=9 → 10 (patamar atingido;
  helper público candidato refino XS futuro).
- Pattern Smart→Option N=7 → 8.

**18 tests adicionados P227** (4 unit content + 11 unit
stdlib + 3 E2E layout). Workspace: 2039 → **2057 verdes**
(+18). 4 adaptações intencionais Grid/Table constructors
pre-existentes. 0 regressões reais. 0 violations.

**Categoria A Fase 5 Layout**: 1/5 sub-passos materializados
(A.1 stroke ✓; A.2 fill + A.3 per-cell + A.4 Block/Boxed
+ A.5 Place per-cell pendentes).

**Status ADR-0079 mantido PROPOSTO** (sub-passo 1/13-15
materialização Fase 5 candidata; promoção a IMPLEMENTADO
continua diferida até completar série α/β/γ/δ OU scope-out
parcial formal humano).

**Status ADR-0080 mantido PROPOSTO** (N=8 atingido mas
promoção EM VIGOR diferida P228 administrativo XS
candidato per política minimalista P158).

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P228 anotação — Categoria A sub-passo 2 (fill Grid + Table)

**Data**: 2026-05-13.

**Segundo sub-passo materialização Fase 5 Layout candidata
— paralelo estructural P227**:

**A.2 materializado**:
- **Grid +1 field** `fill: Option<Color>` (9 → 10 fields
  cumulativos pós-P227).
- **Table +1 field** `fill: Option<Color>` (4 → 5 fields).
- **Sem `Value::Fill` variant novo** — Color baseline P25
  reusado.
- **Sem `extract_color` helper** novo — inline match
  trivial em `native_grid`/`native_table` (Opção α
  parsing).
- **Sem constructor stdlib novo** — anti-inflação Decisão
  3 Opção γ (Color tem `native_rgb`/`native_luma`).
- **`native_grid` + `native_table` accept `fill:` named**
  via inline match (rejeita Length explicitamente —
  semantic fill é Color).
- **Renderização Opção β Z-order correcto** em
  `layout_grid`: 1 `FrameItem::Shape::Rect` per cell
  emitido **antes do conteúdo cell** (Z-order: fill →
  conteúdo → stroke). Audit C1 confirmou P227 stroke
  emitido após `for item in cell_items` (Z-order
  correcto preservado; sem `P228.div-N`).

**7 decisões fixadas**:
- Decisão 1 — Opção α `Option<Color>` uniforme.
- Decisão 2 — Opção α parsing trivial Color directo.
- Decisão 3 — Opção γ NÃO criar constructor stdlib
  (anti-inflação).
- Decisão 4 — Opção β Z-order correcto (fill antes;
  conteúdo meio; stroke depois).
- Decisão 5 — Tests E2E Z-order para validar interacção
  P227+P228.
- Decisão 6 — Opção γ L0 NÃO tocado validação ADR-0080
  N=8 → 9.
- Decisão 7 — ADR-0079 anotação Categoria A 2/5 (sem
  promoção).

**Valida ADR-0080 PROPOSTO N=8 → 9** — segunda aplicação
real pós-formalização do pattern "L0 minimal para
refactors aditivos pós-M9c". L0 não tocado em P228.
**Promoção ADR-0080 PROPOSTO → EM VIGOR** candidato P229
administrativo XS fortemente justificado (N=9 ultrapassa
critério N=8+).

**Pattern emergente "refino aditivo paralelo entre variants
irmãos" N=1 → 2 consolidado** (P227 stroke + P228 fill;
Grid + Table recebem mesmo field paralelo ambos
sub-passos).

**Pattern emergente "anti-inflação por aproveitamento de
tipos existentes" N=1 inaugurado P228** — distinto de
P227 onde Stroke composto justificou `native_stroke`.
Color primitivo dispensa constructor novo.

**Reuso patterns cumulativos**:
- Pattern Smart→Option N=8 → 9.
- `extract_length` reuso N=10 preservado (P228 não usa).

**14 tests adicionados P228** (4 unit content + 5 unit
stdlib + 5 E2E layout Z-order). Workspace: 2057 → **2071
verdes** (+14). 6 adaptações intencionais Grid/Table
constructors pre-existentes (adicionam `fill: None`).
0 regressões reais. 0 violations.

**Categoria A Fase 5 Layout**: 2/5 sub-passos
materializados (A.1 stroke ✓; **A.2 fill ✓**; A.3
per-cell + A.4 Block/Boxed + A.5 Place per-cell
pendentes).

**Status ADR-0079 mantido PROPOSTO** (sub-passo 2/13-15
materialização Fase 5 candidata).

**Status ADR-0080 mantido PROPOSTO** (N=9 atingido;
promoção EM VIGOR P229 candidato administrativo XS
muito sólido).

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P230 anotação — Categoria A sub-passo 3 (stroke/fill per-cell GridCell + TableCell; precedência override)

**Data**: 2026-05-13.

**Terceiro sub-passo materialização Fase 5 Layout candidata
— primeira aplicação automática ADR-0080 EM VIGOR pós-P229
promoção**:

**A.3 materializado**:
- **GridCell +2 fields** `stroke: Option<Stroke>` + `fill:
  Option<Color>` (5 → 7 fields cumulativos).
- **TableCell +2 fields** stroke + fill paralelo GridCell
  (5 → 7 fields; refino paralelo).
- **`native_grid_cell` + `native_table_cell` accept
  `stroke:` + `fill:` named args** via reuso helper
  `extract_stroke` P227 (N=1 → 2 cumulativo) + parsing
  inline Color paridade P228.
- **Renderização precedência override** em `layout_grid`:
  - `effective_stroke = cell.stroke.or(grid.stroke)`.
  - `effective_fill = cell.fill.or(grid.fill)`.
  - Per-cell `Some(...)` override Grid-level; per-cell
    `None` inherit Grid-level (paridade ADR-0033
    observable literal).
- **Z-order P227+P228 preservado**: fill efectivo atrás
  do conteúdo → conteúdo cell → stroke efectivo à frente.
- **Refactor pragmático sem `PlacedCell` expandido**:
  match no loop existente em `layout_grid` extrai
  per-cell stroke/fill direct. Consumer geometric
  integration `place_cells` continua B.2 candidato.

**8 decisões fixadas**:
- Decisão 1 — Opção α fields restritos (stroke + fill;
  align/inset/breakable per-cell são B.3 separado).
- Decisão 2 — Opção α precedência override completo via
  `.or()` resolution.
- Decisão 3 — Opção α Z-order limpo cada cell uma vez.
- Decisão 4 — Reuso helper `extract_stroke` N=1 → 2.
- Decisão 5 — Tests E2E precedência 5 explícitos.
- Decisão 6 — **Opção γ aplicação automática ADR-0080
  EM VIGOR** sem decisão explícita Opção γ por sub-passo.
- Decisão 7 — Opção α refino paralelo TableCell (pattern
  N=2 → 3 cumulativo).
- Decisão 8 — `extract_stroke` reuso N=1 → 2 (patamar
  trivial; sem promoção pública).

**Primeira aplicação automática ADR-0080 EM VIGOR
pós-promoção P229** — L0 não tocado sem decisão explícita
por sub-passo. Regra metodológica formal aplicada por
defeito. Pattern emergente "aplicação automática ADR EM
VIGOR sem decisão explícita por sub-passo" N=1
inaugurado P230.

**Pattern emergente "refino aditivo paralelo entre
variants irmãos" N=2 → 3 cumulativo** (Grid+Table
P227/P228; **GridCell+TableCell P230**). Pattern
consolida-se para cells estructurados.

**Pattern emergente "precedência per-cell vs
container-level via `.or()` resolution" N=1 inaugurado
P230** — reusável A.4 Block/Boxed (per-element vs
ancestor) + B.3 align/inset/breakable per-cell.

**Reuso patterns cumulativos**:
- Helper `extract_stroke` reuso N=1 → 2 (P227 cria; P230
  reusa primeiro). Patamar trivial; promoção pública
  diferida (paridade `extract_length` N=10 patamar).

**15 tests adicionados P230** (4 unit content + 6 unit
stdlib + 5 E2E precedência). Workspace: 2071 → **2086
verdes** (+15). Adaptações intencionais N=~10
(P224+P157B+grid_placement tests pre-existentes).
0 regressões reais. 0 violations.

**Categoria A Fase 5 Layout**: 3/5 sub-passos
materializados (A.1 stroke ✓; A.2 fill ✓; **A.3
per-cell ✓**; A.4 Block/Boxed + A.5 Place per-cell
pendentes).

**Status ADR-0079 mantido PROPOSTO** (sub-passo 3/13-15
materialização Fase 5 candidata; promoção a IMPLEMENTADO
continua diferida).

**Status ADR-0080 mantido EM VIGOR** — primeira aplicação
automática pós-promoção P229.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

---

## Status

**`PROPOSTO`** — autorização arquitectural concedida em
princípio; materialização caso-a-caso fica em aberto.
**Política "sem novas reservas" P158 preservada literal**
— 13-15 sub-passos identificados mas NÃO reservados.
Sub-passo materializado pós-P226: **1** (P227 A.1).
