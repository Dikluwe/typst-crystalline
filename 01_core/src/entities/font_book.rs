//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/font-book.md
//! @prompt-hash 83090b0b
//! @layer L1
//! @updated 2026-03-27

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

/// Bitmap de cobertura Unicode por blocos de 256 codepoints.
/// P875 — usado pelo shaper para filtrar o fallback global: um candidato só
/// é considerado para um caractere se o bloco desse caractere estiver no bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coverage {
    /// 64 × 64 = 4096 bits → blocos 0..4095 (U+0000..U+0FFFFF).
    pub blocks: [u64; 64],
}

impl Coverage {
    /// Cobertura vazia.
    pub fn new() -> Self {
        Self { blocks: [0; 64] }
    }

    /// Verifica se o bitmap está completamente vazio.
    pub fn is_empty(&self) -> bool {
        self.blocks.iter().all(|b| *b == 0)
    }

    /// Marca o bloco de 256 codepoints a que `codepoint` pertence.
    pub fn insert(&mut self, codepoint: u32) {
        let block = (codepoint / 256) as usize;
        if block >= 4096 {
            return;
        }
        let word = block / 64;
        let bit = block % 64;
        self.blocks[word] |= 1u64 << bit;
    }

    /// Verifica se o bloco de 256 codepoints a que `codepoint` pertence está marcado.
    pub fn contains(&self, codepoint: u32) -> bool {
        let block = (codepoint / 256) as usize;
        if block >= 4096 {
            return false;
        }
        let word = block / 64;
        let bit = block % 64;
        (self.blocks[word] >> bit) & 1 != 0
    }
}

impl Default for Coverage {
    fn default() -> Self {
        Self::new()
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
    /// Cobertura Unicode aproximada por blocos de 256 codepoints.
    pub coverage: Coverage,
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

    /// Selecciona o índice da fonte mais próxima de `(pattern, variant)`.
    ///
    /// Para `FontNamePattern::Literal` usa lookup exacto case-insensitive.
    /// Para `FontNamePattern::Regex` faz scan linear O(n) e retorna a
    /// primeira face que matcha. Critério de desempate: peso + estilo
    /// mais próximos (mesmo de `select`).
    pub fn select_pattern(
        &self,
        pattern: &FontNamePattern,
        variant: &FontVariant,
    ) -> Option<usize> {
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
        let pick = book
            .select_fallback(None, &FontVariant::default(), 0..2)
            .unwrap();
        assert_eq!(pick, 1, "peso 300 (dist 100) vence 700 (dist 300) para pedido 400");
    }

    #[test]
    fn p838_select_fallback_empate_total_primeiro_candidato() {
        // Comparação estritamente maior (como o vanilla): em empate total
        // vence o primeiro candidato do iterador.
        let mut book = FontBook::new();
        book.push(info_flags("Noto Sans CJK JP", 400, false, false)); // 0
        book.push(info_flags("Noto Sans CJK KR", 400, false, false)); // 1
        let pick = book
            .select_fallback(None, &FontVariant::default(), [1, 0])
            .unwrap();
        assert_eq!(pick, 1, "em empate total vence o primeiro do iterador");
    }

    #[test]
    fn p838_select_fallback_vazio_e_none() {
        let book = FontBook::new();
        let like = info_flags("Qualquer", 400, false, false);
        assert!(book.select_fallback(Some(&like), &FontVariant::default(), []).is_none());
        assert!(book.select_fallback(None, &FontVariant::default(), []).is_none());
    }

    // ── P875 — cobertura Unicode por bloco de 256 codepoints ───────────────

    #[test]
    fn p875_coverage_insert_contains_por_bloco() {
        let mut cov = Coverage::new();
        assert!(!cov.contains('α' as u32)); // U+03B1, bloco 0x03
        cov.insert('α' as u32);
        assert!(cov.contains('α' as u32));
        // Qualquer codepoint do mesmo bloco (0x0300..0x03FF) está marcado.
        assert!(cov.contains(0x0300));
        assert!(cov.contains(0x03FF));
        // Outro bloco não está.
        assert!(!cov.contains('A' as u32));
    }

    #[test]
    fn p875_coverage_ignora_codepoints_acima_do_bitmap() {
        let mut cov = Coverage::new();
        // 4096 blocos × 256 = 0x100000; codepoints >= 0x100000 são ignorados.
        cov.insert(0x100000);
        assert!(cov.is_empty());
    }

    #[test]
    fn p875_candidates_for_char_inclui_bloco_coberto() {
        let mut cov = Coverage::new();
        cov.insert('α' as u32);
        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "Greek".into(),
            variant: FontVariant::default(),
            flags: FontFlags::default(),
            coverage: cov,
        });
        let found: Vec<usize> = book.candidates_for_char('α').collect();
        assert_eq!(found, vec![0]);
    }

    #[test]
    fn p875_candidates_for_char_exclui_bloco_nao_coberto() {
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
