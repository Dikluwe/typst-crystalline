# Passo P273.9 — P-Gradient-Relative-Containers-Extended (expande Decisão 3 de P273.6)

**Tipo**: refino estrutural — extensão da Decisão 3 P273.6 de `{Block, Boxed}` para contentores adicionais.
**Magnitude estimada**: S+ (1-3 arms novos consoante decisões Fase A; pode chegar a M se incluir todos os candidatos).
**Pré-requisitos**: P273.7 fechado (Block+Boxed) + P273.8 fechado (build limpo).
**Cluster**: Visualize / Gradient (encerra refino estrutural ampliado).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; décima anotação cumulativa); ADR-0029 (pureza física L1); ADR-0093 (Pattern 2 anotação cumulativa).

---

## §0 — Contexto

P273.7 fechou Decisão 3 para `{Block, Boxed}`. Outros contentores ficaram registados como pendência:

> P273.7 §7: "Lista de contentores activos: **{Block, Boxed}**. Stack/Pad/Grid cell/FrameItem::Group continuam scope-out per ADR-0054 graded."

P273.9 tenta materializar a expansão. Mas cada contentor candidato tem **forma diferente** — Decisão 3γ.2.γ original (popular `parent_bbox` apenas com `width+height` literais) não é directamente aplicável a contentores sem dimensions literais.

### Inventário dos 4 candidatos

| Contentor | Dimensions literais | Padrão save/restore existente | Forma do bbox |
|---|---|---|---|
| **Stack** | `spacing: Option<Length>`, **sem width/height** | Inexistente | Bbox = conteúdo medido (sem dimensions literais) |
| **Pad** | `padding: Sides<Length>`, **sem width/height** | Inexistente | Bbox = conteúdo + padding (sem dimensions literais) |
| **Grid cell** | `cell_origin_x/y/w` (DEBT-37 P84.6) já no Layouter | **Já existe** — save/restore no arm Grid | Bbox = (cell_origin_x, cell_origin_y, cell_origin_w, cell_available_h) — **directamente disponível** |
| **FrameItem::Group** | `frame.size` post-layout | N/A (Group é resultado, não arm de input) | Bbox = (pos, frame.size) — disponível em emit, não em Layouter |

### Decisão estrutural pré-Fase A

Os 4 candidatos têm naturezas materialmente diferentes:

- **Grid cell** é o caso fácil — padrão DEBT-37 já existe; só falta popular `parent_bbox` em paralelo a `cell_origin_*` no save/restore do arm Grid. Aplica Decisão 3γ.2.γ literal (cell dimensions sempre literais).
- **Stack/Pad** exigem Decisão NOVA — `parent_bbox` populado com bbox medido pós-layout (3γ.2.β em P273.6 §A.3 — opção que P273.6 rejeitou por "layout duplo"). Em Stack/Pad o layout duplo já existe por outras razões (`measure_content_constrained` para Grid).
- **FrameItem::Group** é categorialmente diferente — não é arm de `Content::*` no Layouter; é resultado da renderização que vive no `Frame`. Activar requer L3 que consulte `Group.frame.size` no momento de emit. **Sem alteração Layouter — é trabalho L3 puro**.

P273.9 enfrenta a questão: **escopar a 1-3 candidatos** consoante apetite para magnitude.

### Sub-padrão "Sub-passos consecutivos do mesmo cluster" N=5 emergente pós-P273.9

P273.5 + P273.6 + P273.7 + P273.8 + P273.9 = 5 sub-passos consecutivos cluster Gradient. Limiar formalização N=3-4 já atingido com folga; precedente metodológico estabelecido sem ADR meta formal. Pós-P273.9 mantém-se candidato NÃO reservado.

---

## §1 — Sub-passo P273.9.A — Fase A diagnóstico

