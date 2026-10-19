# Relatório — typst-passo-833: `image::raster` — GIF/WebP ausentes (#17) e PNG corrompido omitido silenciosamente (#18, GRAVE)

**Data:** 2026-07-22
**Executor:** Kimi Code (agente principal — prompt lido de `00_nucleo/materialization/typst-passo-833.md`).
**Proveniência das medições:** commit HEAD `8f326781d` (P831+P832) + alterações deste passo na working tree (14 ficheiros modificados, ver `git status` no fim). Medição "antes" com o binário release de P832; "depois" com o binário rebuildado após a implementação. Fixtures: `temp/p831/` (assets de P831) e `temp/p832/` (JPEG corrompido da sonda).
**Baseline da suíte (antes):** `cargo test -p typst-core` → **4522 passed; 0 failed** (medido em P832); `typst-infra` → **672 passed; 0 failed** (medido em P832).

---

## Achado #18 — GRAVE: imagem corrompida omitida silenciosamente, exit 0

### Medição antes (saída literal)

```text
#image("raster5_corrupto.png")  (assinatura PNG válida + corpo lixo)
  cris: exit 0 — PDF válido de 2610 bytes, página em branco;
        stderr interno: "PNG inválido — imagem omitida: Falha ao descodificar imagem: Format error decoding Png: ..."
  van:  exit 1 — error: failed to decode image (Format error decoding Png: ChunkType { type: AGE-, ... } chunk appeared before IHDR chunk)

#image("corrupto.jpg")  (JPEG truncado)
  cris: exit 0 — PDF com o JPEG cru embutido (inválido)
  van:  exit 1 — error: failed to decode image (Format error decoding Jpeg: I/O errors Not enough bytes, expected 1 but found 0)
```

### Código identificado

- O swallow estava em `03_infra/src/export/images.rs:348` — braço `Err(e)` de `process_png_for_pdf` fazia `eprintln!` e **não inseria a imagem**, seguindo o export (comportamento "omitir e seguir").
- JPEG nem sequer era descodificado: embutido cru (`/DCTDecode`), logo JPEG corrompido produzia PDF inválido em silêncio.
- Vanilla: descodifica tudo em avaliação (`RasterImage::new` → `format_image_error`, `lab/.../visualize/image/raster.rs:448-453`) e rejeita o documento.

### Decisão de implementação

O erro não podia subir por L1 (a whitelist de dependências de L1 não permite a crate `image`; o L0 `entities/image-format.md` §Decisão de âmbito já registava esta restrição). A propagação foi feita em **L3**, no pipeline:

- Nova função `validate_document_images(&PagedDocument) -> Result<(), String>` (`03_infra/src/export/images.rs`) — percorre todas as imagens do documento (recursivo em `Group`/`Link`, mesmo critério de `scan_all_images`) e descodifica cada uma: PNG/GIF/WebP via `process_png_for_pdf`, JPEG por descodificação completa (nova — antes nunca descodificado). Erro embrulhado no formato do vanilla: `failed to decode image ({detalhe})`.
- `03_infra/src/pipeline.rs` — chamada logo após o shaping, antes do dispatch de fontes; `Err(msg)` → `(Err(vec![SourceDiagnostic::error(Span::detached(), msg)]), warnings)` — o CLI imprime o erro e sai com exit 1, como qualquer outro erro de compilação.
- `process_png_for_pdf`: mensagem interna passa a ser o detalhe cru do decoder (era "Falha ao descodificar imagem: {e}") — o envelope vanilla é posto pelo caller.
- O `eprintln!` de `scan_all_images` fica como **fallback defensivo** (com a mensagem no formato vanilla), inalcançável no caminho do CLI — só para callers da API pública de export que saltem a validação.
- Não foram alteradas as assinaturas da API pública de export (~250 call sites em testes) — a interrupção acontece no pipeline, antes do export.

### Medição depois (saída literal, exit codes reais)

```text
pngbad: cris=1 van=1
  cris: ...p833_pngbad.typ:<detached>: error: failed to decode image (Format error decoding Png: ChunkType { type: AGE-, critical: true, private: false, reserved: false, safecopy: true } chunk appeared before IHDR chunk)
  van:  error: failed to decode image (Format error decoding Png: ChunkType { type: AGE-, ... } chunk appeared before IHDR chunk)  ← idêntico, verbatim
jpgbad: cris=1 van=1
  cris: ...<detached>: error: failed to decode image (unexpected end of file)
  van:  error: failed to decode image (Format error decoding Jpeg: I/O errors Not enough bytes, expected 1 but found 0)
pngok:  cris=0 van=0  (regressão: PNG válido continua a compilar)
```

**Nuance registada (2):** (a) o span é `<detached>` — o vanilla aponta a posição do `#image(...)`; o erro sobe do pipeline L3, sem span de avaliação (mesma família da nuance de P781/P772s). (b) O detalhe interno do erro JPEG diverge — vem do decoder da crate `image` 0.24 (cristalino) vs o decoder do vanilla (image 0.25/zune); o envelope `failed to decode image (...)` e o comportamento (exit 1) batem. O detalhe PNG é verbatim idêntico (ambos acabam na crate `png`).

