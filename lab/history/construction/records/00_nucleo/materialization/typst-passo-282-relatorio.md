# Passo 282 — Relatório consolidado

**Tema**: Auditoria dupla zero-código — (A1) paridade emit local
vs top-level pós-P281; (A2) estado percentual do projecto. Sub-passo
§C.1 condicional **não disparou** (paridade total confirmada).

**Data**: 2026-05-18
**Branch**: Tekt
**Magnitude**: passo administrativo zero-código.

---

## §1 — Achados auditoria paridade emit (Fase A1)

`diagnosticos/diagnostico-paridade-emit-passo-282.md`.

### §1.1 — Mecanismo arquitectural P281 garante paridade

P281 centralizou Text/Glyph emit em **helpers únicos** (`emit_text_pdf`
+ `emit_glyph_pdf`) invocados tanto pelo `build_page_stream`
(top-level) como pelo `draw_item_local` (local em Group). Diferença
única: o parâmetro `base_y` (top-level: `page_height - pos.y.val()`
Y-inversion explícita; local: `pos.y.0` directo após Group `cm`).

**Win arquitectural**: divergência bit-exact entre top-level e local
é **estructuralmente impossível por construção** — qualquer mudança
futura ao emit afecta automaticamente ambos.

### §1.2 — Tabela de paridade (todas as variantes)

| Variante | Scenario | Paridade bit-exact? | Mecanismo |
|---|---|---|---|
| Text | Type1 | ✓ total | helper único `emit_text_pdf` (faux-bold, tracking, F2/F3 inherited) |
| Text | CIDFont | ✓ total | helper único (hex Identity-H + /F1) |
| Text | Multifont | ✓ total | helper único (`/F{fi+1}` dispatch) |
| Glyph | Type1 | ✓ total | ambos silently ignored |
| Glyph | CIDFont | ✓ total | helper único (`<{:04X}> Tj` + /F1) |
| Glyph | Multifont | ✓ total | helper único (`/F1` hardcoded — decisão pré-P281 preservada) |
| Line | (scenario-indep) | ✓ estructural | mesmo formato `q w m l S Q`; `RG` ausente simetricamente |

### §1.3 — Suspeitas refutadas

**6/6 suspeitas listadas na spec P282 §A1**:

1. ~~Faux-bold local simplificado~~ → refutado (helper único).
2. ~~Tracking local em falta~~ → refutado.
3. ~~/F2/F3 hardcoded para /F1 em local~~ → refutado.
4. ~~Glyph multifont local divergente~~ → refutado.
5. ~~Line local falta `RG` que top-level tem~~ → refutado
   (**ambos** sem `RG` — limitação simétrica).
6. ~~Multifont Glyph dispatch diferente~~ → refutado.

### §1.4 — §C.1 NÃO disparou — zero fixes aplicados

Auditoria não detectou divergências bit-exact não-legítimas. Code
unchanged.

### §1.5 — Pendência fora-do-escopo identificada

**`P-line-color-rg-emit`** (S — L1 + L3): `FrameItem::Line` não tem
campo `color`; emit em ambos top-level e local não emite `RG`
(stroke colour). Limitação simétrica pré-P281 preserved. Resolução
requer:

1. Adicionar `color: Option<Color>` a `FrameItem::Line` em L1
   (`01_core/src/entities/layout_types.rs`).
2. Layouter (L1) propaga cor de stroke do style ao FrameItem.
3. L3 emit (top-level + local) emite `{r:.3} {g:.3} {b:.3} RG\n`
   quando `Some(color)`.

Magnitude S. Não-bloqueante.

---

## §2 — Estado percentual por categoria (Fase A2)

`diagnosticos/estado-percentual-projecto-passo-282.md`.

### §2.1 — Tabela síntese

