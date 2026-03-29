//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/font-book.md
//! @prompt-hash 87be4ed2
//! @layer L1
//! @updated 2026-03-27

/// Estilo de fonte: Normal (upright), Italic (cursivo), Oblique (inclinado).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

impl FontStyle {
    /// Distância conceptual entre dois estilos (para selecção de fonte mais próxima).
    pub fn distance(self, other: Self) -> u16 {
        if self == other {
            0
        } else if self != Self::Normal && other != Self::Normal {
            1
        } else {
            2
        }
    }
}

/// Peso de fonte: 100 (Thin) … 900 (Black). 400 = Regular, 700 = Bold.
/// Unidade: valores CSS standard (100–900).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FontWeight(pub u16);

impl FontWeight {
    pub const THIN:      Self = Self(100);
    pub const EXTRALIGHT: Self = Self(200);
    pub const LIGHT:     Self = Self(300);
    pub const REGULAR:   Self = Self(400);
    pub const MEDIUM:    Self = Self(500);
    pub const SEMIBOLD:  Self = Self(600);
    pub const BOLD:      Self = Self(700);
    pub const EXTRABOLD: Self = Self(800);
    pub const BLACK:     Self = Self(900);

    /// Cria FontWeight a partir de número, clampando para [100, 900].
    pub fn from_number(weight: u16) -> Self {
        Self(weight.clamp(100, 900))
    }

    /// O número CSS entre 100 e 900.
    pub fn to_number(self) -> u16 {
        self.0
    }

    /// Distância absoluta entre dois pesos — para selecção da fonte mais próxima.
    pub fn distance(self, other: Self) -> u16 {
        self.0.abs_diff(other.0)
    }
}

impl Default for FontWeight {
    fn default() -> Self { Self::REGULAR }
}

/// Largura de fonte em unidades de 0.1% (NORMAL = 1000 = 100%).
/// Mapeia os 9 valores OpenType: UltraCondensed (500) … UltraExpanded (2000).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FontStretch(pub u16);

impl FontStretch {
    pub const ULTRA_CONDENSED: Self = Self(500);
    pub const EXTRA_CONDENSED: Self = Self(625);
    pub const CONDENSED:       Self = Self(750);
    pub const SEMI_CONDENSED:  Self = Self(875);
    pub const NORMAL:          Self = Self(1000);
    pub const SEMI_EXPANDED:   Self = Self(1125);
    pub const EXPANDED:        Self = Self(1250);
    pub const EXTRA_EXPANDED:  Self = Self(1500);
    pub const ULTRA_EXPANDED:  Self = Self(2000);

    /// Cria FontStretch a partir do número OpenType (1–9).
    /// Usado na conversão de ttf_parser::Width em L3.
    pub fn from_number(stretch: u16) -> Self {
        match stretch {
            0 | 1 => Self::ULTRA_CONDENSED,
            2     => Self::EXTRA_CONDENSED,
            3     => Self::CONDENSED,
            4     => Self::SEMI_CONDENSED,
            5     => Self::NORMAL,
            6     => Self::SEMI_EXPANDED,
            7     => Self::EXPANDED,
            8     => Self::EXTRA_EXPANDED,
            _     => Self::ULTRA_EXPANDED,
        }
    }
}

impl Default for FontStretch {
    fn default() -> Self { Self::NORMAL }
}

/// Variante completa de fonte (estilo + peso + largura).
/// Identifica univocamente uma face dentro da mesma família.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct FontVariant {
    pub style:   FontStyle,
    pub weight:  FontWeight,
    pub stretch: FontStretch,
}

/// Flags binárias de características de uma face de fonte.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FontFlags {
    /// Todos os glifos têm a mesma largura.
    pub monospace: bool,
    /// Glifos têm hastes nas extremidades (serifs).
    pub serif: bool,
}

/// Metadados de uma face de fonte — campos puramente primitivos.
/// Populado em L3 a partir de bytes via `ttf_parser`; consultado
/// em L1 para selecção de fontes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontInfo {
    /// Nome da família tipográfica.
    pub family:  String,
    /// Variante (estilo, peso, largura).
    pub variant: FontVariant,
    /// Flags de características da face.
    pub flags:   FontFlags,
}

/// Catálogo de metadados de fontes disponíveis.
///
/// `FontBook` em L1 é uma colecção pura com métodos de pesquisa.
/// Populado em L3 via `font_info_from_bytes` (ADR-0022).
/// Não sabe de bytes, paths ou `ttf_parser`.
#[derive(PartialEq, Eq, Hash)]
pub struct FontBook {
    infos: Vec<FontInfo>,
}

impl FontBook {
    pub fn new() -> Self {
        Self { infos: Vec::new() }
    }

    /// Adiciona uma entrada de fonte ao catálogo.
    pub fn push(&mut self, info: FontInfo) {
        self.infos.push(info);
    }

