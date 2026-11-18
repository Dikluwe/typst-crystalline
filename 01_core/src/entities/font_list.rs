//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/font_list.md
//! @prompt-hash 2655bef0
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
use std::sync::Arc;

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
            Self::Regex(re) => re.is_match(name),
        }
    }

    /// Se for literal, devolve a string; caso contrário `None`.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Literal(lit) => Some(lit.as_str()),
            Self::Regex(_) => None,
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
            name: FontNamePattern::Literal(name.to_lowercase()),
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
pub struct FontList(Arc<[FontFamily]>);

impl FontList {
    /// Constrói lista a partir de vector non-empty.
    /// Devolve `None` se vazio (réplica semântica de vanilla
    /// `"font fallback list must not be empty"`).
    pub fn new(families: Vec<FontFamily>) -> Option<Self> {
        if families.is_empty() {
            None
        } else {
            Some(Self(families.into()))
        }
    }

    /// Constrói lista com uma única família literal (forma string do
    /// vanilla), variants vazio.
    pub fn single(name: EcoString) -> Self {
        Self(vec![FontFamily::new(name)].into())
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
        let list = FontList::new(vec![FontFamily::new(EcoString::from("arial"))]);
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
        ])
        .unwrap();
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
    fn font_list_clone_preserva_valores_passo_132b() {
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
    fn shared_fixture() -> FontList {
        FontList::new(vec![
            FontFamily::new_named(
                FontNamePattern::Literal("first".into()),
                vec!["Regular".into(), "Bold".into()],
                Some("SemiBold".into()),
                Some("600".into()),
                Some("italic".into()),
            ),
            FontFamily::new_regex(Regex::new("Fallback.*").unwrap(), vec!["Book".into()]),
            FontFamily::new("last".into()),
        ])
        .unwrap()
    }
    #[test]
    fn shared_font_list_clone_reuses_complete_immutable_payload() {
        let original = shared_fixture();
        let clone = original.clone();
        assert_eq!(original.as_slice(), clone.as_slice());
        assert_eq!(
            original.as_slice().as_ptr(),
            clone.as_slice().as_ptr(),
            "list clone must not rebuild the family vector"
        );
        assert_eq!(
            original.as_slice()[0].variants.as_ptr(),
            clone.as_slice()[0].variants.as_ptr(),
            "nested immutable variants must remain owned once"
        );
        drop(original);
        assert_eq!(clone.as_slice()[0].variant.as_deref(), Some("SemiBold"));
        assert!(clone.as_slice()[1].name.is_match("Fallback Book"));
    }
    #[test]
    fn shared_font_list_style_clone_preserves_shared_owner() {
        use crate::entities::layout_types::{Pt, TextStyle};
        let mut original = TextStyle::regular(Pt(11.0));
        original.font = Some(shared_fixture());
        let mut clone = original.clone();
        assert_eq!(
            original.font.as_ref().unwrap().as_slice().as_ptr(),
            clone.font.as_ref().unwrap().as_slice().as_ptr()
        );
        clone.font = Some(FontList::single("replacement".into()));
        assert_eq!(
            original.font.as_ref().unwrap().as_slice()[0].name.as_str(),
            Some("first")
        );
        assert_eq!(
            clone.font.as_ref().unwrap().as_slice()[0].name.as_str(),
            Some("replacement")
        );
    }
    #[test]
    fn shared_font_list_frame_clone_retains_owner_after_source_drop() {
        use crate::entities::layout_types::{FrameItem, Point, Pt, TextStyle};
        let mut style = TextStyle::regular(Pt(11.0));
        style.font = Some(shared_fixture());
        let ptr = style.font.as_ref().unwrap().as_slice().as_ptr();
        let original = FrameItem::Text {
            pos: Point::ZERO,
            text: "café α".into(),
            style,
            bidi: None,
        };
        let clone = original.clone();
        let FrameItem::Text { style, text, .. } = &clone else {
            panic!("text clone changed variant")
        };
        assert_eq!(style.font.as_ref().unwrap().as_slice().as_ptr(), ptr);
        drop(original);
        assert_eq!(text.as_str(), "café α");
        assert_eq!(
            style.font.as_ref().unwrap().as_slice()[0].weight.as_deref(),
            Some("600")
        );
    }
    #[test]
    fn shared_font_list_distinct_owners_keep_ordered_value_hash_equality() {
        use std::collections::{hash_map::DefaultHasher, HashMap};
        use std::hash::{Hash, Hasher};
        let a = shared_fixture();
        let b = shared_fixture();
        assert_ne!(a.as_slice().as_ptr(), b.as_slice().as_ptr());
        assert_eq!(a, b);
        let hash = |x: &FontList| {
            let mut h = DefaultHasher::new();
            x.hash(&mut h);
            h.finish()
        };
        assert_eq!(hash(&a), hash(&b));
        fn check_map_keys<K: Eq + Hash>(first: K, equal: &K, other: K) {
            let mut map = HashMap::new();
            map.insert(first, 7);
            assert_eq!(map.get(equal), Some(&7));
            map.insert(other, 9);
            assert_eq!(map.len(), 2);
        }
        let mut reversed = b.as_slice().to_vec();
        reversed.reverse();
        let reversed = FontList::new(reversed).unwrap();
        assert_ne!(b, reversed);
        check_map_keys(a, &b, reversed);
    }
    #[test]
    fn shared_font_list_every_named_field_remains_distinguishable() {
        use std::collections::HashSet;
        let base = shared_fixture();
        fn unique_count<K: Eq + std::hash::Hash>(
            values: impl IntoIterator<Item = K>,
        ) -> usize {
            values.into_iter().collect::<HashSet<K>>().len()
        }
        let mut values = vec![base.clone()];
        for n in 0..6 {
            let mut families = base.as_slice().to_vec();
            match n {
                0 => families[0].name = FontNamePattern::Literal("different".into()),
                1 => families[0].variants.reverse(),
                2 => families[0].variant = Some("Other".into()),
                3 => families[0].weight = Some("700".into()),
                4 => families[0].style = Some("oblique".into()),
                5 => families[1].name = FontNamePattern::Literal("Fallback.*".into()),
                _ => unreachable!(),
            }
            let changed = FontList::new(families).unwrap();
            assert_ne!(base, changed);
            values.push(changed);
        }
        assert_eq!(unique_count(values), 7);
        assert_eq!(base.as_slice()[0].variants, vec!["Regular", "Bold"]);
    }
    #[test]
    fn shared_font_list_nonempty_construction_and_many_live_clones() {
        fn send_sync<T: Send + Sync>() {}
        send_sync::<FontList>();
        assert!(FontList::new(vec![]).is_none());
        let original = FontList::new(
            (0..128)
                .map(|n| FontFamily::new(format!("Family {n}").into()))
                .collect(),
        )
        .unwrap();
        let ptr = original.as_slice().as_ptr();
        let clones: Vec<_> = (0..64).map(|_| original.clone()).collect();
        assert!(clones.iter().all(|list| list.as_slice().as_ptr() == ptr));
        drop(original);
        assert!(clones.iter().all(|list| list.len() == 128
            && !list.is_empty()
            && list.as_slice()[127].name.as_str() == Some("family 127")));
    }
}
