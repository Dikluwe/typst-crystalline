# Passo 233 — B.1 DEBT-34d Auto track sizing fix (Fase 5 Layout candidata Categoria B 1/3; fecha DEBT-34d preservado per `P224.div-1`)

**Série**: 233 (décimo-nono sub-passo Layout pós-M9c;
**sexto sub-passo materialização Fase 5 Layout candidata**
per ADR-0079 PROPOSTO; **primeiro sub-passo Categoria B**
"algorítmicos isolados"; **primeira fecho de DEBT preservado
conscientemente pós-M9c** — DEBT-34d preservado P224.div-1
há 18 sub-passos; quarta aplicação automática ADR-0080 EM
VIGOR pós-P229).
**Marco**: nenhum status ADR; **DEBT-34d FECHADO**; saldo
DEBTs 12 → **11**; **pattern emergente "fecho de DEBT
preservado conscientemente em sub-passo posterior" N=1
inaugurado P233** (paridade conceitual com pattern
"divergência factual material `Pxxx.div-N`" mas distinto
— fecho de DEBT vs registo de divergência); pattern
"aplicação automática ADR-0080 EM VIGOR" N=3 → 4 cumulativo;
**primeiro sub-passo Categoria B algorítmico pós-Fase 5
Categoria A fechada**.
**Tipo**: refino algorítmico puro a `layout_grid` (ou
`grid_placement.rs` per audit C1); **zero fields novos**
em Content variants; **zero novas stdlib funcs**;
implementação de algoritmo "auto" track sizing
(measure-based pre-pass).
**Magnitude**: M (~2-3h; paridade diagnóstico P226 B.1).
**Pré-condição**: P232 concluído (A.5 Place per-cell;
Categoria A 5/5 ✓ fechada estructuralmente; 2106 verdes;
0 violations; saldo DEBTs 12); humano fixou B.1 (decisão
literal pós-P232 §8); `Content::Grid { columns: Vec<TrackSizing>,
... }` baseline P224.A com `TrackSizing` enum incl.
`Auto` variant; `measure_content` helper baseline P222 +
P83 (constrained measure); `place_cells` algorítmico
P224.C; **DEBT-34d preservado P224.div-1** documentado
em `00_nucleo/DEBT.md` (audit C1 obrigatório); pattern
"aplicação automática ADR-0080 EM VIGOR" N=3 baseline
P230+P231+P232.
**Output**: 1 ficheiro relatório curto + código alterado
em ~3-5 ficheiros L1 + L0 NÃO tocado (quarta aplicação
automática ADR-0080 EM VIGOR) + DEBT-34d **FECHADO** +
P224.div-1 anotada **RESOLVIDA P233** + inventário 148
anotação cumulativa (footnote ⁵²) + ADR-0079 anotação
**Categoria B 1/3** (B.1 ✓; B.2+B.3 pendentes).

---

## §1 Trabalho

DEBT-34d preservado conscientemente em P224 per
`P224.div-1`. P226 diagnóstico Categoria B.1 marcou
literal: **"DEBT-34d Auto track sizing fix (M)"**. P233
fecha DEBT-34d implementando algoritmo "auto" track
sizing real.

**Audit C1 obrigatório**: ler DEBT-34d completo em
`00_nucleo/DEBT.md`. **Decisão crítica C1**: P233 cobre
EXACTAMENTE o que DEBT-34d documenta sem expansão
escopo. Se DEBT-34d for amplo, **atomização ADR-0036
aplicada** — P233 cobre primeiro sub-item; restantes
ficam DEBT-34d-rest preservado.

**Hipótese provável (sujeita a audit C1)**:
- DEBT-34d documenta "Auto track sizing por measure" — `TrackSizing::Auto`
  variant existe baseline P224.A mas implementação atribui
  tamanho fixo (e.g., remaining/n) em vez de measure-based.
- Algoritmo correcto vanilla: **pre-pass measure** das cells
  → max per track → distribuir remaining proporcionalmente
  (fr fractional weight se mix).

**P233 materializa B.1**:
- **Pre-pass measure** das cells via `measure_content`
  baseline P222 + P83 (constrained measure).
