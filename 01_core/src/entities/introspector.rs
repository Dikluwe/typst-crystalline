//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/introspector.md
//! @prompt-hash 3192b0fe
//! @layer L1
//! @updated 2026-04-30
//!
//! `Introspector` trait + `TagIntrospector` impl concreta.
//! P165 sub-passo .D (M3 Introspection — núcleo do query layer).
//!
//! Plain trait sem `#[comemo::track]` — tracking deferido para M7+
//! quando fixpoint memoization for relevante. Struct concreta lê
//! sub-stores expostos como fields públicos (composição visível).

use std::collections::HashMap;

use crate::entities::counter_registry::CounterRegistry;
use crate::entities::element_kind::ElementKind;
use crate::entities::label::Label;
use crate::entities::label_registry::LabelRegistry;
use crate::entities::location::Location;
use crate::entities::metadata_store::MetadataStore;
use crate::entities::value::Value;

/// Interface de consulta sobre elementos indexados pela introspecção.
///
/// M3 minimal: 5 métodos read-only. `position_of` é stub (retorna
/// sempre `None`) — mecanismo de população virá em M5+ ou M9 quando
/// layout integrar.
///
/// **P168 (M5 sub-passo 2)**: adicionado `figure_number_for_label`
/// para suportar primeira migração real (figure-ref em layout_ref).
pub trait Introspector {
    /// Vector de todas as `Location`s indexadas com este kind, em
    /// ordem de aparecimento no walk.
    fn query_by_kind(&self, kind: ElementKind) -> Vec<Location>;

    /// `Some(location)` se a label existir; `None` caso contrário.
    fn query_by_label(&self, label: &Label) -> Option<Location>;

    /// Primeira `Location` indexada com este kind, ou `None` se
    /// nenhuma existir.
    fn query_first(&self, kind: ElementKind) -> Option<Location>;

    /// `Some(loc)` apenas se houver **exactamente** uma `Location`
    /// indexada com este kind. `None` se 0 ou >1.
    fn query_unique(&self, kind: ElementKind) -> Option<Location>;

    /// M3 stub: retorna sempre `None`. Mapa Location→Position fica
    /// vazio até consumer real (layout) integrar em M5/M9.
    fn position_of(&self, location: Location) -> Option<()>;

    /// P168 (M5): número 1-based da figura associada à label, **apenas
    /// se a figura é numerada+captioned**. Equivalente ao
    /// `state.figure_label_numbers.get(label).copied()` legacy.
    /// Retorna `None` se label não existe, não pertence a uma figura,
    /// ou figura não tem numbering+caption.
    fn figure_number_for_label(&self, label: &Label) -> Option<usize>;

    /// **P169 (M9 sub-passo 1)** — todos os values embebidos via
    /// `metadata(value)` vanilla, na ordem de aparecimento no walk.
    /// Retorna slice vazio se nenhum metadata existir.
    fn query_metadata(&self) -> &[Value];

    /// **P170 (M9 sub-passo 2)** — formato hierárquico do counter
    /// como string ("1.2.3"). Equivalente a
    /// `state.format_hierarchical(key)` legacy. Suporta lacuna #5.
    fn formatted_counter(&self, key: &str) -> Option<String>;
}

/// Implementação concreta de `Introspector` construída a partir de
/// `Vec<Tag>` via `rules/introspect/from_tags::from_tags`.
///
/// Sub-stores são `pub` para composição visível e acesso directo em
/// testes e consumers M4+. Mutação só durante fase de construção
/// via métodos `pub(crate)` dos próprios sub-stores.
#[derive(Debug, Clone, Default)]
pub struct TagIntrospector {
    pub labels:     LabelRegistry,
    pub counters:   CounterRegistry,
    pub kind_index: HashMap<ElementKind, Vec<Location>>,
    // P168 (M5 sub-passo 2): mapa Label → número 1-based para
    // figuras numeradas+captioned. Populado por `from_tags` quando
    // `ElementPayload::Figure.is_counted == true` E há label associada.
    // Equivalente paralelo a `CounterStateLegacy.figure_label_numbers`
    // — usado por `references.rs::layout_ref` em M5.
    pub figure_label_numbers: HashMap<Label, usize>,
    /// **P169 (M9 sub-passo 1)** — values embebidos via `metadata(value)`
    /// vanilla. Acumulado por `from_tags` em ordem de aparecimento.
    pub metadata: MetadataStore,
    // positions: HashMap<Location, Position> — adiado para M5/M9.
}

