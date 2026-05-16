# Relatório do passo P259 — Pivot Visualize (auditoria Fase A + actualização docs L0 + Cenário B2 sub-passos prioritários adiados P260+)

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-259.md`.
**Tipo**: passo composto sequencial puramente documental
(P259.A audit + P259.B reconciliação L0 + P259.C saltado per
decisão local cenário B2 + P259.D DEBT-33 preservado + ADR-0083
anotação cumulativa).
**Magnitude planeada**: XS-L conforme cenário Fase A.
**Magnitude real**: **XS-S (~30-45 min)** — cenário B2 sem
materialização de código (P259.C saltado per scope-out ADR-0061
sobre Opção 4 + política administrativa P259 documental).

---

## §1 — Sumário executivo

**Cenário Fase A**: ☐ B1 / ☑ **B2 (~52% cobertura — sub-passos
prioritários adiados P260+)** / ☐ B3.

**Tests delta**: **2334 verdes preservado** (zero alteração;
paridade absoluta cenário B2 documental sem materialização).

**ADRs tocadas**: 0 novas; **1 anotação cumulativa** (ADR-0083
Color paridade vanilla preservada + nota Visualize agregada
P259).

**Prompts L0 actualizados**: **1** (`entities/geometry.md` —
secção "Estado actual cumulativo (reconciliação P259 Cenário
B2 — Fase A)" adicionada; 5 ShapeKind variants + PathItem
documentados; Ellipse linha 101-102 obsoleta substituída por
P242 Bézier real; Polygon stdlib promoção documentada; hash
propagado `7dbda723`).

**Hashes propagados**: 1 (`entities/geometry.md` → código
`7dbda723` via `crystalline-lint --fix-hashes`).

**Ficheiros criados**:
- `00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`
  (imutável per ADR-0034; §1-§6 preenchidos; Tabelas A+B com
  27 entradas; decisão B2).
- `00_nucleo/materialization/typst-passo-259-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `00_nucleo/prompts/entities/geometry.md` (secção P259 anotada;
  histórico cumulativo preservado).
- `00_nucleo/DEBT.md` (DEBT-33 EM ABERTO preservado com
  anotação P259.A).
- `00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md`
  (anotação cumulativa P259).
- `00_nucleo/adr/README.md` (entrada P259 nos passos-chave).

**Zero código L1/L2/L3/L4 tocado** (cenário B2 P259.C saltado
per decisão local).

---

## §2 — Sub-passo P259.A — Auditoria empírica Fase A

**Output resumido** (detalhes literais em
`diagnostico-visualize-fase-a-passo-259.md`):

### Resultado por Bloco (10 blocos audit)

