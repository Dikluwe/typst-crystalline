//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/font-list.md
//! @prompt-hash 583df8dd
//! @layer L1
//! @updated 2026-04-24
//!
//! Tipo `FontList` — lista priorizada de famílias de fonte.
//!
//! Réplica estrutural parcial de `typst::text::FontList` vanilla
//! com divergência consciente (ADR-0053):
//! - `covers` é **inabitado** neste passo (`enum Covers {}` sem
//!   variantes). Forma estrutural reservada para futuro.
//! - Dict form do vanilla é **rejeitada** até `regex` ser
//!   autorizado em L1.
//!
//! Paridade ADR-0033 parcial: string + array aceites; dict
//! rejeitada com mensagem clara.
//!
//! Ver ADR-0053 e diagnóstico
//! `00_nucleo/diagnosticos/diagnostico-font-list-passo-132a.md`.

use ecow::EcoString;

/// Enum inabitado. Reserva forma estrutural para futuro
/// suporte a coverage filtering (ADR-0053 decisão 2).
///
/// `Option<Covers>` só pode ser `None` por construção — adicionar
/// variantes no futuro é mudança additive, compatível.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Covers {}

/// Família de fonte com coverage opcional.
///
/// Name é lowercased na construção (réplica do vanilla).
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FontFamily {
    /// Nome da família, lowercased na construção.
    pub name: EcoString,
    /// Coverage filter. Sempre `None` neste passo (`Covers`
    /// inabitado).
    pub covers: Option<Covers>,
}

impl FontFamily {
    /// Constrói família a partir de nome, normalizando para
    /// lowercase (paridade vanilla).
    pub fn new(name: EcoString) -> Self {
        Self {
            name: name.to_lowercase().into(),
            covers: None,
        }
    }
}

/// Lista priorizada de famílias. Non-empty por construção.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FontList(Vec<FontFamily>);

impl FontList {
    /// Constrói lista a partir de vector non-empty.
    /// Devolve `None` se vazio (réplica semântica de vanilla
    /// `"font fallback list must not be empty"`).
    pub fn new(families: Vec<FontFamily>) -> Option<Self> {
        if families.is_empty() {
            None
        } else {
            Some(Self(families))
        }
    }

    /// Constrói lista com uma única família (forma string do
    /// vanilla).
    pub fn single(name: EcoString) -> Self {
        Self(vec![FontFamily::new(name)])
    }

    /// Slice das famílias (prioridade = ordem).
    pub fn as_slice(&self) -> &[FontFamily] {
        &self.0
    }

    /// Número de famílias.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Se a lista é vazia (nunca acontece por construção — o
    /// construtor rejeita empty).
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_family_new_normaliza_lowercase_passo_132b() {
        let f = FontFamily::new(EcoString::from("Arial"));
        assert_eq!(f.name, "arial");
        assert!(f.covers.is_none());
    }

    #[test]
    fn font_family_new_case_insensitive_passo_132b() {
        let f1 = FontFamily::new(EcoString::from("arial"));
        let f2 = FontFamily::new(EcoString::from("ARIAL"));
        let f3 = FontFamily::new(EcoString::from("Arial"));
        assert_eq!(f1.name, f2.name);
        assert_eq!(f2.name, f3.name);
    }

    #[test]
    fn font_family_covers_sempre_none_passo_132b() {
        // Covers é enum inabitado; só pode ser None.
        let f = FontFamily::new(EcoString::from("any"));
        match f.covers {
            None => {} // OK, único caso possível.
            Some(_) => unreachable!("Covers é inabitado"),
        }
    }

    #[test]
    fn font_list_single_tem_um_elemento_passo_132b() {
        let list = FontList::single(EcoString::from("Arial"));
        assert_eq!(list.len(), 1);
        assert_eq!(list.as_slice()[0].name, "arial");
    }

    #[test]
    fn font_list_new_rejeita_vector_vazio_passo_132b() {
        let result = FontList::new(vec![]);
        assert!(result.is_none());
    }

    #[test]
    fn font_list_new_aceita_um_elemento_passo_132b() {
        let list = FontList::new(vec![
            FontFamily::new(EcoString::from("arial")),
        ]);
        assert!(list.is_some());
        assert_eq!(list.unwrap().len(), 1);
    }

    #[test]
    fn font_list_new_aceita_multiplos_passo_132b() {
        let list = FontList::new(vec![
            FontFamily::new(EcoString::from("inria serif")),
            FontFamily::new(EcoString::from("noto sans")),
            FontFamily::new(EcoString::from("libertinus")),
        ]);
        assert!(list.is_some());
        assert_eq!(list.unwrap().len(), 3);
    }

    #[test]
    fn font_list_preserva_ordem_passo_132b() {
        let list = FontList::new(vec![
            FontFamily::new(EcoString::from("primeira")),
            FontFamily::new(EcoString::from("segunda")),
        ]).unwrap();
        assert_eq!(list.as_slice()[0].name, "primeira");
        assert_eq!(list.as_slice()[1].name, "segunda");
    }

    #[test]
    fn font_list_partial_eq_passo_132b() {
        let a = FontList::single(EcoString::from("arial"));
        let b = FontList::single(EcoString::from("arial"));
        let c = FontList::single(EcoString::from("helvetica"));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn font_list_clone_o1_via_ecow_passo_132b() {
        // EcoString é clone O(1); FontList clone copia Vec e
        // cada FontFamily clona EcoString. Pequeno overhead mas
        // ainda barato. Validação: clone preserva igualdade.
        let a = FontList::single(EcoString::from("arial"));
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn covers_inabitado_estruturalmente_passo_132b() {
        // Validação compile-time: match vazio é exaustivo para
        // enum sem variantes. Se Covers tivesse alguma variante,
        // este código não compilaria.
        fn _nunca_chamado(c: Covers) -> ! {
            match c {}
        }
    }

    #[test]
    fn font_list_is_empty_sempre_false_passo_132b() {
        // Por construção (new rejeita empty), is_empty é sempre
        // false para uma FontList construída.
        let list = FontList::single(EcoString::from("arial"));
        assert!(!list.is_empty());
    }
}
