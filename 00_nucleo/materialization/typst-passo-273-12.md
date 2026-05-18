# Passo P273.12 — Dedup bbox-aware (refino arquitectural pós-P273.10)

**Tipo**: refino arquitectural — chave de dedup `pat_ptr_to_idx` estendida com `parent_bbox` effective.
**Magnitude estimada**: S-M (~60-100 LOC L3; 0 L1; 6-10 testes).
**Pré-requisitos**: P273.11 fechado (helper Stack extraído; cleanup intra-cluster encerrado).
**Cluster**: Visualize / Gradient (terceiro de 6 sub-passos para fechar cluster).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; décima segunda anotação cumulativa); ADR-0029 (pureza física L1 preserved — passo L3-only).

---

## §0 — Contexto

Limitação registada em **P273.6 §9 quarto bullet** e preservada em **todos os relatórios subsequentes**:

> "Dedup bbox-aware — gradient com mesmo Arc usado em contextos distintos: actualmente primeiro wins."

P273.12 fecha essa pendência.

### Origem arquitectural do problema

O export L3 dedupa gradients via `Arc::as_ptr(g) as usize` (sub-padrão "Dedup Arc::as_ptr resources" N=2 — P73 image_resources + P263 pattern_resources). O `HashMap<usize, usize>` (`pat_ptr_to_idx`) mapeia `Arc::as_ptr` → índice no `Vec<GradientObject>`.

Pós-P273.6, cada `GradientObject` carrega `parent_bbox_at_emit: Option<Rect>`. Pós-P273.10, Inner-wins via `parent_bbox_at_emit.or(parent_bbox_override)`.

**Problema**: o `GradientObject` é singleton por `Arc::as_ptr`. Quando o mesmo `Arc<Gradient>` aparece em dois callsites com `parent_bbox_effective` diferentes (e.g. mesma definition usada em Block A 200×100 + Block B 400×200), **apenas o primeiro `parent_bbox` é capturado**; o segundo callsite renderiza com bbox da primeira ocorrência.

### Demonstração concreta

```typst
#let grad = gradient.linear(red, blue, relative: "parent")

#block(width: 200pt, height: 100pt)[
  #rect(width: 100%, height: 100%, fill: grad)  // bbox A: 200×100
]

#block(width: 400pt, height: 200pt)[
  #rect(width: 100%, height: 100%, fill: grad)  // bbox B: 400×200 (mas renderiza com bbox A)
]
```

**Comportamento actual**: segundo rect renderiza com gradient configurado para 200×100 (do primeiro block), produzindo gradient esticado ou comprimido em vez de adaptado ao 400×200.

**Comportamento esperado**: cada rect renderiza com gradient adaptado ao seu próprio contexto Parent.

### Solução arquitectural

Chave de dedup expandida: `(Arc::as_ptr, parent_bbox_effective_serializado)` em vez de `Arc::as_ptr` apenas.

- Mesmo Arc + mesmo bbox effective → mesmo PDF pattern (dedup preserved).
- Mesmo Arc + bbox effective diferente → PDF patterns distintos (semântica correcta).
- Bbox effective `None` (gradient `relative=self` ou top-level) → preserved P262-P273.11 (dedup por Arc apenas).

### Trade-off de tamanho PDF

Pior caso: N callsites com mesmo Arc em contextos diferentes → N PDF patterns onde antes era 1. Custo aceitável — semântica correcta sobrepõe-se ao custo bytes em casos genuínos. Caso comum (Arc usado em contexto único ou contextos idênticos) preserved literal — sem inflação.

---

## §1 — Sub-passo P273.12.A — Fase A diagnóstico

