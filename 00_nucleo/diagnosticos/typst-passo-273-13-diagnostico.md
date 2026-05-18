# Diagnóstico Fase A P273.13.A — Fix draw_item_local Group gradient (caminho emit real)

**Data**: 2026-05-18.
**Passo**: typst-passo-273.13.A.
**Magnitude**: S documental (~20-25 min).
**Cluster**: Visualize / Gradient (quarto sub-passo na sequência terminar cluster — inserido por priorização B).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Vigésimo quarto consumo directo de fonte** (cluster Gradient
sub-padrão "L3-only parent_bbox" N=1 → N=2 cumulativo).

---

## §A.1 — Inventário literal `draw_item_local`

`03_infra/src/export.rs`:

- **Linha 2366** — `fn draw_item_local(ops: &mut String, item: &FrameItem)` declaração.
- **Linha 2370** — arm `FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit: _ }`.
  - `parent_bbox_at_emit: _` ignored — P273.7.1 cleanup; será revertido para uso real em P273.13 (paralelo a P273.12 para os 3 build_page_stream emit sites).
- **Linhas 2377-2380** — fallback solid color para stroke:
  ```rust
  if let Some(s) = stroke {
      let (r, g, b, _) = s.paint.to_color().to_rgba_f32();
      ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n",
                            r, g, b, s.thickness));
  }
  ```
  **Localização exacta do bug P263 §8 #3 + P273.12 §9 quarto bullet**:
  `s.paint.to_color()` ignora Paint::Gradient, retorna primeiro stop
  color. Gradient renderiza como solid color quando dentro de Group.
- **Linha 2449** — `_ => {}` catch-all silenciosamente descarta:
  - `FrameItem::Group` (Group dentro de Group); nested Groups
    silenciosamente perdidos.
  - `FrameItem::Text` (texto dentro de Group); diferido per
    comentário "Texto e outros tipos em grupos: adiado para passo
    futuro".

### Callsites de `draw_item_local` (3 sítios)

- **Linha 2234** — `build_page_stream_type1` Group arm
  (linhas 2209-2237). Loop `for child in items { draw_item_local(...) }`.
- **Linha 2720** — `build_page_stream_cidfont` Group arm. Análogo.
- **Linha 2906** — `build_page_stream_multifont` Group arm. Análogo.

Todos 3 callsites são **dentro do dispatch FrameItem::Group em
build_page_stream_*** — têm acesso a:
- `pat_ptr_to_idx` (function param).
- `pat_refs` (function param).
- `pos`, `inner_width`, `inner_height` (Group destructure local).

---

## §A.2 — Inventário `emit_stroke_paint` pós-P273.12

`03_infra/src/export.rs:1969`:

```rust
fn emit_stroke_paint(
    ops: &mut String,
    paint: &Paint,
    thickness: f64,
    effective_bbox: Option<Rect>,        // P273.12
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,
    pat_refs: &[PatternRef],
) {
    match paint {
        Paint::Solid(c) => { /* RGB literal */ }
        Paint::Gradient(g) => {
            // P273.12 — lookup via DedupKey bbox-aware
            let key = dedup_key_for(g, effective_bbox);
            if let Some(&idx) = pat_ptr_to_idx.get(&key) { /* /Pattern CS /Pi SCN */ }
            else { /* fallback solid */ }
        }
    }
}
```

`emit_stroke_paint_type1` é wrapper trivial.

Para `draw_item_local` consumir patterns reais, basta:
1. Adicionar 3 params (`effective_bbox`, `pat_ptr_to_idx`, `pat_refs`).
2. Substituir `s.paint.to_color()` fallback pela chamada
   `emit_stroke_paint(...)`.

---

## §A.3 — Decisão 1 fixada: mecanismo de propagação

**Fixada**: **1α — parameter threading explícito**.

Razões:
1. **Coerência com P273.10** mesmo mecanismo
   (`scan_all_gradients.walk`).
2. Sem custo de criar struct nova só para 3 params.
3. Cascade pequeno (3 callsites em build_page_stream_*) — controlado.
4. Sub-padrão "L3-only parent_bbox" cresce N=1 → **N=2 cumulativo
   emergente**.

Signature novo:

```rust
fn draw_item_local(
    ops: &mut String,
    item: &FrameItem,
    parent_bbox_override: Option<Rect>,        // P273.13 novo
    pat_ptr_to_idx: &HashMap<DedupKey, usize>, // P273.13 novo
    pat_refs: &[PatternRef],                   // P273.13 novo
)
```

---

## §A.4 — Decisão 2 fixada: Group bbox source

**Fixada**: **2α — Group bbox próprio** (paridade total P273.10).

`draw_item_local` arm `FrameItem::Group` constrói `group_bbox`
literal-equivalente a `scan_all_gradients.walk` arm Group + a
`pattern_resources_for_page.walk` arm Group:

```rust
let group_bbox = Rect {
    x: Pt(pos.x.0),
    y: Pt(pos.y.0),
    w: Pt(*inner_width),
    h: Pt(*inner_height),
};
for child in items {
    draw_item_local(ops, child, Some(group_bbox), pat_ptr_to_idx, pat_refs);
}
```

**Crítico**: literal-equivalente ao Group bbox calculado em
`scan_all_gradients.walk` (linhas 459-470 actuais) para que
`dedup_key_for(g, effective_bbox)` produza chave idêntica → lookup
encontra pattern registado.

