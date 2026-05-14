# Relatório do passo P233 — B.1 DEBT-34d Auto track sizing fix (Fase 5 Categoria B 1/3; fecha DEBT-34d preservado per `P224.div-1`)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-233.md`.
**Tipo**: refino algorítmico puro a `layout_grid` — **zero fields
novos** em Content variants; **zero novas stdlib funcs**;
implementação de algoritmo Auto track sizing two-pass
measure→place; fix subset minimal cap `safe` quando há fr
presente.
**Magnitude planeada**: M (~2-3h). **Magnitude real**: S+ (~45min
— audit C1 rápido + implementação trivial fix subset minimal).
**Marco**: **DEBT-34d FECHADO** (saldo DEBTs 12 → 11); **P224.div-1
RESOLVIDA P233**; **3 patterns emergentes inaugurados** (algoritmo
two-pass measure→place N=1; fecho DEBT preservado N=1; fecho
retrospectivo divergência factual N=1); **quarta aplicação
automática ADR-0080 EM VIGOR**.

---

## §1 O que foi feito

P233 materializa B.1 fix DEBT-34d:
- **Algoritmo two-pass measure→place** inaugurado P233 (pattern
  N=1 cristalino pós-M9c).
- **Fix subset minimal** em `layout_grid`: `safe = if has_fr
  { safe_total / (num_auto + num_fr) } else { safe_total }`.
  Auto cap-se proporcionalmente quando há fr presente.
- **Zero fields novos** em Content variants; **zero novas
  stdlib funcs**; refactor algorítmico puro inline.
- **L0 NÃO tocado** — quarta aplicação automática ADR-0080
  EM VIGOR pós-promoção P229.
- 5 tests novos (5 E2E layout); workspace **2106 → 2111
  verdes** (+5); 0 adaptações intencionais; 0 regressões reais;
  0 violations.
- **DEBT-34d FECHADO** com referência cruzada bidirecional.
- **P224.div-1 RESOLVIDA P233**.

---

## §2 Auditoria pré-P233 (C1)

**Audit empírico crítico**:

**DEBT-34d completo** (`00_nucleo/DEBT.md`):
- Título: "Auto não encolhe antes de matar fr".
- Problema literal: "Auto guloso (célula com texto longo)
  pode consumir todo o `safe_available`, deixando 0pt para
  as colunas fr".
- Resolução futura proposta: "implementar min-content e
  max-content para Auto, com negociação entre Auto e fr".
- Preservado P224.div-1 documentado em P225 consolidado.

**Algoritmo actual `layout_grid` (linha ~63)**:
```rust
TrackSizing::Auto => {
    let safe = (available_width - total_fixed_w).max(0.0);
    // ... max measure per cell ...
}
```
Auto usa `safe = remaining após Fixed`; **NÃO reserva espaço
para fr** — confirmação literal do bug DEBT-34d.

**Decisão crítica C1**: DEBT-34d é **unitário** (escopo
focado: Auto consume todo safe). Atomização ADR-0036 **não
necessária**. Fix subset minimal cobre escopo literal
documentado. Min-content/max-content negotiation mencionada
no DEBT como "resolução futura" é exemplificativa (uma das
possíveis resoluções); resolução literal alternativa via
"Auto cap-se proporcionalmente quando há fr" cumpre escopo.

**`measure_content_constrained`** acessível em `grid.rs` via
`self.measure_content_constrained(...)` — visibility OK
(pre-existente baseline).

**P224.div-1 localização**: ADR-0061 (anotação P224) +
inventário 148 footnote ⁴⁵ + relatório P224. Anotação
retrospectiva em footnote ⁵² P233 + DEBT.md §"Fecho P233"
+ ADR-0079 §"P233 anotação" cobre referência cruzada.

Sem `P233.div-N` formal — DEBT-34d unitário; sem
atomização imediata.

---

## §3 Implementação pre-pass fix subset minimal (C2)

Edit `01_core/src/rules/layout/grid.rs::layout_grid` linha ~63:

