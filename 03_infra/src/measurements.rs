//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/measurements.md
//! @prompt-hash 0520956b
//! @layer L3
//! @updated 2026-05-12
//!
//! **P204G (M8)** — Measurements internos per ADR-0073:
//! cache stats (`crystalline_evict` calls + last `max_age`)
//! e counts de invocação dos 20 métodos do trait
//! `Introspector` via wrapper newtype `CountingIntrospector`.
//!
//! Forma dual (P204A C10):
//! - Logging opt-in via `CRYSTALLINE_MEASUREMENTS=1` (lido em L4).
//! - Tests dedicados sobre asserts de counts.
//!
//! Caminhos fixados (P204G C2/C3):
//! - C2 = B (counter próprio `AtomicUsize` global; feature
//!   `comemo::testing` rejeitada por desproporção).
//! - C3 = a (wrapper newtype; não invade L1 production).
//!
//! Localização L3: estado global mutável (`AtomicUsize`)
//! proibido em L1 (V13); tipos novos proibidos em L4 (V12,
//! cf. nota literal `04_wiring/src/main.rs:101`). L3 é a
//! camada canónica para infra I/O-adjacente.

use std::sync::atomic::{AtomicUsize, Ordering};

use std::num::NonZeroUsize;

use ecow::EcoString;
use typst_core::entities::bib_entry::BibEntry;
use typst_core::entities::content::Content;
use typst_core::entities::element_kind::ElementKind;
use typst_core::entities::introspector::Introspector;
use typst_core::entities::label::Label;
use typst_core::entities::location::Location;
use typst_core::entities::position::Position;
use typst_core::entities::selector::Selector;
use typst_core::entities::value::Value;

// ── Globais ────────────────────────────────────────────────────────

static EVICT_CALLS: AtomicUsize = AtomicUsize::new(0);
static LAST_MAX_AGE: AtomicUsize = AtomicUsize::new(0);

/// Ordem fixa dos 20 métodos do trait `Introspector`. Índice nesta
/// constante = índice em `CALL_COUNTERS`.
pub const INTROSPECTOR_METHODS: [&str; 26] = [
    "query_by_kind",
    "query_by_label",
    "query_first",
    "query_unique",
    "position_of",
    "figure_number_for_label",
    "query_metadata",
    "formatted_counter",
    "state_value",
    "state_final_value",
    "query",
    "formatted_counter_at",
    "bib_entry_for_key",
    "bib_number_for_key",
    "is_numbering_active",
    "figure_number_at_index",
    "is_numbering_active_at",
    "flat_counter_at",
    "resolved_label_for",
    "headings_for_toc",
    // P207B (M9c)
    "query_labelled",
    // P207C (M9c)
    "label_count",
    // P207D (M9c) — 4 page-aware methods
    "pages",
    "page",
    "page_numbering",
    "page_supplement",
];

static CALL_COUNTERS: [AtomicUsize; 26] = [
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0),
];

// ── API pública ────────────────────────────────────────────────────

/// Snapshot dos counters do `comemo::evict` cristalino.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub evict_calls:  usize,
    pub last_max_age: usize,
}

/// Snapshot dos counters de invocação do trait `Introspector`.
///
/// `total` agrega chamadas a todos os 26 métodos (20 originais +
/// `query_labelled` P207B + `label_count` P207C + 4 page-aware
/// P207D: `pages`, `page`, `page_numbering`, `page_supplement`).
/// `per_method` preserva ordem de `INTROSPECTOR_METHODS`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CallCounts {
    pub total:      usize,
    pub per_method: Vec<(&'static str, usize)>,
}

impl CallCounts {
    /// Lookup por nome do método. Retorna 0 se nome desconhecido.
    pub fn count_for(&self, method: &str) -> usize {
        self.per_method
            .iter()
            .find(|(name, _)| *name == method)
            .map(|(_, count)| *count)
            .unwrap_or(0)
    }
}

/// Snapshot leve (`Ordering::Relaxed`) dos counters de cache.
pub fn cache_stats() -> CacheStats {
    CacheStats {
        evict_calls:  EVICT_CALLS.load(Ordering::Relaxed),
        last_max_age: LAST_MAX_AGE.load(Ordering::Relaxed),
    }
}

