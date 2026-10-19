//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/queries.md
//! @prompt-hash a867e6a4
//! @layer L1
//! @updated 2026-10-06

use super::*;

impl Introspector for TagIntrospector {
    fn query_by_kind(&self, kind: ElementKind) -> Vec<Location> {
        self.kind_index.get(&kind).cloned().unwrap_or_default()
    }

    fn query_by_label(&self, label: &Label) -> Option<Location> {
        self.labels.lookup(label)
    }

    fn query_first(&self, kind: ElementKind) -> Option<Location> {
        self.kind_index.get(&kind).and_then(|v| v.first().copied())
    }

    fn query_unique(&self, kind: ElementKind) -> Option<Location> {
        self.kind_index
            .get(&kind)
            .filter(|v| v.len() == 1)
            .and_then(|v| v.first().copied())
    }

    fn position_of(
        &self,
        location: Location,
    ) -> Option<crate::entities::position::Position> {
        // **P205C (F3)**: impl real per ADR-0074. Delega a
        // `SealedPositions::position_of` (sub-store sealed
        // injectado pós-layout via `inject_positions`).
        // Pre-injecção (default empty), devolve `None` —
        // comportamento P204D §C6a preservado para consumers
        // ainda não migrados.
        //
        // Chamada directa ao método (não via Tracked handle):
        // este impl roda dentro do trait method
        // `Introspector::position_of` que já é tracked a nível
        // do trait (P204B). Re-tracking interno seria recursivo
        // e desnecessário.
        self.positions.position_of(location)
    }
    fn html_position_of(
        &self,
        location: Location,
    ) -> Option<crate::entities::position::HtmlPosition> {
        self.html_positions.get(&location).copied()
    }

    fn figure_number_for_label(&self, label: &Label) -> Option<usize> {
        self.figure_label_numbers.get(label).copied()
    }

    fn query_metadata(&self) -> &[Value] {
        self.metadata.query()
    }

    fn formatted_counter(&self, key: &CounterKey) -> Option<String> {
        self.counters.format(key)
    }

    fn state_value(&self, key: &str, location: Location) -> Option<&Value> {
        self.state.value_at(key, location)
    }

    fn state_final_value(&self, key: &str) -> Option<&Value> {
        self.state.final_value(key)
    }

    fn state_display_value(
        &self,
        key: String,
        location: Location,
    ) -> Option<crate::entities::content::Content> {
        self.state_displays.get(&(key, location)).cloned()
    }

    fn counter_display_value(
        &self,
        key: CounterKey,
        location: Location,
    ) -> Option<crate::entities::content::Content> {
        self.counter_displays.get(&(key, location)).cloned()
    }

