# Spec do passo P253 — Passo administrativo XS promoção ADR-0079 Fase 5 Layout roadmap (PROPOSTO → IMPLEMENTADO; cenário decisional A/B/C fixo pós-audit C1 §2.8; paridade pattern ADR-0061 P221 IMPLEMENTADO; oitava aplicação cumulativa "passo administrativo XS")

**Data**: 2026-05-14.
**Spec**: P253 administrativo XS pós-P252 Boxed A.4 COMPLETO +
ADR-0082 N=3 limiar atingido.
**Tipo**: passo administrativo XS puramente documental
promovendo ADR-0079 (Fase 5 Layout roadmap) de `PROPOSTO`
para `IMPLEMENTADO`. **Zero código tocado.** **Zero variant
Content.** **Zero entity novo.** **Zero L0 prompts tocados.**
**Magnitude planeada**: XS (~30-45 min documental;
paridade P156K + P160A + ADR-0062-create + P244 + P249
precedentes administrativos XS).
**Marco**: **oitava aplicação cumulativa do padrão "passo
administrativo XS"** N=7 → **N=8 cumulativo** (P156A
historiograma + P156K ADRs meta + ADR-0062-create + P160A +
P238 + P244 + P249 + **P253**); **promoção formal ADR-0079
PROPOSTO → IMPLEMENTADO** (paridade pattern ADR-0061 P221
IMPLEMENTADO + ADR-0060 P155 IMPLEMENTADO); décima sexta
aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=15 → 16 cumulativo (lição
refinada P253: "promoção ADR roadmap → IMPLEMENTADO exige
audit empírico cumulativo de sub-passos materializados antes
de declarar critério satisfeito").

---

## §1 O que será feito

P253 promove ADR-0079 (Fase 5 Layout roadmap) de `PROPOSTO`
para `IMPLEMENTADO`. **3 cenários decisionais preliminares**
A/B/C documentados em §3.1; **decisão final fixa
empíricamente pós-audit C1 §2.8** baseado em contagem real
de sub-passos cumulativos vs critério literal ADR-0079.

### §1.1 Estado pré-P253 (factual; confirmado audit empírico 2026-05-14)

**ADR-0079 actual**:
- Status: `PROPOSTO` (criada 2026-05-13 pós-P226).
- Validado: diagnóstico amplo P226 + decisão humana P225 §8.
- Critério promoção literal §"Critério de promoção":
  > ADR-0079 transita PROPOSTO → **IMPLEMENTADO** quando:
  > 1. **Todos 13-15 sub-passos** identificados materializados.
  > 2. **OU decisão humana de scope-out parcial formal**
  >    (e.g., categorias A+B materializadas; C+D scope-out
  >    formal por trade-off magnitude/risco; ADR-0079
  >    anotada com nota "Fase 5 mínima cumprida; C+D adiadas
  >    por scope-out formal").

**Categorias declaradas ADR-0079**:

| Categoria | Sub-passos | Magnitude | Reabertura |
|-----------|------------|-----------|------------|
| A Cosméticos | 5 (A.1-A.5) | M-L | não |
| B Algorítmicos isolados | 3 (B.1-B.3) | M+ a L | não |
| C Estruturais | 2 (C.1, C.2) | L+ a XL | sim (C.1, C.2) |
| D Runtime queries | 5-6 (D.1-D.6) | L+ a XL | sim (D) |

**Estado cumulativo factual pós-P252** (sub-passos materializados
P227-P252 contagem preliminar):

| Categoria | Sub-passos cumulativos | Estado |
|-----------|------------------------|--------|
| A.1-A.5 | P227 (stroke Grid+Table) + P228 (fill) + P230 + P232 (align Grid-level) + P234 (placement) + P235 (inset Grid-level) + P242 (radius+clip) + P247 (outset+fill+stroke Block+Boxed) + P250 (Block 4 scope-outs spacing+above+below+sticky) + P252 (Boxed stroke-overhang) | **muito reforçada**; Block A.4 COMPLETO 10/10; Boxed A.4 COMPLETO 6/6 |
| B.1-B.3 | P234 (placement algorítmico cells) | parcial (B.2 cumprido; B.1/B.3 a confirmar §2) |
| C.1 | P245 (Place float real) | **cumprida** |
| C.2 | P251 (cell-level row break γ-Items) | **parcial** (cell-level apenas; multi-region completo NÃO cumprido — DEBT-56b candidato não-aberto P158) |
| D.1-D.6 | P236 (state_final) + P237 (state_at) + P240 (state.display) + P241 (counter.display) | parcial (~4/5-6) |

**Total cumulativo preliminar**: ~14-16 contagens granulares.
**Audit C1 §2 confirma contagem exacta**.

**Precedentes empíricos `IMPLEMENTADO`**:
- **ADR-0061** PROPOSTO 2026-04-25 → IMPLEMENTADO 2026-05-12
  P221 (Fase 1+2+3 cumpridas; columns/colbreak prosseguem
  como roadmap pós-IMPLEMENTADO).
- **ADR-0060** PROPOSTO 2026-04-25 → IMPLEMENTADO P155 (Fase
  1 fechada; Fase 2/3 prosseguem como roadmap).
- **Termo "graded" NÃO usado** explicitamente em precedentes;
  uso é "IMPLEMENTADO" com restantes sub-fases prosseguindo
  como roadmap.

### §1.2 Trabalho a fazer P253

**1. Audit C1 §2** — confirmar contagem exacta cumulativa
+ estado real categorias A/B/C/D.

**2. Decisão arquitectural §3.1** — cenário A/B/C fixo
empíricamente pós-audit §2.8.

**3. ADR-0079 §"Status" actualização**:
- Cenário A: `PROPOSTO → IMPLEMENTADO` com scope-out formal
  C.2 multi-region completo + D.2-D.6 restantes.
- Cenário B: `PROPOSTO → IMPLEMENTADO` total (sem scope-out)
  se contagem cumulativa atingir 13-15 literal.
- Cenário C: `PROPOSTO` preservado + anotação cumulativa
  apenas (se contagem insuficiente para promoção).

**4. ADR-0079 §"Aplicações cumulativas (sub-passos materializados)"**
extensão com tabela completa P227-P252.

**5. ADR-0079 §"Estado final P253"** (cenários A/B):
documentar scope-outs formais + roadmap pós-IMPLEMENTADO
para sub-passos diferidos.

**6. README ADRs** anotação:
- Cenário A/B: PROPOSTO 13 → 12; IMPLEMENTADO 23 → 24;
  total 69 preservado.
- Cenário C: distribuição preservada.

**7. Anotações cumulativas ADR-0061 + ADR-0080 + ADR-0082**:
- ADR-0061: anotação "paridade pattern P221 IMPLEMENTADO"
  precedente para ADR-0079 P253 (cenários A/B).
- ADR-0080 §"Lição refinada P253" N=15 → 16 cumulativo
  ("promoção ADR roadmap → IMPLEMENTADO exige audit empírico
  cumulativo de sub-passos materializados antes de declarar
  critério satisfeito").
- ADR-0082 §"Aplicações citantes": N=3 limiar atingido P252
  preservado (P253 administrativo XS não-citante).

**8. Relatório P253** estrutura canónica administrativos XS.

### §1.3 Tests esperados

Tests P253 novos: **0** (paridade absoluta administrativos
XS precedentes P156A/P156K/ADR-0062-create/P160A/P238/P244/
P249). Workspace pós-P253: **2304 verdes preservado**.

### §1.4 Adaptações pre-existentes

**N=0** (paridade absoluta administrativos XS).

---

## §2 Verificação empírica pré-P253 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=15 → 16 cumulativo

Audit C1 obrigatório bloqueante pós-P236.div-1. Lição refinada
N=15 P252 ("refactor cross-cutting de entity primitivo exige
mapa empírico exhaustive de todos os construtores literais
antes de modificar struct") expande para **N=16 cumulativo**:
"promoção ADR roadmap → IMPLEMENTADO exige audit empírico
cumulativo de sub-passos materializados antes de declarar
critério satisfeito".

### §2.1 ADR-0079 status + critério promoção (já confirmado 2026-05-14)

Audit empírico anterior confirmou:
- Status `PROPOSTO` actual.
- Critério literal §"Critério de promoção": "13-15 sub-passos
  materializados OU scope-out parcial formal humano".
- 4 categorias declaradas A/B/C/D.

### §2.2 Inventário cumulativo sub-passos materializados P227-P252

Audit empírico requerido:

```bash
grep -B2 -A 5 "P22[7-9]\|P2[3-5][0-9]" 00_nucleo/adr/typst-adr-0079-*.md | head -80
ls 00_nucleo/materialization/typst-passo-22[7-9]*.md \
   00_nucleo/materialization/typst-passo-2[3-5][0-9]*.md 2>/dev/null | wc -l
```

Confirmar contagem cumulativa exacta sub-passos materializados
desde P227 (primeiro sub-passo Fase 5 candidata) até P252.

**Hipótese preliminar**: ~14-16 contagens granulares
(P227+P228+P230+P232+P234+P235+P236+P237+P240+P241+P242+P245+
P246+P247+P248+P250+P251+P252 = ~18 contagens com alguns
agregados).

**Mapping para categorias A/B/C/D**: confirmar empíricamente
quais passos mapeiam para que categoria. Spec §1.1 tabela é
preliminar.

### §2.3 Estado real Categoria A (cumprimento exhaustive)

Categoria A.1-A.5 (5 sub-passos):
- A.1: stroke render Block + Boxed → cumprido (P247 + P252).
- A.2: fill render → cumprido (P228 + P247).
- A.3: align/inset render → cumprido (P230 + P232 + P235).
- A.4: cosméticos restantes → **muito reforçada** (Block 10/10
  + Boxed 6/6 + TableCell row break).
- A.5: ? — confirmar empíricamente.

### §2.4 Estado real Categoria B (cumprimento exhaustive)

Categoria B.1-B.3 (3 sub-passos):
- B.1: ? — confirmar.
- B.2: placement algorítmico cells → cumprido (P234).
- B.3: ? — confirmar.

### §2.5 Estado real Categoria C (cumprimento exhaustive)

Categoria C.1-C.2 (2 sub-passos):
- C.1: Place float real → **cumprida P245**.
- C.2: cell-level multi-region → **parcial P251** (cell-level
  row break γ-Items); multi-region completo (Reabertura 2+3)
  NÃO cumprido.

### §2.6 Estado real Categoria D (cumprimento exhaustive)

Categoria D.1-D.6 (5-6 sub-passos):
- D.1: state(key, init) runtime → ? (ADR-0066 PROPOSTO; pode
  estar parcial via P236/P237).
- D.2-D.6: state.display + counter.display + state.final +
  state.at + metadata + others → parcial via P236+P237+P240+
  P241 (~4/5-6 cumpridos).

### §2.7 Contagem cumulativa total + decisão arquitectural §2.8

**Contagem preliminar**: 
- Categoria A: ~5/5 cumprida (muito reforçada).
- Categoria B: ~1-2/3 (B.2 cumprido; outros parciais).
- Categoria C: ~1.5/2 (C.1 cumprida; C.2 parcial).
- Categoria D: ~4/5-6.
- **Total cumulativo**: ~11-13/13-15 (limiar inferior atingido;
  limiar superior próximo mas não 100%).

**Decisão arquitectural §2.8 final** baseada em:
- Se contagem >= 13 literal → **Cenário B** IMPLEMENTADO total
  (sem scope-out).
- Se contagem 11-13 com gaps documentáveis → **Cenário A**
  IMPLEMENTADO com scope-out formal C.2 multi-region completo
  + D restante.
- Se contagem < 11 → **Cenário C** PROPOSTO preservado +
  anotação cumulativa apenas.

**Recomendação subjectiva preliminar**: **Cenário A** —
paridade directa ADR-0061 P221 pattern; scope-outs claramente
documentáveis; patamar conceptual sólido pós-P252 (Block A.4
COMPLETO + Boxed A.4 COMPLETO + ADR-0082 N=3 limiar). Decisão
final §3.1 confirma pós-audit empírico.

### §2.8 Tests baseline pré-P253

```bash
cargo test --workspace
```

Esperado: **2304 verdes** (estado pós-P252).

### §2.9 Hashes L0 baseline

```bash
crystalline-lint --fix-hashes
```

Esperado: **"Nothing to fix"** (paridade administrativo XS;
zero L0 prompts tocados).

### `P253.div-N` antecipadas — possíveis

- **`P253.div-1`** se §2.2 revelar discrepância contagem
  cumulativa esperada vs real (ex: passos não-canónicos).
- **`P253.div-2`** se §2.3-§2.6 revelar mapping diferente
  categorias.
- **`P253.div-3`** se §2.8 baseline ≠ 2304 → reconciliação
  prévia tests.
- **`P253.div-4`** se §2.9 hashes alterados (improvável;
  passo administrativo XS).

---

## §3 Decisões fixadas P253 — 8 decisões

### Decisão 0 — Audit C1 lição N=15 → 16 cumulativo

Pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=15 → **16 cumulativo**. Refino procedural P253: "promoção
ADR roadmap → IMPLEMENTADO exige audit empírico cumulativo
de sub-passos materializados antes de declarar critério
satisfeito". Anotação em ADR-0080 §"Lição refinada P253".

### Decisão 1 — Cenário promoção final fixo pós-audit §2.8

**3 cenários preliminares**:

**Cenário A** (recomendado preliminar): **PROPOSTO → IMPLEMENTADO
com scope-out formal**:
- Status `IMPLEMENTADO`.
- §"Status final P253" documenta:
  - Cumprido cumulativamente: A.1-A.5 (Block 10/10 + Boxed
    6/6 + TableCell row break parcial) + B.2 + C.1 + D parcial.
  - Scope-out formal: C.2 multi-region completo (Reaberturas
    2+3 não-disparadas; DEBT-56b candidato não-aberto P158)
    + D.2-D.6 restantes (prosseguem como roadmap pós-IMPLEMENTADO).
- Paridade pattern ADR-0061 P221.

**Cenário B**: **PROPOSTO → IMPLEMENTADO total** (sem scope-out):
- Status `IMPLEMENTADO` com declaração "13-15 sub-passos
  cumpridos cumulativamente".
- Requer audit §2.2 confirmar contagem >= 13 literal.

**Cenário C**: **PROPOSTO preservado + anotação cumulativa
apenas**:
- Status `PROPOSTO` mantido.
- §"Aplicações cumulativas (sub-passos materializados)"
  extensão P227-P252.
- ADRs distribuição preservada literal.
- Decisão humana futura quando atingir 13-15 sub-passos
  verdadeiros.

**Final fixo §2.8**: executor cristaliza A/B/C baseado em
contagem empírica + recomendação preliminar Cenário A se
contagem 11-13 com gaps documentáveis.

### Decisão 2 — Estrutura §"Status final P253" (cenário A)

Se Cenário A fixado:

```markdown
## Status final P253

ADR-0079 transita PROPOSTO → IMPLEMENTADO em 2026-05-14 P253
após cumprimento cumulativo dos sub-passos abaixo:

### Cumprido cumulativamente (P227-P252)

| Categoria | Sub-passo cumprido | Passo origem |
|-----------|--------------------|--------------|
| A.1 (stroke) | Grid + Table + Block + Boxed | P227 + P247 + P252 |
| A.2 (fill) | Grid + Table + Block + Boxed | P228 + P247 |
| A.3 (align/inset) | Grid + Table cell-level | P230 + P232 + P235 |
| A.4 (cosméticos) | Block A.4 COMPLETO 10/10 + Boxed A.4 COMPLETO 6/6 + radius/clip | P242 + P247 + P250 + P252 |
| A.5 (placement Place) | Place float real cell+page | P245 (C.1) |
| B.2 | Placement algorítmico cells colspan/rowspan | P234 |
| C.1 | Place float real (Reabertura 1 disparada) | P245 |
| C.2 (parcial) | Cell-level row break γ-Items | P251 |
| D parcial | state_final + state_at + state.display + counter.display | P236 + P237 + P240 + P241 |

**Total cumprido cumulativamente**: ~11-13 sub-passos granulares.

### Scope-out formal P253 (humano)

- **C.2 multi-region completo** (Reabertura 2 P216B extensão
  + Reabertura 3 DEBT-56b): scope-out formal por trade-off
  magnitude (~10-15h cumulativo) vs benefício marginal pós-P251
  γ-Items cell-level já materializado. DEBT-56b candidato
  **não-aberto** per política P158 "sem novas reservas".
- **D.2-D.6 restantes** (state.final two-pass real refino,
  metadata, here/locate, query, position): prosseguem como
  **roadmap pós-IMPLEMENTADO** (paridade ADR-0061 P221
  columns/colbreak; ADR-0060 P155 Fase 2/3 prosseguindo).

### ADRs derivadas pós-P253

- ADR-0082 §"Aplicações citantes" N=3 limiar atingido P252
  (promoção EM VIGOR humana possível separadamente).
- ADRs cumulativamente anotadas: 0054 + 0061 + 0080 + 0082.
- Sem novas ADRs criadas P253.
```

### Decisão 3 — ADR-0079 §"Aplicações cumulativas" extensão

Adicionar tabela completa P227-P252 (paridade pattern
ADR-0061 §"Aplicações cumulativas pós-ADR-0062-create"):

```markdown
| Passo | Feature(s) | Categoria | Tests Δ |
|-------|-----------|-----------|--------:|
| P227 | stroke Grid + Table | A.1 | +N |
| P228 | fill render | A.2 | +N |
| P230 | (continuação) | A.3 | +N |
| P232 | align Grid-level | A.3 | +N |
| P234 | placement cells | B.2 | +N |
| P235 | inset Grid-level | A.3 | +N |
| P236 | state_final | D parcial | +N |
| P237 | state_at | D parcial | +N |
| P240 | state.display | D parcial | +N |
| P241 | counter.display | D parcial | +N |
| P242 | radius + clip | A.4 | +N |
| P245 | Place float real | C.1 | +N |
| P246 | cell layout migration | (infrastructure) | +N |
| P247 | outset + fill + stroke Block+Boxed | A.4 | +20 |
| P248 | breakable + height + cell overflow | A.4 | +26 |
| P249 | ADR meta admin XS (ADR-0082) | (administrativo) | 0 |
| P250 | Block 4 scope-outs (spacing+above+below+sticky) | A.4 | +21 |
| P251 | TableCell row break γ-Items | C.2 parcial | +18 |
| P252 | Boxed stroke-overhang refactor cross-cutting Stroke | A.4 | +10 |
| **P253** | **(administrativo XS — promoção ADR-0079)** | — | **0** |
```

Tests Δ valores reais a preencher pós-audit §2.2.

### Decisão 4 — README ADRs anotação

Cenário A/B: PROPOSTO 13 → **12** (ADR-0079 sai); IMPLEMENTADO
23 → **24** (ADR-0079 entra); **total 69 preservado**.

Cenário C: distribuição preservada literal (anotação cumulativa
apenas).

### Decisão 5 — Anotações cumulativas ADR-0061 + ADR-0080 + ADR-0082

- **ADR-0061**: anotação P253 "paridade pattern P221
  IMPLEMENTADO precedente para ADR-0079 promoção" (cenário
  A/B).
- **ADR-0080**: §"Lição refinada P253" N=15 → **16
  cumulativo** ("promoção ADR roadmap → IMPLEMENTADO exige
  audit empírico cumulativo de sub-passos materializados
  antes de declarar critério satisfeito") + sub-padrão "ADR
  Fase X roadmap → IMPLEMENTADO via scope-out formal humano"
  N=? cumulativo a verificar (ADR-0060 + ADR-0061 + **ADR-0079**
  = N=3 cumulativo se ADR-0079 promove A/B).
- **ADR-0082**: §"Aplicações citantes" N=3 limiar atingido
  P252 preservado; **promoção EM VIGOR humana possível**
  separadamente (não-disparada P253 administrativo).

### Decisão 6 — Sem nova ADR criada; sem reabertura existentes

P253 promove ADR-0079 existente; sem criar ADRs novas.
Política P158 "sem novas reservas" preservada.

### Decisão 7 — Anti-inflação 45ª aplicação cumulativa

- Opção β L0 minimal: zero L0 prompts tocados; hashes
  preservados literal.
- Opção α promoção ADR existente (paridade ADR-0061 P221).
- Opção α anotação cumulativa minimal ADRs (0061 + 0080 +
  0082).
- Opção α paridade administrativos XS precedentes (N=7 → 8
  cumulativo).
- Opção α scope-outs formais documentados se Cenário A.

### Decisão 8 — Patterns emergentes cumulativos P253 (3-4)

- **"Passo administrativo XS"** N=7 → **8 cumulativo** (P156A
  + P156K + ADR-0062-create + P160A + P238 + P244 + P249 +
  **P253**).
- **"ADR Fase X roadmap → IMPLEMENTADO via scope-out formal
  humano"** N=? a verificar (precedentes ADR-0060 P155 +
  ADR-0061 P221 + **ADR-0079 P253** se Cenário A/B fixado
  = N=3 cumulativo; atinge limiar formalização interno).
- **"Spec C1 audit obrigatório bloqueante"** N=15 → **16
  cumulativo**.
- **"Promoção ADR PROPOSTO → IMPLEMENTADO pós-cumulativo
  materializado"** sub-padrão emergente N=1 inaugurado P253
  (ou N=3 cumulativo se contar ADR-0060/0061 precedentes).

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

| Categoria | Ficheiro | Trabalho |
|-----------|----------|----------|
| ADR-0079 | `00_nucleo/adr/typst-adr-0079-layout-fase-5-completar.md` | Status `PROPOSTO → IMPLEMENTADO` (cenário A/B) ou preservado (cenário C); §"Aplicações cumulativas" extensão tabela P227-P252; §"Status final P253" novo (cenário A/B) com scope-out formal + roadmap pós-IMPLEMENTADO |
| README ADRs | `00_nucleo/adr/README.md` | Cenário A/B: PROPOSTO 13 → 12; IMPLEMENTADO 23 → 24; total 69 preservado; Cenário C: preservado |
| ADR-0061 anotação | `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md` | Anotação cumulativa P253 "paridade pattern P221 IMPLEMENTADO precedente para ADR-0079 promoção" (cenário A/B) |
| ADR-0080 anotação | `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` | §"Lição refinada P253" N=15 → 16 cumulativo; sub-padrão "ADR Fase X roadmap → IMPLEMENTADO via scope-out formal humano" N=? cumulativo (verificar precedentes) |
| ADR-0082 anotação | `00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md` | §"Aplicações citantes" preservada literal (N=3 limiar atingido P252); anotação "ADR-0079 promoção P253 não-citante (administrativo XS)" |
| Relatório P253 | `00_nucleo/materialization/typst-passo-253-relatorio.md` | Estrutura canónica administrativos XS (paridade P156K + P160A + P244 + P249) |

**Sem L0 prompts tocados.** **Sem entities tocadas.** **Sem
rules tocadas.** **Sem stdlib tocada.** **Sem inventário 148
tocado** (refino administrativo não-feature).

---

## §5 Critério aceitação P253 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde preservado** (zero código) |
| `cargo test --workspace` | **2304 verdes preservado** (paridade administrativo XS) |
| `crystalline-lint .` | **0 violations preservado** |
| `crystalline-lint --fix-hashes` | **"Nothing to fix"** (paridade administrativo XS) |
| Content variants | **62 preservado** |
| ShapeKind variants | **5 preservado** |
| Block / Boxed / TableCell fields | preservados |
| Stroke fields | **3 preservado** (P252 final) |
| Layouter fields | preservado |
| Regions fields | **4 preservado** |
| Stdlib funcs | **64 preservado** |
| §A.5 distribuição | preservada literal (refino administrativo) |
| Cobertura Layout per metodologia | **~98-99% preservado** |
| Cobertura user-facing total | **~75-76% preservado** |
| **ADRs distribuição (cenário A/B)** | PROPOSTO 13 → **12**; IMPLEMENTADO 23 → **24**; EM VIGOR 29 preservado; total **69 preservado** |
| **ADRs distribuição (cenário C)** | preservada literal |
| ADR-0079 status (cenário A/B) | **`PROPOSTO → IMPLEMENTADO`** com §"Status final P253" |
| ADR-0079 status (cenário C) | **`PROPOSTO` preservado** + §"Aplicações cumulativas" extensão |
| ADR-0080 anotação | §"Lição refinada P253" N=16 cumulativo |
| ADR-0061 anotação | "paridade pattern P221 IMPLEMENTADO precedente" (cenário A/B) |
| ADR-0082 anotação | preservada literal (N=3 limiar atingido P252 preservado) |
| DEBT-30/34c/34e/56 | sentinelas preservadas |
| L0 hashes propagados | **0** (paridade administrativo XS) |
| Adaptações pre-existentes | **N=0** (paridade absoluta administrativo XS) |
| Regressões reais | **0** mandatório |
| Patterns emergentes | "Passo administrativo XS" N=7 → 8 cumulativo; "ADR Fase X roadmap → IMPLEMENTADO via scope-out formal humano" N=? cumulativo; "Spec C1 audit obrigatório bloqueante" N=15 → 16 cumulativo |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2304 verdes pré-P253 →
   2304 verdes pós-P253 (paridade absoluta; zero código).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P253 não toca trait Introspector nem qualquer código L1.
3. **Backward compat literal**: zero código alterado; toda
   funcionalidade pré-P253 preservada literal.

**Promoções ADR esperadas (cenário A recomendado preliminar)**:

- **ADR-0079** PROPOSTO → **IMPLEMENTADO** com scope-out formal
  C.2 multi-region completo + D.2-D.6 restantes prosseguindo
  como roadmap.
- ADR-0061 anotação "paridade pattern P221 precedente"
  cumulativa.
- ADR-0080 §"Lição refinada P253" anotada N=16 cumulativo +
  sub-padrão "ADR Fase X roadmap → IMPLEMENTADO via scope-out
  formal humano" N=3 cumulativo (ADR-0060 + ADR-0061 + ADR-0079).
- ADR-0082 §"Aplicações citantes" preservada literal.
- **Sem outras ADRs criadas**.

---

## §6 Próximo sub-passo pós-P253

P253 fecha promoção ADR-0079 (cenário A/B/C). Restantes
pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **ADR-0082 → EM VIGOR humana** | Passo administrativo XS promoção (N=3 limiar atingido P252) | XS | **alta** (limiar atingido; decisão humana directa) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | **alta** (Layout fechado Cenário A/B; pivot razoável estratégicamente) |
| **D.2-D.6 restantes (roadmap pós-IMPLEMENTADO)** | state.final two-pass + metadata + here/locate + query + position | L+ cumulativo | baixa (scope-out formal P253 se Cenário A) |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **DEBT-56b abrir P-passo** | C.2 multi-region completo (Reabertura 2+3) | L+ | baixa (scope-out formal P253 se Cenário A) |
| **A.4 TableCell row break refino γ-Content** | Refino P251 γ-Items (re-layout tail Content) | L+ | baixa (P251 γ-Items suficiente) |
| **Pausa marco** | Layout Fase 5 IMPLEMENTADO + Block + Boxed COMPLETOS + ADR-0082 N=3 limiar | XS | baixa |

**Recomendação subjectiva pós-P253**: **ADR-0082 → EM VIGOR
humana** (passo administrativo XS) — primeira aplicação cumulativa
do padrão "ADR meta PROPOSTO → EM VIGOR pós-N=3 citantes"
(paridade ADR-0065). Magnitude XS pura administrativa; valida
ADR-0082 empíricamente como pattern sólido cumulativo.

Alternativa estratégica forte: **pivot outro módulo** —
Visualize 54% / Text 52% / Model 50%. Layout fechado Cenário
A/B pós-P253; pivot razoável estratégicamente após patamar
conceptual máximo atingido.

**Decisão humana fica em aberto literal** pós-P253.

**Estado esperado pós-P253 (cenário A/B)**:
- Tests workspace: **2304 verdes preservado**.
- Content variants: **62 preservado**.
- Block / Boxed / TableCell fields: preservados.
- Stroke fields: **3 preservado**.
- Layouter / Regions fields: preservados.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: preservada literal.
- Cobertura Layout per metodologia: **~98-99% preservado**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição (cenário A/B)**: PROPOSTO 13 → **12**;
  IMPLEMENTADO 23 → **24**; EM VIGOR 29 preservado; **total
  69 preservado**.
- **Saldo DEBTs: 11 preservado** (DEBT-56b candidato
  não-aberto P158).
- **45 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P253** (3-4):
  - "Passo administrativo XS" N=7 → **8 cumulativo**.
  - "ADR Fase X roadmap → IMPLEMENTADO via scope-out formal
    humano" N=? cumulativo (cenário A/B: N=3 = ADR-0060 +
    ADR-0061 + ADR-0079 — limiar formalização interno
    atingido).
  - "Spec C1 audit obrigatório bloqueante" N=15 → **16
    cumulativo**.
- **Categoria A Fase 5 Layout**: Block COMPLETO + Boxed
  COMPLETO; **A.4 finalizada cumulativa**.
- **Categoria C.1 Fase 5 Layout**: cumprida P245 (Reabertura 1).
- **Categoria C.2 Fase 5 Layout**: parcial P251; multi-region
  completo scope-out formal P253 (cenário A).
- **Categoria D Fase 5 Layout**: parcial (~4/5-6); restantes
  prosseguem como roadmap pós-IMPLEMENTADO (cenário A).
- **Marco interno (cenário A/B)**: **Layout Fase 5 IMPLEMENTADO**
  via cumprimento cumulativo + scope-out formal humano
  C.2/D.2-D.6; paridade pattern ADR-0061 P221; oitava
  aplicação cumulativa "passo administrativo XS"; padrão
  "ADR Fase X roadmap → IMPLEMENTADO via scope-out formal
  humano" atinge N=3 cumulativo (limiar formalização
  interno); lição C1 audit N=16 cumulativa refinada
  procedimentalmente.

---

## §7 Notas operacionais para o executor

1. **Audit C1 BLOQUEANTE prioridade absoluta**. Não materializar
   antes de §2.1-§2.9 completos. **Lição N=16 cumulativa**:
   refino procedural "promoção ADR roadmap → IMPLEMENTADO
   exige audit empírico cumulativo de sub-passos materializados
   antes de declarar critério satisfeito".

2. **Decisão 1 final fixa pós-audit §2.7-§2.8**. Cenário
   A/B/C cristalizado empíricamente:
   - Se contagem >= 13 literal → Cenário B.
   - Se contagem 11-13 com gaps documentáveis → **Cenário A
     recomendado** (paridade ADR-0061 P221).
   - Se contagem < 11 → Cenário C.

3. **Cenário A é recomendação subjectiva preliminar** baseada
   em:
   - Patamar conceptual sólido pós-P252 (Block A.4 COMPLETO
     + Boxed A.4 COMPLETO + C.1 cumprida + C.2 parcial +
     D parcial).
   - Paridade directa ADR-0061 P221 IMPLEMENTADO via "Fase
     1+2+3 cumpridas; columns/colbreak prosseguem como
     roadmap pós-IMPLEMENTADO".
   - Política P158 preservada (DEBT-56b candidato não-aberto).

4. **Cenário C é mais conservador** mas pode ser preferível
   se humano decidir aguardar materialização completa
   13-15 antes de promover. Spec não impõe — executor decide
   empíricamente.

5. **Custo real esperado**: ~30-45 min (paridade P156K +
   P160A + ADR-0062-create + P244 + P249 administrativos
   XS precedentes). Maior parcela: audit C1 contagem
   cumulativa exacta (~10-15 min) + redacção §"Status final
   P253" cenário A (~15-20 min) + anotações ADR-0061/0080/
   0082/README (~5-10 min) + relatório P253 (~5-10 min).

6. **Sem `P253.div-N` antecipado normal**. 4 cenários
   contingenciais em §2; pouco prováveis.

7. **Anti-inflação 45ª aplicação cumulativa** pós-P205D
   preservar: Opção β L0 minimal (zero L0 tocado) + Opção α
   promoção ADR existente + Opção α anotação cumulativa
   minimal ADRs (0061 + 0080 + 0082) + Opção α paridade
   administrativos XS precedentes + Opção α scope-outs
   formais documentados (cenário A).

8. **Cenário B raro mas possível** se §2.2 audit revelar
   contagem >= 13 literal. Cenário B é mais ambicioso:
   declara "13-15 sub-passos cumpridos cumulativamente"
   sem scope-out. Maior risco interpretativo (cumulativo
   granular vs categórico estricto). Cenário A é mais
   defensível empíricamente.

9. **Pós-P253 cenário A/B**: **ADR-0082 → EM VIGOR humana**
   continua candidato XS sequente (N=3 limiar atingido P252
   preservado independente P253). Decisão humana separada.

10. **Pós-P253 cenário C**: ADR-0079 PROPOSTO continua;
    sub-passos materialização adicionais podem prosseguir
    (D restantes, C.2 multi-region completo). Magnitude
    cumulativa L+ a XL — decisão humana caso-a-caso.

11. **Marco "Layout Fase 5 IMPLEMENTADO" (cenário A/B)**:
    Documentar em relatório P253 §"Marco P253" como milestone
    conceptual paridade ADR-0061 P221. **Terceira ADR roadmap
    a transitar IMPLEMENTADO** (ADR-0060 P155 + ADR-0061 P221
    + ADR-0079 P253 = N=3 cumulativo "ADR Fase X roadmap →
    IMPLEMENTADO via scope-out formal humano"); limiar
    formalização interno atingido — candidato a ADR meta
    formalizar pattern N=3-4 futuro.
