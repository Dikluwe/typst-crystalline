# Passo P273.13 — Fix `draw_item_local` Group gradient (fecha trabalho P273.10/P273.12 incompleto)

**Tipo**: refino estrutural — fecho de pendência específica registada P263 §8 #3 + exposta em P273.12 §9 quarto bullet.
**Magnitude estimada**: S (~40-70 LOC L3; 0 L1; 4-8 testes).
**Pré-requisitos**: P273.12 fechado (DedupKey + dedup bbox-aware; scan_all_gradients + pattern_resources_for_page consume DedupKey).
**Cluster**: Visualize / Gradient (quarto sub-passo na sequência terminar cluster; inserido por priorização B).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; décima terceira anotação cumulativa); ADR-0029 (pureza física L1 preserved — passo L3-only); ADR-0093 (Pattern 2 anotação cumulativa).

---

## §0 — Contexto

Pendência cumulativa em três sítios:

1. **P263 §8 trabalho futuro #3** (original 2026-05-16): "`draw_item_local` Gradient support pendência — função recursiva para shapes em groups com `cm` transformations não recebe `pat_ptr_to_idx`/`pat_refs` no escopo recursivo. Fallback `first_stop_color` para esse caso específico."
2. **P273.10** corrigiu o caminho de *registo* (scan + resources) mas não o caminho de *emit real* (draw_item_local).
3. **P273.12 §9 quarto bullet** explicitou: "`draw_item_local` Group recursion path (linhas 2347+) usa solid fallback `s.paint.to_color()` para gradient strokes — NÃO consume pattern dict. Limitação pré-existente P273.10 preserved; gradients efectivos dentro de Groups via draw_item_local renderizam como solid color."

### O caminho factual quebrado

```
build_page_stream_*  →  draw_item  (top-level shapes; consome pat_ptr_to_idx via emit_stroke_paint)
                    →  FrameItem::Group  →  draw_item_local (recursive)
                                              →  Shape em Group: emit_stroke_paint chamada COM fallback
                                                                  (sem pat_ptr_to_idx/pat_refs no escopo)
                                                                  → solid color fallback
```

Resultado: gradients dentro de Groups são *registados* no PDF `/Pattern << >>` (P273.10+P273.12) mas o *render real* via `draw_item_local` cai para solid color. PDF tem patterns inúteis declarados; shapes Group renderizam como flat colors.

### Solução arquitectural

Propagar `pat_ptr_to_idx`, `pat_refs` (e `parent_bbox_override` para construir DedupKey lookup) através de `draw_item_local` recursivo. Mesmo padrão que P273.10 aplicou ao `scan_all_gradients.walk` — parameter threading explícito.

### Diferença vs P273.10/P273.12

- P273.10 fez parameter threading em `scan_all_gradients.walk` (caminho registo).
- P273.12 expandiu chave para DedupKey (caminho registo).
- **P273.13 faz parameter threading em `draw_item_local` (caminho emit real)** — fecha o lado correspondente.

Sub-padrão "L3-only parent_bbox" cresce N=1 → N=2 cumulativo (P273.10 inaugural + P273.13 reaplicação).

---

## §1 — Sub-passo P273.13.A — Fase A diagnóstico