**Magnitude**: S documental (~25-35 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-12-diagnostico.md`.

### §A.1 — Inventário do dedup actual

Listar literal em `03_infra/src/export.rs`:

- **`scan_all_gradients`** — função declarada (linha X); helper `walk` recursivo pós-P273.10.
- **`pat_ptr_to_idx: HashMap<usize, usize>`** — chave actual `Arc::as_ptr(g) as usize`; valor índice no `Vec<GradientObject>`.
- **Insertion site** — provavelmente `pat_ptr_to_idx.entry(ptr).or_insert_with(|| { ... push GradientObject; idx })`. Confirmar empírico.
- **`pattern_resources_for_page`** — função análoga pós-P273.10 (recursive walk); enumera gradients seen para listar em `/Pattern << >>` PDF dict. Verificar se chave de dedup é a mesma.

### §A.2 — Inventário do `GradientObject`

Listar literal em `03_infra/src/export.rs`:

- Struct `GradientObject` — fields actuais (pós-P273.6: `kind, function_id, shading_id, pattern_id, parent_bbox_at_emit: Option<Rect>`).
- Inserção em `scan_all_gradients` — quando `parent_bbox_at_emit` é populated.
- Consumo no dispatcher (linha ~1638-1669) — como `effective_parent_bbox` é resolvido pós-P273.10.

### §A.3 — Decisão 1 — Forma da chave de dedup

Opções:

- **1α — Tuple `(usize, Option<RectKey>)`** onde `RectKey` é forma hash-friendly de `Rect`. Chave directa; `HashMap` literal; preserves dedup semantics quando bbox é `None`.
- **1β — Struct `DedupKey { arc_ptr: usize, bbox: Option<RectKey> }`** — mesma semântica que 1α com nome explícito. Mais legível em sítios consumidores.
- **1γ — Bbox quantizada** — `Rect` em pt convertido para `(i32, i32, i32, i32)` em milipontos para evitar issues de float hashing. Mais robusto a fp noise mas perde precisão sub-mp.

Recomendação spec: **1β + 1γ combinados**:
- `DedupKey { arc_ptr: usize, bbox: Option<RectKey> }`.
- `RectKey = (i32, i32, i32, i32)` em milipontos quantizados.

Razões:
1. `f64` em chave de `HashMap` é problemático (NaN, precision creep). Quantização resolve.
2. Milipontos quantizados (1mpt = 0.001 pt) preservam precisão sub-typográfica (typografia trabalha em pt; sub-pt raramente significativo).
3. Struct nomeado auto-documenta intent.

Decisão final na Fase A.

### §A.4 — Decisão 2 — Quando criar GradientObject distinto

Pseudocódigo do novo `scan_all_gradients`:

```rust
let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
let dedup_key = DedupKey {
    arc_ptr: Arc::as_ptr(g) as usize,
    bbox: effective_bbox.map(rect_to_key),
};

let idx = pat_dedup_key_to_idx
    .entry(dedup_key)
    .or_insert_with(|| {
        // Cria novo GradientObject com effective_bbox
        let new_idx = grad_objs.len();
        grad_objs.push(GradientObject {
            kind, function_id: next_id, ...,
            parent_bbox_at_emit: effective_bbox,
        });
        new_idx
    });
```

**Decisão crítica**: callsite-side ou scan-side?

- **2α — Callsite-side**: o `effective_bbox` calculado na call `apply_parent_transform` (dispatcher) influencia o dedup key. Mas dispatcher acontece DEPOIS de `scan_all_gradients`. Inviável sem refactor maior.
- **2β — Scan-side**: `scan_all_gradients` calcula `effective_bbox` (mesma lógica que dispatcher consumer pós-P273.10: `parent_bbox_at_emit.or(parent_bbox_override)`) e usa-o como dedup key. Caller-state preserved.

Recomendação spec: **2β**. Razões:
1. Dedup precisa decidir antes de criar PDF pattern objects.
2. `effective_bbox` é determinístico de `(parent_bbox_at_emit, parent_bbox_override)`.
3. Refactor mínimo — apenas a função dedup decide.

### §A.5 — Decisão 3 — Impacto cross-page

Detalhe operacional: `pat_ptr_to_idx` é global ao documento ou per-page?

- **3α — Global ao documento**: actual; um pattern PDF é reusado entre pages.
- **3β — Per-page**: cada page tem o seu próprio dedup map.

Pós-P273.12 com bbox na chave, ambos comportamentos preservados — mudança é só na chave, não no escopo. Mas verificar Fase A se `pattern_resources_for_page` precisa de actualização análoga.

### §A.6 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P273.11 | Mudança em chave de dedup | Bbox `None` (Self_/None relative) preserved literal; tests onde Arc é único context unaffected |
| PDF size explosion | N callsites mesmo Arc + contextos N → N PDF patterns | §A.7 critério: dedup ainda funciona quando bbox é igual; só duplica quando contextos diferem |
| Float in HashMap key issue | `f64` não impl `Hash` | Decisão 1γ: quantização milipontos `(i32, i32, i32, i32)` |
| Refactor scan_all_gradients quebra Group recursion P273.10 | P273.10 helper `walk` recursivo | Decisão 2β preserva signature; só altera lookup key |
| Bug latent análogo a P273.9/P273.10 | Estado existente pode descartar campo | Verificar empírico em §A.1 inventário pós-cascade P273.6 |

### §A.7 — Decisões a fixar na Fase A

1. **Decisão 1** (forma da chave): 1α / 1β / 1γ / combinação. Recomendação: 1β + 1γ.
2. **Decisão 2** (callsite vs scan-side): 2α / 2β. Recomendação: 2β.
3. **Decisão 3** (cross-page): confirmar global ou per-page consoante inventário §A.5.

### §A.8 — Critério de aceitação Fase A

- §A.1 cita `scan_all_gradients` + `pat_ptr_to_idx` literal (path:linha pós-P273.10).
- §A.2 confirma `GradientObject` field actuais.
- §A.5 risco "regressão P262-P273.11" mitigado por preservação `bbox=None` path.
- §A.7 Decisões 1+2+3 fixadas.

---

## §2 — Sub-passo P273.12.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental.

**Não criar ADR-0095**. Anotar ADR-0091 — décima segunda anotação consecutiva.

Template:

```
## Anotação cumulativa P273.12 — Dedup bbox-aware (refino arquitectural pós-P273.10)

**Data**: 2026-05-XX.
**Motivo**: P273.6 §9 quarto bullet identificou limitação: "gradient com
mesmo Arc usado em contextos distintos: actualmente primeiro wins".
Limitação preserved em todos os relatórios subsequentes; P273.12
fecha.

**Decisão 1 fixada (chave dedup)**: [1α/1β/1γ/combinação — preencher
pós-Fase A].
**Decisão 2 fixada (callsite vs scan-side)**: [2α/2β].
**Decisão 3 fixada (cross-page)**: [3α/3β].

**Sub-padrão "Dedup Arc::as_ptr resources"** N=2 → **N=3 cumulativo**
(P73 image + P263 pattern + P273.12 pattern bbox-aware) — atinge
limiar formalização ADR meta N=3-4.

**Defaults preservam P262-P273.11 bit-exact**:
- Gradient `relative=self/None` (bbox effective = None) → preserved
  literal (dedup por Arc apenas, semantic P262-P273.11).
- Arc usado em context único → idem.
- Apenas Arc com bbox effective diferentes em N contexts produz N
  PDF patterns (semântica correcta vs primeira-wins).

**Trade-off PDF size**: pior caso N→N patterns; caso comum preserved.

**Sub-padrão "Bug arquitectural intencional corrigido"** N=0 → N=1
inaugural — limitação documentada P273.6 §9 corrigida 6 sub-passos
depois, sem fix tactical no momento, com refino arquitectural
deliberado.
```

---

## §3 — Sub-passo P273.12.C — Materialização (testes primeiro)

**Magnitude**: S (~60-100 LOC L3; 0 L1; 6-10 testes).

### Ordem literal

1. Fase A §1 produzida + Decisões fixadas.
2. ADR-0091 anotação §2 escrita.
3. `crystalline-lint --fix-hashes`.
4. **Testes-primeiro**.
5. Código:
   - L3 — definir `RectKey` + `DedupKey` (Decisão 1β + 1γ).
   - L3 — refactor `scan_all_gradients` para usar `DedupKey` em vez de `usize` (Decisão 2β).
   - L3 — propagar `parent_bbox_override` no scan (já existe pós-P273.10 via helper `walk`).
   - L3 — actualizar `pattern_resources_for_page` análoga (Decisão 3).
   - Eventual fix tactical de bug latent (§A.6).
6. Verificação final.

### Cap LOC (ADR-0094 Pattern 1; caps realistas pós-experiência cluster)

- **L3 hard cap**: ≤ 100 LOC.
- **L3 soft cap**: ≤ 70 LOC.
- **L1 hard cap**: 0 LOC (sem touch Layouter).
- **Tests hard cap**: ≤ 12.
- **Tests soft cap**: ≤ 8.

### Tests propostos (lista mínima — completar pós-Fase A)

1. `p273_12_same_arc_same_bbox_dedup_to_single_pattern` — Arc reusado em dois callsites com bboxes idênticos → 1 PDF pattern (preserved P262-P273.11).
2. `p273_12_same_arc_different_bbox_creates_two_patterns` — Arc reusado em dois Blocks com dimensions diferentes → 2 PDF patterns distintos.
3. `p273_12_arc_with_bbox_none_unchanged` — Self_/None relative preserved P262 bit-exact (1 pattern para 2 occurrences mesmo Arc).
4. `p273_12_rect_to_key_quantization` — unit test de `rect_to_key` confirma `(i32, i32, i32, i32)` em milipontos com round half-up.
5. `p273_12_three_contexts_three_patterns` — Arc em 3 contextos diferentes → 3 patterns.
6. `p273_12_observable_diff_pdf_bytes` — E2E confirma bytes PDF do segundo pattern são distintos do primeiro (Coords diferentes).
7. `p273_12_pattern_resources_for_page_aligned` — `/Pattern << >>` PDF dict lista todos os patterns dedup-distinct.
8. Regressão integrada: 2632 verdes preserved + tests novos.

### Alterações esperadas no código

```rust
// L3 — 03_infra/src/export.rs

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct RectKey(i32, i32, i32, i32);  // milipontos

fn rect_to_key(r: Rect) -> RectKey {
    RectKey(
        (r.x.0 * 1000.0).round() as i32,
        (r.y.0 * 1000.0).round() as i32,
        (r.w.0 * 1000.0).round() as i32,
        (r.h.0 * 1000.0).round() as i32,
    )
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct DedupKey {
    arc_ptr: usize,
    bbox: Option<RectKey>,
}

// scan_all_gradients refactor:
// Antes: HashMap<usize, usize> (arc_ptr → idx)
// Depois: HashMap<DedupKey, usize>

fn walk(items, parent_bbox_override, dedup_map, ...) {
    for item in items {
        match item {
            FrameItem::Shape { stroke: Some(Stroke { paint: Paint::Gradient(g), .. }), parent_bbox_at_emit, .. } => {
                let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
                let key = DedupKey {
                    arc_ptr: Arc::as_ptr(g) as usize,
                    bbox: effective_bbox.map(rect_to_key),
                };
                let idx = dedup_map.entry(key).or_insert_with(|| {
                    let new_idx = grad_objs.len();
                    grad_objs.push(GradientObject {
                        // ...
                        parent_bbox_at_emit: effective_bbox,
                    });
                    new_idx
                });
                // ...
            }
            FrameItem::Group { /* ... */ } => {
                walk(items, Some(group_bbox), dedup_map, ...);
            }
            _ => {}
        }
    }
}
```

### Verificação final

- Cap LOC respeitado.
- `cargo build` sem novos warnings.
- `cargo test --workspace` verde — 2632 → 2632 + tests novos.
- `crystalline-lint .` zero violations.
- Hashes L0 — `entities/gradient.md` propaga (anotação P273.12).
- Tests P262-P273.11 inalterados bit-exact (Self_/None preserved; Arc-único preserved).
- DEBT saldo 10 preserved.
- Test E2E observable diff confirma bbox-aware dedup.

---

## §4 — Sub-padrões cumulativos pós-P273.12

| Sub-padrão | Pós-P273.11 | Pós-P273.12 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 18 | 19 |
| Reutilização literal helpers cross-passos | 17 | 17 (preserved) |
| Cap LOC hard vs soft explícito | 14 | 15 |
| Aplicação meta-ADR (ADR-0093) | 7 | 8 |
| Aplicação meta-ADR (ADR-0094) | 10 | 11 |
| Pattern DEBT-37 replicado | N=4 (preserved) | N=4 (preserved) |
| Template-passo replicado literal | N=2 (preserved) | N=2 (preserved) |
| Sub-passos consecutivos do mesmo cluster | N=7 emergente | **N=8 cumulativo emergente** |
| Layout duplo arquitectural aceite | N=1 (preserved) | N=1 (preserved) |
| L3-only parent_bbox | N=1 (preserved) | **N=1 reused** (P273.12 também é L3-only) |
| Bug latent corrigido em scope creep | N=1/2 (preserved) | N=1/2 (preserved) |
| Extract helper de replicação inline | N=1 (preserved) | N=1 (preserved) |
| **Dedup Arc::as_ptr resources** | N=2 | **N=3 cumulativo (limiar atingido)** |
| **Bug arquitectural intencional corrigido** | N=0 | **N=1 inaugural emergente** |
| Diagnóstico imutável | 27 | 28 (23º consumo) |

Sub-padrão "Dedup Arc::as_ptr resources" cresce N=2 → **N=3** — atinge limiar formalização ADR meta N=3-4. P73 (image) + P263 (pattern por Arc) + P273.12 (pattern bbox-aware). Candidato meta-ADR formalização NÃO reservado.

Sub-padrão "Bug arquitectural intencional corrigido" inaugurado N=1. Precedente: limitação documentada com decisão deliberada de adiar fix arquitectural; corrigida em sub-passo subsequente quando contexto madura. Distingue de "Bug latent corrigido em scope creep" (que é defeito não-detectado descoberto durante outro passo).

---

## §5 — Limitações conscientes P273.12

- Quantização milipontos (1mpt precision) — sub-mpt precision perde-se. Aceitável; typografia trabalha em pt.
- N callsites mesmo Arc + N bboxes distintos → N PDF patterns (PDF size inflation no pior caso). Caso comum (Arc único context ou contextos idênticos) preserved.
- Dedup é per-`(Arc, bbox)` — gradient com mesma definition mas Arc diferente continua a ser tratado como gradient distinto. Aceitável (Arc é identidade de definition).
- Não toca dedup de outros resources (imagens via P73). Cluster específico.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.8 + revê Decisões 1+2+3.
5. Utilizador executa P273.12.B + P273.12.C → relatório.
6. Utilizador upload do relatório.
7. Claude web analisa + propõe **P273.13** (próximo da sequência: CMYK-ICC krilla paridade — **verificar API krilla na Fase A**).

---

## §7 — Pendências preservadas pós-P273.12

Inalteradas vs P273.11 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M; **VERIFICAR Fase A se krilla API existe**).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Sequência para fechar cluster Gradient:

- **P273.13** — CMYK-ICC krilla paridade (S-M; possível bloqueador API).
- **P273.14** — Bbox medido pós-layout (M).
- **P273.15** — Bbox.y topo-exacto inline (M-L; **BLOQUEADO** por DEBT-56).

---

## §8 — Critério de fecho do passo

P273.12 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.8 cumprido.
- ADR-0091 anotada (décima segunda anotação consecutiva).
- L3 alterado dentro do cap LOC; L1 intocado.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P273.11 inalterados bit-exact (Self_/None preserved; Arc-único preserved).
- DEBT saldo 10 preserved.
- Test E2E observable diff confirma 2 PDF patterns distintos para Arc partilhado em bboxes diferentes.
- Sub-padrões §4 atualizados.

---

## §9 — Numeração

Spec usa **P273.12** continuando a sequência decimal. Terceiro passo da sub-sequência "terminar cluster Gradient" (escopo máximo).

Sequência prevista preserved:
- ✓ P273.10 — Group L3-only (S; fechado).
- ✓ P273.11 — Extract Stack helper (XS; fechado).
- **P273.12 — Dedup bbox-aware** (S-M; este passo).
- P273.13 — CMYK-ICC krilla (S-M; verificar API).
- P273.14 — Bbox medido pós-layout (M).
- P273.15 — Bbox.y topo-exacto inline (M-L; bloqueado DEBT-56).

Predição: cluster termina entre P273.13 e P273.15.