| Categoria | Cobertura empírica | Direcção principal |
|---|---|---|
| Model | ~60% | Footnote (M); Outline (M); Title (XS) |
| Layout | ~72% | Multi-region columns (M+; ADR-0078 sub-fase b) |
| Visualize | ~50% | Curve (S-M); Tiling (M); Transparency (M+) |
| Math | ~40% | Accent (XS); Cancel (XS); Op (M); Lr (S); Underover (S) |
| **Text** | **~30%** | **Underline/Overline/Strikethrough (XS each)**; SmartQuote (S) |
| Introspection | ~70% | Outline (M); Footnote integration |
| **Export** | **~85%** | `RG` em Line (S); transparency vectorial (M); font subsetting (M+) |
| Stdlib | ~50% | **Calc trig+hyperbolic+log (XS×~20 fns)** |
| Eval | ~80% | DEBT-2 lazy capture; DEBT-50 Show selector |
| CLI | ~75% | typst watch incremental (M+); package management (M+) |

**Cobertura agregada estimada**: **~63%** (média ponderada).

### §2.2 — Insights chave

- **Categoria mais robusta**: Export (~85% — cluster Gradient + Group
  emit fechado em P281).
- **Categoria mais sub-implementada**: Text (~30% — quick wins XS
  disponíveis).
- **Math é frente concentrada**: 6 features XS-M cada permitiriam
  saltar 40% → 75%.
- **Stdlib calc tem ROI alto**: XS por função × ~20 funções (trig,
  hyperbolic, log, exp, fract) saltam 22% → 70%.

### §2.3 — Inventário factual

- Cristalino: **61 `Content` variants** + **88 `native_*` stdlib fns**.
- Vanilla: **168 `#[elem]` definitions** + **159 `#[func]` definitions**
  + **41 calc fns**.

---

## §3 — Próximos passos sugeridos (top 10)

Ordenados por valor user-facing × ausência de bloqueador × continuidade.

### Rank 1-3: quick wins ROI alto

1. **`P-stdlib-calc-trig`** — Calc trigonometric/hyperbolic/log
   (XS × ~20 fns; salta calc 22% → 70%).
2. **`P-curve-geometry`** — Curve geometry primitive (S-M; ADR-0078
   §sub-fase b; DEBT-pending).
3. **`P-text-deco-emit`** — Underline/Overline/Strikethrough
   (S combinado; Text 30% → 45%).

### Rank 4-6: features médias com valor médio

4. **`P-math-accent-cancel`** — Math Accent + Cancel (XS+S; Math
   40% → 50%).
5. **`P-smartquote`** — SmartQuote typographic (S; Text 30% → 40%).
6. **`P-line-color-rg-emit`** — `RG` em Line (S; Export 85% → 87%;
   resolve pendência P282 §1.5).

### Rank 7-9: clusters M de alto valor user-facing

7. **`P-footnote-cluster`** — Footnote (M; Model 60% → 70%).
8. **`P-outline-cluster`** — Outline (M; Introspection 70% → 80%).
9. **`P-math-op-lr`** — Math Op + Lr (M; Math 40% → 60%).

### Rank 10: bloqueado / M+

10. **`P-columns-multi-region`** — Multi-region column flow Fase 3
    ADR-0078 (M+; Layout 72% → 85%).

---

## §4 — Decisão humana sobre P283+

3 caminhos sugeridos:

### Opção A: **Horizontal** (rampa de cobertura)

Bater quick wins em série: calc trig + text deco + math accent/cancel +
smartquote + RG-em-line. **~5-6 passos XS-S**, ganha ~15% cobertura
agregada. Vantagem: alto ROI por passo; permite consolidar disciplina
operacional pós-P281.

### Opção B: **Vertical** (frente concentrada)

Atacar 1-2 clusters M de alto valor: Curve + Footnote OU Outline. **~3-5
passos M**, ganha ~10% mas desbloqueia features canónicas de documentos
académicos. Vantagem: visibilidade user-facing alta.

### Opção C: **Híbrida** (recomendada)

Iniciar com 1-2 quick wins (calc trig + text deco) — confiança e
momentum — seguidos de Curve (geometry primitive base para vários
ADR-0078 sub-fases) OU Footnote (feature alto valor académico).

Decisão fica para o humano com base em prioridades não-técnicas
(audiência alvo, demonstrabilidade, etc.).

---

## §5 — Sub-padrões emergentes (sem formalização ADR)

### §5.1 — "Auditoria dupla zero-código" — N=1 inaugural

