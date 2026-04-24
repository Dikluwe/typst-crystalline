//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/style_chain.md
//! @prompt-hash 167b28fd
//! @layer L1
//! @updated 2026-04-23
//!
//! Passo 99 (ADR-0038): `StyleChain` ganhou métodos para aceitar uma
//! colecção `Styles` (enum `Style` do módulo `style`). A representação
//! interna (`StyleDelta` com bold/italic/size) permanece como backing
//! dos accessors existentes — a coexistência é intencional até que
//! o pipeline `#set`/`#show` e o Layouter migrem para `StyleChain`
//! directamente (DEBT sucessor registado em 99.E).

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::layout_types::{Pt, TextStyle};
use crate::entities::style::{Style, Styles};

/// Um delta de estilo — apenas as propriedades que este nó define explicitamente.
/// Propriedades ausentes são herdadas do nó pai na cadeia.
///
/// Passo 99 (ADR-0038): adicionados `fill` e `heading_level` para suportar
/// as variantes forward-compat do enum `Style`. Os accessors antigos
/// (`bold()`, `italic()`, `size()`) continuam a ignorar estes campos.
/// Passo 126 (ADR-0038 anotada): `weight` adicionado como primeira
/// propriedade numérica; capturado por `#set text(weight: N)` mas
/// ainda não consumido por layout (inerte).
#[derive(Debug, Clone, PartialEq)]
pub struct StyleDelta {
    pub bold:   Option<bool>,
    pub italic: Option<bool>,
    pub size:   Option<f64>,   // em pontos tipográficos
    /// Cor de preenchimento do texto (Passo 99, ADR-0038, forward-compat).
    pub fill:   Option<crate::entities::layout_types::Color>,
    /// Nível de heading quando aplicado via `#set heading(level: N)` futuro
    /// (Passo 99, ADR-0038, forward-compat).
    pub heading_level: Option<u8>,
    /// Peso da fonte (Passo 126, ADR-0038). Valor raw `u16` (CSS/OpenType
    /// 0-1000). Capturado pelo eval mas ainda inerte em layout.
    pub weight: Option<u16>,
    /// Espaçamento adicional entre glyphs (Passo 127, ADR-0038). Preserva
    /// `Length` inteiro (`abs + em`); resolve para pt quando consumer
    /// conhecer font-size. Inerte em layout.
    pub tracking: Option<crate::entities::layout_types::Length>,
    /// Espaço entre linhas (Passo 128, DEBT-1 subset). Em vanilla é
    /// propriedade de `par` (não de `text`); capturado em `#set text`
    /// por conveniência temporária — migra para `eval_set_par` quando
    /// este for activado. Inerte em layout.
    pub leading: Option<crate::entities::layout_types::Length>,
    /// Código de língua (Passo 130, DEBT-1 subset). BCP 47 como raw
    /// string — vanilla usa tipo `Lang` com validação; cristalino
    /// captura sem validar, consumer futuro normaliza. Inerte em layout.
    pub lang: Option<EcoString>,
}

impl StyleDelta {
    pub const fn empty() -> Self {
        Self {
            bold: None, italic: None, size: None,
            fill: None, heading_level: None,
            weight: None, tracking: None, leading: None,
            lang: None,
        }
    }
}

/// Nó interno da lista ligada.
#[derive(Debug, Clone)]
struct StyleNode {
    delta:  StyleDelta,
    parent: Option<Arc<StyleNode>>,
}

/// Lista ligada imutável de deltas de estilo.
///
/// Clone é O(1) — apenas o `Arc` do nó de topo é clonado.
/// Leitura é O(N) percorrendo a cadeia até encontrar o primeiro delta
/// que define a propriedade (N tipicamente < 10).
///
/// Equivalente simplificado de `StyleChain` do Typst original.
/// Suporta `#set text(bold: true)` e herança em blocos aninhados.
///
/// Pureza L1: não usa I/O de sistema — apenas `Arc` e `Vec` em memória.
#[derive(Debug, Clone)]
pub struct StyleChain(Option<Arc<StyleNode>>);

impl StyleChain {
    /// Cadeia vazia — resolve para os defaults codificados em cada accessor.
    pub const fn empty() -> Self {
        StyleChain(None)
    }

    /// Cadeia com os valores por defeito do motor Typst.
    /// bold: false, italic: false, size: 11.0pt
    pub fn default_chain() -> Self {
        let root = StyleNode {
            delta: StyleDelta {
                bold:   Some(false),
                italic: Some(false),
                size:   Some(11.0),
                ..StyleDelta::empty()
            },
            parent: None,
        };
        StyleChain(Some(Arc::new(root)))
    }

    /// Cria uma nova cadeia que herda desta e aplica `delta` por cima.
    /// Custo: O(1) — cria um novo `Arc`.
    pub fn push(&self, delta: StyleDelta) -> Self {
        let node = StyleNode {
            delta,
            parent: self.0.clone(),
        };
        StyleChain(Some(Arc::new(node)))
    }

