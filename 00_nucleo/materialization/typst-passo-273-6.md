# Passo P273.6 — P-Gradient-Relative-Callsite-Activation (fecho 3γ.2)

**Tipo**: refino estrutural fecho-de-pendência (activa `parent_bbox: Option<Rect>` deixado em `#[allow(dead_code)]` em P273.5).
**Magnitude estimada**: S (Layouter arms save/restore + L3 callsite consulta o campo; sem novas crates; sem novos tipos L1).
**Pré-requisitos**: P273.5 fechado (Rect struct + `parent_bbox` campo L1 + callsite L3 com page_bbox fallback).
**Cluster**: Visualize / Gradient (encerra refino estrutural definitivamente).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; oitava anotação cumulativa); ADR-0029 (pureza física L1); ADR-0093 (Pattern 2 anotação cumulativa).

---

## §0 — Contexto

P273.5 fechou parcialmente a pendência P273 §7:

- `apply_parent_transform` perdeu `#[allow(dead_code)]` (callsite L3 real activado).
- **Mas**: `parent_bbox: Option<Rect>` no Layouter ganhou novo `#[allow(dead_code)]` — o callsite L3 usa `page_bbox` directo como fallback, não consulta o campo Layouter.

Consequência factual registada no relatório P273.5 §9: **a path `RelativeTo::Parent` é identity transform por construção** porque `page_bbox` aplicado a coords já page-relative dá output bit-exact `RelativeTo::Self_`. Gradients aninhados em Block com `relative=parent` continuam a renderizar como `relative=self`.

P273.6 fecha esta lacuna: arms Block/Boxed populam `parent_bbox` com a bbox real do contentor; callsite L3 lê o campo do Layouter em vez de `page_bbox` directo; `#[allow(dead_code)]` cai por consumo real.

### Decisão 3 final activada

Recordando spec P273.5 §A.3:
- **3γ — Parent == contentor imediato com fallback página** (recomendação spec).
- Relatório P273.5 escolheu **3γ híbrida** com 3γ.1 (fallback página, identity) materializado + 3γ.2 (save/restore real) preservado.

P273.6 materializa **3γ.2** — completa a semântica 3γ.

### Precedente directo no Layouter cristalino

DEBT-37 P84.6 — `cell_origin_x/y/w: Option<f64>` no Layouter, save/restore no arm `Content::Grid`. Padrão N=2 (P84.6 + P273.5 estrutural). P273.6 atinge **N=3** real (com consumer), passando o limiar de formalização ADR meta N=3-4.

### Lista de contentores a tratar — discussão

A spec P273.5 §A.7 deixou a lista de contentores para a Fase A; P273.5 não fixou. P273.6 fixa explicitamente:

- **Obrigatórios**: `Content::Block` — container structural com bbox explicitamente computável (cursor + width/height ou page-width fallback).
- **Recomendado**: `Content::Boxed` — container inline com bbox local; vanilla aplica `RelativeTo::Parent` à bbox do Boxed.
- **Diferido**: `Content::Stack`, `Content::Group` (`FrameItem::Group`), `Content::Pad`, `Content::Grid` cells — cada um requer análise. Spec P273.6 cobre Block + Boxed; outros entram em sub-passo futuro `P273.7` se valer a pena, ou ficam permanentemente como "página fallback aceita per ADR-0054 graded".

---

## §1 — Sub-passo P273.6.A — Fase A diagnóstico

