# Prompt L0 — rules/layout/image — `Content::Image` como bloco com ancoramento vertical

Hash do Código: [a preencher pelo humano após guardar]

**Camada**: L1 · **Alvo**: `01_core/src/rules/layout/image.rs`  
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (anti-deriva), ADR-0109 (atomização forma B)  
**Prompts relacionados**: `entities/elements/image.md`, `rules/layout/shape_block_behaviour.md`

---

## Propósito

Layout de `Content::Image` no fluxo do documento. Reproduz o comportamento do Typst vanilla: uma imagem é um elemento de **bloco** que quebra o parágrafo corrente, aplica espaçamento `above`/`below` por defeito e ancora verticalmente na grelha de linhas quando sucede texto não-bloco.

Base de evidência (P768):

- Em `lab/typst-original/crates/typst-layout/src/rules.rs`, `IMAGE_RULE` realiza `ImageElem` como `BlockElem::single_layouter`.
- No cristalino, `Content::Image` chamava `flush_line()` mas não aplicava `above`/`below` nem ancorava na próxima baseline, desviando o texto seguinte em ~19–39 pt em `A #image(...) B`.

---

## Contrato de comportamento

1. **Quebra de parágrafo implícita**: antes de posicionar a imagem, termina a linha/parágrafo corrente.
2. **Não participa em linhas de texto**: a imagem nunca aparece na mesma linha horizontal que texto circundante.
3. **Espaçamento de bloco**: aplica `above`/`below` de `1.2em` por defeito (paridade `BlockElem::spacing` do vanilla), com colapso de margem entre blocos consecutivos via `Layouter.block_chain_active` / `prev_block_below_pending`.
4. **Ancoramento vertical (P769)**:
   - Quando a imagem sucede texto não-bloco no mesmo parágrafo, a *base* da imagem ancora em `baseline_before_flush + above`; o topo fica em `base + height`; a próxima baseline do texto fica em `top + below + cap_height`.
   - Quando a imagem sucede outro bloco ou é a primeira de uma Sequence sem texto antes, mantém-se o modelo de bloco: base da imagem em `cursor_y − cap_height`, avanço `base + height + below`.
5. **Sub-layouts isolados**: dentro de `place(...)`, células de grid, etc. (`is_sub_frame == true`), preserva-se o posicionamento directo sem protocolo de bloco nem ancoramento especial.

---

## Constantes e tipos

```rust
const PX_TO_PT: f64 = 0.75;  // 96 DPI: 1 px = 72/96 pt

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
    sizer:       &dyn ImageSizer,
) -> ImageDimensions
```

1. Lê dimensões intrínsecas via `sizer.size(data)`; se `None`, fallback `100×100 pt`.
2. Calcula aspect ratio (`w / h`; fallback `1.0` se `h == 0`).
3. Aplica overrides:
   - ambos fornecidos → usa directamente (ignora aspect ratio).
   - só largura → `height = width / aspect`.
   - só altura → `width = height * aspect`.
   - nenhum → dimensões intrínsecas convertidas.
4. Retorna dimensões finais e intrínsecas.

Função auxiliar `extract_pt`:

- `Value::Float(f)` → `Some(*f)`.
- `Value::Length(l)` → `Some(l.abs.to_pt())` (componente absoluta; ignora em).
- Outros → `None`.

---

## `layout`

```rust
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &ImageElem,
)
```

1. Chama `layouter.ensure_initial_baseline()`.
2. Calcula dimensões via `calculate_dimensions`.
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
- O `FrameItem::Image.pos.y` é a base da imagem em coordenadas do Layouter (Y crescente para cima a partir da base da página); a imagem estende-se para cima pela sua altura.

---

## Testes

Manter os testes unitários existentes de `calculate_dimensions`. Adicionar testes de layout que verifiquem o posicionamento de `Content::Image` no fluxo principal, isolada e após texto.

---

## Critério de aceitação

- `A #image(...) B` no cristalino alinha-se ao vanilla (ΔY ≈ 0 em A, base/topo da imagem, B).
- Imagens isoladas e em sub-layouts não regressam.
- `cargo test --workspace` verde.
- `crystalline-lint .` zero violações (excepto V7 esperado).
- Header `@prompt` / `@prompt-hash` actualizado em `image.rs`.
