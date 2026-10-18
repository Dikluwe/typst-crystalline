//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/state_registry.md
//! @prompt-hash 368ce021
//! @layer L1
//! @updated 2026-10-17
//!
//! Histórico materializado legado e ações causais compartilhadas de State.
//! Leitores modernos resolvem as ações com seu próprio init. A ordenação usa
//! a posição registrada no documento; payloads Set são compartilhados por Arc.

use std::collections::HashMap;
use std::sync::Arc;

use crate::entities::func::Func;
use crate::entities::location::Location;
use crate::entities::state_update::StateUpdate;
use crate::entities::value::Value;

#[derive(Debug, Clone)]
pub(crate) enum RecordedStateUpdate {
    Set(Arc<Value>),
    Func(Func),
}

#[derive(Debug, Clone)]
struct StateAction {
    location: Location,
    update: RecordedStateUpdate,
}

/// Sub-store de runtime mutable state para `Introspector`.
///
/// Read-only após construção. Mutação só via `pub(crate) fn init`/
/// `update` durante construção em `from_tags`.
#[derive(Debug, Clone, Default)]
pub struct StateRegistry {
    /// `key → ordered list of (Location, Value)`. Init é a primeira
    /// entrada (cronologicamente). Updates seguintes adicionam ao Vec.
    inner: HashMap<String, Vec<(Location, Arc<Value>)>>,
    document_order: HashMap<Location, usize>,
    actions: HashMap<String, Arc<Vec<StateAction>>>,
}

impl StateRegistry {
    pub(crate) fn record_action(
        &mut self,
        key: &str,
        update: &StateUpdate,
        location: Location,
    ) {
        let actions = self.actions.entry(key.into()).or_default();
        if actions.iter().any(|action| action.location == location) {
            return;
        }
        let update = match update {
            StateUpdate::Set(value) => {
                let value = Arc::new((**value).clone());
                self.inner
                    .entry(key.into())
                    .or_default()
                    .push((location, value.clone()));
                RecordedStateUpdate::Set(value)
            }
            StateUpdate::Func(func) => RecordedStateUpdate::Func(func.clone()),
        };
        Arc::make_mut(actions).push(StateAction { location, update });
    }

    pub(crate) fn actions_for(&self, key: &str) -> Vec<(u128, RecordedStateUpdate)> {
        let Some(actions) = self.actions.get(key) else { return Vec::new() };
        let mut ordered: Vec<_> = actions
            .iter()
            .map(|action| (self.order_key(action.location), action.update.clone()))
            .collect();
        ordered.sort_by_key(|(order, _)| *order);
        ordered
    }

    pub(crate) fn legacy_init(
        &self,
        key: &str,
        location: Option<Location>,
    ) -> Option<Arc<Value>> {
        let (at, value) = self.inner.get(key)?.first()?;
        location
            .is_none_or(|end| self.order_key(*at) <= self.order_key(end))
            .then(|| value.clone())
    }

    pub(crate) fn observe_location(&mut self, location: Location) {
        let ordinal = self.document_order.len();
        self.document_order.entry(location).or_insert(ordinal);
    }

    /// Register a fresh causal block without changing existing identities or histories.
    pub(crate) fn observe_locations_after(
        &mut self,
        anchor: Location,
        locations: &[Location],
    ) {
        let Some(anchor_order) = self.document_order.get(&anchor).copied() else {
            return;
        };
        let mut seen = std::collections::HashSet::new();
        let fresh: Vec<_> = locations
            .iter()
            .copied()
            .filter(|location| {
                !self.document_order.contains_key(location) && seen.insert(*location)
            })
            .collect();
        if fresh.is_empty() {
            return;
        }
        for ordinal in self.document_order.values_mut() {
            if *ordinal > anchor_order {
                *ordinal += fresh.len();
            }
        }
        for (offset, location) in fresh.into_iter().enumerate() {
            self.document_order.insert(location, anchor_order + offset + 1);
        }
    }

