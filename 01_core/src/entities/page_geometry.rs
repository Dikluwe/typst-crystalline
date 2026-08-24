//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/page_geometry.md
//! @prompt-hash 77a59d27
//! @layer L1
//! @updated 2026-08-24

use crate::entities::dir::Dir;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Paper {
    name: &'static str,
    width_mm: f64,
    height_mm: f64,
}

impl Paper {
    pub const A4: Self = Self::new("a4", 210.0, 297.0);
    const fn new(name: &'static str, width_mm: f64, height_mm: f64) -> Self {
        Self { name, width_mm, height_mm }
    }
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "a0" => Some(Self::new("a0", 841.0, 1189.0)),
            "a1" => Some(Self::new("a1", 594.0, 841.0)),
            "a2" => Some(Self::new("a2", 420.0, 594.0)),
            "a3" => Some(Self::new("a3", 297.0, 420.0)),
            "a4" => Some(Self::new("a4", 210.0, 297.0)),
            "a5" => Some(Self::new("a5", 148.0, 210.0)),
            "a6" => Some(Self::new("a6", 105.0, 148.0)),
            "a7" => Some(Self::new("a7", 74.0, 105.0)),
            "a8" => Some(Self::new("a8", 52.0, 74.0)),
            "a9" => Some(Self::new("a9", 37.0, 52.0)),
            "a10" => Some(Self::new("a10", 26.0, 37.0)),
            "a11" => Some(Self::new("a11", 18.0, 26.0)),
            "iso-b1" => Some(Self::new("iso-b1", 707.0, 1000.0)),
            "iso-b2" => Some(Self::new("iso-b2", 500.0, 707.0)),
            "iso-b3" => Some(Self::new("iso-b3", 353.0, 500.0)),
            "iso-b4" => Some(Self::new("iso-b4", 250.0, 353.0)),
            "iso-b5" => Some(Self::new("iso-b5", 176.0, 250.0)),
            "iso-b6" => Some(Self::new("iso-b6", 125.0, 176.0)),
            "iso-b7" => Some(Self::new("iso-b7", 88.0, 125.0)),
            "iso-b8" => Some(Self::new("iso-b8", 62.0, 88.0)),
            "iso-c3" => Some(Self::new("iso-c3", 324.0, 458.0)),
            "iso-c4" => Some(Self::new("iso-c4", 229.0, 324.0)),
            "iso-c5" => Some(Self::new("iso-c5", 162.0, 229.0)),
            "iso-c6" => Some(Self::new("iso-c6", 114.0, 162.0)),
            "iso-c7" => Some(Self::new("iso-c7", 81.0, 114.0)),
            "iso-c8" => Some(Self::new("iso-c8", 57.0, 81.0)),
            "din-d3" => Some(Self::new("din-d3", 272.0, 385.0)),
            "din-d4" => Some(Self::new("din-d4", 192.0, 272.0)),
            "din-d5" => Some(Self::new("din-d5", 136.0, 192.0)),
            "din-d6" => Some(Self::new("din-d6", 96.0, 136.0)),
            "din-d7" => Some(Self::new("din-d7", 68.0, 96.0)),
            "din-d8" => Some(Self::new("din-d8", 48.0, 68.0)),
            "sis-g5" => Some(Self::new("sis-g5", 169.0, 239.0)),
            "sis-e5" => Some(Self::new("sis-e5", 115.0, 220.0)),
            "ansi-a" => Some(Self::new("ansi-a", 216.0, 279.0)),
            "ansi-b" => Some(Self::new("ansi-b", 279.0, 432.0)),
            "ansi-c" => Some(Self::new("ansi-c", 432.0, 559.0)),
            "ansi-d" => Some(Self::new("ansi-d", 559.0, 864.0)),
            "ansi-e" => Some(Self::new("ansi-e", 864.0, 1118.0)),
            "arch-a" => Some(Self::new("arch-a", 229.0, 305.0)),
            "arch-b" => Some(Self::new("arch-b", 305.0, 457.0)),
            "arch-c" => Some(Self::new("arch-c", 457.0, 610.0)),
            "arch-d" => Some(Self::new("arch-d", 610.0, 914.0)),
            "arch-e1" => Some(Self::new("arch-e1", 762.0, 1067.0)),
            "arch-e" => Some(Self::new("arch-e", 914.0, 1219.0)),
            "jis-b0" => Some(Self::new("jis-b0", 1030.0, 1456.0)),
            "jis-b1" => Some(Self::new("jis-b1", 728.0, 1030.0)),
            "jis-b2" => Some(Self::new("jis-b2", 515.0, 728.0)),
            "jis-b3" => Some(Self::new("jis-b3", 364.0, 515.0)),
            "jis-b4" => Some(Self::new("jis-b4", 257.0, 364.0)),
            "jis-b5" => Some(Self::new("jis-b5", 182.0, 257.0)),
            "jis-b6" => Some(Self::new("jis-b6", 128.0, 182.0)),
            "jis-b7" => Some(Self::new("jis-b7", 91.0, 128.0)),
            "jis-b8" => Some(Self::new("jis-b8", 64.0, 91.0)),
            "jis-b9" => Some(Self::new("jis-b9", 45.0, 64.0)),
            "jis-b10" => Some(Self::new("jis-b10", 32.0, 45.0)),
            "jis-b11" => Some(Self::new("jis-b11", 22.0, 32.0)),
            "sac-d0" => Some(Self::new("sac-d0", 764.0, 1064.0)),
            "sac-d1" => Some(Self::new("sac-d1", 532.0, 760.0)),
            "sac-d2" => Some(Self::new("sac-d2", 380.0, 528.0)),
            "sac-d3" => Some(Self::new("sac-d3", 264.0, 376.0)),
            "sac-d4" => Some(Self::new("sac-d4", 188.0, 260.0)),
            "sac-d5" => Some(Self::new("sac-d5", 130.0, 184.0)),
            "sac-d6" => Some(Self::new("sac-d6", 92.0, 126.0)),
            "iso-id-1" => Some(Self::new("iso-id-1", 85.6, 53.98)),
            "iso-id-2" => Some(Self::new("iso-id-2", 74.0, 105.0)),
            "iso-id-3" => Some(Self::new("iso-id-3", 88.0, 125.0)),
            "asia-f4" => Some(Self::new("asia-f4", 210.0, 330.0)),
            "jp-shiroku-ban-4" => Some(Self::new("jp-shiroku-ban-4", 264.0, 379.0)),
            "jp-shiroku-ban-5" => Some(Self::new("jp-shiroku-ban-5", 189.0, 262.0)),
            "jp-shiroku-ban-6" => Some(Self::new("jp-shiroku-ban-6", 127.0, 188.0)),
            "jp-kiku-4" => Some(Self::new("jp-kiku-4", 227.0, 306.0)),
            "jp-kiku-5" => Some(Self::new("jp-kiku-5", 151.0, 227.0)),
            "jp-business-card" => Some(Self::new("jp-business-card", 91.0, 55.0)),
            "cn-business-card" => Some(Self::new("cn-business-card", 90.0, 54.0)),
            "eu-business-card" => Some(Self::new("eu-business-card", 85.0, 55.0)),
            "fr-tellière" => Some(Self::new("fr-tellière", 340.0, 440.0)),
            "fr-couronne-écriture" => {
                Some(Self::new("fr-couronne-écriture", 360.0, 460.0))
            }
            "fr-couronne-édition" => {
                Some(Self::new("fr-couronne-édition", 370.0, 470.0))
            }
            "fr-raisin" => Some(Self::new("fr-raisin", 500.0, 650.0)),
            "fr-carré" => Some(Self::new("fr-carré", 450.0, 560.0)),
            "fr-jésus" => Some(Self::new("fr-jésus", 560.0, 760.0)),
            "uk-brief" => Some(Self::new("uk-brief", 406.4, 342.9)),
            "uk-draft" => Some(Self::new("uk-draft", 254.0, 406.4)),
            "uk-foolscap" => Some(Self::new("uk-foolscap", 203.2, 330.2)),
            "uk-quarto" => Some(Self::new("uk-quarto", 203.2, 254.0)),
            "uk-crown" => Some(Self::new("uk-crown", 508.0, 381.0)),
            "uk-book-a" => Some(Self::new("uk-book-a", 111.0, 178.0)),
            "uk-book-b" => Some(Self::new("uk-book-b", 129.0, 198.0)),
            "us-letter" => Some(Self::new("us-letter", 215.9, 279.4)),
            "us-legal" => Some(Self::new("us-legal", 215.9, 355.6)),
            "us-tabloid" => Some(Self::new("us-tabloid", 279.4, 431.8)),
            "us-executive" => Some(Self::new("us-executive", 184.15, 266.7)),
            "us-foolscap-folio" => Some(Self::new("us-foolscap-folio", 215.9, 342.9)),
            "us-statement" => Some(Self::new("us-statement", 139.7, 215.9)),
            "us-ledger" => Some(Self::new("us-ledger", 431.8, 279.4)),
            "us-oficio" => Some(Self::new("us-oficio", 215.9, 340.36)),
            "us-gov-letter" => Some(Self::new("us-gov-letter", 203.2, 266.7)),
            "us-gov-legal" => Some(Self::new("us-gov-legal", 215.9, 330.2)),
            "us-business-card" => Some(Self::new("us-business-card", 88.9, 50.8)),
            "us-digest" => Some(Self::new("us-digest", 139.7, 215.9)),
            "us-trade" => Some(Self::new("us-trade", 152.4, 228.6)),
            "newspaper-compact" => Some(Self::new("newspaper-compact", 280.0, 430.0)),
            "newspaper-berliner" => Some(Self::new("newspaper-berliner", 315.0, 470.0)),
            "newspaper-broadsheet" => {
                Some(Self::new("newspaper-broadsheet", 381.0, 578.0))
            }
            "presentation-16-9" => Some(Self::new("presentation-16-9", 297.0, 167.0625)),
            "presentation-4-3" => Some(Self::new("presentation-4-3", 280.0, 210.0)),
            _ => None,
        }
    }
    pub const fn name(self) -> &'static str {
        self.name
    }
    pub const fn width_pt(self) -> f64 {
        self.width_mm * 72.0 / 25.4
    }
    pub const fn height_pt(self) -> f64 {
        self.height_mm * 72.0 / 25.4
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageBinding {
    Auto,
    Left,
    Right,
}

impl PageBinding {
    pub fn resolve(self, dir: Dir) -> Self {
        match self {
            Self::Auto if dir == Dir::LTR => Self::Left,
            Self::Auto => Self::Right,
            explicit => explicit,
        }
    }
    pub fn swap(self, page: std::num::NonZeroUsize) -> bool {
        match self {
            Self::Left => page.get() % 2 == 0,
            Self::Right => page.get() % 2 == 1,
            Self::Auto => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1140_20_1_paper_normativo() {
        assert_eq!(Paper::from_name("A4"), Some(Paper::A4));
        assert_eq!(Paper::from_name("us-letter").unwrap().name(), "us-letter");
        assert!(Paper::from_name("inventado").is_none());
        assert!((Paper::A4.width_pt() - 595.275590551).abs() < 1e-9);
    }
    #[test]
    fn p1140_20_1_binding_direcao_e_paridade() {
        use std::num::NonZeroUsize;
        assert_eq!(PageBinding::Auto.resolve(Dir::LTR), PageBinding::Left);
        assert_eq!(PageBinding::Auto.resolve(Dir::RTL), PageBinding::Right);
        assert!(PageBinding::Left.swap(NonZeroUsize::new(2).unwrap()));
        assert!(PageBinding::Right.swap(NonZeroUsize::new(1).unwrap()));
    }
    #[test]
    fn p1140_20_1_fold_margem_preserva_lados() {
        use crate::entities::layout_types::PageMarginSpec;
        let outer = PageMarginSpec::uniform(10.0);
        let delta = PageMarginSpec {
            left: Some(20.0),
            right: None,
            top: None,
            bottom: None,
            two_sided: Some(true),
        };
        let got = delta.fold(outer);
        assert_eq!(got.left, Some(20.0));
        assert_eq!(got.right, Some(10.0));
        assert_eq!(got.two_sided, Some(true));
    }
}
