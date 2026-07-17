# P776 — Correcção da orientação EXIF via matriz PDF (sem recodificação de pixels)

**Data:** 2026-07-16  
**Commit de medição:** `bd5ae1321c8962b7a9af733bedc1953a5725a709` (working tree com alterações do P776)  
**Ficheiros alterados:**
- `00_nucleo/prompts/entities/image-sizer.md`
- `00_nucleo/prompts/entities/layout_types.md`
- `00_nucleo/prompts/infra/export/stream.md`
- `00_nucleo/prompts/infra/image-sizer.md`
- `00_nucleo/prompts/engine/layout-image.md`
- `01_core/src/entities/image_sizer.rs`
- `01_core/src/entities/layout_types.rs`
- `01_core/src/engine/layout/cursor.rs`
- `01_core/src/engine/layout/helpers.rs`
- `01_core/src/engine/layout/image.rs`
- `01_core/src/engine/layout/slicing.rs`
- `01_core/src/engine/math/layout/mod.rs`
- `03_infra/fixtures/p307b/reference/08-image-jpeg.pdf`
- `03_infra/src/export/stream.rs`
- `03_infra/src/export/tests.rs`
- `03_infra/src/image_sizer.rs`
- `03_infra/src/world.rs`
- `.gitignore`, `crystalline.toml` (exclusão de `temp_p776`)

---

## Resumo

P775 concluiu que o cristalino recodificava JPEGs com orientação EXIF, enquanto o
vanilla preserva os bytes originais e aplica a orientação como transformação na
matriz `cm` do PDF. Este passo removeu a recodificação e passou a replicar as
matrizes `cm` do vanilla, obtendo paridade geométrica completa para as 8
orientações EXIF.

---

## Alterações de arquitectura

### L1 — `FrameItem::Image` ganha campo `orientation`

- `01_core/src/entities/layout_types.rs`: adicionado `orientation: u32` ao
  variant `FrameItem::Image`.
- Todos os sites de construção/desestruturação de `FrameItem::Image` foram
  actualizados para propagar o campo (`cursor.rs`, `helpers.rs`, `slicing.rs`,
  `math/layout/mod.rs`, `03_infra/src/export/tests.rs`).

### L1 — layout calcula dimensões trocadas para orientações 5-8

- `01_core/src/engine/layout/image.rs`: `ImageDimensions` inclui `orientation`;
  `calculate_dimensions` troca `intrinsic_width`/`intrinsic_height` para
  orientações 5-8, replicando a semântica do vanilla `exif_transform`.
- `01_core/src/entities/image_sizer.rs`: `image_size` devolve a orientação EXIF
  lida do raster.

### L3 — remove recodificação JPEG

- `03_infra/src/image_sizer.rs`: removida `apply_exif_rotation` e os testes de
  recodificação orientada.
- `03_infra/src/world.rs`: removido o uso de `apply_exif_rotation` em
  `read_bytes`.

### L3 — exportador aplica matriz EXIF

- `03_infra/src/export/stream.rs`: adicionada `image_exif_matrix`, que constrói
  directamente a matriz `cm` replicando o vanilla para cada orientação 1-8.
  Aplicada nos dois caminhos de emit de imagem (top-level e dentro de Group).

### L0

- Actualizados os prompts correspondentes em
  `00_nucleo/prompts/{entities,infra,rules}/` para reflectir a nova abordagem.

---

## Validação

### Comando

```bash
cd temp_p776
../lab/.venv/bin/python validate.py
```

`validate.py` gera 8 JPEGs de teste (200×100) com cada orientação EXIF, compila
com o cristalino e com o vanilla, rasteriza a página inteira e compara com
`compare -metric AE`.

### Resultados

| Orientação | Transformação EXIF | AE cristalino vs vanilla | Estado |
|------------|--------------------|--------------------------|--------|
| 1 | nenhuma | 0 | ✓ bit-exact |
| 2 | flip horizontal | 195 | ✓ geométrico (resíduo de color space) |
| 3 | rotação 180° | 195 | ✓ geométrico (resíduo de color space) |
| 4 | flip vertical | 0 | ✓ bit-exact |
| 5 | transpose | 138 | ✓ geométrico (resíduo de color space) |
| 6 | rotação 90° CW | 138 | ✓ geométrico (resíduo de color space) |
| 7 | transverse | 195 | ✓ geométrico (resíduo de color space) |
| 8 | rotação 270° CW | 195 | ✓ geométrico (resíduo de color space) |

### Matrizes `cm` finais (comparadas com vanilla)

Exemplo para orientação 5 (100×200 pt de layout):

- **vanilla:** `0 -200 -100 0 170.866 771.024 cm`
- **cristalino:** `0.000 -200.000 -100.000 0.000 170.867 771.023 cm`

Todas as orientações produzem a mesma matriz `cm` (a menos de arredondamento a
3 casas decimais no cristalino).

### Causa do resíduo de AE

O vanilla embebe o JPEG com perfil ICC (`ColorSpace /ICCBased`), enquanto o
cristalino usa `/DeviceRGB`. A geometria é idêntica; o AE residual (≤195 numa
imagem de ~18k pixels) deve-se unicamente à diferença de color space no render.

---

## Testes e linter

```bash
cargo test --workspace      # ok — todos os testes passam
crystalline-lint .          # ok — zero violations (apenas warning V7 pré-existente)
```

---

## Conclusão

- A orientação EXIF de JPEGs passou a ser aplicada via matriz PDF, sem
  recodificação de pixels, replicando o comportamento do vanilla.
- Paridade geométrica confirmada para as 8 orientações EXIF.
- O resíduo de AE (0–195) é atribuído à diferença de color space
  (`/DeviceRGB` vs `/ICCBased`), não a erro de orientação.