- **Max per track** auto (e.g., column auto width = max
  cell width na coluna).
- **Distribuição remaining space** entre tracks fr (se mix
  auto + fr existe).
- **Fecho DEBT-34d** formal pós-P233.
- **P224.div-1 anotada RESOLVIDA P233**.

**Decisão arquitectural central — 8 decisões fixadas**:

### Decisão 1 — Interpretação literal DEBT-34d (audit C1 crítico; condicional)

3 opções consideradas (estructurais):

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | P233 cobre DEBT-34d completo (escopo literal) | Subset minimal; coerente atomização |
| β | P233 cobre apenas auto sizing simples (excluir multi-span/mixed) | Atomização excessiva se DEBT-34d unitário |
| γ | P233 expande escopo (incluir DEBT-34d + relacionados) | Inflacionário; viola atomização ADR-0036 |

**Decisão fixada — Opção α (pendente audit C1)**: P233
cobre EXACTAMENTE escopo DEBT-34d documenta. Se DEBT-34d
for amplo (sub-itens múltiplos), **atomização ADR-0036**
— P233 primeiro sub-item; restantes DEBT-34d-rest
preservado.

### Decisão 2 — Algoritmo auto sizing Opção α (measure-based pre-pass)

Vanilla `auto` algoritmo standard:
1. Pre-pass: para cada cell em track auto, medir conteúdo
   (`measure_content`).
2. Track size auto = max(measures de cells na track auto).
3. Distribuir remaining space (total - sum(auto+fixed))
   proporcionalmente entre fr tracks.
4. Final pass: placement com tamanhos definitivos.

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Pre-pass measure two-pass (vanilla standard) | Algoritmo correcto; magnitude M |
| β | Single-pass com estimativa fixed | Resultado incorrecto; viola paridade vanilla |
| γ | Pre-pass measure + iterative refinement | Inflacionário; magnitude L+ |

**Decisão fixada — Opção α** (two-pass standard):
- Pre-pass measure: usa `measure_content` baseline P222 +
  P83.
- Two-pass paridade vanilla literal.
- Reuso `measure_content` N=N+1 (audit C1 confirma
  contagem actual).
- **Pattern emergente "algoritmo two-pass measure→place"
  N=1 inaugurado P233** — primeiro algoritmo two-pass
  cristalino pós-M9c.

### Decisão 3 — Localização do algoritmo Opção β (consolidar `layout_grid`)

Audit C1 determina exactamente. 3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | `grid_placement.rs::place_cells` (P224.C) | Algorítmico isolado mas placement-only; auto sizing precede placement |
| **β** | `layout_grid` baseline P224 + refinos cumulativos | Consolida lógica; preferência anti-inflação |
| γ | Novo módulo `track_sizing.rs` | Inflacionário; viola anti-inflação |

**Decisão fixada — Opção β** (consolidar em `layout_grid`):
- Pre-pass measure como step inicial dentro de `layout_grid`.
- Sem novo módulo (anti-inflação).
- Refactor inline pre-pass + adaptar lógica placement
  pre-existente para usar tamanhos calculados.

**Audit C1 confirma**: se algoritmo actual já está
distribuído entre `layout_grid` e `place_cells`, P233
adiciona pre-pass no início de `layout_grid` antes do
call para `place_cells`.

### Decisão 4 — Distribuição remaining space para fr tracks

Vanilla fr weight: tracks fr partilham remaining space
proporcionalmente ao weight.

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | `(total - sum(fixed+auto)) * fr_i / sum(fr)` | Vanilla standard; trivial |
| β | Apenas fixed; fr não-implementado em P233 | DEBT-34d sub-item; atomização |
| γ | Iterative com min/max constraints | Inflacionário |

**Decisão fixada — Opção α** (vanilla standard):
- Cálculo trivial `remaining / total_fr * fr_i` per
  track fr.
- Audit C1 confirma se fr está em escopo DEBT-34d ou
  separado.

Se DEBT-34d cobre apenas auto (não fr), Opção β atomização
— P233 implementa auto; fr fica DEBT-34d-rest preservado.

