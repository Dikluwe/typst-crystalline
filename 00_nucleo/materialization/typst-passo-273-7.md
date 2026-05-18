# Passo P273.7 — P-Gradient-Relative-Callsite-Boxed (completa Decisão 3 P273.6)

**Tipo**: refino estrutural — extensão directa de P273.6 para `Content::Boxed`.
**Magnitude estimada**: S menor que P273.6 (sem cascade FrameItem novo; apenas mais um arm save/restore + 1 emit shape site).
**Pré-requisitos**: P273.6 fechado (FrameItem::Shape.parent_bbox_at_emit cascade ~86 sites; Block save/restore real; L3 dispatcher consumindo effective_parent_bbox).
**Cluster**: Visualize / Gradient (extensão final do refino estrutural).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; nona anotação cumulativa); ADR-0029 (pureza física L1 preserved); ADR-0093 (Pattern 2 anotação cumulativa).

---

## §0 — Contexto

P273.6 fechou 3γ.2 para `Content::Block` apenas — a Fase A reduziu o escopo a `{Block}` per cap LOC após o cascade ~86 sites no `FrameItem::Shape`. Boxed ficou registado como pendência específica em §8 do relatório P273.6 ("P273.7 — Boxed save/restore (se necessário; análogo Block)").

P273.7 fecha essa pendência aplicando o padrão Block literal a `Content::Boxed`.

### O que NÃO muda vs P273.6

- `FrameItem::Shape.parent_bbox_at_emit: Option<Rect>` — campo já existe; cascade já feito.
- L1 `Rect` struct — já existe (P273.5).
- Layouter `parent_bbox: Option<Rect>` — campo já existe; consumer real activo no arm Block (P273.6).
- L3 GradientObject + dispatcher — já consome `effective_parent_bbox`.
- L1 `#[allow(dead_code)]` zero — já fechado P273.6.

### O que muda

- L1 arm `Content::Boxed` ganha save/restore análogo ao arm Block.
- L1 emit shape site dentro de Boxed (linha 1478 per relatório P273.6 §2.3) já popula `parent_bbox_at_emit: self.parent_bbox` — sem alteração necessária aí (o emit site lê o campo Layouter actual, que agora é populado também por Boxed save/restore).

Resultado: gradient `relative=parent` aninhado em `#box(width: ..., height: ..., body)` ganha semântica observable real (paridade Block).

### Limites mantidos vs ampliados

- **Decisão 1** (semântica bbox 3γ.2.γ) — mantida literal: popular `parent_bbox` apenas quando `width.is_some() && height.is_some()` no Boxed.
- **Decisão 2** (propagação L1→L3 Prop-A revisitada) — mantida literal: nenhuma mudança L3.
- **Decisão 3** (lista contentores) — passa de `{Block}` para `{Block, Boxed}`. Stack/Pad/Group/Grid cell continuam scope-out per ADR-0054 graded.

### Ambiguidade técnica do Boxed: bbox.y em contexto inline

Block é structural — `cursor.y` no momento do arm corresponde ao topo do block. Boxed é inline — `cursor.y` é a baseline da linha em curso, não o topo do box.

Vanilla `RelativeTo::Parent` para gradient dentro de box vê o box como contentor com origin no canto top-left do box. Em Boxed cristalino actual, **o topo do box não está definido com precisão** porque o Layouter inline não recalcula line_height para o box (per P156H limitação consciente: "height em contexto inline alteraria line_height — refino futuro"). A spec resolve:

- **3γ.2.γ no Boxed**: `parent_bbox.y` populated com `cursor.y` directo. Aproximação aceitável — gradient é orientado pelo box mas com baseline-relative y. Per ADR-0054 graded; precisão exacta fica fora de escopo, alinhado com a limitação P156H pre-existente.

Decisão de aceitar aproximação na Fase A (ou rejeitar e diferir P273.7 inteiro até refactor inline line_height existir).

---

## §1 — Sub-passo P273.7.A — Fase A diagnóstico

