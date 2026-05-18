# Diagnóstico Fase A P273.8.A — Cleanup `unused_variable: parent_bbox_at_emit` em export.rs

**Data**: 2026-05-18.
**Passo**: typst-passo-273.7.1.A.
**Magnitude**: XS-XXS documental (~5 min).
**Cluster**: Visualize / Gradient (cleanup pós-cluster).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Décimo nono consumo directo de fonte** (cleanup XS residual P273.6).

---

## §A.1 — Localização literal e propósito dos 4 sítios

`03_infra/src/export.rs` — confirmação empírica via `cargo build` (warnings inalterados pós-P273.7; export.rs não tocado em P273.7):

| Linha | Função / Contexto | Pattern literal | Propósito do bloco |
|---|---|---|---|
| **2003** | `draw_item` (page-level direct emit, Y-inversion via `pdf_y = page_height - pos.y - height`) | `FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } =>` | PDF emit shape: `q ... fill/stroke ... path ... Q`. Gradient paint resolvido via `emit_stroke_paint` que consulta `pat_ptr_to_idx` pré-computado. |
| **2275** | `draw_item_local` (Group children — pais aplicou Y-inversion; filhos usam `local_y = pos.y.0` directo) | `FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } =>` | PDF emit shape local-coord (sem Y-inversion). Análogo §2003 mas em contexto Group/Transform. |
| **2521** | `draw_item` variante (provavelmente Y-inversion + `emit_stroke_paint`) | `FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } =>` | PDF emit shape com gradient stroke via `emit_stroke_paint(&mut ops, &s.paint, s.thickness, pat_ptr_to_idx, pat_refs)`. |
| **2705** | `draw_item` variante 3 (idêntica linha 2521 — duplicação Z-order ou clip path) | `FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } =>` | Análogo §2521. |

**Forma do binding**: todos 4 sítios destructuram explicitamente os 7 campos sem `..` rest pattern. O binding `parent_bbox_at_emit` é capturado pelo cascade automático P273.6 mas nunca lido no corpo do `match` arm.

---

## §A.2 — Confirmação: `scan_all_gradients` NÃO está entre os 4 sítios

`scan_all_gradients` (path:linha):
- Função declarada em **`export.rs:357`**.
- Pattern-match consumidor activo em **`export.rs:376`** (`parent_bbox_at_emit` destructurado).
- **Consumo activo** em **`export.rs:406-407`** (`parent_bbox_at_emit: *parent_bbox_at_emit` → injectado em `GradientObject`).

Confirmado empíricamente: linha 376 ≠ 2003, 2275, 2521, 2705. `scan_all_gradients` é o ÚNICO consumidor activo do campo + 1 site em `export.rs:1638-1643` (dispatcher destructure `GradientObject { ..., parent_bbox_at_emit }` para computar `effective_parent_bbox`).

Os 4 sítios candidatos a cleanup são todos PDF emit-paths que dependem do pipeline pré-computado por `scan_all_gradients` (resolução via `pat_ptr_to_idx` + `pat_refs`) — **não precisam ler o campo directamente**.

---

## §A.3 — Decisão 1 fixada: forma do cleanup

**Decisão fixada**: **Opção α** — `parent_bbox_at_emit: _` no pattern.

**Razões**:
1. Sinala intenção declarativamente: "consciência do campo + decisão de ignorar".
2. Não cria binding-fantasma `_parent_bbox_at_emit` que possa colidir futuramente em rename/refactor.
3. Mais robusto a refactors — se o campo desaparecer da struct ou for renomeado, o pattern parte de imediato (sinala que o ignore precisa ser reavaliado).
4. Idiomático Rust + alinhado com a sugestão do próprio `rustc` (mensagem: `help: try ignoring the field: \`parent_bbox_at_emit: _\``).

**Opção γ (`..` rest pattern) rejeitada**: o pattern actual é destructure completo explícito (7 campos) — substituir um destructure explícito por `..` reduziria a precisão declarativa do pattern (deixaria de sinalizar quais campos a função sabe que existem).

---

## §A.4 — Decisão 2 fixada: zero "consumidor esquecido"

Análise empírica dos 4 sítios confirma que TODOS são "ignorantes conscientes":

- Os 4 sítios são **PDF emit paths** que serializam shape geometry + paint ops em PDF content streams.
- Gradient paint nesses sítios é processado via **`emit_stroke_paint`** (linhas 2532, 2716, etc.) que consulta **`pat_ptr_to_idx`** pré-computado por `scan_all_gradients`.
- `scan_all_gradients` (sítio único consumidor real) já capturou `parent_bbox_at_emit` para o `GradientObject`; o dispatcher (linha 1638-1643) já resolveu `effective_parent_bbox` para construir o pattern PDF.
- Os emit-paths só referenciam o pattern ID resolvido — **não precisam aceder ao campo directamente**.

Resultado: **0 sítios precisam abrir DEBT separado**. Todos 4 são candidatos legítimos a `_`.

---

## §A.5 — Análise de risco

| Risco | Estado |
|---|---|
| Warning continua após patch | Mitigado — `cargo build` empírico confirma linhas 2003/2275/2521/2705 exactas |
| Quebra compile | Mitigado — `_` em destructure pattern puro é semanticamente idêntico a binding sem uso |
| Regressão tests | Mitigado — cleanup é warning-only; bytes PDF idênticos bit-exact |
| Sítio é consumidor esquecido | **§A.4 confirma 0 sítios afectados** |
| Hash drift L0 | export.rs não tem L0 directo; `crystalline-lint` zero impact esperado |

---

## §A.6 — Critério de aceitação Fase A

- ✓ §A.1 confirma path:linha empírico (4 warnings: 2003/2275/2521/2705) + propósito de cada bloco identificado.
- ✓ §A.2 confirma `scan_all_gradients` (linha 376) é o único consumidor activo + dispatcher (1638) — NÃO está entre os 4 sítios.
- ✓ §A.3 Decisão 1 fixada: **Opção α** (`parent_bbox_at_emit: _`).
- ✓ §A.4 Decisão 2 fixada: **0 consumidores esquecidos** — todos 4 são ignorantes conscientes legítimos.

**Fase A produzida — critério §A.6 cumprido absoluto.**

---

## §A.7 — Plano de execução (Fase C)

### Cap LOC (ADR-0094 Pattern 1)

- **L3 hard cap**: ≤ 8 LOC (4 substituições × 2 LOC esperado).
- **L3 soft cap**: ≤ 4 LOC (substituições puras 1 LOC cada).
- **Tests hard cap**: 0 novos.
- **Tests soft cap**: 0.

### Patch literal nos 4 sítios

Substituição idêntica em cada um:

```diff
- FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } => {
+ FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit: _ } => {
```

4 substituições × 1 LOC cada = **4 LOC real** (cap soft 4 respeitado).

### Verificação

- `cargo build` → 0 warnings `unused variable: parent_bbox_at_emit`.
- `cargo test --workspace` → 2620 verdes preserved bit-exact (cleanup é cosmético).
- `crystalline-lint .` → 0 violations.

---

*Diagnóstico imutável produzido em 2026-05-18. Décimo nono consumo directo de fonte. Decisões 1+2 fixadas; pronto para Fase C (~4 LOC L3; 0 testes; cleanup puramente cosmético).*
