# Passo 225 — Encerramento Fase 4 Layout candidata (documental; fecha série α "terminar Layout")

**Série**: 225 (décimo-primeiro sub-passo Layout pós-M9c;
**encerramento documental Fase 4 Layout candidata**; paridade
estrutural P221 para Fase 3).
**Marco**: **fecho formal série α "terminar Layout"** —
segundo encerramento de Fase Layout pós-M9c (primeiro foi
P221 Fase 3); pattern emergente "encerramento Fase Layout
pós-M9c" N=1 → 2 formalizado.
**Tipo**: passo documental puro — zero código tocado.
Anotações cumulativas + recálculos documentais + blueprint
marca cirúrgica.
**Magnitude**: S (~30min-1h).
**Pré-condição**: P224 concluído (Grid refino substantivo
Opção δ; 3 variants novos GridHeader/Footer/Cell; módulo
`grid_placement.rs` 264 LOC; 2039 tests verdes; §A.5
`grid(...)` impl⁺; cobertura Layout 89% real per metodologia;
DEBT-34e ENCERRADO; **DEBT-34d preservado per `P224.div-1`**;
Fase 4 Layout candidata 3/3 sub-passos materializados);
humano fixou "Escreva" para próximo passo (continuação Caminho
1 P224 §10); ADR-0061 IMPLEMENTADO desde P221 (preservado).
**Output**: 1 ficheiro relatório curto + 4 ficheiros canónicos
editados (ADR-0061 anotação final série α; DEBT.md
actualização cumulativa; inventário 148 consolidação;
blueprint §2.1 Opção γ refresh + §3.0terdecies marca).

---

## §1 Trabalho

P222 + P223 + P224 materializaram cumulativamente Fase 4
Layout candidata em 3 sub-passos:

- **P222** — `native_measure` stdlib expose graded; helper
  visibility promotion; ADR-0066 Bloco C primeira
  materialização parcial.
- **P223** — `Content::Place` refino +2 fields (`float`/
  `clearance`); DEBT-37 §"Divergência" fechada.
- **P224** — `Content::Grid` refino substantivo composto +5
  fields + 3 variants Content novos (GridHeader/Footer/Cell)
  + módulo `grid_placement.rs` placement algorítmico real;
  DEBT-34e fechada; **DEBT-34d preservado per `P224.div-1`**.

**Série α "terminar Layout" fechada estructuralmente** (Opção
α P221 §8 cumprida 3/3 sub-passos). P225 fecha **documentalmente**:

- **ADR-0061** anotação final série α (status IMPLEMENTADO
  preservado desde P221; sem nova transição).
- **DEBT.md** actualização cumulativa (DEBT-37 §"Divergência"
  fechada via P223 anotada; DEBT-34e ENCERRADO via P224
  anotada; DEBT-34d preservado aberto com nota `P224.div-1`).
- **Inventário 148** consolidação cumulativa (Tabela B.2
  +3 entradas Grid* + footnote ⁴⁶ consolidada preservando
  ⁴³+⁴⁴+⁴⁵).
- **Blueprint §2.1** Opção γ refresh — "89% (12 impl + 4
  impl⁺ + 2 parcial)" reflectir cobertura real pós-P224.
- **Blueprint marca §3.0terdecies** — encerramento série α
  "terminar Layout" (paridade pattern marca-por-fecho
  §3.0duodecies P221).

**Decisão arquitectural central — fecho de série pós-M9c
N=2 cumulativo**:

P225 é o **segundo encerramento de Fase Layout pós-M9c**
(primeiro foi P221 Fase 3). Pattern emergente "encerramento
Fase Layout pós-M9c" formalizado N=1 → 2 cumulativo.
Forma metodológica paridade P221:
- Anotações cumulativas em ADRs (preserva histórico).
- Reclassificação documental cumulativa em inventário 148.
- Marca cirúrgica blueprint (paridade §3.0duodecies pattern).
- Zero código tocado.
- Patterns emergentes registados sem promoção formal a ADRs
  meta (política P158 preservada).

**Diferença qualitativa face a P221**:
- P221 transitou 2 ADRs (ADR-0078 + ADR-0061 PROPOSTO →
  IMPLEMENTADO) + fechou 1 DEBT (DEBT-56).
- P225 transita **0 ADRs** (ADR-0061 já IMPLEMENTADO desde
  P221; ADR-0066 mantém PROPOSTO per pattern emergente
  N=1 "ADR PROPOSTO com materialização parcial graded"
  inaugurado P222) + fecha **0 DEBTs novos** (DEBT-34e já
  fechado via P224; **DEBT-34d preservado per `P224.div-1`**).
- P225 é encerramento **mais leve** que P221 — anotações
  cumulativas + recálculos + marca.

Reuso de dados (sem recolha nova):

- ADR-0061 anotações P222 + P223 + P224 cumulativas
  individuais (já materializadas durante cada sub-passo).
- DEBT-37 §"Divergência" fechada via P223 anotação.
- DEBT-34e ENCERRADO via P224 anotação.
- DEBT-34d preservado per `P224.div-1` empírico (refino
  algorítmico de track sizing distinto não endereçável
  por placement work).
- Inventário 148 footnotes ⁴³ (P222) + ⁴⁴ (P223) + ⁴⁵ (P224)
  individuais — base para ⁴⁶ P225 consolidada cumulativa.
- Blueprint §3.0duodecies P221 forma — paridade para
  §3.0terdecies P225.