**Magnitude**: XS-S documental (~15-20 min — menor que P273.6.A porque a maior parte do diagnóstico está em P273.6.A).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-7A-diagnostico.md`.

### §A.1 — Inventário do arm `Content::Boxed` no Layouter

Listar literal em `01_core/src/rules/layout/mod.rs`:

- Linha exacta do arm `Content::Boxed`.
- Estrutura actual: inset.left avance cursor.x → layout_content body → inset.right avance cursor.x.
- Onde a linha 1478 (emit shape interno per relatório P273.6 §2.3) está em relação ao arm Boxed.
- Confirmar que o emit shape site já popula `parent_bbox_at_emit: self.parent_bbox` (P273.6 alteração).

### §A.2 — Inventário do arm `Content::Block` pós-P273.6 (referência template)

Listar literal o save/restore P273.6 implementado em Block — é o template a replicar:

```rust
// Template do P273.6 (Block arm):
let saved_parent_bbox = self.parent_bbox;
if let (Some(w), Some(h)) = (width, height) {
    let w_pt = w.resolve_pt(font);
    let h_pt = h.resolve_pt(font);
    self.parent_bbox = Some(Rect {
        x: self.regions.current.cursor_x,
        y: self.regions.current.cursor_y,
        w: Pt(w_pt),
        h: Pt(h_pt),
    });
}
// ... layout body ...
self.parent_bbox = saved_parent_bbox;
```

### §A.3 — Decisão semântica bbox.y para Boxed inline

Opções:

- **3γ.2.γ-inline-baseline-y** (recomendada): `bbox.y = cursor.y` literal. Aproximação aceitável — alinha com a limitação consciente P156H "baseline mid-linha não suportada"; aproxima sufficient para gradient propósito visual. Per ADR-0054 graded.
- **3γ.2.γ-inline-topo-estimado**: `bbox.y = cursor.y - ascender`. Tenta o topo do box subtraindo o ascender da font corrente. Mais correcto semanticamente; introduz dependência adicional na font_metrics no arm.
- **3γ.2.γ-inline-defer**: Não popular `parent_bbox` no Boxed enquanto refactor line_height inline não existir. Boxed continua a usar fallback page_bbox (3γ.1). Conservador.

Recomendação spec: **3γ.2.γ-inline-baseline-y** (aproximação documentada). Razões:
1. Não introduz dependência na font_metrics no arm save/restore.
2. Coerente com limitação P156H pre-existente.
3. Test E2E pode validar que output PDF DIFERE de fallback page_bbox — semântica observable mesmo com bbox.y aproximada.

Decisão final na Fase A.

### §A.4 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P273.6 | Save/restore Boxed afecta outras shapes inline | Defaults: Boxed sem dimensions literais → `parent_bbox` outer preservado; coerente com Block |
| bbox.y semanticamente errada | Aproximação baseline-relative vs topo-relative | Limitação consciente documentada §5; coerente P156H |
| Cap LOC estourado | Spec menor que P273.6 esperado | Cap hard L1 ≤30 LOC; soft ≤20 (apenas 1 arm save/restore + 0 cascade) |
| `#[allow(dead_code)]` reabrir | Nenhuma estructura nova | Zero `#[allow]` introduzido — emit shape site já lê `self.parent_bbox` desde P273.6 |
| Test E2E não-observable | Boxed bbox.y aproximada produz transform pouco distinguível | Test E2E usa Boxed com dimensions visualmente significativas (e.g. 200×100 pt vs page 595×842) |

### §A.5 — Decisões a fixar na Fase A

1. **Decisão 1** (semântica bbox.y inline): 3γ.2.γ-inline-baseline-y / -topo-estimado / -defer. Recomendação spec: -baseline-y.
2. **Confirmação**: nenhuma outra decisão — todos os outros aspectos (3γ.2.γ semântica W/H; Prop-A propagação; cap LOC; testes) herdam de P273.6 literal.

### §A.6 — Critério de aceitação Fase A

