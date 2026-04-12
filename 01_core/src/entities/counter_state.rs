//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_state.md
//! @prompt-hash d8ae28a6
//! @layer L1
//! @updated 2026-04-12

/// Estado de contadores que viaja com o Layouter durante uma passagem.
///
/// Cristalino diverge do Typst original aqui: o original resolve contadores
/// em duas passagens com `comemo` (para suportar referências para a frente).
/// Esta implementação usa uma única passagem — suficiente para numeração
/// sequencial de headings.
///
/// DEBT-10: Resolver contadores em duas passagens com estado global quando
/// o motor de introspecção completo for implementado (Passos 60+).
#[derive(Debug, Clone, Default)]
pub struct CounterState {
    /// Níveis activos de heading. `[1, 2]` representa a secção 1.2.
    heading: Vec<usize>,
    /// Se a numeração de headings está activa.
    /// Activada por `#set heading(numbering: "1.1")` ou equivalente.
    pub heading_numbering: bool,
}

impl CounterState {
    pub fn new() -> Self { Self::default() }

    /// Avança o contador para o nível indicado.
    ///
    /// - Se `level` for maior que o comprimento actual: preenche com zeros
    ///   até `level - 1` e adiciona 1.
    /// - Se `level` for igual ao comprimento: incrementa o último elemento.
    /// - Se `level` for menor que o comprimento: trunca e incrementa.
    ///
    /// Exemplos:
    /// - `[]` + level 1 → `[1]`
    /// - `[1]` + level 2 → `[1, 1]`
    /// - `[1, 1]` + level 1 → `[2]`
    /// - `[1, 2]` + level 2 → `[1, 3]`
    pub fn step_heading(&mut self, level: usize) {
        let level = level.max(1);
        self.heading.truncate(level);
        if self.heading.len() < level {
            self.heading.resize(level - 1, 0);
            self.heading.push(1);
        } else {
            // len() == level após truncate
            if let Some(last) = self.heading.last_mut() {
                *last += 1;
            }
        }
    }

    /// Retorna a string formatada do nível actual.
    /// Retorna `None` se o vector estiver vazio.
    ///
    /// Exemplos: `[1]` → `"1"`, `[1, 2]` → `"1.2"`.
    pub fn format_heading(&self) -> Option<String> {
        if self.heading.is_empty() {
            None
        } else {
            Some(self.heading.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_heading_nivel_1_inicial() {
        let mut s = CounterState::new();
        s.step_heading(1);
        assert_eq!(s.format_heading(), Some("1".to_string()));
    }

    #[test]
    fn step_heading_dois_niveis() {
        let mut s = CounterState::new();
        s.step_heading(1);
        s.step_heading(2);
        assert_eq!(s.format_heading(), Some("1.1".to_string()));
    }

    #[test]
    fn step_heading_nivel_2_apos_nivel_2() {
        let mut s = CounterState::new();
        s.step_heading(1);
        s.step_heading(2);
        s.step_heading(2);
        assert_eq!(s.format_heading(), Some("1.2".to_string()));
    }

    #[test]
    fn step_heading_volta_ao_nivel_1() {
        let mut s = CounterState::new();
        s.step_heading(1);
        s.step_heading(2);
        s.step_heading(1);
        assert_eq!(s.format_heading(), Some("2".to_string()));
    }

    #[test]
    fn step_heading_tres_niveis_sequencia_completa() {
        let mut s = CounterState::new();
        s.step_heading(1); // [1]
        s.step_heading(2); // [1, 1]
        s.step_heading(3); // [1, 1, 1]
        s.step_heading(2); // [1, 2]
        s.step_heading(1); // [2]
        assert_eq!(s.format_heading(), Some("2".to_string()));
    }

    #[test]
    fn format_heading_vazio_retorna_none() {
        let s = CounterState::new();
        assert_eq!(s.format_heading(), None);
    }
}