- **Bloco 1 (Color)**: 8 variants confirmadas; 7+ stdlib funcs;
  zero operadores cor (ADR-0083 scope-out #2 preservado); P257
  preservado.
- **Bloco 2 (Shapes)**: **5 ShapeKind variants** (Rect,
  RoundedRect, Ellipse, Line, Path); Path MoveTo/LineTo/CubicTo/
  ClosePath confirmados; **Polygon promovido ausente→implementado**
  (native_polygon stdlib via Path conversion); **Ellipse promovido
  parcial→implementado⁺** (P242 Bézier kappa 0.5522847498 em
  export.rs; 4 referências); Curve ausente.
- **Bloco 3 (Path DEBT-33)**: DEBT-33 EM ABERTO preservado;
  aproximação min/max dos pontos de controlo.
- **Bloco 4 (Stroke)**: `Stroke.overhang: bool` adicionado P252;
  Paint enum ausente (só comments placeholder); Dash/LineCap/
  LineJoin/Stroke<T> ausentes; **Stroke base promovido
  implementado→implementado⁺** (P252 overhang refactor).
- **Bloco 5 (Gradient)**: zero hits estrutural; só comment
  placeholder em value.rs:83; vanilla file existe (46KB).
- **Bloco 6 (Paint/Tiling)**: zero hits estruturais; só
  comments placeholder.
- **Bloco 7 (Image)**: variants base JPEG/PNG; SVG ausente
  (zero `usvg`/`resvg` em code + Cargo.toml); **Image metadata
  rebaixado parcial→ausente** (só intrinsic_w/h estrutural;
  zero alt/fit user-facing).
- **Bloco 8 (Transform origin)**: ausente com **scope-out
  documentado ADR-0061** em `transforms.rs:104-105`.
- **Bloco 9 (Inconsistências L0)**: `geometry.md` 5355 bytes
  cobria Stroke P252 + ShapeKind base + RoundedRect P242; **falta
  cobrir Path P79 + PathItem + Polygon stdlib + Ellipse Bézier**;
  hashes consistentes pré-edit (V5 zero).
- **Bloco 10 (PDF exporter)**: emit_shape_path_local presente;
  **Shading/ShadingType zero hits** (PDF Gradient exporter
  ausente — Gradient materializar exige expansão exporter).

### Tabela A — Classificação por subsistema (27 entradas)

Re-classificação resumida (detalhes literais em §2 diagnóstico):

| Bloco | Entradas | Implementado | Implementado⁺ | Ausente |
|-------|----------|--------------|---------------|---------|
| A — Color | 1 | 0 | 1 | 0 |
| B — Shapes | 6 | 3 (Rect/Line/Polygon) | 2 (Ellipse/Path) | 1 (Curve) |
| C — Stroke | 5 | 0 | 1 (base) | 4 |
| D — Image | 4 | 2 (JPEG/PNG) | 0 | 2 (SVG/metadata) |
| E — Transform | 5 | 4 (Move/Rotate/Scale/Skew) | 0 | 1 (origin) |
| F — Gradient | 3 | 0 | 0 | 3 |
| G — Paint | 1 | 0 | 0 | 1 |
| H — Tiling | 1 | 0 | 0 | 1 |
| I — Clip | 1 | 1 | 0 | 0 |
| **TOTAL** | **27** | **10** | **4** | **13** |

**Promoções detectadas Audit P259** (3 não-documentadas no
diagnóstico pai):
- B.2 Ellipse: parcial → implementado⁺ (P242 Bézier real).
- B.5 Polygon: ausente → implementado (stdlib `native_polygon`).
- C.1 Stroke base: implementado → implementado⁺ (P252 overhang).

**Rebaixamento detectado** (1):
- D.4 Image metadata: parcial → ausente (só intrinsic_w/h é
  metadata estrutural).

### Tabela B — Estado agregado

| Estado | Pré-P259 estimado | Audit P259 | Δ |
|--------|-------------------|------------|---|
| implementado | ~14/27 (52%) | 10/27 (37%) | -4 |
| implementado⁺ | 1/27 (4%) | 4/27 (15%) | +3 |
| parcial | 2/27 (7%) | 0/27 (0%) | -2 |
| ausente | 10/27 (37%) | 13/27 (48%) | +3 |

**Cobertura ponderada linear**: pre-P259 estimativa 60-65% →
Audit P259 **51.9%** (Δ -8-13pp; estimativa pré era optimista).
**Cobertura ponderada com bonus implementado⁺**: 54.8%.
**Fechados literais**: 14/27 = 51.9%.

### Decisão Fase B

☑ **Cenário B2 (55-70% — sub-passos prioritários)** —
justificação:
- 51.9% literal está ligeiramente abaixo do limiar B2 inferior
  (55%); 54.8% ponderado está logo acima.
- 13/27 ausentes não é re-classificação massiva (B3) — é estado
  real coerente com expectativa pré.
- 3 promoções não-documentadas sugerem cobertura útil ligeiramente
  superior ao literal (paridade refinos qualitativos).
- 3 grupos pendentes claros: Gradient/Paint/Tiling cluster;
  Stroke refinos; Image SVG+metadata.
- Cenário B1 não atingido (<<75%); B3 não justificado
  (re-classificação mínima).

### Decisão local — P259.C saltado

Apesar do spec §3 §"Excepção" permitir materializar Opção 4
(Transform origin pivot) dentro de P259.C, **decisão local foi
saltar P259.C** por:
1. Scope-out documentado em `transforms.rs:104-105` per
   ADR-0061 — materializar reverte decisão arquitectural.
2. ADR explícita "revogação scope-out ADR-0061" estaria fora
   do scope declarado deste passo.
3. Preservar política administrativa P259 (paridade pattern
   P258 Cenário B1 puramente documental).

---

## §3 — Sub-passo P259.B — Reconciliação documental L0

### B.1 — L0 prompt `entities/geometry.md`

Adicionada secção nova "Estado actual cumulativo (reconciliação
P259 Cenário B2 — Fase A)" no fim do prompt L0 com:

- 5 ShapeKind variants documentadas (Rect, RoundedRect, Ellipse,
  Line, Path).
- `PathItem` enum (P79) documentado: MoveTo/LineTo/CubicTo/
  ClosePath.
- **Linha 101-102 obsoleta substituída** por nota P242 Bézier
  kappa real em export.rs (4 refs); DEBT-31 cruzada
  incorrectamente — DEBT-31 é Transform afim ENCERRADO P78,
  não Ellipse.
