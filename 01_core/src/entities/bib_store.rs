//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/bib_store.md
//! @prompt-hash 4051b23d
//! @layer L1
//! @updated 2026-06-23
//!
//! `BibStore` — sub-store de `TagIntrospector` para entries
//! bibliográficas + numeração 1-based. P181B (M9 sub-passo bib).
//!
//! **P429 (DEBT-63)** — `resolved_style` deixou de viver em
//! `BibliographyElem` (cache de estado computado dentro de um struct de
//! domínio). O style CSL resolvido em eval time é agora transportado numa
//! tabela lateral `bib_styles` indexada por `BibStyleKey` (hash
//! determinístico do `BibliographyElem` puro). O layout reproduz a mesma
//! chave a partir do primeiro `Content::Bibliography` e consulta a tabela.

use std::collections::HashMap;
use std::sync::Arc;

use hayagriva::citationberg::IndependentStyle;

use crate::entities::bib_entry::BibEntry;

/// Chave determinística para o style resolvido de uma `Bibliography`.
pub type BibStyleKey = u64;

/// Acumulador de entries bibliográficas + mapa key→número 1-based +
/// tabela lateral de styles resolvidos (P429).
///
/// `from_tags` (P181E) popula via `add_bibliography` +
/// `assign_number` ao processar `Tag::Start(_, info)` onde
/// `info.payload == ElementPayload::Bibliography { entries }`. Os styles
/// resolvidos são injectados pelo pipeline (`03_infra`) a partir do
/// `Module` produzido pelo eval.
///
/// Ordem de `entries` preservada por `Vec` interno; multi-Bibliography
/// concatena (cláusula 2 P181A). `numbers` preserva primeiro número
/// via `or_insert` (cláusula 3 P181A).
#[derive(Debug, Clone, Default)]
pub struct BibStore {
    entries: Vec<BibEntry>,
    numbers: HashMap<String, u32>,
    /// **P429** — styles CSL resolvidos em eval time, indexados pela chave
    /// do `BibliographyElem` correspondente. Tabela lateral: não polui a
    /// identidade do elemento.
    bib_styles: HashMap<BibStyleKey, Arc<IndependentStyle>>,
}

impl BibStore {
    /// Cria store vazio. Equivalente a `Default::default()`.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Slice das entries em ordem de inserção (= ordem de aparecimento
    /// no walk; multi-Bib concatena).
    pub fn entries(&self) -> &[BibEntry] {
        &self.entries
    }

    /// Linear scan sobre `entries` por `BibEntry.key`. Replica
    /// `state.bib_entries.iter().find(|e| e.key == *key)` actual em
    /// `layout/mod.rs:584` (P181G migrará caller para via Introspector).
    pub fn entry_for_key(&self, key: &str) -> Option<&BibEntry> {
        self.entries.iter().find(|e| e.key == key)
    }

    /// Lookup O(1) via HashMap. `Some(n)` se key registada via
    /// `assign_number`; `None` caso contrário.
    pub fn number_for_key(&self, key: &str) -> Option<u32> {
        self.numbers.get(key).copied()
    }

    /// Número de entries acumuladas.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Número de keys com número atribuído. Cresce com `or_insert` —
    /// keys duplicadas em multi-Bibliography contam **uma** vez.
    /// Replica `state.bib_numbers.len()` em walk arm
    /// `Content::Bibliography` (introspect.rs:569).
    pub fn numbers_len(&self) -> usize {
        self.numbers.len()
    }

    /// `true` se nenhuma Bibliography foi adicionada.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.numbers.is_empty()
    }

    /// Adiciona entries por `extend`. Replica
    /// `state.bib_entries.extend(...)` actual em `introspect.rs:572`.
    /// **Não** atribui numbers — caller chama `assign_number` em
    /// separado para cada entry.
    pub(crate) fn add_bibliography(&mut self, entries: Vec<BibEntry>) {
        self.entries.extend(entries);
    }

    /// Atribui `number` à `key` se ainda não houver entrada — usa
    /// `or_insert`. Replica `state.bib_numbers.entry(key).or_insert(...)`
    /// actual em `introspect.rs:570`. Primeiro número de uma key
    /// persiste em multi-Bibliography com keys duplicadas.
    pub(crate) fn assign_number(&mut self, key: String, number: u32) {
        self.numbers.entry(key).or_insert(number);
    }

    /// **P429** — regista um style resolvido para a chave indicada.
    pub fn add_style(&mut self, key: BibStyleKey, style: Arc<IndependentStyle>) {
        self.bib_styles.insert(key, style);
    }

    /// **P429** — devolve o style resolvido associado à chave, se existir.
    pub fn style_for_key(&self, key: BibStyleKey) -> Option<&Arc<IndependentStyle>> {
        self.bib_styles.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::bib_entry::BibEntry;
    use crate::rules::layout::bib_csl::resolve_style_name;

    fn make_entry(key: &str) -> BibEntry {
        BibEntry {
            key:          key.to_string(),
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
        }
    }

    #[test]
    fn empty_produz_estado_vazio() {
        let store = BibStore::empty();
        assert!(store.entries().is_empty());
        assert_eq!(store.len(), 0);
        assert!(store.is_empty());
        assert_eq!(store.entry_for_key("any"), None);
        assert_eq!(store.number_for_key("any"), None);
        assert_eq!(store.style_for_key(0), None);
    }

    #[test]
    fn add_bibliography_extend_replica_legacy() {
        let mut store = BibStore::empty();
        store.add_bibliography(vec![make_entry("a"), make_entry("b")]);
        assert_eq!(store.len(), 2);
        store.add_bibliography(vec![make_entry("c"), make_entry("d")]);
        assert_eq!(store.len(), 4);
    }

    #[test]
    fn assign_number_or_insert_nao_sobrescreve() {
        let mut store = BibStore::empty();
        store.assign_number("a".to_string(), 1);
        store.assign_number("a".to_string(), 2);
        assert_eq!(store.number_for_key("a"), Some(1));
    }

    #[test]
    fn entry_for_key_inexistente_devolve_none() {
        let store = BibStore::empty();
        assert_eq!(store.entry_for_key("nao_existe"), None);
    }

    #[test]
    fn number_for_key_inexistente_devolve_none() {
        let store = BibStore::empty();
        assert_eq!(store.number_for_key("nao_existe"), None);
    }

    #[test]
    fn entry_for_key_populado_devolve_referencia() {
        let mut store = BibStore::empty();
        store.add_bibliography(vec![make_entry("intro"), make_entry("conclusao")]);
        let found = store.entry_for_key("intro");
        assert!(found.is_some());
        assert_eq!(found.unwrap().key, "intro");
        assert_eq!(store.entry_for_key("conclusao").unwrap().key, "conclusao");
    }

    #[test]
    fn entries_preserva_ordem_em_multi_bib() {
        let mut store = BibStore::empty();
        store.add_bibliography(vec![make_entry("primeiro"), make_entry("segundo")]);
        store.add_bibliography(vec![make_entry("terceiro")]);
        let keys: Vec<&str> = store.entries().iter().map(|e| e.key.as_str()).collect();
        assert_eq!(keys, vec!["primeiro", "segundo", "terceiro"]);
    }

    #[test]
    fn add_style_e_lookup_por_chave() {
        let mut store = BibStore::empty();
        let style = Arc::new(resolve_style_name("ieee").expect("ieee existe"));
        store.add_style(123, style.clone());
        assert!(store.style_for_key(123).is_some());
        assert_eq!(store.style_for_key(123).unwrap().info.title.value, style.info.title.value);
        assert!(store.style_for_key(999).is_none());
    }
}
