# Diagnóstico Fase A P278.A — Cleanup XS+S combinado (3 sub-ops)

**Data**: 2026-05-18.
**Passo**: typst-passo-278.A.
**Magnitude**: M documental (~500 linhas; complexidade 3 sub-ops auditadas).
**Cluster**: Cleanup / Cluster Gradient residual / Fix funcional.
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085 — 3 sub-operações independentes verificadas.
**32º consumo directo de fonte** (continuação P277 N=36; 31º consumo).

---

## §A.1 — Sub-op 1: content-md-debt56-update

### Auditoria empírica das referências DEBT-56 em content.md

`grep -n "DEBT-56" 00_nucleo/prompts/entities/content.md` retorna **5
ocorrências**:

| Linha | Texto (linha + contexto) | Tipo | Acção |
|---|---|---|---|
| 283 | `refino quando refactor de Layouter para multi-region acontecer (DEBT-56 + Fase 3 Layout).` | **factual desactualizada** | substituir referência por ADR-0078 §sub-fase (b) |
| 436 | `(pode exceder largura da página). Refino com refactor multi-region (DEBT-56 + Fase 3).` | **factual desactualizada** | substituir referência |
| 686 | `útil por arm; refino com refactor multi-region per DEBT-56).` | **factual desactualizada** | substituir referência |
| 796 | `width: limitar largura útil em contexto inline exigiria refactor multi-region (DEBT-56).` | **factual desactualizada** | substituir referência |
| 824 | `width/height armazenados mas não impõem limite real (refino multi-region per DEBT-56).` | **factual desactualizada** | substituir referência |

**Análise**: todas as 5 referências apontam para "refactor multi-region"
como solução pendente. **DEBT-56 foi encerrado P221** (column flow
Fase 3 Layout materializada via sub-fases P216A+B + P217-P220). A
parte **"refino multi-region completo Opção A"** continua scope-out
per **ADR-0078 §"Decisão" sub-fase (b)** (Fase 4 candidata NÃO-
reservada per política P158; descoberta P273.16 confirmou).

