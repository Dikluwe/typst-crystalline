//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Fonts
//! Responsabilidade: Implementar contratos de descoberta de fontes via `typst_kit` e impressão em stdout.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use typst_kit::fonts::{self, FontStore};

#[path = "../00_nucleo/contracts/fonts_io.rs"]
pub mod fonts_io;

use fonts_io::{IFontDiscoverer, IFontPrinter};

/// Provedor de descoberta de fontes usando `typst_kit::fonts`.
pub struct SystemFontDiscoverer;

impl IFontDiscoverer for SystemFontDiscoverer {
    fn discover(
        &self,
        include_system: bool,
        include_embedded: bool,
        font_paths: &[impl AsRef<Path>],
    ) -> io::Result<FontStore> {
        let mut fonts = FontStore::new();

        if include_system {
            fonts.extend(fonts::system());
        }

        #[cfg(feature = "embedded-fonts")]
        if include_embedded {
            fonts.extend(fonts::embedded());
        } else {
            // Se o feature existe mas foi ignorado via macro, consome a variável
            let _ = include_embedded;
        }

        #[cfg(not(feature = "embedded-fonts"))]
        {
            // Consome a variável se a feature não estiver habilitada
            let _ = include_embedded;
        }

        for path in font_paths {
            fonts.extend(fonts::scan(path.as_ref()));
        }

        Ok(fonts)
    }
}

/// Impressor padrão que escreve as fontes catalogadas no `stdout`.
pub struct StandardFontPrinter;

impl IFontPrinter for StandardFontPrinter {
    fn print_family(&mut self, family_name: &str) -> io::Result<()> {
        writeln!(io::stdout(), "{}", family_name)
    }

    fn print_variant_details(&mut self, details: &str) -> io::Result<()> {
        writeln!(io::stdout(), "{}", details)
    }
}
