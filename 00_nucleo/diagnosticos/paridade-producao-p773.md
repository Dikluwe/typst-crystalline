# P773 — Leitura de DPI real de metadados de imagem

**Data:** 2026-07-16  
**Commit de medição:** `06f2a670e01278458757c85e72fc8f9af38082c9`  
**Ficheiros alterados:**
- `00_nucleo/prompts/entities/image-sizer.md`
- `00_nucleo/prompts/infra/image-sizer.md`
- `00_nucleo/prompts/rules/layout-image.md`
- `01_core/src/entities/image_sizer.rs`
- `01_core/src/rules/layout/image.rs`
- `03_infra/src/image_sizer.rs`

---

## Resumo

P770 fixou a conversão px→pt para 1 px = 1 pt (72 DPI fallback), o que estava
correcto para imagens sem metadados de DPI. No entanto, o vanilla usa o DPI
real quando disponível (EXIF, JFIF APP0, PNG `pHYs`). Imagens reais com DPI
não-padrão renderizavam ampliadas no cristalino.

Este passo confirma a divergência, implementa a leitura dos três metadados de
DPI e integra-os no cálculo de dimensões de `Content::Image`. A paridade com o
vanilla foi verificada para PNG e JPEG a 300 DPI, e o caso sem metadados
(`tiny.png`, P770) não regrediu.

---

## Hipótese e confirmação

**Hipótese:** vanilla aplica `pt = px * (72.0 / dpi)` usando o DPI dos
metadados; cristalino usava `pt = px * 1.0` para todas as imagens.

**Prova:** geraram-se imagens de teste 200×160 px a 300 DPI:

```bash
convert -size 200x160 xc:lightblue -density 300 /tmp/p773-dpi300.png
convert -size 200x160 xc:lightblue -density 300 /tmp/p773-dpi300.jpg
```

Medição via `mutool draw -F trace`:

| Formato | Vanilla (transform) | Cristalino antes | Cristalino depois |
|---------|---------------------|------------------|-------------------|
| PNG     | `48.000097 × 38.400079` | `200 × 160` | `48 × 38.4` |
| JPG     | `48 × 38.4` | `200 × 160` | `48 × 38.4` |

A fórmula `200 px * (72 / 300) = 48 pt` explica as dimensões do vanilla.
A pequena diferença no PNG (`48.000097`) vem do arredondamento de pixels por
metro (11811 ppm → 299.9994 DPI) no vanilla; o cristalino usa o mesmo valor
de ppm e obtém 48 pt exactos.

---

## Implementação

### L1 — contrato (`01_core/src/entities/image_sizer.rs`)

O trait `ImageSizer` ganhou o método:

```rust
fn dpi(&self, data: &[u8]) -> Option<f64>;
```

`NullImageSizer` retorna `None`, forçando o fallback 72 DPI nos testes L1.

### L3 — parsing (`03_infra/src/image_sizer.rs`)

`ImageSizeImageSizer::dpi` delega para `determine_dpi`, com prioridade:

1. **EXIF** — segmento APP1 em JPEG (`Exif\0\0`) ou chunk `eXIf` em PNG.
   Lê `XResolution` (tag `0x011A`) do IFD 0, suportando TIFF LE/BE.
2. **JFIF APP0** — segmento APP0 com `units == 1` (DPI), usa `Xdensity`.
3. **PNG `pHYs`** — chunk `pHYs` com `unit == 1` (metro) ou `unit == 2`
   (centímetro), converte para DPI.

Nenhuma crate EXIF externa foi adicionada; o parsing é manual sobre os bytes
do cabeçalho.

### L1 — layout (`01_core/src/rules/layout/image.rs`)

`calculate_dimensions` passou a usar:

```rust
let dpi = sizer.dpi(data).unwrap_or(DEFAULT_DPI); // 72.0
let px_to_pt = 72.0 / dpi;
```

A constante `PX_TO_PT = 1.0` foi substituída por `DEFAULT_DPI = 72.0`.

---

## Testes

- `cargo test --workspace` — verde.
- `crystalline-lint .` — zero violações (apenas V7 pré-existente, prompt
  órfão `package_version_resolution.md`).

Testes unitários novos em `03_infra/src/image_sizer.rs`:
- `png_1x1_sem_dpi` — imagem PNG sem metadados retorna `None`.
- `png_1x1_com_phys_300dpi` — chunk `pHYs` a 11811 ppm → ~300 DPI.
- `jpeg_jfif_300dpi_inline` — APP0 JFIF com Xdensity=300 → 300 DPI.
- `imagens_reais_300dpi_se_existirem` — valida `/tmp/p773-dpi300.png` e
  `/tmp/p773-dpi300.jpg` quando presentes.
- `exif_dpi_inline_le` / `exif_dpi_inline_be` — TIFF LE/BE com
  `XResolution = 300/1`.

Testes de regressão em `01_core/src/rules/layout/image.rs` mantiveram-se e
continuam a usar fallback 72 DPI (os `MockSizer` retornam `None` em `dpi`).

---

## Validação end-to-end

Documento `test.typ`:

```typst
#image("p773-dpi300.png")
#image("p773-dpi300.jpg")
```

Resultado `mutool draw -F trace`:

```text
Cristalino:
<fill_image ... transform="48 0 0 38.4 ..." width="200" height="160"/>
<fill_image ... transform="48 0 0 38.4 ..." width="200" height="160"/>

Vanilla:
<fill_image ... transform="48.000097 0 0 38.400079 ..." width="200" height="160"/>
<fill_image ... transform="48 0 0 38.4 ..." width="200" height="160"/>
```

Regressão P770 (`tiny.png`, sem metadados):

```text
Cristalino: transform="100 0 0 80 ..."
Vanilla:    transform="100 0 0 80 ..."
```

---

## Notas e próximos passos

- **Rotação EXIF** (`Orientation` tag `0x0112`) não foi tratada neste passo.
  Fica como achado separado; o P773 limitou-se ao DPI.
- O parsing EXIF cobre apenas `XResolution` do IFD 0. Casos raros (resolução
  apenas no IFD de thumbnail, ou `ResolutionUnit` != inch) podem necessitar de
  extensão futura.