- Pattern "L0 minimal para refactors" N=7 cumulativo
  (P217+P218+P219+P220+P222+P223+P224 todos Opção γ; P224
  reforçou via divergência consciente face a spec C6
  Opção α).
- Pattern "Field armazenado semantic adiada" N=5 cumulativo
  (P156D + P156E + P156G + P223 + P224 `repeat`).

---

## §2 Cláusulas (10)

### C1 — Auditoria estado factual pré-P225

Verificação empírica antes de anotações:

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
grep -c "^    [A-Z][A-Za-z]\+\(\s\|{\|(\)" 01_core/src/entities/content.rs
grep -n "Status.*IMPLEMENTADO" 00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md
grep -n "DEBT-34e.*ENCERRADO\|DEBT-34d" 00_nucleo/DEBT.md
```

Critério:
- Tests **2039 verdes** (P224 baseline confirmado).
- **0 violations** preservadas.
- **59 variants** Content (GridHeader/Footer/Cell adicionados
  em P224).
- ADR-0061 status IMPLEMENTADO ✓.
- ADR-0066 status PROPOSTO ✓ (preservado).
- ADR-0078 status IMPLEMENTADO ✓ (preservado desde P221).
- DEBT-34e ENCERRADO ✓ (P224); DEBT-34d EM ABERTO ✓.

Se algum critério divergir: registar `P225.div-1` e
investigar antes de prosseguir.

### C2 — ADR-0061 anotação final série α

Editar `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`
adicionando bloco **`### P225 anotação — Encerramento série α
"terminar Layout" 2026-05-13`** após bloco P224:

```markdown
**Série α "terminar Layout" fechada formalmente em P225**:

Trajectória completa pós-M9c Fase 4 Layout candidata
(Opção α P221 §8):
- P222 `measure(body)` stdlib expose graded (Bloco C
  ADR-0066 primeira materialização parcial).
- P223 `Content::Place` refino +2 fields (`float` +
  `clearance` semantic adiada; DEBT-37 §"Divergência"
  fechada).
- P224 `Content::Grid` refino substantivo composto +5
  fields + 3 variants novos + módulo `grid_placement.rs`
  + DEBT-34e ENCERRADO (DEBT-34d preservado per
  `P224.div-1`).

**Cumulativo Fase 4** (3 sub-passos):
- 3 variants Content novos (GridHeader + GridFooter +
  GridCell; 56 → 59).
- +7 fields refino a 2 variants existentes (Place +2
  P223; Grid +5 P224).
- 4 stdlib funcs novas (native_measure + native_grid_cell
  + native_grid_header + native_grid_footer; 55 → 59).
- 2 stdlib refinadas (native_place +2 named args;
  native_grid +5 named args).
- 1 helper visibility promotion (measure_content
  `pub(super)` → `pub(crate)`).
- 1 módulo L1 novo (`grid_placement.rs` 264 LOC).
- 2 DEBTs fechados (DEBT-37 §"Divergência" via P223;
  DEBT-34e via P224).
- 1 DEBT preservado aberto per `P224.div-1` (DEBT-34d
  refino algorítmico track sizing distinto não endereçável
  por placement work).
- 0 ADR transitions (ADR-0061 já IMPLEMENTADO desde P221;
  ADR-0066 mantém PROPOSTO per pattern emergente N=1).
- 52 tests cumulativos (P222 11 + P223 14 + P224 27);
  1998 → 2039 verdes.
- Reclassificações: 3 entradas parcial → impl⁺ (`measure`
  + `place` + `grid`).
- **Cobertura Layout per metodologia**: 78% Fase 3 fechada
  → 83% pós-P223 → **89% pós-P224** (+11pp cumulativo
  Fase 4 real per metodologia; **+17pp visível per
  reclassificações** ausente → parcial → impl⁺ pós-Fase
  3+4 cumulativos).
- Cobertura user-facing total: 65% → **67%** (+2pp
  cumulativo).

**Patterns emergentes cumulativos consolidados Fase 4**:
- "L0 minimal para refactors" N=5 → 6 → **7** (P222 + P223
  + P224 todos Opção γ; P224 divergência consciente vs
  spec C6 reforçou em vez de suspender).
- "Field armazenado semantic adiada" N=3 → 4 → **5**
  (`weak`/`breakable`/`float`/`repeat`).
- "ADR PROPOSTO com materialização parcial graded" N=1
  inaugurado P222 (ADR-0066 mantém PROPOSTO apesar Bloco
  C primeira materialização parcial).
- "Refino aditivo a variant existente" N=1 → **2** (P223
  Place; P224.A Grid).
- "Fecho de divergência documentada via refino" N=1
  inaugurado P223 (DEBT-37 §"Divergência").
- "Fecho cumulativo de DEBTs via refino composto" N=1
  parcialmente inaugurado P224 (apenas DEBT-34e fecha;
  DEBT-34d preservado per `P224.div-1`).
- "Subset Fase agregado L cumulativo pós-M9c" N=1 → **2**
  (P218+P220 trivial; **P224 substantivo com atomização
  A/B/C**).
- "Divergência factual material registada como `Pxxx.div-N`"
  N=1 → **2** (P215.div-1 reabriu Fase 3 sub-fase b; P224.div-1
  preservou DEBT-34d). Pattern de honestidade arquitectural
  consolidado.
- "Consumer geometric integration deferido pós-algorítmico"
  N=1 inaugurado P224.

