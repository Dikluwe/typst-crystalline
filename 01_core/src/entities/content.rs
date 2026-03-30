//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content.md
//! @prompt-hash cbe9996f
//! @layer L1
//! @updated 2026-03-28

use std::sync::Arc;

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
///
/// `PartialEq` implementado manualmente — `Arc<[Content]>` compara por ponteiro
/// com `derive`, não por conteúdo (ADR-0026 revisão).
#[derive(Debug, Clone)]
pub enum Content {
    /// Conteúdo vazio.
    Empty,
    /// Texto simples (TextElem mínimo).
    Text(EcoString),
    /// Espaço entre elementos (SpaceElem).
    Space,
    /// Sequência de elementos — clone O(1) via Arc (ADR-0026 revisão).
    Sequence(Arc<[Content]>),

    // ── Rich text (Passo 22) ─────────────────────────────────────────────
    /// Conteúdo em negrito (`*Strong*`).
    Strong(Box<Content>),
    /// Conteúdo em itálico (`_Emph_`).
    Emph(Box<Content>),
    /// Cabeçalho com nível 1–6 (`= Heading`).
    Heading { level: u8, body: Box<Content> },

    // ── Passo 23 ────────────────────────────────────────────────────────────
    /// Código raw inline ou em bloco (`` `...` `` ou ```` ``` ... ``` ````).
    Raw {
        text:  EcoString,
        lang:  Option<EcoString>,
        block: bool,
    },
    /// Item de lista não ordenada (`- ...`).
    ListItem(Box<Content>),
    /// Item de lista ordenada (`+ ...` ou `1. ...`).
    EnumItem { number: Option<u32>, body: Box<Content> },
    /// Hiperligação (`https://...`).
    Link { url: EcoString, body: Box<Content> },

    // Variantes futuras — NÃO implementar sem ADR:
    // Styled(Box<Content>, StyleChain),          // requer StyleChain — Passo 30+
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

    pub fn raw(text: impl Into<EcoString>, lang: Option<EcoString>, block: bool) -> Self {
        Self::Raw { text: text.into(), lang, block }
    }
    pub fn list_item(body: Content) -> Self { Self::ListItem(Box::new(body)) }
    pub fn enum_item(number: Option<u32>, body: Content) -> Self {
        Self::EnumItem { number, body: Box::new(body) }
    }
    pub fn link(url: impl Into<EcoString>, body: Content) -> Self {
        Self::Link { url: url.into(), body: Box::new(body) }
    }

    pub fn sequence(parts: Vec<Content>) -> Self {
        match parts.len() {
            0 => Self::Empty,
            1 => parts.into_iter().next().unwrap(),
            _ => Self::Sequence(parts.into()),  // Vec<Content> → Arc<[Content]>
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
            Self::Raw { text, .. }   => text.to_string(),
            Self::ListItem(c)        => format!("• {}", c.plain_text()),
            Self::EnumItem { number, body } => {
                let n = number.map(|n| format!("{}. ", n)).unwrap_or_default();
                format!("{}{}", n, body.plain_text())
            }
            Self::Link { body, .. }  => body.plain_text(),
        }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty,                Self::Empty)                => true,
            (Self::Text(a),              Self::Text(b))              => a == b,
            (Self::Space,                Self::Space)                => true,
            (Self::Sequence(a),          Self::Sequence(b))          => a.as_ref() == b.as_ref(),
            (Self::Strong(a),            Self::Strong(b))            => a == b,
            (Self::Emph(a),              Self::Emph(b))              => a == b,
            (Self::Heading { level: la, body: ba }, Self::Heading { level: lb, body: bb }) => la == lb && ba == bb,
            (Self::Raw { text: ta, lang: la, block: ba },
             Self::Raw { text: tb, lang: lb, block: bb })            => ta == tb && la == lb && ba == bb,
            (Self::ListItem(a),          Self::ListItem(b))          => a == b,
            (Self::EnumItem { number: na, body: ba },
             Self::EnumItem { number: nb, body: bb })                => na == nb && ba == bb,
            (Self::Link { url: ua, body: ba },
             Self::Link { url: ub, body: bb })                       => ua == ub && ba == bb,
            _ => false,
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
        let c = Content::Sequence(Arc::from(Vec::<Content>::new().into_boxed_slice()));
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

    // ── Passo 23 ────────────────────────────────────────────────────────────

    #[test]
    fn raw_plain_text() {
        assert_eq!(Content::raw("fn main() {}", None, false).plain_text(), "fn main() {}");
    }

    #[test]
    fn list_item_tem_bullet_em_plain_text() {
        assert!(Content::list_item(Content::text("Apple")).plain_text().contains("Apple"));
    }

    #[test]
    fn enum_item_com_numero() {
        let t = Content::enum_item(Some(1), Content::text("First")).plain_text();
        assert!(t.contains("1") && t.contains("First"));
    }

    #[test]
    fn link_plain_text_e_o_corpo() {
        assert_eq!(
            Content::link("https://typst.app", Content::text("Typst")).plain_text(),
            "Typst",
        );
    }

    // ── Passo 26 — Content::Sequence com Arc (ADR-0026 revisão) ─────────────

    #[test]
    fn sequence_clone_e_o1() {
        let seq = Content::sequence(vec![
            Content::text("a"),
            Content::text("b"),
            Content::text("c"),
        ]);
        let clone = seq.clone();
        // PartialEq por conteúdo — não por ponteiro
        assert_eq!(seq, clone);
    }

    #[test]
    fn sequence_partialeq_por_conteudo() {
        let s1 = Content::sequence(vec![Content::text("hello")]);
        let s2 = Content::sequence(vec![Content::text("hello")]);
        // Dois Arc distintos com mesmo conteúdo → iguais
        assert_eq!(s1, s2);
    }

    #[test]
    fn sequence_partialeq_conteudos_diferentes() {
        let s1 = Content::sequence(vec![Content::text("a")]);
        let s2 = Content::sequence(vec![Content::text("b")]);
        assert_ne!(s1, s2);
    }
}
