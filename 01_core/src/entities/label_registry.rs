//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/label_registry.md
//! @prompt-hash 06720061
//! @layer L1
//! @updated 2026-05-12
//!
//! `LabelRegistry` — sub-store Label→[Location] (multi-label) para
//! `Introspector`. P165 sub-passo .B (M3 Introspection); refactor
//! multi-label em P207C (M9c).
//!
//! Read-only após construção. Mutação só via `pub(crate) fn add`
//! durante construção em `from_tags`.

use std::collections::HashMap;

use crate::entities::label::Label;
use crate::entities::location::Location;

/// Mapeamento `Label → Vec<Location>` (multi-label) construído pelo
/// motor de introspecção.
///
/// **P207C (M9c)**: refactor de `HashMap<Label, Location>` para
/// `HashMap<Label, Vec<Location>>` — cada `add` acumula em vez de
/// ignorar. `lookup` mantém comportamento single-Location (devolve
/// **primeira** inserção) para compatibilidade com call-sites
/// pre-P207C; `lookup_all` e `count` expõem semântica multi-label
/// completa.
#[derive(Debug, Clone, Default)]
pub struct LabelRegistry {
    inner: HashMap<Label, Vec<Location>>,
}

impl LabelRegistry {
    /// Cria registry vazio.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Lookup de label (compatibilidade single-Location). Retorna
    /// `Some(location)` com a **primeira** Location inserida para
    /// `label`; `None` se label nunca foi adicionada.
    pub fn lookup(&self, label: &Label) -> Option<Location> {
        self.inner.get(label).and_then(|v| v.first().copied())
    }

    /// **P207C (M9c)** — Todas as Locations associadas a `label`,
    /// em ordem de inserção. Slice vazio se label desconhecido.
    pub fn lookup_all(&self, label: &Label) -> &[Location] {
        self.inner.get(label).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// **P207C (M9c)** — Número de Locations associadas a `label`.
    /// 0 se label desconhecido. Equivalente a `lookup_all(label).len()`.
    pub fn count(&self, label: &Label) -> usize {
        self.inner.get(label).map(|v| v.len()).unwrap_or(0)
    }

    /// Número de **labels únicas** registadas (chaves do mapa interno).
    /// Não é o total de pares (Label, Location) — para isso usar
    /// `iter().count()`.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// `true` se nenhuma label foi adicionada.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// **P207B (M9c)** — Iterador determinístico sobre todos os pares
    /// `(label, location)`, ordenados alfabéticamente por `Label.0`
    /// (inner `String`).
    ///
    /// **P207C**: em multi-label, emite **um par por Location** —
    /// entradas com a mesma label aparecem agrupadas e consecutivas,
    /// na ordem de inserção dentro do grupo (ordem do `Vec`). Custo
    /// O(n log n) por invocação onde `n` = nº de labels únicas.
    pub fn iter(&self) -> impl Iterator<Item = (&Label, &Location)> + '_ {
        let mut entries: Vec<(&Label, &Vec<Location>)> =
            self.inner.iter().collect();
        entries.sort_by(|(la, _), (lb, _)| la.0.cmp(&lb.0));
        entries
            .into_iter()
            .flat_map(|(label, locs)| locs.iter().map(move |loc| (label, loc)))
    }

