//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/lang.md
//! @prompt-hash 4f6bd2ca
//! @layer L1
//! @updated 2026-08-24

use crate::entities::lang::Lang;

const NAMES: &[(&str, &str)] = &[
    ("en", "Equation"),
    ("pt", "Equação"),
    ("de", "Gleichung"),
    ("fr", "Équation"),
    ("es", "Ecuación"),
    ("it", "Equazione"),
];

pub fn equation_supplement_for_lang(lang: Option<&Lang>) -> &'static str {
    let Some(lang) = lang else { return "Equation" };
    NAMES
        .iter()
        .find_map(|(code, name)| (lang.as_str() == *code).then_some(*name))
        .unwrap_or("Equation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn p11404c_nomes_medidos_e_fallback_ingles() {
        for (code, expected) in NAMES {
            let lang = Lang::from_str(code).unwrap();
            assert_eq!(equation_supplement_for_lang(Some(&lang)), *expected);
        }
        let unknown = Lang::from_str("nl").unwrap();
        assert_eq!(equation_supplement_for_lang(None), "Equation");
        assert_eq!(equation_supplement_for_lang(Some(&unknown)), "Equation");
    }
}
