//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content.md
//! @prompt-hash cbe9996f
//! @layer L1
//! @updated 2026-03-28

use ecow::EcoString;

/// Conteúdo declarativo produzido por `eval()`.
///
/// Diverge intencionalmente do original (`typst-library/foundations/content/`),
/// que usa vtable (`unsafe trait NativeElement`), proc macros e Arc manual.
/// Replicar essa metaprogramação em L1 seria arquitecturalmente inferior.
/// Enum linear com variantes declarativas — mais simples e testável.
///
/// **Invariante L1**: não desenha, não mede, não renderiza.
/// Qualquer operação que precise de métricas de fonte ou I/O pertence a L3.
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    /// Conteúdo vazio.
    Empty,
    /// Texto simples (TextElem mínimo).
    Text(EcoString),
    /// Espaço entre elementos (SpaceElem).
    Space,
    /// Sequência de elementos.
    Sequence(Vec<Content>),

    // ── Rich text (Passo 22) ─────────────────────────────────────────────
    /// Conteúdo em negrito (`*Strong*`).
    Strong(Box<Content>),
    /// Conteúdo em itálico (`_Emph_`).
    Emph(Box<Content>),
    /// Cabeçalho com nível 1–6 (`= Heading`).
    Heading { level: u8, body: Box<Content> },

    // Variantes futuras — NÃO implementar sem ADR:
    // Styled(Box<Content>, StyleChain),          // requer StyleChain — Passo 30+
    // Raw { text: EcoString, lang: Option<EcoString> },
    // Link { url: EcoString, body: Box<Content> },
    // Elem(Arc<dyn NativeElement>),               // vtable — Passo 20+
}

impl Content {
    /// Cria conteúdo de texto.
    pub fn text(s: impl Into<EcoString>) -> Self {
        Self::Text(s.into())
    }

    /// Cria conteúdo vazio.
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Cria uma sequência, normalizando casos degenerados.
    ///
    /// - 0 partes → `Empty`
    /// - 1 parte → desembrulha (evita `Sequence([x])`)
    /// - n > 1 → `Sequence(parts)`
    pub fn strong(body: Content) -> Self { Self::Strong(Box::new(body)) }
    pub fn emph(body: Content)   -> Self { Self::Emph(Box::new(body)) }
    pub fn heading(level: u8, body: Content) -> Self {
        Self::Heading { level: level.clamp(1, 6), body: Box::new(body) }
    }

    pub fn sequence(parts: Vec<Content>) -> Self {
        match parts.len() {
            0 => Self::Empty,
            1 => parts.into_iter().next().unwrap(),
            _ => Self::Sequence(parts),
        }
    }

    /// Retorna `true` se este conteúdo não contém informação visível.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Sequence(v) => v.is_empty(),
            _ => false,
        }
    }

    /// Extrai texto plano recursivamente — para verificação em testes.
    pub fn plain_text(&self) -> String {
        match self {
            Self::Empty              => String::new(),
            Self::Text(s)            => s.to_string(),
            Self::Space              => " ".to_string(),
            Self::Sequence(v)        => v.iter().map(|c| c.plain_text()).collect(),
            Self::Strong(c)          => c.plain_text(),
            Self::Emph(c)            => c.plain_text(),
            Self::Heading { body, .. } => body.plain_text(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_plain_text() {
        assert_eq!(Content::text("hello").plain_text(), "hello");
        assert_eq!(Content::text("").plain_text(), "");
    }

    #[test]
    fn empty_is_empty() {
        assert!(Content::empty().is_empty());
        assert_eq!(Content::empty().plain_text(), "");
    }

    #[test]
    fn space_nao_e_empty() {
        assert!(!Content::Space.is_empty());
        assert_eq!(Content::Space.plain_text(), " ");
    }

    #[test]
    fn sequence_zero_partes_e_empty() {
        let c = Content::sequence(vec![]);
        assert!(c.is_empty());
        assert_eq!(c, Content::Empty);
    }

    #[test]
    fn sequence_uma_parte_desembrulha() {
        let c = Content::sequence(vec![Content::text("a")]);
        assert_eq!(c, Content::text("a"));
    }

    #[test]
    fn sequence_multiplas_partes() {
        let c = Content::sequence(vec![
            Content::text("a"),
            Content::Space,
            Content::text("b"),
        ]);
        assert_eq!(c.plain_text(), "a b");
        assert!(!c.is_empty());
    }

    #[test]
    fn sequence_is_empty_para_vec_vazio() {
        let c = Content::Sequence(vec![]);
        assert!(c.is_empty());
    }

    #[test]
    fn clone_e_partial_eq() {
        let c1 = Content::text("hello");
        let c2 = c1.clone();
        assert_eq!(c1, c2);
        assert_ne!(Content::text("a"), Content::text("b"));
        assert_ne!(Content::text("a"), Content::Space);
    }

    #[test]
    fn strong_plain_text_preservado() {
        assert_eq!(Content::strong(Content::text("bold")).plain_text(), "bold");
    }

    #[test]
    fn emph_plain_text_preservado() {
        assert_eq!(Content::emph(Content::text("em")).plain_text(), "em");
    }

    #[test]
    fn heading_level_clamped() {
        assert!(matches!(Content::heading(0, Content::Empty), Content::Heading { level: 1, .. }));
        assert!(matches!(Content::heading(9, Content::Empty), Content::Heading { level: 6, .. }));
        assert!(matches!(Content::heading(3, Content::Empty), Content::Heading { level: 3, .. }));
    }

    #[test]
    fn heading_plain_text() {
        let h = Content::heading(1, Content::text("Title"));
        assert_eq!(h.plain_text(), "Title");
    }

    #[test]
    fn nested_sequence_plain_text() {
        let inner = Content::sequence(vec![Content::text("x"), Content::text("y")]);
        let outer = Content::sequence(vec![inner, Content::Space, Content::text("z")]);
        assert_eq!(outer.plain_text(), "xy z");
    }
}
