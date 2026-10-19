# Relatório de Paridade — P772

**Passo:** 772 (lote 2)  
**Data:** 2026-07-16  
**Foco:** varredura de `lacuna-inventario` — identificar o próximo módulo por tamanho e classificar os seus itens.

---

## Recontagem por módulo

Comando:

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn | head -15
```

Resultado:

| Módulo | Itens | Estado |
|---|---|---|
| `typst_library::foundations::calc` | 45 | Tratado em P765a |
| `typst_library::math::style` | 32 | Tratado em P765b |
| `typst_library::layout::grid::resolve` | 24 | Infra-estrutura Rust (resolução interna de grid) |
| `typst_library::diag` | 24 | Tratado |
| `typst_library::foundations::ops` | 20 | Tratado em P765a |
| `typst_utils` | 14 | Infra-estrutura Rust |
| **`typst_library::visualize::image::raster`** | **13** | **Próximo módulo analisado neste passo** |
| `typst_library::pdf::accessibility` | 12 | Infra-estrutura Rust |
| `typst_syntax::span` | 10 | Infra-estrutura Rust |
| `typst_syntax::package` | 8 | Infra-estrutura Rust |

---

## Módulo escolhido: `typst_library::visualize::image::raster`

### Itens listados em `lacuna-inventario`

| # | Item | Tipo | Classificação |
|---|---|---|---|
| 1 | `ExchangeFormat` | enum | Infra-estrutura Rust — formato interno de intercâmbio (PNG/JPG/GIF/WebP), não símbolo de língua. |
| 2 | `PixelEncoding` | enum | Infra-estrutura Rust — codificação de canal para dados raw de pixeis, não símbolo de língua. |
| 3 | `PixelFormat` | struct | Infra-estrutura Rust — descritor de buffer raw de pixeis, não símbolo de língua. |
| 4 | `RasterFormat` | enum | Infra-estrutura Rust — discrimina exchange vs pixel, não símbolo de língua. |
| 5 | `RasterImage` | struct | Infra-estrutura Rust — representação interna de imagem descodificada. |
| 6 | `RasterImageInner` | struct | Infra-estrutura Rust — representação interna. |
| 7 | `apply_rotation` | fn | Infra-estrutura Rust — aplica rotação EXIF aos pixels descodificados. |
| 8 | `determine_dpi` | fn | Infra-estrutura Rust — extrai DPI de EXIF/JFIF/PNG. |
| 9 | `exif_dpi` | fn | Infra-estrutura Rust — helper de `determine_dpi`. |
| 10 | `exif_rotation` | fn | Infra-estrutura Rust — lê tag EXIF Orientation. |
| 11 | `format_image_error` | fn | Infra-estrutura Rust — formata mensagem de erro de descodificação. |
| 12 | `jpeg_dpi` | fn | Infra-estrutura Rust — extrai DPI do segmento JFIF APP0. |
| 13 | `png_dpi` | fn | Infra-estrutura Rust — extrai DPI do chunk pHYs do PNG. |

### Justificação da classificação

O módulo `image::raster` é, na sua totalidade, infra-estrutura Rust de descodificação de imagens. Nenhum dos seus tipos ou funções é exposto directamente à linguagem Typst — o utilizador não escreve `ExchangeFormat`, `PixelFormat`, `apply_rotation`, etc. São detalhes de implementação chamados internamente por `image()`.

A exposição indirecta à linguagem acontece através de:
- Suporte a formatos de ficheiro: GIF, WebP, PNG, JPEG.
- Suporte a dados raw de pixeis via argumento `format`.
- Aplicação de rotação EXIF.
- Leitura de metadados de DPI.

Essas capacidades estão em falta ou são parciais no cristalino, mas não constituem *bugs* no sentido de comportamento incorrecto de um símbolo já migrado — são funcionalidades ainda não implementadas. Corrigir qualquer uma delas exigiria adicionar suporte a formatos/decodificação/DPI/EXIF, o que ultrapassa o escopo de um passo de varredura de stdlib e não se enquadra na regra de correcção de "bugs reais" (comportamento incorrecto de símbolos de língua já existentes).

### Comparação com o cristalino

- **Cristalino (`03_infra/src/export/images.rs`)**: detecta JPEG/PNG por magic bytes, descodifica PNG com a crate `image`, passa JPEG raw para o PDF. Não extrai DPI, não aplica rotação EXIF, não suporta GIF/WebP/pixeis raw.
- **Vanilla (`lab/typst-original/crates/typst-library/src/visualize/image/raster.rs`)**: suporta PNG/JPG/GIF/WebP e pixeis raw; aplica rotação EXIF; extrai DPI de EXIF/JFIF/PNG.

### Decisão

Nenhuma correcção implementada neste passo. Todos os 13 itens são classificados como **infra-estrutura Rust / lacuna de funcionalidade**, não como bugs reais de língua.

---

## Próximo passo

P772a (lote 3) deve avançar para o módulo seguinte por tamanho ainda não classificado. Com base na recontagem actual, os próximos candidatos são:

- `typst_library::pdf::accessibility` (12 itens) — provavelmente infra-estrutura Rust.
- `typst_syntax::span` (10 itens) — infra-estrutura Rust.
- `typst_syntax::package` (8 itens) — infra-estrutura Rust.

Se todos continuarem a classificar-se como infra-estrutura, a série de varredura pode convergir rapidamente para o fim da lista `lacuna-inventario`.

---

## Validação

- `cargo test --workspace` — todos passaram (sem alterações de código).
- `crystalline-lint .` — 0 violações (apenas V7 esperado, prompt órfão).

---

## Ficheiros alterados

- `00_nucleo/diagnosticos/paridade-producao-p772.md` — este relatório.
