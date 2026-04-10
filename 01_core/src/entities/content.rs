//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content.md
//! @prompt-hash cbe9996f
//! @layer L1
//! @updated 2026-04-03

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::layout_types::{Pt, TextStyle};

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
    /// Texto simples com estilo capturado em eval (Passo 30).
    /// O estilo reflecte as `#set text()` rules activas no momento da produção.
    Text(EcoString, TextStyle),
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

    // ── Matemática (Passo 34) ────────────────────────────────────────────────
    /// Equação matemática (`$...$` inline, `$ ... $` block).
    /// `block: true` → equação em linha própria (display mode).
    /// O motor de equações (Passo 36+) processa `body`.
    Equation {
        body:  Box<Content>,
        block: bool,
    },

    /// Sequência de nós matemáticos — corpo interno de uma equação.
    MathSequence(Arc<[Content]>),

    /// Identificador matemático: variável, função, símbolo (`x`, `sin`, `alpha`).
    MathIdent(EcoString),

    /// Texto literal em modo matemático (`"texto"` dentro de `$...$`).
    MathText(EcoString),

    /// Fracção matemática (`a/b` ou `frac(a, b)`).
    MathFrac {
        num: Box<Content>,
        den: Box<Content>,
    },

    /// Base com índice e/ou expoente (`x_1^2`).
    /// `sub` = índice (subscript), `sup` = expoente (superscript).
    MathAttach {
        base: Box<Content>,
        sub:  Option<Box<Content>>,
        sup:  Option<Box<Content>>,
    },

    /// Raiz matemática (`√x`, `∛x`, `∜x`).
    /// `index`: None = raiz quadrada, Some(n) = raiz n-ésima.
    MathRoot {
        index:    Option<Box<Content>>,
        radicand: Box<Content>,
    },

    /// Expressão entre delimitadores (`(...)`, `[...]`, `{...}`).
    /// `open`/`close` são os caracteres delimitadores.
    /// Mantida como variante própria para que o layout possa
    /// seleccionar variantes de tamanho (Passo 42).
    MathDelimited {
        open:  char,
        body:  Box<Content>,
        close: char,
    },

    // Variantes futuras — NÃO implementar sem ADR:
    // Styled(Box<Content>, StyleChain),          // requer StyleChain — Passo 30+
    // Elem(Arc<dyn NativeElement>),               // vtable — Passo 20+
}

impl Content {
    /// Cria conteúdo de texto com estilo por defeito (regular 11pt).
    /// Em eval, usar `Content::Text(s, TextStyle::from(&ctx.styles))` directamente
    /// para capturar o estilo activo no momento da produção.
    pub fn text(s: impl Into<EcoString>) -> Self {
        Self::Text(s.into(), TextStyle::regular(Pt(11.0)))
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
            Self::Empty                 => String::new(),
            Self::Text(s, _)            => s.to_string(),
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
            Self::Equation { body, block } => {
                if *block { format!("\n{}\n", body.plain_text()) }
                else       { body.plain_text() }
            }
            Self::MathSequence(nodes) => nodes.iter().map(|n| n.plain_text()).collect(),
            Self::MathIdent(s)        => s.to_string(),
            Self::MathText(s)         => s.to_string(),
            Self::MathFrac { num, den } => {
                format!("({})/({})", num.plain_text(), den.plain_text())
            }
            Self::MathAttach { base, sub, sup } => {
                let mut s = base.plain_text();
                if let Some(sub) = sub { s.push_str(&format!("_{}", sub.plain_text())); }
                if let Some(sup) = sup { s.push_str(&format!("^{}", sup.plain_text())); }
                s
            }
            Self::MathRoot { index, radicand } => match index {
                None    => format!("sqrt({})", radicand.plain_text()),
                Some(i) => format!("root({}, {})", i.plain_text(), radicand.plain_text()),
            },
            Self::MathDelimited { open, body, close } => {
                format!("{}{}{}", open, body.plain_text(), close)
            }
        }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty,                Self::Empty)                => true,
            (Self::Text(a, sa),          Self::Text(b, sb))          => a == b && sa == sb,
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
            (Self::Equation { body: ba, block: ka },
             Self::Equation { body: bb, block: kb })                 => ba == bb && ka == kb,
            (Self::MathSequence(a), Self::MathSequence(b))           => a.as_ref() == b.as_ref(),
            (Self::MathIdent(a),    Self::MathIdent(b))              => a == b,
            (Self::MathText(a),     Self::MathText(b))               => a == b,
            (Self::MathFrac { num: na, den: da },
             Self::MathFrac { num: nb, den: db })                    => na == nb && da == db,
            (Self::MathAttach { base: ba, sub: sa, sup: pa },
             Self::MathAttach { base: bb, sub: sb, sup: pb })        => ba == bb && sa == sb && pa == pb,
            (Self::MathRoot { index: ia, radicand: ra },
             Self::MathRoot { index: ib, radicand: rb })             => ia == ib && ra == rb,
            (Self::MathDelimited { open: oa, body: ba, close: ca },
             Self::MathDelimited { open: ob, body: bb, close: cb })  => oa == ob && ba == bb && ca == cb,
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

    // ── Passo 34 — variantes matemáticas ─────────────────────────────────────

    #[test]
    fn content_equation_inline_plain_text() {
        let eq = Content::Equation {
            body:  Box::new(Content::MathIdent("x".into())),
            block: false,
        };
        assert_eq!(eq.plain_text(), "x");
    }

    #[test]
    fn content_equation_block_plain_text() {
        let eq = Content::Equation {
            body:  Box::new(Content::MathIdent("x".into())),
            block: true,
        };
        assert_eq!(eq.plain_text(), "\nx\n");
    }

    #[test]
    fn content_math_frac_plain_text() {
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        assert_eq!(frac.plain_text(), "(a)/(b)");
    }

    #[test]
    fn content_math_attach_plain_text() {
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            sub:  None,
            sup:  Some(Box::new(Content::MathText("2".into()))),
        };
        assert_eq!(attach.plain_text(), "x^2");
    }

    #[test]
    fn content_math_root_quadrada() {
        let root = Content::MathRoot {
            index:    None,
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        assert_eq!(root.plain_text(), "sqrt(x)");
    }

    #[test]
    fn content_math_root_cubica() {
        let root = Content::MathRoot {
            index:    Some(Box::new(Content::MathText("3".into()))),
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        assert_eq!(root.plain_text(), "root(3, x)");
    }

    #[test]
    fn content_math_sequence_plain_text() {
        let seq = Content::MathSequence(Arc::from(vec![
            Content::MathIdent("x".into()),
            Content::MathText("+".into()),
            Content::MathIdent("y".into()),
        ].into_boxed_slice()));
        assert_eq!(seq.plain_text(), "x+y");
    }

    #[test]
    fn content_math_partialeq() {
        let a = Content::MathIdent("x".into());
        let b = Content::MathIdent("x".into());
        let c = Content::MathIdent("y".into());
        assert_eq!(a, b);
        assert_ne!(a, c);
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