Nenhuma referência é histórica/contextual (todas são "para refino
futuro ver DEBT-56"). 5/5 são **factuais desactualizadas** e devem
ser actualizadas.

**Acção §A.1**: substituir 5 referências para apontar para
`ADR-0078 §sub-fase (b)` em vez de "DEBT-56" fechado.

**Magnitude**: ~5 LOC L0 (substituição inline em cada linha).

---

## §A.2 — Sub-op 2: helper-group-bbox

### Verificação empírica dos sítios replicados

`grep -n "FrameItem::Group { pos, inner_width, inner_height, items"`
retorna **6 sítios** (não 3 como spec):

| Sítio | Linha | Função | Uso do `group_bbox` |
|---|---|---|---|
| 1 | **454** | `scan_all_gradients.walk` arm Group | DedupKey lookup para gradient register |
| 2 | **524** | `pattern_resources_for_page.walk` arm Group | DedupKey lookup para page resources enum |
| 3 | **2236** | `build_page_stream_type1` dispatch Group | Passa override para draw_item_local |
| 4 | **2478** | `draw_item_local` arm Group (P273.13) | Recursão com override para children |
| 5 | **2744** | `build_page_stream_cidfont` dispatch Group | Passa override |
| 6 | **2940** | `build_page_stream_multifont` dispatch Group | Passa override |

**Construção idêntica em todos os 6 sítios**:

```rust
let group_bbox = Rect {
    x: Pt(pos.x.0),
    y: Pt(pos.y.0),
    w: Pt(*inner_width),
    h: Pt(*inner_height),
};
```

(Variantes mínimas: 3 sítios usam `Pt(...)` directamente; 3 sítios
usam `typst_core::entities::layout_types::Pt(...)` qualificado).

### §A.2.1 — Helper proposto

`fn group_bbox_from_frame_item_fields(pos, inner_width, inner_height)
-> Rect` — assinatura simples; sem dependências externas. Net LOC
esperado:

- **Adicionado**: ~12 LOC (definição + comentário doc).
- **Removido**: ~36 LOC (6 sítios × ~6 LOC cada).
- **Net**: **-24 LOC**.

### §A.2.2 — Verificação que não há lógica divergente

Todos os 6 sítios constroem **idênticamente**. Sub-op 2 é refactoring
mecânico puro — zero risco de regressão.

**Magnitude**: net -24 LOC L3.

---

## §A.3 — Sub-op 3: draw-item-local-text-image — reformulação

### Inventário do catch-all `_ => {}`

`fn draw_item_local` (linha 2376) match `item: &FrameItem`:

| FrameItem variant | Arm explícito? | Comportamento |
|---|---|---|
| `Text` | ❌ | Descartado silenciosamente |
| `Line` | ❌ | Descartado silenciosamente |
| `Glyph` | ❌ | Descartado silenciosamente |
| `Image` | ❌ | Descartado silenciosamente |
| `Shape` | ✓ (linha ~2385) | Emit local com `emit_stroke_paint` |
| `Group` | ✓ (linha 2478, P273.13) | Recursão com override |

**Catch-all linha 2490**: `_ => {} // Texto e outros tipos em grupos: adiado para passo futuro.`

### §A.3.1 — Análise empírica do "bug"

**Cenário onde Text/Image em Group ocorre**:
- `Content::Transform` cria `FrameItem::Group` via `helpers.rs
  collect_sub_items` — items podem incluir Text.
- `Content::Block { clip: true }` + `Content::Boxed { clip: true }`
  criam `FrameItem::Group { items: body_items }` — body_items podem
  incluir Text/Image.

**Quando emit chama draw_item_local**: 3 callsites (linhas 2243,
2751, 2937) em `build_page_stream_type1/cidfont/multifont` arms de
Group.

**Comportamento actual**: Text inside Group → silently dropped no
PDF emit. Bug funcional **factualmente reproducible**.

### §A.3.2 — Magnitude REAL para full bug fix

Implementação completa Text+Image em draw_item_local requer **font
scenario context** — 3 variantes:

1. **build_page_stream_type1** usa Helvetica builtin (F1/F2/F3).
2. **build_page_stream_cidfont** usa CIDFont com `char_to_gid` map.
3. **build_page_stream_multifont** usa per-font `char_to_gid` arrays.

Para Text emission em draw_item_local funcionar nas 3 variantes:
- Adicionar parameters `fonts`, `per_font_char_to_gid`,
  `char_to_gid`, `style_context` (não-trivial).
- OU criar 3 variantes de `draw_item_local` (massa de code
  duplication).
- OU passar closure `text_emitter: &dyn Fn(&Text)` (complexidade
  dyn).

Image emission em draw_item_local requer `ptr_to_idx: &HashMap` +
`img_refs: &[ImageRef]` — mais simples (apenas 2 params).

**Estimativa LOC**:
- Image arm only: ~30 LOC L3.
- Text arm (Helvetica path) only: ~40 LOC L3.
- Text arm full (3 variantes): ~100-150 LOC L3 + threading.
- Combinado completo: **~150-200 LOC L3** + parameter cascade.

**Cap L3 hard 150 ameaçado** se sub-op 3 for full bug fix per spec.

### §A.3.3 — Reformulação sub-op 3 — match exaustivo com stubs documentados

Per spec §C.3.3 alternative: "manter `_ => {}` mas adicionar
comentário... documentando a decisão" + opção "match exaustivo".

**Decisão Fase A**: converter `_ => {}` em **match exaustivo com
arms explícitos** para Text/Line/Glyph/Image, cada um com **stub
documentado** que preserva comportamento actual (no-op) mas:
1. Expõe limitação explicitamente no código.
2. Força revisão se novo `FrameItem` variant for adicionado.
3. Cria estrutura clara para futuro fix funcional (basta substituir
   stubs).

Estimativa: ~25 LOC L3 (4 arms × ~5 LOC com comments doc).

**Bug fix funcional pleno fica como pendência específica para passo
dedicado** — `P279.X-bis-text-image-em-group-emit` ou similar.

### §A.3.4 — Magnitude reformulada

- Sub-op 3 (reformulada): ~25 LOC L3 (match exaustivo stubs).
- Cap soft L3 50 (per spec §A.5 sub-op 3) — folga 50%.

### §A.3.5 — Sub-padrão "Match exaustivo sem fall-through em L3"

Precedente: `is_locatable` em L1 introspect. Sub-op 3 é primeira
aplicação em L3 (`draw_item_local`). N=1 cumulativo cross-layer
(L1 inaugural; L3 reaplicação).

---

## §A.4 — Estado workspace baseline

```
cargo test --workspace
# typst-core: 2187 passed (preserved P277 baseline)
# typst-infra: 418 passed
# typst-shell: 24, cli: 21, bins: 2
# Total: 2652 passed (= 2644 P273.17 + 8 P277)

cargo run -p crystalline-lint --quiet
# ✓ No violations found
```

---

## §A.5 — Casos de teste planeados

### Sub-op 1: 0 testes (L0 documental).

### Sub-op 2: 1 teste regressão

`p278_group_bbox_helper_consolidacao` — verificar que pipeline com
gradient inside Group preserved bit-exact após extracção do helper
(bytes PDF idênticos). Test pode usar caso similar aos tests
P273.10/13 existentes.

Tests existentes (P273.10/12/13) já cobrem casos com Group + gradient
+ DedupKey lookup — qualquer regressão na extracção dispararia
falha automaticamente. Sub-op 2 fica protegida sem necessidade de
novos testes específicos (alternativa: adicionar 1 teste unit do
helper isolado).

### Sub-op 3: 0 testes funcionais novos

Reformulada para match-exaustivo com stubs (transparency, não bug
fix funcional). Comportamento actual preserved bit-exact. Testes
existentes verdes preserved.

**Pendência criada**: bug fix funcional Text+Image em Group requer
passo dedicado com font scenario threading; ~5-10 testes funcionais
nesse passo futuro.

### Total testes novos esperados: 0-1

---

## §A.6 — Gates de paragem

Per spec §A.6:

| # | Condição | Estado |
|---|---|---|
| 1 | §A.1 detecta 0 ocorrências factuais | ✓ Não disparou (5 factuais) |
| 2 | §A.1 detecta >10 ocorrências | ✓ Não disparou (5 dentro do esperado) |
| 3 | §A.2 detecta 0 ou 1 sítio replicado | ✓ Não disparou (**6 sítios** — mais que estimado mas dentro do esperado para extracção) |
| 4 | §A.3 não consegue reproduzir bug | ✓ Não disparou (bug factualmente confirmado) |
| 5 | §A.3 catch-all tem lógica intencional | ✓ Não disparou (catch-all é silencioso) |
| 6 | Tests workspace ≠ 2652 baseline | ✓ Não disparou (2652 preserved) |
| 7 | Cap LOC L3 hard 150 ameaçado | ⚠ **Disparou parcial**: sub-op 3 full bug fix excederia cap. **Reformulação aplicada** (match exaustivo stubs vs full Text+Image emission). |
| 8 | Cap doc Fase A hard 600 ameaçado | ✓ Não disparou (este doc ~500 linhas) |

**Gate 7 disparado parcial**: aplicado reformulação per spec §C.3.3
alternative (match exaustivo + stubs documentados). Bug fix
funcional fica como pendência dedicada futura.

**Outros 7 gates: zero disparos**. Passo prossegue.

---

## §A.7 — Critério de aceitação Fase A

- ✓ §A.1 5 referências DEBT-56 inventariadas; 5/5 factuais
  desactualizadas; acção: substituir por ADR-0078.
- ✓ §A.2 6 sítios `group_bbox` confirmados (mais que estimado);
  refactoring mecânico puro; net -24 LOC.
- ✓ §A.3 catch-all `_ => {}` confirmado; 4 variantes silenciadas
  (Text/Line/Glyph/Image); bug funcional reprodutível.
- ✓ §A.3.3 reformulação aplicada: match exaustivo com stubs vs full
  bug fix (cap LOC L3 hard 150 não estoura).
- ✓ §A.4 baseline workspace confirmado 2652 + lint zero.
- ✓ §A.6 gates: 1 parcial (#7) com reformulação; 7 zero disparos.

**Fase A produzida — critério §A.7 cumprido (1 gate disparado parcial
com reformulação documentada).**

---

## §A.8 — Plano §C operações

Per spec §C ordem: **C.1 → C.2 → C.3**.

### Sub-op 1 (C.1) — 5 LOC L0

Substituir 5 referências DEBT-56 em `00_nucleo/prompts/entities/content.md`:
- "(DEBT-56 + Fase 3 Layout)" → "(refactor multi-region; ADR-0078 §sub-fase b)"
- "multi-region (DEBT-56 + Fase 3)" → "multi-region (ADR-0078 §sub-fase b; DEBT-56 fechado P221 para columns/colbreak)"
- "refactor multi-region per DEBT-56" → "refactor multi-region per ADR-0078 §sub-fase b"
- "refactor multi-region (DEBT-56)" → "refactor multi-region (ADR-0078 §sub-fase b)"
- "(refino multi-region per DEBT-56)" → "(refino multi-region per ADR-0078 §sub-fase b)"

`crystalline-lint --fix-hashes` para propagar `content.md` hash.

### Sub-op 2 (C.2) — net -24 LOC L3

1. Adicionar helper privado em `03_infra/src/export.rs`:

```rust
/// Constrói o bbox cristalino (Y-down, sem inversion) de um Group dado
/// a sua posição e dimensions internas. Helper extraído P278 para
/// consolidar 6 sítios replicados (scan_all_gradients.walk +
/// pattern_resources_for_page.walk + draw_item_local + 3 build_page_stream
/// variantes).
fn group_bbox_from_fields(
    pos: typst_core::entities::layout_types::Point,
    inner_width: f64,
    inner_height: f64,
) -> typst_core::entities::layout_types::Rect {
    typst_core::entities::layout_types::Rect {
        x: typst_core::entities::layout_types::Pt(pos.x.0),
        y: typst_core::entities::layout_types::Pt(pos.y.0),
        w: typst_core::entities::layout_types::Pt(inner_width),
        h: typst_core::entities::layout_types::Pt(inner_height),
    }
}
```

2. Substituir 6 ocorrências literais por `group_bbox_from_fields(pos, *inner_width, *inner_height)`.

### Sub-op 3 (C.3) — ~25 LOC L3 match exaustivo

Substituir linha 2490 `_ => {}` por 4 arms explícitos:

```rust
FrameItem::Text { .. } => {
    // P278 — Text dentro de Group descartado: emit funcional requer
    // font scenario context (Helvetica/CIDFont/multifont). Bug fix
    // pleno fica como pendência específica `P279.X-bis-text-image-em-group-emit`.
}
FrameItem::Line { .. } => {
    // P278 — Line dentro de Group descartado: refino futuro emit
    // local. Pendência mesma cluster que Text.
}
FrameItem::Glyph { .. } => {
    // P278 — Glyph dentro de Group descartado: idem (requer font context).
}
FrameItem::Image { .. } => {
    // P278 — Image dentro de Group descartado: emit funcional requer
    // ptr_to_idx + img_refs threading. Pendência mesma cluster.
}
```

**Comportamento preserved bit-exact** (todos arms são no-op stubs). 
**Match exaustivo expõe limitação** explicitamente.

---

## §A.9 — Sub-padrões esperados

### "Extract helper de replicação inline" N=3 cumulativo

- N=1 (P273.11): Stack measurement helper extraído.
- N=2 (P277): `path_bbox`-`polygon()` consolidação implícita.
- **N=3 (P278 sub-op 2)**: `group_bbox_from_fields` helper extracted
  from 6 sites.

**Limiar formalização N≥3-4 atingido**. Decisão **NÃO formalizar
ADR** (anti-padrão over-formalização P273.17 §0). Registar em §5
do relatório.

### "Match exaustivo sem fall-through" N=2 cumulativo (cross-layer)

- N=1 (`is_locatable` L1): inaugural.
- **N=2 (P278 sub-op 3, L3)**: reaplicação em export.rs.

### "Cleanup combinado em passo único" N=1 inaugural

P278 inaugura. 3 sub-operações atómicas mas independentes no mesmo
passo.

### "Diagnóstico imutável" N=36 → N=37 cumulativo (32º consumo)

### "Reformulação de sub-op por cap LOC" N=1 inaugural

P278 sub-op 3 reformulada (match exaustivo stubs vs full bug fix)
para respeitar cap LOC L3 hard 150. Disciplina anti-scope-creep
preserved.

---

*Diagnóstico imutável produzido em 2026-05-18. 32º consumo.
3 sub-ops verificadas empiricamente; sub-op 3 reformulada per cap
LOC (match exaustivo stubs vs full bug fix); bug fix funcional
Text+Image em Group fica como pendência específica para passo
dedicado futuro. Sub-padrão "Extract helper de replicação inline"
N=3 cumulativo atinge limiar formalização — registado sem ADR
(anti-padrão over-formalização). Sub-padrão "Match exaustivo sem
fall-through" N=1→N=2 cumulativo cross-layer (L1+L3).*