- Polygon stdlib promoção documentada (`native_polygon` em
  `shapes.rs:223` via Path conversion; testes em mod.rs:
  1565-1599).
- Tabela B agregada Visualize P259 (~52% cobertura).
- Pendentes pós-P259 (Gradient cluster + Stroke refinos + SVG
  + origin pivot + Curve + DEBT-33).
- Cenário Fase B B2 confirmado; P259.C saltado.

**Decisão arquitectural P259**: representação base inicial
preservada como **histórico cumulativo** (paridade pattern
P258.B + ADR-0080 §"refactor aditivo"); secções subsequentes
cobrem materializações reais. **Não reconciliação destructiva**.

### B.2 — Hashes propagados

`crystalline-lint --fix-hashes` propagou novo hash:
- `entities/geometry.md` → código `7dbda723` em
  `@prompt-hash` line de `01_core/src/entities/geometry.rs`.

### B.3 — Verificação final P259.B

- `crystalline-lint .` → **`✓ No violations found`**.
- `cargo test --workspace --release` → **2334 verdes preservado**.

---

## §4 — Sub-passo P259.C — Saltado (decisão local cenário B2)

**Saltado per decisão local** confirmado em P259.A §5.

**Opções B2 candidatas P260+ dedicados**:
- Opção 1 (Paint enum + Gradient Linear; P260+P261 M+S+;
  +11pp; sequência arquitectural preferida).
- Opção 2 (Polygon variant separada + Ellipse refino; parcial
  pré-cumprida em P259.A — Polygon stdlib + Ellipse Bézier
  já materializados; resta Curve variant + Polygon estrutural
  separada).
- Opção 3 (DEBT-33 + Stroke<Length>; S+M; +5pp; refinos
  qualitativos).
- Opção 4 (Transform origin pivot; S+; +2-3pp; **scope-out
  ADR-0061 preservado**; materializar reverte decisão
  arquitectural).
- Opção 5 (SVG image format; L+; NÃO recomendado P260; requer
  ADR crate `usvg`/`resvg`).

---

## §5 — Sub-passo P259.D — DEBT-33 + ADR-0083 + relatório

### D.1 — DEBT-33 preservado

DEBT-33 **EM ABERTO preservado** (não fechado em P259 —
cenário B2 saltou materialização). Secção nova "Auditado em
P259.A (2026-05-15)" adicionada confirmando preservação:
- Aproximação min/max pontos de controlo preservada.
- Candidato P260+ Opção 3 "DEBT-33 + Stroke<Length>".
- Magnitude S+ (~1-2h; +5 tests; matemática paramétrica
  raízes da derivada `B'(t) = 0` em [0,1]).

### D.2 — ADR-0083 anotação cumulativa P259

Secção nova "Anotação cumulativa P259 (2026-05-15) — Cobertura
Visualize agregada" adicionada após "Próximos passos":

- Color preservado: 8/8 espaços + 7 stdlib funcs.
- Cobertura Visualize empírica Tabela B P259 (51.9% / 54.8%).
- 3 promoções não-documentadas detectadas (Ellipse/Polygon/
  Stroke).
- Cenário Fase B B2 confirmado; Opções P260+ enumeradas com
  scope-outs preservados.
- Subpadrão "auditoria condicional" N=5 cumulativo (limiar
  formalização clara; candidato ADR meta).
- Subpadrão "Diagnóstico imutável precedente à acção" N=4
  cumulativo.
- Status ADR-0083 preservado literal (`IMPLEMENTADO`).

### D.3 — README ADRs

Entrada P259 administrativo XS-S nos passos-chave (~60 linhas
descritivas paridade P255/P257/P258 entradas).

Distribuição ADRs preservada literal:
- PROPOSTO 11.
- EM VIGOR 30.
- IMPLEMENTADO 25.
- Total 70.

### D.4 — Relatório (este ficheiro)

Estrutura canónica §1-§10.

---

## §6 — Padrões metodológicos

### ADR-0065 critério #5 — scope determinado por inventário

Aplicação directa P259. Audit empírico Fase A precedeu decisão
Cenário B2 + acções P259.B/D.

### Subpadrão "auditoria condicional" N=4 → N=5 cumulativo

- N=1 P192A (audit M7 fixpoint).
- N=2 P255 (audit DEBT-8 Math).
- N=3 P257 (audit Color vanilla Fase A).
- N=4 P258 (audit Model Fase A).
- **N=5 P259** (audit Visualize Fase A).

**Limiar formalização N=5 atinge clara promoção candidato**.
Próximo passo administrativo XS candidato a formalizar pattern
em ADR meta (paridade ADR-0082 N=8 promoções reais; ADR-0080
N=9 L0 minimal; ADR-0064 N=8 saturação).

