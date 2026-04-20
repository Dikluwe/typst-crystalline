# Prompt L0 — entities/image_sizer
Hash do Código: 96f228bc

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/image_sizer.rs`
**ADRs relevantes**: ADR-0029 (pureza física), ADR-0001 (l1_allowed_external)

## Contexto

`ImageSizer` é o contrato para leitura das dimensões intrínsecas de uma imagem
em píxeis. A implementação pertence a L3 (usa I/O de cabeçalho de ficheiro).
L1 define apenas o trait e uma implementação nula para testes.

## Tipos públicos

### ImageSizer

```rust
pub trait ImageSizer {
    /// Retorna (largura_px, altura_px) ou None se os bytes forem inválidos.
    fn size(&self, data: &[u8]) -> Option<(u32, u32)>;
}
```

### NullImageSizer

```rust
pub struct NullImageSizer;

impl ImageSizer for NullImageSizer {
    fn size(&self, _data: &[u8]) -> Option<(u32, u32)> {
        None
    }
}
```

Usada em testes L1 que não precisam de dimensões reais. Retorna sempre `None`,
fazendo o motor de dimensões usar o fallback 100×100 pt.

## Invariantes

- `ImageSizer` não tem estado partilhado nem I/O em L1.
- `NullImageSizer` é a única implementação em L1.
- Implementações reais (L3) usam bibliotecas de I/O fora de L1.
