//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/font_book.md
//! @prompt-hash 8668e6de
//! @layer L1
//! @updated 2026-10-07

use std::sync::Arc;
use unicode_segmentation::UnicodeSegmentation;

use crate::entities::font_list::FontNamePattern;

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
    pub const THIN: Self = Self(100);
    pub const EXTRALIGHT: Self = Self(200);
    pub const LIGHT: Self = Self(300);
    pub const REGULAR: Self = Self(400);
    pub const MEDIUM: Self = Self(500);
    pub const SEMIBOLD: Self = Self(600);
    pub const BOLD: Self = Self(700);
    pub const EXTRABOLD: Self = Self(800);
    pub const BLACK: Self = Self(900);

    /// Cria FontWeight a partir de número, clampando para [100, 900].
    pub fn from_number(weight: u16) -> Self {
        Self(weight.clamp(100, 900))
    }

    /// Mapeia nome simbólico (9 canónicos do Typst vanilla) para
    /// `FontWeight`. Devolve `None` para qualquer outro nome
    /// (Passo 129, DEBT-1 subset).
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "thin" => Some(Self::THIN),
            "extralight" => Some(Self::EXTRALIGHT),
            "light" => Some(Self::LIGHT),
            "regular" => Some(Self::REGULAR),
            "medium" => Some(Self::MEDIUM),
            "semibold" => Some(Self::SEMIBOLD),
            "bold" => Some(Self::BOLD),
            "extrabold" => Some(Self::EXTRABOLD),
            "black" => Some(Self::BLACK),
            _ => None,
        }
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
    fn default() -> Self {
        Self::REGULAR
    }
}

/// Largura de fonte em unidades de 0.1% (NORMAL = 1000 = 100%).
/// Mapeia os 9 valores OpenType: UltraCondensed (500) … UltraExpanded (2000).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FontStretch(pub u16);

impl FontStretch {
    pub const ULTRA_CONDENSED: Self = Self(500);
    pub const EXTRA_CONDENSED: Self = Self(625);
    pub const CONDENSED: Self = Self(750);
    pub const SEMI_CONDENSED: Self = Self(875);
    pub const NORMAL: Self = Self(1000);
    pub const SEMI_EXPANDED: Self = Self(1125);
    pub const EXPANDED: Self = Self(1250);
    pub const EXTRA_EXPANDED: Self = Self(1500);
    pub const ULTRA_EXPANDED: Self = Self(2000);

    /// Cria FontStretch a partir do número OpenType (1–9).
    /// Usado na conversão de ttf_parser::Width em L3.
    pub fn from_number(stretch: u16) -> Self {
        match stretch {
            0 | 1 => Self::ULTRA_CONDENSED,
            2 => Self::EXTRA_CONDENSED,
            3 => Self::CONDENSED,
            4 => Self::SEMI_CONDENSED,
            5 => Self::NORMAL,
            6 => Self::SEMI_EXPANDED,
            7 => Self::EXPANDED,
            8 => Self::EXTRA_EXPANDED,
            _ => Self::ULTRA_EXPANDED,
        }
    }
}

impl Default for FontStretch {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl FontStretch {
    /// Distância absoluta entre duas larguras — para selecção da fonte mais
    /// próxima (P838 — usada no scoring de `select_fallback`, paridade com a
    /// `distance` do vanilla).
    pub fn distance(self, other: Self) -> u16 {
        self.0.abs_diff(other.0)
    }
}

/// Variante completa de fonte (estilo + peso + largura).
/// Identifica univocamente uma face dentro da mesma família.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct FontVariant {
    pub style: FontStyle,
    pub weight: FontWeight,
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

/// Cobertura Unicode exacta de uma face de fonte.
///
/// P937 — representação por runs alternadas de codepoints fora/dentro do
/// conjunto, portada do vanilla (`typst-library/src/text/font/info.rs:269-318`).
/// Elimina os falsos positivos do bitmap por bloco de 256 codepoints (P880).
#[derive(Clone)]
pub struct Coverage(Arc<[u32]>, Arc<[u32]>);

const COVERAGE_BLOCK_RUNS: usize = 16;

impl PartialEq for Coverage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Coverage {}
impl std::hash::Hash for Coverage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.0, state);
    }
}

impl Coverage {
    /// Cobertura vazia.
    pub fn new() -> Self {
        Self(Arc::from([]), Arc::from([]))
    }

    /// Build exact coverage from codepoints in arbitrary order.
    /// Compact contiguous sequences before sorting and merging intervals.
    /// The vector API and canonical alternating gap/count encoding are retained.
    pub fn from_codepoints(codepoints: impl Into<Vec<u32>>) -> Self {
        let mut intervals: Vec<(u32, u32)> = Vec::new();
        let mut points = codepoints.into().into_iter();
        if let Some(mut start) = points.next() {
            let mut end = start;
            for point in points {
                if end.checked_add(1) == Some(point) {
                    end = point;
                } else if (start..=end).contains(&point) {
                    // Already covered by the active interval.
                } else if point.checked_add(1) == Some(start) {
                    start = point;
                } else {
                    intervals.push((start, end));
                    start = point;
                    end = point;
                }
            }
            intervals.push((start, end));
        }
        intervals.sort_unstable_by_key(|&(start, _)| start);

        let mut runs: Vec<u32> = Vec::new();
        let mut next = 0u32;
        for (start, end) in intervals {
            let end = end + 1;
            if end <= next {
                continue;
            }
            if start <= next && !runs.is_empty() {
                *runs.last_mut().unwrap() += end - next;
            } else {
                runs.extend([start - next, end - start]);
            }
            next = end;
        }
        Self::from_runs(runs)
    }