- §A.1 cita arm Boxed literal (path:linha) + emit shape site interno (linha 1478 per relatório P273.6).
- §A.2 cita template P273.6 (Block arm) que será replicado.
- §A.3 Decisão 1 fixada.
- §A.4 risco "regressão P262-P273.6" mitigado por save/restore análogo Block.
- §A.5 confirmação que P273.7 não introduz `#[allow(dead_code)]` novo.

---

## §2 — Sub-passo P273.7.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental.

**Não criar ADR-0095**. Anotar ADR-0091 — nona anotação consecutiva.

Template:

```
## Anotação cumulativa P273.7 — Boxed save/restore (completa Decisão 3 P273.6)

**Data**: 2026-05-XX.
**Motivo**: P273.6 fechou 3γ.2 para Content::Block apenas; Boxed
diferido per cap LOC. P273.7 estende save/restore para Content::Boxed
aplicando template P273.6 literal.

**Decisão 1 fixada (bbox.y inline)**: [3γ.2.γ-inline-baseline-y /
-topo-estimado / -defer — preencher pós-Fase A].

**Mudanças**:
- L1 arm Content::Boxed: save/restore parent_bbox análogo Block.
- L1 emit shape site interno Boxed: já populated desde P273.6
  (sem alteração).
- L3 dispatcher: inalterado (consome effective_parent_bbox).

**Pattern DEBT-37 cell_origin_* replicado** N=3 → N=3 (não cresce —
P273.7 é extensão da terceira aplicação, não quarta). Cluster
Gradient mantém o limiar atingido N=3-4 atingido em P273.6.

**Defaults preservam P262-P273.6 bit-exact**:
- Boxed sem dimensions literais → `parent_bbox` outer preservado.
- Self_/None relative ignora `parent_bbox_at_emit`.
- 2612 baseline P273.6 preserved.

**Limitação consciente bbox.y aproximada**: cursor.y é baseline em
contexto inline; refino topo-exacto requer refactor line_height
(diferido). Coerente com P156H limitação consciente.

**Sub-padrões cumulativos**:
- Anotação cumulativa em vez de ADR nova N=15 → 16.
- Reutilização literal helpers cross-passos N=15 → 16.
- Sub-passos decimais consecutivos do mesmo cluster N=2 → 3.
- Cap LOC hard vs soft explícito N=9 → 10.
- Aplicação meta-ADR (ADR-0093) N=4 → 5.
- Aplicação meta-ADR (ADR-0094) N=5 → 6.
```

---

## §3 — Sub-passo P273.7.C — Materialização (testes primeiro)

**Magnitude**: XS-S (~15-30 LOC L1; 0 L3; 5-7 testes novos).

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0091 anotação §2 escrita pós-Decisão 1.
3. `crystalline-lint --fix-hashes` — refactor preserva hashes L0 onde possível; `content.md` anotação Boxed propaga.
4. **Testes-primeiro**.
5. Código:
   - L1 — arm `Content::Boxed` save/restore análogo Block (template P273.6 literal com bbox.y per Decisão 1).
   - **Sem cascade** — `FrameItem::Shape.parent_bbox_at_emit` campo já existe.
   - **Sem L3** — dispatcher já consome `effective_parent_bbox`.
6. Verificação final.

### Cap LOC (ADR-0094 Pattern 1)

- **L1 hard cap**: ≤ 30 LOC (apenas 1 arm save/restore).
- **L1 soft cap**: ≤ 20 LOC.
- **L3 hard cap**: 0 LOC (nenhuma alteração esperada).
- **Tests hard cap**: ≤ 8 novos.
- **Tests soft cap**: ≤ 5.

### Tests propostos (lista mínima — completar pós-Fase A)

