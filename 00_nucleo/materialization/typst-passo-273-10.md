# Passo P273.10 — P-Gradient-Group-L3-Only (sub-padrão "L3-only parent_bbox" inaugural)

**Tipo**: refino estrutural — estende cobertura `parent_bbox` para `FrameItem::Group` via mecanismo L3 puro.
**Magnitude estimada**: S (~30-60 LOC L3; 0 L1; ~6-10 testes).
**Pré-requisitos**: P273.9 fechado (Block + Boxed + Grid cell + Stack + Pad cobertos).
**Cluster**: Visualize / Gradient (sequência terminar cluster — primeiro de até 6 sub-passos).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; décima primeira anotação cumulativa); ADR-0029 (pureza física L1 preserved — passo L3-only); ADR-0093 (Pattern 2 anotação cumulativa).

---

## §0 — Contexto

P273.9 fechou Decisão 3 para containers no Layouter (Block + Boxed + Grid cell + Stack + Pad). **FrameItem::Group** é categorialmente diferente:

- Não é arm de `Content::*` no Layouter.
- É resultado de renderização — vive como `FrameItem::Group { pos, frame, .. }` num parent frame.
- O `frame.size` é conhecido **em emit-time L3**, não em Layouter-time L1.

P273.9 §A.4 inventário identificou esta diferença categorial; Decisão 1ε (Apenas Group L3-only) ficou como alternativa não-escolhida (1γ venceu). P273.9 §8 registou `P273.X-bis-group` como pendência específica.

P273.10 materializa essa pendência.

### Por que L3-only

Group **não passa pelo `Layouter.parent_bbox`** porque é construído pós-layout (sub-frame embebido). Para gradient `relative=parent` dentro de Group, a bbox parent deve ser construída em emit-time consultando `group.frame.size`.

Trabalho L3 puro: dispatcher de `FrameItem::Group` no `export.rs` constrói `Rect` a partir de `pos + frame.size` e fornece-o ao processamento de children — em particular ao `scan_all_gradients` recursivo que captura `parent_bbox_at_emit` para cada `FrameItem::Shape` interna.

### Sub-padrão inaugural "L3-only parent_bbox" N=1

Inaugura o sub-padrão emergente: contentores post-layout cuja bbox é conhecida apenas em L3 emit-time usam **L3 dispatcher override** em vez de L1 Layouter save/restore. Precedente metodológico para futuros casos análogos.

Distingue-se de:
- **Pattern DEBT-37** (N=4 P273.9) — Layouter save/restore para containers L1.
- **Layout duplo arquitectural aceite** (N=1 P273.9) — `measure_content_constrained` em L1 para containers sem dimensions literais.
- **L3-only parent_bbox** (N=1 P273.10 inaugural) — emit-time override para containers post-layout.

---

## §1 — Sub-passo P273.10.A — Fase A diagnóstico

