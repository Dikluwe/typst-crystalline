//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_registry.md
//! @prompt-hash b72b6747
//! @layer L1
//! @updated 2026-04-30
//!
//! `CounterRegistry` — sub-store de counters por kind para
//! `Introspector`. P165 sub-passo .C (M3 Introspection).
//!
//! Forma simplificada M3: flat counter por kind. Hierarquia rica
//! adiada para M9+ paralelamente a `CounterKey` enum vanilla.

use std::collections::HashMap;

use crate::entities::counter_update::CounterUpdate;

/// Counters indexados por kind (string). M3 minimal: flat counters
/// (1 nível por kind). Mutável só durante construção em
/// `from_tags` via `pub(crate) fn apply`.
#[derive(Debug, Clone, Default)]
pub struct CounterRegistry {
    inner: HashMap<String, Vec<usize>>,
}

impl CounterRegistry {
    /// Cria registry vazio.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Devolve slice actual do counter. `None` se nunca foi tocado.
    pub fn value(&self, key: &str) -> Option<&[usize]> {
        self.inner.get(key).map(Vec::as_slice)
    }

    /// Número de kinds distintos com counter registado.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// `true` se nenhum counter foi tocado.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Aplica `update` ao counter identificado por `key`. Apenas
    /// usado pelo construtor `from_tags` em
    /// `rules/introspect/from_tags.rs`.
    ///
    /// Semântica:
    /// - `Step`: incrementa o último elemento. Se vector vazio,
    ///   inicializa em `[1]`.
    /// - `Update(v)`: define para `[v]` (reseta).
    pub(crate) fn apply(&mut self, key: String, update: CounterUpdate) {
        let entry = self.inner.entry(key).or_default();
        match update {
            CounterUpdate::Step => {
                if let Some(last) = entry.last_mut() {
                    *last += 1;
                } else {
                    entry.push(1);
                }
            }
            CounterUpdate::Update(v) => {
                entry.clear();
                entry.push(v);
            }
        }
    }

    /// **P170 (M9 sub-passo 2)** — aplica step hierárquico ao nível
    /// indicado. Paridade exacta com `CounterStateLegacy::step_hierarchical`.
    ///
    /// Comportamento (chave "heading", level):
    /// - `[]`     + 1 → `[1]`
    /// - `[1]`    + 2 → `[1, 1]`
    /// - `[1, 1]` + 1 → `[2]`
    /// - `[1, 2]` + 2 → `[1, 3]`
    ///
    /// `level` é clamped a mínimo 1.
    pub(crate) fn apply_hierarchical(&mut self, key: String, level: usize) {
        let level = level.max(1);
        let counter = self.inner.entry(key).or_default();
        counter.truncate(level);
        if counter.len() < level {
            counter.resize(level - 1, 0);
            counter.push(1);
        } else if let Some(last) = counter.last_mut() {
            *last += 1;
        }
    }

