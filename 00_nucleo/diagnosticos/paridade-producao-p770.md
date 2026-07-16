# Relatório de Paridade — P770

**Passo:** 770  
**Data:** 2026-07-16  
**Foco:** investigar a divergência de altura de `#image(..., width: 2cm, height: 1.5cm)` e corrigir a escala píxel→ponto em imagens sem dimensões explícitas.

---

## Estado do código no momento da medição

- Commit base: `85ffd776f` ("P769 — correcção do ancoramento vertical de Content::Image e auditoria de Curve").
- Alterações não commitadas de P770:
  - `00_nucleo/prompts/rules/layout-image.md` (L0 actualizado com escala 72 DPI, lógica `fit` e scope-out do clip).
  - `01_core/src/rules/layout/image.rs` (`PX_TO_PT = 1.0`, `calculate_dimensions` com `fit`, `@prompt-hash` `d774dfa0`).
- Comando do linter: `crystalline-lint .` — 0 violações (excepto V7 esperado, prompt órfão `package_version_resolution.md`).
- Comando de testes: `cargo test --workspace` — todos passaram.

---

## Parte A — Causa da divergência de altura com `height:` explícito

### Sonda

O vanilla renderizou `#image("tiny.png", width: 2cm, height: 1.5cm)` com uma transformação de imagem de **56,693 × 45,354 pt**, apesar de `height: 1.5cm` equivaler a **42,520 pt**. A leitura directa do PDF (`mutool draw -F trace`) mostra que o vanilla não desenha simplesmente a imagem com `width × height`; em vez disso:

1. Calcula dimensões finais aplicando `ImageFit::Cover` (padrão) ao target `2cm × 1.5cm`, preservando o aspect ratio da imagem original (`100/80 = 1,25`).
2. Emite um `clip_path` exactamente com o rectângulo do target (`56,693 × 42,520 pt`).
3. Desenha a imagem com a transformação `56,693 × 45,354 pt` dentro desse clip.

O cristalino, antes de P770, ignorava o parâmetro `fit` e usava `width`/`height` directamente (42,520 pt de altura). Após P770, o cristalino aplica a mesma lógica de `fit` nas dimensões da transformação da imagem.

### Medições (`/tmp/p770/`)

| Caso | Vanilla transformação | Cristalino transformação | Δ |
|---|---|---|---|
| `#image("tiny.png")` | 100,000 × 80,000 | 100,000 × 80,000 | 0,000 |
| `#image("tiny.png", width: 2cm, height: 1.5cm)` | 56,693 × 45,354 | 56,692 × 45,354 | 0,001 |
| `#image("tiny.png", width: 2cm, height: 2cm)` | 70,866 × 56,693 | 70,865 × 56,692 | 0,001 |

`tiny.png` é 100 × 80 px, sem metadados de DPI relevantes; o vanilla usa `Image::DEFAULT_DPI = 72.0`, logo 1 px = 1 pt.

### "B alinhado" de P769 era coincidência

P769 fechou `A #image("tiny.png") B` com ΔY ≈ 0 em A e B. Após a correcção de escala e `fit`, esse caso continua a bater:

| Caso | Vanilla B | Cristalino B | ΔY |
|---|---|---|---|
| `A #image("tiny.png") B` | 191,742 pt | 191,743 pt | 0,001 pt |

No entanto, quando se forçam dimensões explícitas, o alinhamento de B diverge:

| Caso | Vanilla B | Cristalino B | ΔY |
|---|---|---|---|
| `A #image("tiny.png", width: 2cm, height: 1.5cm) B` | 154,262 pt | 157,096 pt | **2,834 pt** |

A causa da divergência de B não é o ancoramento implementado em P769; é o facto de o vanilla **recortar** a imagem ao target (`clip_path`) e posicionar o cursor com base nesse rectângulo de clip, enquanto o cristalino desenha a imagem completa com as dimensões pós-`fit` e avança o cursor a partir daí. Assim, o "B alinhado" de P769 era resultado correcto para o caso sem dimensões, mas não generaliza para qualquer `fit`.

### Decisão

- Aplicar `fit` à transformação da imagem (cover/contain/stretch) — **feito em P770**.
- O `clip_path` de imagem é **scope-out** para passo futuro; requer alterações em `FrameItem::Image` e no exportador PDF.

---

## Parte B — Correcção da escala `PX_TO_PT`

### Código do vanilla

`lab/typst-original/crates/typst-library/src/visualize/image/mod.rs:406`:

```rust
pub const DEFAULT_DPI: f64 = 72.0;
```

`lab/typst-original/crates/typst-layout/src/image.rs` converte píxeis para pontos assumindo 72 DPI quando não há metadados de DPI na imagem.

### Alteração no cristalino

Em `01_core/src/rules/layout/image.rs`:

```rust
const PX_TO_PT: f64 = 1.0;  // 72 DPI padrão do vanilla: 1 px = 1 pt
```

Anteriormente era `0.75` (assumindo 96 DPI).

### Validação

| Caso | Vanilla | Cristalino |
|---|---|---|
| `#image("tiny.png")` | 100 × 80 pt | 100 × 80 pt |

---

## Testes

- `cargo test -p typst-core image` — 34 passaram.
- `cargo test -p typst-infra p307b_08_image_jpeg` — passou (snapshot não regressou; `tiny.jpg` é 1×1 px, por isso `fit` não altera o output).
- `cargo test --workspace` — todos passaram.

---

## Ficheiros alterados

- `00_nucleo/prompts/rules/layout-image.md` — L0 actualizado (escala 72 DPI, lógica `fit`, scope-out do clip).
- `01_core/src/rules/layout/image.rs` — implementação de `fit` e `PX_TO_PT = 1.0`; `@prompt-hash` `d774dfa0`.
- `00_nucleo/diagnosticos/paridade-producao-p770.md` — este relatório.

---

## Conclusão

P770 fecha as duas causas que lhe competiam:

1. A divergência de altura com `height:` explícito foi explicada pela aplicação de `ImageFit::Cover` pelo vanilla, o qual redimensiona a imagem fora do target e depois recorta (`clip_path`). O cristalino passou a aplicar a mesma lógica de `fit` na transformação da imagem.
2. A escala píxel→ponto foi corrigida para 72 DPI, igualando as dimensões de imagens sem `width`/`height` explícitos.

O alinhamento de `B` no caso `A #image(..., width: 2cm, height: 1.5cm) B` ainda diverge (2,834 pt) porque o cristalino ainda não emite o `clip_path` de imagem. Esse trabalho fica scope-out para um passo futuro.