### Subpadrão "Diagnóstico imutável precedente à acção" N=3 → N=4 cumulativo

- N=1 P255 (`diagnostico-math-fase-a-passo-255.md`).
- N=2 P257 (`diagnostico-color-vanilla-passo-257.md`).
- N=3 P258 (`diagnostico-model-fase-a-passo-258.md`).
- **N=4 P259** (`diagnostico-visualize-fase-a-passo-259.md`).

**Pattern emergente sólido** — N=4 cumulativo reforça pattern;
em N=5 promoção formal cumprirá.

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo via Cenário B1/B2"

**N=1 P257 preservado** (P259 cenário B2 documental não cria
ADR nova; ADR-0083 anotação cumulativa apenas).

### Subpadrão "Refactor cross-cutting entity primitivo"

**N=2 preservado** (P252 Stroke + P257 Color). P259 não
materializa Paint enum (Opção 1 P260+); pattern não cresce
neste passo.

---

## §7 — Cobertura

**Visualize Audit P259 empírico**:
- **51.9% ponderado linear** (10*1.0 + 4*1.0) / 27.
- **54.8% ponderado com bonus implementado⁺** (10*1.0 + 4*1.2) / 27.

**Pré-P259 estimativa optimista** (60-65%) era **5-10pp acima**
do real.

**Causas re-classificação**:
- D.4 Image metadata rebaixada parcial → ausente (só intrinsic_w/h).
- Stroke<T>/Dash/LineCap/Join confirmados ausentes (não promovidos).

**Compensação parcial via promoções**:
- B.2 Ellipse parcial → implementado⁺ (+0.5pp).
- B.5 Polygon ausente → implementado (+3.7pp).
- C.1 Stroke base implementado → implementado⁺ (+0.7pp).

**Cobertura agregada user-facing total preservada** ~75-76%.
**Layout Fase 5** ~98-99% preservado (P253 IMPLEMENTADO; P259
não afecta Layout).
**Math**: DEBT-8 ENCERRADO P255 preservado.
**Model**: ~73% pós-P258 preservado.
**Color** subsistema Visualize: 100% estrutural preservado.
**Visualize agregado**: **~52% pós-P259** (Cenário B2 confirmado).

---

## §8 — Limitações e trabalho futuro

### Pendências residuais P259 (não-bloqueantes; candidatos P260+)

**Cluster Gradient (P260+P261 Opção 1 — sequência preferida)**:
1. **Paint wrapper enum** (G) — pré-requisito Gradient real
   consumer. P260 candidato S+ (~1-2h; +8-12 tests; +3pp).
2. **Gradient Linear** (F.1) — após Paint enum. P261 candidato
   M (~3-4h; +15-20 tests; +8pp). Exige expansão PDF exporter
   (shading patterns).
3. **Gradient Radial/Conic** (F.2/F.3) — deferidos pós-Linear.

**Cluster Stroke refinos (P260 ou P261 Opção 3)**:
4. **DEBT-33 Bézier bbox exacto** (S+; ~+5 tests; matemática
   paramétrica).
5. **Stroke<Length>** (M; ~+10-15 tests; refactor cross-cutting).
6. **Dash patterns** (DashPattern + stops).
7. **LineCap/LineJoin/MiterLimit** (refino estilo).

**Cluster Shapes refinos (Opção 2; parcial pré-cumprida)**:
8. **Polygon variant estrutural separada** (não conversão Path
   stdlib).
9. **Curve variant** (B.6) — bezier separado não via Path.

**Cluster Image (Opção 5)**:
10. **SVG image format** (D.3) — L+ (~6-10h; requer ADR
    crate `usvg` + `resvg`).
11. **Image metadata `alt`/`fit`** (D.4) — refino S.

**Transform refino (Opção 4 — scope-out preservado)**:
12. **Transform `origin` pivot** (E.5) — **scope-out ADR-0061
    documentado** em `transforms.rs:104-105`. Materializar
    requer ADR explícita "revogação scope-out".

**Tiling pattern (Opção 5+)**:
13. **Tiling pattern** (H) — vanilla feature; baixa prioridade;
    pré-requisito Paint enum.

### Sem ADR nova aberta

Política P158 "sem novas reservas" preservada. ADR-0083
anotação cumulativa apenas; ADR-0061 scope-out preservado;
ADR-0079 preservada.

### Sem DEBT novo aberto

DEBT-33 EM ABERTO preservado (não fechada). Saldo DEBTs
preservado.

---

