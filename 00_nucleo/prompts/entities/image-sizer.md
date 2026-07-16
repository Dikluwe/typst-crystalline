# Prompt L0 — entities/image_sizer
Hash do Código: aaad8ac7

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/image_sizer.rs`
**ADRs relevantes**: ADR-0029 (pureza física), ADR-0001 (l1_allowed_external)

## Contexto

`ImageSizer` é o contrato para leitura das dimensões intrínsecas de uma imagem
em píxeis, do seu DPI (P773) e da sua orientação EXIF (P774/P776). A implementação
pertence a L3 (usa I/O de cabeçalho/ficheiro). L1 define apenas o trait e uma
implementação nula para testes.

## Tipos públicos

### ImageSizer

```rust
pub trait ImageSizer {
    /// Retorna (largura_px, altura_px) ou None se os bytes forem inválidos.
    fn size(&self, data: &[u8]) -> Option<(u32, u32)>;

    /// Retorna o DPI (dots per inch) da imagem, se disponível nos metadados.
    /// Prioridade: EXIF > JFIF APP0 > PNG pHYs. Fallback 72 DPI é aplicado pelo
    /// consumidor quando este método retorna None (P773).
    fn dpi(&self, data: &[u8]) -> Option<f64>;

    /// Retorna o valor da tag EXIF Orientation (0x0112), se presente.
    /// Valores 1-8 conforme especificação EXIF; 1 significa "sem rotação".
    /// A transformação visual é aplicada no exportador PDF via matriz `cm`
    /// (P776); o contrato em L1 apenas transporta o valor.
    fn orientation(&self, data: &[u8]) -> Option<u32>;
}
```

### NullImageSizer

```rust
pub struct NullImageSizer;

impl ImageSizer for NullImageSizer {
    fn size(&self, _data: &[u8]) -> Option<(u32, u32)> {
        None
    }

    fn dpi(&self, _data: &[u8]) -> Option<f64> {
        None
    }

    fn orientation(&self, _data: &[u8]) -> Option<u32> {
        None
    }
}
```

Usada em testes L1 que não precisam de dimensões reais. Retorna sempre `None`,
 fazendo o motor de dimensões usar o fallback 100×100 pt, 72 DPI e sem rotação.

## Invariantes

- `ImageSizer` não tem estado partilhado nem I/O em L1.
- `NullImageSizer` é a única implementação em L1.
- Implementações reais (L3) usam bibliotecas de I/O fora de L1.