### Decisão 5 — Tests E2E auto sizing

Crítico testar algoritmo two-pass:
- Grid `columns: auto`, 1 col, 3 cells diferentes →
  largura = max cell.
- Grid `columns: (auto, auto)`, 2 cells por row → cada
  col independente.
- Grid `columns: (1fr, auto)` mix → fr e auto coexistem.
- Grid `columns: auto`, cell vazia → largura zero.
- Grid colspan > 1 → cell distribuída entre tracks (audit
  C1; pode estar fora escopo DEBT-34d).

**Decisão fixada — 5-7 tests E2E auto sizing** (paridade
hipótese; ajuste pós-audit C1).

### Decisão 6 — DEBT-34d fecho formal

Pós-P233:
- DEBT-34d status `ABERTO` → **`FECHADO`** (paridade
  pattern DEBT-30 fechado).
- Referência cruzada bidirecional: DEBT-34d ↔ P233
  relatório.
- Saldo DEBTs: 12 → **11** (-1).

**Decisão fixada — fecho formal**:
- Anotação DEBT-34d com bloco `## Fecho P233` documentando.
- Referência cruzada em DEBT.md + P233 relatório.

### Decisão 7 — `P224.div-1` resolved

`P224.div-1` documentou "DEBT-34d preservado consciente".
Pós-P233, divergência **resolvida**.