    /// Build the exact union of half-open integer ranges.
    /// Empty/reversed ranges contribute no members.
    pub fn from_ranges(ranges: impl IntoIterator<Item = std::ops::Range<u32>>) -> Self {
        let mut compact = Vec::new();
        let mut input = ranges.into_iter().filter(|range| range.start < range.end);
        if let Some(first) = input.next() {
            let mut start = first.start;
            let mut end = first.end;
            for range in input {
                if range.start <= end && start <= range.end {
                    start = start.min(range.start);
                    end = end.max(range.end);
                } else {
                    compact.push(start..end);
                    start = range.start;
                    end = range.end;
                }
            }
            compact.push(start..end);
        }
        compact.sort_unstable_by_key(|range| range.start);
        let mut runs = Vec::new();
        let mut next = 0;
        for range in compact {
            if range.end <= next {
                continue;
            }
            if range.start <= next && !runs.is_empty() {
                *runs.last_mut().unwrap() += range.end - next;
            } else {
                runs.extend([range.start - next, range.end - range.start]);
            }
            next = range.end;
        }
        Self::from_runs(runs)
    }

    // Derived immutable checkpoint ends; canonical runs remain the only identity.
    fn from_runs(runs: Vec<u32>) -> Self {
        let mut ends = Vec::with_capacity(runs.len().div_ceil(COVERAGE_BLOCK_RUNS));
        let mut cursor = 0u32;
        for (i, &run) in runs.iter().enumerate() {
            cursor += run;
            if (i + 1) % COVERAGE_BLOCK_RUNS == 0 || i + 1 == runs.len() {
                ends.push(cursor);
            }
        }
        Self(runs.into(), ends.into())
    }

    /// Exact membership by sparse checkpoint search and at most sixteen runs.
    pub fn contains(&self, codepoint: u32) -> bool {
        self.contains_visit(codepoint, || {})
    }

    fn contains_visit(&self, codepoint: u32, mut visit: impl FnMut()) -> bool {
        let Some(&last) = self.1.last() else { return false };
        visit();
        if codepoint >= last {
            return false;
        }
        visit();
        let block = if codepoint < self.1[0] {
            0
        } else {
            self.1.partition_point(|&end| {
                visit();
                end <= codepoint
            })
        };
        let mut cursor = if block == 0 {
            0
        } else {
            visit();
            self.1[block - 1]
        };
        let start = block * COVERAGE_BLOCK_RUNS;
        let end = (start + COVERAGE_BLOCK_RUNS).min(self.0.len());
        // Even-sized blocks always start outside, matching the canonical format.
        let mut inside = false;
        for &run in &self.0[start..end] {
            visit();
            if cursor > codepoint {
                return false;
            }
            if (cursor..cursor + run).contains(&codepoint) {
                return inside;
            }
            cursor += run;
            inside = !inside;
        }
        false
    }

    #[cfg(test)]
    fn query_backings(&self) -> [(*const u32, usize); 2] {
        [(self.0.as_ptr(), self.0.len()), (self.1.as_ptr(), self.1.len())]
    }

    /// True se não cobre nenhum codepoint.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Itera todos os codepoints cobertos (útil para testes e debug).
    /// Port do vanilla `Coverage::iter` (info.rs:329-338).
    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        let mut inside = false;
        let mut cursor = 0u32;
        self.0.iter().flat_map(move |run| {
            let range = if inside { cursor..cursor + run } else { 0..0 };
            inside = !inside;
            cursor += run;
            range
        })
    }
}

impl Default for Coverage {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Coverage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Coverage").field(&self.0).finish()
    }
}

/// Metadados de uma face de fonte — campos puramente primitivos.
/// Populado em L3 a partir de bytes via `ttf_parser`; consultado
/// em L1 para selecção de fontes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontInfo {
    /// Nome da família tipográfica.
    pub family: String,
    /// Variante (estilo, peso, largura).
    pub variant: FontVariant,
    /// Flags de características da face.
    pub flags: FontFlags,
    /// Cobertura Unicode exacta (runs de codepoints) — P937.
    pub coverage: Coverage,
}

/// Catálogo de metadados de fontes disponíveis.
///
/// `FontBook` em L1 é uma colecção pura com métodos de pesquisa.
/// Populado em L3 via `font_info_from_bytes` (ADR-0022).
/// Não sabe de bytes, paths ou `ttf_parser`.
pub struct FontBook {
    infos: Vec<FontInfo>,
    // Derived from this book only; never slot identity or cache authority.
    literal_slots: Vec<usize>,
}

impl PartialEq for FontBook {
    fn eq(&self, other: &Self) -> bool {
        self.infos == other.infos
    }
}
impl Eq for FontBook {}
impl std::hash::Hash for FontBook {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.infos, state);
    }
}

// ASCII-only folding is exactly the existing case-insensitive equality.
fn literal_family_cmp(left: &str, right: &str) -> std::cmp::Ordering {
    left.bytes()
        .map(|byte| byte.to_ascii_lowercase())
        .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase()))
}

impl FontBook {
    pub fn new() -> Self {
        Self { infos: Vec::new(), literal_slots: Vec::new() }
    }

    /// Adiciona uma entrada de fonte ao catálogo.
    pub fn push(&mut self, info: FontInfo) {
        let slot = self.infos.len();
        self.infos.push(info);
        let at = self.literal_slots.partition_point(|&previous| {
            literal_family_cmp(&self.infos[previous].family, &self.infos[slot].family)
                .is_le()
        });
        self.literal_slots.insert(at, slot);
    }
    /// Metadados de todas as fontes no catálogo.
    pub fn infos(&self) -> &[FontInfo] {
        &self.infos
    }

    pub fn len(&self) -> usize {
        self.infos.len()
    }
    pub fn is_empty(&self) -> bool {
        self.infos.is_empty()
    }

