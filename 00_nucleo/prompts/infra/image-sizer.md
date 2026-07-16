# Prompt L0 — infra/image_sizer
Hash do Código: ecad9fbf

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/image_sizer.rs`
**ADRs relevantes**: ADR-0029 (pureza física — I/O em L3)

## Contexto

Implementação de `ImageSizer` usando a crate `imagesize`. Lê apenas o cabeçalho
do ficheiro (não descodifica píxeis) — eficiente para grandes imagens.

A crate `imagesize` não entra em L1 — V14 não dispara. Pertence apenas ao
`Cargo.toml` de L3.

## Tipos públicos

```rust
pub struct ImageSizeImageSizer;

impl ImageSizer for ImageSizeImageSizer {
    fn size(&self, data: &[u8]) -> Option<(u32, u32)>;

    fn dpi(&self, data: &[u8]) -> Option<f64>;

    fn orientation(&self, data: &[u8]) -> Option<u32>;
}
```

Delega `size` para `imagesize::blob_size(data)`, convertendo `ImageSize` para
`(u32, u32)`.

Delega `dpi` para parsing manual dos metadados da imagem, sem descodificar
píxeis. A prioridade de leitura é **EXIF > JFIF APP0 > PNG `pHYs`** (paridade
com `typst_library::visualize::image::raster`). O fallback 72 DPI é aplicado
pelo consumidor em L1.

Delega `orientation` para parsing manual do segmento EXIF (APP1 em JPEG,
chunk `eXIf` em PNG), lendo a tag `Orientation` (`0x0112`) do IFD 0,
suportando TIFF little-endian (`II`) e big-endian (`MM`).

### Fontes de DPI

- **EXIF**: segmento APP1 em JPEG (`Exif\0\0`) ou chunk `eXIf` em PNG.
  Lê a tag `XResolution` (0x011A) do IFD 0, suportando TIFF little-endian
  (`II`) e big-endian (`MM`).
- **JFIF APP0**: segmento APP0 com identificador `JFIF\0` e `units == 1`
  (DPI); usa `Xdensity`.
- **PNG `pHYs`**: chunk `pHYs` com `unit == 1` (metro) ou `unit == 2`
  (centímetro); converte pixels por unidade para DPI.

### Rotação EXIF (P776)

A orientação EXIF **não** é aplicada aos pixels neste módulo. O contrato
`ImageSizer::orientation` apenas lê o valor da tag `Orientation` (0x0112) e
entrega-o a L1. A transformação visual é aplicada mais tarde, no exportador
PDF, através da matriz `cm` do operador `Do` — replicando o mecanismo do
vanilla (`typst-pdf/src/image.rs::exif_transform`), que preserva os bytes
originais do JPEG em vez de os recodificar.

O mapeamento dos valores 1-8 (semântica EXIF):
- 1: inalterado
- 2: flip horizontal
- 3: rotação 180°
- 4: flip vertical
- 5: flip horizontal + rotação 270° (transpose)
- 6: rotação 90°
- 7: flip horizontal + rotação 90° (transverse)
- 8: rotação 270°

## Invariantes

- `imagesize` declarado apenas em `03_infra/Cargo.toml`.
- `ImageSizeImageSizer` implementa o trait L1 — não adiciona API própria.
- Parsing de metadados não usa crates EXIF externas; depende apenas de
  `imagesize` e da stdlib.
- Este módulo **não** descodifica nem recodifica pixels para aplicar orientação
  EXIF; os bytes originais do JPEG/PNG são preservados.
