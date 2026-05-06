//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/introspector.md
//! @prompt-hash 918d279b
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
use crate::entities::selector::Selector;
use crate::entities::bib_entry::BibEntry;
use crate::entities::bib_store::BibStore;
use crate::entities::resolved_label_store::ResolvedLabelStore;
use crate::entities::state_registry::StateRegistry;
use crate::entities::value::Value;

/// Interface de consulta sobre elementos indexados pela introspecção.
///
/// M3 minimal: 5 métodos read-only. `position_of` é stub (retorna
/// sempre `None`) — mecanismo de população virá em M5+ ou M9 quando
/// layout integrar.
///
/// **P168 (M5 sub-passo 2)**: adicionado `figure_number_for_label`
/// para suportar primeira migração real (figure-ref em layout_ref).
///
/// **P204B (M8)** — `#[comemo::track]` aplicado per ADR-0073
/// (paridade vanilla literal). Trait fica `Send + Sync`.
#[comemo::track]
pub trait Introspector: Send + Sync {
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

    /// **P204D (M8)** — assinatura migrada de stub `Option<()>`
    /// para `Option<Position>` per ADR-0073 (paridade vanilla
    /// literal: `fn position(&self, loc: Location) -> Option<DocumentPosition>`).
    ///
    /// `TagIntrospector` impl retorna sempre `None` — Position
    /// vive em `Layouter.runtime.positions` (single-pass populated
    /// durante layout, per P203A C5). Consumers que precisem de
    /// Position acedem via Layouter directamente, não via
    /// trait. ADR-0073 §C6a (P204D diagnóstico).
    ///
    /// Future trait impl que **tenha** acesso a Layouter runtime
    /// (ex: PagedIntrospector pós-layout, análogo vanilla)
    /// pode override e retornar `Some(Position)`.
    fn position_of(&self, location: Location) -> Option<crate::entities::position::Position>;

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

    /// **P171 (M9 sub-passo 3)** — valor do state `key` na Location
    /// indicada. Aplica updates ordenados até `location` (inclusive).
    /// Retorna `None` se key não foi inicializada.
    fn state_value(&self, key: &str, location: Location) -> Option<&Value>;

    /// **P171 (M9 sub-passo 3)** — valor final do state `key` (último
    /// update aplicado). Equivalente a `state_value(key, last_loc)`.
    fn state_final_value(&self, key: &str) -> Option<&Value>;

    /// **P175 (M9 sub-passo 5)** — query genérica via `Selector`.
    /// P175 minimal: só `Selector::Kind(kind)`, que delega a
    /// `query_by_kind`. Variants futuros (`Label`, `And`, `Or`,
    /// `Where`) ficam para passos dedicados.
    fn query(&self, selector: &Selector) -> Vec<Location>;

    /// **P177 (M9 sub-passo 7)** — formato hierárquico do counter
    /// na `Location` indicada. `None` se key inexistente ou history
    /// vazia para `loc <= location`.
    fn formatted_counter_at(&self, key: &str, location: Location) -> Option<String>;

    /// **P181F** — entry bibliográfica por chave. Replica
    /// `state.bib_entries.iter().find(|e| e.key == *key)` actual em
    /// `layout/mod.rs:584` (P181G migrará caller). Linear scan sobre
    /// `BibStore::entries`; `None` se key não existe.
    fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry>;

    /// **P181F** — número 1-based associado à chave bibliográfica.
    /// Replica `state.bib_numbers.get(key).copied()` actual em
    /// `layout/mod.rs:590`. Lookup O(1) via `BibStore::numbers`;
    /// `None` se key não existe.
    fn bib_number_for_key(&self, key: &str) -> Option<u32>;

    /// **P182B (M9)** — flag de numeração activa para `key`. Replica
    /// `CounterStateLegacy::is_numbering_active(key)` legacy via
    /// `StateRegistry`: delega a `state.final_value(key)` e devolve
    /// `true` apenas se for `Some(Value::Bool(true))`. Default `false`
    /// (state ausente, `Bool(false)`, ou variant não-Bool).
    /// Convenção de chave: `numbering_active:<feature>` (ex.
    /// `numbering_active:heading`). Resolve lacuna #4 (cf. P182A).
    fn is_numbering_active(&self, key: &str) -> bool;