**Política "sem novas reservas" preservada per P158**:
- **Fase 5 Layout candidata** identificada mas NÃO reservada
  (refinos stroke/fill cosméticos; per-cell align/inset/fill/
  stroke em GridCell; Auto track sizing DEBT-34d; consumer
  geometric integration P224.C).
- **Opção A multi-region** preservada como scope-out per
  ADR-0078 IMPLEMENTADO.

**Estado pós-P225**:
- Sub-fase (a) DEBT-56: 2/2 ✓ (P216A + P216B).
- Sub-fase (b) DEBT-56: 4/4 ✓ (P217-P220).
- DEBT-56 ENCERRADO (P221).
- **Fase 4 candidata 3/3** ✓ (P222-P224); **série α fechada
  estructuralmente E formalmente** em P225.
- Distribuição ADRs preservada P221: PROPOSTO 11 (ADR-0066
  inclusiva); IMPLEMENTADO 21.
- Saldo DEBTs: 13 → **12 abertos** (DEBT-34e fechou em
  P224; **DEBT-34d preservado aberto per `P224.div-1`** vs
  hipótese spec).

**Status ADR-0061 mantido IMPLEMENTADO**. Fase 4 candidata
100% materializada per Opção α P221 §8. Fase 5 candidata
futura **NÃO reservada** per política P158.

**Status ADR-0066 mantido PROPOSTO** — pattern emergente
N=1 "ADR PROPOSTO com materialização parcial graded"
preservado. Promoção formal continua diferida (3 condições
§"Plano promoção" ADR-0066 não satisfeitas).
```

### C3 — DEBT.md actualização cumulativa

Editar `00_nucleo/DEBT.md`:

**DEBT-37** — já ENCERRADO P84.6; anotar **§"Divergência
face ao vanilla" fechada em P223** (anotação histórica
documental; sem reabrir status):

```markdown
**Divergência face ao vanilla — FECHADA P223** ✓:

O comentário original DEBT-37 anotava: "quando `float` for
adicionado, repor a restrição [`scope: Parent` exige
`float: true`]". Em **P223** o campo `float: bool` foi
adicionado a `Content::Place` (semantic real adiada;
pattern N=4 cumulativo); simultaneamente a restrição vanilla
`scope: Parent` sem `float: true` → erro hard foi
restaurada em `native_place` (Decisão 3 Opção α). Paridade
vanilla literal restaurada. 1 test pre-existente adaptado
intencionalmente.
```

**DEBT-34e** — fechada via P224 (já anotada); verificar
critério 5/5 explícito + histórico preservado.

**DEBT-34d** — **preservado em aberto** com nota
**`P224.div-1`** explícita:

```markdown
**Estado pós-P224**: aberto preservado per `P224.div-1`.

P224 hipótese spec previa fecho simultâneo com DEBT-34e via
módulo `grid_placement.rs`. Audit empírico C1 P224 revelou
**distinção factual material**:
- DEBT-34d: "Auto não encolhe antes de matar fr" — **problema
  algorítmico de track sizing** (negociação Auto vs Fraction);
  NÃO placement.
- DEBT-34e: "colspan e rowspan" — algoritmo de placement
  (endereçado por `grid_placement.rs` P224.C).

`P224.div-1` registado como divergência factual material;
DEBT-34d preservado aberto. Refino candidato Fase 5 Layout
NÃO-reservada per política P158.
```

**Saldo DEBTs cumulativo Layout Fase 3+4**:
- Pré-P221: 14 abertos.
- Pós-P221: 13 abertos (DEBT-56 fechou).
- Pós-P224: 12 abertos (DEBT-34e fechou).
- Pós-P225: **12 abertos** (preservado; P225 documental
  puro sem fechos novos).

### C4 — Inventário 148 consolidação cumulativa

Editar `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**§A.5 Layout — estado final pós-P225** (consolidado):

| Linha | Entrada | Pré-Fase 4 | Pós-Fase 4 |
|-------|---------|------------|------------|
| 137 | `place(...)` | parcial ⁵ | impl⁺ ⁵ ⁴⁴ ⁴⁶ |
| 141 | `grid(...)` | parcial ⁵ | impl⁺ ⁵ ⁴⁵ ⁴⁶ |
| 151 | `measure(body)` | parcial | impl⁺ ⁴³ ⁴⁶ |

Tabela A.5 Layout: estado final **`12/4/2/0/0 = 18`**
(zero ausentes preservado; 3 entradas reclassificadas
cumulativamente Fase 4).

**Cobertura Layout per metodologia §A.9** pós-P225:
`(12+4)/18 = **89%**` real ✓ (paridade visual histórica
§2.1 Opção γ refresh em C5).

**Tabela B.2 Content variants** — actualização cumulativa
P222+P223+P224 (já materializadas durante cada sub-passo
individualmente; P225 apenas consolida footnote):

| Variant | Mapeamento | Estado | Passo | Notas |
|---------|------------|--------|-------|-------|
| `Place { ..., float, clearance, ... }` | PlaceElem | impl⁺ | 84.6 + 223 | +2 fields P223 (semantic adiada graded) |
| `GridHeader { body, repeat }` | GridHeader | impl⁺ | 224 | repeat semantic adiada N=5 |
| `GridFooter { body, repeat }` | GridFooter | impl⁺ | 224 | idem |
| `GridCell { body, x, y, colspan, rowspan }` | GridCell | impl⁺ | 224 | paridade P157B; placement algorítmico real via `grid_placement.rs` |

Content variants count: 54 → **59** (cumulativo P217 +
P220 + P224×3).

