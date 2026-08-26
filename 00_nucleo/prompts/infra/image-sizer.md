# Prompt L0 — infra/image_sizer
Hash do Código: f7916d20

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
com `typst_library::visualize::image::raster` — `file:line` no bloco P1031
abaixo). O fallback 72 DPI é aplicado
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

> **Fonte de paridade (P1031)** — o `file:line` do vanilla que faltava. Vanilla ratificado
> (`e0e8ca4d`), `crates/typst-library/src/visualize/image/raster.rs:363-375`:
>
> ```rust
> /// Try to determine the DPI (dots per inch) of the image.
> ///
> /// This is guaranteed to be a positive value, or `None` if invalid or unspecified.
> fn determine_dpi(data: &[u8], exif: Option<&exif::Exif>) -> Option<f64> {
>     // Try to extract the DPI from the EXIF metadata. If that doesn't yield
>     // anything, fall back to specialized procedures for extracting JPEG or PNG
>     // DPI metadata. GIF does not have any.
>     exif.and_then(exif_dpi)
>         .or_else(|| jpeg_dpi(data))
>         .or_else(|| png_dpi(data))
>         .filter(|&dpi| dpi > 0.0)
> }
> ```
>
> **Citação literal** da prioridade **EXIF > JFIF APP0 > PNG `pHYs`** — a cadeia de
> `or_else` fixa exactamente essa ordem, e o comentário do vanilla nomeia-a por extenso.
> Detalhes das três fontes, também no vanilla:
>
> - `exif_dpi` — `raster.rs:378` e seguintes; lê a tag pelo `exif::In::PRIMARY` (IFD 0).
> - `jpeg_dpi` — `raster.rs:393-405`: valida `\xFF\xD8\xFF\xE0\0` no offset 0, `b"JFIF\0"`
>   no offset 6 e `\x01` (units == 1, DPI) no offset 11 — os mesmos três predicados que este
>   L0 descreve.
> - `png_dpi` — `raster.rs:423` e seguintes; decodifica em streaming e pára ao primeiro
>   `IDAT`, isto é, lê o `pHYs` sem descodificar píxeis — o mesmo requisito deste contrato.
> - **Positividade**: `.filter(|&dpi| dpi > 0.0)` — um DPI ≤ 0 é tratado como ausente.
>   Requisito que este L0 não regista explicitamente; vale como nota de implementação a
>   confirmar no lado do cristalino.
>
> **Fallback 72 DPI** — também literal: `crates/typst-library/src/visualize/image/mod.rs:431`
> (`pub const DEFAULT_DPI: f64 = 72.0;`) aplicado em `crates/typst-layout/src/image.rs:47`
> (`let dpi = image.dpi().unwrap_or(Image::DEFAULT_DPI);`). Nota: o vanilla usa **96.0**
> para SVG (`USVG_DEFAULT_DPI`, `mod.rs:434` e `:492`), não 72 — distinção que este L0 não
> faz e que só importa quando o sizer passar a tratar SVG.
>
> **Natureza**: literal para a prioridade, para os predicados de cada formato e para o
> fallback. Não é afirmação sobre a linguagem Typst (não há superfície de linguagem para
> prioridade de metadados) — é paridade de comportamento de infraestrutura, e a prova
> adequada é o `file:line` do vanilla, não `typst.app/docs`.

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
