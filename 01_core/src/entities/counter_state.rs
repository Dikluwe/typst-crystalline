//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_state.md
//! @prompt-hash 96f3d946
//! @layer L1
//! @updated 2026-04-12

use std::collections::HashMap;

use crate::entities::{content::Content, label::Label};

/// Instrução de modificação de um contador.
#[derive(Debug, Clone, PartialEq)]
pub enum CounterAction {
    /// Avança o contador em 1 (flat) ou avança o nível (hierárquico).
    Step,
    /// Força o contador para o valor indicado.
    Update(usize),
}

/// Estado de contadores que viaja com o Layouter durante uma passagem.
///
/// Cristalino diverge do Typst original aqui: o original resolve contadores
/// em duas passagens com `comemo` (para suportar referências para a frente).
/// Esta implementação usa uma única passagem — suficiente para numeração
/// sequencial de headings e contadores planos.
///
/// DEBT-10: Resolver contadores em duas passagens com estado global quando
/// o motor de introspecção completo for implementado (Passos 60+).
#[derive(Debug, Clone, Default)]
pub struct CounterState {
    /// Contadores hierárquicos (ex: heading).
    /// Chave "heading" → `[1, 2]` representa a secção 1.2.
    hierarchical: HashMap<String, Vec<usize>>,
    /// Contadores planos (ex: equation, figure, ou chaves arbitrárias).
    flat: HashMap<String, usize>,
    /// Flags de numeração activa por chave.
    pub numbering_active: HashMap<String, bool>,
    /// Mapa de labels para o texto resolvido na passagem actual.
    /// Chave: Label; Valor: texto formatado (ex: "Secção 1.1", "Equação (2)").
    pub resolved_labels: HashMap<Label, String>,
    /// Títulos catalogados para a TOC (Passo 61).
    /// Tupla: (label automática, corpo do título como Content, nível).
    /// Guardar Content em vez de String preserva formatação (negrito,
    /// itálico, equações inline) na TOC — `plain_text()` destruiria isso.
    pub headings_for_toc: Vec<(Label, Content, usize)>,
    /// Contador interno para gerar labels únicas para cada heading.
    /// Não representa numeração de secções — é apenas um gerador de IDs.
    pub auto_label_counter: usize,
}

impl CounterState {
    pub fn new() -> Self {
        let mut s = Self::default();
        // Figuras são numeradas por defeito — paridade com o Typst original.
        // O método is_numbering_active() não conhece esta regra; o construtor sim.
        // DEBT-14: sem SetRule para `#set figure(numbering: none)`, o utilizador
        // não pode desactivar a numeração de figuras.
        s.numbering_active.insert("figure".to_string(), true);
        s
    }

    /// Verifica se a numeração está activa para uma chave.
    pub fn is_numbering_active(&self, key: &str) -> bool {
        self.numbering_active.get(key).copied().unwrap_or(false)
    }

    /// Avança o contador hierárquico para o nível indicado.
    ///
    /// Comportamento (chave "heading", nível):
    /// - `[]` + 1 → `[1]`
    /// - `[1]` + 2 → `[1, 1]`
    /// - `[1, 1]` + 1 → `[2]`
    /// - `[1, 2]` + 2 → `[1, 3]`
    pub fn step_hierarchical(&mut self, key: &str, level: usize) {
        let level = level.max(1);
        let counter = self.hierarchical.entry(key.to_string()).or_default();
        counter.truncate(level);
        if counter.len() < level {
            counter.resize(level - 1, 0);
            counter.push(1);
        } else if let Some(last) = counter.last_mut() {
            *last += 1;
        }
    }

    /// Formata o contador hierárquico. Retorna `None` se vazio ou chave inexistente.
    pub fn format_hierarchical(&self, key: &str) -> Option<String> {
        let counter = self.hierarchical.get(key)?;
        if counter.is_empty() {
            None
        } else {
            Some(counter.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."))
        }
    }

    /// Avança um contador plano em 1.
    pub fn step_flat(&mut self, key: &str) {
        *self.flat.entry(key.to_string()).or_insert(0) += 1;
    }

    /// Força um contador plano para um valor específico.
    pub fn update_flat(&mut self, key: &str, value: usize) {
        self.flat.insert(key.to_string(), value);
    }

    /// Lê o valor actual de um contador plano.
    pub fn get_flat(&self, key: &str) -> usize {
        self.flat.get(key).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Testes herdados do Passo 57 (via step_hierarchical) ──────────────

    #[test]
    fn step_heading_nivel_1_inicial() {
        let mut s = CounterState::new();
        s.step_hierarchical("heading", 1);
        assert_eq!(s.format_hierarchical("heading"), Some("1".to_string()));
    }

    #[test]
    fn step_heading_dois_niveis() {
        let mut s = CounterState::new();
        s.step_hierarchical("heading", 1);
        s.step_hierarchical("heading", 2);
        assert_eq!(s.format_hierarchical("heading"), Some("1.1".to_string()));
    }

    #[test]
    fn step_heading_nivel_2_apos_nivel_2() {
        let mut s = CounterState::new();
        s.step_hierarchical("heading", 1);
        s.step_hierarchical("heading", 2);
        s.step_hierarchical("heading", 2);
        assert_eq!(s.format_hierarchical("heading"), Some("1.2".to_string()));
    }

    #[test]
    fn step_heading_volta_ao_nivel_1() {
        let mut s = CounterState::new();
        s.step_hierarchical("heading", 1);
        s.step_hierarchical("heading", 2);
        s.step_hierarchical("heading", 1);
        assert_eq!(s.format_hierarchical("heading"), Some("2".to_string()));
    }

    #[test]
    fn step_heading_tres_niveis_sequencia_completa() {
        let mut s = CounterState::new();
        s.step_hierarchical("heading", 1); // [1]
        s.step_hierarchical("heading", 2); // [1, 1]
        s.step_hierarchical("heading", 3); // [1, 1, 1]
        s.step_hierarchical("heading", 2); // [1, 2]
        s.step_hierarchical("heading", 1); // [2]
        assert_eq!(s.format_hierarchical("heading"), Some("2".to_string()));
    }

    #[test]
    fn format_heading_vazio_retorna_none() {
        let s = CounterState::new();
        assert_eq!(s.format_hierarchical("heading"), None);
    }

    // ── Testes novos do Passo 58 ─────────────────────────────────────────

    #[test]
    fn step_flat_incrementa() {
        let mut s = CounterState::new();
        s.step_flat("equation");
        assert_eq!(s.get_flat("equation"), 1);
        s.step_flat("equation");
        assert_eq!(s.get_flat("equation"), 2);
    }

    #[test]
    fn update_flat_forca_valor() {
        let mut s = CounterState::new();
        s.step_flat("figure");
        s.update_flat("figure", 5);
        assert_eq!(s.get_flat("figure"), 5);
    }

    #[test]
    fn step_hierarchical_comportamento_identico_ao_passo_57() {
        let mut s = CounterState::new();
        s.step_hierarchical("heading", 1); // [1]
        s.step_hierarchical("heading", 2); // [1, 1]
        s.step_hierarchical("heading", 1); // [2]
        assert_eq!(s.format_hierarchical("heading"), Some("2".to_string()));
    }

    #[test]
    fn contadores_independentes_nao_interferem() {
        let mut s = CounterState::new();
        s.step_flat("equation");
        s.step_flat("equation");
        s.step_flat("figure");
        assert_eq!(s.get_flat("equation"), 2);
        assert_eq!(s.get_flat("figure"),   1);
    }
}