**Magnitude**: S documental (~20-30 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-6A-diagnostico.md`.

### §A.1 — Inventário do arm `Content::Block` no Layouter

Listar literal em `01_core/src/rules/layout/mod.rs`:

- Linha exacta do arm `Content::Block`.
- Estrutura actual: flush_line → inset.top → offset line_start_x/cursor.x → layout_content body → flush_line → inset.bottom → height min → restore line_start_x/cursor.x.
- Onde a largura efectiva do Block é conhecida no arm — `width: Option<Length>` resolvido vs `page_width - cursor.x` quando None.
- Onde a altura efectiva do Block é conhecida — `height: Option<Length>` resolvido vs altura medida pós-layout body.

### §A.2 — Inventário do arm `Content::Boxed` no Layouter

Análogo. Notar que Boxed é inline (sem flush_line), portanto bbox é diferente:
- x = cursor.x antes do body.
- y = cursor.y (baseline-relative — pode ser ajustada).
- w = `width: Option<Length>` resolvido vs largura medida do body.
- h = `height: Option<Length>` resolvido vs altura medida (line_height para inline).

### §A.3 — Decisão semântica do bbox: pré-layout vs pós-layout

Duas opções:

- **3γ.2.α — bbox pré-layout (estimativa)**: campo `parent_bbox` populado *antes* de `layout_content(body)` com `width`/`height` resolvidos ou estimativa (page_width − cursor.x para w; ∞ ou altura conservadora para h). Disponível ao body durante layout. **Custo**: alto risco de bbox errada quando `width=None` e o body determina a largura.
- **3γ.2.β — bbox pós-layout (medição)**: medir o body com `measure_content_constrained`, popular `parent_bbox` com o resultado, depois fazer `layout_content(body)` novamente para emitir. **Custo**: layout duplo (medição + emit).
- **3γ.2.γ — bbox pré-layout com fields conhecidos**: popular `parent_bbox` apenas quando `width` *e* `height` são `Some(...)`; caso contrário deixar `None` (cai no fallback page_bbox do callsite L3 P273.5). **Custo**: gradient `relative=parent` aninhado em Block sem dimensions explícitas continua a usar page_bbox — comportamento "transparente para Block dimension-less".

Recomendação spec: **3γ.2.γ**. Razões:
1. Sem layout duplo.
2. Sem risco de bbox errada (campo só populado quando dimensão é literal).
3. Semântica observable: utilizador que quer `relative=parent` rigoroso especifica `width`/`height` do container. Block sem dimensions cai no page (3γ.1 identity preserved).
4. Cumulativo: refino futuro pode adicionar medição se 3γ.2.γ for insuficiente empiricamente.

Decisão final na Fase A com base em análise de risco §A.5.

### §A.4 — Inventário do callsite L3 P273.5

Listar literal em `03_infra/src/export.rs`:

- Linhas exactas dos dispatcher arms Linear + Radial RGB-family pós-P273.5.
- Onde `page_bbox` é construído (hardcoded `(0.0, 0.0, page_w, page_h)`).
- Como o Layouter passa contexto ao L3 export — directo via `PagedDocument` (que já contém os `FrameItem`s)?

Ponto crítico: **`parent_bbox` do Layouter precisa de propagar até ao export L3**. Em P273.5 o callsite construiu o bbox no próprio export. Em P273.6 a bbox tem origem L1 (Layouter); precisa viajar via `FrameItem` ou estado equivalente.

Opções de propagação:
- **Prop-A**: `FrameItem::Shape` ganha campo `parent_bbox_at_emit: Option<Rect>` — capturado no momento do emit pelo Layouter. **Custo**: extensão do tipo L1, com cascade em todos os pattern-match (paridade P156C — 12 sítios; baixo).
- **Prop-B**: `FrameItem` ganha variante nova ou campo associado ao gradient especificamente — mais restritivo, menos cascade.
- **Prop-C**: Layouter expõe método ao export para consultar bbox por shape (acoplamento L3→L1 indirecto). Rejeitar (V3 linter ImpureCore / acoplamento errado).

Recomendação spec: **Prop-A** — `FrameItem::Shape` ganha campo `Option<Rect>`. Cascade controlado, pattern já estabelecido.

### §A.5 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P274 | Mudança em `FrameItem::Shape` cascade | Cascade pattern-match completo (12 sítios paridade P156C); campo `Option<Rect>` default `None` preserva semantic |
| Regressão tests P273.5 | Callsite L3 muda fonte do bbox | Quando `FrameItem::Shape.parent_bbox_at_emit` é `None`, fallback ao `page_bbox` directo P273.5 preservado literal |
| Bbox semanticamente errada | 3γ.2.α/β escolha errada | 3γ.2.γ (recomendada) só popula quando width+height literais |
| `#[allow(dead_code)]` permanece | Activação parcial novamente | §A.6 critério: pelo menos um callsite consulta `Layouter.parent_bbox` real |
| Pureza física L1 quebrada | Save/restore arm Block toca Rect (já L1) | Rect é tipo dados L1; save/restore é cursor.x.0 etc. — gestão RAM, não I/O |

### §A.6 — Critério de fecho do `#[allow(dead_code)]` no Layouter

`parent_bbox: Option<Rect>` no Layouter perde `#[allow(dead_code)]` quando:

1. Arm `Content::Block` (e/ou `Content::Boxed`) faz save/restore real do campo.
2. Layouter, no momento de emitir um `FrameItem::Shape` com gradient, lê `self.parent_bbox` e popula `FrameItem::Shape.parent_bbox_at_emit` (Prop-A).
3. Callsite L3 P273.5 lê `shape.parent_bbox_at_emit` e usa-o *em vez do* `page_bbox` directo quando `Some(...)`.
4. `cargo build` sem warning de dead code no campo Layouter sem `#[allow]`.

### §A.7 — Decisões a fixar na Fase A

1. **Decisão 1** (semântica bbox): 3γ.2.α / 3γ.2.β / 3γ.2.γ. Recomendação spec: 3γ.2.γ.
2. **Decisão 2** (propagação L1→L3): Prop-A / Prop-B / Prop-C. Recomendação spec: Prop-A.
3. **Decisão 3** (lista contentores fase actual): {Block} / {Block, Boxed} / {Block, Boxed, Stack, Pad, Group}. Recomendação spec: {Block, Boxed}. Outros ficam scope-out per ADR-0054 graded.

### §A.8 — Critério de aceitação Fase A

- §A.1 cita arm Block literal (path:linha).
- §A.2 cita arm Boxed literal.
- §A.4 cita callsite L3 literal pós-P273.5.
- §A.5 risco "regressão P273.5" mitigado com fallback explícito.
- §A.7 decisões 1/2/3 fixadas com fundamento.

---

## §2 — Sub-passo P273.6.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental.

**Não criar ADR-0095**. Anotar ADR-0091 — oitava anotação consecutiva.

Template:

```
## Anotação cumulativa P273.6 — Parent bbox real save/restore (fecha 3γ.2)

**Data**: 2026-05-XX.
**Motivo**: P273.5 materializou 3γ.1 (callsite L3 com page_bbox fallback;
identity transform). 3γ.2 ficou pendente — Layouter `parent_bbox` em
`#[allow(dead_code)]` sem consumer real. P273.6 fecha: arms Block/Boxed
save/restore real; callsite L3 consulta bbox via FrameItem.

**Decisão 1 (semântica bbox)**: [3γ.2.α/β/γ — preencher pós-Fase A].
**Decisão 2 (propagação L1→L3)**: [Prop-A/B/C — preencher pós-Fase A].
**Decisão 3 (lista contentores)**: [conjunto fixado pós-Fase A].

**Pattern DEBT-37 `cell_origin_*` replicado**: terceira aplicação real
(P84.6 + P273.5 estrutural + **P273.6 com consumer real**). Sub-padrão
N=2 → 3 cumulativo — atinge limiar formalização ADR meta N=3-4.

**Defaults preservam P262-P273.5 bit-exact**:
- Shape sem `parent_bbox_at_emit` (None) → fallback page_bbox P273.5
  preservado literal.
- 2605 baseline P273.5 preserved.

**`#[allow(dead_code)]` no Layouter fechado** — campo `parent_bbox`
consumed por arms Block/Boxed (write) + Layouter emit_shape (read).
```

---

## §3 — Sub-passo P273.6.C — Materialização (testes primeiro)

**Magnitude**: S (~50-90 LOC consoante Decisões 1+2+3).

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0091 anotação §2 escrita pós-fixação Decisões.
3. `crystalline-lint --fix-hashes` (refactor preserva hashes L0 onde possível; `layout_types.md` propaga se `FrameItem::Shape` ganhar campo).
4. **Testes-primeiro**.
5. Código:
   - L1 — `FrameItem::Shape` ganha `parent_bbox_at_emit: Option<Rect>` (se Prop-A; default None).
   - L1 — pattern-match cascade completo paridade P156C (12 sítios; PartialEq, map_*, debug etc. — `FrameItem` é tipo L1 com cascade conhecido).
   - L1 — arm `Content::Block` save/restore `parent_bbox` (e arm `Content::Boxed` se Decisão 3 inclui).
   - L1 — Layouter emit shape popula `parent_bbox_at_emit` a partir de `self.parent_bbox`.
   - L1 — remove `#[allow(dead_code)]` do campo `parent_bbox`.
   - L3 — callsite consulta `shape.parent_bbox_at_emit`; usa quando `Some(...)`; fallback `page_bbox` directo quando `None`.
6. Verificação final.

### Cap LOC (ADR-0094 Pattern 1)

- **L1 hard cap**: ≤ 100 LOC (cascade FrameItem + arms Block+Boxed + emit shape).
- **L1 soft cap**: ≤ 70 LOC.
- **L3 hard cap**: ≤ 40 LOC (callsite dispatcher).
- **L3 soft cap**: ≤ 25 LOC.
- **Tests hard cap**: ≤ 15 novos.
- **Tests soft cap**: ≤ 10.

### Tests propostos (lista mínima — completar pós-Fase A)

1. `p273_6_block_save_restore_parent_bbox` — entrar em Block com width+height literais guarda bbox; sair restaura `None` (ou bbox anterior se aninhado).
2. `p273_6_block_dimensionless_no_parent_bbox` — Block sem width/height não popula `parent_bbox` (Decisão 3γ.2.γ).
3. `p273_6_boxed_save_restore_parent_bbox` — análogo Boxed.
4. `p273_6_nested_blocks_lifo` — Block dentro de Block — restore LIFO correcto.
5. `p273_6_shape_carries_parent_bbox_at_emit` — `FrameItem::Shape` emitido dentro de Block ganha `Some(bbox_do_block)`.
6. `p273_6_shape_outside_block_no_parent_bbox` — `FrameItem::Shape` top-level tem `parent_bbox_at_emit = None`.
7. `p273_6_gradient_relative_parent_inside_block_uses_block_bbox` — Linear gradient `relative=parent` aninhado em Block 200×100 emit usa bbox do Block (não page); transform observable real.
8. `p273_6_radial_relative_parent_mirrors_linear` — paridade Radial.
9. `p273_6_gradient_relative_parent_outside_block_uses_page_bbox` — Top-level continua a usar page_bbox P273.5 (regressão zero).
10. `p273_6_gradient_relative_self_unchanged` — `relative=self` inalterado bit-exact P272.
11. `p273_6_gradient_relative_auto_unchanged` — `relative=auto` resolve Self bit-exact P273.
12. `p273_6_layouter_parent_bbox_no_longer_allow_dead_code` — verificação `cargo build` sem warning de dead code no campo Layouter sem `#[allow]`.
13. Regressão integrada: rodar suite P262-P273.5 — 2605 verdes inalterados.

### Alterações esperadas no código

```rust
// L1 — entities/layout_types.rs (FrameItem::Shape)
pub enum FrameItem {
    // ...
    Shape {
        pos: Point,
        kind: ShapeKind,
        fill: Option<Paint>,
        stroke: Option<Stroke>,
        clip_mask: Option<ShapeKind>,
        /// P273.6 — bbox do contentor imediato no momento do emit.
        /// `Some(rect)` quando shape foi emitida dentro de Block/Boxed
        /// com dimensions literais. `None` quando top-level ou contentor
        /// sem dimensions (cai no fallback page_bbox L3 P273.5).
        parent_bbox_at_emit: Option<Rect>,
    },
    // ...
}

// L1 — rules/layout/mod.rs Layouter struct
pub struct Layouter<'a, M: FontMetrics, S: ImageSizer = NullImageSizer> {
    // ...
-   #[allow(dead_code)]
    pub(super) parent_bbox: Option<Rect>,
}

// L1 — arm Content::Block
Content::Block { body, width, height, inset, breakable } => {
    // ... (flush_line, inset.top, offset cursor.x existente) ...

    // P273.6 — save/restore parent_bbox (Decisão 3γ.2.γ: só popular se literal)
    let saved_parent_bbox = self.parent_bbox;
    if let (Some(w), Some(h)) = (width, height) {
        let w_pt = w.resolve_pt(self.font_size_pt.0);
        let h_pt = h.resolve_pt(self.font_size_pt.0);
        self.parent_bbox = Some(Rect {
            x: Pt(self.cursor_x.0),
            y: Pt(self.cursor_y.0),
            w: Pt(w_pt),
            h: Pt(h_pt),
        });
    }
    // (se width ou height None, parent_bbox preserva o valor outer — caller decide)

    self.layout_content(body);

    self.parent_bbox = saved_parent_bbox;
    // ... (flush_line, inset.bottom existente) ...
}

// L1 — arm Content::Boxed análogo (inline; sem flush_line)

// L1 — emit shape (FrameItem::Shape construído)
self.current_line.push(FrameItem::Shape {
    // ... existing fields ...
    parent_bbox_at_emit: self.parent_bbox,  // P273.6 — copia o campo actual
});

// L3 — 03_infra/src/export.rs callsite
// Quando a shape vem com Some(rect), usa-a; senão fallback P273.5
let parent_bbox = shape.parent_bbox_at_emit
    .map(|r| (r.x.0 as f32, r.y.0 as f32, r.w.0 as f32, r.h.0 as f32))
    .unwrap_or_else(|| (0.0, 0.0, page_w as f32, page_h as f32));  // P273.5 fallback

if relative == RelativeTo::Parent {
    let local = compute_local_coords(...);
    let (tx0, ty0, tx1, ty1) = apply_parent_transform(local, Some(parent_bbox));
    // ...
}
```

### Verificação final

- Cap LOC respeitado (provável estouro soft em L1 dado cascade FrameItem; aceitável per Pattern 1 com registo).
- `cargo build` sem warning de dead code em `parent_bbox` Layouter.
- `cargo test --workspace` verde — 2605 → 2605 + tests novos.
- `crystalline-lint .` zero violations.
- Hashes L0 — `layout_types.md` propaga (`FrameItem::Shape` cascade); arms Block/Boxed cascade em prompts/entities/content.md provavelmente propaga; `layout.md` propaga (Layouter consumer).
- Tests P262-P273.5 inalterados bit-exact.
- DEBT saldo 10 preserved.
- Test #7 produz output PDF observable diferente vs P273.5 — confirmação empírica de que 3γ.2 dá semântica real.

---

## §4 — Sub-padrões cumulativos pós-P273.6

| Sub-padrão | Pós-P273.5 | Pós-P273.6 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 14 | 15 |
| Reutilização literal helpers cross-passos | 14 | 15 |
| Cap LOC hard vs soft explícito | 8 | 9 |
| Aplicação meta-ADR (ADR-0093) | 3 | 4 |
| Aplicação meta-ADR (ADR-0094) | 4 | 5 |
| Diagnóstico imutável | 21 (16º consumo) | 22 (17º consumo) |
| **Pattern DEBT-37 `cell_origin_*` replicado** | N=2 (P84.6 + P273.5 estrutural) | **N=3** (atinge limiar formalização N=3-4) |
| Cascade pattern-match cross-FrameItem (P156C N=12) | N=1 | N=2 |
| **Sub-passos decimais consecutivos do mesmo cluster** | 1 (P273.5) | **2** (P273.5+P273.6) — sub-padrão emergente N=1→2 |

Sub-padrão "Pattern DEBT-37 replicado" atinge N=3 — limiar formalização ADR meta atingido. Candidato a sub-passo administrativo XS futuro NÃO reservado.

---

## §5 — Limitações conscientes P273.6

- Decisão 3γ.2.γ (Block/Boxed com dimensions literais) — Block sem dimensions continua a usar page_bbox fallback (3γ.1). Refino futuro com medição pós-layout fica fora de escopo.
- Lista de contentores: Block + Boxed apenas. Stack, Pad, Grid cell, FrameItem::Group ficam fora — per ADR-0054 graded; sub-passo P273.7 candidato se valer a pena.
- `parent_bbox` armazena bbox aproximado a partir de cursor + dimensions resolvidas; refino exacto com bbox computado post-layout não está incluído.
- Saved/restored LIFO simples — não recovers de panic mid-body (acceptable; pattern uniforme com DEBT-37 + show_rules truncate).

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.8 + revê Decisões 1/2/3.
5. Utilizador executa P273.6.B + P273.6.C → relatório.
6. Utilizador upload do relatório.
7. Claude web analisa + propõe próximo passo.

---

## §7 — Pendências preservadas

Inalteradas vs P273.5:

- P-Gradient-CMYK-ICC (S-M).
- ADR-0055bis variant-aware fonts (M).
- P-Footnote-N (M).
- DEBT-33 Bézier bbox (S+M).
- Stroke\<Length\> / Curve / Polygon (S+M).
- Tiling activação.
- Outro cluster — saída Visualize/Gradient.

Pendências específicas pós-P273.6 (se desejado):
- **P273.7 — parent_bbox para outros contentores** (Stack, Pad, Grid cell, FrameItem::Group): candidato se houver demand empírica; fica fora de escopo até prova de necessidade.
- **P273.6.bis — bbox medido pós-layout** (refino Decisão 1 para 3γ.2.β/α): refino se 3γ.2.γ for empiricamente insuficiente.

**Pós-P273.6 fecha cluster Gradient refino estrutural definitivamente** — `relative=parent` ganha semântica observable real para Block+Boxed. Próximo passo natural: sair do cluster Gradient.

---

## §8 — Critério de fecho do passo

P273.6 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.8 cumprido.
- ADR-0091 anotada (oitava anotação consecutiva).
- L1 + L3 alterados dentro do cap LOC (soft estouros aceitáveis com registo).
- `#[allow(dead_code)]` removido de `parent_bbox` Layouter; `cargo build` sem warning.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P273.5 inalterados bit-exact.
- DEBT saldo 10 preserved.
- Test E2E que confirme output PDF observable diferente vs P273.5 quando gradient `relative=parent` aninhado em Block com dimensions literais.
- Sub-padrões §4 atualizados.

---

## §9 — Numeração — nota

Spec usa **P273.6** continuando o pattern decimal P273.5. Justificação: refino estrutural fechando pendência directa de P273.5 (que por sua vez fechou parcialmente P273 §7). Pattern de decimais consecutivos estabelecido (P270.1-P270.4; P156C-L). Alternativa P275 viável se preferires numeração principal — decidir antes da Fase A.

Pós-P273.6, a cadeia de pendências do cluster Gradient está empiricamente fechada:
- P273 declarou `relative` field cross-variant + `apply_parent_transform` helper (com `#[allow(dead_code)]` consumer-pending).
- P273.5 activou consumer L3 (com `#[allow(dead_code)]` Layouter-field-pending).
- P273.6 activa Layouter field via save/restore (com `#[allow(dead_code)]` zero).

3γ pleno materializado: contentor imediato (Block+Boxed) com fallback página (3γ.1 preserved para contextos não cobertos).