**Decisão fixada — anotação `P224.div-1` status RESOLVIDA
P233**:
- Editar diagnóstico onde P224.div-1 está registado
  (provável `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  ou `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`).
- Adicionar marker `[RESOLVIDA P233]` ao registo P224.div-1.
- Pattern emergente "fecho retrospectivo de divergência
  factual em sub-passo posterior" N=1 inaugurado P233.

### Decisão 8 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação automática N=4)

**Decisão fixada — aplicação automática quarta pós-P229**:

P233 é refactor algorítmico puro:
- Zero fields novos em Content variants.
- Zero novas stdlib funcs.
- Zero alterações a Value enum.
- Apenas algoritmo interno em `layout_grid`.

ADR-0080 EM VIGOR §"Decisão" aplica-se por defeito.
Pattern "aplicação automática ADR EM VIGOR sem decisão
explícita por sub-passo" N=3 → **4 cumulativo**
(P230+P231+P232+**P233**).

L0 prompts NÃO tocados.

Reuso de dados (sem recolha nova):

- `Content::Grid { columns: Vec<TrackSizing>, ... }`
  baseline P224.A.
- `TrackSizing` enum baseline P224.A incl. `Auto` variant.
- `measure_content` helper baseline P222 + P83 (constrained
  measure).
- `place_cells` baseline P224.C.
- `layout_grid` baseline P224 + P227 + P228 + P230.
- **DEBT-34d** baseline P224.div-1 (audit C1 obrigatório).
- Pattern "aplicação automática ADR-0080 EM VIGOR" N=3
  baseline P230+P231+P232.
- ADR-0079 PROPOSTO Categoria B 0/3 baseline P232.

---

## §2 Cláusulas (10 — atomização paridade P232)

### C1 — Auditoria pré-P233: ler DEBT-34d + algoritmo actual + measure_content (CRÍTICO)

**Audit obrigatório**:

```
grep -A 30 "DEBT-34d" 00_nucleo/DEBT.md
grep -B 2 -A 50 "TrackSizing::Auto\|TrackSizing::Fr" 01_core/src/rules/layout/grid.rs
grep -B 2 -A 30 "fn place_cells\|fn layout_grid" 01_core/src/rules/layout/grid.rs
grep -n "measure_content" 01_core/src/rules/layout/
grep -B 2 -A 10 "P224.div-1" 00_nucleo/adr/typst-adr-0079-*.md 00_nucleo/diagnosticos/
```

**Hipótese**:
- DEBT-34d documenta auto sizing measure-based ausente.
- `TrackSizing::Auto` variant baseline P224.A; implementação
  actual atribui tamanho heuristic (fixo ou remaining/n).
- `measure_content` baseline P222 + P83 disponível como
  `pub(super)` ou `pub(crate)`.
- P224.div-1 registado em ADR-0079 ou diagnóstico.

**Decisões críticas C1**:
1. **Escopo P233 estrito**: cobrir EXACTAMENTE o que
   DEBT-34d documenta sem expansão. Se DEBT-34d for amplo
   (e.g., auto + fr + min/max), **atomização ADR-0036**:
   P233 primeiro sub-item; restantes DEBT-34d-rest
   preservado.
2. **Localização algoritmo**: confirmar
   `layout_grid::layout_grid` é o lugar correcto (vs
   `grid_placement.rs::place_cells`). Hipótese provável:
   pre-pass em `layout_grid` antes do call `place_cells`.
3. **`measure_content` visibilidade**: confirmar acessível
   de `grid.rs`. Se não, audit decide refactor visibility
   ou path completo.
4. **P224.div-1 localização**: identificar ficheiro exacto
   para anotação RESOLVIDA P233.

Se hipótese diverge significativamente: registar
`P233.div-N`. Pattern emergente "audit C1 atomização
condicional pós-DEBT preservado" N=1 inaugurado P233 se
DEBT-34d amplo.

### C2 — Implementar pre-pass measure em `layout_grid`

Editar `01_core/src/rules/layout/grid.rs::layout_grid`:

```rust
pub(super) fn layout_grid(
    &mut self, columns, rows, cells,
    gutter, align, inset, header, footer,
    stroke, fill,  // P227+P228
) -> SourceResult<()> {
    // P233 — Pre-pass measure: calcular tamanho real per
    // track auto.
    let mut col_sizes: Vec<f64> = columns.iter().map(|t| match t {
        TrackSizing::Length(l) => l.to_pt(),
        TrackSizing::Auto => 0.0,    // P233 — placeholder; calculado pre-pass.
        TrackSizing::Fr(_) => 0.0,    // Calculado pós distribuição.
    }).collect();

    // P233 — Pre-pass: medir cells em tracks Auto.
    for (cell_idx, cell) in cells.iter().enumerate() {
        let col = cell_idx % columns.len();
        if matches!(columns[col], TrackSizing::Auto) {
            let measured = self.measure_content(cell, /* constraints */)?;
            col_sizes[col] = col_sizes[col].max(measured.width);
        }
    }

    // P233 — Distribuir remaining space para fr tracks (se
    // aplicável; condicional a audit C1 escopo).
    let total_fr: f64 = columns.iter().filter_map(|t| match t {
        TrackSizing::Fr(w) => Some(w.0),
        _ => None,
    }).sum();
    let remaining = self.region_width - col_sizes.iter().sum::<f64>();
    if total_fr > 0.0 && remaining > 0.0 {
        for (i, t) in columns.iter().enumerate() {
            if let TrackSizing::Fr(w) = t {
                col_sizes[i] = remaining * w.0 / total_fr;
            }
        }
    }

    // Final pass: placement com tamanhos definitivos.
    // [...existing P224+P227+P228+P230 logic com col_sizes correctos...]
}
```

**Audit C1 confirma**: signatures exactos + visibilidade
`measure_content` cross-módulo.

Magnitude C2: **M (~1-1.5h)** — algoritmo two-pass + adapt
lógica placement.

### C3 — Adaptar `place_cells` (se necessário)

Audit C1 determina:
- Se `place_cells` recebe `col_sizes` calculados → adapt
  signature para aceitar slice tamanhos vs `Vec<TrackSizing>`.
- Se `place_cells` itera sobre `TrackSizing` directly →
  adapt para usar tamanhos pre-calculados.

Magnitude C3: **S (~30min)** — refactor signature ou
adaptação.

### C4 — Refino rows (paridade columns)

Audit C1 determina:
- Se algoritmo auto cobre **colunas E linhas**, P233
  implementa ambos.
- Se DEBT-34d cobre apenas colunas, linhas DEBT-34d-rest.

**Decisão fixada — paridade rows se DEBT-34d cobre**:
mesmo algoritmo para rows.

Magnitude C4: **S (~30min)** se applicable; **XS (~5min
audit)** se não applicable.

### C5 — Sentinelas P233

Tests P233 (~10-12 tests):

**Unit grid algorítmico** (~4 tests):
- `p233_auto_track_sizing_single_col_calcula_max`.
- `p233_auto_track_sizing_multi_col_independente`.
- `p233_auto_track_sizing_cell_vazia_zero`.
- `p233_auto_track_sizing_mixed_auto_fr_coexiste` (se fr
  em escopo).

**Layout E2E auto sizing** (~6-8 tests):
- `p233_grid_columns_auto_renderiza_max_cell_width`.
- `p233_grid_columns_auto_auto_independente`.
- `p233_grid_columns_1fr_auto_mix_correcto` (se fr em
  escopo).
- `p233_grid_rows_auto_paridade_columns` (se rows em
  escopo).
- `p233_grid_colspan_auto_distribuido` (se colspan em
  escopo; audit C1).
- `p233_grid_baseline_fixed_preserved` — regressão
  P224 fixed columns preservado.
- `p233_grid_baseline_fr_preserved` — regressão P224 fr
  preservado (se aplicável).

Total tests P233: **~10-12 tests**. Esperado pós-P233:
**2106 + 12 = ~2118 verdes** (paridade hipótese; ajuste
pós-implementação).

### C6 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação
automática N=4)

**Decisão fixada — aplicação automática**: quarta
aplicação automática pós-promoção P229. Pattern N=3 → 4
cumulativo.

L0 prompts NÃO tocados.

### C7 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2106 verdes pré-P233 + ~10-12 novos = **~2118 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~3-5 ficheiros L1 (`grid.rs`,
  possível `mod.rs`, `grid_placement.rs`).
- L0 prompts não tocados — "Nothing to fix".

**Risco regressão crítico**: P224 + P227 + P228 + P230
tests Grid baseline. Hipótese N=0-3 adaptações (algoritmo
correcto **não deve** quebrar tests existentes que usam
tamanhos fixed; pode ajustar tests com `Auto` que
assumiam comportamento heuristic actual).

### C8 — Fecho DEBT-34d formal

Editar `00_nucleo/DEBT.md`:

```markdown
### DEBT-34d — Auto track sizing fix
**Status**: ~~ABERTO~~ → **FECHADO P233 (2026-05-13)**.
**Aberto**: P224 (`P224.div-1` divergência consciente).
**Fechado**: P233 — algoritmo two-pass measure→place
implementado em `layout_grid`.

[...conteúdo original preservado...]

## Fecho P233 (2026-05-13)
- Algoritmo "auto" track sizing implementado correctamente
  (measure-based pre-pass; max per track; distribuição
  remaining para fr).
- Two-pass pattern inaugurado cristalino pós-M9c.
- `measure_content` reuso baseline P222.
- ~10-12 tests novos verdes.
- P224.div-1 RESOLVIDA paralelamente.
- Saldo DEBTs: 12 → 11.
```

Referência cruzada bidirecional: P233 relatório §"Fecho
DEBT-34d" + DEBT.md §"Fecho P233".

### C9 — Inventário 148 footnote ⁵² + ADR-0079 Categoria B 1/3 + P224.div-1 RESOLVIDA

**Inventário 148** (`typst-cobertura-vanilla-vs-cristalino.md`):
- §A.5 Layout entrada `grid(...)`: footnote `⁵¹` → `⁵¹ ⁵²`.
- Footnote ⁵² adicionada (~80 linhas) documentando B.1
  materializado + 8 decisões + audit C1 escopo + fecho
  DEBT-34d + patterns emergentes (algoritmo two-pass N=1;
  fecho DEBT consciente N=1; fecho retrospectivo divergência
  factual N=1; aplicação automática ADR-0080 EM VIGOR N=4).

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco
  `### P233 anotação — Categoria B sub-passo 1 (DEBT-34d
  Auto track sizing fix); Categoria B 1/3`.
- Categoria B: 1/3 materializados (B.1 ✓; B.2 + B.3
  pendentes).
- Status ADR-0079 mantido PROPOSTO (6/13-15 sub-passos).

**P224.div-1 RESOLVIDA**: localizar registo original
(diagnóstico ou ADR-0079) e adicionar marker `[RESOLVIDA
P233]` (paridade pattern P215.div-1 que foi resolved P217
ou similar; audit precedente).

### C10 — Critério aceitação P233

- ~10-12 tests novos verdes.
- 2106 tests pre-existentes preservados (após N=0-3
  adaptações intencionais).
- 0 violations.
- Zero fields novos em Content variants.
- Zero novas stdlib funcs.
- Algoritmo two-pass measure→place funcional.
- **DEBT-34d FECHADO** com referência cruzada bidirecional.
- **P224.div-1 anotada RESOLVIDA P233**.
- Saldo DEBTs: 12 → **11**.
- ADR-0079 Categoria B 1/3 anotado.
- ADR-0080 EM VIGOR aplicação automática N=3 → 4.
- Cobertura Layout 89% preservada (refino qualitativo).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-233-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Auditoria pré-P233 + ler DEBT-34d + algoritmo actual
  (C1).
- §3 Implementação pre-pass measure em `layout_grid`
  (C2+C3).
- §4 Distribuição remaining space fr tracks (C4 se
  applicable).
- §5 Decisões substantivas (8 decisões fixadas) + quarta
  aplicação automática ADR-0080 EM VIGOR.
- §6 Resultados verificação + tests (C5+C7).
- §7 **Fecho DEBT-34d formal** + P224.div-1 RESOLVIDA +
  inventário 148 footnote ⁵² + ADR-0079 anotação Categoria
  B 1/3 (C8+C9).
- §8 Próximo sub-passo (P234 candidatos: B.2 consumer
  geometric; B.3 per-cell algorítmico; D.1 state; pivot).

Código alterado:
- **Editado**: `01_core/src/rules/layout/grid.rs`
  (`layout_grid` pre-pass measure + distribuição fr +
  adapt placement).
- **Possivelmente editado**: `01_core/src/rules/layout/grid_placement.rs`
  (signature `place_cells` adapt para tamanhos
  pre-calculados se necessário).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~10-12
  tests novos).
- **Editado**: `00_nucleo/DEBT.md` (DEBT-34d FECHADO P233
  bloco + referência cruzada).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵² P233 + P224.div-1 marker RESOLVIDA P233
  se ali registado).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria B 1/3 P233; P224.div-1 marker
  RESOLVIDA se ali registado).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Implementar `TrackSizing::MinContent` ou `MaxContent`
  vanilla — fora escopo DEBT-34d (audit C1 confirma;
  refino futuro candidato).
- Iterative refinement com min/max constraints — overkill
  para subset paridade vanilla simples.
- Show rules `#show grid: ...` — fora escopo Fase 5.
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categorias A + B + C + D completas (B.1 ✓; B.2 + B.3
  pendentes).