    /// Selecciona o índice da fonte mais próxima de `(family, variant)`.
    ///
    /// Critério: família exacta (case-insensitive) + peso mais próximo
    /// + estilo mais próximo. Retorna `None` se a família não existir.
    pub fn select(&self, family: &str, variant: &FontVariant) -> Option<usize> {
        let candidates: Vec<usize> = self
            .infos
            .iter()
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
            let style_dist = info.variant.style.distance(variant.style);
            (weight_dist, style_dist)
        })
    }

    /// Itera sobre índices de todas as faces de uma família (case-insensitive).
    pub fn select_family<'a>(
        &'a self,
        family: &'a str,
    ) -> impl Iterator<Item = usize> + 'a {
        self.infos
            .iter()
            .enumerate()
            .filter(move |(_, info)| info.family.eq_ignore_ascii_case(family))
            .map(|(i, _)| i)
    }

    /// **P875** — devolve os índices de slots que podem cobrir `c`, i.e., cujo
    /// `coverage` contém o bloco de 256 codepoints a que `c` pertence.
    /// O chamador (L3) ainda deve confirmar com `face_covers_char`/`glyph_index`
    /// (o bitmap é aproximado por bloco), mas isto evita carregar faces cujo
    /// bitmap já exclui o caractere.
    pub fn candidates_for_char(&self, c: char) -> impl Iterator<Item = usize> + '_ {
        let codepoint = c as u32;
        self.infos
            .iter()
            .enumerate()
            .filter(move |(_, info)| info.coverage.contains(codepoint))
            .map(|(i, _)| i)
    }

    /// **P838** — escolhe, entre os índices candidatos, a fonte de fallback
    /// mais próxima de `like` e de `variant`, replicando o scoring do vanilla
    /// (`typst-library/src/text/font/book.rs:139-185` `find_best_variant`).
    ///
    /// A cobertura do caractere **não** é verificada aqui — o chamador (L3)
    /// entrega apenas candidatos que cobrem. Score por candidato (maior
    /// vence; comparação estritamente maior preserva o primeiro candidato em
    /// empate total, como no vanilla):
    ///
    /// 1. `similarity(candidato, like)` (se `like` for `Some`): match de
    ///    `monospace`, match de `serif`, palavras partilhadas no prefixo do
    ///    nome da família, e — em empate — família mais curta.
    /// 2. `Reverse(distance(candidato, variant))`: distâncias de estilo,
    ///    largura e peso.
    ///
    /// Divergência declarada (mecânica): o vanilla tem um 3.º elemento
    /// (preferência por fontes variáveis) e eixos na `distance`; o `FontInfo`
    /// cristalino não tem eixos/flag VARIABLE (VF via `axis_variations`).
    pub fn select_fallback(
        &self,
        like: Option<&FontInfo>,
        variant: &FontVariant,
        candidates: impl IntoIterator<Item = usize>,
    ) -> Option<usize> {
        let mut best = None;
        let mut best_score = None;

        for id in candidates {
            let Some(current) = self.infos.get(id) else { continue };
            let score = (
                like.map(|like| fallback_similarity(current, like)),
                std::cmp::Reverse(fallback_distance(current, variant)),
            );
            if best_score.is_none_or(|b| score > b) {
                best = Some(id);
                best_score = Some(score);
            }
        }

        best
    }

    fn literal_query(
        &self,
        family: &str,
        variant: &FontVariant,
        mut visit: impl FnMut(),
    ) -> Option<usize> {
        let start = self.literal_slots.partition_point(|&slot| {
            visit();
            literal_family_cmp(&self.infos[slot].family, family).is_lt()
        });
        self.literal_slots[start..]
            .iter()
            .copied()
            .take_while(|&slot| {
                visit();
                self.infos[slot].family.eq_ignore_ascii_case(family)
            })
            .min_by_key(|&slot| {
                let info = &self.infos[slot];
                (
                    info.variant.weight.distance(variant.weight),
                    info.variant.style.distance(variant.style),
                )
            })
    }

    #[cfg(test)]
    fn literal_index_bytes(&self) -> usize {
        std::mem::size_of_val(&self.literal_slots)
            + self.literal_slots.capacity() * std::mem::size_of::<usize>()
    }
    /// Selecciona o índice da fonte mais próxima de `(pattern, variant)`.
    ///
    /// Para `FontNamePattern::Literal` usa lookup exacto case-insensitive.
    /// Para `FontNamePattern::Regex` mantém o scan linear de todas as faces
    /// que casam. Critério de desempate: peso + estilo
    /// mais próximos (mesmo de `select`).
    pub fn select_pattern(
        &self,
        pattern: &FontNamePattern,
        variant: &FontVariant,
    ) -> Option<usize> {
        if let FontNamePattern::Literal(name) = pattern {
            return self.literal_query(name.as_str(), variant, || {});
        }

        let candidates: Vec<usize> = self
            .infos
            .iter()
            .enumerate()
            .filter(|(_, info)| pattern.is_match(&info.family))
            .map(|(i, _)| i)
            .collect();

        if candidates.is_empty() {
            return None;
        }

        candidates.into_iter().min_by_key(|&i| {
            let info = &self.infos[i];
            let weight_dist = info.variant.weight.distance(variant.weight);
            let style_dist = info.variant.style.distance(variant.style);
            (weight_dist, style_dist)
        })
    }
}

impl Default for FontBook {
    fn default() -> Self {
        Self::new()
    }
}

/// **P838** — similaridade entre duas faces para fallback, espelhando
/// `similarity` do vanilla (`book.rs:168-185`): primeiro o tipo de fonte
/// (monospace, serif), depois palavras partilhadas no prefixo do nome, e em
/// empate a família mais curta (menos especializada).
fn fallback_similarity(
    left: &FontInfo,
    right: &FontInfo,
) -> (bool, bool, usize, std::cmp::Reverse<usize>) {
    (
        left.flags.monospace == right.flags.monospace,
        left.flags.serif == right.flags.serif,
        shared_prefix_words(&left.family, &right.family),
        std::cmp::Reverse(left.family.len()),
    )
}

/// **P838** — distância de uma face à variante pedida, espelhando `distance`
/// do vanilla (`book.rs:192-226`) sem os eixos de variação (o `FontInfo`
/// cristalino não os tem — VF é tratada por `axis_variations`, P525/P836).
fn fallback_distance(info: &FontInfo, variant: &FontVariant) -> (u16, u16, u16) {
    (
        info.variant.style.distance(variant.style),
        info.variant.stretch.distance(variant.stretch),
        info.variant.weight.distance(variant.weight),
    )
}

