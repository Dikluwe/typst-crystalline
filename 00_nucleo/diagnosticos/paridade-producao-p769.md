# Relatório de execução — P769

**Passo:** 769  
**Data:** 2026-07-16  
**Base:** P768 (auditoria de classificações comportamentais) + P767c (lógica de ancoramento de `Shape`)  
**Foco:** Correcção do ancoramento vertical de `Content::Image` no fluxo principal; medição condicional de `Content::Curve`.  
**L0:** `00_nucleo/prompts/rules/layout-image.md` (hash `ab5f39c1` confirmado pelo linter).

---

## 1. Alterações de código

### 1.1 L0 actualizado

O L0 `00_nucleo/prompts/rules/layout-image.md` estava desatualizado (não cobria a função `layout` nem o ancoramento). Foi reescrito para especificar:

- `Content::Image` como bloco no fluxo principal (`BlockElem::single_layouter`).
- Espaçamento `above`/`below` de `1.2em` com colapso de margem.
- Ancoramento vertical quando sucede texto não-bloco (`image_base = baseline_before_flush + above`).
- Comportamento em sub-layouts isolados (`is_sub_frame`).

### 1.2 `01_core/src/rules/layout/image.rs`

Replicou-se a lógica validada em `shape.rs` (P767c):

- Guardar `baseline_before_flush` antes de `flush_line()`.
- Detectar `had_text_line` via diferença do cursor.
- Aplicar colapso de margem com `block_chain_active` / `prev_block_below_pending`.
- Calcular `above_pt`, `below_pt` e `cap_height`.
- Posicionar a imagem conforme o contexto:
  - Após texto: `image_base = baseline_before_flush + above`.
  - Após outro bloco ou isolada: `image_base = cursor_y - cap_height`.
- Avançar o cursor correspondente (`+ below` ou `+ below + cap_height`).

O header `@prompt-hash` foi actualizado para `ab5f39c1`.

---

## 2. Validação por coordenadas (`mutool trace`)

Documentos gerados em `/tmp/p769-snapshot/`.

### 2.1 `A #image("tiny.png", width: 2cm, height: 1.5cm) B` — caso prioritário de P768

Coordenadas em pt no espaço do PDF (Y crescente para cima a partir da base da página).

| Elemento | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|---|---|---|---|
| "A" baseline | 78,104 | 78,105 | +0,001 |
| `image` base | 89,887 | 91,305 | +1,418 |
| `image` topo | 135,241 | 133,824 | −1,417 |
| "B" baseline | 154,262 | 154,262 | 0,000 |

**Interpretação:** o texto "B" alinhou-se exactamente com o vanilla (ΔY = 0). A diferença na base/topo da imagem (±1,4 pt) é consequência de uma escala ligeiramente diferente (vanilla 45,354 pt de altura vs cristalino 42,519 pt), não do ancoramento. O ancoramento vertical foi corrigido.

### 2.2 `A #image("tiny.png") B` — sem dimensões explícitas

| Elemento | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|---|---|---|---|
| "A" baseline | 78,104 | 78,105 | +0,001 |
| `image` base | 91,304 | 91,305 | +0,001 |
| `image` topo | 171,304 | 151,305 | −19,999 |
| "B" baseline | 191,742 | 171,743 | −19,999 |

**Interpretação:** o ancoramento está correcto (base da imagem e baseline de A alinhadas). O desvio de ~20 pt em "B" vem exclusivamente da diferença de escala da imagem: vanilla renderiza 100×80 pt, cristalino 75×60 pt (factor `PX_TO_PT = 0.75`). Este é um problema separado de resolução de imagem, fora do âmbito deste passo.

### 2.3 `A #curve(curve.move((0pt,0pt)), curve.line((20pt,20pt))) B` — medição condicional

| Elemento | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|---|---|---|---|
| "A" baseline | 78,104 | 78,105 | +0,001 |
| `curve` base | 91,304 | 91,300 | −0,004 |
| `curve` topo | 111,304 | 111,300 | −0,004 |
| "B" baseline | 131,742 | 131,743 | +0,001 |

**Interpretação:** `Content::Curve` já está correctamente ancorado no fluxo de texto. ΔY ≈ 0 em todos os pontos. Não é necessária qualquer correcção.

---

## 3. Decisões registadas (regra 1)

| Achado | Classificação | Decisão |
|---|---|---|
| `Image` com dimensões explícitas: texto subsequente alinha-se com vanilla. | Divergência corrigida | **Fechar** — ancoramento vertical replicado de P767c. |
| `Image` sem dimensões: escala diferente (100×80 vs 75×60 pt). | Problema separado de resolução/rasterização | **Backlog** — investigar `PX_TO_PT` / DPI da imagem num passo dedicado. |
| `Curve` com texto: alinhamento correcto. | Sem divergência | **Não corrigir** — medição confirma comportamento vanilla. |

---

## 4. Regressões verificadas

- Snapshot P307b `08-image-jpeg` actualizado — a posição Y da imagem isolada mudou ligeiramente (~0,46 pt) devido ao novo ancoramento de bloco.
- `cargo test --workspace`: todas as suites passaram.
- `crystalline-lint .`: zero violações excepto `V7` (prompt órfão pré-existente `package_version_resolution.md`).

---

## 5. Validação automática

```bash
cargo test --workspace
```

- **Resultado:** todas as suites passaram.

```bash
crystalline-lint .
```

- **Resultado:** zero violações excepto `V7` pré-existente.

---

## 6. Próximo passo

- Passo dedicado à escala/resolução de imagens raster (`PX_TO_PT` / DPI) para fechar a diferença de dimensões entre vanilla e cristalino.
- Revisitar `Figure`/`Table`/`Grid` e a diferença de ~0,27 pt em equação inline apenas se surgir evidência de impacto real.