    /// **P184C** — número 1-based da figure na posição `idx` (0-indexed)
    /// entre as figures do `kind` indicado, em ordem de aparecimento
    /// no walk. Constrói `format!("figure:{}", kind)` e delega a
    /// `CounterRegistry::value_at_index` (chave populada em P184B
    /// arm Figure de `from_tags`). Default kind `"image"` é
    /// responsabilidade do caller (cf. `mod.rs:431`).
    /// `None` se kind ausente do registry ou idx fora de range.
    fn figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>;

    /// **P185B** — variante location-aware de `is_numbering_active`.
    /// Delega a `state.value_at(key, location)` (snapshot por Location,
    /// não snapshot final) e devolve `true` apenas se for
    /// `Some(Value::Bool(true))`. Default `false` (state ausente em
    /// `location`, `Bool(false)`, ou variant não-Bool). Suporta C1
    /// (heading prefix) — consumer migra em P187 após P185C introduzir
    /// `current_location` no Layouter. Cf. ADR-0068.
    fn is_numbering_active_at(&self, key: &str, location: Location) -> bool;

    /// **P185B** — valor 1-based de counter flat na `Location`
    /// indicada. Delega a `counters.value_at(key, location)?.last().copied()`.
    /// `None` se key inexistente em `location` ou history vazia.
    /// Para counters flat (figure, equation), `.last()` é o número
    /// actual; para hierárquicos (heading), retorna o nível mais
    /// profundo — usar `formatted_counter_at` (P177) nesse caso.
    /// Suporta C2 (equation counter) — consumer migra em P188 após
    /// P185C. Cf. ADR-0068.
    fn flat_counter_at(&self, key: &str, location: Location) -> Option<usize>;

    /// **P193B** — texto resolvido para a `Label` indicada. `Some(text)`
    /// se label registada em `ResolvedLabelStore`; `None` caso
    /// contrário. Delega a `resolved_labels.get(label)`.
    ///
    /// **Estado em P193B**: sub-store fica vazio em produção até
    /// P195 adicionar arm de populate em `from_tags`. Walks E2/E4
    /// (P189B) continuam a popular `state.resolved_labels` legacy
    /// directamente; consumer C4 migra em P194 com
    /// substitution-with-fallback (`resolved_label_for(label)
    /// .or_else(|| state.resolved_labels.get(label))`). Vide P193
    /// consolidado §5.
    fn resolved_label_for(&self, label: &Label) -> Option<&str>;

    /// **P200B** (M5 universal completo) — entries de outline (TOC)
    /// emitidas pelo walk arm Heading pós-recursão. Cada entry é
    /// `(auto-label, frozen body materializado, level)`. Sub-store
    /// `intr.headings_for_toc` populated via `from_tags` arm
    /// `ElementPayload::HeadingForToc`. Fecha **E2-residuo**
    /// (lacuna #3) e completa E2 estruturalmente. Consumer
    /// `layout/outline.rs:24` migrado para substitution-with-fallback.
    fn headings_for_toc(&self) -> &[(Label, crate::entities::content::Content, usize)];
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
    /// **P171 (M9 sub-passo 3)** — runtime mutable state.
    /// `from_tags` popula via arms para `ElementPayload::State` e
    /// `ElementPayload::StateUpdate`.
    pub state: StateRegistry,
    /// **P181B** — sub-store para entries bibliográficas + numeração
    /// 1-based. População começa em P181E (`from_tags` arm
    /// `ElementPayload::Bibliography`); até lá permanece vazio.
    /// Consumer migrará em P181G (Layouter cite-arm via
    /// `Introspector::bib_entry_for_key` / `bib_number_for_key`).
    pub bib_store: BibStore,
    /// **P193B** (M5 sequência §9 P189 passo 1) — sub-store para
    /// mapeamento Label → texto resolvido. População começa em
    /// P195 (`from_tags` arm Labelled emitido após walk arm migrar);
    /// até lá permanece vazio em produção. Consumer C4 migra em
    /// P194 (`layout/references.rs::layout_ref`) com
    /// substitution-with-fallback. Suporta cadeia E2-E6 P189B
    /// fechar incrementalmente.
    pub resolved_labels: ResolvedLabelStore,
    /// **P200B** (M5 universal completo) — sub-store dedicado para
    /// entries de outline (TOC). Tuple por entry: `(auto-label,
    /// frozen body materializado, level)`. População via
    /// `from_tags` arm `ElementPayload::HeadingForToc` emitido
    /// pelo walk arm Heading pós-recursão (3ª Tag depois de
    /// Heading + Labelled auto-toc P196B). Fecha **E2-residuo**
    /// (lacuna #3 declarada desde P189B/P196B) e completa
    /// estruturalmente E2 (4ª mutação). Consumer
    /// `layout/outline.rs:24` lê via Introspector path com
    /// fallback legacy (`state.headings_for_toc`). Mutação 4
    /// legacy preservada como write paralelo M5 (Layouter
    /// assignments `mod.rs:1490, 1521` dependem); cleanup
    /// orgânico em M6.
    pub headings_for_toc: Vec<(Label, crate::entities::content::Content, usize)>,
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

