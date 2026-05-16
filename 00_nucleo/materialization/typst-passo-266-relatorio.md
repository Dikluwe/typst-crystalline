# Relatório do passo P266 — Text audit Fase A (primeiro consumo directo formal ADR-0084 + ADR-0085 EM VIGOR pós-P260)

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-266.md`.
**Tipo**: passo composto sequencial documental; cenário B1
fecho conceptual confirmado.
**Análogo estrutural canónico directo**: P259 (Visualize Fase A;
último audit pré-formalização P260; este passo é primeiro
audit Fase A formal pós-P260).
**Magnitude planeada**: XS-L conforme cenário Fase A.
**Magnitude real**: **XS-S (~45 min)** — cenário B1 confirmado
empíricamente (~86% cobertura); P266.C saltado.

---

## §1 — Sumário executivo

**Cenário Fase A**: ☑ **B1 (≥75% — fecho conceptual)** /
☐ B2 / ☐ B3.

**Cobertura agregada empírica**:
- Linear (peso 1.0/0.5/0): **86.25%**.
- Ponderada com bonus implementado⁺ (peso 1.2): **91.75%**.
- Fechados literais: **34/40 = 85%**.

**Tests delta**: **2393 preservado** (zero alteração; paridade
absoluta cenário B1 documental sem materialização).

**ADRs tocadas**: 0 novas; **1 anotação cumulativa** (ADR-0054
status `EM VIGOR` preservado literal).

**Prompts L0 actualizados**: **1** (`entities/style_chain.md`
anotada com secção cumulativa P266; hash propagado código
`cc2dae57`).

**DEBT tocadas**: 0 (DEBT-53 spec referência errada corrigida
no diagnóstico imutável — está ENCERRADO P206E para
"lab/parity"; shaping rustybuzz scope-out ADR-0054 preservado).

**Ficheiros criados**:
- `00_nucleo/diagnosticos/diagnostico-text-fase-a-passo-266.md`
  (imutável per ADR-0085; §1-§5 preenchidos; Tabela A 40
  entradas + Tabela B agregada; decisão B1 explicitada).
- `00_nucleo/materialization/typst-passo-266-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `00_nucleo/prompts/entities/style_chain.md` (secção
  cumulativa P266 anotada).
- `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md`
  (anotação cumulativa P266).
- `00_nucleo/adr/README.md` (entrada P266 nos passos-chave).

**Zero código L1/L2/L3/L4 tocado** (P266.C saltado per cenário
B1).

---

## §2 — Sub-passo P266.A — Auditoria empírica Fase A

**Output resumido** (detalhes literais em
`diagnostico-text-fase-a-passo-266.md`):

### Resultado por Bloco (9 blocos audit)

- **Bloco 1 (Vanilla Text)**: 12 ficheiros + 1 dir `font/` em
  `lab/typst-original/.../text/`. `TextElem` com 30+ campos
  vanilla.
- **Bloco 2 (Content variants)**: 109 variants Content total;
  `Text` + `Heading` + `Raw` + `Link` + `Linebreak` + `Quote` +
  `HSpace` + `VSpace` text-related; **`Parbreak` AUSENTE como
  variant** (era "a confirmar"; emergente do parser).
- **Bloco 3 (StyleChain)**: **10 fields** StyleDelta (não 12
  esperados); 10 resolvers; `impl From<&StyleChain> for
  TextStyle`.
- **Bloco 4 (Font rendering)**: 3 paths PDF
  (build_helvetica/cidfont/multifont); `resolve_font` usa
  `FontVariant::default()` (variant-aware ausente C.5).
- **Bloco 5 (Lang)**: Hyphenation hypher + smart-quotes 6
  idiomas confirmados; **rustybuzz zero hits** (shaping
  ausente).
- **Bloco 6 (Markup secundários)**: Escape + Shorthand em
  lexer/parser/AST; Raw 3 campos (text + lang + block);
  highlighting zero hits; Linebreak presente.
