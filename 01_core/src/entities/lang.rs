//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/lang.md
//! @prompt-hash 8b797386
//! @layer L1
//! @updated 2026-04-24
//!
//! Tipo `Lang` — código ISO 639-1/2/3 para language tag.
//!
//! Réplica estrutural de `typst::text::lang::Lang` vanilla.
//! Forma interna: `[u8; 3]` + length (2 ou 3). Copy, zero
//! alocação. Paridade ADR-0033 com erro hard em inputs
//! inválidos (ISO 639-1/2/3 format: 2 ou 3 letras ASCII).
//!
//! Ver ADR-0052 e diagnóstico
//! `00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md`.

use std::str::FromStr;

/// Identificador de língua natural (código ISO 639-1/2/3).
///
/// Forma: 3 bytes ASCII + length discriminator (2 ou 3).
/// Valores 2-letter são padded com `b' '`. Sempre lowercase.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Lang([u8; 3], u8);

impl Lang {
    /// Inglês (`en`). Único constant na materialização inicial
    /// (Passo 131B); outras línguas adicionam-se on-demand
    /// quando consumer as exigir.
    pub const ENGLISH: Self = Self(*b"en ", 2);

    /// Devolve o código ISO como slice ASCII (sem padding).
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0[..self.1 as usize])
            .expect("Lang guarda apenas bytes ASCII válidos")
    }
}

impl FromStr for Lang {
    type Err = &'static str;

    /// Parser fiel ao vanilla: aceita 2 ou 3 letras ASCII,
    /// normaliza para lowercase. Rejeita tudo o resto com
    /// mensagem literal do vanilla.
    fn from_str(iso: &str) -> Result<Self, Self::Err> {
        let len = iso.len();
        if matches!(len, 2..=3) && iso.is_ascii() {
            let mut bytes = [b' '; 3];
            bytes[..len].copy_from_slice(iso.as_bytes());
            bytes.make_ascii_lowercase();
            Ok(Self(bytes, len as u8))
        } else {
            Err("expected two or three letter language code (ISO 639-1/2/3)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lang_from_str_iso_639_1_aceita_2_letras_passo_131b() {
        assert!(Lang::from_str("pt").is_ok());
        assert!(Lang::from_str("en").is_ok());
        assert!(Lang::from_str("de").is_ok());
    }

    #[test]
    fn lang_from_str_iso_639_3_aceita_3_letras_passo_131b() {
        assert!(Lang::from_str("por").is_ok());
        assert!(Lang::from_str("fil").is_ok());
    }

    #[test]
    fn lang_from_str_normaliza_case_passo_131b() {
        assert_eq!(Lang::from_str("PT").unwrap().as_str(), "pt");
        assert_eq!(Lang::from_str("En").unwrap().as_str(), "en");
    }

    #[test]
    fn lang_from_str_vazio_devolve_erro_passo_131b() {
        let err = Lang::from_str("").unwrap_err();
        assert!(err.contains("two or three letter"));
    }

    #[test]
    fn lang_from_str_1_letra_devolve_erro_passo_131b() {
        assert!(Lang::from_str("e").is_err());
    }

    #[test]
    fn lang_from_str_4_letras_devolve_erro_passo_131b() {
        assert!(Lang::from_str("engl").is_err());
    }

    #[test]
    fn lang_from_str_nao_ascii_devolve_erro_passo_131b() {
        assert!(Lang::from_str("日本").is_err());
    }

    #[test]
    fn lang_from_str_com_hyphen_devolve_erro_passo_131b() {
        // "en-GB" tem hyphen; FromStr vanilla aceita apenas
        // letters + length 2-3. Length 5 rejeita.
        assert!(Lang::from_str("en-GB").is_err());
    }

    #[test]
    fn lang_as_str_preserva_canonico_passo_131b() {
        assert_eq!(Lang::ENGLISH.as_str(), "en");
    }

    #[test]
    fn lang_as_str_trim_padding_3_letter_passo_131b() {
        let fil = Lang::from_str("fil").unwrap();
        assert_eq!(fil.as_str(), "fil");
        // Sem espaço trailing — length=3 não padded.
    }

    #[test]
    fn lang_english_constante_passo_131b() {
        assert_eq!(Lang::ENGLISH.as_str(), "en");
        // Copy: pode ser usado sem clone.
        let copia = Lang::ENGLISH;
        assert_eq!(copia, Lang::ENGLISH);
    }
}