    /// **P170 (M9 sub-passo 2)** — formato hierárquico do counter
    /// como string ("1.2.3"). Retorna `None` se key não existe ou
    /// counter está vazio. Paridade exacta com
    /// `CounterStateLegacy::format_hierarchical`.
    pub fn format(&self, key: &str) -> Option<String> {
        let counter = self.inner.get(key)?;
        if counter.is_empty() {
            None
        } else {
            Some(counter.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_value_devolve_none() {
        let r = CounterRegistry::empty();
        assert_eq!(r.value("heading"), None);
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn step_inicial_devolve_um() {
        let mut r = CounterRegistry::empty();
        r.apply("heading".to_string(), CounterUpdate::Step);
        assert_eq!(r.value("heading"), Some(&[1usize][..]));
    }

    #[test]
    fn tres_steps_consecutivos_produzem_tres() {
        let mut r = CounterRegistry::empty();
        for _ in 0..3 {
            r.apply("heading".to_string(), CounterUpdate::Step);
        }
        assert_eq!(r.value("heading"), Some(&[3usize][..]));
    }

    #[test]
    fn update_reseta_para_valor_dado() {
        let mut r = CounterRegistry::empty();
        r.apply("heading".to_string(), CounterUpdate::Step);
        r.apply("heading".to_string(), CounterUpdate::Step);
        r.apply("heading".to_string(), CounterUpdate::Update(42));
        assert_eq!(r.value("heading"), Some(&[42usize][..]));
    }

    #[test]
    fn counters_isolados_por_kind() {
        let mut r = CounterRegistry::empty();
        r.apply("heading".to_string(), CounterUpdate::Step);
        r.apply("figure".to_string(), CounterUpdate::Step);
        r.apply("figure".to_string(), CounterUpdate::Step);
        assert_eq!(r.value("heading"), Some(&[1usize][..]));
        assert_eq!(r.value("figure"), Some(&[2usize][..]));
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn lookup_de_kind_inexistente_devolve_none() {
        let mut r = CounterRegistry::empty();
        r.apply("heading".to_string(), CounterUpdate::Step);
        assert_eq!(r.value("inexistent"), None);
    }

    // ── P170 (M9 sub-passo 2) — hierarquia + format ──────────────────────

    #[test]
    fn apply_hierarchical_passa_de_vazio_para_um() {
        let mut r = CounterRegistry::empty();
        r.apply_hierarchical("heading".to_string(), 1);
        assert_eq!(r.value("heading"), Some(&[1usize][..]));
    }

    #[test]
    fn apply_hierarchical_sequencia_typica() {
        // Sequência [1, 2, 2, 3] (paridade com CounterStateLegacy
        // step_hierarchical): produz "1.2.1".
        let mut r = CounterRegistry::empty();
        r.apply_hierarchical("heading".to_string(), 1);  // [1]
        r.apply_hierarchical("heading".to_string(), 2);  // [1, 1]
        r.apply_hierarchical("heading".to_string(), 2);  // [1, 2]
        r.apply_hierarchical("heading".to_string(), 3);  // [1, 2, 1]
        assert_eq!(r.value("heading"), Some(&[1usize, 2, 1][..]));
    }

    #[test]
    fn apply_hierarchical_subir_nivel_reseta_inferior() {
        // [1, 2] + level 1 → [2] (truncar para 1 elemento, incrementar).
        let mut r = CounterRegistry::empty();
        r.apply_hierarchical("heading".to_string(), 1);  // [1]
        r.apply_hierarchical("heading".to_string(), 2);  // [1, 1]
        r.apply_hierarchical("heading".to_string(), 2);  // [1, 2]
        r.apply_hierarchical("heading".to_string(), 1);  // [2]
        assert_eq!(r.value("heading"), Some(&[2usize][..]));
    }

    #[test]
    fn apply_hierarchical_level_zero_clamped_para_um() {
        let mut r = CounterRegistry::empty();
        r.apply_hierarchical("heading".to_string(), 0);
        assert_eq!(r.value("heading"), Some(&[1usize][..]));
    }

    #[test]
    fn format_devolve_string_pontuada() {
        let mut r = CounterRegistry::empty();
        r.apply_hierarchical("heading".to_string(), 1);
        r.apply_hierarchical("heading".to_string(), 2);
        r.apply_hierarchical("heading".to_string(), 3);
        assert_eq!(r.format("heading").as_deref(), Some("1.1.1"));
    }

    #[test]
    fn format_inexistente_devolve_none() {
        let r = CounterRegistry::empty();
        assert_eq!(r.format("heading"), None);
    }

    #[test]
    fn format_de_counter_flat_funciona_tambem() {
        // format() não distingue flat vs hierárquico — joins Vec.
        let mut r = CounterRegistry::empty();
        r.apply("figure".to_string(), CounterUpdate::Step);
        r.apply("figure".to_string(), CounterUpdate::Step);
        // Counter flat após 2 steps continua [2].
        assert_eq!(r.format("figure").as_deref(), Some("2"));
    }
}
