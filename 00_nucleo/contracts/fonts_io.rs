//! Contratos de Interação Externa detectados no `fonts.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Fontes

use std::io::Result;
use std::path::Path;
use typst_kit::fonts::FontStore;

/// Interface para descoberta de fontes.
pub trait IFontDiscoverer {
    /// Descobre as fontes baseando-se nas flags fornecidas.
    fn discover(
        &self,
        include_system: bool,
        include_embedded: bool,
        font_paths: &[impl AsRef<Path>],
    ) -> Result<FontStore>;
}

/// Interface para escrita de resultados de listagem de fontes.
pub trait IFontPrinter {
    /// Imprime o nome de uma família de fontes.
    fn print_family(&mut self, family_name: &str) -> Result<()>;

    /// Imprime os detalhes de uma variante (já formatados pela L1).
    fn print_variant_details(&mut self, details: &str) -> Result<()>;
}