**Magnitude**: S documental (~20-30 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-13-diagnostico.md`.

### §A.1 — Inventário literal `draw_item_local`

Listar em `03_infra/src/export.rs`:

- **Função `draw_item_local`** — provavelmente declarada perto da linha 2347 (per P273.12 §9). Confirmar path:linha.
- **Signature actual** — quais parâmetros recebe; em particular se já tem acesso a `pat_ptr_to_idx`/`pat_refs` (provavelmente não, dado P263 §8 #3).
- **Callsites de `draw_item_local`** — onde é chamada (provavelmente dentro de `draw_item` arm Group e recursivamente dentro do próprio `draw_item_local` arm Group).
- **Sítio do fallback `s.paint.to_color()`** — onde gradients caem para solid. Confirmar linha exacta.

### §A.2 — Inventário do callsite de `emit_stroke_paint` em `draw_item_local`

Pós-P273.12, `emit_stroke_paint` signature é:

```rust
fn emit_stroke_paint(
    ops, paint, thickness,
    effective_bbox: Option<Rect>,        // P273.12 — para construir DedupKey
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,
    pat_refs: &[PatternRef],
)
```

Em `draw_item_local`, o callsite actual provavelmente:
- Não passa `pat_ptr_to_idx`/`pat_refs` (porque não os tem no escopo).
- Cai num arm separado que faz fallback solid.

Inventário: verificar se há duas funções emit distintas (`emit_stroke_paint` vs `emit_stroke_paint_solid_only`) ou se é o mesmo `emit_stroke_paint` chamado com map vazio.

### §A.3 — Decisão 1 — Mecanismo de propagação

Opções:

- **1α — Parameter threading explícito**: `draw_item_local` ganha 3 params novos (`pat_ptr_to_idx`, `pat_refs`, `parent_bbox_override`). Callers em `draw_item` Group arm passam o que têm. Mesmo padrão P273.10 `scan_all_gradients.walk`.
- **1β — Struct context**: criar `EmitContext { pat_ptr_to_idx, pat_refs, parent_bbox_override }` passado em vez de 3 params. Menos signature noise; mais escalável.
- **1γ — Field no Layouter**: rejeitada — L3 é export, não Layouter. Acoplamento errado.

Recomendação spec: **1α** (parameter threading). Razões:

1. Coerência com P273.10 mesmo mecanismo.
2. Sem custo de criar struct nova só para 3 params.
3. Refactor mínimo — apenas signature de `draw_item_local` muda.

Decisão final na Fase A consoante quantos call sites tem `draw_item_local` (se cascade fica grande, 1β justifica-se).

### §A.4 — Decisão 2 — Bbox effective para Group children

Question: quando `draw_item_local` desce no recursive walk para children de um Group, qual é o `parent_bbox_override` que passa?

- **2α — Group bbox próprio** (paridade P273.10): construir `group_bbox` no arm Group em `draw_item_local` (igual a P273.10) e passar como override aos children.
- **2β — Propagar outer override**: passar literal o `parent_bbox_override` que `draw_item_local` recebeu (não construir Group bbox próprio).

Recomendação spec: **2α** — paridade total com P273.10. Group bbox em `draw_item_local` deve ser literal-equivalente ao Group bbox calculado em `scan_all_gradients.walk` para que a DedupKey lookup encontre o pattern correcto registado.

**Crítico**: se 2α não fizer paridade exacta com P273.10, o lookup em `pat_ptr_to_idx` falha (DedupKey constructed inconsistente) → fallback solid persiste. **Fase A deve verificar empíricamente** que a construção é literal idêntica.

### §A.5 — Decisão 3 — Y-inversion considerations

`draw_item_local` é chamada para shapes em groups com `cm` (current transformation matrix) — coordenadas locais. PDF Y-inversion não acontece dentro de `draw_item_local` (já foi aplicada no parent).

Question: o `parent_bbox` construído para Group children deve ser em coords cristalino (P273.10 §A.5 Decisão 2α) ou em coords locais do Group?

- **3α — Coords cristalino** (paridade P273.10): bbox em coords cristalino (sem Y-inversion). Consistente com `scan_all_gradients.walk` Group bbox.
- **3β — Coords locais Group**: bbox relativa ao Group. Tecnicamente diferente — não compatível com DedupKey construída pelo scan.

Recomendação spec: **3α** — paridade total. Decisão crítica para DedupKey lookup funcionar.

### §A.6 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P273.12 | Mudança signature `draw_item_local` | Defaults preservam: Shapes top-level (não dentro de Group) continuam a chamar `emit_stroke_paint` directo com pat_ptr_to_idx; Shapes em Group via `draw_item_local` agora também consomem |
| DedupKey lookup falha | Group bbox em `draw_item_local` ≠ Group bbox em scan | Decisão 2α + 3α paridade literal; tests verificam que pattern idx encontrado é igual |
| Tests P273.12 quebram | Patterns registados mas não usados em P273.12 — agora usados | Tests P273.12 não verificavam render em Group; mantêm-se válidos. **MAS** alguns tests podem inadvertidamente depender do fallback solid color — verificar |
| Cascade signature `draw_item_local` callers | 1-2 callers em `draw_item` Group arm | Cascade pequeno; controlado |
| `cm` transformations interferem | Coords locais vs cristalino | Decisão 3α coords cristalino — bbox preserved em coords do parent frame |

### §A.7 — Decisões a fixar na Fase A

1. **Decisão 1** (mecanismo propagação): 1α / 1β / 1γ. Recomendação: 1α.
2. **Decisão 2** (Group bbox source): 2α / 2β. Recomendação: 2α paridade P273.10.
3. **Decisão 3** (coords cristalino vs local): 3α / 3β. Recomendação: 3α paridade total.

### §A.8 — Critério de aceitação Fase A

- §A.1 cita `draw_item_local` literal (path:linha) + callsites.
- §A.2 confirma `emit_stroke_paint` signature pós-P273.12 + sítio actual fallback.
- §A.4 + §A.5 fixam paridade literal P273.10 para Group bbox construction.
- §A.6 risco "DedupKey lookup falha" mitigado pela paridade.
- §A.7 Decisões 1+2+3 fixadas.

---

## §2 — Sub-passo P273.13.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental.

**Não criar ADR-0095**. Anotar ADR-0091 — décima terceira anotação consecutiva.

Template:

```
## Anotação cumulativa P273.13 — Fix draw_item_local Group gradient (caminho emit real)

**Data**: 2026-05-XX.
**Motivo**: P273.10 corrigiu caminho de *registo* (scan + resources)
para Groups. P273.12 expandiu chave para DedupKey. **Caminho de emit
real** (`draw_item_local` recursive para Group children) continuou
a usar fallback solid color — patterns registados mas não consumidos.
Pendência P263 §8 #3 + P273.12 §9 quarto bullet fechadas.

**Decisão 1 fixada (propagação)**: [1α/1β — preencher pós-Fase A].
**Decisão 2 fixada (Group bbox source)**: [2α/2β — preencher].
**Decisão 3 fixada (coords)**: [3α/3β — preencher].

**Sub-padrão "L3-only parent_bbox" N=1 → N=2 cumulativo** — P273.10
inaugural (scan_all_gradients.walk) + P273.13 reaplicação
(draw_item_local). Padrão consolidado: parameter threading para
`(pat_ptr_to_idx, pat_refs, parent_bbox_override)` em walkers
recursivos L3.

**Defaults preservam P262-P273.12 bit-exact**:
- Shapes top-level (não dentro de Group) → caminho directo
  `emit_stroke_paint` (sem mudança).
- Shapes dentro de Group sem gradient → preserved literal.
- Shapes dentro de Group com Self_/None relative → DedupKey
  `{arc_ptr, None}` lookup encontra pattern registado.
- Shapes dentro de Group com gradient relative=parent → DedupKey
  `{arc_ptr, Some(rect_to_key(group_bbox))}` lookup encontra pattern
  (paridade P273.10/P273.12).

**Patterns registados mas não usados pós-P273.12** — corrigido
em P273.13. PDF size pre-existing inflation (P273.12) agora produz
observable render real.
```

---

## §3 — Sub-passo P273.13.C — Materialização (testes primeiro)

**Magnitude**: S (~40-70 LOC L3; 0 L1; 4-8 testes).

### Ordem literal

1. Fase A §1 produzida + Decisões fixadas.
2. ADR-0091 anotação §2 escrita.
3. `crystalline-lint --fix-hashes`.
4. **Testes-primeiro**.
5. Código:
   - L3 — `draw_item_local` ganha params `pat_ptr_to_idx`, `pat_refs`, `parent_bbox_override` (Decisão 1α).
   - L3 — arm Shape dentro de `draw_item_local` constrói `effective_bbox = parent_bbox_at_emit.or(parent_bbox_override)` (Inner-wins paridade P273.10) e passa a `emit_stroke_paint`.
   - L3 — arm Group dentro de `draw_item_local` constrói `group_bbox` (literal-equivalente a `scan_all_gradients.walk` arm Group; Decisão 2α + 3α) e passa como override aos children.
   - L3 — callsites de `draw_item_local` em `draw_item` (top-level Group arm) passam contexto recebido.
6. Verificação final.

### Cap LOC (ADR-0094 Pattern 1; caps realistas pós-experiência cluster)

- **L3 hard cap**: ≤ 70 LOC.
- **L3 soft cap**: ≤ 50 LOC.
- **L1 hard cap**: 0 LOC (sem touch Layouter).
- **Tests hard cap**: ≤ 10.
- **Tests soft cap**: ≤ 6.

### Tests propostos (lista mínima — completar pós-Fase A)

1. `p273_13_gradient_inside_group_renders_real_pattern` — gradient Linear `relative=self` dentro de Group emit usa pattern dict (NÃO solid fallback); E2E confirma PDF bytes contêm `/Pattern` reference (`/Pi cs /Pi scn` ou equivalente) onde antes era `rgb` solid.
2. `p273_13_gradient_relative_parent_inside_group_uses_group_bbox` — gradient Linear `relative=parent` dentro de Group emit usa pattern com bbox de Group (paridade P273.10 + P273.12).
3. `p273_13_radial_inside_group_mirrors_linear` — paridade Radial.
4. `p273_13_nested_groups_inner_group_bbox_wins` — Group dentro de Group; innermost Group bbox usado para gradient `relative=parent` (paridade Inner-wins).
5. `p273_13_pattern_registered_p273_10_now_consumed` — verifica que patterns registados em scan (P273.10) são efectivamente consumidos no emit (P273.13); count de patterns usados == count registados (no contexto de teste).
6. `p273_13_self_relative_inside_group_uses_arc_only_dedup` — paridade dedup Self_/None preserved bit-exact.
7. Regressão integrada: 2638 verdes preserved bit-exact + tests novos.

### Alterações esperadas no código

```rust
// L3 — 03_infra/src/export.rs

// draw_item_local signature actualizado
fn draw_item_local(
    ops: &mut String,
    item: &FrameItem,
    parent_bbox_override: Option<Rect>,  // P273.13 novo
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,  // P273.13 novo
    pat_refs: &[PatternRef],  // P273.13 novo
    // ... outros params existentes ...
) {
    match item {
        FrameItem::Shape { stroke, parent_bbox_at_emit, .. } => {
            // P273.13 — Inner wins paridade P273.10
            let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
            if let Some(s) = stroke {
                emit_stroke_paint(
                    ops, &s.paint, s.thickness,
                    effective_bbox,  // P273.12 — DedupKey lookup
                    pat_ptr_to_idx,
                    pat_refs,
                );
            }
            // ... resto preserved ...
        }
        FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
            // P273.13 — Group bbox literal-equivalente scan_all_gradients.walk
            // (Decisão 2α + 3α paridade total)
            let group_bbox = Rect {
                x: Pt(pos.x.0),
                y: Pt(pos.y.0),
                w: Pt(*inner_width),
                h: Pt(*inner_height),
            };
            for child in items {
                draw_item_local(
                    ops, child,
                    Some(group_bbox),  // P273.13 — propaga
                    pat_ptr_to_idx,
                    pat_refs,
                    // ...
                );
            }
        }
        _ => { /* preserved */ }
    }
}

