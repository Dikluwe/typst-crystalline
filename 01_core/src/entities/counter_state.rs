//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_state.md
//! @prompt-hash 4b8e4f02
//! @layer L1
//! @updated 2026-04-20

use std::collections::HashMap;

use crate::entities::{content::Content, label::Label, lang::Lang};

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
    /// Mapa de labels para o número de página onde aterraram (Passo 63).
    /// Populado por `layout_labelled` (escrita por `references.rs`).
    /// Começa vazio em cada iteração via `Layouter::new()`.
    pub label_pages: HashMap<Label, usize>,
    /// Mapa de páginas da iteração anterior — lido por `outline.rs` (Passo 65).
    /// Separação leitura/escrita: `references.rs` escreve em `label_pages`;
    /// `outline.rs` lê de `known_page_numbers`. Injectado pelo fixpoint em `layout()`.
    pub known_page_numbers: HashMap<Label, usize>,
    /// Verdadeiro se o documento contém pelo menos um nó `Content::Outline` (Passo 65).
    /// Determina se o fixpoint de páginas é necessário.
    /// Populado pela introspecção — não pela contagem de títulos.
    pub has_outline: bool,
    /// Modo read-only: bloqueia step_* e update_* (Passo 63, DEBT-13).
    /// Activado em `outline.rs` durante a renderização de clones de AST na TOC.
    pub is_readonly: bool,
    /// Números pré-calculados por kind para figuras numeradas (Passo 75, DEBT-14/15).
    /// Chave: kind (ex: "image", "table"); Valor: lista de números 1-based em ordem de aparecimento.
    /// Populado pela introspecção; lido pelo layouter via índice de progresso.
    pub figure_numbers: HashMap<String, Vec<usize>>,
    /// Mapa de label → número da figura (Passo 75, DEBT-14).
    /// Populado pela introspecção quando `Content::Labelled` envolve uma figura numerada.
    pub figure_label_numbers: HashMap<Label, usize>,
    /// Contadores locais por kind — auxiliar interno da introspecção (Passo 75).
    /// Não exposto ao layouter; apenas `figure_numbers` é consumido externamente.
    pub local_figure_counters: HashMap<String, usize>,
    /// Lang activo para resolução de supplements localizados (Passo 158B).
    /// `None` → fallback PT (paridade backwards compat com tests
    /// pré-existentes que esperam "Figura"). Caller pode setar
    /// `state.lang = Some(lang)` antes de passar a `layout()` para
    /// comportamento lang-aware. Refino futuro pode integrar Style
    /// chain lang resolution (NÃO reservado per política P158).
    pub lang: Option<Lang>,
}

impl CounterState {
    pub fn new() -> Self {
        Self::default()
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
        if self.is_readonly { return; }
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
        if self.is_readonly { return; }
        *self.flat.entry(key.to_string()).or_insert(0) += 1;
    }

    /// Força um contador plano para um valor específico.
    pub fn update_flat(&mut self, key: &str, value: usize) {
        if self.is_readonly { return; }
        self.flat.insert(key.to_string(), value);
    }

    /// Lê o valor actual de um contador plano.
    pub fn get_flat(&self, key: &str) -> usize {
        self.flat.get(key).copied().unwrap_or(0)
    }

    /// Converte o valor actual de um contador para texto (Passo 66, DEBT-18).
    ///
    /// Centraliza a lógica de leitura aqui para que `introspect.rs`
    /// e `layout/counters.rs` a possam usar sem criar dependências cruzadas.
    pub fn display_value(&self, kind: &str) -> String {
        if self.hierarchical.contains_key(kind) {
            self.format_hierarchical(kind).unwrap_or_else(|| "0".to_string())
        } else {
            self.get_flat(kind).to_string()
        }
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

    // ── Testes de read-only do Passo 63 (DEBT-13) ───────────────────────

    #[test]
    fn counter_state_readonly_bloqueia_step_flat() {
        let mut s = CounterState::new();
        s.is_readonly = true;
        s.step_flat("equation");
        assert_eq!(s.get_flat("equation"), 0,
            "step_flat não deve avançar em modo read-only");
    }

    #[test]
    fn counter_state_readonly_permite_leitura() {
        let mut s = CounterState::new();
        s.step_flat("equation");  // avança antes de activar read-only
        s.is_readonly = true;
        assert_eq!(s.get_flat("equation"), 1,
            "get_flat deve funcionar mesmo em modo read-only");
    }

    #[test]
    fn counter_state_readonly_bloqueia_step_hierarchical() {
        let mut s = CounterState::new();
        s.is_readonly = true;
        s.step_hierarchical("heading", 1);
        assert_eq!(s.format_hierarchical("heading"), None,
            "step_hierarchical não deve avançar em modo read-only");
    }

    #[test]
    fn counter_state_readonly_bloqueia_update_flat() {
        let mut s = CounterState::new();
        s.step_flat("figure");   // valor = 1
        s.is_readonly = true;
        s.update_flat("figure", 99);
        assert_eq!(s.get_flat("figure"), 1,
            "update_flat não deve mudar valor em modo read-only");
    }

    #[test]
    fn counter_state_readonly_desactivado_apos_restauro() {
        let mut s = CounterState::new();
        s.is_readonly = true;
        s.step_flat("equation");
        assert_eq!(s.get_flat("equation"), 0);
        s.is_readonly = false;
        s.step_flat("equation");
        assert_eq!(s.get_flat("equation"), 1,
            "step_flat deve avançar após desactivar read-only");
    }
}