/// Quantas palavras duas strings partilham no prefixo — mesma função do
/// vanilla (`book.rs:229-234`), com `unicode_words` (ADR-0013).
fn shared_prefix_words(left: &str, right: &str) -> usize {
    left.unicode_words()
        .zip(right.unicode_words())
        .take_while(|(l, r)| l == r)
        .count()
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
            coverage: Coverage::default(),
        }
    }

    #[test]
    fn font_weight_from_name_nomes_canonicos_passo_129() {
        assert_eq!(FontWeight::from_name("thin"), Some(FontWeight::THIN));
        assert_eq!(FontWeight::from_name("extralight"), Some(FontWeight::EXTRALIGHT));
        assert_eq!(FontWeight::from_name("light"), Some(FontWeight::LIGHT));
        assert_eq!(FontWeight::from_name("regular"), Some(FontWeight::REGULAR));
        assert_eq!(FontWeight::from_name("medium"), Some(FontWeight::MEDIUM));
        assert_eq!(FontWeight::from_name("semibold"), Some(FontWeight::SEMIBOLD));
        assert_eq!(FontWeight::from_name("bold"), Some(FontWeight::BOLD));
        assert_eq!(FontWeight::from_name("extrabold"), Some(FontWeight::EXTRABOLD));
        assert_eq!(FontWeight::from_name("black"), Some(FontWeight::BLACK));
    }

    #[test]
    fn font_weight_from_name_desconhecido_e_none_passo_129() {
        assert_eq!(FontWeight::from_name("arcoiris"), None);
        assert_eq!(FontWeight::from_name(""), None);
        assert_eq!(FontWeight::from_name("Bold"), None); // case-sensitive
        assert_eq!(FontWeight::from_name("normal"), None); // sem alias de "regular"
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
            family: "Test Family".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch(1000),
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });
        let idx = book.select(
            "Test Family",
            &FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch(1000),
            },
        );
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
        let idx = book
            .select(
                "Test",
                &FontVariant { weight: FontWeight::REGULAR, ..Default::default() },
            )
            .unwrap();
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

    // ── P838 — select_fallback: scoring de similaridade do vanilla ──────────
    //
    // Paridade com `typst-library/src/text/font/book.rs:139-185`
    // (`find_best_variant` + `similarity` + `distance`). O caso canónico é o
    // achado #24 de P831: texto CJK sem `font:` explícito com
    // `like` = Libertinus Serif (panose [0,…] → serif=false) deve escolher
    // `Noto Sans CJK JP` e não `Droid Sans Fallback` nem `Noto Serif CJK JP`.

    fn info_flags(family: &str, weight: u16, monospace: bool, serif: bool) -> FontInfo {
        FontInfo {
            family: family.into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight(weight),
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags { monospace, serif },
            coverage: Coverage::default(),
        }
    }

    #[test]
    fn p838_fontstretch_distance() {
        assert_eq!(FontStretch(1000).distance(FontStretch(1000)), 0);
        assert_eq!(FontStretch(750).distance(FontStretch(1250)), 500);
    }

    #[test]
    fn p838_select_fallback_caso_cjk_medido_p831() {
        // Réplica exacta do ambiente medido em P838: like = Libertinus Serif
        // (monospace=false, serif=false — panose [0,0,…]).
        let like = info_flags("Libertinus Serif", 400, false, false);
        let mut book = FontBook::new();
        book.push(info_flags("Droid Sans Fallback", 400, false, false)); // 0
        book.push(info_flags("Noto Serif CJK JP", 400, false, true)); // 1
        book.push(info_flags("Noto Sans Mono CJK JP", 400, true, false)); // 2
        book.push(info_flags("Noto Sans CJK JP", 400, false, false)); // 3
        book.push(info_flags("Noto Sans CJK KR", 400, false, false)); // 4

        let pick = book
            .select_fallback(Some(&like), &FontVariant::default(), 0..5)
            .unwrap();
        assert_eq!(
            pick, 3,
            "vanilla escolhe Noto Sans CJK JP: serif/mono match + família mais \
             curta que Droid; JP vence KR por empate total (ordem do book)"
        );
    }

    #[test]
    fn p838_select_fallback_serif_match() {
        // like com serif=true prefere candidato serif (tudo o resto igual,
        // a serifada é até mais curta no contra-exemplo para isolar o flag).
        let like = info_flags("Noto Serif", 400, false, true);
        let mut book = FontBook::new();
        book.push(info_flags("Xyz Sans CJK JP", 400, false, false)); // 0
        book.push(info_flags("Xyz Serif CJK JP", 400, false, true)); // 1
        let pick = book
            .select_fallback(Some(&like), &FontVariant::default(), 0..2)
            .unwrap();
        assert_eq!(pick, 1, "serif match tem prioridade sobre prefixo/comprimento");
    }

    #[test]
    fn p838_select_fallback_mono_match() {
        let like = info_flags("DejaVu Sans Mono", 400, true, false);
        let mut book = FontBook::new();
        book.push(info_flags("DejaVu Sans", 400, false, false)); // 0
        book.push(info_flags("Liberation Mono", 400, true, false)); // 1
        let pick = book
            .select_fallback(Some(&like), &FontVariant::default(), 0..2)
            .unwrap();
        assert_eq!(pick, 1, "monospace match tem prioridade máxima");
    }

    #[test]
    fn p838_select_fallback_shared_prefix_words() {
        // Exemplo do comentário do vanilla: like "Noto Sans" prefere
        // "Noto Sans Arabic" (2 palavras partilhadas) a "IBM Plex Arabic" (0).
        let like = info_flags("Noto Sans", 400, false, false);
        let mut book = FontBook::new();
        book.push(info_flags("IBM Plex Arabic", 400, false, false)); // 0
        book.push(info_flags("Noto Sans Arabic", 400, false, false)); // 1
        let pick = book
            .select_fallback(Some(&like), &FontVariant::default(), 0..2)
            .unwrap();
        assert_eq!(pick, 1);
    }

    #[test]
    fn p838_select_fallback_familia_mais_curta_em_empate() {
        // Exemplo do comentário do vanilla: like "Noto Sans Arabic" — "Noto
        // Sans" e "Noto Sans CJK HK" partilham 2 palavras; vence a mais curta.
        let like = info_flags("Noto Sans Arabic", 400, false, false);
        let mut book = FontBook::new();
        book.push(info_flags("Noto Sans CJK HK", 400, false, false)); // 0
        book.push(info_flags("Noto Sans", 400, false, false)); // 1
        let pick = book
            .select_fallback(Some(&like), &FontVariant::default(), 0..2)
            .unwrap();
        assert_eq!(pick, 1);
    }

    #[test]
    fn p838_select_fallback_distance_sem_like() {
        // Sem `like`, decide a distância à variante (peso aqui).
        let mut book = FontBook::new();
        book.push(info_flags("Test", 700, false, false)); // 0
        book.push(info_flags("Test", 300, false, false)); // 1
        let pick = book.select_fallback(None, &FontVariant::default(), 0..2).unwrap();
        assert_eq!(pick, 1, "peso 300 (dist 100) vence 700 (dist 300) para pedido 400");
    }

    #[test]
    fn p838_select_fallback_empate_total_primeiro_candidato() {
        // Comparação estritamente maior (como o vanilla): em empate total
        // vence o primeiro candidato do iterador.
        let mut book = FontBook::new();
        book.push(info_flags("Noto Sans CJK JP", 400, false, false)); // 0
        book.push(info_flags("Noto Sans CJK KR", 400, false, false)); // 1
        let pick = book.select_fallback(None, &FontVariant::default(), [1, 0]).unwrap();
        assert_eq!(pick, 1, "em empate total vence o primeiro do iterador");
    }

    #[test]
    fn p838_select_fallback_vazio_e_none() {
        let book = FontBook::new();
        let like = info_flags("Qualquer", 400, false, false);
        assert!(book
            .select_fallback(Some(&like), &FontVariant::default(), [])
            .is_none());
        assert!(book.select_fallback(None, &FontVariant::default(), []).is_none());
    }

    // ── P937 — cobertura Unicode exacta por runs de codepoints ────────────

    #[test]
    fn r6_coverage_clone_shares_immutable_sparse_runs() {
        use std::hash::{Hash, Hasher};
        let points: Vec<u32> = (0..65_536).map(|i| i * 16).collect();
        let original = Coverage::from_codepoints(points.clone());
        let copy = original.clone();
        assert_eq!(copy, original);
        assert_eq!(copy.iter().collect::<Vec<_>>(), points);
        let hash = |coverage: &Coverage| {
            let mut state = std::collections::hash_map::DefaultHasher::new();
            coverage.hash(&mut state);
            state.finish()
        };
        assert_eq!(hash(&copy), hash(&original));
        assert_eq!(copy.0.as_ptr(), original.0.as_ptr(), "immutable runs must share");
        drop(original);
        assert!(copy.contains(16));
        assert!(!copy.contains(17));
    }

    #[test]
    fn p937_coverage_vazia() {
        let cov = Coverage::new();
        assert!(cov.is_empty());
        assert!(!cov.contains('A' as u32));
    }

    #[test]
    fn p937_coverage_from_codepoints_exemplo_vanilla() {
        // Exemplo do vanilla (info.rs:274-284): {2,3,4,9,10,11,15,18,19}
        // → [2, 3, 4, 3, 3, 1, 2, 2].
        let cov = Coverage::from_codepoints([2, 3, 4, 9, 10, 11, 15, 18, 19]);
        assert_eq!(&*cov.0, &[2, 3, 4, 3, 3, 1, 2, 2]);
    }

    #[test]
    fn p937_coverage_contains_exacto() {
        let cov = Coverage::from_codepoints(['α' as u32]);
        assert!(cov.contains('α' as u32));
        // Outros codepoints do mesmo bloco grego não são falsos positivos.
        assert!(!cov.contains(0x0300));
        assert!(!cov.contains(0x03FF));
        assert!(!cov.contains('A' as u32));
    }

    #[test]
    fn p937_coverage_iter() {
        let cov = Coverage::from_codepoints([1, 2, 3, 10, 11, 20]);
        let points: Vec<u32> = cov.iter().collect();
        assert_eq!(points, vec![1, 2, 3, 10, 11, 20]);
    }

    #[test]
    fn p937_coverage_codepoint_maximo() {
        // U+10FFFF deve ser representável sem overflow.
        let cov = Coverage::from_codepoints([0x10FFFF]);
        assert!(cov.contains(0x10FFFF));
        assert!(!cov.contains(0x10FFFE));
    }

    #[test]
    fn coverage_interval_construction_matches_independent_set() {
        use std::collections::BTreeSet;
        let controls = [
            vec![],
            vec![0, 0, 0x10FFFF, 0x10FFFE, 0x10FFFF],
            vec![18, 19, 2, 4, 9, 11, 15, 3, 3, 10],
            (0..300).chain(100..450).chain(0..300).collect(),
            (0..300).rev().chain((200..450).rev()).collect(),
        ];
        for input in controls {
            let expected: BTreeSet<_> = input.iter().copied().collect();
            let coverage = Coverage::from_codepoints(input);
            assert_eq!(coverage.iter().collect::<BTreeSet<_>>(), expected);
            assert_eq!(
                coverage.iter().collect::<Vec<_>>(),
                expected.iter().copied().collect::<Vec<_>>()
            );
            for point in (0..500).chain([0x10FFFD, 0x10FFFE, 0x10FFFF]) {
                assert_eq!(coverage.contains(point), expected.contains(&point));
            }
        }
    }

    #[test]
    fn coverage_interval_construction_unordered_and_duplicate_inputs() {
        use std::collections::BTreeSet;
        let mut state = 0x1357_9BDFu32;
        for _ in 0..32 {
            let mut input = Vec::new();
            for _ in 0..512 {
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                input.push(state % 1024);
            }
            let expected: BTreeSet<_> = input.iter().copied().collect();
            let coverage = Coverage::from_codepoints(input.clone());
            input.reverse();
            assert_eq!(coverage, Coverage::from_codepoints(input));
            assert_eq!(
                coverage.iter().collect::<Vec<_>>(),
                expected.iter().copied().collect::<Vec<_>>()
            );
            for point in 0..=1024 {
                assert_eq!(coverage.contains(point), expected.contains(&point));
            }
        }
    }

    #[test]
    fn p937_candidates_for_char_exacto() {
        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "Greek".into(),
            variant: FontVariant::default(),
            flags: FontFlags::default(),
            coverage: Coverage::from_codepoints(['α' as u32]),
        });
        let found: Vec<usize> = book.candidates_for_char('α').collect();
        assert_eq!(found, vec![0]);

        // Mesmo bloco, codepoint não coberto → não é falso positivo.
        let not_found: Vec<usize> = book.candidates_for_char('β').collect();
        assert!(not_found.is_empty());
    }

    #[test]
    fn p937_candidates_for_char_exclui_nao_coberto() {
        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "Latin".into(),
            variant: FontVariant::default(),
            flags: FontFlags::default(),
            coverage: Coverage::new(),
        });
        let found: Vec<usize> = book.candidates_for_char('α').collect();
        assert!(found.is_empty());
    }
}

