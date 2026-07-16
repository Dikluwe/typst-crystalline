# Prompt L0 — rules/layout/image — `Content::Image` como bloco com ancoramento vertical

Hash do Código: 1461abf0

**Camada**: L1 · **Alvo**: `01_core/src/rules/layout/image.rs`  
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (anti-deriva), ADR-0109 (atomização forma B)  
**Prompts relacionados**: `entities/elements/image.md`, `rules/layout/shape_block_behaviour.md`

---

## Propósito

Layout de `Content::Image` no fluxo do documento. Reproduz o comportamento do Typst vanilla: uma imagem é um elemento de **bloco** que quebra o parágrafo corrente, aplica espaçamento `above`/`below` por defeito, ancora verticalmente na grelha de linhas quando sucede texto não-bloco e resolve dimensões finais respeitando `fit` (cover/contain/stretch) e DPI padrão de 72.

Base de evidência (P768/P770):

- Em `lab/typst-original/crates/typst-layout/src/rules.rs`, `IMAGE_RULE` realiza `ImageElem` como `BlockElem::single_layouter`.
- Em `lab/typst-original/crates/typst-layout/src/image.rs`, o vanilla resolve dimensões com `Image::DEFAULT_DPI = 72.0` e aplica `ImageFit` (`Cover`/`Contain`/`Stretch`) mesmo quando `width` e `height` são ambos fornecidos.
- No cristalino, `Content::Image` chamava `flush_line()` mas não aplicava `above`/`below` nem ancorava na próxima baseline (corrigido em P769).
- A escala px→pt e a lógica de `fit` divergiam do vanilla (corrigido em P770).

---

## Contrato de comportamento

1. **Quebra de parágrafo implícita**: antes de posicionar a imagem, termina a linha/parágrafo corrente.
2. **Não participa em linhas de texto**: a imagem nunca aparece na mesma linha horizontal que texto circundante.
3. **Espaçamento de bloco**: aplica `above`/`below` de `1.2em` por defeito (paridade `BlockElem::spacing` do vanilla), com colapso de margem entre blocos consecutivos via `Layouter.block_chain_active` / `prev_block_below_pending`.
4. **Ancoramento vertical (P769)**:
   - Quando a imagem sucede texto não-bloco no mesmo parágrafo, a *base* da imagem ancora em `baseline_before_flush + above`; o topo fica em `base + height`; a próxima baseline do texto fica em `top + below + cap_height`.
   - Quando a imagem sucede outro bloco ou é a primeira de uma Sequence sem texto antes, mantém-se o modelo de bloco: base da imagem em `cursor_y − cap_height`, avanço `base + height + below`.
5. **Sub-layouts isolados**: dentro de `place(...)`, células de grid, etc. (`is_sub_frame == true`), preserva-se o posicionamento directo sem protocolo de bloco nem ancoramento especial.
6. **Resolução de dimensões (P770)**:
   - DPI padrão para conversão px→pt: **72** (`Image::DEFAULT_DPI` do vanilla).
   - `fit` default `"cover"`; valores válidos `"contain"`, `"cover"`, `"stretch"`.
   - Quando `width` e `height` são ambos fornecidos e `fit` é `"stretch"`, usar os valores exactos.
   - Quando `width` e `height` são ambos fornecidos e `fit` é `"cover"` ou `"contain"`, ajustar as dimensões finais da *transformação* da imagem para preencher o rectângulo target preservando o aspect ratio original (paridade `typst-layout/src/image.rs`).
   - Quando apenas um eixo é fornecido, calcular o outro pelo aspect ratio (independente de `fit`).
   - Quando nenhum é fornecido, usar dimensões intrínsecas convertidas por `px * (72 / dpi)`; sem metadados de DPI, `dpi = 72`.
   - **Scope-out P770.1**: o vanilla aplica, além do ajuste de dimensões, um `clip_path` com o rectângulo exacto do `target` (`width` × `height`) e centraliza a imagem dentro desse clip. O cristalino ainda **não** emite esse clip; o resultado visual e o avanço do cursor podem divergir quando `cover`/`contain` produzem dimensões diferentes do target. Fechar o clipping de imagem é scope-out para passo futuro.

---

## Constantes e tipos

```rust
const PX_TO_PT: f64 = 1.0;  // 72 DPI padrão do vanilla: 1 px = 1 pt

pub struct ImageDimensions {
    pub width_pt:         f64,
    pub height_pt:        f64,
    pub intrinsic_width:  Option<u32>,
    pub intrinsic_height: Option<u32>,
}
```

`intrinsic_width`/`intrinsic_height` evitam uma segunda chamada ao `ImageSizer` no layouter (DEBT-28).

---

## `calculate_dimensions`