1. `p273_7_boxed_save_restore_parent_bbox` — entrar em Boxed com `width=Some, height=Some` guarda bbox; sair restaura LIFO.
2. `p273_7_boxed_dimensionless_no_parent_bbox` — Boxed sem `width` ou sem `height` não popula (Decisão 3γ.2.γ).
3. `p273_7_nested_boxed_lifo` — Boxed dentro de Boxed — restore LIFO.
4. `p273_7_boxed_inside_block_outer_wins_then_restores` — Block contém Boxed contém shape; bbox interno do Boxed é o usado; após sair do Boxed, bbox do Block volta.
5. `p273_7_gradient_relative_parent_inside_boxed_uses_boxed_bbox` — Linear gradient `relative=parent` aninhado em Boxed 200×100pt emit usa bbox do Boxed; PDF bytes diferem de page-bbox fallback. **Crítico para confirmar observable diff.**
6. `p273_7_radial_relative_parent_inside_boxed_mirrors_linear` — paridade Radial.
7. `p273_7_gradient_relative_self_unchanged_inside_boxed` — `relative=self` aninhado em Boxed inalterado bit-exact P272.
8. Regressão integrada: 2612 verdes preserved.

### Alterações esperadas no código

```rust
// L1 — rules/layout/mod.rs arm Content::Boxed

Content::Boxed { body, width, height, inset, baseline } => {
    // ... (inset.left existente) ...

    // P273.7 — save/restore parent_bbox análogo Block P273.6 (Decisão 3γ.2.γ).
    let saved_parent_bbox = self.parent_bbox;
    if let (Some(w), Some(h)) = (width, height) {
        let w_pt = w.resolve_pt(font);
        let h_pt = h.resolve_pt(font);
        self.parent_bbox = Some(Rect {
            x: self.regions.current.cursor_x,
            // Decisão 1: cursor.y baseline-relative (3γ.2.γ-inline-baseline-y).
            // Aproximação aceitável; coerente com P156H limitação line_height inline.
            y: self.regions.current.cursor_y,
            w: Pt(w_pt),
            h: Pt(h_pt),
        });
    }

    self.layout_content(body);

    // P273.7 — restore parent_bbox LIFO.
    self.parent_bbox = saved_parent_bbox;

    // ... (inset.right existente) ...
}
```

**Nada mais**. Cascade `FrameItem::Shape.parent_bbox_at_emit` já feito P273.6. Emit shape site interno Boxed (linha 1478 per relatório P273.6 §2.3) já lê `self.parent_bbox` (que agora é populated também por Boxed save/restore). L3 dispatcher já consome o campo.

### Verificação final

- Cap LOC respeitado (~15-25 LOC esperado).
- `cargo build` sem novos warnings de dead code.
- `cargo test --workspace` verde — 2612 → 2612 + tests novos.
- `crystalline-lint .` zero violations.
- Hashes L0 — `content.md` propaga (anotação Boxed P273.7); outros preserved.
- Tests P262-P273.6 inalterados bit-exact.
- DEBT saldo 10 preserved.
- Test #5 produz output PDF observable diferente vs fallback — confirmação 3γ.2 dá semântica real também para Boxed.

---

## §4 — Sub-padrões cumulativos pós-P273.7

| Sub-padrão | Pós-P273.6 | Pós-P273.7 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 15 | 16 |
| Reutilização literal helpers cross-passos | 15 | 16 |
| Cap LOC hard vs soft explícito | 9 | 10 |
| Aplicação meta-ADR (ADR-0093) | 4 | 5 |
| Aplicação meta-ADR (ADR-0094) | 5 | 6 |
| Diagnóstico imutável | 22 (17º consumo) | 23 (18º consumo) |
| Pattern DEBT-37 `cell_origin_*` replicado | N=3 (limiar atingido) | N=3 (P273.7 é extensão da 3ª aplicação, não 4ª) |
| Cascade pattern-match cross-FrameItem | N=2 | N=2 (P273.7 sem cascade novo) |
| **Sub-passos decimais consecutivos do mesmo cluster** | **N=2** (P273.5+P273.6) | **N=3 emergente** (+P273.7) |
| **Template-passo replicado literal** | N=0 antes | **N=1 emergente** — P273.7 replica template P273.6 com 1 ajuste (bbox.y inline-baseline) |

