//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/selector.md
//! @prompt-hash 0cba412a
//! @layer L1
//! @updated 2026-05-12
//!
//! `Selector` — predicado para `Introspector::query` (P175 / M9
//! sub-passo 5; estendido em P209B + P209C + P209D / M9c Bloco VI).
//!
//! Vanilla equivalente: `foundations/selector.rs` com 10+ variants.
//! Cristalino P175 implementou `Kind(ElementKind)` minimal; P209B
//! estendeu com `Label(Label)` + `Location(Location)`; P209C
//! adicionou compósitos `And(EcoVec<Self>)` + `Or(EcoVec<Self>)`;
//! P209D fecha com `Regex(Regex)` per ADR-0077 PROPOSTO.

use ecow::EcoVec;

use crate::entities::element_kind::ElementKind;
use crate::entities::label::Label;
use crate::entities::location::Location;
use crate::entities::regex::Regex;

/// Predicado para `Introspector::query`. Variants P175 +
/// P209B + P209C + P209D.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    /// Selector de kind — matches todos os elementos de um tipo.
    Kind(ElementKind),
    /// **P209B (M9c)** — Selector de label específica.
    Label(Label),
    /// **P209B (M9c)** — Selector de Location específica (singleton).
    Location(Location),
    /// **P209C (M9c)** — Composição N-ária: TODOS os sub-selectors
    /// devem matchar. Query = intersecção `Vec<Location>`. Vazio:
    /// `vec![]` (Opção A — sem "universo" em cristalino
    /// single-pass).
    And(EcoVec<Selector>),
    /// **P209C (M9c)** — Composição N-ária: AO MENOS UM sub-selector
    /// deve matchar. Query = união dedupliquada preservando ordem
    /// de primeira-aparição. Vazio: `vec![]`.
    Or(EcoVec<Selector>),
    /// **P209D (M9c)** — Selector por pattern regex sobre Content
    /// text. Wrapper L1 `Regex` (ADR-0077) com Hash/Eq via pattern
    /// string. **Query arm é stub `vec![]` documentado**: cristalino
    /// single-pass não tem Content text durante query phase;
    /// semântica funcional fica deferred per P209A A3 (consumer
    /// real emerge quando query-by-text for acessível, P212+).
    Regex(Regex),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn igualdade_estrutural() {
        let a = Selector::Kind(ElementKind::Heading);
        let b = Selector::Kind(ElementKind::Heading);
        assert_eq!(a, b);
    }

    #[test]
    fn kinds_distintos_sao_diferentes() {
        let h = Selector::Kind(ElementKind::Heading);
        let f = Selector::Kind(ElementKind::Figure);
        assert_ne!(h, f);
    }

    #[test]
    fn hash_determinismo() {
        let s = Selector::Kind(ElementKind::Heading);
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        s.hash(&mut h1);
        s.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    // ── P209B (M9c Bloco VI) — variants Label + Location ─────────────

    #[test]
    fn p209b_selector_label_estrutural() {
        let a = Selector::Label(Label("intro".to_string()));
        let b = Selector::Label(Label("intro".to_string()));
        let c = Selector::Label(Label("outro".to_string()));
        assert_eq!(a, b);
        assert_ne!(a, c);
        // Hash determinístico.
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        a.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn p209b_selector_location_estrutural() {
        let a = Selector::Location(Location::from_raw(7));
        let b = Selector::Location(Location::from_raw(7));
        let c = Selector::Location(Location::from_raw(99));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn p209b_variants_distintos_desiguais() {
        // Kind(X) ≠ Label(X-as-string) ≠ Location(...).
        let kind = Selector::Kind(ElementKind::Heading);
        let label = Selector::Label(Label("heading".to_string()));
        let loc = Selector::Location(Location::from_raw(0));
        assert_ne!(kind, label);
        assert_ne!(kind, loc);
        assert_ne!(label, loc);
    }

    // ── P209C (M9c Bloco VI) — variants compósitos And + Or ──────────

    #[test]
    fn p209c_selector_and_estrutural() {
        let inner = EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Label(Label("a".to_string())),
        ]);
        let a = Selector::And(inner.clone());
        let b = Selector::And(inner.clone());
        let c = Selector::And(EcoVec::from(vec![
            Selector::Kind(ElementKind::Figure),
        ]));
        assert_eq!(a, b);
        assert_ne!(a, c);
        // Hash determinístico recursivo.
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        a.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn p209c_selector_or_estrutural() {
        let inner = EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Location(Location::from_raw(7)),
        ]);
        let a = Selector::Or(inner.clone());
        let b = Selector::Or(inner.clone());
        assert_eq!(a, b);
        // And vs Or distintos mesmo com conteúdo idêntico.
        let and_same = Selector::And(inner.clone());
        assert_ne!(a, and_same);
    }

    #[test]
    fn p209c_selector_and_or_vazio_estrutural() {
        // Vazio é representável e Hash determinístico.
        let empty_and: Selector = Selector::And(EcoVec::new());
        let empty_or:  Selector = Selector::Or(EcoVec::new());
        assert_eq!(empty_and, Selector::And(EcoVec::new()));
        assert_eq!(empty_or, Selector::Or(EcoVec::new()));
        assert_ne!(empty_and, empty_or);
    }

    #[test]
    fn p209c_selector_nested_recursivo() {
        // And dentro de Or — composição recursiva funciona.
        let inner_and = Selector::And(EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Label(Label("a".to_string())),
        ]));
        let nested = Selector::Or(EcoVec::from(vec![
            inner_and.clone(),
            Selector::Kind(ElementKind::Figure),
        ]));
        // Igualdade preservada.
        let nested_copy = nested.clone();
        assert_eq!(nested, nested_copy);
    }

    // ── P209D (M9c Bloco VI) — variant Regex ─────────────────────────

    #[test]
    fn p209d_selector_regex_estrutural() {
        let a = Selector::Regex(Regex::new("\\d+").unwrap());
        let b = Selector::Regex(Regex::new("\\d+").unwrap());
        let c = Selector::Regex(Regex::new("\\w+").unwrap());
        assert_eq!(a, b);
        assert_ne!(a, c);
        // Hash determinístico recursivo via Regex::Hash (pattern).
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        a.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn p209d_selector_regex_in_or_composicao() {
        // Composição de Regex com outros variants via Or — confirma
        // recursive Hash continua a funcionar com Regex variant.
        let or = Selector::Or(EcoVec::from(vec![
            Selector::Regex(Regex::new("[a-z]+").unwrap()),
            Selector::Kind(ElementKind::Heading),
        ]));
        let or_copy = or.clone();
        assert_eq!(or, or_copy);
    }
}