### Efeito colateral necessário — regeneração do fixture P307b-08 (golden)

A suite `p307b_snapshot_tests::p307b_08_image_jpeg` falhou após a correcção: o fixture `03_infra/fixtures/p307b/sources/tiny.jpg` (558 bytes, "mínimo válido" de P307a.2) tem **DQT em falta para o componente Cb** — medido: **o vanilla também o rejeita** (`error: failed to decode image (Format error decoding Jpeg: Error parsing DQT segment. Reason:No quantization table for component Cb)`, exit 1). O fixture nunca foi compilável pelo vanilla; a referência binária só existia porque o cristalino embutia JPEG cru sem validar.

Acção (protocolo de regeneração do L0 `infra/export-fixtures.md` §"Protocolo de regeneração" — "mudança justificada num passo dedicado", este): `tiny.jpg` substituído por um JPEG 1×1 válido (Pillow, 634 bytes — **verificado: o vanilla compila-o, exit 0**), referência `reference/08-image-jpeg.pdf` regenerada com `CRYSTALLINE_PDF_FIXED_EPOCH=0` (2 invocações byte-idênticas verificadas por `diff`): 3497 bytes, md5 `d4a62272`. `MANIFEST.md` actualizado (tabela + nota + total). **Regeneração de golden — assinalada ao dono para revisão.**

## Achado #17 — GIF e WebP não suportados

### Medição antes

`#image("raster3.gif")` / `#image("raster7.webp")` → cris `error: unknown image format` (exit 1); van compila (GIF estático no frame 1).

### Código identificado / decisão de dependência

- Cristalino: `01_core/src/entities/image_format.rs` — enum só `Jpeg|Png|Unknown`.
- Vanilla: `raster.rs:80-81,248-251` (`GifDecoder`/`WebPDecoder` da crate `image`).
- **A crate `image` 0.24 já era dependência de L3** (`03_infra/Cargo.toml`, features `png`/`jpeg`) — o passo manda implementar "usando a mesma crate/abordagem do vanilla se possível"; bastou activar as features `gif`/`webp` (decoders pequenos, sem motor novo). Não foi caso de decisão de escopo por peso de dependência.

### Diff

- `01_core/src/entities/image_format.rs` — variantes `Gif` (`GIF87a`/`GIF89a`) e `WebP` (`RIFF`+`WEBP` no offset 8) + 2 testes de detecção.
- `03_infra/Cargo.toml` — features `gif`, `webp` na crate `image` (Cargo.lock actualizado).
- `03_infra/src/export/images.rs` — braços `Gif`/`WebP` em `scan_all_images` reutilizando `process_png_for_pdf` (sempre foi formato-genérica via `image::load_from_memory`; nome histórico, documentado). GIF descodifica o primeiro frame (estático) — paridade vanilla. Intrinsic sizes via `imagesize` (já suporta GIF/WebP).
- L0s actualizados: `entities/image-format.md` (enum, extensão P833, lacuna fechada), `infra/export/images.md` (formatos, `validate_document_images`), `engine/stdlib/figure_image.md` (item 4 + parágrafo de divergência revistos). `crystalline-lint --fix-hashes` → 3 headers actualizados.

### Medição depois (exit codes reais + texto extraído)

```text
gif:  cris=0 van=0 — ambos: "Depois do GIF."
webp: cris=0 van=0 — ambos: "Depois do WebP."
```

## Validação

- `cargo test -p typst-core` → **4524 passed; 0 failed**; 2 ignored. Cálculo: 4522 (baseline P832) + 2 testes novos (`detecta_gif`, `detecta_webp`) = 4524. ✔
- `cargo test -p typst-infra` → **678 passed; 0 failed**; 5 ignored. Cálculo: 672 + 6 testes novos (`p833_validate_png_corrompido_erro_formato_vanilla`, `p833_validate_jpeg_corrompido_erro`, `p833_validate_png_valido_ok`, `p833_validate_gif_ok_e_descodifica`, `p833_validate_webp_ok_e_descodifica`, `p833_validate_imagem_dentro_de_group`) = 678. ✔
- `crystalline-lint .` → **exit 0, zero violations** (3 headers re-hasheados).
- Snapshot P307b: 9/9 fixtures verdes após regeneração do 08 (ver secção acima).

## Ficheiros alterados (para o commit)

`00_nucleo/prompts/engine/stdlib/figure_image.md`, `00_nucleo/prompts/entities/image-format.md`, `00_nucleo/prompts/infra/export/images.md`, `01_core/src/engine/stdlib/figure_image.rs`, `01_core/src/entities/image_format.rs`, `03_infra/Cargo.toml`, `Cargo.lock`, `03_infra/fixtures/p307b/{MANIFEST.md,reference/08-image-jpeg.pdf,sources/tiny.jpg}`, `03_infra/src/export/{images.rs,mod.rs,tests.rs}`, `03_infra/src/pipeline.rs`, este relatório.