**Magnitude**: S documental (~25-35 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-10-diagnostico.md`.

### §A.1 — Inventário do dispatcher Group em export.rs

Listar literal em `03_infra/src/export.rs`:

- **Sítio(s) onde `FrameItem::Group` é dispatchado** — provavelmente em `draw_item` (page-level) e `draw_item_local` (recursivo dentro de Group). Confirmar empírico via `grep "FrameItem::Group"`.
- **Forma do dispatch** — recursão sobre `group.frame.items`? Iteração linear? Transform aplicada (translate via pos)?
- **`scan_all_gradients` recursão** — confirmar se já desce dentro de Groups; se sim, como propaga `parent_bbox` actualmente (provavelmente `None`).

### §A.2 — Inventário do tipo `FrameItem::Group` em L1

Listar literal em `01_core/src/entities/layout_types.rs`:

- Variante `FrameItem::Group { pos, frame, .. }`.
- Estrutura `Frame { size, items, .. }`.
- Tipo `size` — `Size { x, y }` ou `Point`? Confirma.

### §A.3 — Bug latent verificação

P273.9 §2.4 corrigiu `translate_frame_item` em `helpers.rs` que descartava `parent_bbox_at_emit`. P273.10 deve **verificar empíricamente** se há outros sites com bug análogo, especificamente em código que processa Groups + Shapes:

- `scan_all_gradients` em export.rs — destructure Shape; já consume `parent_bbox_at_emit` per P273.6 §2.5.
- Sites de recursão Group — `draw_item_local` recursivo dentro de `group.frame.items`. Se este site reconstrói FrameItem::Shape para coords locais, pode descartar campo.

Decisão Fase A se for descoberto bug análogo: abrir DEBT separado ou incluir fix tactical neste passo (precedente P273.9 §2.4).

### §A.4 — Decisão 1 — Mecanismo de override L3

Opções:

- **1α — Parameter threading explícito**: funções `draw_item_local`/`scan_all_gradients` recursivas ganham parâmetro `parent_bbox_override: Option<Rect>`. Caller (dispatcher Group) passa `Some(group_bbox)`; default `None` propaga.
- **1β — Re-stamping**: dispatcher Group itera `group.frame.items`, para cada FrameItem::Shape sem `parent_bbox_at_emit` populado, popula com `group_bbox` antes de descer. Modifica items in-place (precisa de `Vec` mutável; pode quebrar `&Frame`).
- **1γ — Híbrido**: scan_all_gradients ganha parâmetro; dispatcher Group passa override. Re-stamping apenas em casos onde scan já passou.

Recomendação spec: **1α** (parameter threading). Razões:

1. Sem mutação — preserva `&Frame` imutável.
2. Compose-na-fly — Group dentro de Group propaga override correctamente via LIFO no parameter.
3. Sub-padrão "L3-only parent_bbox" via signature explícita — auto-documentado.

### §A.5 — Decisão 2 — Semântica Group bbox

Opções:

- **2α — bbox exacto frame**: `Rect { x: pos.x, y: pos.y, w: frame.size.x, h: frame.size.y }`. Geometric bbox.
- **2β — bbox lógico**: bbox que reflicta o "intent" do Group (e.g. bbox do conteúdo medido). Mais subtil; pode divergir de geometric bbox em casos com padding embedded.
- **2γ — bbox post-Y-inversion**: aplicar Y-inversion no `pos.y` para coords PDF antes de construir Rect. Tecnicamente correcto para PDF coords.

Recomendação spec: **2α** (geometric exact bbox em coords cristalino — sem Y-inversion; matched contra coords locais do gradient interno que também são cristalino-relativas). Razões:

1. Factual e directo — `frame.size` é o que existe; "bbox lógico" exigiria refino arquitectural maior.
2. Consistente com `parent_bbox_at_emit` Layouter — todos os bbox em coords cristalino, Y-inversion é responsabilidade exclusiva do PDF emit final.

### §A.6 — Decisão 3 — Override precedence

Quando `FrameItem::Shape.parent_bbox_at_emit` JÁ está populado (Block/Boxed/Grid/Stack/Pad P273.9) E o Shape está dentro de um Group:

- **3α — Inner wins** — Shape's próprio `parent_bbox_at_emit` (do Layouter L1) prevalece. Override Group apenas se Shape's campo é `None`.
- **3β — Outer wins** — Group override sobrescreve sempre. Shape's próprio campo descartado.
- **3γ — Closest container wins** — Inner (Shape's próprio) é mais "perto"; outer (Group) é mais "longe". Inner sempre vence quando ambos presentes.

Recomendação spec: **3α/3γ (equivalentes)** — Inner wins. Razões:

1. Paridade vanilla — `relative=parent` em vanilla resolve ao contentor mais próximo.
2. Layouter L1 conhece o contentor canónico (Block/Boxed/Grid/Stack/Pad); Group é wrapper estrutural pós-layout que NÃO redefine "parent" semanticamente.
3. Defaults preservam P273.9 bit-exact — Shapes que tinham `parent_bbox_at_emit` populado continuam a ter.

### §A.7 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P273.9 | Override Group altera path quando Shape's campo é None | Decisão 3α: Inner wins; Shapes com campo populated unaffected; Shapes sem campo continuam a usar fallback page (= identity); mudança observable apenas quando Shape é gradient relative=parent E parent não-Layouter (caso novo) |
| Bug análogo P273.9 §2.4 latent | Recursão Group pode reconstruir Shape descartando campo | §A.3 confirma empírico; fix tactical incluído se necessário |
| Param signature cascade | `scan_all_gradients` ganha param afecta callers | scan_all_gradients é internal; cascade controlado; LOC dentro do cap |
| L3 cap LOC estourado | Threading param + override logic | Cap hard 60 / soft 40 — estimativa real ~30-50 LOC dentro de soft |
| Sub-padrão N=1 inaugural sem precedente | Risco escolha errada de mecanismo | §A.4 documenta 3 opções; recomendação fundamentada |

### §A.8 — Critério de aceitação Fase A

- §A.1 cita dispatcher Group literal (path:linha).
- §A.2 confirma `Frame.size` tipo (Size vs Point).
- §A.3 confirmação empírica de bugs análogos (zero ou >0 com decisão de fix tactical).
- §A.4 Decisão 1 fixada (α/β/γ).
- §A.5 Decisão 2 fixada (α/β/γ).
- §A.6 Decisão 3 fixada (α/β/γ).

---

## §2 — Sub-passo P273.10.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental.

**Não criar ADR-0095**. Anotar ADR-0091 — décima primeira anotação consecutiva.

Template:

```
## Anotação cumulativa P273.10 — Group L3-only parent_bbox (sub-padrão "L3-only" inaugural)

**Data**: 2026-05-XX.
**Motivo**: P273.9 escopou save/restore L1 a 5 containers Layouter
(Block + Boxed + Grid cell + Stack + Pad). FrameItem::Group é
categorialmente diferente — pós-layout; bbox conhecida apenas em
L3 emit-time. P273.10 fecha pendência P273.9 §8.

**Decisão 1 fixada (mecanismo override)**: [α/β/γ — preencher pós-Fase A].
**Decisão 2 fixada (Group bbox)**: [α/β/γ].
**Decisão 3 fixada (override precedence)**: [α/β/γ — Inner wins
esperado].

**Sub-padrão emergente "L3-only parent_bbox"**: **N=1 inaugural**.
Distingue de Pattern DEBT-37 (N=4 P273.9; L1 Layouter save/restore)
e Layout duplo arquitectural aceite (N=1 P273.9; measure_content_constrained
em L1).

**Defaults preservam P262-P273.9 bit-exact**:
- Shapes com `parent_bbox_at_emit` populated (P273.9 5 containers)
  unaffected — Inner wins.
- Shapes sem campo populated continuam a usar fallback page (P273.5)
  exceto quando dentro de Group.
- Self_/None relative ignora override.

**`#[allow(dead_code)]` zero** — todos os campos consumed.
```

---

## §3 — Sub-passo P273.10.C — Materialização (testes primeiro)

**Magnitude**: S (~30-50 LOC L3; 0 L1; ~6-10 testes).

### Ordem literal

1. Fase A §1 produzida + Decisões fixadas.
2. ADR-0091 anotação §2 escrita pós-Fase A.
3. `crystalline-lint --fix-hashes`.
4. **Testes-primeiro**.
5. Código:
   - L3 `scan_all_gradients` ganha parâmetro `parent_bbox_override: Option<Rect>` (Decisão 1α).
   - L3 dispatcher de `FrameItem::Group` constrói `group_bbox` e passa como override quando desce nos children.
   - L3 lógica de Inner-wins (Decisão 3α): Shape com `parent_bbox_at_emit: Some(...)` ignora override; `None` aceita override.
   - Eventual fix tactical de bug análogo §A.3.
6. Verificação final.

### Cap LOC (ADR-0094 Pattern 1; caps recalibrados realisticamente)

- **L3 hard cap**: ≤ 60 LOC.
- **L3 soft cap**: ≤ 40 LOC.
- **L1 hard cap**: 0 LOC (sem touch Layouter).
- **Tests hard cap**: ≤ 12.
- **Tests soft cap**: ≤ 8.

### Tests propostos (lista mínima — completar pós-Fase A)

1. `p273_10_group_bbox_constructed_from_frame_size` — unit test do helper L3 que constrói `Rect` de `(pos, frame.size)`.
2. `p273_10_shape_inside_group_inherits_group_bbox` — Shape sem `parent_bbox_at_emit` populado, dentro de Group, recebe `group_bbox` via override.
3. `p273_10_shape_with_populated_bbox_inside_group_inner_wins` — Shape **com** `parent_bbox_at_emit` populated (e.g. era dentro de Block), depois dentro de Group, **mantém** o próprio campo (Inner wins; Decisão 3α).
4. `p273_10_nested_groups_innermost_wins` — Group dentro de Group; innermost group_bbox é usado para Shape sem campo populated.
5. `p273_10_gradient_relative_parent_inside_group_uses_group_bbox` — Linear/Radial `relative=parent` dentro de Group emit usa group_bbox (E2E observable diff).
6. `p273_10_gradient_relative_self_inside_group_unchanged` — `relative=self` dentro de Group bit-exact P272.
7. `p273_10_shape_outside_group_unchanged` — Shapes top-level (não dentro de Group) preserved P273.9 bit-exact.
8. Regressão integrada: 2625 verdes preserved bit-exact + tests novos.

### Alterações esperadas no código

```rust
// L3 — 03_infra/src/export.rs

// scan_all_gradients ganha param
fn scan_all_gradients(
    items: &[FrameItem],
    parent_bbox_override: Option<Rect>,  // P273.10 novo
    // ... outros params ...
) {
    for item in items {
        match item {
            FrameItem::Shape { /* ... */, parent_bbox_at_emit, .. } => {
                // P273.10 — Inner wins (Decisão 3α)
                let effective_bbox = parent_bbox_at_emit
                    .or(parent_bbox_override);
                // ... resto preserved P273.6 ...
            }
            FrameItem::Group { pos, frame, .. } => {
                // P273.10 — Group bbox L3-only override
                let group_bbox = Rect {
                    x: Pt(pos.x.0),
                    y: Pt(pos.y.0),
                    w: Pt(frame.size.x.0),
                    h: Pt(frame.size.y.0),
                };
                // recurse com override (Decisão 1α — param threading)
                scan_all_gradients(
                    &frame.items,
                    Some(group_bbox),  // P273.10 — override
                    // ... outros params ...
                );
            }
            _ => { /* preserved */ }
        }
    }
}

// Callers de scan_all_gradients top-level passam `None`
```

### Verificação final

- Cap LOC respeitado (~30-50 LOC esperado vs cap soft 40 — margem realista).
- `cargo build` sem novos warnings.
- `cargo test --workspace` verde — 2625 → 2625 + tests novos.
- `crystalline-lint .` zero violations.
- Hashes L0 — `entities/gradient.md` propaga (anotação P273.10).
- Tests P262-P273.9 inalterados bit-exact (Inner wins preserva campo populated).
- DEBT saldo 10 preserved.
- Tests Group existentes inalterados — verificar tests P140 (multi-font sub-frame), P109 (frame composition) que usam Group.
- Test E2E observable diff confirmado para Group.

---

## §4 — Sub-padrões cumulativos pós-P273.10

| Sub-padrão | Pós-P273.9 | Pós-P273.10 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 17 | 18 |
| Reutilização literal helpers cross-passos | 17 | 17 (sem reuse novo — primeiro mecanismo L3-only) |
| Cap LOC hard vs soft explícito | 12 | 13 |
| Aplicação meta-ADR (ADR-0093) | 6 | 7 |
| Aplicação meta-ADR (ADR-0094) | 8 | 9 |
| Pattern DEBT-37 `cell_origin_*` replicado | N=4 (preserved) | N=4 (preserved — P273.10 não toca Layouter) |
| Template-passo replicado literal | 2 | 2 (preserved — mecanismo diferente, não template) |
| Sub-passos consecutivos do mesmo cluster | 5 | **6 cumulativo emergente** |
| Layout duplo arquitectural aceite | N=1 | N=1 (preserved — P273.10 sem layout duplo) |
| **L3-only parent_bbox** | N=0 | **N=1 inaugural emergente** |
| Diagnóstico imutável | 25 | 26 (21º consumo) |

Sub-padrão "L3-only parent_bbox" inaugural — precedente para futuros containers post-layout. Limiar formalização N=3-4 longe.

---

## §5 — Limitações conscientes P273.10

- Group bbox é geometric exact (`frame.size`); refino para bbox lógico fora de escopo per ADR-0054 graded.
- Inner wins (Decisão 3α) — Shapes com `parent_bbox_at_emit` populated (P273.9 5 containers) ignoram Group override; semântica "contentor mais próximo vence" preserved.
- Eventual fix tactical bug análogo (§A.3) — não introduz cascade adicional; corrige apenas o site detectado.
- Override só aplicado quando dispatcher Group desce nos children — Shapes dentro de Group via outros caminhos (e.g. translate sem Group dispatcher) preserved literal.
- Param threading explícito — adiciona signature noise vs alternativa global state; aceitável dado scan_all_gradients é internal.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.8 + revê Decisões 1+2+3.
5. Utilizador executa P273.10.B + P273.10.C → relatório.
6. Utilizador upload do relatório.
7. Claude web analisa + propõe **P273.11** (próximo da sequência: Extract Stack measurement helper).

---

## §7 — Pendências preservadas pós-P273.10

Inalteradas vs P273.9 (nível cluster):

- **P273.11** — Extract Stack measurement helper (XS; cleanup §9 P273.9; próximo na sequência).
- **P273.12** — Dedup bbox-aware (S-M; refino arquitectural).
- **P273.13** — CMYK-ICC krilla paridade (S-M; feature nova; **VERIFICAR Fase A se krilla API existe**).
- **P273.14** — Bbox medido pós-layout (M; refino qualitativo sem demanda).
- **P273.15** — Bbox.y topo-exacto inline (M-L; **BLOQUEADO** por DEBT-56 refactor multi-region).

**Pendências externas que vão bloquear o cluster**:
- **DEBT-56 refactor multi-region** — bloqueador de P273.15. Trabalho prévio externo necessário; será relatado quando lá chegarmos.

Cluster Gradient avança em direcção a terminar; predição factual cluster fecha empíricamente entre P273.13 e P273.15 consoante disponibilidade da krilla API.

---

## §8 — Critério de fecho do passo

P273.10 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.8 cumprido.
- ADR-0091 anotada (décima primeira anotação consecutiva).
- L3 alterado dentro do cap LOC; L1 intocado.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P273.9 inalterados bit-exact (Inner wins preserva).
- DEBT saldo 10 preserved.
- Test E2E observable diff confirmado para Group (Linear + Radial).
- Sub-padrões §4 atualizados.

---

## §9 — Numeração

Spec usa **P273.10** continuando a sequência decimal P273.5/P273.6/P273.7/P273.8/P273.9. Decisão utilizador na sessão: "continuar 273.X". P273.10 inicia a sub-sequência "terminar cluster Gradient" (escopo máximo — 6 passos até bloqueador externo).

Sequência prevista:
- **P273.10** — Group L3-only (este passo; S).
- **P273.11** — Extract Stack helper (XS; cleanup).
- **P273.12** — Dedup bbox-aware (S-M).
- **P273.13** — CMYK-ICC krilla (S-M; verificar API).
- **P273.14** — Bbox medido pós-layout (M).
- **P273.15** — Bbox.y topo-exacto inline (M-L; **bloqueado DEBT-56**).

Predição: cluster termina entre P273.13 e P273.15.