    /// Metadados de todas as fontes no catálogo.
    pub fn infos(&self) -> &[FontInfo] {
        &self.infos
    }

    pub fn len(&self) -> usize { self.infos.len() }
    pub fn is_empty(&self) -> bool { self.infos.is_empty() }

    /// Selecciona o índice da fonte mais próxima de `(family, variant)`.
    ///
    /// Critério: família exacta (case-insensitive) + peso mais próximo
    /// + estilo mais próximo. Retorna `None` se a família não existir.
    pub fn select(&self, family: &str, variant: &FontVariant) -> Option<usize> {
        let candidates: Vec<usize> = self.infos.iter()
            .enumerate()
            .filter(|(_, info)| info.family.eq_ignore_ascii_case(family))
            .map(|(i, _)| i)
            .collect();

        if candidates.is_empty() {
            return None;
        }

        candidates.into_iter().min_by_key(|&i| {
            let info = &self.infos[i];
            let weight_dist = info.variant.weight.distance(variant.weight);
            let style_dist  = info.variant.style.distance(variant.style);
            (weight_dist, style_dist)
        })
    }

    /// Itera sobre índices de todas as faces de uma família (case-insensitive).
    pub fn select_family<'a>(
        &'a self,
        family: &'a str,
    ) -> impl Iterator<Item = usize> + 'a {
        self.infos.iter()
            .enumerate()
            .filter(move |(_, info)| info.family.eq_ignore_ascii_case(family))
            .map(|(i, _)| i)
    }
}

impl Default for FontBook {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_info(family: &str, weight: u16, style: FontStyle) -> FontInfo {
        FontInfo {
            family: family.into(),
            variant: FontVariant {
                weight: FontWeight(weight),
                style,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
        }
    }

    #[test]
    fn fontbook_vazio() {
        let book = FontBook::new();
        assert!(book.is_empty());
        assert!(book.select("Any", &FontVariant::default()).is_none());
    }

    #[test]
    fn fontbook_select_exacto() {
        let mut book = FontBook::new();
        book.push(FontInfo {
            family:  "Test Family".into(),
            variant: FontVariant {
                style:   FontStyle::Normal,
                weight:  FontWeight::REGULAR,
                stretch: FontStretch(1000),
            },
            flags: FontFlags::default(),
        });
        let idx = book.select("Test Family", &FontVariant {
            style:   FontStyle::Normal,
            weight:  FontWeight::REGULAR,
            stretch: FontStretch(1000),
        });
        assert_eq!(idx, Some(0));
    }

    #[test]
    fn fontbook_select_familia_case_insensitive() {
        let mut book = FontBook::new();
        book.push(make_info("Liberation Sans", 400, FontStyle::Normal));
        assert!(book.select("liberation sans", &FontVariant::default()).is_some());
        assert!(book.select("LIBERATION SANS", &FontVariant::default()).is_some());
    }

    #[test]
    fn fontbook_select_peso_mais_proximo() {
        let mut book = FontBook::new();
        book.push(make_info("Test", 300, FontStyle::Normal));
        book.push(make_info("Test", 700, FontStyle::Normal));
        // Pedir 400 — mais próximo é 300 (dist=100) vs 700 (dist=300)
        let idx = book.select("Test", &FontVariant {
            weight: FontWeight::REGULAR,
            ..Default::default()
        }).unwrap();
        assert_eq!(book.infos()[idx].variant.weight, FontWeight(300));
    }

    #[test]
    fn fontbook_select_family_iterator() {
        let mut book = FontBook::new();
        book.push(make_info("A", 400, FontStyle::Normal));
        book.push(make_info("B", 400, FontStyle::Normal));
        book.push(make_info("A", 700, FontStyle::Normal));

        let a_indices: Vec<usize> = book.select_family("A").collect();
        assert_eq!(a_indices, vec![0, 2]);
    }

    #[test]
    fn fontbook_select_familia_inexistente() {
        let book = FontBook::new();
        assert!(book.select("NonExistent", &FontVariant::default()).is_none());
    }

    #[test]
    fn fontweight_distance() {
        assert_eq!(FontWeight(400).distance(FontWeight(700)), 300);
        assert_eq!(FontWeight(400).distance(FontWeight(400)), 0);
    }

    #[test]
    fn fontstretch_from_number() {
        assert_eq!(FontStretch::from_number(5), FontStretch::NORMAL);
        assert_eq!(FontStretch::from_number(1), FontStretch::ULTRA_CONDENSED);
        assert_eq!(FontStretch::from_number(9), FontStretch::ULTRA_EXPANDED);
    }

    #[test]
    fn fontstyle_distance() {
        assert_eq!(FontStyle::Normal.distance(FontStyle::Normal), 0);
        assert_eq!(FontStyle::Normal.distance(FontStyle::Italic), 2);
        assert_eq!(FontStyle::Italic.distance(FontStyle::Oblique), 1);
    }
}