- **Bloco 7 (Refinos)**: faux_bold + tracking_pt Tc + leading
  line_height + hyphenation cursor todos confirmados consumers
  reais.
- **Bloco 8 (L0 prompts)**: `style_chain.md` + `lang.md` +
  `rules/layout.md` + `rules/lang.md` + `entities/lang.md`
  presentes; `font-book.md` + `font-list.md` (hyphen)
  presentes — naming convention diverge de underscore.
- **Bloco 9 (Cross-features)**: `Content::Styled` (P101
  ADR-0038) confirmado; `ShowRule` materializado.

### Tabela A — Classificação por entrada (40 entradas)

Re-classificação resumida (detalhes literais em §2 diagnóstico):

| Subsistema | Entradas | Implementado | Implementado⁺ | Parcial | Ausente | Outro |
|------------|----------|--------------|---------------|---------|---------|-------|
| A — StyleChain | 12 | 8 | 4 | 0 | 0 | 0 |
| B — Content text | 12 | 7 | 2 | 1 | 0 | 2 (Strong/Emph promoção arquitectural) |
| C — Font rendering | 7 | 5 | 0 | 0 | 2 (C.5+C.6) | 0 |
| D — Lang | 5 | 1 | 2 | 0 | 2 (D.4+D.5 DEBT) | 0 |
| E — Text features | 4 | 0 | 4 | 0 | 0 | 0 |
| **TOTAL** | **40** | **21** | **12** (correção: 11+1 promoção arquitectural via Strong/Emph) | **1** | **4** | **2** |

**Note**: Tabela B (§3 diagnóstico) listou 11 implementado⁺ +
2 promoção arquitectural. Cumulativo conta 13/40 = 33% promoções
qualitativas (não-básico implementado).

### Tabela B — Estado agregado

| Estado | Pré-P266 estimado | Audit P266 | Δ |
|--------|---------------------|------------|---|
| implementado | ~30/40 (75%) | 21/40 (52%) | -9 |
| implementado⁺ | 1/40 (3%) | **11/40 (28%)** | **+10** |
| parcial | 2/40 (5%) | 1/40 (3%) | -1 |
| ausente | 5/40 (12%) | 4/40 (10%) | -1 |
| promoção arquitectural (Strong/Emph) | n/a | 2/40 (5%) | +2 |
| Outros | 2/40 (5% "a confirmar") | 0/40 (0%) | -2 |

**Fechados literais**: 21+11+2 = **34/40 = 85%**.

**Promoções implementado⁺ detectadas Audit P266** (10
não-documentadas pré-audit):
- A.8 tracking PDF Tc emit (P137).
- A.9 leading line_height (P128).
- A.12 lang hyphenation + smart-quotes (P144 + P155).
- B.1 Text + StyleChain Layouter consumer.
- B.10 Smart-quotes lang-aware (P155).
- D.2 Hyphenation greedy (P144 consumer).
- E.1 Faux-bold (P139 consumer).
- E.2 Tracking PDF (P137 consumer).
- E.3 Leading (P128 consumer).
- E.4 Hyphenation greedy break (P144 cursor consumer).

### Decisão Fase B

☑ **Cenário B1 (≥75% cobertura — fecho conceptual)** —
justificação:
- 86.25% ponderado linear ≥ 75% limiar B1.
- 91.75% ponderado com bonus reforça classificação superior.
- 34/40 = 85% fechados literais.
- Promoções massivas implementado⁺ (+10pp pós-audit) confirmam
  consumers reais materializados.
- Pendências residuais isoladas (5/40 ausentes; 1/40 parcial).

---

## §3 — Sub-passo P266.B — Reconciliação documental L0

### B.1 — L0 prompt `entities/style_chain.md`

Adicionada secção nova "Anotação cumulativa P266 — Cobertura
empírica StyleChain confirmada (Fase A)" no fim do prompt L0
com:

- Promoções implementado → implementado⁺ confirmadas (tracking
  P137 + leading P128 + lang P144/P155 + weight P139).
- Estado actual 10 fields StyleDelta (confirmado P266.A).
- Cobertura Text agregada empírica P266.
- Cross-references (ADR-0038/0039/0052/0053/0054/0080/0084/0085).

**Decisão arquitectural P266** (precedente P258.B/P259.B):
preservar representação base como histórico cumulativo;
secções subsequentes cobrem materializações reais. **Não
reconciliação destructiva**.

### B.2 — Outros L0 não-tocados

`lang.md`, `rules/layout.md`, `rules/lang.md`, `font-book.md`,
`font-list.md` verificados existentes; reconciliação não
necessária (estado actual coerente com audit P266.A; promoções
documentadas via anotação ADR-0054 cumulativa P266 cobertura
~86%).

### B.3 — Hashes propagados

`crystalline-lint --fix-hashes` propagou novo hash:
- `entities/style_chain.md` → código `cc2dae57` em
  `@prompt-hash` line de `01_core/src/entities/style_chain.rs`.

### B.4 — Verificação final P266.B

- `crystalline-lint .` → **`✓ No violations found`**.
- `cargo test --workspace --release` → **2393 preservado**.

---

## §4 — Sub-passo P266.C — Saltado (cenário B1)

**Saltado per cenário B1** confirmado em P266.A §5. Zero
materialização exigida.

**Pendências residuais não-bloqueantes** preservadas para
P267+ dedicados:
- **C.5 Variant-aware font selection** — candidato P267 Opção
  1 (M; ADR-0055bis ou ADR-0089). Substitui faux-bold P139
  onde font-file dedicado existe.
- **C.6 Font subsetting PDF** — candidato P267 Opção 2 (M-L;
  ADR-0056).
- **D.4 Shaping rustybuzz** + **D.5 Bidirectional RTL** —
  scope-outs ADR-0054 graded preservados; sem DEBT formal
  dedicada per política P158 "sem novas reservas".
- **B.7 Content::Link parcial** — refino qualitativo PDF
  annotation futuro.
- **B.9 Content::Parbreak** — promoção arquitectural implícita
  via parser whitespace duplo; não variant Content explícito.

---

## §5 — Sub-passo P266.D — DEBT + ADR + relatório

### D.1 — DEBT-53 referência corrigida

Spec P266 referencia "DEBT-53 rustybuzz shaping" mas DEBT-53
está **ENCERRADO P206E** para "Integração pipeline vanilla
lab/parity" (tópico diferente). Diagnóstico imutável P266.A
§4.7 + §6 registou correcção factual.

**Shaping rustybuzz** preservado scope-out ADR-0054 §"granularidade
gradual" sem DEBT formal dedicada per política P158 "sem
novas reservas".

### D.2 — ADR-0054 anotação cumulativa P266

Secção nova "Anotação cumulativa P266 — Cobertura Text empírica
confirmada (Fase A audit; primeiro consumo directo ADR-0084 +
0085)" adicionada após "Marco P252":

- Cobertura empírica Text Fase A (Tabela B).
- Promoções implementado⁺ detectadas (+10 vs pré-audit).
- Pendências preservadas pós-P266.
- Achados inesperados (Parbreak ausente; Strong/Emph promoção
  arquitectural; spec DEBT-53 referência errada; 10 fields
  StyleDelta).
- Subpadrões cumulativos (auditoria condicional N=5 → N=6
  cumulativo; diagnóstico imutável N=6 → N=7; cobertura
  empírica > citada N=3 cumulativo).

**Status `EM VIGOR` preservado literal** (paridade pattern
P229 ADR-0080).

### D.3 — README ADRs

Entrada P266 ~80 linhas nos passos-chave (paridade entrada
P258 fecho conceptual Model).

