//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 59811524
//! @layer L1
//! @updated 2026-04-24
//!
//! Helper puro de hyphenation (Passo 144, ADR-0057). Wrap fino
//! sobre `hypher::hyphenate`, mapeando o nosso `Lang` (ISO 639-1/2/3
//! ASCII, ADR-0052) para `hypher::Lang` e devolvendo posições de
//! quebra em **chars** dentro da palavra original. Sem I/O — padrões
//! TeX vivem como bytes embebidos pelo `hypher` em compile-time
//! (ADR-0029 + ADR-0030).
//!
//! Política de fallback (ADR-0057):
//! - Idioma não suportado pelo `hypher` → `Vec::new()` (silent).
//! - Lang com 3 letras (ISO 639-2/3) → `Vec::new()` (`hypher` só
//!   aceita códigos de 2 letras).
//! - Palavra sem pontos de quebra (uma sílaba) → `Vec::new()`.

use crate::entities::lang::Lang;

/// Devolve índices, em **chars**, onde um hífen de quebra pode
/// ser inserido na palavra para o idioma dado.
///
/// Ex: `"extensive"` em `"en"` → `vec![2, 5]` (corresponde a
/// `ex|ten|sive`).
///
/// `Vec::new()` quando idioma não é suportado, código não é 2
/// letras, ou palavra não tem pontos de quebra.
pub fn hyphenate(word: &str, lang: &Lang) -> Vec<usize> {
    let iso = lang.as_str();
    if iso.len() != 2 {
        return Vec::new();
    }
    let bytes = iso.as_bytes();
    let code: [u8; 2] = [bytes[0], bytes[1]];
    let Some(hypher_lang) = hypher::Lang::from_iso(code) else {
        return Vec::new();
    };

    let syllables: Vec<&str> = hypher::hyphenate(word, hypher_lang).collect();
    if syllables.len() <= 1 {
        return Vec::new();
    }

    let mut points = Vec::with_capacity(syllables.len() - 1);
    let mut acc = 0usize;
    for syll in syllables.iter().take(syllables.len() - 1) {
        acc += syll.chars().count();
        points.push(acc);
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn hyphenate_palavra_en_devolve_pontos_correctos() {
        let lang = Lang::ENGLISH;
        let points = hyphenate("extensive", &lang);
        assert_eq!(points, vec![2, 5],
            "hypher segmenta \"extensive\" em ex|ten|sive (en)");
    }

    #[test]
    fn hyphenate_palavra_pt_devolve_pontos() {
        let lang = Lang::from_str("pt").unwrap();
        let points = hyphenate("exemplo", &lang);
        // hypher PT pattern produces at least one breakpoint;
        // exact positions are crate-specific. Verificamos não-vazio
        // e dentro de range.
        assert!(!points.is_empty(),
            "\"exemplo\" em pt deve ter pelo menos um ponto de quebra");
        for p in &points {
            assert!(*p > 0 && *p < "exemplo".chars().count(),
                "ponto {} fora de range (1..6)", p);
        }
    }

    #[test]
    fn hyphenate_idioma_3_letras_devolve_vazio() {
        // Lang ISO 639-3 (3 letras) — hypher só aceita 2-letras.
        let lang = Lang::from_str("por").unwrap();
        assert_eq!(hyphenate("exemplo", &lang), Vec::<usize>::new(),
            "código 3-letras devolve vec vazio (silent skip)");
    }

    #[test]
    fn hyphenate_idioma_sem_padroes_devolve_vazio() {
        // Código ISO 2-letras improvável: "xx" não está nos
        // padrões TeX do hypher.
        let lang = Lang::from_str("xx").unwrap();
        assert_eq!(hyphenate("anything", &lang), Vec::<usize>::new(),
            "idioma desconhecido pelo hypher → silent skip");
    }

    #[test]
    fn hyphenate_palavra_curta_devolve_vazio() {
        // hypher respeita bounds (left_min, right_min) — palavras
        // curtas não produzem quebras.
        let lang = Lang::ENGLISH;
        assert_eq!(hyphenate("ao", &lang), Vec::<usize>::new(),
            "palavra de 2 chars sem pontos de quebra (en bounds 2,3)");
    }
}
