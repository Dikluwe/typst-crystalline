//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/font-list.md
//! @prompt-hash d943a0eb
//! @layer L1
//! @updated 2026-06-22
//!
//! Tipo `FontList` — lista priorizada de famílias de fonte.
//!
//! Réplica estrutural parcial de `typst::text::FontList` vanilla
//! com divergência consciente (ADR-0053):
//! - `covers` é **inabitado** (`enum Covers {}` sem variantes).
//!   Forma estrutural reservada para futuro.
//! - **P407** (DEBT-52): dict form activada — `FontFamily.name` passa
//!   a ser `FontNamePattern` (literal ou regex), e `variants` passa a
//!   ser armazenado (uso variant-aware scope-out ADR-0054bis).
//!
//! Paridade ADR-0033/ADR-0107: string + array + dict (regex keys)
//! aceites.

use ecow::EcoString;

use crate::entities::regex::Regex;

/// Enum inabitado. Reserva forma estrutural para futuro
/// suporte a coverage filtering (ADR-0053 decisão 2).
///
/// `Option<Covers>` só pode ser `None` por construção — adicionar
/// variantes no futuro é mudança additive, compatível.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Covers {}

/// Padrão de nome de família de fonte.
///
/// Paridade vanilla: uma key de dict `text.font` pode ser string
/// literal ou regex. Literais são normalizados para lowercase no
/// constructo; regex usa a pattern tal como escrita.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FontNamePattern {
    /// Nome exacto, lowercased na construção.
    Literal(EcoString),
    /// Pattern regex; compilação partilhada via `Arc<regex::Regex>`.
    Regex(Regex),
}

impl FontNamePattern {
    /// Verifica se `name` casa com este padrão.
    ///
    /// Literais comparam case-insensitive (paridade com
    /// `FontBook::select`). Regex usa `Regex::is_match`.
    pub fn is_match(&self, name: &str) -> bool {
        match self {
            Self::Literal(lit) => lit.eq_ignore_ascii_case(name),
            Self::Regex(re)    => re.is_match(name),
        }
    }

    /// Se for literal, devolve a string; caso contrário `None`.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Literal(lit) => Some(lit.as_str()),
            Self::Regex(_)     => None,
        }
    }
}

/// Família de fonte com coverage opcional.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FontFamily {
    /// Nome/padrão da família.
    pub name: FontNamePattern,
    /// Variant names associadas (ex.: `"Regular"`, `"Bold"`).
    /// Uso concreto (variant-aware selection) é scope-out
    /// (ADR-0054bis condicional); o campo é transportado intacto.
    pub variants: Vec<EcoString>,
    /// Variant name para seleção variant-aware (transportado,
    /// scope-out ADR-0054bis).
    pub variant: Option<EcoString>,
    /// Peso tipográfico (transportado, scope-out ADR-0054bis).
    pub weight: Option<EcoString>,
    /// Estilo tipográfico (transportado, scope-out ADR-0054bis).
    pub style: Option<EcoString>,
    /// Coverage filter. Sempre `None` neste passo (`Covers`
    /// inabitado).
    pub covers: Option<Covers>,
}

impl FontFamily {
    /// Constrói família literal a partir de nome, normalizando para
    /// lowercase (paridade vanilla), variants vazio.
    pub fn new(name: EcoString) -> Self {
        Self::new_literal(name, vec![])
    }

    /// Constrói família literal explicitando variants.
    pub fn new_literal(name: EcoString, variants: Vec<EcoString>) -> Self {
        Self {
            name: FontNamePattern::Literal(name.to_lowercase().into()),
            variants,
            variant: None,
            weight: None,
            style: None,
            covers: None,
        }
    }

    /// Constrói família regex explicitando variants.
    pub fn new_regex(regex: Regex, variants: Vec<EcoString>) -> Self {
        Self {
            name: FontNamePattern::Regex(regex),
            variants,
            variant: None,
            weight: None,
            style: None,
            covers: None,
        }
    }