    pub(crate) fn order_key(&self, location: Location) -> u128 {
        self.document_order
            .get(&location)
            .map_or_else(|| location.as_u128(), |n| *n as u128)
    }

    /// Cria registry vazio.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Regista `init` para o state `key` na `Location` indicada.
    /// Apenas a primeira chamada `init` para cada key é considerada
    /// (multiple `state(key, init)` no mesmo doc é incomum mas o
    /// primeiro ganha nesta vista legada).
    pub(crate) fn init(&mut self, key: String, init: Value, location: Location) {
        // Apenas adiciona se key não existe ainda — primeiro init ganha.
        self.inner
            .entry(key)
            .or_insert_with(|| vec![(location, Arc::new(init))]);
    }

    /// Regista um update para o state `key` na `Location` indicada.
    /// Se key não foi inicializada, o update é ignorado (comportamento
    /// defensivo da vista legada, distinto do init por leitor).
    pub(crate) fn update(&mut self, key: String, value: Value, location: Location) {
        // Aplicar StateUpdate::Set: registar (location, value).
        if let Some(history) = self.inner.get_mut(&key) {
            history.push((location, Arc::new(value)));
        }
        // Se key não existe ainda, ignorar update (sem init prévio).
    }

    /// Devolve o valor do state `key` na `Location` indicada.
    /// Algoritmo: encontrar último update onde loc <= location; se
    /// nenhum, devolve o init (que é a primeira entrada).
    pub fn value_at(&self, key: &str, location: Location) -> Option<&Value> {
        let history = self.inner.get(key)?;
        history
            .iter()
            .rfind(|(loc, _)| self.order_key(*loc) <= self.order_key(location))
            .map(|(_, v)| v.as_ref())
    }

    /// Devolve o valor final do state `key` (último update aplicado).
    pub fn final_value(&self, key: &str) -> Option<&Value> {
        let history = self.inner.get(key)?;
        history.last().map(|(_, v)| v.as_ref())
    }

    /// Número de keys distintas registadas.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// `true` se nenhuma key foi registada.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loc(raw: u128) -> Location {
        Location::from_raw(raw)
    }

    #[test]
    fn empty_devolve_none() {
        let r = StateRegistry::empty();
        assert_eq!(r.value_at("counter", loc(1)), None);
        assert_eq!(r.final_value("counter"), None);
        assert!(r.is_empty());
    }

    #[test]
    fn init_only_devolve_init() {
        let mut r = StateRegistry::empty();
        r.init("counter".to_string(), Value::Int(0), loc(10));
        assert_eq!(r.value_at("counter", loc(10)), Some(&Value::Int(0)));
        assert_eq!(r.value_at("counter", loc(20)), Some(&Value::Int(0)));
        assert_eq!(r.final_value("counter"), Some(&Value::Int(0)));
    }

    #[test]
    fn update_apos_init_aplica_no_ponto_correcto() {
        let mut r = StateRegistry::empty();
        r.init("counter".to_string(), Value::Int(0), loc(10));
        r.update("counter".to_string(), Value::Int(5), loc(20));
        // Antes do update: ainda init.
        assert_eq!(r.value_at("counter", loc(10)), Some(&Value::Int(0)));
        assert_eq!(r.value_at("counter", loc(15)), Some(&Value::Int(0)));
        // Após o update: novo valor.
        assert_eq!(r.value_at("counter", loc(20)), Some(&Value::Int(5)));
        assert_eq!(r.value_at("counter", loc(30)), Some(&Value::Int(5)));
        // Final.
        assert_eq!(r.final_value("counter"), Some(&Value::Int(5)));
    }

    #[test]
    fn multiplos_updates_em_ordem() {
        let mut r = StateRegistry::empty();
        r.init("c".to_string(), Value::Int(0), loc(10));
        r.update("c".to_string(), Value::Int(1), loc(20));
        r.update("c".to_string(), Value::Int(2), loc(30));
        r.update("c".to_string(), Value::Int(3), loc(40));
        assert_eq!(r.value_at("c", loc(15)), Some(&Value::Int(0)));
        assert_eq!(r.value_at("c", loc(25)), Some(&Value::Int(1)));
        assert_eq!(r.value_at("c", loc(35)), Some(&Value::Int(2)));
        assert_eq!(r.value_at("c", loc(45)), Some(&Value::Int(3)));
        assert_eq!(r.final_value("c"), Some(&Value::Int(3)));
    }

    #[test]
    fn keys_distintas_sao_isoladas() {
        let mut r = StateRegistry::empty();
        r.init("a".to_string(), Value::Int(1), loc(10));
        r.init("b".to_string(), Value::Int(100), loc(11));
        r.update("a".to_string(), Value::Int(2), loc(20));
        assert_eq!(r.value_at("a", loc(20)), Some(&Value::Int(2)));
        assert_eq!(r.value_at("b", loc(20)), Some(&Value::Int(100)));
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn update_sem_init_e_ignorado() {
        let mut r = StateRegistry::empty();
        r.update("c".to_string(), Value::Int(5), loc(20));
        // key nunca foi inicializada → update ignorado.
        assert_eq!(r.value_at("c", loc(20)), None);
    }

    #[test]
    fn segundo_init_e_ignorado() {
        let mut r = StateRegistry::empty();
        r.init("c".to_string(), Value::Int(0), loc(10));
        r.init("c".to_string(), Value::Int(99), loc(15)); // ignored
        assert_eq!(r.value_at("c", loc(15)), Some(&Value::Int(0)));
    }
}