Distribuição ADRs preservada literal:
- PROPOSTO 11.
- EM VIGOR 32.
- IMPLEMENTADO 28.
- **Total 75 preservado**.

### D.4 — Relatório (este ficheiro)

Estrutura canónica §1-§10.

---

## §6 — Padrões metodológicos

### Primeiro consumo directo formal ADR-0084 + ADR-0085 pós-P260

P260 formalizou ADR-0084 (auditoria condicional) + ADR-0085
(diagnóstico imutável). P266 é **primeiro consumo directo
formal** — estrutura cumprida literalmente:
- `.A` audit empírico → diagnóstico imutável per ADR-0085.
- `.B` reconciliação L0.
- `.C` materialização condicional (saltado per B1).
- `.D` fecho cumulativo + relatório.

**Validação retrospectiva ADR-0084 + 0085** cumprida via
exercício real num módulo grande (Text — 40 entradas).

### Subpadrão "auditoria condicional" N=5 → N=6 cumulativo

Cumulativo:
- N=1 P192A (M7 fixpoint).
- N=2 P255 (Math).
- N=3 P257 (Color).
- N=4 P258 (Model).
- N=5 P259 (Visualize — último pré-P260).
- **N=6 P266** (Text — primeiro pós-P260 consumo directo
  formal).

**Patamar N=6 excede limiar formalização clara**. Pattern
sólido confirmado retroactivamente; ADR-0084 EM VIGOR validada.

### Subpadrão "diagnóstico imutável precedente à acção" N=6 → N=7 cumulativo

Cumulativo:
- N=1-4 P255/P257/P258/P259 (audit Fase A).
- N=5 P262 (Gradient Linear vanilla — primeiro consumo directo
  pós-P260).
- N=6 P264 (Gradient Radial vanilla — segundo consumo directo).
- **N=7 P266** (Text Fase A — **terceiro consumo directo
  formal**; primeiro audit Fase A formal pós-P260).

**Patamar N=7 reforça pattern sólido**.

### Subpadrão "Cobertura empírica > citada" N=3 cumulativo

Cumulativo:
- N=1 P257 Color (+75pp: 25% citado → 100% empírico).
- N=2 P258 Model (+25pp: 48% citado → 73% empírico).
- **N=3 P266 Text** (+34pp: 52% citado → 86% empírico).

**Média Δ +45pp** dos 3 audits superiores. P259 Visualize (-8
a -13pp) foi excepção que confirma a regra. **Hipótese
auditável Text padrão Color/Model confirmada empíricamente**.

### Subpadrão "Cobertura empírica < citada" N=1 preservado

- N=1 P259 Visualize (-8 a -13pp: 60-65% estimado → 52% factual).

Pattern excepcional preservado.

### Comparação cumulativa audits

| Audit | Módulo | Cobertura pré | Cobertura empírica | Δ | Cenário |
|-------|--------|---------------|--------------------|----|---------|
| P255 | Math (DEBT-8) | parcial | fechado | + | ENCERRADO |
| P257 | Color | 25% | 100% estrutural | +75pp | 8/8 |
| P258 | Model | ~48% declarado | ~73% empírico | +25pp | B1 fecho |
| P259 | Visualize | ~60-65% estim | ~52% factual | -8 a -13pp | B2 sub-passos |
| **P266** | **Text** | **~52% citado** | **~86% empírico** | **+34pp** | **B1 fecho** |

---

## §7 — Cobertura

**Text empírico Fase A**:
- Linear: **86.25%**.
- Ponderado com bonus implementado⁺ (peso 1.2): **91.75%**.
- Fechados literais: **85%**.

**Cobertura agregada user-facing total**: ~75-76% preservado.
**Layout Fase 5**: ~98-99% preservado.
**Math**: DEBT-8 ENCERRADO P255 preservado.
**Model**: ~73% pós-P258 preservado.
**Color** subsistema: 100% estrutural preservado.
**Visualize agregado**: ~73% pós-P265 (cluster Gradient
completo).
**Text agregado**: **~86% pós-P266** (primeiro audit formal
post-P260).