- Tocar em L0 prompts — ADR-0080 EM VIGOR aplicação
  automática N=4.
- Reabrir decisões arquiteturais — B.1 é Categoria B
  algorítmico isolado (sem reabrir P156G+H, P224, etc.).
- Refactor `TrackSizing` enum — preservado P224.A
  baseline literal.
- Criar novo módulo `track_sizing.rs` — anti-inflação per
  Decisão 3 Opção β consolidar `layout_grid`.
- Promoção pattern emergente "algoritmo two-pass" a ADR
  meta — N=1 insuficiente; limiar N=3-4 não atingido.
- Promoção pattern "fecho DEBT consciente" a ADR meta —
  N=1 insuficiente.
- Expandir DEBT-34d escopo para DEBT-34d-rest (atomização
  ADR-0036 sujeita a audit C1).

---

## §5 Riscos a evitar

1. **Audit C1 revela DEBT-34d amplo** — sub-itens múltiplos
   (e.g., auto + fr + min/max). Mitigação: **atomização
   ADR-0036**; P233 primeiro sub-item; restantes
   DEBT-34d-rest preservado conscientemente.
2. **`measure_content` visibility cross-módulo
   insuficiente**: audit C1 confirma. Mitigação: paridade
   pattern P222 promoção visibility (`pub(super)` →
   `pub(crate)` se necessário).
