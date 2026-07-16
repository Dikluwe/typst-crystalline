//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/image-sizer.md
//! @prompt-hash 1cd66d65
//! @layer L1
//! @updated 2026-04-19

/// Contrato para leitura das dimensões intrínsecas de uma imagem.
///
/// Implementado em L3 com imagesize ou equivalente.
/// L1 define apenas o contrato — zero dependências externas.
pub trait ImageSizer {
    /// Retorna (largura_px, altura_px) ou None se os bytes forem inválidos.
    fn size(&self, data: &[u8]) -> Option<(u32, u32)>;

    /// Retorna o DPI (dots per inch) da imagem, se disponível nos metadados.
    /// Prioridade: EXIF > JFIF APP0 > PNG pHYs. Fallback 72 DPI é aplicado pelo
    /// consumidor quando este método retorna None (P773).
    fn dpi(&self, data: &[u8]) -> Option<f64>;

    /// Retorna o valor da tag EXIF Orientation (0x0112), se presente (P774).
    /// 1 = normal; 2-8 = transformações de espelhamento/rotação.
    fn orientation(&self, data: &[u8]) -> Option<u32>;
}

/// Implementação nula — retorna sempre None.
/// Usada em testes L1 que não precisam de dimensões reais.
#[derive(Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_sizer_retorna_none() {
        assert_eq!(NullImageSizer.size(&[]), None);
        assert_eq!(NullImageSizer.size(&[1, 2, 3]), None);
    }
}
