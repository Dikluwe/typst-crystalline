# Prompt L0 — infra/image_sizer
Hash do Código: de415c23

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
    fn size(&self, data: &[u8]) -> Option<(u32, u32)>
}
```

Delega para `imagesize::blob_size(data)`, convertendo `ImageSize` para
`(u32, u32)`.

## Invariantes

- `imagesize` declarado apenas em `03_infra/Cargo.toml`.
- `ImageSizeImageSizer` implementa o trait L1 — não adiciona API própria.