3. **Algoritmo two-pass quebra tests P224 baseline**:
   tests P224 usaram `TrackSizing::Auto` com heuristic
   actual. Mitigação: N=0-3 adaptações intencionais
   documentadas (tests com Auto agora produzem tamanhos
   measure-correct).
4. **Pre-pass measure quebra placement P224.C
   algorítmico**: P224.C `place_cells` assumia tamanhos
   conhecidos. P233 adicionar pre-pass não muda assumption.
   Mitigação: tests E2E P224 preservados.
5. **fr distribution edge cases**: `remaining < 0`
   (Auto excede região) ou `total_fr = 0` ou `cell vazia`.
   Mitigação: testes específicos C5 (cell vazia zero;
   sem fr não distribui).
6. **Iteração order pre-pass**: ordem de cells importa
   para max? Não — max é comutativo. Mitigação: pattern
   max comutativo preservado.
7. **DEBT-34d localização**: ficheiro `00_nucleo/DEBT.md`
   provavelmente; audit C1 confirma path exacto.
8. **P224.div-1 localização ambígua**: pode estar em
   ADR-0079 ou diagnóstico ou ambos. Mitigação: audit
   C1 grep + anotação em todos lugares onde aparece.
9. **L0 tocado por engano**: tentação por "algoritmo
   importante; documentar formal". Rejeitada — ADR-0080
   EM VIGOR aplicação automática; algoritmo interno em
   `layout_grid` L1 não cria nova entidade L0.