#[cfg(test)]
mod range_constructor_tests {
    use super::Coverage;
    use std::{collections::BTreeSet, ops::Range};

    #[test]
    fn range_constructor_unions_unordered_overlapping_adjacent_and_empty_ranges() {
        let cases: Vec<Vec<Range<u32>>> = vec![
            vec![],
            vec![0..1],
            vec![12..16, 2..5, 4..12, 3..3, Range { start: 8, end: 2 }, 12..16],
            vec![100..200, 0..20, 20..100],
            vec![0x10ffff..0x110000, 1..2],
        ];
        for input in cases {
            let expected: BTreeSet<u32> = input.iter().cloned().flatten().collect();
            let coverage = Coverage::from_ranges(input.clone());
            assert_eq!(
                coverage.iter().collect::<BTreeSet<_>>(),
                expected,
                "input={input:?}"
            );
            assert_eq!(
                coverage.iter().collect::<Vec<_>>(),
                expected.iter().copied().collect::<Vec<_>>()
            );
            for point in [
                0, 1, 2, 5, 11, 12, 19, 20, 99, 100, 199, 200, 0x10fffe, 0x10ffff,
                0x110000,
            ] {
                assert_eq!(coverage.contains(point), expected.contains(&point));
            }
            assert_eq!(coverage.is_empty(), expected.is_empty());
        }
    }