    /// Insere par `(label, location)`. Apenas usado pelo construtor
    /// `from_tags` em `rules/introspect/from_tags.rs`. **P207C**:
    /// faz `push` no `Vec` interno — multi-label preserva ordem de
    /// inserção; inserções repetidas de `(label, location)`
    /// idêntico são preservadas como duplicados no Vec.
    pub(crate) fn add(&mut self, label: Label, location: Location) {
        self.inner.entry(label).or_default().push(location);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn label(s: &str) -> Label {
        Label(s.to_string())
    }

    #[test]
    fn empty_lookup_devolve_none() {
        let r = LabelRegistry::empty();
        assert_eq!(r.lookup(&label("foo")), None);
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn add_e_lookup_round_trip() {
        let mut r = LabelRegistry::empty();
        let loc = Location::from_raw(42);
        r.add(label("intro"), loc);
        assert_eq!(r.lookup(&label("intro")), Some(loc));
        assert_eq!(r.len(), 1);
        assert!(!r.is_empty());
    }

    #[test]
    fn cinco_labels_distintos_resolvem_correctamente() {
        let mut r = LabelRegistry::empty();
        let pares: Vec<(&str, u128)> = vec![
            ("a", 1), ("b", 2), ("c", 3), ("d", 4), ("e", 5),
        ];
        for (k, raw) in &pares {
            r.add(label(k), Location::from_raw(*raw));
        }
        for (k, raw) in &pares {
            assert_eq!(r.lookup(&label(k)), Some(Location::from_raw(*raw)));
        }
        assert_eq!(r.len(), 5);
    }

    #[test]
    fn duplicada_preserva_primeira_location() {
        let mut r = LabelRegistry::empty();
        let loc1 = Location::from_raw(7);
        let loc2 = Location::from_raw(99);
        r.add(label("dup"), loc1);
        r.add(label("dup"), loc2);
        assert_eq!(r.lookup(&label("dup")), Some(loc1), "primeira ganha");
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn lookup_de_label_inexistente_devolve_none() {
        let mut r = LabelRegistry::empty();
        r.add(label("real"), Location::from_raw(1));
        assert_eq!(r.lookup(&label("fake")), None);
    }

    // ── P207B (M9c) — iter ordenado por Label ───────────────────────

    #[test]
    fn p207b_iter_em_registry_vazio_devolve_iterador_vazio() {
        let r = LabelRegistry::empty();
        assert_eq!(r.iter().count(), 0);
    }

    #[test]
    fn p207b_iter_ordena_por_label_independente_da_ordem_de_add() {
        // Inserção em ordem arbitrária; iter() devolve alfabética.
        let mut r = LabelRegistry::empty();
        r.add(label("gamma"), Location::from_raw(30));
        r.add(label("alpha"), Location::from_raw(10));
        r.add(label("beta"),  Location::from_raw(20));
        let collected: Vec<(Label, Location)> = r.iter()
            .map(|(l, loc)| (l.clone(), *loc))
            .collect();
        assert_eq!(
            collected,
            vec![
                (label("alpha"), Location::from_raw(10)),
                (label("beta"),  Location::from_raw(20)),
                (label("gamma"), Location::from_raw(30)),
            ]
        );
    }

    // ── P207C (M9c) — Multi-label semântica ────────────────────────

    #[test]
    fn p207c_label_registry_multilabel_lookup_primeira() {
        // Mesmo label adicionado 2× com Locations diferentes.
        // lookup deve devolver a PRIMEIRA inserção (compat).
        let mut r = LabelRegistry::empty();
        r.add(label("intro"), Location::from_raw(7));
        r.add(label("intro"), Location::from_raw(99));
        assert_eq!(r.lookup(&label("intro")), Some(Location::from_raw(7)));
        // len() continua a contar labels únicas (= 1).
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn p207c_label_registry_lookup_all_retorna_todas() {
        let mut r = LabelRegistry::empty();
        r.add(label("intro"), Location::from_raw(7));
        r.add(label("intro"), Location::from_raw(13));
        r.add(label("intro"), Location::from_raw(99));
        // Ordem de inserção preservada.
        assert_eq!(
            r.lookup_all(&label("intro")),
            &[
                Location::from_raw(7),
                Location::from_raw(13),
                Location::from_raw(99),
            ]
        );
        // Label desconhecido: slice vazio.
        assert_eq!(r.lookup_all(&label("ausente")), &[] as &[Location]);
    }

    #[test]
    fn p207c_label_registry_count_zero_para_desconhecido() {
        let r = LabelRegistry::empty();
        assert_eq!(r.count(&label("ausente")), 0);
    }

    #[test]
    fn p207c_label_registry_count_um_para_label_unica() {
        let mut r = LabelRegistry::empty();
        r.add(label("intro"), Location::from_raw(1));
        assert_eq!(r.count(&label("intro")), 1);
    }

    #[test]
    fn p207c_label_registry_count_multiplo_para_label_repetida() {
        let mut r = LabelRegistry::empty();
        r.add(label("intro"), Location::from_raw(1));
        r.add(label("intro"), Location::from_raw(2));
        r.add(label("intro"), Location::from_raw(3));
        assert_eq!(r.count(&label("intro")), 3);
        // Outros labels permanecem em 0.
        assert_eq!(r.count(&label("outro")), 0);
    }

    #[test]
    fn p207c_label_registry_iter_agrupa_multilabel() {
        // Multi-label: iter emite 1 par por Location; entradas com
        // mesma label aparecem consecutivas (ordem inserção dentro
        // do grupo); ordem entre labels é alfabética.
        let mut r = LabelRegistry::empty();
        r.add(label("beta"),  Location::from_raw(2));
        r.add(label("alpha"), Location::from_raw(10));
        r.add(label("alpha"), Location::from_raw(11));
        r.add(label("alpha"), Location::from_raw(12));
        r.add(label("beta"),  Location::from_raw(3));

        let collected: Vec<(Label, Location)> = r.iter()
            .map(|(l, loc)| (l.clone(), *loc))
            .collect();
        assert_eq!(
            collected,
            vec![
                // alpha primeiro (alfabético); 3 Locations em ordem.
                (label("alpha"), Location::from_raw(10)),
                (label("alpha"), Location::from_raw(11)),
                (label("alpha"), Location::from_raw(12)),
                // beta depois; 2 Locations em ordem de inserção.
                (label("beta"),  Location::from_raw(2)),
                (label("beta"),  Location::from_raw(3)),
            ]
        );
        // Total de pares (5) ≠ labels únicas (2).
        assert_eq!(r.len(), 2);
        assert_eq!(r.iter().count(), 5);
    }
}
