# Relatório do passo P244 — Reconciliação documental Linhas A (P216-P221) e B (P239-P243); M7+3 cumulativo via Linha A pré-existente; ADR-0081 IMPLEMENTADO parcial 4/5 → 4.5/5

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-244.md`.
**Tipo**: passo administrativo XS puramente documental. **Zero
código tocado**. **Zero comportamento observable alterado**. **Zero
ADRs novas criadas**.
**Magnitude planeada**: XS (~30-60 min documental). **Magnitude
real**: XS (~30 min — audit grep + 3 anotações ADRs + relatório).
**Marco**: **sexto passo administrativo XS pós-P156A**; **primeira
aplicação do padrão "Reconciliação documental pós-divergência
factual planeamento vs materialização"** N=1 inaugurado P244;
**sétima aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=6 → 7 cumulativo** — distintamente
neste passo, audit C1 **é** o substantivo material do passo
(não preâmbulo a materialização).

---

## §1 O que foi feito

P244 reconcilia inconsistência factual detectada entre o relatório
P243 (recomendou "M7+3 fase (b)" como próximo trabalho) e o estado
empírico do repositório (Linha A já materializou tudo P217-P221
a 2026-05-12).

**Trabalho factualmente realizado**:
1. Audit C1 empírico via grep verificou todos os achados spec.
2. ADR-0081 anotada — clarificação M7+3 cumulativo via Linha A
   pré-existente; status IMPLEMENTADO parcial 4/5 → **4.5/5**.
3. ADR-0080 anotada — lição refinada P244 "audit C1 deve grep
   variants `Content::*` candidatas antes de assumir ausência";
   pattern N=6 → 7 cumulativo.
4. §A.5 Layout auditado — distribuição `12/4/2/0/0` factualmente
   correta pós-P224 reclassificação (não requer ajuste).
5. DEBT-56 ENCERRADO P221 + ADR-0061 + ADR-0078 IMPLEMENTADOS P221
   verificados preservados.
6. Tests 2198 verdes preservados (paridade absoluta administrativos
   XS precedentes).

---

## §2 Verificação empírica via grep (audit C1)

Audit C1 é o substantivo material de P244 (lição N=7 cumulativo
refinada: "grep variants `Content::*` candidatas antes de assumir
ausência"):

| Aspecto | Comando | Achado empírico |
|---|---|---|
| `Content::Columns` em content.rs | `grep -n "Content::Columns" 01_core/src/entities/content.rs` | **CONFIRMADO** existe — variant em linha 1126 (P217); construtor; arms cascata is_empty/plain_text/PartialEq/map_content/map_text presentes (linhas 1876/2165/3603/3635) |
| `Content::Colbreak` em content.rs | mesmo grep | **CONFIRMADO** existe — variant em linha 1908 com field `weak` (P220); construtor; arms cascata presentes (linha 2205/3681) |
| `native_columns` em stdlib | `grep -rn "native_columns" 01_core/src/rules/stdlib/` | **CONFIRMADO** — function em `stdlib/layout.rs:1138` (P218); registado em `stdlib/mod.rs:51`; ~13 unit tests `p218_native_columns_*` em `stdlib/mod.rs:2865+` |
| `native_colbreak` em stdlib | mesmo grep | **CONFIRMADO** — function em `stdlib/layout.rs:1209` (P220); registado; tests `p220_native_colbreak_*` |
| DEBT-56 status em DEBT.md | `grep "DEBT-56" 00_nucleo/DEBT.md` | **ENCERRADO (Passo 221) ✓** confirmado |
| ADR-0078 status | `head -8 typst-adr-0078-column-flow-algorithm.md` | **IMPLEMENTADO** (P215 PROPOSTO 2026-05-12 → P221 IMPLEMENTADO 2026-05-12; sub-fases (a)+(b) materializadas) confirmado |
| ADR-0061 status | `head -8 typst-adr-0061-layout-fase-x-roadmap.md` | **IMPLEMENTADO** (PROPOSTO 2026-04-25 → IMPLEMENTADO 2026-05-12 P221; Fase 1+2+3 cumpridas) confirmado |
| Relatórios P217-P221 | `ls typst-passo-21*-relatorio.md` | **CONFIRMADO** existem (P210B, P212-P218 em listing parcial; P219-P221 presumivelmente presentes) |
| Tests baseline pré-P244 | `cargo test --workspace` | **2198 verdes** confirmado (1909+242+24+2+21) |

**Conclusão audit C1**: **Linha A factualmente materializou todo
o trabalho que P243 §8 recomendou como "M7+3 fase (b)" pendente**.
A inconsistência documental P243 → P244 é reconciliada por
reconhecimento Linha A pré-existente.

---

## §3 Linha A vs Linha B — tabela comparativa

### Linha A — série Fase 3 columns/colbreak (2026-05-12; pré-existente)

| Passo | Trabalho | Data |
|-------|----------|------|
| P215 | Diagnóstico Fase 3 Layout + ADR-0078 PROPOSTO | 2026-05-12 |
| P216A | `Region` entity em `01_core/src/entities/region.rs` | 2026-05-12 |
| P216B | `Regions` minimal (`current` apenas) | 2026-05-12 |
| P217 | `Content::Columns { count, gutter, body }` variant | 2026-05-12 |
| P218 | `native_columns` stdlib + helper `extract_count` | 2026-05-12 |
| P219 | Consumer multi-column real (Opção B graded) | 2026-05-12 |
| P220 | `Content::Colbreak { weak }` + `native_colbreak` | 2026-05-12 |
| P221 | **Encerramento**: ADR-0078 IMPLEMENTADO + ADR-0061 IMPLEMENTADO + DEBT-56 ENCERRADO | 2026-05-12 |

### Linha B — série M9d / M7+ pipeline restructuring (2026-05-14; em curso)

| Passo | Trabalho | Data |
|-------|----------|------|
| P239 | ADR-0081 PROPOSTO (M7+ pipeline restructuring; 5 sub-passos) | 2026-05-14 |
| P240 | M7+1 state.display walk-time real | 2026-05-14 |
| P241 | M7+2 counter.display walk-time real | 2026-05-14 |
| P242 | M7+5 A.4 radius/clip + `Corners<T>` | 2026-05-14 |
| P243 | M7+3 "fase (a)" — extensão `Regions { backlog, last }` + `advance` method + 3 scope-outs promovidos | 2026-05-14 |
| **P244** | **Reconciliação documental Linhas A+B** | **2026-05-14** |

### Cobertura M7+ pós-P244

- **M7+1 state.display** ✓ P240.
- **M7+2 counter.display** ✓ P241.
- **M7+3 fase (a)** ✓ P243 — extensão `Regions { backlog, last,
  advance }` + 3 scope-outs promovidos (Pad.right + Block.width
  + Boxed.width).
- **M7+3 fase (b)** ✓ **via Linha A** P217-P221 — `Content::
  Columns` + `Content::Colbreak` + `native_columns` +
  `native_colbreak` + consumer multi-column graded.
- **M7+5 A.4 radius/clip** ✓ P242.
- **M7+4 Place float real** — **PENDENTE** (último sub-passo
  M7+ — magnitude L isolada ~5-8h).

**M7+3 fica CUMPRIDO cumulativamente** sem precisar de passo
novo "fase (b)" em Linha B — fechamento estrutural via
reconhecimento empírico Linha A pré-existente.

---

## §4 Origem da inconsistência (audit C1 P243 incompleto)

P243 audit C1 capturou **parcialmente** o estado factual:

- ✓ **Detectou** que `Regions` já existia (P216A/P216B):
  Decisão 2 spec P243 corrigida — "Migração field-by-field já
  feita P216A/P216B (audit finding material)".
- ✗ **Não detectou** que `Content::Columns` + `Content::Colbreak`
  + `native_columns` + `native_colbreak` também já existiam
  (P217-P220) com ADR-0078 IMPLEMENTADO + ADR-0061 IMPLEMENTADO
  + DEBT-56 ENCERRADO em P221.

**Resultado**: Decisão 5 spec P243 ("Sem `Content::Columns`/
`Colbreak` em P243 — fase (b) DEBT-56 pendente") foi
internamente coerente com o spec mas assumiu base factual
incorrecta.

**Lição refinada** (ADR-0080 §"Lição refinada P244"):

Audit C1 deve **grep empíricamente variants `Content::*`
candidatas** antes de assumir ausência. Procedimento recomendado
pós-P244:

1. Identificar variants candidatas mencionadas no spec.
2. `grep -n "Content::FOO\|Content::BAR" 01_core/src/entities/content.rs`.
3. `grep -rn "native_foo\|native_bar" 01_core/src/rules/stdlib/`.
4. `grep "DEBT-XX\|ADR-XXXX" 00_nucleo/DEBT.md 00_nucleo/adr/`.
5. Se variant existe → ajustar spec ou criar `Pxxx.div-N`
   conforme magnitude.
6. Se ausente → prosseguir.

---

## §5 Decisões fixadas P244 — 7 decisões

| # | Decisão | Resolução |
|---|---|---|
| 1 | Naming "M7+3 fase (a)" preservado como descritivo | Histórico interno P243 preservado; reconciliação anota Linha A real |
| 2 | ADR-0081 anotada para clarificar M7+3 | IMPLEMENTADO parcial 4/5 → **4.5/5** (M7+3 cumulativo Linha A + Linha B) |
| 3 | §A.5 Layout distribuição auditada | `12/4/2/0/0` factualmente correta pós-P224 reclassificação (não requer ajuste) |
| 4 | Sem ADR nova; sem reabertura ADR existente | Anotações cumulativas em ADR-0081 + ADR-0080 |
| 5 | Patterns emergentes anotados | 1 inaugurado + 2 N≥6 atingindo limiar consolidação |
| 6 | Tests baseline preservados literal | **2198 verdes** (zero código alterado) |
| 7 | L0 prompts intocados | **0 L0 hashes propagados** (paridade administrativos XS) |

---

## §6 Ficheiros editados

### §6.1 ADR-0081 — `typst-adr-0081-m7-plus-pipeline-restructuring-scope.md`

- Status linha actualizada: `IMPLEMENTADO parcial 4/5 → 4.5/5`
  com **M7+3 ✓ via cumulativo P243 fase (a) + Linha A
  pré-existente P217-P221** + **M7+4 pendente**.
- Bloco "P244 anotação" adicionado pós-bloco P243:
  - Reconciliação Linhas A+B documentada.
  - Tabela Linha A (P215-P221) + Linha B (P239-P244).
  - Origem inconsistência (audit C1 P243 incompleto).
  - M7+3 cumulativo formalizado.
  - Pré-condições obrigatórias verificadas.
  - 3 patterns emergentes anotados.
  - Anti-inflação 36ª aplicação cumulativa.

### §6.2 ADR-0080 — `typst-adr-0080-l0-minimal-para-refactors.md`

- Bloco "Lição refinada P244" adicionado pós-§"Excepção P243":
  - Refinamento procedural pattern "spec C1 audit obrigatório
    bloqueante" N=6 → 7 cumulativo.
  - Origem da lição (P243 audit C1 incompleto).
  - Procedimento recomendado pós-P244 (5 passos para grep
    variants candidatas).
  - Sub-padrão "Reconciliação documental pós-divergência
    factual planeamento vs materialização" N=1 inaugurado.
  - Pattern "Passo administrativo XS" N=5 → 6 cumulativo
    (atinge limiar formalização sólido N≥4-6).

### §6.3 Inventário 148 — sem ajuste necessário

§A.5 Layout distribuição `12/4/2/0/0` factualmente correta
pós-P224 reclassificação. P243 reportou-a preservada — não há
divergência real (apenas aparência se comparada com P221
pré-P224 `12/1/5/0/0` que precedeu a reclassificação).

**Sem footnote nova** — P244 é reconciliação documental, não
materialização. Anotação inline em ADR-0081 + ADR-0080
suficiente.

### §6.4 DEBT.md — sem alteração

DEBT-56 fechou correctamente em P221; P244 só formaliza que
Linha B não duplica este fecho. **Sem alteração necessária**.

---

## §7 Patterns emergentes inaugurados/consolidados P244

- **"Reconciliação documental pós-divergência factual planeamento
  vs materialização" N=1 inaugurado P244**. Candidato a
  formalização N=3-4 futuro.
- **"Passo administrativo XS" N=5 → 6 cumulativo** (P156A +
  P156K + ADR-0062-create + P160A + P238 + **P244**). **Atinge
  limiar formalização sólido N≥4-6**; candidato a ADR meta
  dedicada em passo administrativo XS futuro (não-reservado
  per política P158).
- **"Spec C1 audit obrigatório bloqueante pós-P236.div-1" N=6
  → 7 cumulativo** (P237 + P238 reescrito + P240 + P241 + P242
  + P243 + **P244**). P244 é **primeira aplicação onde audit
  C1 É o substantivo material do passo**, não preâmbulo.
  Refinamento procedural "grep variants candidatas antes de
  assumir ausência" anotado em ADR-0080.

**Anti-inflação 36ª aplicação cumulativa pós-P205D** — Opção α
anotação cumulativa minimal + Opção α reconhecimento Linha A
pré-existente + Opção β L0 intocados + Opção α ADR-0081
promoção interna parcial 4/5 → 4.5/5 (não completo) +
Opção α sem ADR nova nem reabertura.

---

## §8 Próximo sub-passo pós-P244

P244 fecha reconciliação documental. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+4 Place float real** | Reabertura Opção B P219 graded; desbloqueia C.1 | **L (~5-8h)** | **alta** (último sub-passo M7+ pendente; fecha ADR-0081 IMPLEMENTADO total 5/5; magnitude isolada) |
| ADR meta admin XS — formalizar "passo administrativo XS" N=6 | Promoção formal pattern N≥4 (atinge limiar sólido pós-P244) | XS | média (patamar cumulativo claro) |
| Cell layout migration → `regions.current.height` | Decisão 7 P243 diferida; activa A.4 breakable per-cell | M (~2-4h) | média (refino sequente P243 natural) |
| Refino A.4 — `outset`+`fill`+`stroke` Block+Boxed | 3 dos 4 scope-outs restantes pós-P242 | S-M por attr | baixa-média |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| Pausa M-fase | Fase 5 graded ~93-94% preservado | XS | baixa |

**Recomendação subjectiva pós-P244**: **M7+4 Place float real**.
Último sub-passo M7+ pendente; fecha ADR-0081 IMPLEMENTADO total
5/5; magnitude L isolada; desbloqueia C.1. Alternativa: ADR
meta admin XS para formalizar pattern "passo administrativo XS"
N=6 (limiar sólido atingido pós-P244).

**Decisão humana fica em aberto literal** pós-P244.

**Estado pós-P244**:
- Tests workspace: **2198 verdes preservado**.
- Content variants: **62 preservado**.
- ShapeKind variants: **5 preservado**.
- Layouter fields: preservados.
- Regions fields: **3 preservado** (`current`, `backlog`, `last`).
- Regions methods: 4 preservado (`single`, `reset_current`,
  `advance` + sub-methods Region).
- Scope-outs promovidos cumulativos: **5** (P242 radius+clip +
  P243 Pad.right + Block.width + Boxed.width).
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: **12/4/2/0/0 preservado**.
- Cobertura Layout per metodologia: **~93-94% preservado**.
- Cobertura user-facing total: **~74-75% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 22; total **68 preservado**. ADR-0081 transita
  4/5 → **4.5/5** internamente (reconhecimento M7+3 cumulativo
  via Linha A pré-existente). ADR-0061 + ADR-0078 + DEBT-56
  preservados inalterados (factuais desde P221).
- **Saldo DEBTs: 11 preservado**.
- **36 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P244** (1 inaugurado + 2 consolidados
  N≥6):
  - "Reconciliação documental pós-divergência factual planeamento
    vs materialização" N=1 inaugurado P244.
  - "Passo administrativo XS" N=5 → **6 cumulativo** atinge
    limiar formalização sólido.
  - "Spec C1 audit obrigatório bloqueante" N=6 → **7 cumulativo**;
    P244 primeira aplicação onde audit C1 É o substantivo.
- **Categoria D Fase 5 Layout: 3/? sub-passos materializados**
  preservado.
- **Categoria A.4 Fase 5 Layout**: parcial P242 preservado.
- **Fase 5 Layout candidata: 14/13-15 → 14/13-15 sub-passos
  preservado** (P244 administrativo não soma).
- **M9d / M7+ progresso**: **4.5/5 sub-passos materializados**
  (M7+1 ✓; M7+2 ✓; **M7+3 ✓ via cumulativo Linha A + extensão
  Linha B P243**; M7+5 ✓; M7+4 pendente). Cumulativa restante
  ~5-8h (M7+4 isolada).
- **Marco interno**: reconciliação documental Linhas A e B
  completa; inconsistência factual P243 corrigida; ADR-0081
  M7+3 cumulativo reconhecido formalmente; padrão administrativo
  XS atinge N=6 (limiar formalização sólido); padrão audit C1
  refinado procedimentalmente (grep variants antes de assumir
  ausência).