impl TagIntrospector {
    /// Construtor vazio. Equivalente a `Default::default()`.
    pub fn empty() -> Self {
        Self::default()
    }
}

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

    fn position_of(&self, _location: Location) -> Option<()> {
        // M3: stub. Consumer real virá em M5/M9.
        None
    }

    fn figure_number_for_label(&self, label: &Label) -> Option<usize> {
        self.figure_label_numbers.get(label).copied()
    }

    fn query_metadata(&self) -> &[Value] {
        self.metadata.query()
    }

    fn formatted_counter(&self, key: &str) -> Option<String> {
        self.counters.format(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::counter_update::CounterUpdate;

    fn loc(raw: u128) -> Location {
        Location::from_raw(raw)
    }

    fn lbl(s: &str) -> Label {
        Label(s.to_string())
    }

    #[test]
    fn empty_devolve_vazio_em_todos_os_queries() {
        let i = TagIntrospector::empty();
        assert_eq!(i.query_by_kind(ElementKind::Heading), Vec::<Location>::new());
        assert_eq!(i.query_by_label(&lbl("foo")), None);
        assert_eq!(i.query_first(ElementKind::Heading), None);
        assert_eq!(i.query_unique(ElementKind::Heading), None);
        assert_eq!(i.position_of(loc(1)), None);
    }

    #[test]
    fn populado_responde_correctamente() {
        let mut i = TagIntrospector::empty();
        i.labels.add(lbl("intro"), loc(7));
        i.counters.apply("heading".to_string(), CounterUpdate::Step);
        i.kind_index.entry(ElementKind::Heading).or_default().push(loc(7));
        i.kind_index.entry(ElementKind::Heading).or_default().push(loc(13));

        // 2 headings → query_by_kind retorna 2 em ordem.
        assert_eq!(
            i.query_by_kind(ElementKind::Heading),
            vec![loc(7), loc(13)]
        );
        // query_first → primeira.
        assert_eq!(i.query_first(ElementKind::Heading), Some(loc(7)));
        // query_unique → None porque há 2.
        assert_eq!(i.query_unique(ElementKind::Heading), None);
        // query_by_label.
        assert_eq!(i.query_by_label(&lbl("intro")), Some(loc(7)));
        // position_of stub.
        assert_eq!(i.position_of(loc(7)), None);
    }

    #[test]
    fn unique_devolve_some_quando_so_existe_um() {
        let mut i = TagIntrospector::empty();
        i.kind_index.entry(ElementKind::Figure).or_default().push(loc(99));
        assert_eq!(i.query_unique(ElementKind::Figure), Some(loc(99)));
        assert_eq!(i.query_first(ElementKind::Figure), Some(loc(99)));
    }

    #[test]
    fn kinds_distintos_isolados() {
        let mut i = TagIntrospector::empty();
        i.kind_index.entry(ElementKind::Heading).or_default().push(loc(1));
        i.kind_index.entry(ElementKind::Citation).or_default().push(loc(2));

        assert_eq!(i.query_by_kind(ElementKind::Heading), vec![loc(1)]);
        assert_eq!(i.query_by_kind(ElementKind::Citation), vec![loc(2)]);
        assert_eq!(i.query_by_kind(ElementKind::Figure), Vec::<Location>::new());
    }
}
