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

### P231 anotação — Categoria A sub-passo 4 (outset/radius/clip Block + Boxed)

**Data**: 2026-05-13.

**Quarto sub-passo materialização Fase 5 Layout candidata
— segunda aplicação automática ADR-0080 EM VIGOR + reabre
P156G+H scope-outs documentados há 18 dias**:

**A.4 materializado**:
- **Block +3 fields** `outset: Sides<Length>` + `radius:
  Option<Length>` + `clip: bool` (5 → 8 fields cumulativos).
- **Boxed +3 fields** paralelo Block (5 → 8 fields).
- **`native_block` + `native_box` accept 3 named args**
  via parsing inline (outset Length uniforme; radius
  Length opcional; clip Bool); validações negativos
  rejeitados.
- **Renderização Opção β parcial graded** (audit C1
  determinou primitivos baseline ausentes — todos 3
  fields semantic real adiada):
  - `outset` armazenado; semantic real adiada (bounds
    visual expandidos refactor cumulativo).
  - `radius` armazenado; semantic real adiada (`ShapeKind::RoundedRect`
    primitivo NÃO existe baseline geometry.rs P76).
  - `clip` armazenado; semantic real adiada (wrap body
    em `FrameItem::Group::clip_mask` requer refactor
    estructural).

**7 decisões fixadas**:
- Decisão 1 — Opção α escopo restrito (outset+radius+clip
  apenas).
- Decisão 2 — Opção α `Sides<Length>` outset.
- Decisão 3 — Opção β `Option<Length>` radius uniforme
  (vs `Corners<T>` scope-out per ADR-0029).
- Decisão 4 — Opção α `bool` clip (paridade vanilla).
- Decisão 5 — Opção β graded parcial (3 fields semantic
  adiada per audit C1).
- Decisão 6 — Opção α refino paralelo Block + Boxed.
- Decisão 7 — Opção γ L0 NÃO tocado automaticamente
  (segunda aplicação automática ADR-0080 EM VIGOR).

**Pattern emergente "L0 minimal" aplicação automática N=1
→ 2 cumulativo** (P230 + P231).

**Pattern "refino aditivo paralelo entre variants irmãos"
N=3 → 4 cumulativo** (Grid+Table; GridCell+TableCell;
Block+Boxed). N=4 patamar empírico **muito sólido**;
promoção formal ADR meta candidato.