**Magnitude**: S documental (~30-45 min — maior que P273.7.A por causa da análise dos 4 candidatos heterogéneos).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-9-diagnostico.md`.

### §A.1 — Inventário Grid cell (caso fácil)

Listar literal em `01_core/src/rules/layout/`:

- Arm `Content::Grid` (provavelmente em `grid.rs` per relatório P246; confirmar empírico via `grep`).
- Save/restore actual de `cell_origin_x/y/w` + `cell_available_h` no arm.
- Forma do construção da bbox: `Rect { x: cell_origin_x, y: cell_origin_y, w: cell_origin_w, h: cell_available_h }`.
- Tipo Pt vs f64 — `cell_origin_*` é `Option<f64>`; `Rect` é `Pt`. Conversão `f64 → Pt` trivial via `Pt(...)`.

### §A.2 — Inventário Stack (caso médio)

Listar literal em `01_core/src/rules/layout/mod.rs::layout_content`:

- Arm `Content::Stack { children, dir, spacing }`.
- Estrutura actual: itera children; aplica spacing entre.
- **Dimensions disponíveis**: nenhuma literal — Stack é content-based.
- **Medição possível**: `measure_content_constrained` poderia ser invocado em todos os children + spacing; bbox = soma. Custo: layout duplo (medição + emit).

### §A.3 — Inventário Pad (caso médio)

Listar literal em `01_core/src/rules/layout/mod.rs::layout_content`:

- Arm `Content::Pad { body, left, top, right, bottom }`.
- Estrutura actual: avança cursor por inset.top/left; layout body; avança por inset.bottom; restaura cursor.
- **Dimensions disponíveis**: insets literais; **largura/altura do body adquiridos só pós-layout**.
- Pad bbox = (cursor antes do arm, body_width + left + right, body_height + top + bottom). Layout duplo necessário para conhecer body_width/height.

### §A.4 — Inventário FrameItem::Group (caso categorialmente diferente)

Listar literal em `03_infra/src/export.rs`:

- Sítios onde `FrameItem::Group` é processado (arm dispatcher; recursão para emit de children).
- Como `frame.size` é conhecido — campo do `Frame` (struct) que vive dentro do `Group`.
- Como bbox seria construído L3-only: `Rect { x: group.pos.x, y: group.pos.y, w: group.frame.size.x, h: group.frame.size.y }`.

**Diferença crítica**: Group **não passa pelo `parent_bbox` do Layouter**. É trabalho L3 puro — `scan_all_gradients` recursivo dentro de Group children consulta o Group.bbox em vez do `parent_bbox_at_emit` (que é populated pelo Layouter para Block/Boxed apenas).

### §A.5 — Decisões a fixar na Fase A

#### Decisão 1 — Escopo do passo

Opções:

- **1α — Apenas Grid cell** (S menor; aplica Decisão 3γ.2.γ directo via DEBT-37 reused; ~25-35 LOC L1).
- **1β — Grid cell + FrameItem::Group** (S+ misto L1+L3; Group é trabalho L3 puro adicional ~30-50 LOC L3).
- **1γ — Grid cell + Stack + Pad** (M — Stack/Pad exigem layout duplo via `measure_content_constrained`; ~80-150 LOC L1; risco regressão tests P262-P273.8 alto).
- **1δ — Todos 4 candidatos** (L; magnitude muito acima de S+; **NÃO recomendado**).
- **1ε — Apenas FrameItem::Group** (S menor diferente; trabalho L3 puro; complementa P273.6 em diferente camada).

**Recomendação spec**: **1α** ou **1β**. Razões:

1. **1α (Grid cell apenas)** materializa o caso mais barato e maior valor empírico — Grid é container muito usado; `parent_bbox` para gradient em cell é semântica vanilla clara; padrão DEBT-37 reused literal (sub-padrão "Pattern DEBT-37 replicado" N=3 → 4 cumulativo).
2. **1β (+ Group)** adiciona caso L3 categorialmente diferente — sub-padrão emergente "L3-only parent_bbox" inaugural (N=1). Valor: gradients aninhados em transforms/groups ganham semântica vanilla observable sem refactor Layouter.
3. **1γ (Stack+Pad)** rejeitada para este passo — layout duplo é decisão arquitectural separada; magnitude M; risco regressão alto. Adiar para `P273.X-bis-stack-pad` se valer a pena empíricamente.

Decisão final na Fase A.

#### Decisão 2 — Semântica bbox para Grid cell

- **2α — Bbox exacto cell (recomendado)**: `Rect { x: cell_origin_x, y: cell_origin_y, w: cell_origin_w, h: cell_available_h }`. Todos 4 disponíveis no Layouter pós-P84.6. Per ADR-0029 puro (são campos `f64` já em L1).
- **2β — Bbox aproximado sem altura**: `Rect { x, y, w, h: page_height }` se `cell_available_h` not always Some. Conservador. Rejeitado em §A.5 se §A.1 confirma que `cell_available_h` é sempre Some no save/restore do arm Grid.

Recomendação spec: **2α**.

#### Decisão 3 — Semântica bbox para FrameItem::Group (se 1β/1ε)

- **3α — Bbox exacto group (recomendado)**: `Rect { x: group.pos.x, y: group.pos.y, w: group.frame.size.x, h: group.frame.size.y }`.
- **3β — Diferir Group**: scope-out per ADR-0054 graded se §A.4 revelar complicação inesperada.

Recomendação spec: **3α** (Group bbox é factual e directo).

### §A.6 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P273.8 | Save/restore Grid cell afecta outros contextos | `parent_bbox` save/restore complementa `cell_origin_*` existente; defaults preservam Self_/None path |
| Regressão DEBT-37 P246 (`cell_origin_*` consumption) | P273.9 toca o mesmo arm Grid | §A.1 confirma save/restore actual `cell_origin_*`; P273.9 só **adiciona** uma linha `self.parent_bbox = Some(Rect { ... })` |
| L3 Group não comunica com Layouter `parent_bbox` (se 1β) | Group é post-Layouter | §A.4 documenta arquitectura L3-only para Group; emit dispatcher consulta `frame.size` directo |
| `cell_available_h` sometimes None | DEBT-37 design | Fallback: se `cell_available_h.is_none()`, não popular `parent_bbox` (3γ.2.γ-cell-strict análogo Block) |
| Sub-padrão "Pattern DEBT-37 replicado" cresce N=3 → 4 cumulativo | Pode disparar limiar formalização ADR meta | Anotação cumulativa preserved; meta-ADR fica candidato pós-P273.9 |

### §A.7 — Critério de aceitação Fase A

- §A.1 cita arm Grid literal (path:linha; confirmação empírica que `cell_origin_x/y/w + cell_available_h` save/restore existe).
- §A.2 / §A.3 / §A.4 inventários consoante Decisão 1 escopo.
- §A.5 Decisões 1+2+3 fixadas com fundamento.
- §A.6 risco "regressão P246 DEBT-37" mitigado.

---

## §2 — Sub-passo P273.9.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental.

**Não criar ADR-0095**. Anotar ADR-0091 — décima anotação consecutiva.

Template:

```
## Anotação cumulativa P273.9 — Containers estendidos (expande Decisão 3 P273.6 para [conjunto fixado Fase A])