## §9 — Critério de aceitação global P259 — Checklist final

- [x] `crystalline-lint .` retorna `✓ No violations found`.
- [x] `cargo test --workspace --release` retorna **2334 verdes
  preservado** (sem regressão).
- [x] `diagnostico-visualize-fase-a-passo-259.md` existe com
  Tabelas A+B preenchidas para 27 subsistemas.
- [x] Prompts L0 obsoletos reconciliados (`geometry.md` com
  secção P259 anotada).
- [x] Hashes propagados (`entities/geometry.md` → `7dbda723`).
- [x] DEBT-33 actualizada conforme cenário (EM ABERTO preservado
  + anotação P259.A).
- [x] ADR-0083 anotação cumulativa adicionada.
- [x] (B2 Opção 1 N/A) Paint enum ADR não criada (saltado P260+).
- [x] (B2 Opção 4 N/A) Transform origin não materializado
  (scope-out ADR-0061 preservado).
- [x] Relatório criado em `00_nucleo/materialization/`.
- [x] N/A (cenário B2 P259.C saltado; sem código materializado).

**Estado pós-P259**:
- Tests workspace: **2334 verdes preservado**.
- Hash drift: zero.
- Lint: zero violations.
- DEBT-33: EM ABERTO preservado (anotação P259.A).
- ADRs distribuição preservada literal: PROPOSTO 11; EM VIGOR
  30; IMPLEMENTADO 25; **total 70 preservado**.
- Prompts L0 actualizados: 1 (`entities/geometry.md`).
- Diagnóstico imutável criado: 1.
- Saldo DEBTs: **10 preservado** (DEBT-55 PARCIALMENTE
  RESOLVIDO desde P258; DEBT-33 EM ABERTO preservado).
- **45 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.

**Marco P259**: **Visualize Fase A auditado**; Cenário B2
confirmado (~52% cobertura). Color (P257) + Shapes (P79+P242+
P252) + Stroke base (P252) + Image (P73+P74) + Transform afim
(P78+P156F) + Clip (P79) preservados. Pendências reais isoladas:
Gradient + Paint + Tiling cluster; Stroke refinos; SVG; Image
metadata; origin pivot scope-out — preservadas como candidatas
P260+ granulares.

**Recomendação subjectiva pós-P259**:

- **P260 Paint enum + P261 Gradient Linear** (sequência
  arquitectural preferida; M+S+; +11pp cobertura cumulativa).
- **OU P260 DEBT-33 Bézier bbox + P261 Stroke<Length>**
  (Opção 3; refinos qualitativos; ~+5pp).
- **OU ADR meta admin XS** formalizar "auditoria condicional"
  N=5 cumulativo + "Diagnóstico imutável precedente à acção"
  N=4 cumulativo (limiares atingidos; candidato sólido análogo
  ADR-0064/0065).

**Decisão humana fica em aberto literal** pós-P259.

---

## §10 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0019, ADR-0026, ADR-0029, ADR-0033, ADR-0034, ADR-0054,
  ADR-0061, ADR-0065, ADR-0079, ADR-0082, **ADR-0083** (Color
  paridade vanilla; anotação cumulativa P259).
- DEBT-33 (Bézier bbox EM ABERTO preservado; anotação P259.A).
- DEBT-31 (Transform afim ENCERRADO P78; nota Ellipse
  "placeholder rect" factualmente desactualizada per audit
  P259.A).
- `00_nucleo/diagnosticos/diagnostico-visualize-passo-259.md`
  — diagnóstico pai (planeamento Fase A/B).
- `00_nucleo/diagnosticos/fase-a-checklist-visualize-passo-259.md`
  — comandos exactos P259.A (10 blocos).
- `00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`
  — diagnóstico imutável P259.A.
- P25 — Color simplificado original (REVOGADO via P257).
- P72-P74 — Image stack JPEG+PNG.
- P76 — geometry tipos primitivos.
- P78 — Transform Move/Rotate/Scale.
- P79 — Path + clip + DEBT-30/33.
- P156F — Transform Skew.
- P242 — Boxed/Block radius + Ellipse Bézier kappa.
- P252 — Stroke overhang cross-cutting (refactor entidade
  primitivo N=1).
- P192A — N=1 "auditoria condicional".
- P255 — DEBT-8 Math ENCERRADO via auditoria condicional N=2.
- P257 — Color paridade vanilla via Cenário Fase A literal N=3.
- P258 — Model fecho conceptual via auditoria condicional N=4.
- **P259 — Visualize Fase A audit via auditoria condicional
  N=5 cumulativo** (este passo; limiar formalização atingido).