10. **Magnitude exceder M (~2-3h)**: audit C1 amplo pode
    atrasar. Mitigação: time-box audit C1 ~45min; se
    DEBT-34d amplo, atomizar P233 a primeiro sub-item
    imediatamente.
11. **Pattern "fecho DEBT consciente" promoção prematura**:
    N=1 não atinge limiar. Mitigação: documentar pattern
    sem promoção formal.
12. **`P224.div-1` resolved sem audit retrospectivo
    completo**: tentação por "deletar registo P224.div-1"
    como se nunca existisse. Rejeitada — pattern
    histórico preservado: registo P224.div-1 marker
    `[RESOLVIDA P233]` paridade ADR §"Promoção"
    `[HISTÓRICO P226]` pattern.

---

## §6 Hipótese provável

C1 (audit obrigatório) confirmará DEBT-34d documenta
auto track sizing measure-based ausente; `TrackSizing::Auto`
variant baseline P224.A; `measure_content` baseline P222
disponível cross-módulo; P224.div-1 registado em ADR-0079
ou diagnóstico (provável ambos).

C2 implementará pre-pass measure em `layout_grid` (Opção
α two-pass standard).

C3 adaptará `place_cells` signature se necessário
(passing pre-calculated `col_sizes`).

C4 estenderá a rows se DEBT-34d cobre (provável; paridade
columns natural).

C5 criará ~10-12 tests novos.

C6 NÃO tocará L0 (aplicação automática ADR-0080 EM VIGOR
N=3 → 4).

C7 reportará ~2118 verdes; 0 violations; possíveis N=0-3
adaptações.

C8 fechará DEBT-34d formal.

C9 reclassificará footnote ⁵² + ADR-0079 Categoria B 1/3
+ P224.div-1 RESOLVIDA P233.

C10 verifica critério aceitação.

Custo real: **M (~2-2.5h)** — audit C1 ~30-45min +
implementação ~1.5h + tests ~30min.

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.
Pattern emergente "audit C1 atomização condicional pós-DEBT
preservado" pode ser inaugurado se DEBT-34d amplo.

---

## §7 Particularidade P233

P233 é estruturalmente distinto na trajectória pós-M9c:

- **Sexto sub-passo materialização Fase 5 Layout
  candidata** — primeiro Categoria B algorítmico pós-fecho
  Categoria A (P232).
- **Primeiro fecho de DEBT preservado conscientemente
  pós-M9c** — DEBT-34d preservado P224.div-1 há 18
  sub-passos. Pattern emergente "fecho de DEBT preservado
  conscientemente em sub-passo posterior" N=1 inaugurado
  P233.
