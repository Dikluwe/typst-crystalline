# Relatório de Paridade — P771

**Passo:** 771  
**Data:** 2026-07-16  
**Foco:** implementar `clip_path` para imagens com `fit`, fechando a divergência de 2,834 pt no alinhamento de "B" em `A #image(..., width: 2cm, height: 1.5cm) B`.

---

## Estado do código no momento da medição

- Commit base: `3aa25343c` (P770).
- Alterações não commitadas de P771:
  - `00_nucleo/prompts/engine/layout-image.md` — L0 actualizado com centralização no target, `clip_rect` e avanço de cursor a partir do target.
  - `01_core/src/entities/layout_types.rs` — campo `clip_rect: Option<Rect>` em `FrameItem::Image`.
  - `01_core/src/engine/layout/image.rs` — `target_width`/`target_height` em `ImageDimensions`, centralização da imagem, avanço com altura do target, `clip_rect` preenchido para `fit == "cover"`.
  - `03_infra/src/export/stream.rs` — emissão de `re W n` antes do `Do` da imagem quando `clip_rect` está presente.
  - `03_infra/fixtures/p307b/reference/08-image-jpeg.pdf` — snapshot regenerado (conteúdo visual idêntico, content stream agora inclui o clip path).
  - `03_infra/src/export/tests.rs`, `01_core/src/engine/layout/cursor.rs`, `helpers.rs`, `slicing.rs`, `01_core/src/engine/math/layout/mod.rs` — `clip_rect: None` adicionado às construções de `FrameItem::Image`.
- Comando do linter: `crystalline-lint .` — 0 violações (apenas V7 esperado).
- Comando de testes: `cargo test --workspace` — todos passaram.

---

## Sonda da estrutura do `clip_path` no vanilla

Leitura do content stream de `/tmp/p771/image-dims-vanilla.pdf` (caso `A #image("tiny.png", width: 2cm, height: 1.5cm) B`):

```
q 70.86614 750.58563 m 127.55905 750.58563 l 127.55905 708.0659 l 70.86614 708.0659 l
h
W
n/Span<</MCID 1>>BDC
q 56.692913 0 0 45.354332 70.86614 706.6486 cm /x0 Do
Q
EMC
Q
```

Observações:
- O clip é definido por um path rectangular equivalente a `re W n`.
- O rectângulo de clip corresponde exactamente ao `target` (`width × height` pedidos): 56,693 × 42,520 pt.
- A transformação da imagem (`56,693 × 45,354 pt`) excede o target em altura; o clip corta as bordas.
- O vanilla só emite clip quando `fit == "cover"` e ambos os eixos são fornecidos; `"contain"` e `"stretch"` não produzem clip no content stream (a transformação cabe ou iguala o target).

---

## Implementação

### `FrameItem::Image`

Adicionado campo `clip_rect: Option<Rect>`, preenchido apenas para `fit == "cover"` com ambos `width` e `height` fornecidos.

### `01_core/src/engine/layout/image.rs`

- `ImageDimensions` passou a incluir `target_width` e `target_height`.
- Quando ambos os eixos são fornecidos:
  - A transformação da imagem é calculada com `fit` (cover/contain/stretch).
  - A imagem é centralizada dentro do target: `offset = (target - image) / 2`.
  - O avanço do cursor usa a altura do **target**, não da transformação.
  - `clip_rect` é criado com o target quando `fit == "cover"`.
- Quando apenas um eixo ou nenhum é fornecido, o comportamento mantém-se como antes.

### `03_infra/src/export/stream.rs`

No emit top-level de `FrameItem::Image`, antes do `q ... cm /Name Do Q`:

```
{clip_x} {clip_pdf_y} {clip_w} {clip_h} re W n
```

onde `clip_pdf_y = page_height - clip.y - clip.h`.

---

## Validação

### Caso principal — `A #image("tiny.png", width: 2cm, height: 1.5cm) B`

| Ponto | Vanilla | Cristalino | Δ |
|---|---|---|---|
| A | 78,104 pt | 78,105 pt | 0,001 pt |
| Imagem base | 89,887 pt | 89,887 pt | 0,000 pt |
| Imagem topo (visível, target) | 133,824 pt | 133,824 pt | 0,000 pt |
| B | 154,262 pt | 154,262 pt | **0,000 pt** |

Antes de P771, B divergia 2,834 pt.

### Outros modos de `fit`

| Caso | Vanilla B | Cristalino B | Δ |
|---|---|---|---|
| `A #image(..., width: 2cm, height: 2cm) B` (cover) | 168,435 pt | 168,435 pt | 0,000 pt |
| `A #image(..., width: 2cm, height: 1.5cm, fit: "contain") B` | 154,262 pt | 154,262 pt | 0,000 pt |
| `A #image(..., width: 2cm, height: 1.5cm, fit: "stretch") B` | 154,262 pt | 154,262 pt | 0,000 pt |
| `A #image("tiny.png") B` (sem dimensões) | 191,742 pt | 191,743 pt | 0,001 pt |

### Comparação visual

```bash
mutool draw -o /tmp/p771/{caso}-vanilla.png -r 300 /tmp/p771/{caso}-vanilla.pdf
mutool draw -o /tmp/p771/{caso}-cristalino.png -r 300 /tmp/p771/{caso}-cristalino.pdf
compare -metric AE /tmp/p771/{caso}-vanilla.png /tmp/p771/{caso}-cristalino.png /tmp/p771/{caso}-diff.png
```

Resultado (`AE`) para todos os casos: **0** (pixel-exact match).

### Snapshot P307b

O snapshot `08-image-jpeg` regenerado; o conteúdo visual não mudou, mas o content stream PDF passou a incluir o operador de clip (a imagem é 1×1 px, target 30×30 pt, `fit=cover` → emite clip path).

---

## Testes

- `cargo test -p typst-core image` — 34 passaram.
- `cargo test --workspace` — 4153 + 637 + 33 + 2 + 27 + 2 = todos passaram.
- `crystalline-lint .` — 0 violações (apenas V7 esperado).

---

## Ficheiros alterados

- `00_nucleo/prompts/engine/layout-image.md` — L0 actualizado.
- `01_core/src/entities/layout_types.rs` — `clip_rect` em `FrameItem::Image`.
- `01_core/src/engine/layout/image.rs` — lógica de target, centralização e clip.
- `03_infra/src/export/stream.rs` — emissão do clip no PDF.
- `03_infra/src/export/tests.rs` — `clip_rect: None` nos testes.
- `01_core/src/engine/layout/cursor.rs`, `helpers.rs`, `slicing.rs` — `clip_rect: None` ou `..` nos patterns.
- `01_core/src/engine/math/layout/mod.rs` — `clip_rect: None` no constructor.
- `03_infra/fixtures/p307b/reference/08-image-jpeg.pdf` — snapshot regenerado.
- `00_nucleo/diagnosticos/paridade-producao-p771.md` — este relatório.

---

## Conclusão

P771 fecha o scope-out deixado por P770. A cadeia de `image()` está agora alinhada com o vanilla em:

- Escala px→pt (72 DPI) — P770.
- Aplicação de `fit` à transformação — P770.
- Centralização da imagem dentro do target — P771.
- `clip_path` quando `fit=cover` excede o target — P771.
- Avanço de cursor a partir do target — P771.

O caso `A #image(..., width: 2cm, height: 1.5cm) B`, que divergia 2,834 pt em P770, agora alinha-se com ΔY ≈ 0.