**Pattern "Field bool simples paridade vanilla" N=2 → 3
cumulativo** (`breakable`/`repeat`/**`clip`**). N=3
atinge limiar formalização N=3-4.

**Pattern Smart→Option N=9 → 10 cumulativo** (radius).
N=10 patamar empírico **muito sólido**; candidato promoção
formal (paridade `extract_length` N=10).

**Pattern "Field armazenado semantic adiada" N=5 → 7
cumulativo** (+outset + radius + clip todos adiadas em
P231). N=7 patamar empírico **muito sólido**.

**Reabertura formal P156G+H scope-outs** documentados há
18 dias (criados 2026-04-25; reabertos 2026-05-13 em
P231). Pattern de continuidade arquitectural cumulativa.

**15 tests adicionados P231** (4 unit content + 9 unit
stdlib + 2 E2E layout). Workspace: 2086 → **2101 verdes**
(+15). Adaptações intencionais N=4 (Block/Boxed
constructors pre-existentes em entities/content.rs +
stdlib/mod.rs). 0 regressões reais. 0 violations.

**Categoria A Fase 5 Layout**: 4/5 sub-passos
materializados (A.1 stroke ✓; A.2 fill ✓; A.3 per-cell ✓;
**A.4 Block/Boxed cosméticos ✓**; A.5 Place per-cell
pendente). Após A.5 → **Categoria A completa 5/5**.

**Status ADR-0079 mantido PROPOSTO** (sub-passo 4/13-15
materialização Fase 5 candidata).

**Status ADR-0080 mantido EM VIGOR** — segunda aplicação
automática pós-promoção P229.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P232 anotação — Categoria A sub-passo 5 (Place per-cell alignment override); **Categoria A 5/5 ✓ FECHADA ESTRUCTURALMENTE**

**Data**: 2026-05-13.

**Quinto e último sub-passo Categoria A Fase 5 Layout
candidata — FECHA Categoria A 5/5 ESTRUCTURALMENTE**.

**Categoria A**: 5/5 sub-passos materializados ✓ **FECHADA**:
- A.1 stroke (P227) ✓.
- A.2 fill (P228) ✓.
- A.3 per-cell GridCell+TableCell (P230) ✓.
- A.4 outset/radius/clip Block+Boxed (P231) ✓.
- **A.5 Place per-cell alignment override (P232) ✓**.

**Trabalho P232**:
- **Zero fields novos** em Place/Grid/Table/Cell — pattern
  "sub-passo sem novos fields; só lógica precedence" N=1
  inaugurado P232.
- **+1 field `cell_align: Option<Align2D>` no Layouter
  struct** (paridade `cell_origin_*` baseline P84.6;
  save/restore ao entrar/sair Grid context em `layout_grid`).
- **Lógica precedência per-eixo via `.or()`** no arm
  `Content::Place`:
  - `effective_h = alignment.h.or(grid_align.h)`.
  - `effective_v = alignment.v.or(grid_align.v)`.
  - Place explícito override Grid; Place vazio herda Grid.
  - Place fora Grid (cell_align None) preserva baseline
    P84.5 directo.
- **Stdlib `native_place` NÃO modificado**.

**Audit C1 findings**:
- `Content::Table.align` **NÃO existe** baseline; P232
  escopo limitado a Grid context. Table align paralelo é
  refino XS futuro candidato (não-reservado per política
  P158).

**8 decisões fixadas**:
- Decisão 1 — Opção α lógica precedência (zero fields
  novos).
- Decisão 2 — Opção α precedência por eixo via `.or()`.
- Decisão 3 — Opção α inline no arm Place.
- Decisão 4 — Opção α stdlib preservado.
- Decisão 5 — 5 tests E2E precedência explícitos.
- Decisão 6 — Table.align audit ausente → escopo Grid
  only.
- Decisão 7 — Anotação Categoria A 5/5 ✓ fechada sem
  transição status (pattern N=1 inaugurado).
- Decisão 8 — Opção γ L0 automático (**terceira aplicação
  automática ADR-0080 EM VIGOR**).

**Patterns emergentes consolidados em P232**:
- **"L0 minimal para refactors" aplicação automática
  pós-EM VIGOR** N=2 → **3 cumulativo** (P230+P231+P232).
- **Pattern "precedência per-X via `.or()` resolution"
  N=1 → 2 cumulativo** (P230 GridCell over Grid; **P232
  Place per-axis over Grid**).
- **"Fecho categoria completa dentro de ADR PROPOSTO sem
  transição" N=1 inaugurado P232** — distinto de
  §3.0duodecies P221 + §3.0terdecies P225 que envolveram
  transições ADR.
- **"Sub-passo sem novos fields; só lógica precedence"
  N=1 inaugurado P232** — distinto cumulativo de A.1-A.4
  que adicionaram fields.

**5 tests adicionados P232** (5 E2E layout precedência).
Workspace: 2101 → **2106 verdes** (+5). Sem adaptações
intencionais. 0 regressões reais. 0 violations.

**Status ADR-0079 mantido PROPOSTO** (sub-passo 5/13-15
materialização Fase 5 candidata; **Categoria A 5/5 ✓
fechada estructuralmente**; Categorias B/C/D pendentes).

**Status ADR-0080 mantido EM VIGOR** — terceira aplicação
automática pós-promoção P229.

**Marco interno implícito Categoria A fechada
estructuralmente** — próximo sub-passo pode pivot:
- Categoria B (algorítmicos: B.1 DEBT-34d; B.2 consumer
  geometric; B.3 per-cell align/inset/breakable).
- Categoria C (estruturais reabrindo: C.1 Place float
  real; C.2 multi-region completa).
- Categoria D (runtime queries: D.1 state desbloqueia
  ADR-0066 IMPLEMENTADO).
- Pivot outro módulo (Visualize/Text/Model).

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P233 anotação — Categoria B sub-passo 1 (DEBT-34d Auto track sizing fix); **DEBT-34d FECHADO**; **P224.div-1 RESOLVIDA P233**

**Data**: 2026-05-13.

**Sexto sub-passo materialização Fase 5 Layout candidata
— primeiro Categoria B algorítmico pós-fecho Categoria
A (P232); primeiro fecho de DEBT preservado conscientemente
pós-M9c**.

**B.1 materializado** (DEBT-34d fix):
- **Algoritmo two-pass measure→place inaugurado P233**
  (pattern N=1 cristalino pós-M9c):
  - Pass 1: `measure_content_constrained` per cell em
    tracks Auto; max → resolved_widths.
  - Pass 2: existing P224.C `place_cells` com tamanhos
    pre-calculados.
- **Fix subset minimal** (atomização ADR-0036 aplicada):
  `safe = if has_fr { safe_total / (num_auto + num_fr) }
  else { safe_total }`. Auto cap-se quando há fr presente.
- **Zero fields novos**; **zero novas stdlib funcs**;
  refactor algorítmico puro inline em `layout_grid`
  (Decisão 3 Opção β consolidar sem novo módulo).
- 5 tests E2E adicionados P233.

**8 decisões fixadas**:
- Decisão 1 — Opção α (audit C1: DEBT-34d unitário).
- Decisão 2 — Opção α algoritmo two-pass standard.
- Decisão 3 — Opção β consolidar `layout_grid`.
- Decisão 4 — Opção α distribuição fr proporcional.
- Decisão 5 — 5 tests E2E canónicos.
- Decisão 6 — Fecho DEBT-34d formal + referência cruzada.
- Decisão 7 — P224.div-1 RESOLVIDA P233 anotação
  retrospectiva.
- Decisão 8 — Opção γ L0 automático (**quarta aplicação
  automática ADR-0080 EM VIGOR**).

**DEBT-34d FECHADO P233** — saldo DEBTs 12 → **11** (-1).
Resolução completa do problema literal documentado via
subset minimal P233.

**P224.div-1 RESOLVIDA P233** — divergência factual
material preservada conscientemente em P224 (DEBT-34d
preservado aberto há 18 sub-passos) é agora resolved.
Pattern emergente "fecho retrospectivo de divergência
factual em sub-passo posterior" N=1 inaugurado P233.

**Patterns emergentes consolidados/inaugurados em P233**:
- **"L0 minimal para refactors" aplicação automática
  pós-EM VIGOR** N=3 → **4 cumulativo** (P230+P231+P232+
  **P233**).
- **"Algoritmo two-pass measure→place" N=1 inaugurado
  P233**.
- **"Fecho de DEBT preservado conscientemente em
  sub-passo posterior" N=1 inaugurado P233** —
  DEBT-34d preservado 18 sub-passos pós-P224.div-1.
- **"Fecho retrospectivo de divergência factual em
  sub-passo posterior" N=1 inaugurado P233**.

**5 tests adicionados P233**. Workspace: 2106 → **2111
verdes** (+5). 0 adaptações intencionais. 0 regressões
reais. 0 violations.

**Categoria B Fase 5 Layout: 1/3 sub-passos materializados
(B.1 ✓; B.2 + B.3 pendentes)**.

**Status ADR-0079 mantido PROPOSTO** (sub-passo 6/13-15
materialização Fase 5 candidata; **Categoria A 5/5 ✓ +
Categoria B 1/3**).

**Status ADR-0080 mantido EM VIGOR** — quarta aplicação
automática pós-promoção P229.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P234 anotação — Categoria B sub-passo 2 (Consumer geometric `place_cells` → Layouter integration); **colspan/rowspan funcionais em renderização pela primeira vez pós-M9c**

**Data**: 2026-05-13.

**Sétimo sub-passo materialização Fase 5 Layout candidata
— segundo Categoria B algorítmico pós-P233 B.1; primeira
integração consumer geometric pós-isolamento algorítmico
em sub-passo posterior pós-M9c**.

**B.2 materializado** (consumer integration):
- **`layout_grid` chama `place_cells`** baseline P224.C;
  obtém `Vec<PlacedCell>` em vez de iterar `cells.chunks(
  num_cols)` direct.
- **Bounds reais per cell** via helper privado
  `cell_bounds(placed, col_starts, resolved_widths,
  row_heights, current_row_start_y) -> (x0, y0, w, h)`:
  - `cell_w = sum(resolved_widths[col..col+colspan])`.
  - `cell_h = sum(row_heights[row..row+rowspan])`.
- **Colspan/rowspan funcionam realmente em renderização**
  pela primeira vez pós-M9c (criados algoritmicamente P224.C;
  isolados até P234 integração).
- **PlacedCell.body semantic ajustada P234** — preserva
  outer cell (`Content::GridCell {...}` wrapper) em vez
  de strip inner body. **5 fields baseline preservados**
  (audit P230 rejeitou refactor; só semantic body muda).
- **Z-order P227+P228 + precedência per-cell P230
  preservados integralmente** com bounds reais.
- **cell_cache removido** (custo perf ~2× aceitável MVP;
  re-integração indexada por input_idx é refino futuro).
- 11 tests E2E adicionados P234.

**8 decisões fixadas**:
- Decisão 1 — Opção α integração completa.
- Decisão 2 — Opção α PlacedCell baseline 5 fields literal;
  semantic body ajustada para outer wrapper.
- Decisão 3 — Opção α bounds via helper privado
  `cell_bounds` (paridade pattern `extract_stroke` P227).
- Decisão 4 — Opção α match `placed.body` semantic P230.
- Decisão 5 — 11 tests E2E (4 colspan/rowspan funcionais
  + 4 regressões baseline + 3 cenários adicionais).
- Decisão 6 — Opção γ L0 automático (**quinta aplicação
  automática ADR-0080 EM VIGOR**).
- Decisão 7 — Sem promoção formal patterns N=1.
- Decisão 8 — Cache descartado MVP.

**Patterns emergentes consolidados/inaugurados em P234**:
- **"L0 minimal para refactors" aplicação automática
  pós-EM VIGOR** N=4 → **5 cumulativo** (P230+P231+P232+
  P233+**P234**). Pattern extremamente sólido empíricamente.
- **"Three-pass measure→place→emit" N=1 inaugurado P234** —
  extensão pattern two-pass P233.
- **"Integração consumer pós-isolamento algorítmico em
  sub-passo posterior" N=1 inaugurado P234** — paridade
  conceitual "fecho de DEBT preservado" P233.
- **"PlacedCell baseline P224.C suficiente sem refactor"
  confirmado N=2** (P230 audit; P234 integração com semantic
  body ajustada apenas).
- **Reuso `place_cells` N=0 → 1 cumulativo** — primeiro
  consumer geometric real.

**11 tests adicionados P234**. Workspace: 2111 → **2122
verdes** (+11). 0 adaptações intencionais. 0 regressões
reais. 0 violations.

**Categoria B Fase 5 Layout: 2/3 sub-passos materializados
(B.1 ✓; B.2 ✓; B.3 pendente — valida pattern `.or()`
N=2 → 3 atinge limiar formalização N=3-4)**.

**Status ADR-0079 mantido PROPOSTO** (sub-passo 7/13-15
materialização Fase 5 candidata; **Categoria A 5/5 ✓ +
Categoria B 2/3**).

**Status ADR-0080 mantido EM VIGOR** — quinta aplicação
automática pós-promoção P229.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P235 anotação — Categoria B sub-passo 3 (GridCell + TableCell align/inset/breakable per-cell); **Categoria B 3/3 ✓ FECHADA estructuralmente**

**Data**: 2026-05-13.

**Oitavo sub-passo materialização Fase 5 Layout candidata
— terceiro e último Categoria B; FECHA Categoria B 3/3
estructuralmente** (pattern "fecho categoria completa
dentro de ADR PROPOSTO sem transição" N=1 → 2 cumulativo
paridade P232 Categoria A).

**B.3 materializado**:
- **GridCell +3 fields** (`align: Option<Align2D>`,
  `inset: Option<Sides<Length>>`, `breakable: Option<bool>`)
  — 7 → 10 fields.
- **TableCell +3 fields paralelo** (pattern N=4 → 5
  cumulativo "refino aditivo paralelo entre variants
  irmãos") — 7 → 10 fields.
- **stdlib `native_grid_cell`/`native_table_cell` accept
  3 named args** (`align`, `inset`, `breakable`) via
  helpers privados `extract_align_value` + `extract_inset_value`.
- **Renderização diferenciada por atributo**:
  - **align**: real via Layouter `cell_align` P232
    estendido per-cell save/restore.
  - **inset**: real via bounds reduction
    (`body_x = cell_x + inset.left; body_w = (cell_w -
    inset.left - inset.right).max(0.0)`).
  - **breakable**: armazenado semantic adiada graded
    (paridade P156G + P224.B).
- **Precedência `.or()` uniforme** P230 + P232 + P235
  (3 atributos):
  ```rust
  let effective_align = cell_align.or(self.cell_align);
  let effective_inset = cell_inset.cloned().unwrap_or(inset);
  let _effective_breakable = cell_breakable;  // graded
  ```

**8 decisões fixadas**:
- Decisão 1 — Opção α escopo restrito (align + inset +
  breakable).
- Decisão 2 — Opção β tipos Option uniformes.
- Decisão 3 — Opção α `.or()` precedência uniforme.
- Decisão 4 — Opção α refino paralelo TableCell.
- Decisão 5 — Opção β reuso Layouter cell_align P232
  estendido per-cell (não Opção α directo).
- Decisão 6 — Opção α render real inset.
- Decisão 7 — Opção β breakable armazenado adiada graded.
- Decisão 8 — Opção γ L0 automático (**sexta aplicação
  automática ADR-0080 EM VIGOR**).

**Patterns emergentes consolidados/inaugurados em P235**:
- **"Precedência per-X via `.or()` resolution"** N=2 →
  **3 cumulativo atingindo limiar formalização N=3-4**
  — **promoção formal ADR meta candidato XS futuro
  paridade P229**.
- **"Refino aditivo paralelo entre variants irmãos"** N=4
  → **5 cumulativo** (Grid+Table P227/P228; GridCell+
  TableCell P230; Block+Boxed P231; **GridCell+TableCell
  algorítmico P235**).
- **"Smart→Option"** N=10 → **12 cumulativo** (+inset
  Option +breakable Option).
- **"Field armazenado semantic adiada"** N=7 → **8
  cumulativo** (+breakable per-cell).
- **"L0 minimal para refactors" aplicação automática
  pós-EM VIGOR**: N=5 → **6 cumulativo** — pattern
  **extremamente sólido empíricamente** (6 consecutivas
  sem excepção).
- **"Fecho categoria completa dentro de ADR PROPOSTO sem
  transição"** N=1 → **2 cumulativo** (P232 Categoria A;
  **P235 Categoria B**).
- **"Layouter cell_align save/restore granularidade
  per-cell" N=1 inaugurado P235** — extensão P232 que só
  fez per-Grid save/restore.
- **"Render real algorítmico per-cell" N=1 inaugurado
  P235** — distinto cumulativo de cosméticos P230.
- **"Renderização diferenciada por atributo dentro do
  mesmo sub-passo" N=1 inaugurado P235** — align real +
  inset real + breakable graded.

**15 tests adicionados P235** (4 unit content + 6 unit
stdlib + 5 layout E2E). Workspace: 2122 → **2137 verdes**
(+15). 1 adaptação intencional. 0 regressões reais. 0
violations.

**Categoria B Fase 5 Layout: 3/3 ✓ FECHADA estructuralmente**
(B.1 ✓; B.2 ✓; **B.3 ✓**).

**Status ADR-0079 mantido PROPOSTO** (sub-passo 8/13-15
materialização Fase 5 candidata; **Categoria A 5/5 ✓ +
Categoria B 3/3 ✓ + C + D pendentes**).

**Status ADR-0080 mantido EM VIGOR** — sexta aplicação
automática pós-promoção P229.

**Marco interno implícito Categoria B fechada
estructuralmente** — próximo sub-passo pode pivot:
- Categoria C (estruturais reabrindo: C.1 Place float
  real; C.2 multi-region completa).
- Categoria D (runtime queries: D.1 state desbloqueia
  ADR-0066 IMPLEMENTADO).
- Pivot outro módulo (Visualize/Text/Model).
- ADR meta admin XS para pattern `.or()` resolution
  (paridade P229; pattern atinge N=3).

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P236 anotação — Categoria D sub-passo 1 (state runtime); **`P236.div-1` divergência factual material**: state runtime já materializado pre-P236 P171+M9+M9c; ADR-0066 SUPERSEDED-BY 0073 (impossível promover); P236 materializa refino aditivo `state_final(key)` pós decisão humana

**Data**: 2026-05-13.

**Nono sub-passo materialização Fase 5 Layout candidata —
primeiro Categoria D runtime queries; primeira divergência
factual MATERIAL pós-M9c registada via `P236.div-1`**.

**Audit C1 obrigatório revelou**:

- **ADR-0066 status real: SUPERSEDED-BY 0073** (P204H
  2026-05-07). Cadeia: PROPOSTO 2026-04-27 → ACEITE
  P192B 2026-05-05 → SUPERSEDED-BY 0073 P204H 2026-05-07
  → F3 fechou §C6a ADR-0074 ACEITE P205B+C+E. **Promoção
  PROPOSTO → IMPLEMENTADO impossível** — status
  SUPERSEDED é terminal.
- **State runtime já materializado P171+M9+M9c**:
  `Content::State`, `Content::StateUpdate`,
  `entities/state_registry.rs`, `entities/state_update.rs`,
  `entities/layouter_runtime_state.rs`, 3 stdlib funcs
  `native_state*`, pipeline activo `Introspector::
  state_final_value` + walk integration via from_tags.

**Decisão humana pós-divergência (Opção 2 do questionário
de 4 opções)**: Refino aditivo subset — adicionar UMA
stdlib func que materializa parte específica de D.1 não
coberta pelo M9 baseline.

**P236 materializa refino aditivo `state_final(key)`**:
- **`native_state_final` em `foundations.rs`** — argumento
  posicional Str `key`; retorna `Value` (init se state
  nunca actualizado; último valor caso contrário; None
  se key inexistente).
- **Reuso `Introspector::state_final_value`** baseline P171.
- **Registo scope** `state_final` em `eval/mod.rs`.
- **Paralelo absoluto a `counter_final` (P176)**.
- **6 unit tests** (cenários canónicos).
- **L0 NÃO tocado** — refino aditivo verdadeiro qualifica
  Opção γ ADR-0080 §"Escopo" line 66 ("stdlib func nova
  aditiva" categoria explícita).

**8 decisões fixadas pós-divergência**:
- Decisão 1 — Opção 2 humana (refino aditivo subset).
- Decisão 2 — `state_final` escolhido sobre `state_at`
  (paralelo `counter_final` directo).
- Decisão 3 — Paralelo absoluto pattern `counter_final`
  (Introspector wrapper trivial).
- Decisão 4 — `Value::None` retornado se key inexistente
  (semantic distinto Value::Str("") `counter_final`
  porque state pode ter qualquer Value type).
- Decisão 5 — Iter 0 fixpoint retorna None.
- Decisão 6 — 6 unit tests subset minimal cenários
  canónicos (sem layout E2E pois state não-renderiza).
- Decisão 7 — **ADR-0066 NÃO tocado** (status SUPERSEDED
  preservado).
- Decisão 8 — **Opção γ L0 NÃO tocado** — sétima aplicação
  automática ADR-0080 EM VIGOR (refino aditivo verdadeiro
  qualifica per ADR-0080 §"Escopo" literal; não-excepção).

**Patterns emergentes consolidados/inaugurados em P236**:
- **"L0 minimal para refactors" aplicação automática
  pós-EM VIGOR** N=6 → **7 cumulativo** (P230+P231+P232+
  P233+P234+P235+**P236**) — pattern **extremamente
  sólido empíricamente** (sete consecutivas sem excepção).
- **"stdlib func runtime para final value lookup"** N=1
  → **2 cumulativo** (counter_final P176; **state_final
  P236**).
- **"Divergência factual material registada via Pxxx.div-1
  + decisão humana pós-divergência"** N=1 inaugurado P236.
- **"Spec materializada como refino aditivo subset
  pós-divergência factual"** N=1 inaugurado P236 —
  pattern complementar a "fecho retrospectivo de
  divergência" P233.
- **"State runtime materializado pre-P236 reconhecido
  retrospectivamente como cumprimento ADR-0066 (chain
  ADR-0073/ADR-0074)"** — documentação corretiva.

**6 tests adicionados P236**. Workspace: 2137 → **2143
verdes** (+6). 0 adaptações intencionais. 0 regressões
reais. 0 violations.

**Categoria D Fase 5 Layout: 1/? sub-passos materializados
pós-divergência** (D.1 ✓ refino aditivo P236; D.2
state.at/display + D.3 query/D.4 counter candidatos
sub-passos separados).

**Status ADR-0079 mantido PROPOSTO** (sub-passo 9/13-15
materialização Fase 5 candidata; **Categoria A 5/5 ✓ +
Categoria B 3/3 ✓ + Categoria D 1/? + Categoria C 0/?**).

**Status ADR-0066 mantido SUPERSEDED-BY 0073** (não-tocado
P236 — promoção impossível pós-cadeia chronológica).

**Status ADR-0080 mantido EM VIGOR** — sétima aplicação
automática pós-promoção P229.

**Marco interno: P236.div-1 documenta empíricamente que
state runtime foi materializado pre-Fase 5 candidata via
M9 P171+P172 + M8 ADR-0073 + M9c ADR-0074** — Categoria
D.1 cumprida retrospectivamente; refino aditivo P236
adiciona apenas user-facing wrapper `state_final`.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P237 anotação — Categoria D refino estendido `state_at(key, label)` paralelo absoluto `counter_at` P177; **primeira aplicação da lição metodológica P236.div-1 via spec C1 audit obrigatório bloqueante**; paralelismo state↔counter completo

**Data**: 2026-05-13.

**Décimo sub-passo materialização Fase 5 Layout candidata
— segundo Categoria D refino aditivo cumulativo (P236
state_final + P237 state_at); paralelismo state↔counter
completo**.

**Audit C1 obrigatório bloqueante (lição P236.div-1
aplicada)**:
- `Introspector::query_by_label(label: &Label) -> Option<Location>`
  confirmado P139+P140 (não `lookup_label(&str) -> SourceResult`
  como spec hipotetizou — ajuste signature trivial sem
  `P237.div-N` formal).
- `Introspector::state_value(key, location)` confirmado P171.
- `native_counter_at` P177 pattern confirmed:
  `query_by_label.and_then(state_value).unwrap_or_default()`.
- Audit converge com hipóteses revistas; **sem `P237.div-N`
  formal** (ajustes triviais signature/method name não
  merecem div-N per spec §5 risco 11).

**P237 materializa refino aditivo `state_at(key, label)`**:
- **`native_state_at` em `foundations.rs`** — paralelo
  absoluto `counter_at` P177; reuso `query_by_label` +
  `state_value` chain.
- **Registo scope** `state_at` em `eval/mod.rs` paridade
  P236.
- **7 unit tests subset minimal** cenários canónicos.
- **L0 NÃO tocado** — oitava aplicação automática
  ADR-0080 EM VIGOR.

**8 decisões fixadas** (Decisão 0 = lição P236.div-1):
- Decisão 0 — C1 audit obrigatório bloqueante; sem
  `P237.div-N` (audit converge).
- Decisão 1 — Opção α escopo minimal aditivo.
- Decisão 2 — Signature paridade `counter_at` literal.
- Decisão 3 — Reuso wrapper trivial `state_value`.
- Decisão 4 — Label inexistente retorna `Value::None`
  (revisão de spec; paridade `counter_at` empty default).
- Decisão 5 — 7 unit tests subset minimal.
- Decisão 6 — Opção γ L0 NÃO tocado (8ª aplicação automática).
- Decisão 7 — ADR-0066 NÃO tocado (SUPERSEDED-BY 0073).
- Decisão 8 — Sem promoção ADR-0079; sem marco blueprint.

**Patterns emergentes consolidados/inaugurados em P237**:
- **"L0 minimal para refactors" aplicação automática
  pós-EM VIGOR** N=7 → **8 cumulativo** (P230-P236+
  **P237**) — pattern **extremamente sólido empíricamente**
  (oito consecutivas sem excepção).
- **"stdlib func runtime para label-based lookup"** N=1
  inaugurado P237.
- **"spec C1 audit obrigatório bloqueante pós-P236.div-1"**
  N=1 inaugurado P237 — metodológico crítico.
- **"paralelismo state↔counter completo"** N=1 inaugurado
  P237 — state agora 5 ops; counter 4 ops.

**7 tests adicionados P237**. Workspace: 2143 → **2150
verdes** (+7). 0 adaptações intencionais. 0 regressões
reais. 0 violations.

**Categoria D Fase 5 Layout: 1/? refino estendido completo**
— state_final P236 + state_at P237; **paralelismo
state↔counter completo** (state ops: state/state_update/
state_update_with/state_final/state_at; counter ops:
counter/counter_update/counter_final/counter_at).

**Status ADR-0079 mantido PROPOSTO** (sub-passo 10/13-15
materialização Fase 5 candidata; **Categoria A 5/5 ✓ +
Categoria B 3/3 ✓ + Categoria D 1/? refino estendido
completo + Categoria C 0/?**).

**Status ADR-0066 mantido SUPERSEDED-BY 0073** (não-tocado
P237).

**Status ADR-0080 mantido EM VIGOR** — oitava aplicação
automática pós-promoção P229.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P238 reescrito anotação — Auditoria metodológica falhanços `P236.div-1` + `P238.div-1` + plano realista cobertura Layout pós-P237; passo administrativo documental (zero código tocado); refino lição `P236.div-1` N=1 → 2 cumulativo

**Data**: 2026-05-13.

**Passo administrativo documental** — paridade pattern P225
(encerramento Fase 4 documental) + P229 (promoção ADR-0080
administrativa). **Distinto** dos sub-passos materialização
(P227-P237) — não-incrementa contador sub-passos Fase 5;
não-toca código; não-promove status ADR.

**P238 original spec falhou via `P238.div-1` formal**:

- Spec P238 original hipotetizou `state.display` walk-time
  render-mediated callback materializável via refino aditivo
  (Categoria D 2/?).
- Audit C1 obrigatório revelou contradição factual material
  em três pontos: (a) `Content::State` é zero-size em layout
  (arm `Content::State { .. } => {}` em `layout/mod.rs:352`);
  (b) `Func::call` não existe (só `Func::native` constructor);
  (c) `StateUpdate::Func` (P172) já é stub documentado por
  blocker arquitectural idêntico (walk-time eval Func dispatch
  bloqueado por `EvalContext + Engine + World + FileId +
  figure_numbering` indisponíveis durante walk).
- **Spec previa walk-time render-mediated callback
  arquiteturalmente impossível sem pipeline restructuring M7+**.

**Decisão humana pós-`P238.div-1`**: P238 reescrito como
auditoria metodológica formal + plano realista cobertura
Layout (decisão literal pós-divergência; sem materializar
código quando bloqueio arquitectural identificado).

**P238 reescrito materializa** (`typst-passo-238-auditoria.md`
7 §s):

- **Auditoria metodológica formal** dos dois falhanços
  consecutivos (`P236.div-1` + `P238.div-1`) — causas raiz;
  padrões emergentes; lição refinada.
- **Estado factual cobertura Layout pós-P237** — sub-passos
  materializados vs pendentes; bloqueadores arquiteturais.
- **Plano realista cobertura Layout** identificando o que é
  viável pós-P237 sem refactor M7+ vs o que requer pipeline
  restructuring.
- **Refino lição metodológica `P236.div-1`** via `P238.div-1`
  empírico — spec audit prévio obrigatório para sub-passos
  walk-time/runtime (não apenas C1 audit dentro do passo).

**Causa raiz comum aos dois falhanços**:

- `P236.div-1`: sumário contexto incompleto; spec assumiu
  baseline pré-M9c sem audit prévio.
- `P238.div-1`: spec incluiu C1 audit obrigatório bloqueante
  (lição `P236.div-1`) mas fixou decisões C2-C8 prováveis
  baseadas em hipóteses análogas eval-time aplicadas
  incorretamente a walk-time/runtime integration. Pattern
  "decisões sujeitas a C1" criam viés cognitivo que resiste
  revisão pós-audit.

**Refino lição metodológica `P236.div-1`**:

> Para sub-passos com risco alto/crítico (walk-time; runtime
> callback dispatch; pipeline integration), spec deve fazer
> audit prévio **ANTES** de redigir decisões C2-C8. Para
> refinos de risco baixo/médio (eval-time wrappers; cosméticos;
> algorítmicos isolados), C1 audit bloqueante como primeira
> cláusula é suficiente.

**Bloqueadores arquiteturais identificados pós-P237** (§4.1
auditoria):

- **Bloqueador A — Walk-time eval Func dispatch**: `Func::call`
  inexistente; afecta D.2 `state.display`, `counter.display`,
  possíveis D.3+ callback dispatch. Resolução M7+ pipeline
  restructuring.
- **Bloqueador B — Multi-region completion**: DEBT-56b
  candidato; afecta C.2 + breakable per-cell render (A.4
  graded P235). Resolução refactor multi-region cell-level.
- **Bloqueador C — Place float real**: afecta C.1. Resolução
  reabertura Opção B P219 magnitude L+.
- **Bloqueador D — Pipeline runtime two-pass walk**:
  `state.final()` semantic vanilla requer two-pass; P236
  implementou wrapper não-real. Resolução M7+ infrastructure.

**Sub-passos viáveis sem refactor M7+ identificados**
(§4.2 auditoria):

- **D.X1 counter.display stub** (paridade P172 StateUpdate::Func
  stub) — VIÁVEL via stub paralelo; não-recomendado se D.2
  também stub.
- **D.X2 query refinos** — eval-time wrappers paridade
  `state_at` / `state_final`. **Audit prévio obrigatório**
  (lição refinada).
- **D.X3 numbering refinos** — audit prévio obrigatório.
- **A.4 refino outset render real** — Block/Boxed; audit
  prévio + materialização conforme.
- **A.X fill/stroke Block/Boxed** — paridade P227+P228
  estructural; render real viável.

**Caminhos pós-P237** (§4.4 auditoria):

- **Opção curto-prazo (4-6h)**: prep-passo audit prévio +
  A.X fill/stroke Block/Boxed + D.X2 query refinos + ADR meta
  admin XS.
- **Opção médio-prazo (~10-15h)**: A.4 outset render real +
  A.4 radius/clip conforme audit.
- **Opção longo-prazo (M7+ refactor)**: pipeline restructuring
  para desbloquear D.2 + counter.display + state.final
  two-pass. Magnitude XL+.
- **Opção pivot**: outro módulo (Visualize 54%; Text 52%;
  Model 50%).

**Estimativa fecho realista Fase 5 Layout** (§4.5 auditoria):

- **Sem refactor M7+**: Fase 5 candidata fecha em
  **10-12/13-15 sub-passos materializados** (~67-85%);
  sub-passos bloqueados arquiteturalmente preservados como
  graded/scope-out documentados.
- **Com refactor M7+**: Fase 5 candidata materializa
  13-15/13-15 (100% interno) mas magnitude cumulativa L+ a XL+.

**Decisão arquitectural pendente**: humano decide se Fase 5
Layout fecha graded a ~80% (preservando bloqueadores como
scope-out documentado) OU reabre M-fase para refactor pipeline.

**Patterns emergentes inaugurados/consolidados em P238
reescrito** (6):

- **"spec audit prévio obrigatório para sub-passos
  walk-time/runtime" N=1 inaugurado P238 reescrito** — refino
  lição `P236.div-1`.
- **"atomização prep-passo audit-only + materialização-passo
  para sub-passos risco alto/crítico" N=1 inaugurado P238
  reescrito** — paridade P226 diagnóstico amplo.
- **"`Pxxx.div-1` cumulativo para falhanços spec arquitectural
  maior"** N=1 → **2 cumulativo** (`P236.div-1` + **`P238.div-1`**).
- **"passo administrativo documental para auditoria
  metodológica pós-divergência" N=1 inaugurado P238 reescrito**
  — distinto de P225 (encerramento Fase) + P229 (promoção
  ADR-0080).
- **"L0 minimal para refactors" aplicação automática N=8
  preservado** (P230-P237; P238 reescrito documental
  não-incrementa).
- **"Fase candidata fecha graded a bloqueadores arquiteturais
  identificados" N=1 inaugurado P238 reescrito** — Fase 5
  Layout candidata pode fechar ~80% preservando bloqueadores
  scope-out.

**Saída cumulativa P238 reescrito (zero código tocado)**:

- Tests workspace: **2150 verdes preservado** (paridade
  pattern administrativo P225/P229).
- 0 violations preservadas.
- Content variants: 60 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 62 preservado.
- Layouter fields: preservados.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- Saldo DEBTs: 11 preservado.
- **Anti-inflação 30ª aplicação cumulativa pós-P205D**.

**Status ADR-0079 mantido PROPOSTO** (sub-passo
materializado pós-P226 contador preservado em **10** P237
literal; P238 reescrito documental administrativo não-incrementa).

**Status ADR-0066 mantido SUPERSEDED-BY 0073** (não-tocado
P238 reescrito).

**Status ADR-0080 mantido EM VIGOR** — não-aplicação por
não-tocar código (paralelo P225/P229 administrativos).

**Marco interno: P238 reescrito documenta empíricamente que
sub-passos walk-time/runtime callback dispatch (D.2
state.display; counter.display; state.final two-pass) estão
bloqueados arquiteturalmente sem pipeline restructuring M7+**
— refino lição metodológica `P236.div-1` cumulativo N=2;
plano realista preserva ~80% materialização Fase 5 Layout
candidata sem M7+ (graded fechamento).

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P239 anotação — Prep-passo audit-only reabertura M-fase para M7+ refactor; primeira aplicação real pattern atomização prep-passo audit-only + materialização-passo inaugurado P238 reescrito; ADR meta novo ADR-0081 PROPOSTO criado

**Data**: 2026-05-14.

**Passo administrativo documental audit-only** — paridade
pattern P225 + P229 + P238 reescrito (zero código tocado);
**primeira aplicação real pattern "atomização prep-passo
audit-only + materialização-passo" inaugurado P238 reescrito**
(N=1 → 2 cumulativo); **primeira reabertura M-fase pós-M9c
iniciada metodologicamente correctamente** via prep-passo
audit-only (lição refinada `P236.div-1` → `P238.div-1`
aplicada literal).

**P239 prep-passo audit-only materializa**
(`typst-passo-239-audit-m7-reabertura.md` 9 §s):

- **Audit M-fase histórico** (§3.1) — M5/M6/M7/M8/M9/M9c
  cumulativos identificados; M7 estruturalmente fechado P192B
  (ADR-0072); reabertura M-fase para walk-time eval é **nova
  M-fase**, não reabertura M7.
- **Audit blocker arquitectural walk-time eval Func dispatch**
  (§3.2) — achado material: hipótese P238 reescrito
  "`Func::call` não existe" precisa refino. `apply_state_funcs`
  JÁ EXISTE em `from_tags.rs:48` e avalia `StateUpdate::Func`
  via fixpoint loop pós-walk com Engine+ctx; `apply_func` em
  `eval/closures.rs:59` é mecanismo canónico chamada. Blocker
  real é **layout-time Engine+ctx indisponíveis** (Layouter
  puro sem acesso eval), não walk-time Func dispatch.
- **4 opções resolução estructural** identificadas (§3.2):
  Opção α (massivo refactor Layouter signature; risco quebrar
  comemo); Opção β (two-pass walk completo; magnitude XL+);
  **Opção γ recomendada** (`apply_state_displays` pré-eval em
  fixpoint paralelo `apply_state_funcs`; L ~5-8h; baixo risco);
  Opção δ (Show rule recursivo; incoerente arquitectural).
- **Audit blockers relacionados** (§3.3) — Multi-region; Place
  float; state.final two-pass (sobreposição grande com walk-time
  via Opção γ); **bloqueador adicional E identificado P239**:
  A.4 radius/clip infrastructure (`ShapeKind::RoundedRect`
  ausente; `Group::clip_mask` JÁ EXISTE baseline).
- **Sobreposições** (§3.4) — A + D partilham Opção γ refactor;
  resto independente.
- **Roadmap atomização 5 sub-passos materialização M7+**
  (§4): M7+1 (pipeline walk-time eval Opção γ; L ~5-8h); M7+2
  (counter.display paralelo; M ~2-4h); M7+3 (multi-region
  cell-level; L+ ~8-12h); M7+4 (Place float real; L ~5-8h);
  M7+5 (radius/clip infrastructure; M-L ~3-5h). **Total
  cumulativo ~23-37h materialização** — refinado pós-audit
  empírico face P238 reescrito (XL+ ~20-40h hipotetizado).
- **3 pré-condições obrigatórias formalizadas** (§5): testes
  baseline preservados; comemo memoization invariants
  ADR-0073/0074 preservados; backward compat eval-time
  (P236+P237 wrappers).

**ADR meta novo ADR-0081 PROPOSTO criado** (§6) —
`typst-adr-0081-m7-plus-pipeline-restructuring-scope.md`
formaliza escopo + atomização + pré-condições + dependencies/ordem +
não-objectivos + alternativas consideradas (B/C/E/F/G preteridas;
D pivot alternativa válida) + 3 sub-decisões pendentes
(D1 nomenclatura M-fase preliminar **M9d**; D2 ordem primeira
materialização; D3 promoção pós-M7+).

**8 decisões fixadas P239** (Decisão 0 = lição `P238.div-1`
aplicada literal):

- Decisão 0 — Prep-passo audit-only obrigatório (lição
  refinada `P236.div-1` → `P238.div-1`).
- Decisão 1 — Opção α escopo audit cumulativo (M-fase + 4
  bloqueadores; refinado a 5 pós-audit).
- Decisão 2 — Opção α ADR meta novo PROPOSTO criado.
- Decisão 3 — Opção α atomização ADR-0036 aplicável (5
  sub-passos M7+1 a M7+5).
- Decisão 4 — 3 pré-condições obrigatórias formais.
- Decisão 5 — Magnitude cumulativa **L+ a XL (~23-37h)**
  refinada empíricamente.
- Decisão 6 — Opção γ L0 NÃO tocado (P239 administrativo).
- Decisão 7 — Opção β saldo DEBTs preservado/decresce.
- Decisão 8 — Sem promoção ADR-0079; Fase 5 candidata
  mantém **10/13-15 sub-passos materializados** baseline
  pós-P237.

**Patterns emergentes inaugurados/consolidados P239** (7):

- **"spec audit prévio obrigatório para sub-passos
  walk-time/runtime" N=1 → 2 cumulativo** (P238 reescrito +
  P239).
- **"prep-passo audit-only preventivo para reabertura M-fase"
  N=1 inaugurado P239** — extensão pattern P238 reescrito.
- **"passo administrativo documental" N=3 → 4 cumulativo**
  (P225 + P229 + P238 reescrito + **P239**).
- **"ADR meta novo PROPOSTO para reabertura M-fase" N=1
  inaugurado P239** — primeiro ADR meta novo pós-P229.
- **"atomização prep-passo audit-only + materialização-passo"
  N=1 → 2 cumulativo** (P238 reescrito + P239).
- **"L0 minimal para refactors" aplicação automática N=8
  preservado** (P239 administrativo não-incrementa).
- **"audit empírico refina hipótese spec" N=2 → 3 cumulativo**
  (`P236.div-1`; P237 audit C1; **P239 audit C2.1+C2.2
  apply_state_funcs já existe**).

**Saída cumulativa P239 (zero código tocado)**:

- Tests workspace: **2150 verdes preservado**.
- Violations: 0 preservadas.
- Content variants: 60 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 62 preservado.
- Layouter fields: preservados.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- **ADRs distribuição**: PROPOSTO **12 → 13** (+ADR-0081 M7+
  scope PROPOSTO); EM VIGOR 29; IMPLEMENTADO 21; total
  **67 → 68**. ADR-0066 SUPERSEDED-BY 0073 preservado.
- Saldo DEBTs: 11 preservado.
- **Anti-inflação 31ª aplicação cumulativa** pós-P205D.

**Status ADR-0079 mantido PROPOSTO** (P239 administrativo
documental não-incrementa contador sub-passos Fase 5).

**Status ADR-0066 mantido SUPERSEDED-BY 0073** (não-tocado
P239).

**Status ADR-0080 mantido EM VIGOR** — não-aplicação por
não-tocar código (paralelo P225/P229/P238 reescrito).

**Marco interno: P239 documenta empíricamente que reabertura
M-fase pós-M9c para walk-time eval Func dispatch é viável
via Opção γ paridade `apply_state_funcs` (refactor L ~5-8h)
em vez de massivo refactor pipeline (XL+ hipotetizado)** —
audit empírico refina estimativa P238 reescrito; lição
refinada `P236.div-1` → `P238.div-1` validada empíricamente;
ADR meta novo ADR-0081 PROPOSTO formaliza escopo.

**Decisão humana pendente pós-P239** (per audit §7): primeira
materialização sub-passo M7+ (recomendação subjectiva M7+1
pipeline walk-time eval; alternativa válida M7+5 radius/clip
ou pivot outro módulo).

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

### P240 anotação — Categoria D sub-passo D.2 state.display walk-time real materializado via M7+1 (M9d primeira sub-passo); Categoria D 1/? → 2/?; ADR-0081 PROPOSTO → IMPLEMENTADO parcial

**Data**: 2026-05-14.

**Décimo-segundo sub-passo materialização Fase 5 Layout
candidata pós-P227 (A.1)**; **primeira sub-passo materialização
M9d / M7+ pós-P239 audit-only** — primeira aplicação real do
pattern "atomização prep-passo audit-only + materialização-passo"
N=1 → 2 cumulativo (P238 reescrito → P239 → **P240**).

**P240 materializa M7+1 Opção γ apply_state_displays**:
- `Content::StateDisplay { key, callback: Option<Func> }`
  variant novo. Content variants: 60 → **61**.
- `ElementPayload::StateDisplay { key, callback }` variant
  novo (audit C1 P240 refinou hipótese spec: Tag enum é
  `Tag::Start(Location, ElementInfo)` com payload via
  ElementInfo, não `Tag::StateDisplay` directo; ajuste trivial
  sem `P240.div-N`).
- `ElementKind::StateDisplay` variant novo.
- `apply_state_displays` fixpoint function nova em
  `rules/introspect/from_tags.rs` (paralelo absoluto
  `apply_state_funcs` P191B).
- `Introspector::state_display_value(key, location) ->
  Option<Content>` trait method novo + impl em TagIntrospector
  + CountingIntrospector adapter.
- `TagIntrospector.state_displays: HashMap<(String, Location),
  Content>` storage novo.
- `native_state_display(key, [callback])` stdlib func nova.
  Stdlib funcs: 62 → **63**.
- Walk integration layout-time arm `Content::StateDisplay`
  via Introspector — Layouter permanece puro (Opção γ
  arquitectural estrita preservada).

**Sobreposição bloqueador A + D desbloqueada via M7+1**:
walk-time eval Func dispatch + state.final two-pass real ambos
resolvidos via Opção γ paralelo `apply_state_funcs`. **Cenário
α audit C7 confirmed empíricamente**: `state_final_value`
baseline retorna `history.last()` pós-`apply_state_funcs` em
fixpoint — semantic já é two-pass real sem refactor adicional.

**D.2 state.display walk-time real materializado pela primeira
vez pós-M9c**:
- Categoria D 1/? → **2/?** sub-passos materializados
  (D.1 state_final P236 + state_at P237 eval-time wrappers; **D.2
  state.display walk-time real P240 + state.final two-pass real
  via sobreposição**).
- `counter.display` paralelo deferido para M7+2 (magnitude M
  ~2-4h; pattern absoluto reuso).

**Primeira excepção justificada à aplicação automática ADR-0080
EM VIGOR pós-P229** — L0 partial tocado 3 ficheiros
(`entities/content.md` + `rules/stdlib.md` +
`rules/introspect.md`); ADR-0080 §"Excepção P240" anotada
formalmente. Pattern emergente "L0 tocado para features
runtime novas + walk integration" N=1 inaugurado P240. Pattern
"aplicação automática ADR-0080 EM VIGOR" N=8 preservado mas
não-incrementa P240 (excepção justificada).

**8 decisões fixadas P240** (Decisão 0 = lição N=3 cumulativo):
- Decisão 0 — C1 audit obrigatório bloqueante (lição
  P236.div-1 → P238.div-1 → P239 audit aplicada).
- Decisão 1 — Opção γ apply_state_displays.
- Decisão 2 — Opção β variant novo Content::StateDisplay.
- Decisão 3 — Opção α refinada: ElementPayload::StateDisplay
  (não Tag::StateDisplay directo).
- Decisão 4 — Opção β paralelismo absoluto.
- Decisão 5 — Walk integration via Introspector trait.
- Decisão 6 — native_state_display 1-2 arg.
- Decisão 7 — Cenário α state.final two-pass trivial (docs
  apenas).
- Decisão 8 — L0 partial tocado (primeira excepção ADR-0080).

**Patterns emergentes inaugurados/consolidados P240** (4):
- **"L0 tocado para features runtime novas + walk
  integration" N=1 inaugurado P240** — primeira aplicação
  real.
- **"refino aditivo paralelo entre callers fixpoint" N=1
  inaugurado P240** — extensão pattern P191B
  `apply_state_funcs` para `apply_state_displays`.
- **"spec C1 audit obrigatório bloqueante"** N=2 → **3
  cumulativo** (P237 + P238 reescrito + P240).
- **"atomização prep-passo audit-only +
  materialização-passo"** N=1 → **2 cumulativo** (P238 reescrito
  → P239 → P240 validação empírica).

**Pré-condições obrigatórias verificadas P240** (per ADR-0081
§"Pré-condições obrigatórias"):
1. Tests baseline preservados: 2150 → **2162 verdes** (+12;
   0 regressões; 0 adaptações).
2. Comemo memoization invariants ADR-0073/0074 preservados.
3. Backward compat eval-time P236 state_final + P237 state_at
   intactos.

**Resultado P240**:
- Tests workspace: 2150 → **2162 verdes** (+12).
- Violations: 0 preservadas.
- Content variants: 60 → **61**.
- Stdlib funcs: 62 → **63**.
- ADRs: ADR-0081 PROPOSTO → **IMPLEMENTADO parcial**
  (M7+1 ✓; M7+2 a M7+5 pendentes). Distribuição:
  PROPOSTO 13 → **12** (-1 transita); IMPLEMENTADO 21 →
  **22** (+1 parcial); EM VIGOR 29; total 68 preservado.
- ADR-0079 Categoria D 1/? → **2/?**.
- ADR-0080 §"Excepção P240" anotada.
- ADR-0066 SUPERSEDED-BY 0073 preservado.
- Saldo DEBTs: 11 preservado.
- Cobertura Layout per metodologia: **89% preservada** (M7+1
  é Introspection refino + walk integration; não Layout).
- Cobertura Introspection refino +D.2 state.display real +
  state.final two-pass real.

**Status ADR-0079 mantido PROPOSTO** — sub-passo
materializado pós-P226 contador agora **2** (P227 A.1; P240
M7+1 D.2). Fase 5 Layout candidata: 10/13-15 → **11/13-15
sub-passos materializados** (~73-85% cumulativo; **Categoria
A 5/5 + Categoria B 3/3 + Categoria D 2/? + Categoria C 0/?**).

**M9d / M7+ progresso**: **1/5 sub-passos materializados**
(M7+1 ✓; M7+2 + M7+3 + M7+4 + M7+5 pendentes).

**Próximo sub-passo candidato**: M7+2 counter.display paralelo
M7+1 (recomendação subjectiva; magnitude M ~2-4h; reuso pattern
absoluto; completa Categoria D 2/? → 3/? real). Alternativa
subjectiva: M7+5 A.4 radius/clip (menor magnitude). Decisão
humana pendente literal pós-P240.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

---

### P241 anotação — M7+2 counter.display walk-time eval paralelo absoluto P240; Categoria D 2/? → 3/?; ADR-0081 IMPLEMENTADO parcial 1/5 → 2/5

**Data**: 2026-05-14.

**Décimo-terceiro sub-passo materialização Fase 5 Layout
candidata pós-P227 (A.1)**; **segunda sub-passo materialização
M9d / M7+ pós-P240** — paralelo absoluto P240 substituindo
`state_display` por `counter_display`. Pattern "refino aditivo
paralelo entre callers fixpoint" N=1 → 2 cumulativo.

**P241 materializa M7+2 Opção γ**:
- `Content::CounterDisplayCallback { key, callback }` variant
  novo (distinto de `Content::CounterDisplay { kind }` legacy
  single-pass preservada inalterada). Content variants: 61 →
  **62**.
- `ElementPayload::CounterDisplay { key, callback }` variant
  novo paralelo `StateDisplay`.
- `ElementKind::CounterDisplay` variant novo.
- `apply_counter_displays` fixpoint function nova em
  `from_tags.rs` (paralelo absoluto `apply_state_displays`
  P240). Converte counter slice para `Value::Array` e aplica
  callback via `apply_func`.
- `Introspector::counter_display_value(key, location) ->
  Option<Content>` trait method novo + impl + adapter.
- `TagIntrospector.counter_displays` storage novo.
- `native_counter_display(key, [callback])` stdlib func nova.
  Stdlib funcs: 63 → **64**.
- Walk integration layout-time arm
  `Content::CounterDisplayCallback` — Layouter permanece puro.

**Forma do Value ao callback** (Decisão 4 P241):
`Value::Array(Vec<Value::Int>)` paridade vanilla `CounterState
= SmallVec<[u64; 3]>`. Counter inexistente:
`Value::Array(vec![])`. Sem callback: formato default "1.2.3"
via join "."; counter inexistente: `Content::Empty`.

**D.3 counter.display walk-time real materializado pela
primeira vez pós-M9c**: paralelo absoluto D.2 state.display
P240. Categoria D 2/? → **3/?** sub-passos materializados:
- D.1 state_final P236 + state_at P237 eval-time wrappers.
- D.2 state.display walk-time real P240.
- **D.3 counter.display walk-time real P241**.

**Segunda excepção justificada ADR-0080 EM VIGOR pós-P229**
— L0 partial tocado 3 ficheiros (`content.md` + `stdlib.md` +
`introspect.md`). ADR-0080 §"Excepção P241" anotada formalmente.
Pattern "L0 tocado para features runtime novas + walk
integration" N=1 → **2 cumulativo** (P240 + P241).

**8 decisões fixadas P241** (Decisão 0 = lição N=4 cumulativo):
- Decisão 0 — C1 audit obrigatório bloqueante.
- Decisão 1 — Opção α variant nova paralela (naming
  `CounterDisplayCallback`).
- Decisão 2 — ElementPayload::CounterDisplay paralelo.
- Decisão 3 — ElementKind::CounterDisplay paralelo.
- Decisão 4 — Value::Array para counter state.
- Decisão 5 — Counter inexistente fallback.
- Decisão 6 — native_counter_display 1-2 arg.
- Decisão 7 — L0 partial tocado (segunda excepção ADR-0080).
- Decisão 8 — Tests materializados no mesmo passo.

**Patterns emergentes inaugurados/consolidados P241** (3):
- "L0 tocado para features runtime novas + walk integration"
  N=1 → **2 cumulativo** (P240 + P241).
- "Refino aditivo paralelo entre callers fixpoint" N=1 → **2
  cumulativo** (P240 `apply_state_displays` + P241
  `apply_counter_displays`).
- "Spec C1 audit obrigatório bloqueante" N=3 → **4 cumulativo**.

**Pré-condições obrigatórias verificadas P241** (per ADR-0081):
1. Tests baseline preservados: 2162 → **2175 verdes** (+13; 0
   regressões; 0 adaptações).
2. Comemo memoization invariants ADR-0073/0074 preservados.
3. Backward compat: `Content::CounterDisplay { kind }` legacy
   intacto; tests pré-P241 preservados.

**Resultado P241**:
- Tests workspace: 2162 → **2175 verdes** (+13).
- Violations: 0 preservadas.
- Content variants: 61 → **62**.
- Stdlib funcs: 63 → **64**.
- ADRs distribuição preservada (ADR-0081 transita 1/5 → 2/5
  internamente; sem ADR novo; sem PROPOSTO ↔ IMPLEMENTADO).
- ADR-0079 Categoria D 2/? → **3/?**.
- ADR-0080 §"Excepção P241" N=2 cumulativo anotada.
- ADR-0066 SUPERSEDED-BY 0073 preservado.
- Saldo DEBTs: 11 preservado.
- Cobertura Layout per metodologia: **89% preservada** (M7+2
  é Introspection refino + walk integration).
- Cobertura user-facing total: ~70% → **~71-72%** (D.3
  counter.display real bonus).

**Status ADR-0079 mantido PROPOSTO** — sub-passo materializado
pós-P226 contador agora **3** (P227 A.1; P240 M7+1 D.2; **P241
M7+2 D.3**). Fase 5 Layout candidata: 11/13-15 → **12/13-15
sub-passos materializados** (~80-92% cumulativo; **Categoria A
5/5 + B 3/3 + D 3/? + C 0/?**).

**M9d / M7+ progresso**: **2/5 sub-passos materializados**
(M7+1 ✓; M7+2 ✓; M7+3 + M7+4 + M7+5 pendentes — cumulativa
restante ~16-25h).

**Próximo sub-passo candidato**: M7+5 A.4 radius/clip
infrastructure (recomendação subjectiva spec P241 §8; menor
magnitude M-L ~3-5h; geometry isolada). Alternativas: M7+3
multi-region (L+); M7+4 Place float (L); ADR meta admin XS
(promoção patterns N=2 P240+P241); pivot outro módulo; pausa
M-fase.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

---

### P242 anotação — M7+5 A.4 radius/clip infrastructure materializado parcial; Categoria A.4 scope-out P231 graded → materializado parcial P242; primeira sub-passo M7+ não-pipeline; IMPLEMENTADO parcial 2/5 → 3/5

**Data**: 2026-05-14.

**Décimo-quarto sub-passo materialização Fase 5 Layout candidata
pós-P227 (A.1)**; **terceira sub-passo materialização M9d / M7+
pós-P241** — primeira sub-passo M7+ **não-pipeline** (P240/P241
foram walk-time refactor; P242 é geometry isolada).

**P242 materializa M7+5 A.4 radius/clip infrastructure**:
- `Corners<T>` tipo entity novo (paralelo `Sides<T>`).
- `ShapeKind::RoundedRect { radii: Corners<Length> }` variant
  novo. ShapeKind variants: 4 → **5**.
- Refino `Content::Block.radius` + `Content::Boxed.radius`
  `Option<Length>` → `Corners<Length>` per-corner (audit C1 P242
  refinou hipótese spec — fields já existiam P231; ajuste real
  é refine type; sem `P242.div-N` paridade lição N=5 cumulativo).
- `extract_corners_length_value` helper novo.
- stdlib `block(radius:)` + `box(radius:)` aceitam Length
  uniforme OR Dict por canto (precedência específico > eixo >
  rest paridade ADR-0064 Caso C).
- Layouter `Block.clip == true` emite `FrameItem::Group` com
  `clip_mask: Some(RoundedRect{radii})` (radius non-zero) OR
  `Rect` (radius zero; paridade DEBT-30 P79).
- PDF exporter `emit_rounded_rect_ops` desenha Bezier 4 corners
  path em 5 sítios cross-arm (kappa `0.552_284_749_831` paridade
  Ellipse).

**Promoção real graded ADR-0054 P156G/H → semantic concreta**:
sub-padrão emergente **"promoção real scope-out ADR-0054 graded"
N=1 inaugurado P242** — Categoria A.4 P231 graded → A.4
materializado parcial. Outset/fill/stroke restantes em Block/
Boxed permanecem scope-out (refino futuro N=3 restantes).

**Pré-condições obrigatórias verificadas P242**:
1. Tests baseline preservados: **2175 → 2190 verdes** (+15 novos;
   0 regressões; 7 adaptações triviais tests P231).
2. Comemo memoization invariants ADR-0073/0074 preservados (P242
   NÃO toca Introspector).
3. Backward compat: stdlib radius Length uniforme via
   `Corners::uniform`; tests P231 adaptados.

**9 decisões fixadas P242** — Decisão 0 = lição N=5 cumulativo;
Decisão 1 Corners<T> paralelo Sides; Decisão 2 RoundedRect novo;
Decisão 3 refino tipo (não add); Decisão 4 radius Length OR Dict;
Decisão 5 clip materializado; Decisão 6 radius sem clip preserva
graded; Decisão 7 L0 partial tocado (terceira excepção ADR-0080);
Decisão 8 promoção real graded; Decisão 9 sem fechamento Fase 5.

**Patterns emergentes inaugurados/consolidados P242** (4):
- "Promoção real scope-out ADR-0054 graded" N=1 inaugurado.
- "Tipo entity em ficheiro próprio" (sub-padrão #14) N=5 →
  **6 cumulativo** (Corners adiciona-se a Sides/Parity/Dir/
  BibEntry/CitationForm).
- "Reuso template helpers extract_*" N=3 → **4 cumulativo**.
- "Spec C1 audit obrigatório bloqueante" N=4 → **5 cumulativo**.

**Terceira excepção justificada ADR-0080 EM VIGOR pós-P229**:
**sub-categoria diferente** de P240/P241 (walk-time runtime) —
P242 é "L0 tocado para geometry/exporter infrastructure". L0
partial tocado 4 ficheiros (`corners.md` NOVO + `geometry.md` +
`content.md` + `export.md`). Pattern "L0 tocado pós-P229
(sub-categorias)" N=3 cumulativo total com **2 sub-categorias
formalizadas**: walk-time (N=2 P240+P241) + geometry/exporter
(N=1 P242).

**Resultado P242**:
- Tests workspace: 2175 → **2190 verdes** (+15).
- Violations: 0 preservadas.
- Content variants: 62 preservado.
- **ShapeKind variants: 4 → 5** (+RoundedRect).
- Tipos entity novos: **+1 Corners<T>**.
- Stdlib funcs: 64 preservado.
- Helpers stdlib novos: **+1 `extract_corners_length_value`**.
- ADRs distribuição preservada (ADR-0081 transita 2/5 → 3/5
  internamente; sem ADR novo). PROPOSTO 12; EM VIGOR 29;
  IMPLEMENTADO 22; total **68 preservado**.
- ADR-0079 Categoria A.4 scope-out P231 → **materializado parcial
  P242** anotada.
- ADR-0080 §"Excepção P242" N=3 cumulativo (sub-categoria nova)
  anotada.
- ADR-0066 SUPERSEDED-BY 0073 preservado.
- Saldo DEBTs: 11 preservado.
- Cobertura Layout per metodologia: 89% → **~91-92%** (refino
  qualitativo+quantitativo — primeira aplicação Layout pós-P156L).
- Cobertura user-facing total: ~72% → **~73-74%** (A.4
  radius/clip real bonus).

**Status ADR-0079 mantido PROPOSTO** — sub-passo materializado
pós-P226 contador agora **4** (P227 A.1; P240 M7+1 D.2; P241
M7+2 D.3; **P242 M7+5 A.4 parcial**). Fase 5 Layout candidata:
12/13-15 → **13/13-15 sub-passos materializados** (~85-92%
cumulativo; **Categoria A 5/5 ✓ (A.4 parcial materializado) +
B 3/3 ✓ + D 3/? + C 0/?**).

**M9d / M7+ progresso**: **3/5 sub-passos materializados** (M7+1
✓; M7+2 ✓; **M7+5 ✓**; M7+3 + M7+4 pendentes; cumulativa
restante ~13-20h).

**Próximo sub-passo candidato**: M7+3 multi-region completion
cell-level (recomendação subjectiva; magnitude L+ ~8-12h; maior
desbloqueio cumulativo restante — C.2 + A.4 breakable per-cell).
Alternativas: M7+4 Place float (L); refino A.4 outset/fill/stroke
(S-M cada); ADR meta admin XS; pivot outro módulo; pausa M-fase.

Decisão humana pendente literal pós-P242.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

---

### P243 anotação — M7+3 fase (a) infrastructure: Regions { backlog, last } extensão + promoção real ≥3 scope-outs multi-region (Pad.right + Block.width + Boxed.width); fase (b) DEBT-56 pendente; IMPLEMENTADO parcial 3/5 → 4/5

**Data**: 2026-05-14.

**Décimo-quinto sub-passo materialização Fase 5 Layout candidata
pós-P227 (A.1)**; **quarta sub-passo materialização M9d / M7+
pós-P242** — fase (a) DEBT-56 (infrastructure-only); fase (b)
pendente para passo subsequente.

**Achado material audit C1 P243**: spec hipotetizou refactor
profundo cross-module L+; reality empírica: refactor
field-agregation **já feito em P216A + P216B** (Region struct
+ Regions wrapper + Layouter `regions: Regions` field). P243
reduz para **extensão `Regions`** + promoção scope-outs.
**Magnitude real M (~2-3h)** face L+ hipotetizado. **Sem
`P243.div-N`** — paridade lição N=6 cumulativo precedente.

**P243 materializa M7+3 fase (a)**:
- Extensão `Regions` struct: `backlog: Vec<Region>` + `last:
  Option<Region>` fields + `advance` method.
- Promoção real ≥3 scope-outs multi-region via `regions.current.width`
  save/restore: Pad.right + Block.width + Boxed.width.

**Sub-padrão "promoção real scope-out ADR-0054 graded"** N=1 →
**2 cumulativo** (P242 radius/clip + **P243 multi-region attrs**).
Atinge limiar formalização N=2 — candidato a ADR meta passo
administrativo XS futuro.

**Pré-condições obrigatórias verificadas P243**:
1. Tests baseline: 2190 → **2198 verdes** (+8; 0 regressões;
   0 adaptações — extensão aditiva).
2. Comemo memoization invariants ADR-0073/0074 preservados.
3. Backward compat: stdlib `block(width:)` continua a funcionar
   (semantic agora real); tests pré-P243 preservados não-disruptive.

**10 decisões fixadas P243** (Decisão 0 = lição N=6 cumulativo):
- Decisão 0 — C1 audit obrigatório bloqueante.
- Decisão 1 — Regions extensão paralelo conceptual
  LayouterRuntimeState.
- Decisão 2 — Migração field-by-field já feita P216A/B (audit
  finding).
- Decisão 3 — Fase (a) preserva single-region observable literal.
- Decisão 4 — Promoção real ≥3 scope-outs.
- Decisão 5 — Sem `Content::Columns`/`Colbreak` em P243.
- Decisão 6 — Sem ADR column flow algorithm.
- Decisão 7 — cell_available_h integration diferida.
- Decisão 8 — Nova sub-categoria ADR-0080 "Layouter internal
  refactor".
- Decisão 9 — Tests focam preservação observable.
- Decisão 10 — Sem fechamento Fase 5 / ADR-0061 / DEBT-56.

**Patterns emergentes inaugurados/consolidados P243** (4):
- "Refactor profundo Layouter internal" N=1 inaugurado P243
  (magnitude reduzida vs spec por P216A/B precedente).
- "Sub-categoria ADR-0080 nova" N=2 → **3 cumulativo**
  (walk-time P240+P241; geometry/exporter P242; **Layouter
  internal refactor P243**).
- "Promoção real scope-out ADR-0054 graded" N=1 → **2
  cumulativo**.
- "Spec C1 audit obrigatório bloqueante" N=5 → **6 cumulativo**.

**Quarta excepção justificada ADR-0080 EM VIGOR pós-P229** —
sub-categoria nova "Layouter internal refactor". L0 partial
tocado 2 ficheiros (`region.md` extensão + `content.md` secção
scope-outs).

**Resultado P243**:
- Tests workspace: 2190 → **2198 verdes** (+8).
- Violations: 0 preservadas.
- Content variants: 62 preservado.
- ShapeKind variants: 5 preservado.
- **Regions fields**: 1 → **3** (+backlog +last).
- **Regions methods**: +1 (`advance`).
- **Scope-outs promovidos**: 3.
- Tipos entity novos: 0 (Regions já existia P216B; P243 estende).
- Stdlib funcs: 64 preservado.
- ADRs distribuição preservada (ADR-0081 transita 3/5 → 4/5
  internamente; sem ADR novo). PROPOSTO 12; EM VIGOR 29;
  IMPLEMENTADO 22; total **68 preservado**.
- ADR-0079 Categoria A.4 preservada (P242 parcial pós-P243
  parcial+).
- ADR-0080 §"Excepção P243" N=4 sub-categoria nova "Layouter
  internal refactor" anotada.
- DEBT-56 §"Plano" checklist ✓ item 1 ("Refactor minimal
  Layouter") anotado P243 fase (a); fase (b) pendente.

**Status ADR-0079 mantido PROPOSTO** — sub-passo materializado
pós-P226 contador agora **5** (P227 A.1; P240 M7+1 D.2; P241
M7+2 D.3; P242 M7+5 A.4 parcial; **P243 M7+3 fase (a) infra**).
Fase 5 Layout candidata: 13/13-15 → **14/13-15 sub-passos
materializados** (~93-100% cumulativo).

**M9d / M7+ progresso**: **4/5 sub-passos materializados** (M7+1
✓; M7+2 ✓; **M7+3 fase (a) ✓**; M7+5 ✓; M7+3 fase (b) + M7+4
pendentes — cumulativa restante ~10-16h).

**Próximo sub-passo candidato**: M7+3 fase (b) (recomendação
subjectiva; sequência natural pós-fase (a); L ~5-8h; fecha
DEBT-56). Alternativas: M7+4 Place float (L); cell layout
migration (M ~2-4h); refino A.4 outset/fill/stroke (S-M); ADR
meta admin XS; pivot outro módulo; pausa M-fase.

Decisão humana pendente literal pós-P243.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

---

### P245 anotação — M7+4 Place float real materializado: Categoria C.1 transita pendente → CUMPRIDO; ADR-0081 IMPLEMENTADO TOTAL 5/5

**Data**: 2026-05-14.

**Sexto sub-passo materialização Fase 5 Layout candidata pós-P227**;
**quinta e última sub-passo M9d / M7+ pós-P244** — fecha
**ADR-0081 IMPLEMENTADO total 5/5**.

**P245 materializa M7+4 Place float real** (promoção graded
P223 → semantic activa):
- `DeferredFloat` struct local em Layouter (`pub(super)`; não
  L1 entity).
- 3 fields novos no Layouter: `floats_pending`,
  `cursor_y_top_reserve`, `cursor_y_bottom_reserve`.
- Arm `Content::Place { float: true }` activa — buffer +
  flush deferred no `new_page`/`finish`.
- `float: false` preserva P84.5+P84.6 literal.
- `flush_pending_floats` + `emit_deferred_float` methods.
- DEBT-37 sentinela `scope: Parent + float: true` preservada.

**Categoria C.1 Fase 5 Layout** transita **pendente → CUMPRIDO
P245** ✓.

**Status ADR-0081**: IMPLEMENTADO parcial 4.5/5 → **IMPLEMENTADO
total 5/5** ✓. Distribuição ADRs: IMPLEMENTADO **22 → 23**;
total 68 preservado.

**Pré-condições obrigatórias verificadas P245**:
1. Tests baseline preservados: **2198 → 2203 verdes** (+5
   novos; 0 regressões; 0 adaptações).
2. Comemo memoization invariants ADR-0073/0074 preservados.
3. Backward compat: `Place { float: false }` literal preservado.

**Patterns emergentes inaugurados/consolidados P245** (3):
- "Promoção graded → real semantic activação consumer" N=1
  inaugurado P245.
- "Spec C1 audit obrigatório bloqueante" N=7 → **8 cumulativo**.
- "Layouter internal refactor (semantic activation)" N=1 → **2
  cumulativo** (P243 + P245).

**Anti-inflação 37ª aplicação cumulativa** pós-P205D.

**Status ADR-0079 mantido PROPOSTO** — Categoria C.1 ✓ via
P245; **Categoria C.2 multi-region completion permanece
pendente** (scope-out humano candidato para fechar ADR-0079
→ IMPLEMENTADO graded).

Sub-passo materializado pós-P226 contador agora **6** (P227
A.1; P240 M7+1 D.2; P241 M7+2 D.3; P242 M7+5 A.4 parcial;
P243 M7+3 fase (a) infra; **P245 M7+4 C.1 Place float real**).
Fase 5 Layout candidata: 14/13-15 → **15/13-15 sub-passos
materializados** (~100% cumulativo).

**M9d / M7+ progresso**: **5/5 sub-passos materializados** ✓✓✓
COMPLETO.

**Decisão humana pendente pós-P245**: promoção ADR-0079 →
IMPLEMENTADO graded (scope-out C.2) OU continuação refinos
(A.4 outset/fill/stroke; cell layout migration; pivot outro
módulo).

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

---

### P246 anotação — Cell layout migration: regions.cell + métodos effective/enter_cell/exit_cell; Categoria A.4 breakable per-cell arquiteturalmente desbloqueado

**Data**: 2026-05-14.

**Sétimo sub-passo materialização pós-P227** + continuação
pós-M9d / M7+ completo P245 — **refactor consumer puro
non-disruptive**.

**P246 materializa cell layout migration**:
- `Regions.cell: Option<Region>` field novo + 3 métodos
  (`effective`, `enter_cell`, `exit_cell`).
- Migrados: `cell_available_h` + `cell_origin_w` → `regions.cell`
  (entity-side via Region.height/width).
- Preservados: `cell_origin_x` + `cell_origin_y` como Layouter
  fields legacy (Region actual sem `origin: Point`; refactor
  futuro candidato).
- Save/restore em `grid.rs:361+` refactor — 2 chamadas API
  substitui 4 atribuições directas.
- Reads em `placement.rs` refactor — Decisão 1 Opção B
  confirmada empíricamente pós-audit C1.

**Categoria A.4 Fase 5 Layout breakable per-cell**:
**arquiteturalmente desbloqueado P246** — `Content::Block.breakable`
+ `Content::Boxed.height` + `TableCell` overflow podem consultar
`regions.effective()` para decisão real de quebra dentro da
célula. **Activação real (semantic materialização) diferida
a passo futuro não-reservado** per política P158.

**Pré-condições obrigatórias verificadas P246**:
1. Tests baseline preservados: 2203 → **2209 verdes** (+6
   novos P246 entity-side; 0 regressões; 0 adaptações em tests
   pré-existentes — extensão entity-side aditiva non-disruptive).
2. Comemo memoization invariants ADR-0073/0074 preservados.
3. Backward compat E2E: tests P83 + P84.6 + P156G/H + P157A/B
   passam inalterados (semantic preservada literal via wrapper
   API).

**8 decisões fixadas P246** (Decisão 0 = lição N=8 → 9
cumulativo):
- Decisão 0 — C1 audit obrigatório bloqueante (lição refinada
  "mapear empíricamente distribuição de usos por sub-módulo
  antes de fixar arquitectura de migração").
- Decisão 1 — Opção B (snapshot via `regions.cell`) confirmada
  pós-audit empírico.
- Decisão 2 — Arm Grid save/restore refactor (`enter_cell`/
  `exit_cell` API).
- Decisão 3 — Reads em `placement.rs` migrated.
- Decisão 4 — Activação A.4 breakable per-cell DIFERIDA (passo
  futuro).
- Decisão 5 — DEBT-34c + DEBT-37 sentinelas preservadas.
- Decisão 6 — `Region` struct intocada (preservação P216A
  literal).
- Decisão 7 — Anti-inflação 38ª aplicação cumulativa.
- Decisão 8 — Sub-padrão "Layouter consumer migration via API
  wrapper" N=1 inaugurado.

**Patterns emergentes inaugurados/consolidados P246** (2):
- "Layouter consumer migration via API wrapper" N=1
  inaugurado P246 — sub-padrão novo (migração field-by-field
  Layouter privado → API entity-side; reduz acoplamento).
  Candidato a formalização N=3-4 futuro.
- "Spec C1 audit obrigatório bloqueante" N=8 → **9 cumulativo**
  (P237+P238 reescrito+P240+P241+P242+P243+P244+P245+P246).

**Resultado P246**:
- Tests workspace: 2203 → **2209 verdes** (+6 P246
  entity-side; 0 regressões; 0 adaptações).
- Content variants: 62 preservado.
- ShapeKind variants: 5 preservado.
- **Layouter fields: -2** (cell_available_h + cell_origin_w
  migrados); +0 (cell_origin_x/y preservados legacy).
- **Regions fields: 3 → 4** (+cell).
- **Regions methods: +3** (effective, enter_cell, exit_cell).
- Stdlib funcs: 64 preservado.
- L0 prompts: 1 tocado partial (`entities/region.md` —
  paridade P243 extensão Regions; sub-categoria preservada
  "Layouter internal refactor").
- ADRs distribuição preservada literal: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 23; total **68 preservado**. **ADR-0079
  Categoria A.4 breakable per-cell arquiteturalmente
  desbloqueado P246** anotado.

**Status ADR-0079 mantido PROPOSTO** — Categoria C.1 ✓ P245;
Categoria A.4 parcial P242 + arquiteturalmente desbloqueada
P246; Categoria C.2 pendente.

**Sub-passo materializado pós-P226 contador agora 7** (P227
A.1; P240 M7+1 D.2; P241 M7+2 D.3; P242 M7+5 A.4 parcial;
P243 M7+3 fase (a) infra; P245 M7+4 C.1; **P246 cell migration
+ A.4 desbloqueio arquitectural**).

**Próximo sub-passo candidato pós-P246**:
- **A.4 breakable per-cell activação real** (recomendação
  subjectiva; M ~2-4h; materializa desbloqueio P246).
- Refino A.4 outset/fill/stroke (S-M por attr).
- ADR-0079 → IMPLEMENTADO graded (scope-out humano C.2; XS-S).
- ADR meta admin XS.
- Pivot outro módulo OR pausa M-fase.

Decisão humana pendente literal pós-P246.

Anotação cumulativa acima preserva o contexto histórico
para retomada futura.

---

## Status

**`PROPOSTO`** — autorização arquitectural concedida em
princípio; materialização caso-a-caso fica em aberto.
**Política "sem novas reservas" P158 preservada literal**
— 13-15 sub-passos identificados mas NÃO reservados.
Sub-passo materializado pós-P226: **7** (P227 A.1; P240 M7+1
D.2; P241 M7+2 D.3; P242 M7+5 A.4 parcial; P243 M7+3 fase (a)
infra; P245 M7+4 C.1 Place float real; **P246 cell layout
migration + A.4 breakable per-cell arquiteturalmente
desbloqueado**). Categoria C.1 ✓ P245; Categoria C.2 pendente.

---

## Anotação cumulativa P247 — A.4 Block/Boxed fill+stroke+outset semantic real

**P247 materializa Categoria A.4 cumulativa fill+stroke+outset
semantic real activação** (refino aditivo paralelo Block + Boxed):

- **+2 fields novos** em ambos variants (`fill: Option<Color>`,
  `stroke: Option<Stroke>`); paridade simétrica.
- **outset semantic real activado** (cenário A audit C1 §2.4-§2.5
  — outset zero-uso pré-P247): cursor.y avança outset.top antes
  do inset.top; outset.bottom após height min; bounds Shape
  expandem em todos os lados.
- **Layouter activa emissão `FrameItem::Shape`** ANTES do body
  (Z-order via `current_items.insert(items_before, ...)`); reuso
  `ShapeKind::RoundedRect { radii: radius }` quando radius
  non-zero (P242).
- **stdlib `block(fill:, stroke:)` + `box(fill:, stroke:)`**:
  fill aceita `Value::Color` directo; stroke reusa helper
  `extract_stroke` pré-existente P227.

**Categoria A.4 cumulativa pós-P247**:

- Scope-outs Block originais P156G: 0/9 → **5/9 fechados**
  (outset P231→P247 + radius P242 + clip P242 + fill P247 +
  stroke P247); restam 4 (spacing + above + below + sticky).
- Scope-outs Boxed originais P156H: 0/6 → **5/6 fechados**
  (mesmos 5; resta 1 stroke-overhang).
- **Cobertura Layout per metodologia**: ~93-94% → **~94-95%**
  (+1pp refino qualitativo).
- **Categoria A.4 muito reforçada** (5 dos 9 Block + 5 dos 6
  Boxed cosméticos fechados).

**Pré-condições obrigatórias verificadas P247**:

1. **Tests baseline preservados**: 2209 verdes pré-P247 →
   **2229 verdes pós-P247** (+20 P247 testes novos dentro do
   range +15-25 paridade M-L magnitude; 0 regressões; **N=12
   adaptações** em tests pré-existentes — construtores
   explícitos Block/Boxed em entities/content.rs +
   stdlib/mod.rs + layout/tests.rs + introspect.rs; dentro do
   range N=0-10 estimado §1.4 + 2 adicionais introspect.rs +
   layout-internal arm propagação).
2. **Comemo memoization invariants ADR-0073/0074 preservados**.
3. **Backward compat**: Block/Boxed com fill=None, stroke=None,
   outset=Sides::ZERO renderizam idênticos a P246 (test
   `p247_block_fill_none_e_outset_zero_sem_shape` valida).

**9 decisões fixadas P247** (Decisão 0 = lição N=9 → 10
cumulativo; Decisões 1-8 = arquitectura; Decisão 9 = pattern
emergente N=1).

**Patterns emergentes inaugurados/consolidados P247** (3):

- "Agregar promoções scope-outs cosméticos visuais" **N=1
  inaugurado P247** (3 promoções num passo único: outset
  semantic real + fill + stroke).
- "Spec C1 audit obrigatório bloqueante pós-P236.div-1" N=9 →
  **N=10 cumulativo** (lição refinada: "mapear scope-outs
  declarados historicamente vs estado real materializado antes
  de assumir ausência").
- "Promoção real scope-out ADR-0054 graded" N=2 → **N=3
  cumulativo** (P242 radius+clip = N=2 agregado; P247
  outset+fill+stroke = N=3 agregado; contando granular = 5
  promoções reais: radius + clip + outset + fill + stroke).

**Resultado P247**:
- Tests workspace: 2209 → **2229 verdes** (+20 P247).
- Content variants: **62 preservado** (refino aditivo a
  variants existentes, sem novos).
- Block fields: **8 → 10** (+fill, +stroke).
- Boxed fields: **8 → 10** (+fill, +stroke).
- ShapeKind variants: **5 preservado**.
- Stdlib funcs: **64 preservado** (refino consumer existentes).
- Cobertura Layout per metodologia: **~93-94% → ~94-95%**.

Sub-passo materializado pós-P226: **8** (P227 A.1; P240 M7+1
D.2; P241 M7+2 D.3; P242 M7+5 A.4 radius/clip parcial; P243
M7+3 fase (a) infra; P245 M7+4 C.1 Place float real; P246
cell layout migration + A.4 breakable per-cell arquiteturalmente
desbloqueado; **P247 A.4 cumulativa fill+stroke+outset Block+
Boxed**).

---

## Anotação cumulativa P248 — A.4 breakable + Boxed.height overflow + TableCell overflow semantic real activação

**P248 materializa Categoria A.4 cumulativa via 3 activações
graded → real semantic em agregação**:

- **Activação A — Block.breakable**: medição antecipada via
  `measure_content_constrained` puro pré-existente; `new_page()`
  antecipado se bloco não-breakable não cabe na actual mas cabe
  noutra; overlong emit normal (paridade vanilla atómico).
- **Activação B — Boxed.height** overflow: wrap em FrameItem::Group
  com clip_mask Rect altura h quando body excede + clip=true;
  emit normal (overflow visível) quando clip=false (paridade
  vanilla default).
- **Activação C — TableCell.body** overflow clip implícito ao
  limite cell (paridade vanilla); row break real diferido per
  Decisão 3 (DEBT-34e preservado aberto, distinto).

**Categoria A.4 cumulativa pós-P248**:

- Scope-outs Block originais P156G: 5/9 → **6/9 fechados**
  (outset+radius+clip+fill+stroke+**breakable real**); restam 3
  (spacing+above+below+sticky — 4 vanilla mas spacing-related
  contam como agrupado).
- Boxed.height semantic real activada (era "armazenado adiada"
  P156H; agora real).
- TableCell overflow Y clip implícito activado (row break
  real é refino futuro).
- **Cobertura Layout per metodologia**: ~94-95% → **~95-96%**
  (+1pp refino qualitativo).

**Pré-condições obrigatórias verificadas P248**:

1. **Tests baseline preservados**: 2229 verdes pré-P248 → **2255
   verdes pós-P248** (+26 P248 testes novos dentro do range
   +25-35 paridade L magnitude; **0 regressões**; **0 adaptações**
   em tests pré-existentes — defaults `breakable: true` +
   `height: None` + cell sem overflow preservados).
2. **Comemo memoization invariants ADR-0073/0074 preservados** —
   P248 toca Layouter consumer apenas.
3. **Backward compat**: Block com `breakable: true` (default) +
   Boxed com `height: None` + TableCell sem overflow renderizam
   idênticos a P247 (output PDF bit-equivalente; tests
   `p248_block_breakable_true_preserva_emit_normal`,
   `p248_boxed_height_none_preserva_p156h`,
   `p248_table_cell_sem_overflow_preserva_p157b` validam).

**10 decisões fixadas P248** (Decisão 0 = lição N=10 → 11
cumulativo; Decisões 1-3 algoritmos pós-audit; Decisões 4-9
arquitectura + patterns emergentes).

**Patterns emergentes inaugurados/consolidados P248** (4):

- "Agregar promoções graded → real multi-consumer via mecanismo
  comum" **N=1 inaugurado P248** (3 sub-activações com
  `measure_content_constrained` partilhado; distinto P247
  "agregar cosméticos visuais ortogonais").
- "Promoção graded → real semantic activação consumer" N=1 →
  **N=2 cumulativo** (P245 Place float = N=1; P248 agregado =
  N=2; granular = N=4 contando 3 sub-activações P248).
- "Spec C1 audit obrigatório bloqueante pós-P236.div-1" N=10 →
  **N=11 cumulativo** (lição refinada: "mapear pontos de check
  overflow existentes antes de adicionar novos checks
  duplicados").
- "Promoção real scope-out ADR-0054 graded" granular **N=5 → N=8
  cumulativo** P248 (radius + clip + outset + fill + stroke +
  breakable + height + cell_overflow); limiar ADR meta candidata
  reforçado (N≥6 patamar sólido atingido).

**Resultado P248**:
- Tests workspace: 2229 → **2255 verdes** (+26 P248).
- Content variants: **62 preservado**.
- Block fields: **10 preservado** (P247 final; sem novos).
- Boxed fields: **10 preservado**.
- TableCell fields: **5 preservado** (P157B final).
- ShapeKind variants: **5 preservado**.
- Stdlib funcs: **64 preservado** (refino consumer).
- Cobertura Layout per metodologia: **~94-95% → ~95-96%**.

Sub-passo materializado pós-P226: **9** (P227 A.1; P240 M7+1
D.2; P241 M7+2 D.3; P242 M7+5 A.4 radius/clip parcial; P243
M7+3 fase (a) infra; P245 M7+4 C.1 Place float real; P246
cell layout migration; P247 A.4 cumulativa fill+stroke+outset
Block+Boxed; **P248 A.4 cumulativa breakable+height+cell
overflow semantic real activação**).