Sub-padrão emergente "Template-passo replicado literal" inaugura — P273.7 aplica o save/restore P273.6 literalmente a outro arm com diferença mínima (bbox.y semantic). Precedente análogo a P156H que replicou template P156G (vide histórico ADR-0061 — "padrão Block reaplicado a Boxed sem nova decisão arquitectural"). Promoção a sub-padrão consolidado candidato N=3-4.

---

## §5 — Limitações conscientes P273.7

- Decisão 3γ.2.γ-inline-baseline-y — `bbox.y` populated com `cursor.y` baseline-relative em vez de topo do box. Aproximação aceitável per ADR-0054 graded; alinhada com limitação P156H "height em contexto inline alteraria line_height — refino futuro".
- Boxed sem dimensions literais continua a usar `parent_bbox` outer (que pode ser fallback page_bbox). Refino com medição pós-layout fora de escopo.
- Lista de contentores activos: {Block, Boxed}. Stack/Pad/Grid cell/FrameItem::Group continuam scope-out — sub-passos futuros candidatos NÃO reservados.
- Dedup bbox-aware (relatório P273.6 §9) continua aberto — gradients dedup'd por Arc; primeira occurrence captura bbox. P273.7 não altera essa limitação.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico XS.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.6 + revê Decisão 1.
5. Utilizador executa P273.7.B + P273.7.C → relatório.
6. Utilizador upload do relatório.
7. Claude web analisa + propõe próximo passo.

---

## §7 — Pendências preservadas pós-P273.7

Inalteradas vs P273.6 (nível cluster):

- P-Gradient-CMYK-ICC (S-M).
- ADR-0055bis variant-aware fonts (M).
- P-Footnote-N (M).
- DEBT-33 Bézier bbox (S+M).
- Stroke\<Length\> / Curve / Polygon (S+M).
- Tiling activação.
- Outro cluster — saída Visualize/Gradient.

Pendências específicas pós-P273.7 (incremental per ADR-0054 graded):
- **P273.9** — Stack/Pad/Group/Grid cell save/restore (out of scope; per ADR-0054 graded; candidato se houver demand empírica). [P273.8 foi renomeado para cleanup `unused_variable: parent_bbox_at_emit` — esta reserva avança para P273.9.]
- **P273.X-bis — Bbox medido pós-layout** (refino 3γ.2.β/α se 3γ.2.γ for empiricamente insuficiente).
- **P273.X-bis2 — Bbox.y topo-exacto inline** (refino Decisão 1 se aproximação baseline-y for visualmente insuficiente).
- **Dedup bbox-aware** — gradient com mesmo Arc em contextos distintos.

**Pós-P273.7 cluster Gradient refino estrutural definitivamente encerrado para containers structural+inline canónicos**. Próximo passo natural: sair do cluster Gradient.

---

## §8 — Critério de fecho do passo

P273.7 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.6 cumprido.
- ADR-0091 anotada (nona anotação consecutiva).
- L1 alterado dentro do cap LOC.
- `cargo build` sem novos warnings de dead code.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P273.6 inalterados bit-exact.
- DEBT saldo 10 preserved.
- Test E2E #5 confirma output PDF observable diferente vs fallback quando gradient `relative=parent` aninhado em Boxed com dimensions literais.
- Sub-padrões §4 atualizados.

---

## §9 — Numeração — nota

Spec usa **P273.7** continuando pattern decimal P273.5/P273.6. Justificação: extensão directa da Decisão 3 reduzida em P273.6; magnitude S menor; sem feature user-facing nova.

Cadeia P273 → P273.5 → P273.6 → P273.7 documentada literal:
- P273 — `relative` field + `apply_parent_transform` helper (allow(dead_code) consumer-pending).
- P273.5 — consumer L3 activado (page_bbox fallback identity).
- P273.6 — Block save/restore real (3γ.2.γ; cascade ~86 sites).
- P273.7 — Boxed save/restore real (template P273.6 replicado literal).

Pós-P273.7, escopo Decisão 3 cobre {Block, Boxed} canónicos. Stack/Pad/Grid/Group ficam permanentemente como pendência incremental se houver demanda empírica.
