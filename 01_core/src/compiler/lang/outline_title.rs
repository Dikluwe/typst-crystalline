//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/lang.md
//! @prompt-hash 4f6bd2ca
//! @layer L1
//! @updated 2026-08-13
//!
//! **P1034** — título por defeito do `outline()`, localizado por língua.
//!
//! Antes deste passo, `layout/outline.rs` tinha `"Índice"` fixo no código,
//! independente da língua. Duas coisas erradas de uma vez: ignorava o default
//! da linguagem (`en` → "Contents") e, mesmo em português, não era o termo do
//! vanilla.
//!
//! Tabela medida contra o vanilla ratificado (`a51e02804`) em 2026-08-13,
//! `#set text(lang: X)` + `#outline()`, texto extraído do PDF:
//!
//! | lang | vanilla |
//! |------|---------|
//! | en   | Contents |
//! | pt   | Sumário |
//! | de   | Inhaltsverzeichnis |
//! | fr   | Table des matières |
//! | es   | Índice |
//! | it   | Indice |
//! | zh   | 目录 |
//!
//! Mesmo padrão de `figure_supplement_for_lang` (P158B/P1034): lookup exacto,
//! fallback **EN** para língua ausente ou fora da tabela.

use crate::entities::lang::Lang;

/// Títulos de outline por língua — medidos, não inferidos (ver cabeçalho).
const OUTLINE_TITLES: &[(&str, &str)] = &[
    ("en", "Contents"),
    ("pt", "Sumário"),
    ("de", "Inhaltsverzeichnis"),
    ("fr", "Table des matières"),
    ("es", "Índice"),
    ("it", "Indice"),
    ("zh", "目录"),
];

/// Título por defeito do `outline()` para `lang`.
///
/// `None` ou língua fora da tabela → **"Contents"**, porque o default da
/// linguagem Typst é `en` (`Lang::ENGLISH`). Confirmado no vanilla: uma língua
/// sem localização (`lang: "jp"`) cai em inglês, não na língua do ambiente.
pub fn outline_title_for_lang(lang: Option<&Lang>) -> &'static str {
    if let Some(l) = lang {
        let code = l.as_str();
        for (lc, title) in OUTLINE_TITLES.iter() {
            if *lc == code {
                return title;
            }
        }
    }
    "Contents"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn p1034_lang_none_devolve_contents() {
        assert_eq!(outline_title_for_lang(None), "Contents");
    }

    #[test]
    fn p1034_lang_desconhecida_devolve_contents() {
        let lang = Lang::from_str("jp").unwrap();
        assert_eq!(outline_title_for_lang(Some(&lang)), "Contents");
    }

    #[test]
    fn p1034_titulos_medidos_por_lingua() {
        for (code, esperado) in
            [("en", "Contents"), ("pt", "Sumário"), ("de", "Inhaltsverzeichnis")]
        {
            let lang = Lang::from_str(code).unwrap();
            assert_eq!(
                outline_title_for_lang(Some(&lang)),
                esperado,
                "título de outline para '{code}'"
            );
        }
    }
}