**Data**: 2026-05-XX.
**Motivo**: P273.7 fechou Decisão 3 para {Block, Boxed}. P273.9 estende
para [Grid cell / + Group / + outros consoante Decisão 1 Fase A].

**Decisão 1 fixada (escopo)**: [1α / 1β / 1γ / 1ε — preencher pós-Fase A].
**Decisão 2 fixada (Grid bbox)**: [2α / 2β].
**Decisão 3 fixada (Group bbox, se 1β/1ε)**: [3α / 3β].

**Pattern DEBT-37 `cell_origin_*` replicado**: [N=3 preserved se Group
apenas / N=3 → 4 cumulativo se Grid cell].

**Sub-padrão emergente "L3-only parent_bbox"** (se 1β/1ε):
**N=1 inaugural** — Group não passa pelo Layouter; trabalho L3 puro
consultando `frame.size` directo no dispatcher emit. Precedente para
futuros contentores post-layout.

**Defaults preservam P262-P273.8 bit-exact**:
- Grid cell sem `cell_available_h` literal → `parent_bbox` outer preservado.
- Group children em export L3 sem gradient relative=parent → emit literal P273.8.
- Self_/None relative continua a ignorar `parent_bbox_at_emit`.

**`#[allow(dead_code)]` zero** — todos os campos consumed.
```

---

## §3 — Sub-passo P273.9.C — Materialização (testes primeiro)

**Magnitude**: S (1α) / S+ (1β) / M (1γ); spec assume 1α-1β como base.

### Ordem literal

1. Fase A §1 produzida + Decisões fixadas.
2. ADR-0091 anotação §2 escrita.
3. `crystalline-lint --fix-hashes`.
4. **Testes-primeiro**.
5. Código:
   - **Se 1α/1β/1γ inclui Grid**: L1 arm `Content::Grid` (provavelmente em `01_core/src/rules/layout/grid.rs`) ganha save/restore `parent_bbox` em paralelo a `cell_origin_*`.
   - **Se 1β/1ε inclui Group**: L3 dispatcher de `FrameItem::Group` em `03_infra/src/export.rs` consulta `group.frame.size` para construir bbox; passa a `apply_parent_transform` quando gradient interno tem `relative=parent`.
   - **Se 1γ inclui Stack/Pad**: L1 arms ganham `measure_content_constrained` pre-layout + `parent_bbox` populated. **Trabalho substancial — recomendado adiar.**
6. Verificação final.

### Cap LOC (ADR-0094 Pattern 1)

Escopo 1α (Grid apenas):
- **L1 hard cap**: ≤ 40 LOC.
- **L1 soft cap**: ≤ 25 LOC.
- **Tests hard cap**: ≤ 10 novos.
- **Tests soft cap**: ≤ 7.

Escopo 1β (Grid + Group):
- **L1 hard cap**: ≤ 40 LOC.
- **L1 soft cap**: ≤ 25 LOC.
- **L3 hard cap**: ≤ 50 LOC.
- **L3 soft cap**: ≤ 30 LOC.
- **Tests hard cap**: ≤ 15 novos.
- **Tests soft cap**: ≤ 10.

Caps soft calibrados **acima** das estimativas reais — resposta directa ao padrão observado em P274/P273.6/P273.7 de estouros soft sistemáticos.

### Tests propostos (lista mínima — completar pós-Fase A)

**Escopo 1α (Grid cell)**:
1. `p273_9_grid_cell_save_restore_parent_bbox` — entrar em cell Grid guarda bbox cell; sair restaura LIFO.
2. `p273_9_grid_cell_no_cell_available_h_no_parent_bbox` — fallback quando `cell_available_h = None`.
3. `p273_9_nested_grid_cells_lifo` — Grid dentro de Grid — restore LIFO.
4. `p273_9_gradient_relative_parent_inside_grid_cell_uses_cell_bbox` — Linear `relative=parent` aninhado em Grid cell emit usa bbox da cell (não page). **Test E2E observable diff.**
5. `p273_9_radial_relative_parent_inside_grid_cell_mirrors_linear`.
6. `p273_9_gradient_relative_self_inside_grid_cell_unchanged` — bit-exact P272.
7. Regressão DEBT-37: rodar tests P246 — `cell_origin_*` consumption preserved.

**Escopo 1β (+ Group)** adiciona:
8. `p273_9_framegroup_parent_bbox_l3_only` — `FrameItem::Group` dispatcher constrói bbox de `frame.size`; gradient interno consume via `apply_parent_transform`.
9. `p273_9_gradient_relative_parent_inside_group_uses_group_bbox` — E2E observable diff.
10. `p273_9_nested_groups_l3` — Group dentro de Group em export L3.

Regressão integrada: 2620 verdes preserved bit-exact.

### Alterações esperadas no código

#### Escopo 1α (Grid cell)

```rust
// L1 — 01_core/src/rules/layout/grid.rs (ou mod.rs onde arm Grid vive)
// dentro do loop sobre cells