    fn position_of(&self, _location: Location) -> Option<crate::entities::position::Position> {
        // **P204D (M8)**: Position vive em `Layouter.runtime.positions`
        // (single-pass populated). `TagIntrospector` é construído
        // pre-layout via `from_tags`, sem acesso ao Layouter runtime.
        // Retorna sempre `None` — consumers acedem Position via
        // Layouter directamente (`layouter.runtime.positions.get(&loc)`).
        // ADR-0073 §C6a; P204D diagnóstico.
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

    fn state_value(&self, key: &str, location: Location) -> Option<&Value> {
        self.state.value_at(key, location)
    }

    fn state_final_value(&self, key: &str) -> Option<&Value> {
        self.state.final_value(key)
    }

    fn query(&self, selector: &Selector) -> Vec<Location> {
        match selector {
            Selector::Kind(kind) => self.query_by_kind(*kind),
        }
    }

    fn formatted_counter_at(&self, key: &str, location: Location) -> Option<String> {
        let counter = self.counters.value_at(key, location)?;
        if counter.is_empty() {
            None
        } else {
            Some(counter.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."))
        }
    }

    fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry> {
        self.bib_store.entry_for_key(key)
    }

    fn bib_number_for_key(&self, key: &str) -> Option<u32> {
        self.bib_store.number_for_key(key)
    }

    fn is_numbering_active(&self, key: &str) -> bool {
        matches!(self.state.final_value(key), Some(Value::Bool(true)))
    }

    fn figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize> {
        let key = format!("figure:{}", kind);
        // Counter flat: snapshot é `[N]` com tamanho 1 — `.last()`
        // extrai o número 1-based. Para counters hierárquicos
        // (heading), `.last()` daria o nível mais profundo, mas
        // figure é sempre flat.
        self.counters.value_at_index(&key, idx)?.last().copied()
    }

    fn is_numbering_active_at(&self, key: &str, location: Location) -> bool {
        matches!(self.state.value_at(key, location), Some(Value::Bool(true)))
    }

    fn flat_counter_at(&self, key: &str, location: Location) -> Option<usize> {
        self.counters.value_at(key, location)?.last().copied()
    }

    fn resolved_label_for(&self, label: &Label) -> Option<&str> {
        self.resolved_labels.get(label)
    }

    fn headings_for_toc(&self) -> &[(Label, crate::entities::content::Content, usize)] {
        &self.headings_for_toc
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

    // ── P204B (M8) — Sentinel tests ──────────────────────────────────────
    //
    // Confirmam que `#[comemo::track]` foi aplicado ao trait
    // `Introspector` per ADR-0073 (paridade vanilla literal). Falham
    // de compilação se o atributo for removido ou se bounds Send+Sync
    // forem perdidos.

    #[test]
    fn p204b_trait_e_send_sync() {
        // Sentinel: confirma que o trait Introspector é Send+Sync
        // (per ADR-0073 / P204B). Falha de compilação se bounds
        // forem removidos do trait declaration.
        fn assert_send_sync<T: Send + Sync + ?Sized>() {}
        assert_send_sync::<dyn Introspector>();
    }

    #[test]
    fn p204b_dyn_trait_implementa_track() {
        // Sentinel: confirma que `dyn Introspector + 'static`
        // implementa `comemo::Track` (gerado pelo macro
        // `#[comemo::track]`). Falha de compilação se atributo
        // for removido.
        fn assert_track<T: comemo::Track + ?Sized>() {}
        assert_track::<dyn Introspector>();
    }

    #[test]
    fn p204b_tagintrospector_pode_ser_tracked_via_dyn() {
        // Sentinel: confirma que TagIntrospector concreto pode ser
        // usado via &dyn Introspector e o handle .track() é
        // produzido via macro-generated impl. `comemo::Track` é
        // implementado para `dyn Introspector + '__comemo_dynamic`,
        // não para o tipo concreto — coerção e .track() devem
        // funcionar.
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let dyn_ref: &dyn Introspector = &intr;
        // .track() produz Tracked<'_, dyn Introspector + '_>.
        let _tracked = dyn_ref.track();
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

    // ── P175 (M9 sub-passo 5) — query via Selector ──────────────────────

    #[test]
    fn query_vazio_devolve_vec_vazio() {
        let i = TagIntrospector::empty();
        let result = i.query(&Selector::Kind(ElementKind::Heading));
        assert_eq!(result, Vec::<Location>::new());
    }

    #[test]
    fn query_kind_devolve_locations_em_ordem() {
        let mut i = TagIntrospector::empty();
        i.kind_index.entry(ElementKind::Heading).or_default().push(loc(7));
        i.kind_index.entry(ElementKind::Heading).or_default().push(loc(13));
        i.kind_index.entry(ElementKind::Heading).or_default().push(loc(20));
        let result = i.query(&Selector::Kind(ElementKind::Heading));
        assert_eq!(result, vec![loc(7), loc(13), loc(20)]);
    }

    #[test]
    fn query_kind_isola_por_kind() {
        let mut i = TagIntrospector::empty();
        i.kind_index.entry(ElementKind::Heading).or_default().push(loc(1));
        i.kind_index.entry(ElementKind::Figure).or_default().push(loc(2));
        i.kind_index.entry(ElementKind::Citation).or_default().push(loc(3));
        assert_eq!(i.query(&Selector::Kind(ElementKind::Heading)), vec![loc(1)]);
        assert_eq!(i.query(&Selector::Kind(ElementKind::Figure)),  vec![loc(2)]);
        assert_eq!(i.query(&Selector::Kind(ElementKind::Citation)), vec![loc(3)]);
        // Outros kinds → vazio.
        assert!(i.query(&Selector::Kind(ElementKind::Metadata)).is_empty());
    }

    // ── P177 (M9 sub-passo 7) — formatted_counter_at ────────────────────

    #[test]
    fn formatted_counter_at_em_introspector_vazio_devolve_none() {
        let i = TagIntrospector::empty();
        assert_eq!(i.formatted_counter_at("heading", loc(10)), None);
    }

    #[test]
    fn formatted_counter_at_devolve_snapshot_correcto() {
        let mut i = TagIntrospector::empty();
        // Simular sequência [1, 2, 1] em headings via apply_hierarchical_at.
        i.counters.apply_hierarchical_at("heading".to_string(), 1, loc(10)); // [1]
        i.counters.apply_hierarchical_at("heading".to_string(), 2, loc(20)); // [1, 1]
        i.counters.apply_hierarchical_at("heading".to_string(), 1, loc(30)); // [2]

        assert_eq!(i.formatted_counter_at("heading", loc(10)).as_deref(), Some("1"));
        assert_eq!(i.formatted_counter_at("heading", loc(20)).as_deref(), Some("1.1"));
        assert_eq!(i.formatted_counter_at("heading", loc(30)).as_deref(), Some("2"));
        // Antes de qualquer update.
        assert_eq!(i.formatted_counter_at("heading", loc(5)), None);
    }

    #[test]
    fn formatted_counter_at_key_inexistente_devolve_none() {
        let mut i = TagIntrospector::empty();
        i.counters.apply_hierarchical_at("heading".to_string(), 1, loc(10));
        assert_eq!(i.formatted_counter_at("inexistente", loc(20)), None);
    }

    // ── P181B — sub-store BibStore field ────────────────────────────────

    #[test]
    fn empty_inicializa_bib_store_vazio() {
        let i = TagIntrospector::empty();
        assert!(i.bib_store.is_empty());
        assert!(i.bib_store.entries().is_empty());
        assert_eq!(i.bib_store.entry_for_key("any"), None);
        assert_eq!(i.bib_store.number_for_key("any"), None);
    }

    // ── P181F — trait métodos bib_entry_for_key + bib_number_for_key ────

    #[test]
    fn bib_entry_for_key_em_introspector_vazio_devolve_none() {
        let i = TagIntrospector::empty();
        assert_eq!(i.bib_entry_for_key("any"), None);
    }

    #[test]
    fn bib_number_for_key_em_introspector_vazio_devolve_none() {
        let i = TagIntrospector::empty();
        assert_eq!(i.bib_number_for_key("any"), None);
    }

    #[test]
    fn bib_methods_resolvem_apos_populacao_directa_do_sub_store() {
        // Popula directamente via sub-store (sem chamar from_tags
        // — esse caminho é coberto em from_tags::tests P181E).
        // Verifica que os trait methods delegam correctamente.
        let mut i = TagIntrospector::empty();
        i.bib_store.add_bibliography(vec![
            crate::entities::bib_entry::BibEntry {
                key:          "intro".to_string(),
                author:       String::new(),
                title:        String::new(),
                year:         0,
                volume:       None,
                pages:        None,
                journal:      None,
                publisher:    None,
                url:          None,
                doi:          None,
                editor:       None,
                series:       None,
                note:         None,
                isbn:         None,
                location:     None,
                organization: None,
            },
        ]);
        i.bib_store.assign_number("intro".to_string(), 1);

        assert!(i.bib_entry_for_key("intro").is_some());
        assert_eq!(i.bib_entry_for_key("intro").unwrap().key, "intro");
        assert_eq!(i.bib_number_for_key("intro"), Some(1));
        assert_eq!(i.bib_entry_for_key("nao_existe"), None);
        assert_eq!(i.bib_number_for_key("nao_existe"), None);
    }

    // ── P182B — trait method is_numbering_active ────────────────────────

    #[test]
    fn is_numbering_active_em_introspector_vazio_devolve_false() {
        let i = TagIntrospector::empty();
        assert!(!i.is_numbering_active("numbering_active:heading"));
        assert!(!i.is_numbering_active("numbering_active:equation"));
        assert!(!i.is_numbering_active("any"));
    }

    #[test]
    fn is_numbering_active_apos_init_bool_true_devolve_true() {
        let mut i = TagIntrospector::empty();
        i.state.init(
            "numbering_active:heading".to_string(),
            Value::Bool(true),
            loc(10),
        );
        assert!(i.is_numbering_active("numbering_active:heading"));
    }

    #[test]
    fn is_numbering_active_keys_distintas_isoladas() {
        let mut i = TagIntrospector::empty();
        i.state.init(
            "numbering_active:heading".to_string(),
            Value::Bool(true),
            loc(10),
        );
        // Apenas heading está activo; equation não foi inicializado.
        assert!(i.is_numbering_active("numbering_active:heading"));
        assert!(!i.is_numbering_active("numbering_active:equation"));
    }

    #[test]
    fn is_numbering_active_bool_false_devolve_false() {
        let mut i = TagIntrospector::empty();
        i.state.init(
            "numbering_active:heading".to_string(),
            Value::Bool(false),
            loc(10),
        );
        assert!(!i.is_numbering_active("numbering_active:heading"));
    }

    #[test]
    fn is_numbering_active_value_nao_bool_devolve_false() {
        let mut i = TagIntrospector::empty();
        // Variant não-Bool: graceful degradation → false.
        i.state.init(
            "numbering_active:heading".to_string(),
            Value::Int(1),
            loc(10),
        );
        assert!(!i.is_numbering_active("numbering_active:heading"));
    }

    // ── P184C — figure_number_at_index ──────────────────────────────

    #[test]
    fn figure_number_at_index_em_introspector_vazio_devolve_none() {
        let i = TagIntrospector::empty();
        assert_eq!(i.figure_number_at_index("image", 0), None);
        assert_eq!(i.figure_number_at_index("table", 0), None);
    }

    #[test]
    fn figure_number_at_index_apos_populate_devolve_some() {
        // Replica directamente o que arm Figure faz em `from_tags`
        // (P184B): apply_at("figure:{kind}", Step, loc).
        let mut i = TagIntrospector::empty();
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(10),
        );
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(20),
        );
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(30),
        );
        assert_eq!(i.figure_number_at_index("image", 0), Some(1));
        assert_eq!(i.figure_number_at_index("image", 1), Some(2));
        assert_eq!(i.figure_number_at_index("image", 2), Some(3));
    }

    #[test]
    fn figure_number_at_index_kinds_distintos_isolados() {
        let mut i = TagIntrospector::empty();
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(10),
        );
        i.counters.apply_at(
            "figure:table".to_string(),
            CounterUpdate::Step,
            loc(20),
        );
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(30),
        );
        // image: 2 figures (idx 0, 1); table: 1 figure (idx 0).
        assert_eq!(i.figure_number_at_index("image", 0), Some(1));
        assert_eq!(i.figure_number_at_index("image", 1), Some(2));
        assert_eq!(i.figure_number_at_index("table", 0), Some(1));
        assert_eq!(i.figure_number_at_index("table", 1), None);
    }

    #[test]
    fn figure_number_at_index_idx_fora_de_range_devolve_none() {
        let mut i = TagIntrospector::empty();
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(10),
        );
        // 1 figure populada; idx 1+ é fora de range.
        assert_eq!(i.figure_number_at_index("image", 0), Some(1));
        assert_eq!(i.figure_number_at_index("image", 1), None);
        assert_eq!(i.figure_number_at_index("image", 100), None);
    }

    #[test]
    fn figure_number_at_index_default_kind_image() {
        // Replica path do arm Figure quando `kind: None`: chave fica
        // "figure:image". Caller (Layouter) resolve `None` → "image"
        // antes de chamar; trait method não vê `Option`.
        let mut i = TagIntrospector::empty();
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(10),
        );
        assert_eq!(i.figure_number_at_index("image", 0), Some(1));
    }

    // ── P185B — is_numbering_active_at + flat_counter_at ────────────

    #[test]
    fn is_numbering_active_at_em_introspector_vazio_devolve_false() {
        let i = TagIntrospector::empty();
        assert!(!i.is_numbering_active_at("numbering_active:heading", loc(0)));
        assert!(!i.is_numbering_active_at("numbering_active:equation", loc(100)));
    }

    #[test]
    fn is_numbering_active_at_apos_init_bool_true_devolve_true_em_loc_posterior() {
        let mut i = TagIntrospector::empty();
        i.state.init(
            "numbering_active:heading".to_string(),
            Value::Bool(true),
            loc(10),
        );
        assert!(i.is_numbering_active_at("numbering_active:heading", loc(15)));
        // Em loc(10) (mesma location) também — value_at usa <=.
        assert!(i.is_numbering_active_at("numbering_active:heading", loc(10)));
    }

    #[test]
    fn is_numbering_active_at_re_update_reflecte_location_consultada() {
        // Caso central: valida que value_at retorna snapshot por
        // Location, não snapshot final.
        let mut i = TagIntrospector::empty();
        i.state.init(
            "numbering_active:heading".to_string(),
            Value::Bool(true),
            loc(10),
        );
        i.state.update(
            "numbering_active:heading".to_string(),
            Value::Bool(false),
            loc(20),
        );
        // Antes do update: init activo.
        assert!(i.is_numbering_active_at("numbering_active:heading", loc(15)));
        // Após o update: desactivado.
        assert!(!i.is_numbering_active_at("numbering_active:heading", loc(25)));
        // Diferença face a is_numbering_active (snapshot final): este
        // último daria sempre `false` (último update aplicado).
        assert!(!i.is_numbering_active("numbering_active:heading"));
    }

    #[test]
    fn is_numbering_active_at_bool_false_devolve_false() {
        let mut i = TagIntrospector::empty();
        i.state.init(
            "numbering_active:heading".to_string(),
            Value::Bool(false),
            loc(10),
        );
        assert!(!i.is_numbering_active_at("numbering_active:heading", loc(15)));
    }

    #[test]
    fn is_numbering_active_at_value_nao_bool_devolve_false() {
        let mut i = TagIntrospector::empty();
        // Variant não-Bool: graceful degradation → false.
        i.state.init(
            "numbering_active:heading".to_string(),
            Value::Int(1),
            loc(10),
        );
        assert!(!i.is_numbering_active_at("numbering_active:heading", loc(15)));
    }

    #[test]
    fn flat_counter_at_em_introspector_vazio_devolve_none() {
        let i = TagIntrospector::empty();
        assert_eq!(i.flat_counter_at("figure:image", loc(0)), None);
        assert_eq!(i.flat_counter_at("equation", loc(100)), None);
    }

    #[test]
    fn flat_counter_at_apos_populate_devolve_some_em_loc_posterior() {
        let mut i = TagIntrospector::empty();
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(10),
        );
        assert_eq!(i.flat_counter_at("figure:image", loc(15)), Some(1));
        // Em loc(10) (mesma location) também.
        assert_eq!(i.flat_counter_at("figure:image", loc(10)), Some(1));
    }

    #[test]
    fn flat_counter_at_re_update_reflecte_location_consultada() {
        // Caso central: valida snapshot por Location.
        let mut i = TagIntrospector::empty();
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(10),
        );
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(20),
        );
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(30),
        );
        assert_eq!(i.flat_counter_at("figure:image", loc(15)), Some(1));
        assert_eq!(i.flat_counter_at("figure:image", loc(25)), Some(2));
        assert_eq!(i.flat_counter_at("figure:image", loc(35)), Some(3));
    }

    #[test]
    fn flat_counter_at_keys_distintas_isoladas() {
        let mut i = TagIntrospector::empty();
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(10),
        );
        i.counters.apply_at(
            "figure:table".to_string(),
            CounterUpdate::Step,
            loc(20),
        );
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(30),
        );
        // image: 2 steps em loc(10) e loc(30).
        assert_eq!(i.flat_counter_at("figure:image", loc(15)), Some(1));
        assert_eq!(i.flat_counter_at("figure:image", loc(35)), Some(2));
        // table: 1 step em loc(20); ausente em loc(15).
        assert_eq!(i.flat_counter_at("figure:table", loc(15)), None);
        assert_eq!(i.flat_counter_at("figure:table", loc(25)), Some(1));
    }

    #[test]
    fn flat_counter_at_location_anterior_a_qualquer_apply_devolve_none() {
        let mut i = TagIntrospector::empty();
        i.counters.apply_at(
            "figure:image".to_string(),
            CounterUpdate::Step,
            loc(10),
        );
        // Snapshot vazio para Location anterior à primeira apply_at.
        assert_eq!(i.flat_counter_at("figure:image", loc(5)), None);
    }

    // ── P193B — resolved_label_for ──────────────────────────────────────

    #[test]
    fn resolved_label_for_em_introspector_vazio_devolve_none() {
        let i = TagIntrospector::empty();
        assert_eq!(i.resolved_label_for(&lbl("foo")), None);
    }

    #[test]
    fn resolved_label_for_apos_populate_devolve_some() {
        // Populate manual via field directo (P193B abre infra; arm
        // de populate em from_tags vem em P195).
        let mut i = TagIntrospector::empty();
        i.resolved_labels.insert(lbl("intro"), "Secção 1".to_string());
        i.resolved_labels.insert(lbl("metodos"), "Secção 2".to_string());

        // Trait method delega correctamente.
        assert_eq!(i.resolved_label_for(&lbl("intro")), Some("Secção 1"));
        assert_eq!(i.resolved_label_for(&lbl("metodos")), Some("Secção 2"));
        assert_eq!(i.resolved_label_for(&lbl("ausente")), None);
    }
}