```rust
pub fn calculate_dimensions(
    data:        &[u8],
    user_width:  Option<&Value>,
    user_height: Option<&Value>,
    fit:         &str,
    sizer:       &dyn ImageSizer,
) -> ImageDimensions
```

1. Lê dimensões intrínsecas via `sizer.size(data)`; se `None`, fallback `100×100 pt`.
2. Calcula aspect ratio (`w / h`; fallback `1.0` se `h == 0`).
3. Converte dimensões intrínsecas para pt usando `PX_TO_PT` (72 DPI padrão).
4. Extrai `req_w`/`req_h` via `extract_pt`:
   - `Value::Float(f)` → `Some(*f)`.
   - `Value::Length(l)` → `Some(l.abs.to_pt())`.
   - Outros → `None`.
5. Resolve dimensões finais:
   - `fit == "stretch"` e ambos fornecidos → `(req_w, req_h)`.
   - `fit == "cover"` ou `"contain"` e ambos fornecidos → calcular `target = (req_w, req_h)`; ajustar para preencher o target preservando aspect ratio:
     - `wide = aspect > target_w / target_h`.
     - Se `wide == (fit == "contain")` → `(target_w, target_w / aspect)`.
     - Caso contrário → `(target_h * aspect, target_h)`.
   - Só `req_w` → `(req_w, req_w / aspect)`.
   - Só `req_h` → `(req_h * aspect, req_h)`.
   - Nenhum → dimensões intrínsecas convertidas.
6. Retorna dimensões finais e intrínsecas.

---

## `layout`

```rust
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &ImageElem,
)
```

1. Chama `layouter.ensure_initial_baseline()`.
2. Calcula dimensões via `calculate_dimensions(&e.data.0, e.width.as_deref(), e.height.as_deref(), &e.fit, &layouter.sizer)`.
3. Guarda `baseline_before_flush = layouter.regions.current.cursor_y`.
4. Executa `layouter.flush_line()`.
5. Determina `had_text_line = cursor_y_after_flush.0 > baseline_before_flush.0 + epsilon`.
6. Se `!is_sub_frame`:
   - Aplica colapso de margem com o bloco anterior via `block_chain_active` / `prev_block_below_pending`.
   - Calcula `above_pt` e `below_pt` a partir de `1.2em`.
   - Alinha `cursor_x` a `line_start_x`.
7. Verifica overflow de página; se necessário `new_page()`.
8. Calcula `image_base`:
   - `!in_main_flow` → `cursor_y`.
   - `block_chain_active` → `cursor_y - cap_height`.
   - `had_text_line` → `baseline_before_flush + above_pt`.
   - caso contrário → `cursor_y - cap_height`.
9. Emite `FrameItem::Image` em `Point { x: cursor_x, y: image_base }` com `width`, `height` e dimensões intrínsecas.
10. Avança o cursor:
    - Em sub-frame: `cursor_y += height`.
    - No fluxo principal após bloco: `cursor_y = image_base + height + below`.
    - No fluxo principal após texto: `cursor_y = image_base + height + below + cap_height`.
11. Define `prev_block_below_pending = below_pt` e `block_chain_active = true` no fluxo principal.
12. Verifica overflow de página novamente.

---

## Invariantes

- Zero dependências externas em L1 — `imagesize` nunca importado aqui.
- Fallback `100×100` documentado.
- Aspect ratio sempre preservado quando apenas um override é fornecido.
- `fit` é aplicado mesmo quando ambos `width` e `height` são fornecidos.
- O `FrameItem::Image.pos.y` é a base da imagem em coordenadas do Layouter (Y crescente para cima a partir da base da página); a imagem estende-se para cima pela sua altura.

---

## Testes

Manter os testes unitários existentes de `calculate_dimensions`. Adicionar testes que cubram:

- `fit = "stretch"` com ambos os eixos forçados usa dimensões exactas.
- `fit = "cover"`/`"contain"` com ambos os eixos forçados preserva aspect ratio.
- Conversão px→pt com `PX_TO_PT = 1.0` (72 DPI).
- Layout de `Content::Image` no fluxo principal, isolada e após texto.

---

## Critério de aceitação

- Imagens sem dimensões explícitas: dimensões da transformação igualam o vanilla (100×80pt no caso de teste P770); `A #image(...) B` alinha-se ao vanilla (ΔY ≈ 0 em A e B).
- Imagens com dimensões explícitas: a transformação da imagem aplica `fit` igual ao vanilla (cover/contain/stretch); dimensões da transformação batem a ±0,001pt.
- Imagens isoladas e em sub-layouts não regressam.
- `cargo test --workspace` verde.
- `crystalline-lint .` zero violações (excepto V7 esperado).
- Header `@prompt` / `@prompt-hash` actualizado em `image.rs`.