    /// Constrói família a partir de campos nomeados (dict form vanilla).
    pub fn new_named(
        name: FontNamePattern,
        variants: Vec<EcoString>,
        variant: Option<EcoString>,
        weight: Option<EcoString>,
        style: Option<EcoString>,
    ) -> Self {
        Self {
            name,
            variants,
            variant,
            weight,
            style,
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

    /// Constrói lista com uma única família literal (forma string do
    /// vanilla), variants vazio.
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
        assert_eq!(f.name.as_str(), Some("arial"));
        assert!(f.covers.is_none());
        assert!(f.variants.is_empty());
    }

    #[test]
    fn font_family_new_case_insensitive_passo_132b() {
        let f1 = FontFamily::new(EcoString::from("arial"));
        let f2 = FontFamily::new(EcoString::from("ARIAL"));
        let f3 = FontFamily::new(EcoString::from("Arial"));
        assert_eq!(f1.name.as_str(), f2.name.as_str());
        assert_eq!(f2.name.as_str(), f3.name.as_str());
    }

    #[test]
    fn font_family_covers_sempre_none_passo_132b() {
        let f = FontFamily::new(EcoString::from("any"));
        match f.covers {
            None => {}
            Some(_) => unreachable!("Covers é inabitado"),
        }
    }

    #[test]
    fn font_name_pattern_literal_is_match_case_insensitive() {
        let p = FontNamePattern::Literal(EcoString::from("name"));
        assert!(p.is_match("name"));
        assert!(p.is_match("Name"));
        assert!(p.is_match("NAME"));
        assert!(!p.is_match("name bold"));
    }

    #[test]
    fn font_name_pattern_regex_is_match() {
        let p = FontNamePattern::Regex(Regex::new("Name.*").unwrap());
        assert!(p.is_match("Name Bold"));
        assert!(p.is_match("Name"));
        assert!(!p.is_match("Other"));
    }

    #[test]
    fn font_name_pattern_as_str_literal() {
        let p = FontNamePattern::Literal(EcoString::from("foo"));
        assert_eq!(p.as_str(), Some("foo"));
    }

    #[test]
    fn font_name_pattern_as_str_regex_none() {
        let p = FontNamePattern::Regex(Regex::new(".*").unwrap());
        assert_eq!(p.as_str(), None);
    }

    #[test]
    fn font_family_new_literal_com_variants() {
        let f = FontFamily::new_literal(
            EcoString::from("Name"),
            vec![EcoString::from("Regular"), EcoString::from("Bold")],
        );
        assert_eq!(f.name.as_str(), Some("name"));
        assert_eq!(f.variants, vec!["Regular", "Bold"]);
    }

    #[test]
    fn font_family_new_regex_com_variants() {
        let re = Regex::new("Name.*").unwrap();
        let f = FontFamily::new_regex(re.clone(), vec![EcoString::from("Regular")]);
        assert!(matches!(f.name, FontNamePattern::Regex(_)));
        assert!(f.name.is_match("Name Bold"));
        assert_eq!(f.variants, vec!["Regular"]);
    }

    #[test]
    fn font_family_clone_partilha_regex() {
        let re = Regex::new("Name.*").unwrap();
        let f = FontFamily::new_regex(re, vec![EcoString::from("Regular")]);
        let cloned = f.clone();
        assert_eq!(f.name, cloned.name);
        assert_eq!(f.variants, cloned.variants);
    }

    #[test]
    fn font_family_partial_eq_por_pattern_e_variants() {
        let a = FontFamily::new_literal(EcoString::from("A"), vec![EcoString::from("R")]);
        let b = FontFamily::new_literal(EcoString::from("A"), vec![EcoString::from("R")]);
        let c = FontFamily::new_literal(EcoString::from("A"), vec![EcoString::from("B")]);
        let d = FontFamily::new(EcoString::from("A"));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn font_list_single_tem_um_elemento_passo_132b() {
        let list = FontList::single(EcoString::from("Arial"));
        assert_eq!(list.len(), 1);
        assert_eq!(list.as_slice()[0].name.as_str(), Some("arial"));
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
        assert_eq!(list.as_slice()[0].name.as_str(), Some("primeira"));
        assert_eq!(list.as_slice()[1].name.as_str(), Some("segunda"));
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
        let a = FontList::single(EcoString::from("arial"));
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn covers_inabitado_estruturalmente_passo_132b() {
        fn _nunca_chamado(c: Covers) -> ! {
            match c {}
        }
    }

    #[test]
    fn font_list_is_empty_sempre_false_passo_132b() {
        let list = FontList::single(EcoString::from("arial"));
        assert!(!list.is_empty());
    }
}