// Callers em draw_item (top-level Group arm) actualizados:
// build_page_stream_* já tem pat_ptr_to_idx + pat_refs no escopo;
// passa-os + None (sem outer override no top-level Group).
```

### Verificação final

- Cap LOC respeitado.
- `cargo build` sem novos warnings.
- `cargo test --workspace` verde — 2638 → 2638 + tests novos.
- `crystalline-lint .` zero violations.
- Hashes L0 — `entities/gradient.md` propaga (anotação P273.13).
- Tests P262-P273.12 inalterados bit-exact (defaults preserved).
- DEBT saldo 10 preserved.
- **Pendência P263 §8 #3 fechada** — `draw_item_local` consume pattern dict.
- **Pendência P273.12 §9 quarto bullet fechada** — gradients dentro de Group renderizam real.
- Test E2E confirma observable diff: gradient dentro de Group renderiza com pattern (não solid fallback).

---

## §4 — Sub-padrões cumulativos pós-P273.13

| Sub-padrão | Pós-P273.12 | Pós-P273.13 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 19 | 20 |
| Reutilização literal helpers cross-passos | 17 | 17 (preserved) |
| Cap LOC hard vs soft explícito | 15 | 16 |
| Aplicação meta-ADR (ADR-0093) | 8 | 9 |
| Aplicação meta-ADR (ADR-0094) | 11 | 12 |
| Pattern DEBT-37 replicado | N=4 (preserved) | N=4 (preserved) |
| Template-passo replicado literal | N=2 (preserved) | N=2 (preserved) |
| Sub-passos consecutivos do mesmo cluster | N=8 emergente | **N=9 cumulativo emergente** |
| Layout duplo arquitectural aceite | N=1 (preserved) | N=1 (preserved) |
| **L3-only parent_bbox** | N=1 inaugural | **N=2 cumulativo** (P273.10 + P273.13) |
| Dedup Arc::as_ptr resources | N=3 (limiar atingido) | N=3 (preserved; reused via DedupKey lookup) |
| Bug arquitectural intencional corrigido | N=1 (preserved) | N=1 (preserved — P273.13 é fix tactical de pendência específica, não arquitectural) |
| Bug latent corrigido em scope creep | N=1 ou 2 | N=1 ou 2 (preserved) |
| Extract helper de replicação inline | N=1 (preserved) | N=1 (preserved) |
| Diagnóstico imutável | 28 | 29 (24º consumo) |

Sub-padrão "L3-only parent_bbox" cresce N=1 → **N=2 cumulativo** — P273.10 inaugural (scan_all_gradients.walk) + P273.13 reaplicação (draw_item_local). Padrão consolidado mas ainda longe do limiar formalização N=3-4.

### Discussão metodológica

P273.13 **não** inaugura sub-padrão novo. É reaplicação de mecanismo P273.10 + fecho de pendência específica registada P263 §8 + P273.12 §9. Trabalho mecânico bem identificado.

Distingue de "Bug arquitectural intencional corrigido" P273.12 onde havia decisão arquitectural deliberada de adiar. P273.13 não tinha decisão arquitectural deliberada — era reaplicação que ficou por fazer (pendência específica documentada). Aceitação: cluster acumula pendências menores que vão sendo fechadas incrementalmente.

---

## §5 — Limitações conscientes P273.13

- Patterns registados mas não usados pós-P273.12 — corrigido em P273.13. PDF size inflation P273.12 agora produz observable render real (esperado).
- `draw_item` (top-level) e `draw_item_local` (Group children) mantêm-se funções separadas — refactor para unificar fica fora de escopo.
- Group bbox construction literal-equivalente entre scan e draw_item_local — sensible a divergência se um dos dois sítios for refactorado independentemente no futuro. **Recomendação**: helper partilhado `group_bbox_from_frame_item` candidato XS futuro (sub-padrão "Extract helper de replicação inline" P273.11 precedente N=2 se materializado).
- Cluster específico — não toca outros sítios análogos.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.8 + revê Decisões 1+2+3.
5. Utilizador executa P273.13.B + P273.13.C → relatório.
6. Utilizador upload do relatório.
7. Claude web analisa + propõe **P273.14** (próximo da sequência: CMYK-ICC krilla paridade — **verificar API krilla na Fase A**).

---

## §7 — Pendências preservadas pós-P273.13

Inalteradas vs P273.12 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M; **VERIFICAR Fase A se krilla API existe**).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Sequência para fechar cluster Gradient (renumeração pós-inserção P273.13):

- ✓ P273.10 — Group L3-only (fechado).
- ✓ P273.11 — Extract Stack helper (fechado).
- ✓ P273.12 — Dedup bbox-aware (fechado).
- **P273.13** — Fix draw_item_local Group gradient (este passo; INSERIDO).
- **P273.14** — CMYK-ICC krilla paridade (S-M; verificar API; era P273.13 pré-inserção).
- **P273.15** — Bbox medido pós-layout (M; era P273.14).
- **P273.16** — Bbox.y topo-exacto inline (M-L; era P273.15; **BLOQUEADO** por DEBT-56).

Pendência específica nova candidata XS:
- **P273.X-bis-helper-group-bbox** — extract helper `group_bbox_from_frame_item` partilhado entre `scan_all_gradients.walk` + `pattern_resources_for_page.walk` + `draw_item_local`. Sub-padrão "Extract helper de replicação inline" precedente N=2 se materializado. NÃO reservado.

---

## §8 — Critério de fecho do passo

P273.13 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.8 cumprido.
- ADR-0091 anotada (décima terceira anotação consecutiva).
- L3 alterado dentro do cap LOC; L1 intocado.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P273.12 inalterados bit-exact.
- DEBT saldo 10 preserved.
- **Pendência P263 §8 #3 fechada** — verificar literal que `draw_item_local` consume `pat_ptr_to_idx` real.
- **Pendência P273.12 §9 quarto bullet fechada** — verificar literal observable diff: gradient dentro de Group renderiza com pattern (não solid).
- Test E2E observable diff confirma.
- Sub-padrões §4 atualizados.

---

## §9 — Numeração

Spec usa **P273.13** continuando a sequência decimal. **Inserido** na sequência "terminar cluster Gradient" por priorização B na decisão anterior.

Renumeração da sequência prevista (pós-inserção):
- ✓ P273.10 — Group L3-only (fechado).
- ✓ P273.11 — Extract Stack helper (fechado).
- ✓ P273.12 — Dedup bbox-aware (fechado).
- **P273.13** — Fix draw_item_local (este passo).
- P273.14 — CMYK-ICC krilla (era P273.13).
- P273.15 — Bbox medido pós-layout (era P273.14).
- P273.16 — Bbox.y topo-exacto inline (era P273.15; bloqueado DEBT-56).

**Predição revisada**: cluster termina entre P273.14 e P273.16 consoante disponibilidade da krilla API.

Trajectória meta-metodológica: cluster Gradient agora atravessou 9 sub-passos consecutivos (P273.5-P273.13). Padrão honesto: sub-passos descobrem trabalho residual; sequência cresce organicamente quando pendências são expostas. Disciplina preservada: cada inserção é documentada (P273.10 §8 expôs P273.X-bis-group → tornou-se nada; P273.12 §9 expôs draw_item_local → tornou-se P273.13 inserido).