    fn query(&self, selector: &Selector) -> Vec<Location> {
        let mut result = match selector {
            Selector::NativeElement(function) => self
                .elements
                .iter()
                .filter(|(_, entry)| {
                    crate::compiler::eval::selector_matching::native_element_matches(
                        entry.content(),
                        function,
                    )
                })
                .map(|(location, _)| *location)
                .collect(),
            Selector::Element { function, fields } => {
                let has_label_filter =
                    fields.iter().any(|(name, _)| name.as_str() == "label");
                let ordinary_fields: Vec<_> = fields
                    .iter()
                    .filter(|(name, _)| name.as_str() != "label")
                    .cloned()
                    .collect();
                self.elements.iter().filter(|(location, entry)| {
                    if entry.fields().is_some() || !has_label_filter {
                        return crate::compiler::eval::selector_matching::element_selector_matches(entry, function, fields);
                    }
                    let labels_match = fields.iter().filter(|(name, _)| name.as_str() == "label").all(|(name, expected)| {
                        if let crate::entities::value::Value::Label(label) = expected {
                            self.labels.lookup_all(label).contains(*location)
                        } else {
                            crate::compiler::eval::selector_matching::element_selector_matches(entry, function, &[(name.clone(), expected.clone())])
                        }
                    });
                    labels_match && crate::compiler::eval::selector_matching::element_selector_matches(entry, function, &ordinary_fields)
                }).map(|(location, _)| *location).collect()
            }
            Selector::Kind(kind) => self.query_by_kind(*kind),
            Selector::Label(label) => {
                let mut matches = self.labels.lookup_all(label).to_vec();
                matches.sort_by_key(|location| self.counters.order_key(*location));
                matches.dedup();
                matches
            }
            // P209B (M9c): Location match → singleton trivial.
            Selector::Location(loc) => vec![*loc],
            // P209C (M9c): intersecção N-ária. Vazio → vec![]
            // (Opção A; cristalino single-pass sem "universo").
            Selector::And(sels) => {
                if sels.is_empty() {
                    return Vec::new();
                }
                let mut iter = sels.iter().map(|s| self.query(s));
                let first: Vec<Location> = iter.next().unwrap();
                iter.fold(first, |acc, next| {
                    acc.into_iter().filter(|loc| next.contains(loc)).collect()
                })
            }
            // P209C (M9c): união N-ária dedupliquada preservando
            // ordem de primeira-aparição.
            Selector::Or(sels) => {
                use std::collections::HashSet;
                let mut seen: HashSet<Location> = HashSet::new();
                let mut result: Vec<Location> = Vec::new();
                for s in sels {
                    for loc in self.query(s) {
                        if seen.insert(loc) {
                            result.push(loc);
                        }
                    }
                }
                result
            }
            // P209D (M9c): **stub `vec![]` documentado**. Cristalino
            // single-pass não tem Content text durante query phase.
            // Variant é materializado estructuralmente (ADR-0076 +
            // ADR-0077); semântica de match-by-text fica para
            // sub-passo dedicado quando Content text durante query
            // for acessível (P212+).
            Selector::Regex(_re) => Vec::new(),
            // **P417 (M)**: query por campo é scope-out neste passo.
            // O Introspector indexa `ElementPayload` (ex.: depth de heading),
            // não Content fields genéricos. Show rules consomem `Where`
            // em tempo de realização; query fica para passo dedicado
            // se houver consumer real.
            Selector::Where { .. } => Vec::new(),
            // **P504** — `selector(base).within(ancestor)`: retém matches de
            // `base` cujo parent imediato (ou algum ancestral) matcha
            // `ancestor`.
            Selector::Within { base, ancestor } => {
                let base_matches = self.query(base);
                let ancestor_matches: std::collections::HashSet<Location> =
                    self.query(ancestor).into_iter().collect();
                base_matches
                    .into_iter()
                    .filter(|loc| {
                        let mut current = Some(*loc);
                        while let Some(current_loc) = current {
                            if let Some(parent) = self.parent_locations.get(&current_loc)
                            {
                                if ancestor_matches.contains(parent) {
                                    return true;
                                }
                                current = Some(*parent);
                            } else {
                                break;
                            }
                        }
                        false
                    })
                    .collect()
            }
            Selector::Before { base, end, inclusive } => {
                let base_matches = self.query(base);
                let Some(boundary) = self.query(end).into_iter().next() else {
                    return base_matches;
                };
                base_matches
                    .into_iter()
                    .filter(|location| {
                        self.counters.order_key(*location)
                            < self.counters.order_key(boundary)
                            || (*inclusive && location == &boundary)
                    })
                    .collect()
            }
            Selector::After { base, start, inclusive } => {
                let base_matches = self.query(base);
                let Some(boundary) = self.query(start).into_iter().next() else {
                    return base_matches;
                };
                base_matches
                    .into_iter()
                    .filter(|location| {
                        self.counters.order_key(*location)
                            > self.counters.order_key(boundary)
                            || (*inclusive && location == &boundary)
                    })
                    .collect()
            }
        };
        if crate::compiler::eval::operators::equality::selector_contains_element(selector)
        {
            result.sort_by_key(|location| self.counters.order_key(*location));
            result.dedup();
        }
        result
    }