#[cfg(test)]
mod v16_walk_order_contract_tests {
    use super::*;
    use crate::entities::locator::Locator;

    #[test]
    fn v16_walk_order_block_preserves_identity_density_and_idempotence() {
        let mut allocator = Locator::new();
        let later = allocator.next();
        let anchor = allocator.next();
        let inserted = allocator.next();
        let unknown = allocator.next();
        let next = allocator.next();
        let mut registry = StateRegistry::empty();
        registry.observe_location(anchor);
        registry.observe_location(later);
        registry.observe_locations_after(anchor, &[inserted, inserted, anchor, later]);
        assert_eq!(registry.order_key(anchor), 0);
        assert_eq!(registry.order_key(inserted), 1);
        assert_eq!(registry.order_key(later), 2);
        let previous = registry.document_order.clone();
        registry.observe_locations_after(anchor, &[inserted]);
        registry.observe_locations_after(unknown, &[next]);
        assert_eq!(registry.document_order, previous);
        registry.observe_location(next);
        assert_eq!(registry.order_key(next), 3);
    }
}

#[cfg(test)]
mod a1_action_tests {
    use super::*;

    #[test]
    fn a1_actions_follow_causal_order_and_share_set_payloads_across_snapshots() {
        let mut registry = StateRegistry::empty();
        let first = Location::from_raw(100);
        let second = Location::from_raw(1);
        registry.observe_location(first);
        registry.observe_location(second);
        registry.record_action(
            "k",
            &StateUpdate::Set(Box::new(Value::Array(vec![Value::Int(7)]))),
            first,
        );
        let snapshot = registry.clone();
        registry.record_action("k", &StateUpdate::Set(Box::new(Value::Int(9))), second);
        registry.record_action("k", &StateUpdate::Set(Box::new(Value::Int(99))), second);
        let original = snapshot.actions_for("k");
        let changed = registry.actions_for("k");
        assert_eq!(original.len(), 1);
        assert_eq!(changed.len(), 2);
        assert_eq!(
            changed.iter().map(|(order, _)| *order).collect::<Vec<_>>(),
            vec![0, 1]
        );
        let RecordedStateUpdate::Set(old) = &original[0].1 else { panic!("set action") };
        let RecordedStateUpdate::Set(new) = &changed[0].1 else { panic!("set action") };
        assert!(Arc::ptr_eq(old, new));
        assert!(std::ptr::eq(old.as_ref(), snapshot.final_value("k").unwrap()));
        assert_eq!(registry.final_value("k"), Some(&Value::Int(9)));
    }
}
