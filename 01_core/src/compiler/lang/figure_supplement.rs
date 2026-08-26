//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/lang/figure_supplement.md
//! @prompt-hash e7e4ec4d
//! @layer L1
//! @updated 2026-04-27
//!
//! Supplement automático por lang em figure (Passo 158B,
//! Model figure-kinds sub-passo 2).
//!
//! Tabela estática mapeando `(kind, lang)` para prefix
//! localizado ("Figure"/"Figura"/"Abbildung"/etc.). Lookup
//! linear por par exact match; **fallback EN** — default da
//! linguagem Typst (P1034; era PT até lá, por backwards compat
//! com os próprios testes).
//!
//! Reuso explícito do padrão `localize_quotes(lang)` em
//! `quotes.rs` (P155) — primeiro reuso cross-feature do
//! pattern P155 (subpadrão emergente N=1).
//!
//! Cobertura inicial: 3 kinds × 6 langs = 18 entradas + EN
//! fallback. Outras langs/kinds caem no fallback EN;
//! expansível em passo futuro sem breaking change (NÃO
//! reservado per política P158).

use crate::entities::lang::Lang;

/// Tabela inicial de supplements por (kind, lang) — Passo 158B.
///
/// Cobertura: 3 kinds (image/table/raw) × 6 langs (pt/en/de/
/// fr/es/it) = 18 entradas. Outras combinações caem no fallback
/// EN (P1034).
const LANG_SUPPLEMENTS: &[((&str, &str), &str)] = &[
    // (kind, lang) → supplement
    (("image", "pt"), "Figura"),
    (("image", "en"), "Figure"),
    (("image", "de"), "Abbildung"),
    (("image", "fr"), "Figure"),
    (("image", "es"), "Figura"),
    (("image", "it"), "Figura"),
    (("table", "pt"), "Tabela"),
    (("table", "en"), "Table"),
    (("table", "de"), "Tabelle"),
    (("table", "fr"), "Tableau"),
    (("table", "es"), "Tabla"),
    (("table", "it"), "Tabella"),
    (("raw", "pt"), "Listagem"),
    (("raw", "en"), "Listing"),
    (("raw", "de"), "Listing"),
    (("raw", "fr"), "Listing"),
    (("raw", "es"), "Listado"),
    (("raw", "it"), "Listato"),
];

/// Default fallback supplement por kind quando `lang` é `None` ou
/// desconhecido.
///
/// **P1034** — passou de PT para **EN**. O default da linguagem Typst é
/// `en` (`Lang::ENGLISH`, vanilla `TextElem::lang` `#[default(Lang::ENGLISH)]`),
/// logo um documento sem `#set text(lang:)` tem de dizer "Figure", não
/// "Figura" — medido contra o vanilla em P1031/P1034.
///
/// A escolha anterior (PT) vinha de P158B e a razão registada era
/// **"preservar backwards compat com tests pré-existentes que esperam
/// 'Figura'"** — isto é, o fallback estava alinhado com os testes, não com a
/// linguagem. Era a causa-raiz do Achado 6, e não um acidente do ambiente de
/// build. Corrigir aqui cobre os três chamadores de uma vez
/// (`introspect_with_introspector`, `fixpoint::run_fixpoint` e o helper de
/// teste), que passam `lang: None`.
const DEFAULT_SUPPLEMENTS_EN: &[(&str, &str)] =
    &[("image", "Figure"), ("table", "Table"), ("raw", "Listing")];

/// Devolve supplement localizado para `(kind, lang)`.
///
/// **Lookup**:
/// 1. Tenta exact match em `LANG_SUPPLEMENTS` por `(kind,
///    lang.as_str())`.
/// 2. Se lang desconhecido, tenta o fallback EN em
///    `DEFAULT_SUPPLEMENTS_EN` para o `kind`.
/// 3. Se kind também desconhecido, devolve `kind` capitalizado
///    (primeira letra maiúscula).
///
/// `lang: None` (não setado) → **inglês**, que é o default da linguagem
/// (P1034); antes caía em PT.
pub fn figure_supplement_for_lang(kind: &str, lang: Option<&Lang>) -> String {
    // Tenta exact match (kind, lang).
    if let Some(l) = lang {
        let code = l.as_str();
        for ((k, lc), supp) in LANG_SUPPLEMENTS.iter() {
            if *k == kind && *lc == code {
                return supp.to_string();
            }
        }
    }
    // Fallback EN por kind (P1034 — default da linguagem é `en`).
    for (k, supp) in DEFAULT_SUPPLEMENTS_EN.iter() {
        if *k == kind {
            return supp.to_string();
        }
    }
    // Kind desconhecido — capitalizar primeira letra.
    capitalize_first(kind)
}

/// Capitaliza a primeira letra de uma string (UTF-8 aware).
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn lookup_image_pt_devolve_figura() {
        let lang = Lang::from_str("pt").unwrap();
        assert_eq!(figure_supplement_for_lang("image", Some(&lang)), "Figura");
    }

    #[test]
    fn lookup_table_de_devolve_tabelle() {
        let lang = Lang::from_str("de").unwrap();
        assert_eq!(figure_supplement_for_lang("table", Some(&lang)), "Tabelle");
    }

    #[test]
    fn lookup_raw_it_devolve_listato() {
        let lang = Lang::from_str("it").unwrap();
        assert_eq!(figure_supplement_for_lang("raw", Some(&lang)), "Listato");
    }

    #[test]
    fn fallback_lang_desconhecido_devolve_en() {
        // **P1034** — inverte o fallback de P158B §8.2 (que era PT "para
        // backwards compat" com os próprios testes). Medido no vanilla
        // ratificado: `#set text(lang: "jp")` + figure → "Figure 1".
        let lang = Lang::from_str("jp").unwrap();
        assert_eq!(figure_supplement_for_lang("image", Some(&lang)), "Figure");
        assert_eq!(figure_supplement_for_lang("table", Some(&lang)), "Table");
        assert_eq!(figure_supplement_for_lang("raw", Some(&lang)), "Listing");
    }

    #[test]
    fn fallback_lang_none_devolve_en() {
        // **P1034** — `lang` não setado = default da linguagem = `en`.
        assert_eq!(figure_supplement_for_lang("image", None), "Figure");
        assert_eq!(figure_supplement_for_lang("table", None), "Table");
    }

    #[test]
    fn fallback_kind_desconhecido_devolve_capitalizado() {
        // kind = "custom" não está em LANG_SUPPLEMENTS nem
        // DEFAULT_SUPPLEMENTS_EN → devolve "Custom" (capitalizado).
        let lang = Lang::from_str("en").unwrap();
        assert_eq!(figure_supplement_for_lang("custom", Some(&lang)), "Custom");
        // Idem com lang None.
        assert_eq!(figure_supplement_for_lang("custom", None), "Custom");
    }

    #[test]
    fn lookup_image_en_devolve_figure() {
        let lang = Lang::from_str("en").unwrap();
        assert_eq!(figure_supplement_for_lang("image", Some(&lang)), "Figure");
    }

    #[test]
    fn lookup_image_fr_devolve_figure() {
        let lang = Lang::from_str("fr").unwrap();
        assert_eq!(figure_supplement_for_lang("image", Some(&lang)), "Figure");
    }
}
