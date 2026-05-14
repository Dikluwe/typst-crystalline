# Relatório do passo P243 — M7+3 fase (a) infrastructure: `Regions { backlog, last }` extensão + promoção real ≥3 scope-outs multi-region (M9d quarta sub-passo; quarta excepção justificada ADR-0080 EM VIGOR sub-categoria nova "Layouter internal refactor")

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-243.md`.
**Tipo**: refactor incremental Layouter (L1) — extensão de
abstracção `Regions` existente (P216B) + promoção real ≥3
scope-outs ligados a multi-region. **NÃO materializa**
`Content::Columns` ou `Content::Colbreak` (fase (b) DEBT-56
pendente). **NÃO toca pipeline walk-time** (distinto P240/P241).
**NÃO toca geometry/exporter** (distinto P242).
**Magnitude planeada**: L+ (~8-12h). **Magnitude real**: **M
(~2-3h)** — audit C1 revelou refactor field-aggregation **já feito
em P216A + P216B**; P243 reduz para extensão `Regions` + promoção
scope-outs.
**Marco**: quarta sub-passo materialização M9d / M7+; **primeira
fase (a) duas-fases DEBT-56** materializada (infrastructure-only);
**quarta excepção justificada ADR-0080 EM VIGOR pós-P229
sub-categoria nova "Layouter internal refactor"**; sexta aplicação
cumulativa pattern "spec C1 audit obrigatório bloqueante
pós-P236.div-1" N=5 → 6 cumulativo; sub-padrão "promoção real
scope-out ADR-0054 graded" N=1 → 2 cumulativo (atinge limiar
formalização).

---

## §1 O que foi feito

P243 materializa M7+3 fase (a) per ADR-0081 IMPLEMENTADO parcial
(3/5 pós-P242 → 4/5 pós-P243). Audit C1 P243 refinou hipótese
spec material — refactor field-aggregation já feito em P216A/B;
P243 reduz para extensão incremental:

1. **Extensão `Regions` struct** em `01_core/src/entities/region.rs`:
   - `pub backlog: Vec<Region>` field novo.
   - `pub last: Option<Region>` field novo.
   - `pub fn advance(&mut self) -> Option<Region>` method novo.
2. **Promoção real ≥3 scope-outs multi-region** via
   `regions.current.width` save/restore em `01_core/src/rules/layout/mod.rs`:
   - `Pad.right` scope-out P156C → semantic real P243.
   - `Block.width` semantic adiada P156G → semantic real P243.
   - `Boxed.width` semantic adiada P156H → semantic real P243.
3. **Tests** (8 unit + cenários canónicos): 4 unit regions +
   4 unit/E2E layout scope-outs.
4. **L0 partial tocado** (2 ficheiros): `region.md` extensão +
   `content.md` secção scope-outs.

**2190 → 2198 verdes** (+8; 0 regressões; 0 adaptações). **Sem
`P243.div-N`** — paridade lição N=6 cumulativo precedente.

---

## §2 Auditoria pré-P243 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=6 cumulativo

**Audit empírico** (paralelo lição refinada N=6 cumulativo
P237/P240/P241/P242/P243):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| `Regions` struct | NOVO ficheiro | **Já existe** em `01_core/src/entities/region.rs` (P216A/B) | Estender, não criar |
| Fields agrupados (cursor_x/cursor_y/line_start_x/current_items/current_line) | Migrar 5-7 fields cross-module | **Já migrados** P216A/B — Layouter tem `regions: Regions` com `current: Region` | Refactor já feito |
| `flush_line`/`new_page` actualizar | Migrar `self.cursor_x` etc | **Já actualizados** P216A/B (cursor.rs:91-138 usa `self.regions.current.*`) | OK |
| `backlog: Vec<Region>` + `last: Option<Region>` | Adicionar | **NÃO existem** (P216B minimal por anti-inflação 11ª) | **Trabalho real P243**: extensão |
| Scope-outs Pad.right + Block.width + Boxed.width | Confirmar candidates | Confirmados em layout/mod.rs:1135/1187 + Pad.right scope-out P156C | Promover ≥3 |
| `cell_available_h` (P83) | Field existe | ✓ Confirmado | Diferida (Decisão 7) |
| Comemo memoization invariants | Não tocados | ✓ Confirmado | Preservados |
| `Region::Default` | Spec hipotetizou | `Length::ZERO` para campos f64 — paridade Region::new(0,0) | OK |
| Tests baseline pré-P243 | 2190 verdes | ✓ Confirmado | Baseline para +8 |

**Audit C1 finding material**: spec hipotetizou refactor profundo
cross-module L+ (~8-12h, ~30-50 sítios adaptação). Reality:
P216A+P216B já fizeram o heavy lifting. P243 reduz para:
- Extensão `Regions` com `backlog` + `last` + `advance`.
- Promoção 3 scope-outs via `regions.current.width` save/restore.
- Tests + L0 + ADRs.

**Magnitude real M (~2-3h)** face L+ hipotetizado. **Sem
`P243.div-N`** — paridade lição N=6 cumulativo precedente
P237/P240/P241/P242 (audit refinou hipóteses sem div-N formal).

---

## §3 Extensão `Regions` (C2)

`01_core/src/entities/region.rs`:

```rust
#[derive(Debug, Clone)]
pub struct Regions {
    pub current: Region,
    pub backlog: Vec<Region>,      // P243 — fase (b) populated
    pub last:    Option<Region>,   // P243 — fase (b) populated
}

