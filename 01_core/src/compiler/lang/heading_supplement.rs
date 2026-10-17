//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/lang/heading_supplement.md
//! @prompt-hash 662c8ecf
//! @layer L1
//! @updated 2026-09-18

use crate::entities::lang::Lang;

/// Entradas base não vazias de `heading` no pin vanilla ratificado
/// `586e1bd43`. Regiões não são representáveis pelo `Lang` cristalino atual.
const NAMES: &[(&str, &str)] = &[
    ("ar", "الفصل"),
    ("bg", "Раздел"),
    ("ca", "Secció"),
    ("cs", "Kapitola"),
    ("cy", "Adran"),
    ("da", "Afsnit"),
    ("de", "Abschnitt"),
    ("el", "Κεφάλαιο"),
    ("en", "Section"),
    ("eo", "Sekcio"),
    ("es", "Sección"),
    ("et", "Peatükk"),
    ("eu", "Atala"),
    ("fi", "Osio"),
    ("fr", "Section"),
    ("ga", "Ceannteideal"),
    ("gl", "Sección"),
    ("he", "חלק"),
    ("hr", "Odjeljak"),
    ("hu", "Fejezet"),
    ("id", "Bagian"),
    ("is", "Kafli"),
    ("it", "Sezione"),
    ("ja", "節"),
    ("la", "Caput"),
    ("lt", "Skyrius"),
    ("lv", "Sadaļa"),
    ("nb", "Kapittel"),
    ("nl", "Hoofdstuk"),
    ("nn", "Kapittel"),
    ("no", "Avsnitt"),
    ("pl", "Rozdział"),
    ("pt", "Seção"),
    ("ro", "Secțiunea"),
    ("ru", "Раздел"),
    ("sk", "Kapitola"),
    ("sl", "Poglavje"),
    ("sq", "Kapitull"),
    ("sr", "Поглавље"),
    ("sv", "Avsnitt"),
    ("tl", "Seksyon"),
    ("tr", "Bölüm"),
    ("uk", "Розділ"),
    ("vi", "Phần"),
    ("zh", "小节"),
];

/// Resolve o suplemento automático de heading por igualdade exata de língua.
/// Língua ausente ou sem entrada usa o default inglês da linguagem.
pub fn heading_supplement_for_lang(lang: Option<&Lang>) -> &'static str {
    let Some(lang) = lang else { return "Section" };
    NAMES
        .iter()
        .find_map(|(code, name)| (lang.as_str() == *code).then_some(*name))
        .unwrap_or("Section")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn p1459_todas_as_entradas_ratificadas_e_fallback_ingles() {
        const EXPECTED: &[(&str, &str)] = &[
            ("ar", "الفصل"),
            ("bg", "Раздел"),
            ("ca", "Secció"),
            ("cs", "Kapitola"),
            ("cy", "Adran"),
            ("da", "Afsnit"),
            ("de", "Abschnitt"),
            ("el", "Κεφάλαιο"),
            ("en", "Section"),
            ("eo", "Sekcio"),
            ("es", "Sección"),
            ("et", "Peatükk"),
            ("eu", "Atala"),
            ("fi", "Osio"),
            ("fr", "Section"),
            ("ga", "Ceannteideal"),
            ("gl", "Sección"),
            ("he", "חלק"),
            ("hr", "Odjeljak"),
            ("hu", "Fejezet"),
            ("id", "Bagian"),
            ("is", "Kafli"),
            ("it", "Sezione"),
            ("ja", "節"),
            ("la", "Caput"),
            ("lt", "Skyrius"),
            ("lv", "Sadaļa"),
            ("nb", "Kapittel"),
            ("nl", "Hoofdstuk"),
            ("nn", "Kapittel"),
            ("no", "Avsnitt"),
            ("pl", "Rozdział"),
            ("pt", "Seção"),
            ("ro", "Secțiunea"),
            ("ru", "Раздел"),
            ("sk", "Kapitola"),
            ("sl", "Poglavje"),
            ("sq", "Kapitull"),
            ("sr", "Поглавље"),
            ("sv", "Avsnitt"),
            ("tl", "Seksyon"),
            ("tr", "Bölüm"),
            ("uk", "Розділ"),
            ("vi", "Phần"),
            ("zh", "小节"),
        ];

        assert_eq!(NAMES, EXPECTED, "owner divergiu do oráculo ratificado");
        for (code, expected) in EXPECTED {
            let lang = Lang::from_str(code).unwrap();
            assert_eq!(
                heading_supplement_for_lang(Some(&lang)),
                *expected,
                "lang={code}",
            );
        }

        let without_heading_translation = Lang::from_str("af").unwrap();
        assert_eq!(heading_supplement_for_lang(None), "Section");
        assert_eq!(
            heading_supplement_for_lang(Some(&without_heading_translation)),
            "Section",
        );
    }
}