    /// Cria uma nova cadeia aplicando `styles` (colecção de `Style`) como
    /// delta. Passo 99 (ADR-0038) — entrada tipada para `Content::Styled`.
    ///
    /// Cada variante do enum `Style` é projectada no campo correspondente
    /// de `StyleDelta`. O `match` é exaustivo — adicionar uma nova variante
    /// ao enum `Style` obriga a tratá-la aqui.
    pub fn push_styles(&self, styles: &Styles) -> Self {
        let mut delta = StyleDelta::empty();
        for style in styles.iter() {
            match style {
                Style::Bold(b)         => delta.bold = Some(*b),
                Style::Italic(i)       => delta.italic = Some(*i),
                Style::Size(pt)        => delta.size = Some(pt.val()),
                Style::Fill(c)         => delta.fill = Some(*c),
                Style::HeadingLevel(l) => delta.heading_level = Some(*l),
            }
        }
        self.push(delta)
    }

    /// Resolve `fill` (cor de texto) percorrendo a cadeia até ao primeiro
    /// delta que o define. Forward-compat — ainda não consumido pelo
    /// Layouter/export (ver DEBT-Style sucessor em DEBT.md).
    pub fn fill(&self) -> Option<crate::entities::layout_types::Color> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.fill {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `heading_level` percorrendo a cadeia até ao primeiro delta
    /// que o define. Forward-compat — o AST ainda representa headings via
    /// `Content::Heading{level, ..}` directamente.
    pub fn heading_level(&self) -> Option<u8> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = n.delta.heading_level {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    /// Resolve `bold` percorrendo a cadeia até ao primeiro delta que o define.
    pub fn bold(&self) -> bool {
        self.resolve_bool(|d| d.bold).unwrap_or(false)
    }

    /// Resolve `italic`.
    pub fn italic(&self) -> bool {
        self.resolve_bool(|d| d.italic).unwrap_or(false)
    }

    /// Resolve `size` em pontos tipográficos.
    pub fn size(&self) -> f64 {
        self.resolve_f64(|d| d.size).unwrap_or(11.0)
    }

    fn resolve_bool(&self, f: impl Fn(&StyleDelta) -> Option<bool>) -> Option<bool> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = f(&n.delta) {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }

    fn resolve_f64(&self, f: impl Fn(&StyleDelta) -> Option<f64>) -> Option<f64> {
        let mut node = self.0.as_deref();
        while let Some(n) = node {
            if let Some(v) = f(&n.delta) {
                return Some(v);
            }
            node = n.parent.as_deref();
        }
        None
    }
}

/// Conversão para `TextStyle` plano — **ponto único de resolução**
/// (ADR-0039, Passo 100). O Layouter mantém `StyleChain` como
/// source-of-truth; `From<&StyleChain>` achata em `TextStyle` quando
/// emite um `FrameItem::Text`.
impl From<&StyleChain> for TextStyle {
    fn from(chain: &StyleChain) -> Self {
        TextStyle {
            bold:          chain.bold(),
            italic:        chain.italic(),
            size:          Pt(chain.size()),
            fill:          chain.fill(),
            heading_level: chain.heading_level(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_chain_defaults() {
        let chain = StyleChain::default_chain();
        assert!(!chain.bold());
        assert!(!chain.italic());
        assert_eq!(chain.size(),   11.0);
    }

    #[test]
    fn style_chain_push_herda() {
        let base  = StyleChain::default_chain();
        let child = base.push(StyleDelta { bold: Some(true), italic: None, size: None , ..StyleDelta::empty() });
        assert!(child.bold());
        assert!(!child.italic());  // herdado
        assert_eq!(child.size(),   11.0);   // herdado
    }

    #[test]
    fn style_chain_push_multiplos_niveis() {
        let base  = StyleChain::default_chain();
        let mid   = base.push(StyleDelta { bold: Some(true), italic: None, size: None , ..StyleDelta::empty() });
        let child = mid.push(StyleDelta { bold: None, italic: None, size: Some(14.0) , ..StyleDelta::empty() });
        // bold herdado de mid, size de child, italic do root
        assert!(child.bold());
        assert!(!child.italic());
        assert_eq!(child.size(),   14.0);
    }

    #[test]
    fn style_chain_clone_e_o1() {
        let base  = StyleChain::default_chain();
        let chain = base.push(StyleDelta { bold: Some(true), italic: None, size: None , ..StyleDelta::empty() });
        let clone = chain.clone();
        assert!(clone.bold());
        // Clone correcto — mesmo que O(1) não seja verificável directamente
    }

    #[test]
    fn text_style_from_style_chain() {
        let chain = StyleChain::default_chain()
            .push(StyleDelta { bold: Some(true), italic: None, size: Some(14.0) , ..StyleDelta::empty() });
        let ts = TextStyle::from(&chain);
        assert!(ts.bold);
        assert!(!ts.italic);
        assert_eq!(ts.size.val(),   14.0);
    }

    #[test]
    fn empty_chain_usa_defaults() {
        let chain = StyleChain::empty();
        assert!(!chain.bold());
        assert!(!chain.italic());
        assert_eq!(chain.size(),   11.0);
    }

    // ── Passo 99 (ADR-0038): Styles/Style integração ──────────────────────

    use crate::entities::layout_types::Color;

    #[test]
    fn push_styles_projecta_bold_italic_size() {
        let base = StyleChain::default_chain();
        let styles = Styles::from_iter([
            Style::Bold(true),
            Style::Italic(true),
            Style::Size(Pt(18.0)),
        ]);
        let child = base.push_styles(&styles);
        assert!(child.bold());
        assert!(child.italic());
        assert_eq!(child.size(), 18.0);
    }

    #[test]
    fn push_styles_herda_propriedade_nao_definida() {
        let base = StyleChain::default_chain();
        // Só define bold — italic e size devem cair no default.
        let styles = Styles::from_iter([Style::Bold(true)]);
        let child = base.push_styles(&styles);
        assert!(child.bold());
        assert!(!child.italic());
        assert_eq!(child.size(), 11.0);
    }

    #[test]
    fn push_styles_topo_ganha_sobre_base() {
        let base = StyleChain::default_chain()
            .push_styles(&Styles::from_iter([Style::Bold(true)]));
        let child = base.push_styles(&Styles::from_iter([Style::Bold(false)]));
        assert!(!child.bold(), "o delta mais próximo do texto ganha");
    }

    #[test]
    fn fill_forward_compat() {
        let base = StyleChain::default_chain();
        assert_eq!(base.fill(), None, "sem Fill, None");
        let red = Color::rgb(255, 0, 0);
        let child = base.push_styles(&Styles::from_iter([Style::Fill(red)]));
        assert_eq!(child.fill(), Some(red));
    }

    #[test]
    fn heading_level_forward_compat() {
        let base = StyleChain::default_chain();
        assert_eq!(base.heading_level(), None);
        let child = base.push_styles(&Styles::from_iter([Style::HeadingLevel(3)]));
        assert_eq!(child.heading_level(), Some(3));
    }

    #[test]
    fn chain_aninhada_fill_heading_level_top_wins() {
        let base = StyleChain::default_chain()
            .push_styles(&Styles::from_iter([
                Style::Fill(Color::rgb(0, 0, 255)),
                Style::HeadingLevel(1),
            ]));
        let child = base.push_styles(&Styles::from_iter([
            Style::Fill(Color::rgb(255, 0, 0)),
        ]));
        // Fill: o topo define — ganha.
        assert_eq!(child.fill(), Some(Color::rgb(255, 0, 0)));
        // HeadingLevel: o topo não define — herda do pai.
        assert_eq!(child.heading_level(), Some(1));
    }

    // ── Passo 99.D: Teste de integração conceptual ───────────────────────

    use crate::entities::content::Content;

    /// Integração: um `Content::Styled` constrói-se com `Styles`; a
    /// resolução via `StyleChain` devolve os mesmos valores que foram
    /// aplicados como delta. Isto valida que a fundação é usável sem
    /// activar `#set` no eval (Passo 99, ADR-0038).
    #[test]
    fn integracao_content_styled_resolve_via_style_chain() {
        let body   = Content::text("hello");
        let styles = Styles::from_iter([
            Style::Bold(true),
            Style::Size(Pt(18.0)),
        ]);
        let styled = Content::Styled(Box::new(body), styles);

        // O consumidor futuro (`eval_markup`) faria:
        //   1. ler os estilos do Content::Styled;
        //   2. push na StyleChain;
        //   3. passar a StyleChain à avaliação do body.
        // Aqui simulamos o passo 2+3 manualmente.
        let chain = match &styled {
            Content::Styled(_body, ss) =>
                StyleChain::default_chain().push_styles(ss),
            _ => panic!("esperado Content::Styled"),
        };

        assert!(chain.bold());
        assert!(!chain.italic());  // default
        assert_eq!(chain.size(), 18.0);
    }

    /// Integração: `Styled` aninhado — o delta mais próximo do texto ganha
    /// (top-wins), consistente com o vanilla (ADR-0033).
    #[test]
    fn integracao_styled_aninhado_top_wins() {
        let inner_body = Content::text("hi");
        let inner = Content::Styled(
            Box::new(inner_body),
            Styles::from_iter([Style::Italic(true)]),
        );
        let outer = Content::Styled(
            Box::new(inner),
            Styles::from_iter([Style::Bold(true), Style::Italic(false)]),
        );

        // Simular o caminho que o eval tomaria: outer primeiro (mais
        // longe do texto), depois inner (mais perto).
        let chain = match &outer {
            Content::Styled(body, ss_outer) => {
                let chain_outer = StyleChain::default_chain().push_styles(ss_outer);
                match body.as_ref() {
                    Content::Styled(_, ss_inner) => chain_outer.push_styles(ss_inner),
                    _ => panic!("esperado Content::Styled aninhado"),
                }
            }
            _ => panic!("esperado Content::Styled"),
        };

        // bold=true (só outer define).
        assert!(chain.bold());
        // italic=true — inner está mais perto do texto e define Italic(true),
        // sobrepondo o Italic(false) do outer. Top-wins (paridade vanilla).
        assert!(chain.italic());
    }
}