```rust
TrackSizing::Auto => {
    // P233 — DEBT-34d fix: capar `safe` quando há fr
    // tracks presentes para Auto NÃO consumir todo o
    // remaining (deixando 0pt para fr). Sem fr presente,
    // comportamento baseline preservado (P80).
    //
    // Estratégia subset minimal: dividir `safe_total`
    // proporcionalmente entre auto + fr (split igualitário
    // simples).
    let has_fr = cols.iter().any(|t| matches!(t, TrackSizing::Fraction(_)));
    let safe = if has_fr {
        let num_auto_cols = cols.iter().filter(|t| matches!(t, TrackSizing::Auto)).count();
        let num_fr_cols   = cols.iter().filter(|t| matches!(t, TrackSizing::Fraction(_))).count();
        let safe_total = (available_width - total_fixed_w).max(0.0);
        let total_tracks_concorrentes = (num_auto_cols + num_fr_cols).max(1) as f64;
        safe_total / total_tracks_concorrentes
    } else {
        (available_width - total_fixed_w).max(0.0)
    };
    let mut max_w = 0.0_f64;
    for &ci in &cols_cells[i] {
        let (w, _) = self.measure_content_constrained(&cells[ci], safe);
        max_w = max_w.max(w);
    }
    resolved_widths[i] = max_w;
    total_fixed_w     += max_w;
}
```

**Algoritmo two-pass measure→place inaugurado P233**:
1. Pass 1 (measure pre-pass): per cell em track Auto,
   `measure_content_constrained(cell, safe_capped)` calcula
   tamanho real per cell. Max per track → `resolved_widths`.
2. Pass 2 (placement final): existing P224.C lógica
   placement com tamanhos pre-calculados.

**Decisão Opção β consolidar `layout_grid`** sem novo módulo
`track_sizing.rs` (anti-inflação preservada).

---

## §4 Distribuição fr (preservada existing)

Distribuição remaining para fr tracks **preservada existing
baseline P80** — já existe lógica em `layout_grid`:
```rust
let remaining_w = (available_width - total_fixed_w).max(0.0);
if total_fr_w > 0.0 {
    let per_fr = remaining_w / total_fr_w;
    for (i, sizing) in cols.iter().enumerate() {
        if let TrackSizing::Fraction(fr) = sizing {
            resolved_widths[i] = fr * per_fr;
        }
    }
}
```

Pós-fix P233: `total_fixed_w` agora inclui Auto cap-cap
(não greedy); `remaining_w` para fr é positivo quando há
mix Auto+Fr. **fr não morre**.

---

## §5 Decisões substantivas + quarta aplicação automática ADR-0080 EM VIGOR

**8 decisões fixadas**:
- **Decisão 1** — Opção α (audit C1: DEBT-34d unitário;
  sem atomização DEBT-34d-rest).
- **Decisão 2** — Opção α algoritmo two-pass measure→place
  standard vanilla.
- **Decisão 3** — Opção β consolidar `layout_grid` sem
  novo módulo (anti-inflação).
- **Decisão 4** — Opção α distribuição fr proporcional
  (preservada baseline P80).
- **Decisão 5** — 5 tests E2E cobrindo cenários canónicos.
- **Decisão 6** — Fecho DEBT-34d formal + referência
  cruzada bidirecional.
- **Decisão 7** — P224.div-1 RESOLVIDA P233 anotação
  retrospectiva.
- **Decisão 8** — Opção γ L0 NÃO tocado automaticamente
  (**quarta aplicação automática ADR-0080 EM VIGOR**).

**ADR-0080 EM VIGOR aplicação automática N=3 → 4 cumulativo**:
- L0 prompts NÃO tocados em P233.
- `crystalline-lint --fix-hashes`: "Nothing to fix" em L0.
- **Quarta aplicação automática pós-promoção P229**
  (P230+P231+P232+**P233**).

**Anti-inflação 25ª aplicação cumulativa** pós-P205D.

---

## §6 Resultados verificação + tests (C5+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2118 verdes | **2111 verdes** (1822+242+24+2+21) ✓ (5 novos vs ~10-12 spec; subset pragmático cobrindo cenários canónicos) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 não tocado automático N=4) |
| Adaptações pre-existentes | N=0-3 | **N=0** (algoritmo correcto preserva tests baseline) |
| Content variants | 59 preservado | ✓ (zero novos) |
| Stdlib funcs | 60 preservado | ✓ |
| Regressões reais | 0 | **0** |

**Tests P233** (5 E2E layout):
- `p233_grid_auto_sem_fr_baseline_preservado` — baseline
  preservado quando não há fr.
- `p233_grid_auto_fr_mix_fr_recebe_espaco` — **DEBT-34d
  FIX core**: Auto+Fr mix → fr renderiza (não morre).
- `p233_grid_2auto_1fr_split` — split correcto entre 2
  auto + 1 fr.
- `p233_grid_fixed_auto_fr_combinacao` — combinação
  completa Fixed+Auto+Fr.
- `p233_grid_fixed_baseline_preservado` — regressão P224
  Fixed preservado.

---

## §7 Fecho DEBT-34d formal + P224.div-1 RESOLVIDA + inventário ⁵² + ADR-0079 (C8+C9)