    fn formatted_counter_at(
        &self,
        key: &CounterKey,
        location: Location,
    ) -> Option<String> {
        let counter = self.counters.value_at(key, location)?;
        if counter.is_empty() {
            None
        } else {
            Some(counter.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."))
        }
    }

    fn element_at(
        &self,
        location: Location,
    ) -> Option<&crate::entities::content::Content> {
        self.elements.get(&location).map(|entry| entry.content())
    }

    fn counter_values_at(
        &self,
        key: &CounterKey,
        location: Location,
    ) -> Option<&[usize]> {
        let counter = self.counters.value_at(key, location)?;
        if counter.is_empty() {
            None
        } else {
            Some(counter)
        }
    }

    fn counter_final_values(&self, key: &CounterKey) -> Option<&[usize]> {
        let counter = self.counters.value(key)?;
        if counter.is_empty() {
            None
        } else {
            Some(counter)
        }
    }

    fn counter_key_for_label(&self, label: &Label) -> Option<&CounterKey> {
        self.label_to_counter_key.get(label)
    }

    fn heading_has_numbering(&self, location: Location) -> Option<bool> {
        self.heading_numbering.get(&location).copied()
    }

    fn equation_has_numbering(&self, location: Location) -> Option<bool> {
        self.equation_numbering.get(&location).copied()
    }

    fn equation_numbering_pattern(&self, location: Location) -> Option<&str> {
        self.equation_numbering_pattern.get(&location).map(|s| s.as_str())
    }

    fn equation_numbering_content(
        &self,
        location: Location,
    ) -> Option<crate::entities::content::Content> {
        self.equation_numbering_contents.get(&location).cloned()
    }

    fn equation_supplement_content(
        &self,
        location: Location,
    ) -> Option<crate::entities::content::Content> {
        self.equation_supplement_contents.get(&location).cloned()
    }

    fn unreferencable_label_kind(&self, label: &Label) -> Option<UnreferencableKind> {
        self.unreferencable_labels.get(label).copied()
    }

    fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry> {
        self.bib_store.entry_for_key(key)
    }

    fn bib_number_for_key(&self, key: &str) -> Option<u32> {
        self.bib_store.number_for_key(key)
    }

    fn citation_number_for_key(&self, key: &str) -> Option<u32> {
        self.bib_store.citation_number_for_key(key)
    }

    fn citation_order(&self) -> &[String] {
        self.bib_store.citation_order()
    }

    fn back_refs_for_key(&self, key: &str) -> Vec<usize> {
        self.bib_store.back_refs_for_key(key)
    }

    fn figures_for_lof(&self) -> &[(usize, String)] {
        &self.figures_for_lof
    }

    fn tables_for_lot(&self) -> &[(usize, String)] {
        &self.tables_for_lot
    }

    fn figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize> {
        let key = CounterKey::Str(format!("figure:{}", kind).into());
        // Counter flat: snapshot é `[N]` com tamanho 1 — `.last()`
        // extrai o número 1-based. Para counters hierárquicos
        // (heading), `.last()` daria o nível mais profundo, mas
        // figure é sempre flat.
        self.counters.value_at_index(&key, idx)?.last().copied()
    }

    fn flat_counter_at(&self, key: &CounterKey, location: Location) -> Option<usize> {
        self.counters.value_at(key, location)?.last().copied()
    }

    fn resolved_label_for(&self, label: &Label) -> Option<&str> {
        self.resolved_labels.get(label)
    }

    fn headings_for_toc(
        &self,
    ) -> &[(Label, Option<String>, crate::entities::content::Content, usize)] {
        &self.headings_for_toc
    }

    fn headings_for_bookmarks(
        &self,
    ) -> &[(Label, Option<String>, crate::entities::content::Content, usize)] {
        &self.headings_for_bookmarks
    }

    fn query_labelled(&self) -> Vec<(Label, Location)> {
        // **P207B (M9c)**: delega a `LabelRegistry::iter()` (ordenado
        // por Label). Clone+copy O(n) preserva ownership do registry;
        // consumers ficam livres para mutar o resultado.
        self.labels
            .iter()
            .map(|(label, location)| (label.clone(), *location))
            .collect()
    }

    fn label_count(&self, label: &Label) -> usize {
        // **P207C (M9c)**: delega a `LabelRegistry::count` (multi-label
        // semântica). Distingue 0 / 1 / N Locations por label.
        self.labels.count(label)
    }

    fn pages(&self, _location: Location) -> Option<std::num::NonZeroUsize> {
        // **P207D (M9c)**: paridade com vanilla — ignora location e
        // devolve total de páginas. `None` pre-injecção.
        self.page_store.total_pages()
    }

    fn page(&self, location: Location) -> Option<std::num::NonZeroUsize> {
        // **P207D (M9c)**: delega a `SealedPositions` (sub-store
        // P205B). `page_store` não é necessário aqui.
        self.positions.position_of(location).map(|p| p.page)
    }

    fn page_numbering(
        &self,
        location: Location,
    ) -> Option<&crate::entities::numbering::Numbering> {
        // **P207D (M9c)**: combina `page(location)` com
        // `PageStore::numbering_for_page`. Auto-bypass do trait
        // method `page` para evitar recursão tracked.
        let page = self.positions.position_of(location)?.page;
        self.page_store.numbering_for_page(page)
    }

    fn page_supplement(
        &self,
        location: Location,
    ) -> Option<&crate::entities::content::Content> {
        // **P207D (M9c)**: combina `page(location)` com
        // `PageStore::supplement_for_page`. Auto-bypass idêntico ao
        // `page_numbering`.
        let page = self.positions.position_of(location)?.page;
        self.page_store.supplement_for_page(page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_bare_query_matches_registered_identity_without_filter_group() {
        use crate::entities::{content::Content, func::Func, value::IntrospectedContent};
        let mut intr = TagIntrospector::empty();
        let location = Locator::new().next();
        let body = Content::equation(Content::text("a"), true);
        intr.elements
            .insert(location, IntrospectedContent::new(body, Some(Default::default())));
        let function =
            Func::native("alias", crate::compiler::stdlib::native_math_equation);
        assert_eq!(intr.query(&Selector::NativeElement(function)), vec![location]);
    }

    #[test]
    fn kind_queries_keep_causal_order_and_unique_gate() {
        let mut locator = Locator::new();
        let first = locator.next();
        let second = locator.next();
        let mut intr = TagIntrospector::empty();
        intr.kind_index.insert(ElementKind::Heading, vec![first, second]);
        intr.kind_index.insert(ElementKind::Figure, vec![second]);

        assert_eq!(intr.query_by_kind(ElementKind::Heading), vec![first, second]);
        assert_eq!(intr.query_first(ElementKind::Heading), Some(first));
        assert_eq!(intr.query_unique(ElementKind::Heading), None);
        assert_eq!(intr.query_unique(ElementKind::Figure), Some(second));
    }
}

#[cfg(test)]
mod equation_where_label_tests {
    use super::*;
    #[test]
    fn equation_where_label_uses_index_only_for_legacy_none_snapshot() {
        use crate::entities::{
            content::Content,
            value::{IntrospectedContent, Value},
        };
        let Value::Module(math) = crate::compiler::stdlib::make_math_module() else {
            panic!("math module")
        };
        let Some(Value::Func(eq)) = math.scope().get("equation") else {
            panic!("equation")
        };
        let mut intr = TagIntrospector::empty();
        let location = Locator::new().next();
        let label = Label("display".into());
        intr.labels.add(label.clone(), location);
        let body = Content::equation(Content::text("a"), true);
        intr.elements
            .insert(location, IntrospectedContent::new(body.clone(), None));
        let selector = Selector::Element {
            function: eq.clone(),
            fields: vec![
                ("label".into(), Value::Label(label)),
                ("block".into(), Value::Bool(true)),
            ]
            .into(),
        };
        assert_eq!(intr.query(&selector), vec![location]);
        intr.elements
            .insert(location, IntrospectedContent::new(body, Some(Default::default())));
        assert!(intr.query(&selector).is_empty(), "Some fields remain authoritative");
    }
}

#[cfg(test)]
mod circle_origin_label_tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::value::IntrospectedContent;

    #[test]
    fn circle_origin_label_selector_returns_all_distinct_matching_occurrences() {
        let mut locator = Locator::new();
        let first = locator.next();
        let other = locator.next();
        let second = locator.next();
        let matching = Label("matching".into());
        let unrelated = Label("other".into());
        let mut intr = TagIntrospector::empty();
        let equal_content = Content::heading(1, Content::text("same"));
        for location in [first, other, second] {
            intr.counters.observe_location(location);
            intr.elements
                .insert(location, IntrospectedContent::new(equal_content.clone(), None));
        }
        intr.labels.add(matching.clone(), first);
        intr.labels.add(unrelated.clone(), other);
        intr.labels.add(matching.clone(), second);
        assert_eq!(intr.query(&Selector::Label(matching)), vec![first, second]);
        assert_eq!(intr.query(&Selector::Label(unrelated)), vec![other]);
        assert!(intr.query(&Selector::Label(Label("unknown".into()))).is_empty());
    }

    #[test]
    fn circle_origin_label_selector_preserves_causal_order_and_deduplicates_aliases() {
        let mut locator = Locator::new();
        let last = locator.next();
        let first = locator.next();
        let middle = locator.next();
        let label = Label("shared".into());
        let mut intr = TagIntrospector::empty();
        for location in [first, middle, last] {
            intr.counters.observe_location(location);
        }
        // Registry aliases retain one identity. Numeric IDs and registration
        // order are deliberately different from recorded realization order.
        for location in [last, first, last, middle, first] {
            intr.labels.add(label.clone(), location);
        }
        assert_eq!(intr.query(&Selector::Label(label)), vec![first, middle, last]);
    }

    #[test]
    fn circle_origin_label_single_lookup_keeps_first_registered_location() {
        let mut locator = Locator::new();
        let registered_first = locator.next();
        let causal_first = locator.next();
        let label = Label("shared".into());
        let mut intr = TagIntrospector::empty();
        intr.counters.observe_location(causal_first);
        intr.counters.observe_location(registered_first);
        intr.labels.add(label.clone(), registered_first);
        intr.labels.add(label.clone(), causal_first);
        intr.labels.add(label.clone(), registered_first);
        assert_eq!(intr.query_by_label(&label), Some(registered_first));
        assert_eq!(intr.label_count(&label), 3, "raw registry API preserves aliases");
        assert_eq!(intr.query_by_label(&Label("unknown".into())), None);
    }
}
