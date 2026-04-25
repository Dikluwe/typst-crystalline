//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/lang.md
//! @prompt-hash 4426dbc0
//! @layer L1
//! @updated 2026-04-25
//!
//! Smart-quotes lang-aware (Passo 155, ADR-0060 Fase 1, sub-passo 2).
//!
//! Tabela estática mapeando códigos ISO 639-1/2/3 a pares de aspas
//! `(open, close)`. Lookup linear por exact match no `Lang::as_str()`;
//! fallback `DEFAULT_QUOTES` (ASCII) para línguas não listadas.
//!
//! Cristalino's `Lang` (ADR-0052) é 2-3 letras ASCII puro — não tem
//! region/country (e.g. `pt-BR`); por isso o lookup BCP47 com prefixo
//! sugerido na spec P155 simplifica para exact match.

use crate::entities::lang::Lang;

/// Tabela inicial de aspas primárias por idioma (Passo 155).
///
/// Cobertura inicial: 6 idiomas + default ASCII.
/// Outras línguas (zh, ja, ar, ...) caem em `DEFAULT_QUOTES`; expansível
/// em passo futuro sem breaking change.
const LANG_QUOTES: &[(&str, (&str, &str))] = &[
    // (lang_code, (open, close))
    ("pt", ("\u{00AB}", "\u{00BB}")),               // « »
    ("en", ("\u{201C}", "\u{201D}")),               // " "
    ("de", ("\u{201E}", "\u{201C}")),               // „ "
    ("fr", ("\u{00AB}\u{00A0}", "\u{00A0}\u{00BB}")), // « »  com NBSP
    ("es", ("\u{00AB}", "\u{00BB}")),               // « »
    ("it", ("\u{00AB}", "\u{00BB}")),               // « »
];

/// Aspas default (ASCII) para línguas não cobertas pela tabela.
pub const DEFAULT_QUOTES: (&str, &str) = ("\"", "\"");

/// Devolve par `(open, close)` de aspas primárias para o `Lang` dado.
///
/// Lookup por exact match no código ISO. Línguas não cobertas devolvem
/// `DEFAULT_QUOTES` (ASCII).
pub fn localize_quotes(lang: &Lang) -> (&'static str, &'static str) {
    let code = lang.as_str();
    for (key, pair) in LANG_QUOTES.iter() {
        if *key == code {
            return *pair;
        }
    }
    DEFAULT_QUOTES
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn localize_quotes_pt_devolve_aspas_baixas() {
        let lang = Lang::from_str("pt").unwrap();
        assert_eq!(localize_quotes(&lang), ("\u{00AB}", "\u{00BB}"));
    }

    #[test]
    fn localize_quotes_en_devolve_curly() {
        let lang = Lang::from_str("en").unwrap();
        assert_eq!(localize_quotes(&lang), ("\u{201C}", "\u{201D}"));
    }

    #[test]
    fn localize_quotes_de_devolve_par_germanico() {
        let lang = Lang::from_str("de").unwrap();
        // Alemão: open low (U+201E „), close high left (U+201C ").
        assert_eq!(localize_quotes(&lang), ("\u{201E}", "\u{201C}"));
    }

    #[test]
    fn localize_quotes_fr_inclui_nbsp() {
        let lang = Lang::from_str("fr").unwrap();
        let (open, close) = localize_quotes(&lang);
        assert!(open.contains('\u{00A0}'), "FR open deve conter NBSP: {:?}", open);
        assert!(close.contains('\u{00A0}'), "FR close deve conter NBSP: {:?}", close);
    }

    #[test]
    fn localize_quotes_lang_desconhecido_devolve_default_ascii() {
        // 'jp' (3-letter for Japanese) ou outro lang fora da tabela.
        let lang = Lang::from_str("jp").unwrap();
        assert_eq!(localize_quotes(&lang), DEFAULT_QUOTES);
        assert_eq!(localize_quotes(&lang), ("\"", "\""));
    }

    #[test]
    fn localize_quotes_es_e_it_partilham_chevrons() {
        let es = Lang::from_str("es").unwrap();
        let it = Lang::from_str("it").unwrap();
        assert_eq!(localize_quotes(&es), ("\u{00AB}", "\u{00BB}"));
        assert_eq!(localize_quotes(&it), ("\u{00AB}", "\u{00BB}"));
    }

    #[test]
    fn localize_quotes_3_letter_iso_caia_default() {
        // ISO 639-2/3 codes (3 letras) que não estão na tabela.
        // `por` (Portuguese) é o 3-letter code; vão a default.
        let lang = Lang::from_str("por").unwrap();
        assert_eq!(localize_quotes(&lang), DEFAULT_QUOTES);
    }
}