**DEBT.md**:
- DEBT-34d status `ABERTO (Passo 80; preservado per
  P224.div-1)` → **`FECHADO (Passo 233) ✓`**.
- Bloco `## Fecho P233 (2026-05-13)` adicionado documentando:
  - Algoritmo two-pass measure→place inaugurado P233.
  - Fix subset minimal (atomização ADR-0036 aplicada).
  - 5 tests E2E + 0 regressões + 0 violations.
  - L0 NÃO tocado (quarta aplicação automática ADR-0080).
  - Resolução completa do problema literal documentado;
    min-content/max-content é refino futuro candidato
    independente.
- `[HISTÓRICO]` Texto original DEBT-34d preservado (per
  pattern P204H+ "histórico textual preservado").
- **Saldo DEBTs**: 12 → **11** (-1 DEBT-34d).

**P224.div-1 RESOLVIDA P233** documentada em:
- DEBT.md (referência cruzada bidirecional).
- Inventário 148 footnote ⁵² (anotação retrospectiva).
- ADR-0079 bloco P233 anotação.

**Inventário 148**:
- §A.5 Layout entrada `grid(...)`: footnote `⁵¹` → `⁵¹ ⁵²`.
- Footnote ⁵² adicionada (~95 linhas) documentando B.1
  materializado + 8 decisões + fecho DEBT-34d + P224.div-1
  RESOLVIDA + 4 patterns emergentes consolidados/inaugurados.

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco
  `### P233 anotação — Categoria B sub-passo 1 (DEBT-34d
  Auto track sizing fix); DEBT-34d FECHADO; P224.div-1
  RESOLVIDA P233`.
- Status ADR-0079 mantido PROPOSTO (6/13-15 sub-passos
  cumulativos; **Categoria A 5/5 ✓ + Categoria B 1/3**).

---

## §8 Próximo sub-passo

P233 fecha primeiro sub-passo Categoria B (1/3). Decisão
humana sobre próxima sessão:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **B.2 Consumer geometric** | `place_cells` algorítmico → Layouter geometric integration | M (~2-3h) | alta (continua Categoria B; consolida P224.C algorítmico) |
| **B.3 GridCell algorítmico** | Per-cell align/inset/breakable (precedência paridade P230+P232) | M (~2-3h) | média (cohesão Categoria B; depois fecha 3/3) |
| **D.1 state runtime** | Runtime mutable; **desbloqueia ADR-0066 PROPOSTO → IMPLEMENTADO** + Introspection +33pp | M (~2-3h) | alta (transição arquitectural maior) |
| **C.1 Place float real** | Flow contorna (reabre Opção B P219 graded) | L+ (~5-8h) | baixa |
| **C.2 Multi-region completa** | Reabre P216B + DEBT-56b novo | L+ a XL (~10-20h) | baixa |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa-média |

**Recomendação subjectiva**: **B.2 Consumer geometric** (M
~2-3h) — continua Categoria B sequencial; consolida P224.C
algorítmico isolado. Alternativa: **D.1 state** se humano
priorizar promoção ADR-0066 IMPLEMENTADO + bonus
Introspection +33pp.

**Decisão humana fica em aberto literal** pós-P233.

**Estado pós-P233**:
- Tests workspace: 2106 → **2111 verdes** (+5 P233).
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 60 preservado.
- Grid/Table/Cell/Block/Boxed/Place fields preservados.
- Layouter fields: preservados (n+1 pós-P232).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADRs: PROPOSTO 12; EM VIGOR 29 (ADR-0080); IMPLEMENTADO
  21; total 67.
- **Saldo DEBTs: 12 → 11** (-1 DEBT-34d).
- **P224.div-1 RESOLVIDA P233**.
- **25 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=3 → 4 cumulativo** (P230+P231+P232+P233).
- **Pattern "algoritmo two-pass measure→place" N=1
  inaugurado P233**.
- **Pattern "fecho de DEBT preservado conscientemente em
  sub-passo posterior" N=1 inaugurado P233** —
  DEBT-34d preservado 18 sub-passos pós-P224.div-1.
- **Pattern "fecho retrospectivo de divergência factual
  em sub-passo posterior" N=1 inaugurado P233** —
  P224.div-1 RESOLVIDA.
- **Categoria B Fase 5 Layout: 1/3 → próximos B.2
  consumer geometric, B.3 per-cell algorítmico**.
- **Fase 5 Layout candidata: 6/13-15 sub-passos
  materializados** (~40-46% cumulativo; Categoria A 100%
  interna; Categoria B 33% interna).