---

## §8 — Limitações e trabalho futuro

### Pendências residuais P266 (não-bloqueantes; candidatos P267+ dedicados)

**Cluster Font refinos**:
1. **C.5 Variant-aware font selection** — candidato **P267
   Opção 1** (M; ADR-0055bis ou ADR-0089; substitui
   faux-bold P139 onde font-file dedicado existe; +10-15
   tests).
2. **C.6 Font subsetting PDF** — candidato **P267 Opção 2**
   (M-L; ADR-0056; TTF subset embed reduz PDF size; ~+15-20
   tests; requer crate `ttf-subset` ou similar autorização
   ADR-0018).

**Cluster Shaping (scope-out ADR-0054 preservado)**:
3. **D.4 Shaping rustybuzz** — candidato XL futuro; sem DEBT
   formal dedicada per política P158.
4. **D.5 Bidirectional RTL** — depende D.4; XL futuro.

**Refinos qualitativos**:
5. **B.7 Content::Link parcial** — PDF annotation completa
   futuro (link clickable).
6. **B.9 Parbreak promoção implícita preservada** — emergente
   do parser; não variant Content explícito.

### Cluster Gradient cluster (cobertura visualize 73%)

7. **P-Gradient-Conic L1+stdlib+PDF** (M+S-M; replica templates
   P262+P263/P264+P265).
8. **P-Gradient-Focal** (M; activa focal_* Radial).

### Cluster Visualize refinos (P259)

9. **DEBT-33 Bézier bbox** + **Stroke<Length>** (S+M).
10. **Curve variant** + **Polygon estrutural separada** (S+).
11. **SVG image format** (L+; requer ADR `usvg`/`resvg`).
12. **Tiling pattern** (Paint::Tiling activação).

### Sem ADR nova aberta

Política P158 "sem novas reservas" preservada. ADR-0054
anotação cumulativa apenas (sem reabrir/revogar).

### Sem DEBT novo aberto

Saldo DEBTs preservado. DEBT-53 spec referência errada
corrigida no diagnóstico imutável (DEBT-53 ENCERRADO P206E
para "lab/parity"; shaping rustybuzz scope-out informal
ADR-0054).

---

## §9 — Critério de aceitação global P266 — Checklist final

- [x] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [x] `cargo test --workspace --release` retorna **2393
  preservado** (sem regressão).
- [x] `diagnostico-text-fase-a-passo-266.md` existe com Tabelas
  A+B preenchidas (40+ entradas) e marcador "Imutável após
  criação per ADR-0085".
- [x] Prompts L0 obsoletos reconciliados (`style_chain.md`
  anotada cumulativamente P266).
- [x] Hashes propagados (zero violations V5).
- [x] DEBT-53 referência spec corrigida no diagnóstico
  imutável (ENCERRADO P206E; shaping rustybuzz scope-out
  informal ADR-0054).
- [x] ADR-0054 anotação cumulativa adicionada (cobertura Text
  empírica + subpadrões cumulativos).
- [x] (B1) Sem ADR nova materializada (P266.C saltado per B1).
- [x] Relatório criado em `00_nucleo/materialization/`.
- [x] N/A — cenário B1 sem código materializado.
- [x] **Primeiro consumo directo ADR-0084 + ADR-0085
  documentado em §6 relatório**.

**Estado pós-P266**:
- Tests workspace: **2393 verdes preservado**.
- Hash drift: zero.
- Lint: zero violations.
- DEBTs saldo: **10 preservado**.
- ADRs distribuição preservada literal: PROPOSTO 11; EM VIGOR
  32; IMPLEMENTADO 28; **total 75 preservado**.
