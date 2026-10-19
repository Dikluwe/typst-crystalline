# Relatório de Paridade — P777

**Data:** 2026-07-16
**Passo:** P777
**Objetivo:** Corrigir o color space de JPEGs RGB para `/ICCBased` sRGB, eliminando a diferença de render face ao vanilla deixada pelo P776.

---

## Estado base da medição

- **Commit base:** `6ba0a1042e6787da0d4be5f8f14850b7f58a3132`
- **Estado:** working tree com alterações não commitadas (lista em `git diff HEAD --stat` abaixo).
- **Comando de teste:** `cargo test --workspace --release` — 644 passed, 0 failed, 5 ignored.
- **Linter:** `crystalline-lint .` — zero violações relacionadas com esta mudança (único warning pré-existente V7 sobre `package_version_resolution.md`, não afeta P777).

```text
 00_nucleo/prompts/infra/export/builder.md          |  30 +++-
 00_nucleo/prompts/infra/export/images.md           |  13 +-
 00_nucleo/prompts/infra/export/stream.md           |   6 +-
 03_infra/fixtures/p307b/reference/08-image-jpeg.pdf | Bin 2961 -> 3421 bytes
 03_infra/src/export/builder.rs                     |  87 +++++++++---
 03_infra/src/export/images.rs                      | 150 +++++++++++++++++----
 03_infra/src/export/mod.rs                         |   7 +-
 03_infra/src/export/stream.rs                      |   7 +-
 03_infra/src/export/tests.rs                       |   4 +-
 9 files changed, 243 insertions(+), 61 deletions(-)
```

---

## Alterações implementadas

### 1. `03_infra/src/export/images.rs`

- Adicionada constante `SRGB_ICC_PROFILE` (480 bytes, perfil ICC sRGB extraído do PDF vanilla).
- Adicionada `build_icc_profile_stream` que emite o stream `/ICCBased` com:
  - `/N 3`
  - `/Range [0 1 0 1 0 1]`
  - `Length` igual ao tamanho comprimido.
- `ImageXObject::Jpeg` agora transporta `icc_profile_id: Option<usize>`.
- `scan_all_images` e `build_jpeg_xobject` associam/emitem `/ColorSpace [/ICCBased {id} 0 R]` para JPEGs RGB.
- JPEGs grayscale e CMYK mantêm `/DeviceGray` e `/DeviceCMYK`, respetivamente.

### 2. `03_infra/src/export/builder.rs`

- Reserva e emite o objeto ICC profile nos 3 caminhos de geração de PDF (Helvetica, CIDFont, Multifont).

### 3. `03_infra/src/export/stream.rs`

- Aumentada a precisão da matriz `cm` de imagens de 3 para 5 casas decimais.

### 4. L0s

- `00_nucleo/prompts/infra/export/images.md` — atualizado para refletir `/ICCBased` para JPEGs RGB.
- `00_nucleo/prompts/infra/export/stream.md` — atualizado para refletir a precisão da matriz `cm`.
- `00_nucleo/prompts/infra/export/builder.md` — atualizado para incluir o ICC profile object.
- Hashes recalculados com `crystalline-lint --fix-hashes .` — zero drift warnings.

### 5. Testes / snapshots

- `03_infra/src/export/tests.rs` — `pipeline_jpeg_usa_jpeg_color_space` agora espera `/ICCBased` em vez de `/DeviceRGB`.
- `03_infra/fixtures/p307b/reference/08-image-jpeg.pdf` — regenerado com `UPDATE_P307B_SNAPSHOTS=1` (tamanho 2961 B → 3421 B, devido ao perfil ICC adicionado).

---

## Resultados de validação contra vanilla

Validação corrida em `temp_p776/` com `../lab/.venv/bin/python validate.py`.

| Orientação | Color space cristalino | Color space vanilla | AE residual |
|-----------|------------------------|---------------------|-------------|
| 1         | `icc`                  | `icc`               | 0           |
| 2         | `icc`                  | `icc`               | 195         |
| 3         | `icc`                  | `icc`               | 195         |
| 4         | `icc`                  | `icc`               | 0           |
| 5         | `icc`                  | `icc`               | 138         |
| 6         | `icc`                  | `icc`               | 138         |
| 7         | `icc`                  | `icc`               | 195         |
| 8         | `icc`                  | `icc`               | 195         |

### Análise do resíduo

- O color space está agora em paridade total com o vanilla (`icc` em todas as orientações).
- O resíduo de AE (0–195) **não** é causado pelo perfil ICC:
  - Os bytes do perfil ICC emitidos são idênticos aos do PDF vanilla.
  - `mutool draw` sobre o cristalino vs vanilla dá AE=0, confirmando que a diferença não está no decoding da imagem.
- A causa residual é uma diferença de sub-pixel na matriz `cm` das imagens, originada por imprecisão nas unidades de página/margem do cristalino:
  - Cristalino (`PageConfig::default()`): `width=595.28`, `height=841.89`, margem calculada via `min*2.5/21`.
  - Vanilla: `width=595.2756`, `height=841.8898`, margem `70.86614 pt` (2.5 cm exato).
- Corrigir isto exige alterar `01_core/src/entities/layout_types.rs`, o que está fora do scope do P777 e tem impacto alargado em snapshots existentes.

---

## Decisões

1. **Aceitar `/ICCBased` sRGB para JPEGs RGB** — paridade de color space com o vanilla atingida.
2. **Manter a precisão 5 casas decimais na matriz `cm`** — melhoria legítima, não regressão funcional; único impacto foi o snapshot JPEG, já regenerado.
3. **Documentar o resíduo de AE como fora de scope do P777** — a causa é a precisão das unidades de página em L1, não o color space.

---

## Resíduos

- AE residual 0–195 em imagens JPEG com orientações EXIF 2, 3, 5, 6, 7, 8.
- Causa raiz: diferença de sub-pixel na matriz `cm` devido a unidades de página/margem do cristalino vs vanilla.
- Próximo passo sugerido: passo dedicado à alinhamento das dimensões de página `PageConfig::default()` com o vanilla (`595.2756 × 841.8898 pt` e margem 2.5 cm exato), avaliando o impacto nos snapshots existentes.
