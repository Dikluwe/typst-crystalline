# Relatório do passo P221 — Encerramento Fase 3 Layout (DEBT-56 fecha; ADRs IMPLEMENTADO)

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-221.md`.
**Tipo**: passo documental puro (zero código tocado);
transições de status ADR + DEBT + recálculos cumulativos.
**Magnitude planeada**: S (~30min-1h). **Magnitude real**: S
(~50min).
**Marco**: **fecho de série Layout Fase 3** — primeiro
encerramento de Fase Layout pós-M9c (paridade estrutural
P156I Fase 2 fechada pré-M9c + P155 Fase 1 Model fechada
pré-M9c; pattern emergente N=2 cumulativo formalizado em
§3.0duodecies).

---

## §1 O que foi feito

P221 fecha cirúrgicamente a série Layout Fase 3 cumulativa
materializada P217+P218+P219+P220 (sub-fase b DEBT-56)
precedida por P216A+P216B (sub-fase a DEBT-56) e P215
(diagnóstico). **2 ADRs transitam** PROPOSTO → IMPLEMENTADO
(ADR-0078 column flow algorithm + ADR-0061 Layout Fase X
roadmap). **1 DEBT fecha** (DEBT-56 ENCERRADO via
materialização; saldo 14 → 13). Inventário 148 cumulativo
actualizado (Tabela B.2 Content +Columns/Colbreak; §A.5
zero ausentes Layout; footnote ⁴² consolidada preservando
⁴⁰+⁴¹). Blueprint §2.1 Layout Opção γ (78% + nota
explícita "12 impl + 5 parcial"); §3.0duodecies marca-
por-fecho adicionada. Tests **1987 verdes preservados**;
**0 violations**; "Nothing to fix" hashes. Sem `P221.div-N`.

---

## §2 Auditoria pré-P221 (C1)

Verificação empírica antes de transições:

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo test --workspace` | 1987 verdes | **1987 verdes** (1698 + 242 + 24 + 2 + 21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| Content variants | 56 | **56** (Columns + Colbreak adicionados) ✓ |
| Arms `Columns`/`Colbreak` em `layout/mod.rs` | presentes | 7 ocorrências ✓ |

Estado factual P220 baseline confirmado integralmente. Sem
divergência; sem `P221.div-1`.

---

## §3 Transições ADR (C2 + C3)

**ADR-0078 column flow algorithm**: PROPOSTO (P215
2026-05-12) → **IMPLEMENTADO** (P221 2026-05-12). Mesmo
dia; trajectória rápida pós-M9c.

- Cabeçalho: `Status` + `Data` + `Materialização` (7
  relatórios P216A-P221) + `Validado` (Opção B graded
  literal; multi-region flow real Opção A scope-out).
- Bloco `### P221 encerramento série 2026-05-12`
  adicionado após P220 anotação (antes de "## Alternativas
  consideradas") com:
  - Resumo cumulativo P216A+B+P217-P220.
  - **6 condições §"Plano materialização" satisfeitas
    explicitamente** (paridade ADR-0075 7 cond; ADR-0073
    9 cond):
    1. ✓ Region + Regions abstractions (P216A+B).
    2. ✓ Content::Columns + Content::Colbreak variants (P217+P220).
    3. ✓ native_columns + native_colbreak stdlib (P218+P220).
    4. ✓ Layouter consumer Opção B graded (P219+P220).
    5. ✓ Test suite multi-column verde (44 tests Fase 3).
    6. ✓ Multi-region flow real scope-out documentado.
  - Pattern N=4 "L0 minimal" + N=1 "encerramento Fase
    Layout pós-M9c" registados sem promoção formal.

**ADR-0061 Layout Fase X roadmap**: PROPOSTO (P156B
2026-04-25) → **IMPLEMENTADO** (P221 2026-05-12).
Trajectória ~17 dias, 12 sub-passos materializados (P156C-L
+ P216A-P220 + P221).

- Cabeçalho: `Status` + `Data` + `Validado` (P156B
  diagnóstico + P156C-L + P216A-P220 + P221).
- §"Status" actualizada — Caminho 1 100% cumprido (Fase
  1 4/4 + Fase 2 3/3 + Fase 3 sub-passo 1+2 + sub-fase a
  2/2 + sub-fase b 4/4); refinos `measure`/`place` ficam
  como **Fase 4 Layout candidata NÃO-reservada** per
  política P158.
- Bloco `### P221 encerramento Fase 3 2026-05-12`
  adicionado após §"Status" com materialização cumulativa
  série Layout completa (4+3+2+2+4+1 sub-passos =
  16 cumulativos).

**Distribuição ADRs pós-P221**: PROPOSTO 13 → **11**;
IMPLEMENTADO 19 → **21** (paridade `00_nucleo/adr/README.md`
actualizado).

---

## §4 DEBT-56 ENCERRADO (C4)

`00_nucleo/DEBT.md` editado:

- **Título DEBT-56**: "Column flow Fase 3 Layout (L+;
  refactor multi-region do Layouter) — **ENCERRADO (Passo
  221) ✓**".
- **Fechado em**: 2026-05-12 (P221).
- **Resolvido por**: Sub-fases (a) refactor Region/Regions
  (P216A+B) + (b) consumer real graded (P217-P220)
  materializadas.
- **Critério de fecho 5/5 satisfeito explicitamente**:
  - ✅ ADR column flow `IMPLEMENTADO` (ADR-0078 P221).
  - ✅ `columns()` + `colbreak()` materializados (Opção B
    + Opção β graded).
  - ✅ Tests verdes (1987 workspace; 0 regressões).
  - ✅ Lint zero (0 violations; "Nothing to fix" hashes).
  - ✅ ADR-0061 Fase 3 marca columns/colbreak completos
    (Caminho 1 100% IMPLEMENTADO P221).
- **Saldo DEBTs**: pré-P221 14 abertos → pós-P221 **13
  abertos**.
- **Histórico preservado** abaixo do título com marca
  [HISTÓRICO] (paridade pattern P201/P202+P206E
  "histórico textual preservado").

CLOSED via materialização — paridade pattern P206E
(DEBT-53 fechado via materialização; precedente cumulativo
agora N=2 — DEBT-53 + DEBT-56).

---

## §5 Inventário 148 actualização cumulativa (C5)

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
editado em 4 secções:

**§A.5 Layout** (linhas 140 + 144):
- `columns(n)`: footnotes `⁴⁰` → `⁴⁰ ⁴²`; coluna "Referência"
  passa a "Passos 217 + 218 + 219 (encerrado P221)".
- `colbreak()`: footnotes `⁴¹` → `⁴¹ ⁴²`; coluna "Referência"
  passa a "Passo 220 (encerrado P221)".

**Tabela A** (linha 431 + 436):
- Layout footnotes: `⁴⁰ ⁴¹` → `⁴⁰ ⁴¹ ⁴²` (⁴² P221
  consolidada adicionada; ⁴⁰+⁴¹ preservadas histórico).
- Distribuição §A.5 final: **`12/1/5/0/0 = 18`** (preservada
  P220 — zero ausentes Layout).
- Total user-facing: `68/24/27/20/2 = 141` (preservado
  P220).
- Cobertura: Layout 13/18 = **72% per metodologia §A.9**;
  user-facing total 92/141 ≈ **65%**.

**Tabela B.2 Content variants**: 2 entradas novas adicionadas
após Quote (linha 322):
- `Columns {count, gutter, body}` ↔ ColumnsElem (`parcial`;
  Passo 221 cumulativo P217-P219).
- `Colbreak {weak}` ↔ ColbreakElem (`parcial`; Passo 220).
- Linha "Vanilla-only (ausentes)" actualizada — `ColumnsElem`
  + `ColbreakElem` removidos.
- Tabela B Content variants pós-P221: 54 → **56**.

**Footnote ⁴² P221 consolidada** (~50 linhas) adicionada
imediatamente após ⁴¹, antes de ³⁹ (lugar consistente com
ordem cumulativa §"Footnotes"). Documenta:
- Reapresentação consolidada das reclassificações
  cumulativas (`columns` + `colbreak` ausente → parcial).
- Distribuição §A.5 final + cobertura 72%/78% Opção γ.
- Tabela B Content 54 → 56; stdlib funcs 53 → 55.
- Tests workspace cumulativo P216A-P220: 1939 → 1987 (+48).
- Multi-region flow real Opção A scope-out per ADR-0078.
- Transições ADR (0078 + 0061) + DEBT-56 ENCERRADO.
- Distribuição ADRs PROPOSTO 13 → 11; IMPLEMENTADO 19 → 21.
- Footnotes ⁴⁰+⁴¹ preservadas (paridade P204H+ histórico).

**Decisão Opção γ §"Footnotes"**: **manter ⁴⁰ + ⁴¹ + ⁴²
em conjunto** (não consolidar removendo ⁴⁰+⁴¹). Reduz
inflação acumulativa mas preserva rastreabilidade
P217-P220 incremental — paridade pattern "histórico
textual preservado" P204H+.

---

## §6 Blueprint §2.1 Opção γ + §3.0duodecies (C6)

`00_nucleo/diagnosticos/blueprint-projecto.md` editado em
2 sítios:

**§2.1 linha Layout** (linha 195 — Opção γ fixada):

Antes:
```
| Layout ⁽ᴾ²¹⁴⁾ | 78% | quase total (Fase 1+2+3 sub-passo 1 fechadas) |
```

Depois:
```
| Layout ⁽ᴾ²¹⁴⁾⁽ᴾ²²¹⁾ | 78% (12 impl + 5 parcial) | quase total — **Fase 3 fechada estructuralmente P221** (DEBT-56 ENCERRADO; ADR-0078+ADR-0061 IMPLEMENTADO); refinos `measure`/`place` Fase 4 candidata NÃO-reservada |
```

Opção γ: paridade visual histórica 78% mantida + nota
explícita distribuição "12 impl + 5 parcial" (per
descoberta auditada P220). Decisão fixada — primeira
aplicação pattern; possível reuso futuro em outras
categorias com distribuição não-óbvia.

**§3.0duodecies Marca de actualização — [P221]
Encerramento Fase 3 Layout (DEBT-56 fecha; ADRs
IMPLEMENTADO)** adicionada após §3.0undecies (P214) e
antes de §3.1 (paridade pattern marca-por-fecho
§3.0quater→§3.0nonies + §3.0decies/undecies):

- Distinção qualitativa face a marcas anteriores:
  encerramento de Fase Layout pós-M9c (não série, não
  marco, não recálculo).
- 3 transições de status (ADR-0078 + ADR-0061 IMPLEMENTADO;
  DEBT-56 ENCERRADO).
- Mudanças factuais cumulativas P216A-P220 (2 variants;
  2 stdlib funcs; 1 type novo; ~325 substituições
  mecânicas; 1 arm refactored; 1 arm aditivo; 48 tests
  novos cumulativos).
- Pattern emergente "encerramento Fase pós-M9c" N=1
  formalizado.
- Pattern emergente "L0 minimal para refactors" N=4 →
  candidatura formal (Caminho 4 §8 deferido).
- Política "sem novas reservas" preservada (Fase 4 Layout
  + Opção A multi-region documentadas mas NÃO reservadas).

---

## §7 Resultados verificação (C7+C8+C9)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo test --workspace` | 1987 preservado | **1987 verdes** ✓ (zero código tocado) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 prompts não tocados) |
| ADR-0078 status IMPLEMENTADO | sim | ✓ (linha 3 + bloco encerramento) |
| ADR-0061 status IMPLEMENTADO | sim | ✓ (linha 3 + §"Status" + bloco encerramento Fase 3) |
| DEBT-56 ENCERRADO | sim | ✓ (título DEBT-56 anotado) |
| Inventário 148 §A.5 footnotes ⁴² | sim | ✓ (linhas 140/144/431/436 anotadas) |
| Blueprint §3.0duodecies + §2.1 P221 | sim | ✓ (marca + Opção γ adicionadas) |
| README ADRs distribuição | 11/21 | ✓ ADR-0061 + ADR-0078 entries actualizadas |

**Auditoria cumulativa final** (5 verificações C9):
1. ✓ ADR-0078 status IMPLEMENTADO.
2. ✓ ADR-0061 status IMPLEMENTADO.
3. ✓ DEBT-56 ENCERRADO.
4. ✓ Inventário 148 §A.5 Layout linha actualizada (⁴²).
5. ✓ Blueprint §3.0duodecies + §2.1 marcador P221.

---

## §8 Decisão humana caminhos 1-5 em aberto (C10)

P221 fecha Fase 3 Layout estructuralmente (DEBT-56 ENCERRADO;
ADR-0078+ADR-0061 IMPLEMENTADO). Decisão humana sobre
próxima sessão entre opções:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **Fase 4 Layout** — refinos `measure`/`place` (sub-passos isolados S+ cada; depende ADR-0066 PROPOSTO para measure) | varia | média (Layout 78% → 89% se ambos materializados; isolados de refactor) |
| **Caminho 2** | Pivot Bloco C P222 — `measure(body)` stdlib expose (single feature subset Caminho 1) | S+ (~1-2h) | média (win rápido §A.9 estricto 83% → 100%) |
| **Caminho 3** | Pivot outro módulo (Visualize 54%; Text features 52%; Markup 78%; Model 50%) | varia | baixa-média |
| **Caminho 4** | ADR meta administrativa — formalizar pattern "L0 minimal para refactors" N=4 + "encerramento Fase pós-M9c" N=1 | XS (~30min) | baixa (passo administrativo paridade P213/P214/P160A) |
| **Caminho 5** | Adiar Layout completo; outro objectivo arquitectural (e.g. paridade vanilla retomada per ADR-0073/0075 já ACEITES) | varia | baixa |

**Recomendação subjectiva**:
- Se humano quer continuar Layout: **Caminho 1** ou
  **Caminho 2** (subset isolado).
- Se humano quer consolidar metodologia: **Caminho 4**
  (ADR meta documentaria padrões N=4 + N=1 emergentes).
- Se humano quer pivot: **Caminho 3** (Visualize ou Text
  features mais baixos % cobertura).

**Estado humano fixado pré-P221**: "focar no Layout até
onde der". Caminho 1/2 alinha; Caminho 4 desvia para
documentação. Caminho 3/5 representa mudança de
prioridade.

**Decisão humana fica em aberto literal**. P221 não
compromete trabalho subsequente per política "sem novas
reservas" P158.

**Estado pós-P221**:
- **Sub-fase (a) DEBT-56**: 2/2 ✓ (P216A + P216B).
- **Sub-fase (b) DEBT-56**: 4/4 ✓ (P217 + P218 + P219 +
  P220).
- **DEBT-56 ENCERRADO** (CLOSED via materialização).
- **ADR-0078 IMPLEMENTADO**; **ADR-0061 IMPLEMENTADO**.
- **Distribuição ADRs**: PROPOSTO 13 → 11; IMPLEMENTADO
  19 → 21.
- **Saldo DEBTs**: 14 → 13 abertos.
- Layout cobertura: **78% per metodologia** (estável;
  ganho qualitativo via 2 reclassificações ausente →
  parcial cumulativas P219+P220; **zero ausentes Layout
  pós-P220 preservado**).
- Tests workspace: **1987 verdes**; `crystalline-lint`:
  **0 violations**.
- Cumulativo P216A+B+P217+P218+P219+P220+P221 = ~325
  substituições + 2 variants + 18 arms exhaustivos +
  2 stdlib funcs + 1 helper + 1 constante + 1 arm
  refactored + 1 arm aditivo + 41 tests novos código +
  P221 documental puro em **7 sessões**.
- **15 aplicações cumulativas anti-inflação pós-P205D**
  (P220 sub-passo agregado; P221 não inova mas preserva
  política).
- **Pattern emergente "L0 minimal para refactors" N=4**
  formalizado em §3.0duodecies (P217+P218+P219+P220 todos
  Opção γ). Promoção formal Caminho 4 candidato.
- **Pattern emergente "encerramento Fase Layout pós-M9c"
  N=1 inaugurado** em P221. Reusável Fase 4 candidata.
- **Pattern "sub-passo agregado paridade precedente" N=2**
  (P156E + P220) — anti-inflação fundamentada.
- **Pattern "co-habitação L0 N=2" estável** (P216B Region
  + Regions; precedente P156H Box+Boxed).
- **Pattern "stub transparente → consumer real graded"
  N=1 estável** (P217+P218+P219 série).
- **Marco M9c preservado como referência arquitectural
  estável** (P207-P212 + P213/P214 recálculos +
  P215+P216A-P221 Layout Fase 3).