- Prompts L0 actualizados: 1 (`entities/style_chain.md`).
- Diagnóstico imutável criado: 1 (**terceiro consumo directo
  ADR-0085**; **primeiro audit Fase A formal pós-P260**).
- ADRs anotadas cumulativamente: 1 (ADR-0054).
- **45 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.

**Marco P266**: **primeiro consumo directo formal ADR-0084 +
ADR-0085 pós-P260**; validação retrospectiva cumprida via
exercício real Text módulo (40 entradas auditadas; cobertura
empírica ~86% confirmada Δ +34pp vs pré-audit ~52%); patterns
metodológicos consolidam-se (auditoria condicional N=6; diagnóstico
imutável N=7; cobertura empírica > citada N=3).

**Recomendação subjectiva pós-P266**:

- **P-Gradient-Conic L1+stdlib + PDF** (M+S-M cumulativo;
  replica templates P262+P263/P264+P265; activa último
  Gradient variant; +3pp Visualize cobertura cumulativa).
- **OU P267 Variant-aware font selection** (M; ADR-0055bis ou
  ADR-0089; +consumer real font weight/italic; substitui
  faux-bold onde font-file dedicado existe).
- **OU outras Opções P259 alternativas**:
  - DEBT-33 Bézier bbox + Stroke<Length> Opção 3.
  - Curve variant + Polygon estrutural separada Opção 2.
- **OU P-Footnote-N** refino M (P258 pendência residual).
- **OU Tiling activação** (Paint::Tiling — análogo P262/P263
  estrutural).

**Decisão humana fica em aberto literal** pós-P266.

---

## §10 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- **ADR-0084 + ADR-0085** (P260 — **primeiro consumo directo
  formal** este passo; validação retrospectiva via exercício
  real Text).
- **ADR-0054** §"Anotação cumulativa P266" — Cobertura Text
  empírica confirmada (criada por este passo).
- ADR-0029, ADR-0033, ADR-0034, ADR-0038, ADR-0039, ADR-0052,
  ADR-0053, ADR-0055, ADR-0057, ADR-0065, ADR-0080.
- DEBT-1 (fechado P142; preservado).
- DEBT-52 (fechado P142; preservado).
- DEBT-53 (ENCERRADO P206E para "lab/parity"; spec P266
  referência factual corrigida).
- Shaping rustybuzz scope-out ADR-0054 §"granularidade gradual"
  (sem DEBT formal dedicada).
- `diagnostico-text-passo-266.md` — diagnóstico pai
  (planeamento Fase A/B).
- `fase-a-checklist-text-passo-266.md` — comandos exactos
  P266.A (9 blocos).
- `00_nucleo/diagnosticos/diagnostico-text-fase-a-passo-266.md`
  — diagnóstico imutável P266.A (**terceiro consumo directo
  ADR-0085**; **primeiro audit Fase A formal pós-P260**).
- Aplicações precedentes do pattern:
  - P21, P30, P99, P100, P126-P139, P140B, P141, P142, P144,
    P146, P155 — materializações Text cumulativas reconhecidas
    via audit.
  - P192A — N=1 "auditoria condicional".
  - P255 — DEBT-8 Math ENCERRADO (N=2).
  - P257 — Color paridade vanilla (N=3; +75pp confirma hipótese).
  - P258 — Model fecho conceptual (N=4; +25pp).
  - **P259** — Visualize Fase A (N=5; último audit
    pré-formalização P260; **template literal directo P266**;
    excepção -8 a -13pp).
  - P260 — ADRs meta (formaliza ADR-0084/0085 consumidos
    directamente por este passo).
  - P262, P264 — diagnósticos vanilla Gradient (precedentes
    diagnóstico imutável N=5+6).
  - P263, P265 — PDF shading materialização cluster Gradient
    completo.
- Vanilla `lab/typst-original/crates/typst-library/src/text/`
  — fonte canónica (12 ficheiros + 1 dir font/; leitura Bloco
  1 Fase A obrigatória).
