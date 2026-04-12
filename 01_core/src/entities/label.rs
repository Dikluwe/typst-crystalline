//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/label.md
//! @prompt-hash 467b0548
//! @layer L1
//! @updated 2026-04-12

/// Etiqueta de conteúdo — identificador semântico atribuído a um nó.
///
/// Produzida pela sintaxe `<nome>` em Typst (ex: `= Introdução <intro>`).
/// Usada pelo motor de introspecção para resolver referências cruzadas.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_eq_por_conteudo() {
        let a = Label("intro".to_string());
        let b = Label("intro".to_string());
        assert_eq!(a, b);
    }

    #[test]
    fn label_neq_nomes_distintos() {
        let a = Label("intro".to_string());
        let b = Label("outro".to_string());
        assert_ne!(a, b);
    }
}
