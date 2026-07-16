# Prompt L0 — infra/image_sizer
Hash do Código: 828449ef

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
}
```

Delega `size` para `imagesize::blob_size(data)`, convertendo `ImageSize` para
`(u32, u32)`.

Delega `dpi` para parsing manual dos metadados da imagem, sem descodificar
píxeis. A prioridade de leitura é **EXIF > JFIF APP0 > PNG `pHYs`** (paridade
com `typst_library::visualize::image::raster`). O fallback 72 DPI é aplicado
pelo consumidor em L1.

### Fontes de DPI

- **EXIF**: segmento APP1 em JPEG (`Exif\0\0`) ou chunk `eXIf` em PNG.
  Lê a tag `XResolution` (0x011A) do IFD 0, suportando TIFF little-endian
  (`II`) e big-endian (`MM`).
- **JFIF APP0**: segmento APP0 com identificador `JFIF\0` e `units == 1`
  (DPI); usa `Xdensity`.
- **PNG `pHYs`**: chunk `pHYs` com `unit == 1` (metro) ou `unit == 2`
  (centímetro); converte pixels por unidade para DPI.

## Invariantes

- `imagesize` declarado apenas em `03_infra/Cargo.toml`.
- `ImageSizeImageSizer` implementa o trait L1 — não adiciona API própria.
- Parsing de metadados não usa crates EXIF externas; depende apenas de
  `imagesize` e da stdlib.