**Footnote ⁴⁶ P225 consolidada** (~80 linhas) adicionada
após ⁴⁵ documentando:
- Trajectória completa Fase 4 Layout candidata 3/3
  sub-passos (P222+P223+P224).
- 3 entradas reclassificadas parcial → impl⁺.
- Distribuição §A.5 final `12/4/2/0/0 = 18` (zero ausentes
  preservado).
- Cobertura Layout per metodologia: 78% Fase 3 fechada
  → 89% real pós-Fase 4 (+11pp cumulativo).
- Cobertura user-facing total: 65% → 67% (+2pp).
- Content variants: 56 → 59 (+3 cumulativos Fase 4).
- Stdlib funcs: 55 → 59 (+4 cumulativos Fase 4).
- DEBT-37 §"Divergência" fechada P223 (anotação
  histórica).
- DEBT-34e ENCERRADO P224 (5/5 critério).
- **DEBT-34d preservado aberto per `P224.div-1`** (refino
  algorítmico track sizing distinto).
- ADR-0066 mantém PROPOSTO (pattern N=1 "ADR PROPOSTO com
  materialização parcial graded" inaugurado P222).
- ADR-0061 mantém IMPLEMENTADO (Fase 4 candidata 100%
  cumprida sem nova transição).
- 5 patterns emergentes cumulativos consolidados (L0
  minimal N=7; Field semantic adiada N=5; refino aditivo
  variant existente N=2; fecho divergência via refino N=1;
  subset Fase agregado L pós-M9c N=2).
- Política "sem novas reservas" preservada (Fase 5 candidata
  identificada mas não reservada).

**Decisão sobre footnotes anteriores**: **manter ⁴³ + ⁴⁴ +
⁴⁵ + ⁴⁶** em conjunto (paridade pattern P221 que manteve
⁴⁰ + ⁴¹ + ⁴² histórico preservado per P204H+ "histórico
textual preservado"). Reduz inflação acumulativa mas preserva
rastreabilidade incremental P222-P224.

### C5 — Blueprint §2.1 Opção γ refresh + §3.0terdecies marca

Editar `00_nucleo/diagnosticos/blueprint-projecto.md`:

**§2.1 linha Layout** Opção γ refresh (paridade decisão P221
Opção γ):

Antes (pós-P221):
```
| Layout ⁽ᴾ²¹⁴⁾⁽ᴾ²²¹⁾ | 78% (12 impl + 5 parcial) | quase total — Fase 3 fechada estructuralmente P221 (DEBT-56 ENCERRADO; ADR-0078+ADR-0061 IMPLEMENTADO); refinos `measure`/`place` Fase 4 candidata NÃO-reservada |
```

Depois (pós-P225, Opção γ refresh):
```
| Layout ⁽ᴾ²¹⁴⁾⁽ᴾ²²¹⁾⁽ᴾ²²⁵⁾ | **89% (12 impl + 4 impl⁺ + 2 parcial)** | quase total — **Fase 4 candidata fechada formalmente P225** (série α "terminar Layout" cumprida 3/3 sub-passos cumulativos); refinos cosméticos stroke/fill + Auto track sizing DEBT-34d Fase 5 candidata NÃO-reservada |
```

Opção γ refresh: divergência metodológica visual vs real
**fechada via materialização cumulativa**. Pattern de
honestidade: 89% real per metodologia = 89% per paridade
visual histórica refrescada. Distribuição "12 impl + 4
impl⁺ + 2 parcial" reflecte estado real pós-Fase 4.

**§3.0terdecies Marca de actualização — [P225]
Encerramento Fase 4 Layout candidata (série α "terminar
Layout" fechada formalmente)** adicionada após §3.0duodecies
(P221) e antes de §3.1 (paridade pattern marca-por-fecho):

```markdown
### §3.0terdecies Marca de actualização — [P225] **Encerramento Fase 4 Layout candidata (série α "terminar Layout" fechada formalmente)**

**Data**: 2026-05-13.

P225 fecha cirúrgicamente a **série α "terminar Layout"
cumulativa** materializada em P222+P223+P224 (Fase 4 Layout
candidata 3/3 sub-passos). Encerramento documental puro —
sem código tocado em P225.

**Distinção qualitativa face a marca §3.0duodecies (P221)**:

- §3.0duodecies (P221): encerramento Fase 3 Layout pós-M9c
  pattern N=1 inaugurado; 2 ADRs transitam PROPOSTO →
  IMPLEMENTADO; 1 DEBT fecha.
- **§3.0terdecies (esta P225)**: encerramento Fase 4 Layout
  pattern N=1 → 2 cumulativo formalizado; 0 ADRs transitam
  (ADR-0061 já IMPLEMENTADO desde P221; ADR-0066 mantém
  PROPOSTO per pattern emergente); 1 DEBT fechou em P224
  (DEBT-34e); **1 DEBT preservado aberto per `P224.div-1`**
  (DEBT-34d).

**Trajectória completa Fase 4** (3 sub-passos cumulativos):

- **P222** `measure(body)` stdlib expose graded (Bloco C
  ADR-0066 primeira materialização parcial).
- **P223** `Content::Place` refino +2 fields `float` +
  `clearance` semantic adiada; DEBT-37 §"Divergência" fechada.
- **P224** `Content::Grid` refino substantivo composto +5
  fields + 3 variants novos GridHeader/Footer/Cell + módulo
  `grid_placement.rs` placement algorítmico real; DEBT-34e
  ENCERRADO; DEBT-34d preservado per `P224.div-1`.

**Mudanças factuais cumulativas P222-P224**:
- 3 variants Content novos (GridHeader + GridFooter +
  GridCell) — total 56 → 59.
- +7 fields refino a 2 variants existentes (Place +2 P223;
  Grid +5 P224).
- 4 stdlib funcs novas (`native_measure` + `native_grid_cell`
  + `native_grid_header` + `native_grid_footer`) — total
  55 → 59.
- 2 stdlib refinadas (`native_place` +2 named args;
  `native_grid` +5 named args).
- 1 helper visibility promotion (`measure_content`
  `pub(super)` → `pub(crate)`).
- 1 módulo L1 novo (`grid_placement.rs` 264 LOC com
  algoritmo `place_cells` paridade vanilla).
- 52 tests cumulativos: 1998 → **2039 verdes**.
- 2 DEBTs fechados (DEBT-37 §"Divergência" via P223;
  DEBT-34e via P224).
- **§A.5 Layout zero ausentes preservado** (estado terminal
  estrutural pós-P221 mantido).
- **Cobertura Layout per metodologia**: 78% Fase 3 fechada
  → **89% real pós-Fase 4** (+11pp cumulativo; paridade
  visual histórica Opção γ refrescada para "89% (12 impl
  + 4 impl⁺ + 2 parcial)").

**Pattern emergente "encerramento Fase Layout pós-M9c"
N=1 → 2 formalizado**:

- P221 — primeira aplicação (Fase 3 Layout).
- **P225 — segunda aplicação (Fase 4 Layout)**.
- N=2 patamar — pattern reusável para encerramentos Fase
  futura (Fase 5 candidata Layout; Model Fase 3 candidata;
  etc.).

**Pattern emergente "L0 minimal para refactors" N=6 → 7
consolidado**:

- Cumulativo: P217+P218+P219+P220+P222+P223+P224 todos
  Opção γ (P224 divergência consciente face a spec C6 Opção
  α reforçou em vez de suspender).
- N=7 patamar empírico **muito sólido** — ultrapassa limiar
  formalização N=3-4.
- **Promoção formal a ADR meta documental** fica como
  **Caminho 2 candidato P225 §8** (não promove em P225
  per política P158; Caminho 2 candidato sólido para
  passo administrativo XS dedicado se humano priorizar).

**Pattern emergente "Field armazenado semantic adiada"
N=4 → 5 consolidado**:

- Cumulativo: P156D `weak` + P156E `weak` + P156G `breakable`
  + P223 `float` + P224 `repeat` Header/Footer.
- N=5 patamar empírico forte. Promoção formal a ADR meta
  documental candidato.

**Pattern emergente "ADR PROPOSTO com materialização parcial
graded" N=1 estável**:
- ADR-0066 mantém PROPOSTO apesar Bloco C primeira
  materialização parcial via P222. Distintivo de
  trajectórias ADR-0078 (transitou pós-série materialização
  P217-P220 + P221) e ADR-0061 (transitou pós-Fases 1+2+3
  cumpridas P221).

**Pattern emergente "fecho de divergência documentada via
refino" N=1 estável** (DEBT-37 §"Divergência" via P223).

**Pattern emergente "subset Fase agregado L cumulativo
pós-M9c" N=1 → 2 cumulativo**:
- P218+P220 — agregados triviais (variant + stdlib + arm).
- **P224 — primeiro agregado substantivo (L) com atomização
  interna A/B/C explícita**.

**Pattern emergente "divergência factual material registada
como `Pxxx.div-N`" N=1 → 2 cumulativo**:
- P215.div-1 — primeira aplicação (Layout Fase 3 sub-fase
  decomposição empírica).
- **P224.div-1 — segunda aplicação (DEBT-34d preservado;
  refino algorítmico track sizing distinto não endereçável
  por placement work).**
- Pattern de honestidade arquitectural consolidado.

**Pattern emergente "consumer geometric integration deferido
pós-algorítmico" N=1 inaugurado P224** — algoritmo
materializado + testado isoladamente; integração geometric
refino futuro candidato Fase 5.

**Política "sem novas reservas" preservada per P158**:
- Fase 5 Layout candidata identificada mas **NÃO reservada**
  (refinos cosméticos stroke/fill; per-cell align/inset/fill/
  stroke em GridCell; Auto track sizing DEBT-34d; consumer
  geometric integration para placement P224.C; flow real
  Place float; Opção A multi-region para columns/colbreak).
- Reservas conceptuais identificadas mas não formalizadas
  como DEBTs ou ADRs novos.

**Estado pós-P225**:
- **Fase 3 Layout fechada** (P221) ✓.
- **Fase 4 Layout candidata fechada** (P225) ✓.
- **Série α "terminar Layout" fechada formalmente** —
  3/3 sub-passos Fase 4 candidata cumpridos (Opção α P221
  §8 100% materializada).
- Cobertura Layout per metodologia: **89% real** (+17pp
  cumulativo pós-M9c P213-P225).
- Cobertura user-facing total: **67%** (+1pp cumulativo;
  Layout não é maioria mas contribui +5pp pp acumulativo).
- Tests workspace: 1939 (pre-M9c) → **2039 verdes** (+100
  cumulativo pós-M9c P213-P225).
- ADRs: distribuição preservada P221 (PROPOSTO 11;
  IMPLEMENTADO 21).
- DEBTs abertos: 14 (pre-M9c) → **12** (-2 cumulativo:
  DEBT-56 P221; DEBT-34e P224; DEBT-34d preservado per
  `P224.div-1`).
- 18 aplicações cumulativas anti-inflação pós-P205D
  (P225 documental preserva política).
- **Layout em estado terminal estructural** — refinos
  remanescentes são cosméticos (stroke/fill) ou exigem
  reabertura arquitectural maior (Opção A multi-region;
  Auto track sizing; runtime queries genuínas; flow real
  Place float). Fase 5 candidata NÃO-reservada.

**Trajectória aberta pós-P225**:
- Caminhos identificados em §10 do relatório (decisão
  humana literal).
- P225 não compromete trabalho subsequente per política
  "sem novas reservas" P158.

Marco M9c preservado como referência arquitectural estável.
Trajectória M9c+ inclui 13 sub-passos materializados
cumulativamente (P213-P225) cobrindo Layout Fase 3 fechada
+ Fase 4 candidata fechada + recálculos administrativos
iniciais.

Reescrita ampla deste blueprint mantém-se fora-de-escopo
(per padrão consolidado P204H+...+P221). Esta marca
cirúrgica regista **encerramento de Fase Layout pós-M9c
N=2 cumulativo** — segundo encerramento formal pós-M9c
(primeiro foi §3.0duodecies P221).
```

### C6 — Verificação tests workspace

Critério: 2039 verdes preservados (P225 é documental puro;
zero código tocado).

```
cargo test --workspace 2>&1 | tail -3
```

**Erro tolerado**: zero. P225 não toca código; qualquer
red indica problema externo.

Hipótese provável: 2039 verdes preservados literais.

### C7 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. **Sem hashes propagados** — P225
só edita ficheiros documentais (DEBT.md, ADR-0061, blueprint,
inventário 148); não toca L0 prompts em
`00_nucleo/prompts/`.

Esperado: "Nothing to fix" pós-`--fix-hashes`.

### C8 — Verificação auditoria cumulativa final

Auditoria empírica pós-P225 (5 verificações paridade P221
C9):

```
# ADR-0061 status (preservado IMPLEMENTADO)
grep "Status.*IMPLEMENTADO" 00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md

# DEBT-37 divergência fechada (anotação P223 preservada;
# anotação P225 nova "consolidada série α")
grep -A 2 "Divergência face ao vanilla.*FECHADA" 00_nucleo/DEBT.md

# DEBT-34e ENCERRADO (P224 preservado)
grep -A 2 "DEBT-34e.*ENCERRADO" 00_nucleo/DEBT.md

# DEBT-34d preservado per P224.div-1
grep -A 5 "DEBT-34d.*EM ABERTO\|P224.div-1" 00_nucleo/DEBT.md

# Inventário 148 footnote ⁴⁶
grep -A 1 "⁴⁶" 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md

# Blueprint §3.0terdecies + §2.1 P225
grep "§3.0terdecies" 00_nucleo/diagnosticos/blueprint-projecto.md
grep "Layout ⁽ᴾ²¹⁴⁾⁽ᴾ²²¹⁾⁽ᴾ²²⁵⁾" 00_nucleo/diagnosticos/blueprint-projecto.md
```

Critério (6 verificações):
1. ✓ ADR-0061 status IMPLEMENTADO preservado.
2. ✓ DEBT-37 divergência fechada (anotação P223 + P225
   consolidação).
3. ✓ DEBT-34e ENCERRADO (P224 anotado).
4. ✓ DEBT-34d EM ABERTO preservado per `P224.div-1`.
5. ✓ Inventário 148 footnote ⁴⁶ P225 consolidada.
6. ✓ Blueprint §3.0terdecies + §2.1 marcador P225.

### C9 — README ADRs distribuição cumulativa preservada

Editar (opcional) `00_nucleo/adr/README.md` adicionando entrada
P225 anotação:

```markdown
- **Passo 225 — Encerramento Fase 4 Layout candidata
  documental (série α "terminar Layout" fechada formalmente)**
  (passo documental puro; **não materializa código**).
  **Segundo encerramento de Fase Layout pós-M9c** (paridade
  estrutural P221 Fase 3). 0 ADRs transitam (ADR-0061 já
  IMPLEMENTADO desde P221; ADR-0066 mantém PROPOSTO per
  pattern emergente N=1); 0 DEBTs fecham em P225 (DEBT-34e
  fechou em P224); 1 DEBT preservado aberto per
  `P224.div-1` (DEBT-34d refino algorítmico distinto).
  Inventário 148 footnote ⁴⁶ consolidada; blueprint
  §3.0terdecies marca + §2.1 Opção γ refresh "89%".
  6 patterns emergentes cumulativos formalizados N=2/N=5/N=7/
  N=1/N=1/N=2. Distribuição ADRs **preservada P221**: total
  ADRs inalterado; PROPOSTO 11; IMPLEMENTADO 21. **Saldo
  DEBTs: 14 → 12 cumulativo pós-M9c** (DEBT-56 P221;
  DEBT-34e P224; DEBT-34d preservado per `P224.div-1`).
  Tests workspace: 2039 verdes preservados. 0 violations
  preservadas. "Nothing to fix" hashes.
```

**Distribuição ADRs**: total **preservada P221** (PROPOSTO
11; IMPLEMENTADO 21). P225 não transita ADRs.

### C10 — Critério de aceitação P225

Critério (paridade P221 C10):
- 2 ADRs preservadas IMPLEMENTADO (ADR-0061 + ADR-0078;
  ambas desde P221).
- 1 ADR preservada PROPOSTO (ADR-0066; pattern N=1
  "materialização parcial graded").
- 0 DEBTs novos fechados em P225 (DEBT-34e fechou em P224;
  DEBT-34d preservado per `P224.div-1`).
- 1 anotação histórica DEBT-37 consolidada (fecho de
  §"Divergência" via P223 cumulativo).
- Inventário 148 actualizado cumulativamente (footnote ⁴⁶).
- Blueprint §3.0terdecies marca + §2.1 Opção γ refresh.
- 2039 tests verdes preservados (zero código tocado).
- 0 violations preservadas.
- "Nothing to fix" lint.
- Layout em estado terminal estructural reconhecido formalmente.

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-225-relatorio.md`.

Estrutura (~5-7 KB; paridade P221 estrutura formal) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas — encerramento Fase
  4 Layout candidata).
- §2 Auditoria pré-P225 (C1; estado factual P224 baseline).
- §3 ADR-0061 anotação final série α (C2).
- §4 DEBT.md actualização cumulativa (C3; DEBT-37 §"Divergência"
  anotação histórica + DEBT-34e preservado fechado + DEBT-34d
  preservado aberto).
- §5 Inventário 148 consolidação (C4; Tabela B.2 actualização
  cumulativa + footnote ⁴⁶ consolidada).
- §6 Blueprint §2.1 Opção γ refresh + §3.0terdecies marca (C5).
- §7 Resultados verificação (C6+C7+C8; tests + lint + auditoria
  pós).
- §8 Próximo trabalho (caminhos 1-5; sem fixar).

Código alterado: **zero** (passo documental puro).

Ficheiros canónicos editados:
- **Editado**: `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`
  (+ bloco anotação encerramento série α P225).
- **Editado**: `00_nucleo/DEBT.md` (DEBT-37 §"Divergência"
  anotação histórica + DEBT-34d nota `P224.div-1` consolidada).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (Tabela B.2 cumulativa + footnote ⁴⁶ P225 consolidada
  preservando ⁴³+⁴⁴+⁴⁵).
- **Editado**: `00_nucleo/diagnosticos/blueprint-projecto.md`
  (§2.1 Layout Opção γ refresh "89% (12 impl + 4 impl⁺ + 2
  parcial)" + §3.0terdecies marca encerramento série α).
- **Editado** (opcional): `00_nucleo/adr/README.md`
  (+ entrada P225 anotação cumulativa).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Promover ADR-0066 PROPOSTO → IMPLEMENTADO — pattern
  emergente N=1 "ADR PROPOSTO com materialização parcial
  graded" preservado; 3 condições §"Plano promoção"
  ADR-0066 continuam pendentes.
- Reabrir DEBT-34d ou fechar via P225 — preservado aberto
  per `P224.div-1` empírico; refino algorítmico track
  sizing distinto fora de escopo P225 (Fase 5 candidata
  NÃO-reservada).
- Promover ADR meta documental "L0 minimal para refactors"
  N=7 — Caminho 2 candidato P225 §8 separado (passo
  administrativo XS se humano priorizar; não em P225).
- Promover ADR meta documental "Field armazenado semantic
  adiada" N=5 — idem Caminho separado.
- Reescrita ampla blueprint — pattern P204H+
  "fora-de-escopo" preservado.
- Tocar em código `.rs` — passo documental puro.
- Tocar em hashes L0 — sem prompts L0 alterados em P225.
- Abrir novos DEBTs — Fase 5 candidata identificada mas
  NÃO formalizada como DEBT.
- Criar ADR nova — P225 é encerramento, não nova decisão.
- Reescrita §3.1 datada 2026-04-25 — preservação histórica
  per padrão P204H+.
- Materializar refinos Fase 5 candidata (stroke/fill;
  per-cell GridCell; Auto track sizing; consumer geometric
  integration; flow real Place float; runtime queries
  genuínas) — diferidos.

---

## §5 Riscos a evitar

1. **Tentação de promover ADR-0066 PROPOSTO → IMPLEMENTADO
   por "completar" Bloco C ao fechar série α**: rejeitada
   — 3 condições §"Plano promoção" não satisfeitas
   (state(), 2-pass pipeline, E2E feature observable
   dependente runtime). Pattern N=1 "ADR PROPOSTO com
   materialização parcial graded" preservado literal.
2. **Tentação de fechar DEBT-34d "porque está perto"**:
   rejeitada — `P224.div-1` empírico fixou preservação;
   refino algorítmico track sizing distinto não endereçável
   por placement work. Fase 5 candidata NÃO-reservada.
3. **Tentação de promover patterns N=5/N=7 a ADRs meta
   formais em P225**: rejeitada — Caminhos separados
   candidatos (Caminho 2 P225 §8); promoção formal
   exige passo administrativo XS dedicado per pattern
   P160A + P213/P214; políticas P158 + P204H+ preservadas.
4. **Marca §3.0terdecies inflada**: paridade §3.0duodecies
   P221 estrutura. Sem detalhe técnico excessivo —
   referenciar relatórios P222-P224 individuais em vez de
   duplicar conteúdo.
5. **Footnote ⁴⁶ consolida ⁴³+⁴⁴+⁴⁵ removendo histórico**:
   Decisão fixada em C4 — **manter** ⁴³ + ⁴⁴ + ⁴⁵ + ⁴⁶
   (paridade P221 que manteve ⁴⁰+⁴¹+⁴² histórico preservado).
6. **Distribuição ADRs incorrecta**: P225 NÃO transita ADRs.
   PROPOSTO 11 preservado; IMPLEMENTADO 21 preservado.
   Verificar contagem global em README ADRs.
7. **Mudança observable inadvertida**: P225 é documental
   puro; `cargo test` deve permanecer 2039 verdes. Qualquer
   regressão indica problema externo.
8. **L0 hashes desactualizados**: P225 não toca L0 prompts;
   `crystalline-lint --fix-hashes` deve reportar "Nothing
   to fix".
9. **Opção γ §2.1 refresh confusa**: divergência metodológica
   visual vs real **fechou-se via materialização**. Linha
   actualizada "89% (12 impl + 4 impl⁺ + 2 parcial)" reflecte
   realidade. Mitigação: nota explícita em legenda da tabela.
10. **Reabrir P224.div-1 ao mencionar DEBT-34d**: rejeitada
    — DEBT-34d preservado aberto literal; nota documental
    apenas anota estado, não tenta resolver.

---

## §6 Hipótese provável

C1 confirmará estado factual P224 baseline (2039 verdes;
0 violations; 59 variants; DEBT-34e ENCERRADO; DEBT-34d
aberto preservado).

C2 anotará ADR-0061 encerramento final série α (status
IMPLEMENTADO preservado; sem transição).

C3 actualizará DEBT.md cumulativamente (DEBT-37 §"Divergência"
anotação histórica + DEBT-34d nota `P224.div-1` consolidada).

C4 actualizará inventário 148 cumulativamente (Tabela B.2
+ footnote ⁴⁶ consolidada preservando ⁴³+⁴⁴+⁴⁵ histórico).

C5 actualizará blueprint §2.1 Opção γ refresh ("89% (12
impl + 4 impl⁺ + 2 parcial)") + §3.0terdecies marca-por-fecho.

C6 reportará 2039 tests verdes preservados.

C7 reportará 0 violations; "Nothing to fix".

C8 verifica 6 critérios auditoria cumulativa pós.

C9 actualizará (opcional) README ADRs com entrada P225.

C10 deixa caminhos 1-5 em aberto; recomendação subjectiva
Caminho 2 (ADR meta documental L0 minimal N=7 + Field
semantic adiada N=5) ou Caminho 1 (Fase 5 candidata Layout)
ou pivot outro módulo.

Custo real: S (~30min-1h documental). Maior parcela em
C4 (footnote ⁴⁶ consolidação ~80 linhas) + C5 (marca
§3.0terdecies elaboração).

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P225

P225 é estruturalmente distinto na trajectória pós-M9c:

- **Segundo encerramento de Fase Layout pós-M9c** —
  precedente estrutural P221 (Fase 3 Layout fechada).
  Pattern emergente "encerramento Fase Layout pós-M9c"
  N=1 → 2 formalizado.
- **Segundo passo documental puro pós-M9c** (paridade P221
  estrutura formal; P213+P214 foram recálculos administrativos
  mas tocaram inventário 148; P221 e P225 são puramente
  cumulativos).
- **0 ADRs transitam** — distintivo P221 que transitou 2
  ADRs (ADR-0078 + ADR-0061 PROPOSTO → IMPLEMENTADO).
  Pattern emergente "encerramento Fase sem novas transições
  ADR" N=1 inaugurado P225.
- **0 DEBTs fecham em P225** — distintivo P221 que fechou
  DEBT-56. P225 documenta fechos anteriores cumulativos
  (DEBT-37 §"Divergência" via P223; DEBT-34e via P224).
  **DEBT-34d preservado aberto per `P224.div-1`** —
  pattern "divergência factual material" N=2 cumulativo
  registado formalmente.
- **Pattern emergente "encerramento Fase Layout pós-M9c"
  N=2 formalizado em §3.0terdecies** — reusável Fase 5
  candidata Layout ou Model Fase 3 candidata futura sem
  promoção formal a ADR meta (política P158 preservada).
- **Opção γ blueprint §2.1 refresh** — primeira aplicação
  pós-P221 do pattern "refresh metodológico via
  materialização" (78% → 89%); divergência metodológica
  visual vs real **fechada via materialização cumulativa**
  Fase 4.
- **Distribuição ADRs preservada P221** — total inalterado;
  PROPOSTO 11; IMPLEMENTADO 21.
- **Saldo DEBTs cumulativo pós-M9c**: 14 → 12 (-2 sobre
  série completa P213-P225).

Por isso §5 risco 1 (promover ADR-0066 prematuramente) é
o mais provável. Tentação óbvia é "fechar série α implica
fechar ADR-0066 Bloco C". Defesa: 3 condições §"Plano
promoção" não satisfeitas; pattern N=1 preservado literal.

**Critério de aceitação P225**:
- 2 ADRs preservadas IMPLEMENTADO (ADR-0061 + ADR-0078).
- 1 ADR preservada PROPOSTO (ADR-0066).
- 0 DEBTs novos fechados em P225.
- 1 anotação histórica consolidada (DEBT-37 §"Divergência").
- Inventário 148 actualizado cumulativamente (footnote ⁴⁶).
- Blueprint §3.0terdecies + §2.1 Opção γ refresh "89%".
- 2039 tests verdes preservados.
- 0 violations preservadas.
- "Nothing to fix" lint.
- Layout em estado terminal estructural reconhecido formalmente.