    #[test]
    fn range_constructor_matches_point_api_and_hash_under_permutations() {
        use std::hash::{Hash, Hasher};
        let hash = |c: &Coverage| {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            c.hash(&mut h);
            h.finish()
        };
        for seed in 0..32u32 {
            let mut ranges: Vec<_> = (0..20u32)
                .map(|i| {
                    let start = (i * 17 + seed * 11) % 97;
                    start..start + (i % 9)
                })
                .collect();
            let oracle = Coverage::from_codepoints(
                ranges.iter().cloned().flatten().collect::<Vec<_>>(),
            );
            for _ in 0..3 {
                let c = Coverage::from_ranges(ranges.clone());
                assert_eq!(c, oracle);
                assert_eq!(hash(&c), hash(&oracle));
                ranges.rotate_left(3);
                ranges.reverse();
            }
        }
    }

    #[test]
    fn range_constructor_preserves_full_unicode_and_neutral_upper_endpoint() {
        let c = Coverage::from_ranges(std::iter::once(0..0x110000));
        assert_eq!(c.iter().count(), 0x110000);
        assert!(c.contains(0) && c.contains(0x10ffff));
        assert!(!c.contains(0x110000));
        let c = Coverage::from_ranges(std::iter::once(u32::MAX - 2..u32::MAX));
        assert_eq!(c.iter().collect::<Vec<_>>(), vec![u32::MAX - 2, u32::MAX - 1]);
        assert!(!c.contains(u32::MAX));
    }
}

#[cfg(test)]
mod literal_index_tests {
    use super::*;
    use std::hash::{Hash, Hasher};