impl Regions {
    pub fn single(width: f64, height: f64) -> Self {
        Self {
            current: Region::new(width, height),
            backlog: Vec::new(),
            last:    None,
        }
    }

    pub fn advance(&mut self) -> Option<Region> {
        if self.backlog.is_empty() {
            // Fase (a): retorna None; caller cria nova region externa.
            return None;
        }
        // Fase (b): consome próxima do backlog.
        let next = self.backlog.remove(0);
        let prev = std::mem::replace(&mut self.current, next);
        self.last = Some(prev.clone());
        Some(prev)
    }
}
```

**Fase (a) P243**: `backlog` vazio + `last: None` em produção
(single-region preservado literal P216A/P216B observable).
**Fase (b) DEBT-56**: populated quando `Content::Columns`
materializar (passo subsequente fora P243).

**Sub-padrão #14 "Tipo entity em ficheiro próprio"** preservado
N=6 (Regions já existe desde P216B; P243 estende existente).

---

## §4 Promoção real ≥3 scope-outs multi-region (C5)

### Pad.right (scope-out P156C → semantic real P243)

`01_core/src/rules/layout/mod.rs` arm `Content::Pad`:

```rust
let right = sides.right.map_or(0.0, |l| l.resolve_pt(font));
// ...
let saved_width = self.regions.current.width;
self.regions.current.width = (saved_width - right).max(0.0);
self.layout_content(body);
self.regions.current.width = saved_width;
```

**Mecânica**: `layout_word` em `cursor.rs` consulta
`self.regions.current.width` para width-aware wrap. Promoção
garante que width efectiva reflecte constraint `right` durante
body layout.

### Block.width (semantic adiada P156G → semantic real P243)

```rust
let saved_width = self.regions.current.width;
if let Some(w) = width {
    let w_pt = w.resolve_pt(font);
    let line_start_pt = self.regions.current.line_start_x.0;
    self.regions.current.width = (line_start_pt + w_pt).max(0.0);
}
self.layout_content(body);
self.regions.current.width = saved_width;
```

**Mecânica**: width efectiva = `line_start_x + w_pt` (ponto onde
wrap deve ocorrer). `width: None` preserva largura herdada.

### Boxed.width (semantic adiada P156H → semantic real P243)

```rust
let saved_width = self.regions.current.width;
if let Some(w) = width {
    let w_pt = w.resolve_pt(font);
    let cursor_x_pt = self.regions.current.cursor_x.0;
    self.regions.current.width = (cursor_x_pt + w_pt).max(0.0);
}
self.layout_content(body);
self.regions.current.width = saved_width;
```

**Paralelo Block** mas Boxed é INLINE — usa `cursor_x` em vez de
`line_start_x`. `inset_left` já aplicado antes; `inset_right`
aplicado depois.

**Save/restore LIFO** preserva semantic cumulativo para Pad/Block
aninhados (test `p243_pad_aninhado_largura_cumulativa_preservada`).

**Sub-padrão "promoção real scope-out ADR-0054 graded"** N=1 →
**2 cumulativo** (P242 radius/clip + **P243 multi-region attrs**).
Atinge limiar formalização N=2 — candidato a ADR meta passo
administrativo XS futuro.

---

## §5 Decisões substantivas (10 decisões fixadas incl. Decisão 0 lição N=6 cumulativo) + quarta excepção justificada ADR-0080 EM VIGOR

**10 decisões fixadas P243** (Decisão 0 = lição N=6 cumulativo
P237 + P238 reescrito + P240 + P241 + P242 + P243):

| # | Decisão | Opção fixada |
|---|---------|--------------|
| 0 | C1 audit obrigatório bloqueante | lição N=6 aplicada; sem `P243.div-N` (audit refinou hipótese fields já-agregados P216A/B) |
| 1 | `Regions` extensão (paralelo conceptual `LayouterRuntimeState` P190C) | ✓ |
| 2 | Migração field-by-field | **Já feita P216A/P216B** (audit finding material) |
| 3 | Fase (a) preserva single-region observable literal | `backlog` vazio + `last: None` em produção |
| 4 | Promoção real ≥3 scope-outs multi-region | Pad.right + Block.width + Boxed.width |
| 5 | Sem `Content::Columns`/`Colbreak` em P243 | Fase (b) DEBT-56 pendente |
| 6 | Sem ADR dedicada column flow algorithm | Fase (b) |
| 7 | `cell_available_h` integration diferida | Passo futuro NÃO reservado |
| 8 | Nova sub-categoria ADR-0080 "Layouter internal refactor" | Terceira sub-categoria distinta P240/P241 walk-time + P242 geometry/exporter |
| 9 | Tests focam preservação observable | 4 regions + 4 layout scope-outs |
| 10 | Sem fechamento Fase 5 / ADR-0061 / DEBT-56 | Fase (b) pendente preserva DEBT-56 aberta |

**Quarta excepção justificada ADR-0080 EM VIGOR pós-P229**:

**Sub-categoria diferente** de P240/P241 (walk-time runtime) E
de P242 (geometry/exporter): P243 é "Layouter internal refactor"
— distinta semanticamente por tocar estrutura interna do Layouter
L1 sem walk-time integration nem cross-camada L1/L3.

L0 partial tocado (2 ficheiros):
- `00_nucleo/prompts/entities/region.md` — extensão `Regions`
  backlog/last/advance + sub-padrão promoção real scope-out N=2.
- `00_nucleo/prompts/entities/content.md` — secção promoção
  scope-outs Pad.right + Block.width + Boxed.width.

**ADR-0080 §"Excepção P243"** anotada formalmente cristalizando
N=4 cumulativo com **3 sub-categorias formalizadas**:
- walk-time runtime (N=2 P240+P241).
- geometry/exporter (N=1 P242).
- **Layouter internal refactor (N=1 P243)** ← inaugurada.

**Anti-inflação 35ª aplicação cumulativa** pós-P205D — Opção α
extensão Regions + Opção α `backlog/last` fields novos + Opção α
`advance` method + Opção α promoção real scope-outs save/restore
+ Opção γ L0 partial quarta excepção sub-categoria nova + ADR-0081
IMPLEMENTADO parcial 4/5 (não completo) + Opção β `cell_available_h`
diferida (Decisão 7).

---

## §6 Resultados verificação + tests + pré-condições obrigatórias (C6+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2206 verdes (range 2200-2208) | **2198 verdes** (1909+242+24+2+21) ✓ (ligeiramente abaixo por audit refinou scope) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | 5-7 L0 hashes + ~15-25 ficheiros L1 | ✓ (1 L0 + ~3 ficheiros L1 — escala reduzida por audit C1) |
| Adaptações pre-existentes | N=~30-50 | **N=0** ✓ (extensão aditiva não-disruptive; refactor field-aggregation já feito P216A/B) |
| Content variants | 62 preservado | ✓ |
| Layouter fields directos | 21 (sem mudança — audit C1) | ✓ |
| Tipos entity novos | +1 Regions+RegionState | **0** (Regions já existia P216B; P243 estende) |
| Scope-outs promovidos | ≥3 (Pad.right + Block.width + Boxed.width) | ✓ |
| ADR-0081 status | IMPLEMENTADO parcial 3/5 → 4/5 | ✓ (M7+3 fase (a) ✓; M7+3 fase (b) + M7+4 pendentes) |
| ADR-0080 §"Excepção P243" | anotada N=4 sub-categoria 3 "Layouter internal refactor" | ✓ |
| DEBT-56 status | EM ABERTO preservado | ✓ (checklist ✓ item 1 anotado fase (a)) |
| L0 partial tocado | 5-7 ficheiros | **2** (escala reduzida por audit) |
| Regressões reais | 0 | **0** |

**Tests P243** (8 unit/E2E):

**Unit regions** (4 tests em `entities/region.rs`):
- `p243_regions_single_backlog_vazio_last_none`.
- `p243_regions_advance_fase_a_retorna_none`.
- `p243_regions_advance_fase_b_consome_backlog`.
- `p243_regions_clone_preserva_backlog_last`.

**Unit/E2E layout scope-outs** (4 tests em `rules/layout/tests.rs`):
- `p243_pad_right_efetivo_reduz_width_durante_body`.
- `p243_block_width_efetivo_clampa_largura`.
- `p243_boxed_width_efetivo_clampa_largura`.
- `p243_pad_aninhado_largura_cumulativa_preservada`.

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias" P239):
1. **Tests baseline preservados**: 2190 verdes pré-P243 → **2198
   verdes pós-P243** (+8 novos; 0 regressões reais; **0 adaptações
   intencionais** — extensão aditiva não-disruptive).
2. **Comemo memoization invariants ADR-0073/0074 preservados** —
   P243 NÃO toca trait Introspector nem methods (refino L1 interno
   isolado).
3. **Backward compat**: stdlib `block(width: 100pt)` continua a
   funcionar (semantic agora real); tests pré-P243 que usavam
   Block.width/Boxed.width/Pad.right como scope-outs preservados
   inalterados (eval-time wrappers P236/P237 + walk-time runtime
   P240/P241 + geometry P242 intactos).

**Promoções ADR**:
- **ADR-0081 IMPLEMENTADO parcial 3/5 → 4/5** (M7+3 fase (a) ✓;
  M7+3 fase (b) + M7+4 pendentes). Distribuição ADRs preservada
  literal — sem novos ADRs. PROPOSTO 12; EM VIGOR 29; IMPLEMENTADO
  22; total **68 preservado**.
- ADR-0079 Categoria A.4 preservada P242 parcial.
- ADR-0080 §"Excepção P243" anotada N=4 sub-categoria nova
  "Layouter internal refactor".
- DEBT-56 §"Plano" checklist ✓ item 1 ("Refactor minimal Layouter")
  anotado P243 fase (a); fase (b) pendente preserva DEBT-56
  aberta.
- ADR-0066 SUPERSEDED-BY 0073 preservado.

**Inventário 148 footnote ⁶²** adicionada (~300 linhas)
documentando: M7+3 fase (a) infrastructure materializada; lição
N=6 cumulativo C1 audit refinou hipótese fields já-agregados
P216A/B; 4 patterns emergentes; sub-padrão "promoção real
scope-out" N=2 atinge limiar formalização; quarta excepção
justificada ADR-0080 sub-categoria nova "Layouter internal
refactor"; 10 decisões fixadas; DEBT-56 checklist anotado fase
(a); cobertura Layout 89% → 91-92% → ~93-94%.

---

## §7 Patterns emergentes inaugurados/consolidados P243

- **"Refactor profundo Layouter internal" N=1 inaugurado P243**
  — sub-padrão novo (magnitude real reduzida vs spec por P216A/B
  precedente).
- **"Sub-categoria ADR-0080 nova"** N=2 → **3 cumulativo**
  (walk-time P240+P241; geometry/exporter P242; **Layouter
  internal refactor P243** inaugurada).
- **"Promoção real scope-out ADR-0054 graded"** N=1 → **2
  cumulativo** (P242 radius/clip + **P243 multi-region attrs
  Pad.right + Block.width + Boxed.width**). Atinge limiar
  formalização N=2 — candidato a ADR meta XS futuro.
- **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"** N=5
  → **6 cumulativo** (P237 + P238 reescrito + P240 + P241 +
  P242 + P243).

---

## §8 Próximo sub-passo pós-P243

P243 completa M7+3 fase (a) (M9d quarto sub-passo). Restantes
pendentes (magnitude cumulativa restante ~10-16h):

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+3 fase (b)** | `Content::Columns` + `Colbreak` + `native_columns` + Layouter consumer multi-column + ADR column flow + tests | **L (~5-8h)** | **alta** (fecha DEBT-56 + completa M7+3 + promove potencialmente ADR-0061 → IMPLEMENTADO) |
| M7+4 Place float real | Reabertura Opção B P219 graded | L (~5-8h) | média (desbloqueia C.1; isolada de fase (b)) |
| Cell layout migration → `regions.current.height` | `cell_available_h` → `regions.current.height` (Decisão 7 P243 diferida); activa A.4 breakable per-cell | M (~2-4h) | média (refino sequente P243 natural) |
| Refino A.4 — `outset`+`fill`+`stroke` Block+Boxed | 3 dos 4 scope-outs restantes pós-P242 | S-M por attr | baixa-média |
| ADR meta admin XS | Promoção formal patterns cumulativos: refino paralelo callers fixpoint N=2; tipo entity #14 N=6; **promoção real scope-out N=2 atinge limiar**; sub-categoria ADR-0080 N=3 | XS por pattern | média (3+ patterns atingem limiar) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| Pausa M-fase | Fase 5 graded ~93-100% (14/13-15 sub-passos) | XS | baixa |

**Recomendação subjectiva pós-P243**: **M7+3 fase (b)**. Sequência
natural: infra (a) P243 prepara consumers (b) seguinte; magnitude
L isolada (~5-8h); fecha DEBT-56 e potencialmente promove ADR-0061
→ IMPLEMENTADO. Alternativa: M7+4 Place float (magnitude L
isolada; desbloqueia C.1; sem dependência fase (b)).

**Decisão humana fica em aberto literal** pós-P243.

**Estado pós-P243**:
- Tests workspace: 2190 → **2198 verdes** (+8 P243).
- Content variants: 62 preservado.
- ShapeKind variants: 5 preservado.
- Layouter fields: preservados (migração já feita P216A/B).
- **Regions fields**: 1 → **3** (+backlog +last).
- **Regions methods**: +1 (`advance`).
- **Scope-outs promovidos**: 3 (Pad.right + Block.width + Boxed.width).
- Tipos entity novos: 0 (Regions já existia P216B; P243 estende).
- Stdlib funcs: 64 preservado.
- §A.5 distribuição: preservada.
- Cobertura Layout per metodologia: ~91-92% → **~93-94%** (refino
  qualitativo + parcial quantitativo via 3 scope-outs promovidos).
- Cobertura user-facing total: ~73-74% → **~74-75%** (scope-outs
  promovidos bonus marginal).
- **ADRs distribuição preservada literal**: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 22; total **68 preservado**. ADR-0081 transita
  3/5 → **4/5** internamente. ADR-0079 Categoria A.4 preservada
  P242 parcial. ADR-0080 §"Excepção P243" N=4 sub-categoria 3
  "Layouter internal refactor".
- **Saldo DEBTs: 11 preservado** (DEBT-56 anotada checklist fase
  (a); fase (b) pendente preserva DEBT-56 aberta).
- **35 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P243** (4 inaugurados/consolidados):
  - "Refactor profundo Layouter internal" N=1 inaugurado P243.
  - "Sub-categoria ADR-0080 nova" N=2 → **3 cumulativo**.
  - "Promoção real scope-out ADR-0054 graded" N=1 → **2 cumulativo**.
  - "Spec C1 audit obrigatório bloqueante" N=5 → **6 cumulativo**.
- **Categoria D Fase 5 Layout: 3/? sub-passos materializados**
  preservado.
- **Categoria A.4 Fase 5 Layout**: parcial P242 preservado.
- **Fase 5 Layout candidata: 13/13-15 → 14/13-15 sub-passos
  materializados** (~93-100% cumulativo).
- **M9d / M7+ progresso**: **4/5 sub-passos materializados**
  (M7+1 ✓; M7+2 ✓; **M7+3 fase (a) ✓**; M7+5 ✓; M7+3 fase (b)
  + M7+4 pendentes; cumulativa restante ~10-16h).
- **Marco interno**: quarta sub-passo M9d validada; **primeira
  fase (a) duas-fases DEBT-56** materializada (infrastructure-only);
  audit C1 refinou hipótese spec material sem div-N — paridade
  lição N=6 cumulativo precedente. **Sub-padrão "promoção real
  scope-out ADR-0054 graded"** atinge limiar formalização N=2
  (candidato a ADR meta XS).