- **Primeiro algoritmo two-pass measure→place cristalino
  pós-M9c** — distinto cumulativo de A.1-A.5 que foram
  aditivos (fields/lógica precedência). Pattern emergente
  "algoritmo two-pass measure→place" N=1 inaugurado P233.
- **Primeira resolução retrospectiva de divergência
  factual material `Pxxx.div-N`** — P224.div-1 RESOLVIDA
  P233. Pattern emergente "fecho retrospectivo de
  divergência factual em sub-passo posterior" N=1
  inaugurado P233.
- **Quarta aplicação automática ADR-0080 EM VIGOR
  pós-promoção P229** — pattern N=3 → 4 cumulativo
  (P230+P231+P232+P233). Pattern muito sólido empíricamente.
- **Distinto cumulativo de A.1-A.5 estructuralmente** —
  primeiro sub-passo algorítmico Fase 5; A.1-A.5 foram
  todos cosméticos. Padrão "categoria muda → tipo
  trabalho muda" naturalmente reflectido.
- **Cobertura Layout per metodologia preservada 89% real**
  — B.1 é refino algorítmico de funcionalidade existente
  (auto sizing); não adiciona feature mas melhora
  qualidade.
- **Anti-inflação 25ª aplicação cumulativa** pós-P205D —
  Opção β consolidar `layout_grid` sem novo módulo +
  Opção α algoritmo standard + Opção γ L0 automático +
  sem helper novo + sem promoção patterns emergentes +
  sem reabrir decisões + ADR-0079 sem promoção.

Por isso §5 risco 1 (DEBT-34d amplo) é o mais provável.
Mitigação fixada: **atomização ADR-0036 imediata** se
audit C1 revelar DEBT-34d com sub-itens múltiplos. Pattern
"atomização ADR-0036 pós-audit C1 condicional" reusável
para B.2, B.3, C.1, C.2, D.1+ se DEBTs/scope-outs amplos.

**Critério de aceitação P233**:
- ~10-12 tests novos verdes.
- 2106 tests pre-existentes preservados (após N=0-3
  adaptações).
- 0 violations.
- Zero fields novos em Content variants.
- Zero novas stdlib funcs.
- Algoritmo two-pass measure→place funcional.
- **DEBT-34d FECHADO** com referência cruzada bidirecional.
- **P224.div-1 RESOLVIDA P233**.
- Saldo DEBTs: 12 → **11**.
- ADR-0079 Categoria B 1/3 anotado.
- ADR-0080 EM VIGOR aplicação automática N=3 → 4.
- Cobertura Layout 89% preservada.

**Estado pós-P233 esperado**:
- Tests workspace: 2106 → **~2118 verdes** (+10-12).
- Stdlib funcs: 60 preservado.
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Grid/Table/Cell/Block/Boxed fields preservados.
- Place fields: 7 preservado.
- Layouter fields: preservados (n+1 pós-P232).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada (refino
  qualitativo).
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO (6/13-15; Categoria A
  5/5 ✓ fechada; Categoria B 1/3); ADR-0080 EM VIGOR.
- **Saldo DEBTs: 12 → 11** (-1 DEBT-34d).
- **P224.div-1 RESOLVIDA P233**.
- **25 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=3 → 4 cumulativo** (P230+P231+P232+P233).
- **Pattern "algoritmo two-pass measure→place" N=1
  inaugurado P233**.
- **Pattern "fecho de DEBT preservado conscientemente
  em sub-passo posterior" N=1 inaugurado P233**.
- **Pattern "fecho retrospectivo de divergência factual
  em sub-passo posterior" N=1 inaugurado P233**.
- **Pattern "atomização ADR-0036 pós-audit C1 condicional"
  N=1 candidato (se DEBT-34d amplo)**.
- **Categoria B Fase 5 Layout: 1/3 → próximos B.2
  consumer geometric, B.3 per-cell algorítmico**.
- **Fase 5 Layout candidata: 6/13-15 sub-passos
  materializados** (~40-46% cumulativo; Categoria A 100%
  interna; Categoria B 33% interna).
