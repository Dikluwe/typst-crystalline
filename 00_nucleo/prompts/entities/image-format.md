# Prompt L0 — entities/image_format
Hash do Código: 5f6b3ebb

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/image_format.rs`
**ADRs relevantes**: ADR-0107 (paridade de língua, não mecânica), ADR-0108 (medir antes de decidir)
**Criado em**: 2026-07-17 (P772p)

---

## Contexto

P772k/P650 confirmaram que `#image()` com formato não reconhecido ou corrompido
é silenciosamente omitido do PDF (`eprintln!` em L3, sem erro de compilação),
enquanto o vanilla trata isto como erro (`unknown image format` — sonda P772p,
`typst_library::visualize::image::mod.rs:344`). A causa é estrutural: a única
detecção de formato do cristalino vivia em `03_infra/src/export/images.rs`
(`detect_format`/`ImageFormat`, `pub(super)`), tarde demais — exportação PDF,
sem caminho para devolver `SourceDiagnostic` amarrado ao span do `#image()`
original.

Este módulo **move** essa detecção (sem duplicar) para L1: é pura detecção de
assinatura binária (magic bytes) — zero I/O, zero dependência externa, portanto
100% legal em L1 sem precisar de injecção via trait (ao contrário de
`ImageSizer`, que precisa da crate `imagesize`). `03_infra/src/export/images.rs`
importa este tipo em vez de manter uma cópia local (ver `infra/export/images.md`).

### Decisão de âmbito (sonda P772p — registada antes de implementar)

O vanilla **decodifica a imagem inteira** neste ponto (`RasterImage::new_impl`,
`image::DynamicImage::from_decoder`), não só o cabeçalho. Replicar isso em L1
exigiria a crate `image` (ou equivalente) em L1 — **viola directamente a pureza
de L1** (zero dependências externas de I/O/decoding, `CLAUDE.md`). Este módulo
faz **só detecção de assinatura** (magic bytes), a opção mais barata da tabela
de decisão do passo — mesmo o vanilla não fazendo assim. A lacuna mais funda
(imagem com assinatura reconhecida mas payload corrompido a meio) ficou aberta
até **P833 (#18)**: passou a ser apanhada em **L3** por
`validate_document_images` (`03_infra/src/export/images.rs`), chamada no
pipeline antes do export — a compilação falha com a mensagem do vanilla
(`failed to decode image ({detalhe})`, span detached — nuance: sem a posição
do `#image(...)`, que o vanilla aponta). Ver `infra/export/images.md`.

### Extensão de formatos (P833, #17)

`Gif` e `WebP` passam a ser reconhecidos (paridade vanilla — GIF fica
estático no primeiro frame). A descodificação acontece em L3 (crate `image`
com features `gif`/`webp`, já dependência de `03_infra`).

---

## Tipos públicos

```rust
/// Formato de imagem detectado a partir dos bytes crus. Detecção por
/// assinatura binária (magic bytes) — não decodifica a imagem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Gif,   // P833 (#17)
    WebP,  // P833 (#17)
    Unknown,
}

/// Detecta o formato pela assinatura binária dos primeiros bytes.
/// JPEG: `FF D8 FF`. PNG: assinatura de 8 bytes `89 50 4E 47 0D 0A 1A 0A`.
/// GIF: `GIF87a`/`GIF89a`. WebP: `RIFF` + `WEBP` no offset 8.
/// Qualquer outra coisa (incluindo SVG, PDF-como-imagem, ou bytes
/// corrompidos) → `Unknown`.
pub fn detect_image_format(data: &[u8]) -> ImageFormat;
```

Movido de `03_infra/src/export/images.rs::{ImageFormat, detect_format}`
(`pub(super)`) — mesma lógica, byte a byte, agora `pub` em L1.

---

## Restrições estruturais

- Zero I/O, zero dependências externas — só comparação de bytes.
- `ImageFormat` deriva `PartialEq`/`Eq` (usado em comparações directas por
  `native_image` e pelo exportador PDF).
- Não inclui SVG como variante reconhecida: a detecção de SVG usada por
  `native_image` (P772p) é por **extensão do caminho** (`.svg`/`.svgz`), não
  por conteúdo — cristalino não tem parser SVG algum (P772k), pelo que
  content-sniffing XML seria trabalho sem consumidor.

## Critérios de Verificação

```
detect_image_format(&[0xFF, 0xD8, 0xFF, ...]) = Jpeg
detect_image_format(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, ...]) = Png
detect_image_format(b"GIF87a...") = Gif
detect_image_format(b"GIF89a...") = Gif
detect_image_format(b"RIFF....WEBP...") = WebP
detect_image_format(b"garbage-not-an-image") = Unknown
detect_image_format(&[]) = Unknown
```

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|-------------------|
| 2026-07-17 | Criação — P772p: move `ImageFormat`/`detect_format` de `03_infra/src/export/images.rs` para L1, reutilizável por `native_image` (validação em avaliação) e pelo exportador PDF (sem duplicar) | `image-format.md`, `image_format.rs`, `infra/export/images.md`, `03_infra/src/export/images.rs` |
| 2026-07-22 | P833 (#17/#18): variantes `Gif`/`WebP`; lacuna de corrupção profunda fechada em L3 (`validate_document_images` no pipeline, mensagem vanilla `failed to decode image ({detalhe})`) | `image-format.md`, `image_format.rs`, `infra/export/images.md`, `03_infra/src/export/images.rs`, `03_infra/src/pipeline.rs` |