    fn info(name: &str, weight: u16, style: FontStyle, stretch: u16) -> FontInfo {
        FontInfo {
            family: name.into(),
            variant: FontVariant {
                weight: FontWeight(weight),
                style,
                stretch: FontStretch(stretch),
            },
            flags: FontFlags::default(),
            coverage: Coverage::from_codepoints(vec![65, 66, 937]),
        }
    }
    fn pattern(s: &str) -> FontNamePattern {
        FontNamePattern::Literal(s.into())
    }
    fn old_pattern(
        book: &FontBook,
        p: &FontNamePattern,
        v: &FontVariant,
    ) -> Option<usize> {
        book.infos
            .iter()
            .enumerate()
            .filter(|(_, i)| p.is_match(&i.family))
            .map(|(id, _)| id)
            .min_by_key(|&id| {
                let i = &book.infos[id];
                (i.variant.weight.distance(v.weight), i.variant.style.distance(v.style))
            })
    }
    fn hash(value: &impl Hash) -> u64 {
        let mut h = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut h);
        h.finish()
    }

    #[test]
    fn literal_queries_have_logarithmic_family_comparison_work() {
        let mut book = FontBook::new();
        for i in 0..4096 {
            book.push(info(&format!("Family {i:04}"), 400, FontStyle::Normal, 1000));
        }
        for name in ["Family 4095", "FAMILY 2048", "Absent Z", "", "Family 0000"] {
            let mut work = 0;
            let got = book.literal_query(name, &FontVariant::default(), || work += 1);
            assert_eq!(got, book.select(name, &FontVariant::default()));
            assert_eq!(got, book.select_pattern(&pattern(name), &FontVariant::default()));
            assert!(work<=16,"literal query scans unrelated families: {name:?}, actual comparisons={work}");
        }
    }

    #[test]
    fn variants_case_unicode_and_regex_preserve_original_selection() {
        let names = [
            "",
            "A",
            "a",
            "AA",
            "Alpha",
            "ALPHA",
            "alpha",
            "Zeta",
            "École",
            "école",
            "İ",
            "i\u{307}",
            "Σ",
            "σ",
            "Straße",
            "STRASSE",
            "name\0tail",
        ];
        let mut book = FontBook::new();
        for n in names {
            for weight in [100, 300, 400, 500, 900] {
                for style in [FontStyle::Normal, FontStyle::Italic, FontStyle::Oblique] {
                    book.push(info(
                        n,
                        weight,
                        style,
                        if weight == 300 { 500 } else { 1500 },
                    ));
                }
            }
        }
        let before = book.infos.clone();
        let mut ps = names.iter().map(|n| pattern(n)).collect::<Vec<_>>();
        ps.extend([pattern("Not Here"), pattern("éCOLE"), pattern("alphA")]);
        for regex in [".*", "(?i)^a", "^$", "É|é", "NoSuchFamily"] {
            ps.push(FontNamePattern::Regex(
                crate::entities::regex::Regex::new(regex).unwrap(),
            ));
        }
        for weight in [100, 250, 350, 400, 450, 700, 900] {
            for style in [FontStyle::Normal, FontStyle::Italic, FontStyle::Oblique] {
                for stretch in [500, 1000, 1500] {
                    let v = FontVariant {
                        weight: FontWeight(weight),
                        style,
                        stretch: FontStretch(stretch),
                    };
                    for p in &ps {
                        assert_eq!(
                            book.select_pattern(p, &v),
                            old_pattern(&book, p, &v),
                            "{p:?} {v:?}"
                        );
                    }
                }
            }
        }
        assert_eq!(before, book.infos);
        assert_eq!(
            book.select_pattern(&pattern("A"), &FontVariant::default()),
            book.select("a", &FontVariant::default())
        );
    }

    #[test]
    fn append_after_miss_and_better_variant_never_reuses_stale_selection() {
        let mut a = FontBook::new();
        let mut b = FontBook::new();
        let v = FontVariant::default();
        assert_eq!(a.select_pattern(&pattern("Target"), &v), None);
        a.push(info("Other", 400, FontStyle::Normal, 1000));
        a.push(info("TARGET", 700, FontStyle::Normal, 1000));
        assert_eq!(a.select_pattern(&pattern("target"), &v), Some(1));
        assert_eq!(b.select_pattern(&pattern("target"), &v), None);
        a.push(info("target", 400, FontStyle::Normal, 1500));
        assert_eq!(a.select_pattern(&pattern("Target"), &v), Some(2));
        a.push(info("Target", 400, FontStyle::Normal, 500));
        assert_eq!(a.select_pattern(&pattern("Target"), &v), Some(2));
        b.push(info("Target", 400, FontStyle::Normal, 1000));
        assert_eq!(b.select_pattern(&pattern("target"), &v), Some(0));
        assert_eq!(a.select_family("target").collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(
            a.infos.iter().map(|i| i.family.as_str()).collect::<Vec<_>>(),
            vec!["Other", "TARGET", "target", "Target"]
        );
    }

    #[test]
    fn equal_family_range_work_includes_variants_but_not_unrelated_faces() {
        let mut b = FontBook::new();
        for i in 0..2048 {
            b.push(info(&format!("Z{i:04}"), 400, FontStyle::Normal, 1000));
        }
        for i in 0..64 {
            b.push(info(
                if i % 2 == 0 { "Target" } else { "TARGET" },
                400,
                FontStyle::Normal,
                1000,
            ));
        }
        let mut work = 0;
        assert_eq!(
            b.literal_query("target", &FontVariant::default(), || work += 1),
            Some(2048)
        );
        assert!(work <= 80, "family range must not revisit unrelated faces: {work}");
    }

    #[test]
    fn book_identity_is_exactly_ordered_metadata_and_storage_is_linear() {
        let mut a = FontBook::new();
        let mut b = FontBook::new();
        for size in 1usize..=4096 {
            let i =
                info(&format!("Family {:04}", 4096 - size), 400, FontStyle::Normal, 1000);
            a.push(i.clone());
            b.push(i);
            if size.is_power_of_two() || size == 1112 {
                assert!(a == b);
                assert_eq!(hash(&a), hash(&a.infos));
                assert_eq!(hash(&a), hash(&b));
                assert!(a.literal_index_bytes() <= 16 * size + 64);
                println!(
                    "LITERAL_INDEX_STORAGE slots={} bytes={}",
                    size,
                    a.literal_index_bytes()
                );
            }
        }
        b.push(info("Another", 400, FontStyle::Normal, 1000));
        assert!(a != b);
        assert!(a.infos[0].coverage.contains(937));
    }
}