P282 combina **2 auditorias ortogonais** em paralelo:
- Vertical (A1): paridade emit local vs top-level (deep dive 1 área).
- Horizontal (A2): cobertura percentual por categoria
  (breadth scan 10 áreas).

Diferencia de:
- P125, P275 (auditorias administrativas de DEBTs).
- P280 (auditoria sistemática de classe de bug — N=1 dimensão).

Aguardar reaplicação para considerar formalização.

### §5.2 — "Auditoria sistemática de bug latent class" — N=1 → N=2 cumulativo

- N=1: P280 (classe de walkers top-level que não recursam em Group).
- N=2: **P282 §A1** (classe de divergência emit local vs top-level
  pós-refactor estrutural P281).

Ambas confirmam/refutam hipótese empíricamente. **Limiar formalização**
N≥3-4; aguardar reaplicação cross-context.

### §5.3 — "Win arquitectural validado a posteriori"

P281 unificou emit via helpers únicos como **decisão arquitectural
proactiva** para garantir manutenção futura. P282 §A1 confirma
empíricamente que essa decisão **automaticamente eliminou** uma classe
inteira de bugs latents — paridade local vs top-level é estructuralmente
impossível de violar.

Padrão emergente: "single source of truth como invariante anti-bug".
Aguardar reaplicação para formalização.

---

## §6 — Métricas

| Métrica | Valor |
|---------|-------|
| LOC L3 produção | 0 (zero-código) |
| LOC L0 modificado | 0 (sem alteração L0; secção P281 mantida) |
| Hash L0 | preserved (`export.rs → bc7b8b95`) |
| Testes adicionados | 0 |
| Testes pré-existentes | 2 624 preserved bit-exact |
| Lint | zero violations (preserved) |
| Fixes aplicados | 0 (paridade total confirmada) |
| Pendências registadas | 1 (`P-line-color-rg-emit`, S, não-bloqueante) |
| Frentes accionáveis listadas | 10 |

---

## §7 — Conformidade Cristalina

- ✅ **ADR-0029 pureza física L1**: zero alterações L1/L2/L3/L4.
- ✅ **ADR-0085 diagnóstico imutável**: 2 diagnósticos imutáveis
  produzidos (paridade + estado percentual). **36º + 37º consumo**.
- ✅ **ADR-0094 cap LOC Pattern 1**: zero código alterado.
- ✅ **Anti-padrão over-formalização P273.17 §0**: zero ADR nova;
  3 sub-padrões emergentes registados §5 sem formalização.
- ✅ **Honestidade epistémica**: 6/6 suspeitas spec refutadas com
  evidência empírica literal (cross-reference linhas exactas em
  `export.rs`); auditoria não declarativa.
- ✅ **Auditoria primeiro**: §C.1 só disparava se A1 encontrasse
  fixes — auditoria confirmou paridade, §C.1 NÃO disparou.

---

## §8 — Referências cross-passos

- **P273.10 / P279 / P280** — escala cumulativa de auditorias
  empíricas (gradient walker / image walker / walkers top-level
  class).
- **P281** — unificação β-completa que P282 validou empíricamente.
- **P282 §A1** — paridade emit (deep dive 1 área).
- **P282 §A2** — estado percentual (breadth scan 10 áreas).
- **ADR-0044** — `Engine<'a>` agregador L1 (precedente conceptual de
  helpers únicos em L3 via `PageContext`).
- **ADR-0078** — Layout roadmap (referência para próximos passos
  P-curve / P-columns-multi-region).
- **ADR-0085** — Diagnóstico imutável (36º + 37º consumo).
- **ADR-0094** — Meta-operacional Pattern 1 (cap LOC inaplicável a
  passo zero-código).
- **DEBT-55** (Bibliography hayagriva) — bloqueia subset de Model.

---

*P282 confirma empíricamente que P281 garantiu paridade estructural
(zero divergência local vs top-level). Estado do projecto: ~63%
cobertura agregada vs vanilla, com Export como categoria mais robusta
(~85%) e Text como mais sub-implementada (~30%). 10 frentes
accionáveis listadas com magnitude estimada; decisão sobre direcção
P283+ fica para humano com base em prioridades não-técnicas. Cluster
Gradient + Group emit fechado em todos os planos; cluster próximo
permanece em aberto.*