// P273.9 — save/restore parent_bbox paralelo a cell_origin_* (DEBT-37 reused).
let saved_parent_bbox = self.parent_bbox;
if let Some(cell_h) = self.cell_available_h {
    self.parent_bbox = Some(Rect {
        x: Pt(self.cell_origin_x.unwrap_or(self.cursor_x.0)),
        y: Pt(self.cell_origin_y.unwrap_or(self.cursor_y.0)),
        w: Pt(self.cell_origin_w.unwrap_or(self.page_width.0 - self.cursor_x.0)),
        h: Pt(cell_h),
    });
}

self.layout_content(cell_content);

self.parent_bbox = saved_parent_bbox;
```

#### Escopo 1β adiciona (Group L3)

```rust
// L3 — 03_infra/src/export.rs — dispatcher FrameItem::Group

FrameItem::Group { pos, frame, .. } => {
    // P273.9 — Group bbox L3-only (L1 Layouter não passa por aqui).
    let group_bbox = Rect {
        x: Pt(pos.x.0),
        y: Pt(pos.y.0),
        w: Pt(frame.size.x.0),
        h: Pt(frame.size.y.0),
    };
    // recurse para children, com group_bbox como context override
    for item in &frame.items {
        draw_item_local_with_parent_override(item, Some(group_bbox), ...);
    }
}
```

### Verificação final

- Cap LOC respeitado (com folga vs caps soft realistas).
- `cargo build` sem novos warnings.
- `cargo test --workspace` verde — 2620 → 2620 + tests novos.
- `crystalline-lint .` zero violations.
- Hashes L0 — propaga consoante escopo (Grid arm pode propagar `entities/content.md` se cascade necessário; Group dispatcher é L3 puro).
- Tests P262-P273.8 inalterados bit-exact.
- **Tests DEBT-37 P246 `cell_origin_*` consumption** inalterados bit-exact (verificação explícita).
- DEBT saldo 10 preserved.
- E2E observable diff confirmado para cada container coberto pelo escopo.

---

## §4 — Sub-padrões cumulativos pós-P273.9

| Sub-padrão | Pós-P273.8 | Pós-P273.9 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 16 | 17 |
| Reutilização literal helpers cross-passos | 16 | 17 (1α reusa template P273.6/P273.7) |
| Cap LOC hard vs soft explícito | 11 | 12 |
| Aplicação meta-ADR (ADR-0093) | 5 | 6 |
| Aplicação meta-ADR (ADR-0094) | 7 | 8 |
| Diagnóstico imutável | 24 | 25 (20º consumo) |
| **Pattern DEBT-37 `cell_origin_*` replicado** | N=3 | **N=4 (se 1α/β/γ inclui Grid)** — limiar N=3-4 atingido com margem |
| Template-passo replicado literal | 1 | **2** (P273.9 replica template P273.6/P273.7 a Grid cell com adaptação `cell_*` source) |
| Sub-passos consecutivos do mesmo cluster | 4 | **5 cumulativo emergente** |
| **L3-only parent_bbox** (se 1β/1ε) | 0 | **N=1 inaugural** — Group L3 puro |

Sub-padrão "Pattern DEBT-37 replicado" N=4 confirma consolidação além do limiar N=3-4. Promoção a ADR meta candidato pós-passo administrativo XS NÃO reservado.

---

## §5 — Limitações conscientes P273.9

- Escopo determinado pela Decisão 1 Fase A. **Contentores fora do escopo** (e.g. Stack/Pad se Decisão for 1α/1β) continuam scope-out per ADR-0054 graded.
- Decisão 3γ.2.γ-grid-strict — `parent_bbox` populated apenas se `cell_available_h.is_some()`. Cells sem altura conhecida cae no fallback page_bbox (P273.5).
- Group bbox L3-only (se 1β/1ε) é factual mas usa `frame.size` (geometric bbox), não bbox lógico — refino futuro se distinção empírica aparecer.
- Stack/Pad refino com layout duplo continua diferido per ADR-0054 graded — pendência `P273.X-bis-stack-pad` se valer a pena empíricamente.
- Dedup bbox-aware (relatório P273.6 §9) continua aberto.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico.
3. Utilizador upload do diagnóstico (com Decisões 1+2+3 fixadas).
4. Claude web valida critério §A.7 + revê Decisões.
5. Utilizador executa P273.9.B + P273.9.C → relatório.
6. Utilizador upload do relatório.
7. Claude web analisa + propõe próximo passo.

---

## §7 — Pendências preservadas pós-P273.9

Inalteradas vs P273.8 (nível cluster):

- P-Gradient-CMYK-ICC (S-M).
- ADR-0055bis variant-aware fonts (M).
- P-Footnote-N (M).
- DEBT-33 Bézier bbox (S+M).
- Stroke\<Length\> / Curve / Polygon (S+M).
- Tiling activação.
- Outro cluster — saída Visualize/Gradient.

Pendências específicas pós-P273.9 (incremental per ADR-0054 graded):
- **P273.X-bis-stack-pad** — Stack+Pad save/restore via layout duplo (se Decisão 1 = 1α/1β e Stack/Pad ficarem para depois).
- **P273.X-bis-bbox-medido** — refino 3γ.2.β para containers sem dimensions literais.
- **P273.X-bis2 — bbox.y topo-exacto inline** (P273.7 herdada).
- **Dedup bbox-aware** (P273.6 §9 herdada).

**Pós-P273.9 cluster Gradient refino estrutural extensivamente encerrado** — Block + Boxed + Grid cell (+ Group consoante Decisão 1) cobertos.

---

## §8 — Critério de fecho do passo

P273.9 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.7 cumprido.
- ADR-0091 anotada (décima anotação consecutiva).
- L1 (e/ou L3 se 1β/1ε) alterados dentro do cap LOC.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P273.8 inalterados bit-exact.
- **Tests DEBT-37 P246 cell_origin_* consumption inalterados** (verificação explícita).
- DEBT saldo 10 preserved.
- Test E2E observable diff confirmado para cada container coberto.
- Sub-padrões §4 atualizados.

---

## §9 — Numeração — nota

Spec usa **P273.9** continuando o pattern decimal P273.5/P273.6/P273.7/P273.8. Não há colisão com `P273.X-bis-*` (literais com sufixo word não dígito). Alternativa P275 viável se preferires numeração consecutiva sem decimal — decidir antes da Fase A.

Cadeia P273-P273.9 documentada:
- P273 — `relative` field + helpers (`#[allow(dead_code)]` consumer-pending).
- P273.5 — Callsite L3 activado (page_bbox fallback).
- P273.6 — Block save/restore real (cascade ~86 sites).
- P273.7 — Boxed save/restore (template replicado).
- P273.8 — Cleanup XS 4 warnings.
- **P273.9 — Containers estendidos** (Grid cell + opcional Group).