#[cfg(test)]
mod coverage_query_index_tests {
    use super::*;
    use std::{
        collections::BTreeSet,
        hash::{Hash, Hasher},
    };
    fn sparse() -> Coverage {
        Coverage::from_codepoints((0..4096).map(|i| i * 3).collect::<Vec<_>>())
    }
    fn examined(c: &Coverage, cp: u32) -> (bool, usize) {
        let mut n = 0;
        let result = c.contains_visit(cp, || n += 1);
        (result, n)
    }
    #[test]
    fn sparse_late_hit_has_bounded_actual_entry_reads() {
        let c = sparse();
        let (yes, n) = examined(&c, 4095 * 3);
        assert!(yes);
        assert!(n <= 32, "actual coverage query examined {n} entries; budget32");
    }
    #[test]
    fn sparse_gap_miss_has_bounded_actual_entry_reads() {
        let c = sparse();
        let (yes, n) = examined(&c, 4095 * 3 - 1);
        assert!(!yes);
        assert!(n <= 32, "actual gap query examined {n} entries; budget32");
    }
    #[test]
    fn high_absent_has_bounded_actual_entry_reads() {
        let c = sparse();
        let (yes, n) = examined(&c, 0x10ffff);
        assert!(!yes);
        assert!(n <= 32, "actual absent query examined {n} entries; budget32");
    }
    fn linear(c: &Coverage, cp: u32) -> bool {
        let mut cursor = 0;
        let mut inside = false;
        for &run in c.0.iter() {
            if cursor > cp {
                return false;
            }
            if (cursor..cursor + run).contains(&cp) {
                return inside;
            }
            cursor += run;
            inside = !inside;
        }
        false
    }
    #[test]
    fn membership_iteration_and_canonical_runs_match_independent_set() {
        let mut seed = 123456789u32;
        for trial in 0..48 {
            let mut input = Vec::new();
            for _ in 0..512 {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                input.push(seed % 4096);
            }
            input.extend([0, 0x10ffff, 0xd800, 0x10000, 0x1f600]);
            let expected: BTreeSet<_> = input.iter().copied().collect();
            let c = Coverage::from_codepoints(input.clone());
            input.reverse();
            let d = Coverage::from_codepoints(input);
            assert_eq!(c, d);
            assert_eq!(&*c.0, &*d.0);
            assert_eq!(
                c.iter().collect::<Vec<_>>(),
                expected.iter().copied().collect::<Vec<_>>()
            );
            for cp in (0..4200).chain([
                0xd7ff,
                0xd800,
                0xd801,
                0xffff,
                0x10000,
                0x1f600,
                0x10fffe,
                0x10ffff,
                0x110000,
                u32::MAX,
            ]) {
                assert_eq!(
                    c.contains(cp),
                    expected.contains(&cp),
                    "trial {trial} cp {cp}"
                );
                assert_eq!(c.contains(cp), linear(&c, cp));
            }
        }
    }
    #[test]
    fn range_boundaries_blocks_and_supported_u32_endpoints() {
        let ranges = [
            0..2,
            2..5,
            12..15,
            9..10,
            7..7,
            std::ops::Range { start: 20, end: 18 },
            0xd7ff..0xd802,
            0x10fffe..0x110000,
            u32::MAX - 3..u32::MAX,
        ];
        let points: BTreeSet<_> = ranges.iter().flat_map(|r| r.clone()).collect();
        let c = Coverage::from_ranges(ranges.clone());
        assert_eq!(
            c.iter().collect::<Vec<_>>(),
            points.iter().copied().collect::<Vec<_>>()
        );
        for cp in (0..30).chain([
            0xd7fe,
            0xd7ff,
            0xd800,
            0xd801,
            0xd802,
            0x10fffd,
            0x10fffe,
            0x10ffff,
            0x110000,
            u32::MAX - 4,
            u32::MAX - 3,
            u32::MAX - 1,
            u32::MAX,
        ]) {
            assert_eq!(c.contains(cp), points.contains(&cp));
        }
        for pairs in [0, 1, 7, 8, 9, 15, 16, 17, 64, 4096] {
            let c = Coverage::from_ranges((0..pairs).map(|i| i * 3..i * 3 + 1));
            for cp in 0..pairs * 3 + 3 {
                assert_eq!(c.contains(cp), cp % 3 == 0 && cp / 3 < pairs);
            }
        }
    }
    #[derive(Default, Debug, PartialEq, Eq)]
    struct Trace(Vec<Vec<u8>>);
    impl Hasher for Trace {
        fn finish(&self) -> u64 {
            0
        }
        fn write(&mut self, b: &[u8]) {
            self.0.push(b.to_vec());
        }
    }
    #[test]
    fn equality_hash_debug_and_clone_storage_keep_old_contract() {
        for c in [
            Coverage::new(),
            Coverage::from_codepoints([2, 3, 4, 9, 10, 11, 15, 18, 19]),
            sparse(),
        ] {
            let d = c.clone();
            let a = c.query_backings();
            let b = d.query_backings();
            assert_eq!(a, b);
            let mut h = Trace::default();
            c.hash(&mut h);
            let mut old = Trace::default();
            c.0.hash(&mut old);
            assert_eq!(h, old);
            assert_eq!(format!("{c:?}"), format!("Coverage({:?})", c.0));
            assert_eq!(c, d);
            let runs = c.0.len();
            assert_eq!(a[0].1, runs);
            assert!(a[1].1 <= runs.div_ceil(16));
            assert!(std::mem::size_of::<Coverage>() <= 32);
            println!("COVERAGE_QUERY_STORAGE runs={} canonical_bytes={} derived_bytes={} struct_bytes={}",runs,a[0].1*4,a[1].1*4,std::mem::size_of::<Coverage>());
            drop(c);
            for cp in [0, 2, 3, 7, 9, 4095 * 3, 0x10ffff] {
                assert_eq!(d.contains(cp), linear(&d, cp));
            }
        }
    }
    #[test]
    fn independent_books_preserve_candidate_indices_and_replacement() {
        fn info(name: &str, coverage: Coverage) -> FontInfo {
            FontInfo {
                family: name.into(),
                variant: FontVariant::default(),
                flags: FontFlags::default(),
                coverage,
            }
        }
        let mut b = FontBook::new();
        b.push(info("A", sparse()));
        b.push(info("B", Coverage::from_codepoints([3, 7, 0x10ffff])));
        b.push(info("A", Coverage::new()));
        for cp in [0, 3, 7, 4095 * 3, 0x10ffff] {
            let expected: Vec<_> = b
                .infos()
                .iter()
                .enumerate()
                .filter_map(|(i, f)| linear(&f.coverage, cp).then_some(i))
                .collect();
            assert_eq!(
                b.candidates_for_char(char::from_u32(cp).unwrap()).collect::<Vec<_>>(),
                expected
            );
        }
        let empty = FontBook::new();
        assert!(empty.candidates_for_char('x').next().is_none());
        let mut clone = FontBook::new();
        for info in b.infos() {
            clone.push(info.clone());
        }
        assert_eq!(clone.candidates_for_char('\u{10ffff}').collect::<Vec<_>>(), vec![1]);
    }
}