/// Snapshot leve (`Ordering::Relaxed`) dos counters de invocação.
pub fn introspector_call_counts() -> CallCounts {
    let per_method: Vec<(&'static str, usize)> = INTROSPECTOR_METHODS
        .iter()
        .enumerate()
        .map(|(i, name)| (*name, CALL_COUNTERS[i].load(Ordering::Relaxed)))
        .collect();
    let total = per_method.iter().map(|(_, c)| *c).sum();
    CallCounts { total, per_method }
}

/// Zera todos os counters. Tests usam para isolar measurements
/// entre cenários. **Não** chama `crystalline_evict(0)` — esse
/// é decisão separada do caller.
pub fn reset() {
    EVICT_CALLS.store(0, Ordering::Relaxed);
    LAST_MAX_AGE.store(0, Ordering::Relaxed);
    for counter in CALL_COUNTERS.iter() {
        counter.store(0, Ordering::Relaxed);
    }
}

/// Hook chamado por `crystalline_evict` (L4) antes de
/// `comemo::evict`. Tests podem chamar directamente para simular
/// sequência de evictions.
pub fn record_evict(max_age: usize) {
    EVICT_CALLS.fetch_add(1, Ordering::Relaxed);
    LAST_MAX_AGE.store(max_age, Ordering::Relaxed);
}

fn record_call(idx: usize) {
    CALL_COUNTERS[idx].fetch_add(1, Ordering::Relaxed);
}

// ── Wrapper newtype ────────────────────────────────────────────────

/// Wrapper newtype que delega cada método do trait `Introspector`
/// ao `inner` e incrementa o counter global correspondente.
///
/// Aplicado em test fixtures — não invade L1 production. Para
/// usar com `comemo::Track`, instanciar antes de `.track()` no
/// caller.
pub struct CountingIntrospector<I> {
    inner: I,
}

impl<I> CountingIntrospector<I> {
    pub fn new(inner: I) -> Self { Self { inner } }
    pub fn into_inner(self) -> I { self.inner }
    pub fn inner(&self) -> &I { &self.inner }
}

impl<I: Introspector + Send + Sync> Introspector for CountingIntrospector<I> {
    fn query_by_kind(&self, kind: ElementKind) -> Vec<Location> {
        record_call(0);
        self.inner.query_by_kind(kind)
    }

    fn query_by_label(&self, label: &Label) -> Option<Location> {
        record_call(1);
        self.inner.query_by_label(label)
    }

    fn query_first(&self, kind: ElementKind) -> Option<Location> {
        record_call(2);
        self.inner.query_first(kind)
    }

    fn query_unique(&self, kind: ElementKind) -> Option<Location> {
        record_call(3);
        self.inner.query_unique(kind)
    }

    fn position_of(&self, location: Location) -> Option<Position> {
        record_call(4);
        self.inner.position_of(location)
    }

    fn figure_number_for_label(&self, label: &Label) -> Option<usize> {
        record_call(5);
        self.inner.figure_number_for_label(label)
    }

    fn query_metadata(&self) -> &[Value] {
        record_call(6);
        self.inner.query_metadata()
    }

    fn formatted_counter(&self, key: &str) -> Option<String> {
        record_call(7);
        self.inner.formatted_counter(key)
    }

    fn state_value(&self, key: &str, location: Location) -> Option<&Value> {
        record_call(8);
        self.inner.state_value(key, location)
    }

    fn state_final_value(&self, key: &str) -> Option<&Value> {
        record_call(9);
        self.inner.state_final_value(key)
    }

    fn state_display_value(
        &self,
        key: String,
        location: Location,
    ) -> Option<Content> {
        record_call(9);
        self.inner.state_display_value(key, location)
    }

    fn query(&self, selector: &Selector) -> Vec<Location> {
        record_call(10);
        self.inner.query(selector)
    }

    fn formatted_counter_at(&self, key: &str, location: Location) -> Option<String> {
        record_call(11);
        self.inner.formatted_counter_at(key, location)
    }

    fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry> {
        record_call(12);
        self.inner.bib_entry_for_key(key)
    }

    fn bib_number_for_key(&self, key: &str) -> Option<u32> {
        record_call(13);
        self.inner.bib_number_for_key(key)
    }

    fn is_numbering_active(&self, key: &str) -> bool {
        record_call(14);
        self.inner.is_numbering_active(key)
    }

    fn figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize> {
        record_call(15);
        self.inner.figure_number_at_index(kind, idx)
    }

    fn is_numbering_active_at(&self, key: &str, location: Location) -> bool {
        record_call(16);
        self.inner.is_numbering_active_at(key, location)
    }

    fn flat_counter_at(&self, key: &str, location: Location) -> Option<usize> {
        record_call(17);
        self.inner.flat_counter_at(key, location)
    }

    fn resolved_label_for(&self, label: &Label) -> Option<&str> {
        record_call(18);
        self.inner.resolved_label_for(label)
    }

    fn headings_for_toc(&self) -> &[(Label, Content, usize)] {
        record_call(19);
        self.inner.headings_for_toc()
    }

    fn query_labelled(&self) -> Vec<(Label, Location)> {
        record_call(20);
        self.inner.query_labelled()
    }

    fn label_count(&self, label: &Label) -> usize {
        record_call(21);
        self.inner.label_count(label)
    }

    fn pages(&self, location: Location) -> Option<NonZeroUsize> {
        record_call(22);
        self.inner.pages(location)
    }

    fn page(&self, location: Location) -> Option<NonZeroUsize> {
        record_call(23);
        self.inner.page(location)
    }

    fn page_numbering(&self, location: Location) -> Option<&EcoString> {
        record_call(24);
        self.inner.page_numbering(location)
    }

    fn page_supplement(&self, location: Location) -> Option<&Content> {
        record_call(25);
        self.inner.page_supplement(location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use typst_core::entities::introspector::TagIntrospector;

    /// Tests partilham state global (`AtomicUsize` estáticos). Cargo
    /// corre tests em paralelo; serializar via Mutex para evitar
    /// flakes inter-test.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn lbl(s: &str) -> Label { Label(s.to_string()) }

    // ── Sentinelas (compile-time smoke) ──────────────────────────────

    #[test]
    fn p204g_cache_stats_existe() {
        // Sentinel: confirma que `cache_stats()` está disponível.
        // Falha de compilação se função for removida.
        let _stats: CacheStats = cache_stats();
    }

    #[test]
    fn p204g_introspector_call_counts_existe() {
        // Sentinel: confirma que `introspector_call_counts()` está
        // disponível e devolve `CallCounts`. Falha de compilação se
        // função/tipo forem removidos. Length 26 = 20 originais
        // (P204G) + `query_labelled` (P207B) + `label_count` (P207C)
        // + 4 page-aware (`pages`, `page`, `page_numbering`,
        // `page_supplement`) (P207D).
        let counts: CallCounts = introspector_call_counts();
        assert_eq!(counts.per_method.len(), 26);
    }

    // ── C6 Test 1 (smoke): tracking activo após uso ──────────────────

    #[test]
    fn p204g_smoke_tracking_activo_apos_chamadas() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();

        // Cenário leve: wrapper sobre introspector vazio + 1 evict
        // simulado. Sinal mínimo de que counters propagam: `total
        // > 0` OU `evict_calls > 0`. Cumpre o critério P204G C6
        // Test 1 (algum tracking activo).
        let wrapped = CountingIntrospector::new(TagIntrospector::empty());
        let _ = wrapped.query_by_kind(ElementKind::Heading);
        record_evict(0);

        let stats = cache_stats();
        let counts = introspector_call_counts();
        assert!(
            stats.evict_calls > 0 || counts.total > 0,
            "tracking inactivo: evict_calls={} total={}",
            stats.evict_calls,
            counts.total,
        );
    }

    // ── C6 Test 2 (counts): figure_number_for_label invocado N×  ────

    #[test]
    fn p204g_counts_figure_number_for_label_invocado_3x() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();

        let wrapped = CountingIntrospector::new(TagIntrospector::empty());
        // 3 invocações simulam o cenário figure-ref.typ (3 figures).
        let _ = wrapped.figure_number_for_label(&lbl("fig-alfa"));
        let _ = wrapped.figure_number_for_label(&lbl("fig-beta"));
        let _ = wrapped.figure_number_for_label(&lbl("fig-gama"));

        let counts = introspector_call_counts();
        assert!(
            counts.count_for("figure_number_for_label") >= 3,
            "esperado >= 3 invocações; obtido {}",
            counts.count_for("figure_number_for_label"),
        );
    }

    // ── C6 Test 3 (reset): contadores ficam a zero ───────────────────

    #[test]
    fn p204g_reset_zera_todos_os_counters() {
        let _guard = TEST_LOCK.lock().unwrap();

        record_evict(5);
        let wrapped = CountingIntrospector::new(TagIntrospector::empty());
        let _ = wrapped.query_by_label(&lbl("foo"));

        reset();

        let stats = cache_stats();
        let counts = introspector_call_counts();
        assert_eq!(stats.evict_calls, 0);
        assert_eq!(stats.last_max_age, 0);
        assert_eq!(counts.total, 0);
        for (method, count) in &counts.per_method {
            assert_eq!(*count, 0, "{} não foi zerado", method);
        }
    }

    // ── C6 Test 4 (regressão): evict cresce em chamadas sucessivas ──

    #[test]
    fn p204g_regressao_record_evict_cresce_em_chamadas_sucessivas() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();

        record_evict(10);
        let stats_apos_1 = cache_stats();
        record_evict(20);
        let stats_apos_2 = cache_stats();

        assert_eq!(stats_apos_1.evict_calls, 1);
        assert_eq!(stats_apos_1.last_max_age, 10);
        assert_eq!(stats_apos_2.evict_calls, 2);
        assert_eq!(stats_apos_2.last_max_age, 20);
    }

    // ── C6 Test 5: record_evict não zera measurement counters ───────

    #[test]
    fn p204g_record_evict_nao_zera_measurement_counters() {
        // Decisão de design (per L0 prompt): `reset()` é separado de
        // `record_evict`. Limpar cache do comemo não limpa counters
        // de instrumentação — caller controla.
        let _guard = TEST_LOCK.lock().unwrap();
        reset();

        let wrapped = CountingIntrospector::new(TagIntrospector::empty());
        let _ = wrapped.query_by_kind(ElementKind::Heading);
        record_evict(0);

        let counts = introspector_call_counts();
        assert!(counts.total >= 1, "counters foram zerados por evict");
        assert_eq!(counts.count_for("query_by_kind"), 1);
    }

    // ── Sub-teste auxiliar: count_for para método desconhecido ───────

    #[test]
    fn p204g_call_counts_for_metodo_desconhecido_devolve_zero() {
        let counts = introspector_call_counts();
        assert_eq!(counts.count_for("metodo_inexistente"), 0);
    }
}