**§A.6 verificação empírica**: construção é literal idêntica (3
copies do mesmo Rect construction em 3 funções).

### Pendência expansão escopo — Group arm novo em draw_item_local

Pre-P273.13: `draw_item_local` só tem arm Shape; nested Groups são
silenciosamente descartados via `_ => {}` (linha 2449). P273.13 spec
§3 sugere **adicionar arm Group** para suportar Group dentro de
Group + propagar `parent_bbox_override`.

Adicionar arm Group é scope creep mas:
1. Necessário para Decisão 2α paridade (Group dentro de Group precisa
   recurse via `draw_item_local` para emit children).
2. Corrige bug pre-existente (nested Groups silenciosamente perdidos).
3. LOC limitado (~10 LOC).

**Decisão Fase A**: incluir arm Group novo. **Scope creep aceito**
per paridade arquitectural.

---

## §A.5 — Decisão 3 fixada: coords cristalino

**Fixada**: **3α — coords cristalino** (paridade P273.10 §A.5 Decisão 2α).

`group_bbox` em coords cristalino (Y-down, sem Y-inversion).
Y-inversion é responsabilidade exclusiva do PDF emit final via
`apply_parent_transform` no dispatcher consumer (linhas 1638+).

Decisão crítica para `dedup_key_for(g, effective_bbox)` produzir
chave idêntica entre scan + emit. `rect_to_key` (P273.12) opera em
milipontos quantizados; consistência de coords é pré-requisito.

---

## §A.6 — Análise de risco

| Risco | Estado |
|---|---|
| Regressão tests P262-P273.12 | **Mitigado** — defaults preservam: Shapes top-level continuam a chamar `emit_stroke_paint` directo; Shapes em Group via `draw_item_local` agora também consomem (mas patterns registados em scan já existiam) |
| DedupKey lookup falha (Group bbox ≠ scan) | **Mitigado** — Decisão 2α + 3α literal-equivalente construction; sub-padrão "Triplicação Group bbox" emergente N=1 (candidato extract helper P273.X-bis-helper-group-bbox per spec §7) |
| Tests P273.12 quebram (patterns no longer unused) | **Verificar empírico** — testes P273.12 contam `/ShadingType` ocurrências; render path antes ignorava patterns dentro Group, agora consume |
| Cascade signature draw_item_local | 3 callsites — controlado |
| Nested Groups silenciosamente perdidos (pre-existing bug) | Mitigado por scope creep §A.4 — arm Group novo |

---

## §A.7 — Critério de aceitação Fase A

- ✓ §A.1 cita `draw_item_local` literal (linha 2366) + 3 callsites
  (linhas 2234/2720/2906) + sítio do fallback (linhas 2377-2380).
- ✓ §A.2 confirma `emit_stroke_paint` signature pós-P273.12 + sítio
  actual fallback substituível.
- ✓ §A.3 Decisão 1 fixada: **1α parameter threading**.
- ✓ §A.4 Decisão 2 fixada: **2α Group bbox próprio (paridade
  literal)** + scope creep arm Group aceito.
- ✓ §A.5 Decisão 3 fixada: **3α coords cristalino (paridade
  scan)**.
- ✓ §A.6 risco "DedupKey lookup falha" mitigado pela paridade
  literal triplicada (3 sítios).

**Fase A produzida — critério §A.7 cumprido absoluto.**

---

## §A.8 — Plano de implementação (Fase C)

### Cap LOC (ADR-0094 Pattern 1)

- **L3 hard cap**: ≤ 70 LOC.
- **L3 soft cap**: ≤ 50 LOC.
- **L1 hard cap**: 0 LOC.
- **Tests hard cap**: ≤ 10.
- **Tests soft cap**: ≤ 6.

### Estimativa LOC

| Site | LOC esperado |
|---|---|
| `draw_item_local` signature: 3 params adicionais | ~4 |
| Shape arm: destructure parent_bbox_at_emit + effective_bbox + emit_stroke_paint | ~6 |
| Group arm novo: group_bbox + for-loop recurse | ~10 |
| 3 callsites em build_page_stream_*: passar 3 args | ~6 |
| **Total L3** | **~26** |

Bem dentro do cap soft 50.

### Ordem literal

1. Fase A (este documento).
2. ADR-0091 décima terceira anotação cumulativa.
3. L0 `entities/gradient.md` anotação P273.13.
4. `crystalline-lint --fix-hashes`.
5. Tests-first (~5-6 testes; cap soft 6 respeitado).
6. Implementação L3 (~26 LOC).
7. Verificação final.

### Sub-padrões esperados

- **"L3-only parent_bbox"** N=1 → **N=2 cumulativo emergente**
  (P273.10 inaugural + P273.13 reaplicação).
- **"Sub-passos consecutivos do mesmo cluster"** N=8 → **N=9
  cumulativo emergente** (P273.5/6/7/8/9/10/11/12/13).
- **"Triplicação Group bbox"** N=0 → **N=1 emergente** —
  `scan_all_gradients.walk` + `pattern_resources_for_page.walk` +
  `draw_item_local` constroem mesmo `group_bbox` triplicadamente.
  Candidato extract helper futuro (P273.X-bis-helper-group-bbox).

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo quarto
consumo. Decisões 1α + 2α + 3α fixadas + scope creep arm Group
aceito; pronto para Fase C (~26 LOC L3; ~5-6 testes; pendência P263
§8 #3 + P273.12 §9 quarto bullet fechadas).*
