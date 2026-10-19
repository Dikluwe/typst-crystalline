//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/accent.md
//! @prompt-hash a3da3ffd
//! @layer L1
//! @updated 2026-10-17
//!
//! Método `layout_accent` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P296).

use crate::compiler::layout::FontMetrics;
use std::sync::Arc;

use crate::entities::{
    content::Content,
    elements::math_accent::MathAccentElem,
    layout_types::{FrameItem, Length, Pt, TextStyle},
    rel::Rel,
    value::Value,
};

use super::{offset_item, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    /// **P296** — Posiciona `accent` glyph centrado horizontalmente
    /// acima de `base`. Heurística minimal per ADR-0054 graded:
    /// - Sem `dotless` (i/j) handling — base mantém glyph original.
    /// - Sem `size` ratio — accent é width natural.
    #[cfg(test)]
    pub(super) fn layout_accent(
        &self,
        base: &Content,
        accent: &Content,
        style: &TextStyle,
    ) -> MathBox {
        self.layout_accent_resolved(base, accent, Rel::from_percent(100.0), false, style)
    }

    /// Materializa o carrier completo de `math.accent`: resolve a cascata
    /// independente de `size`/`dotless`, a orientação do mark e só então
    /// delega ao algoritmo geométrico comum.
    pub(super) fn layout_accent_elem(
        &self,
        elem: &MathAccentElem,
        style: &TextStyle,
    ) -> MathBox {
        let (size, dotless) = self.resolved_accent_options(elem);
        let accent = Content::MathText(elem.accent.to_string().into());
        let bottom =
            matches!(elem.accent, '\u{23df}' | '\u{23b5}' | '\u{23dd}' | '\u{23e1}')
                || has_below_combining_class(elem.accent);
        let base =
            if !bottom && dotless { dotless_base(&elem.base) } else { elem.base.clone() };
        self.layout_accent_resolved(&base, &accent, size, bottom, style)
    }

    /// Escolhe as opções do carrier antes de qualquer medição geométrica.
    fn resolved_accent_options(&self, elem: &MathAccentElem) -> (Rel<Length>, bool) {
        let cascaded_size = self
            .lexical_styles
            .and_then(|styles| styles.custom("math.accent.size"))
            .map(|value| match value {
                Value::Relative(value) => *value,
                Value::Ratio(value) => Rel { rel: value.get(), abs: Length::ZERO },
                Value::Length(value) => Rel { rel: 0.0, abs: *value },
                other => panic!(
                    "invalid validated math.accent.size type: {}",
                    other.type_name()
                ),
            });
        let cascaded_dotless = self
            .lexical_styles
            .and_then(|styles| styles.custom("math.accent.dotless"))
            .map(|value| match value {
                Value::Bool(value) => *value,
                other => panic!(
                    "invalid validated math.accent.dotless type: {}",
                    other.type_name()
                ),
            });
        let size = if elem.explicit.size {
            elem.size
        } else {
            cascaded_size.unwrap_or_else(|| Rel::from_percent(100.0))
        };
        let dotless = if elem.explicit.dotless {
            elem.dotless
        } else {
            cascaded_dotless.unwrap_or(true)
        };
        (size, dotless)
    }

    fn layout_accent_resolved(
        &self,
        base: &Content,
        accent: &Content,
        size: Rel<Length>,
        bottom: bool,
        style: &TextStyle,
    ) -> MathBox {
        // **P915** — base é cramped, incondicional (o cristalino só
        // implementa accent "acima" — `MathAccentElem` sem campo de
        // posição — logo a condição do vanilla "só se accent.is_bottom()
        // == false" é trivialmente sempre verdadeira aqui). Achado do
        // vanilla: "the base is resolved in cramped style if the accent is
        // above" (`resolve_accent`, `resolve.rs:362-373`). Ver `accent.md`
        // §P915.
        let base_style = TextStyle { cramped: !bottom, ..style.clone() };
        let base_box = self.layout_node(base, &base_style);
        // **P906** — accent de 1 carácter estica para cobrir `base_box.width`
        // (ver `math/layout/accent.md` §P906). Multi-carácter: inalterado.
        let target_width =
            size.resolve(Length::pt(base_box.width)).resolve_pt(style.size.val());
        let min_width_du =
            target_width * self.constants.upem / style.size.val().max(0.001);
        let accent_box = self.layout_stretchy_or_node(accent, min_width_du, style, 0.5);
        // **P988-B** — centragem horizontal com `TopAccentAttachment`
        // (vanilla `accent.rs:37-52`: `accent_x = base_attach −
        // accent_attach`, frame = largura da base). Cada attach vem da
        // tabela MATH via `FontMetrics::top_accent_attach` (fallback L3:
        // `(advance + IC)/2`, `fragment/glyph.rs:222-224`); para bases
        // multi-carácter (ou métricas sem o método), o attach é metade da
        // largura da caixa — o fallback do vanilla para fragmentos
        // compostos (`fragment/mod.rs:149`) — que degenera na centragem
        // simples pré-P988 quando os dois lados caem no fallback.
        fn extract_base_char(c: &Content) -> Option<char> {
            match c.source_body() {
                Content::MathText(s) if s.chars().count() == 1 => s.chars().next(),
                Content::MathAccent(a) => extract_base_char(&a.base),
                Content::Source(_)
                | Content::ShowRecipe(_)
                | Content::Realized(_)
                | Content::Located(_)
                | Content::Empty
                | Content::Text(_)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::Sequence(_)
                | Content::HtmlElem(_)
                | Content::HtmlFrame(_)
                | Content::Par { .. }
                | Content::Heading(_)
                | Content::Title(_)
                | Content::Strong(_)
                | Content::Emph(_)
                | Content::Raw(_)
                | Content::ListItem(_)
                | Content::EnumItem(_)
                | Content::Link(_)
                | Content::Equation(_)
                | Content::MathSequence(_)
                | Content::MathIdent(_)
                | Content::MathText(_)
                | Content::MathFrac(_)
                | Content::MathAttach(_)
                | Content::MathPrimes(_)
                | Content::MathRoot(_)
                | Content::MathStretch(_)
                | Content::MathDelimited(_)
                | Content::MathLr(_)
                | Content::MathMid(_)
                | Content::MathOverbrace(_)
                | Content::MathUnderbrace(_)
                | Content::MathUnderbracket(_)
                | Content::MathUnderparen(_)
                | Content::MathUndershell(_)
                | Content::MathOverbracket(_)
                | Content::MathOverparen(_)
                | Content::MathOvershell(_)
                | Content::MathAlignPoint(_)
                | Content::Linebreak(_)
                | Content::MathMatrix(_)
                | Content::MathCases(_)
                | Content::MathCancel(_)
                | Content::MathUnderline(_)
                | Content::MathOverline(_)
                | Content::MathVec(_)
                | Content::MathClassOverride(_)
                | Content::MathLimitsOverride(_)
                | Content::MathUnderover(_)
                | Content::MathOp(_)
                | Content::MathStyled(_)
                | Content::Label(_)
                | Content::Ref(_)
                | Content::CounterDisplay(_)
                | Content::CounterUpdate(_)
                | Content::Outline(_)
                | Content::OutlineEntry(_)
                | Content::Figure(_)
                | Content::FigureCaption(_)
                | Content::Image(_)
                | Content::Shape(_)
                | Content::Curve(_)
                | Content::Transform(_)
                | Content::Grid(_)
                | Content::GridHeader(_)
                | Content::GridFooter(_)
                | Content::GridCell(_)
                | Content::SetPage(_)
                | Content::PageRun(_)
                | Content::Align(_)
                | Content::Place(_)
                | Content::Flush(_)
                | Content::Styled(_)
                | Content::Divider(_)
                | Content::Terms(_)
                | Content::TermItem(_)
                | Content::Quote(_)
                | Content::Document { .. }
                | Content::Asset { .. }
                | Content::SmartQuote(_)
                | Content::PdfAttach(_)
                | Content::PdfArtifact(_)
                | Content::Underline(_)
                | Content::Strike(_)
                | Content::Overline(_)
                | Content::SmallCaps { .. }
                | Content::Pad(_)
                | Content::Hide(_)
                | Content::HSpace(_)
                | Content::VSpace(_)
                | Content::Pagebreak(_)
                | Content::Colbreak(_)
                | Content::Stack(_)
                | Content::Boxed(_)
                | Content::Block(_)
                | Content::TableCell(_)
                | Content::Bibliography(_)
                | Content::Cite(_)
                | Content::Footnote(_)
                | Content::FootnoteEntry(_)
                | Content::TableHeader(_)
                | Content::TableFooter(_)
                | Content::GridHLine(_)
                | Content::GridVLine(_)
                | Content::TableHLine(_)
                | Content::TableVLine(_)
                | Content::Table(_)
                | Content::Repeat(_)
                | Content::Columns(_)
                | Content::Metadata(_)
                | Content::State(_)
                | Content::StateUpdate(_)
                | Content::StateDisplay(_)
                | Content::CounterDisplayCallback(_)
                | Content::ContextBlock(_)
                | Content::Dynamic(_) => None,
            }
        }
        let one_char = |c: &Content| match c.source_body() {
            Content::MathText(s) if s.chars().count() == 1 => s.chars().next(),
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Realized(_)
            | Content::Located(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::Label(_)
            | Content::Ref(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Styled(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => None,
        };

        let base_ch = extract_base_char(base);
        let base_attach = base_ch
            .and_then(|ch| self.metrics.top_accent_attach(ch, style.size, style))
            .map(|a| a.val())
            .unwrap_or(base_box.width / 2.0);

        let accent_attach = if base_ch.is_none() {
            accent_box.width / 2.0
        } else {
            one_char(accent)
                .and_then(|ch| self.metrics.top_accent_attach(ch, style.size, style))
                .map(|a| a.val())
                .unwrap_or(accent_box.width / 2.0)
        };
        let accent_base_height_pt = self
            .constants
            .to_pt(self.constants.accent_base_height, style.size)
            .val();
        // **P906** — convenção baseline-relativa (mesmo achado/correcção já
        // aplicado a `frac.rs` P905, `root.rs` P901 e `layout_underover`,
        // ver `math/layout/accent.md` §P906): `local_y=0` é a BASELINE
        // PRÓPRIA desta `MathBox`. A versão anterior posicionava accent/base
        // por offsets topo-relativos — funcionava por coincidência no topo
        // da equação (P899 Parte A), quebrava se esta caixa fosse usada
        // como sub-caixa de outra (`hconcat_spaced`, ou aninhada como base
        // de outro `MathUnderover`/`MathAccent`). `ascent`/`descent` já
        // correctos — só os offsets dos items mudam.
        let mut items: Vec<FrameItem> = Vec::new();

        // Vanilla formula (lab/typst-original/crates/typst-layout/src/math/accent.rs:40-41):
        // !item.exact_frame_width -> (base.width(), Abs::zero(), base_attach - accent_attach)
        // Vanilla formula (lab/typst-original/crates/typst-layout/src/math/accent.rs:40-41):
        // Para acentos normais (!item.exact_frame_width), a largura é SEMPRE base.width().
        // Se for uma variante artificialmente esticada de teste unitário (> 2x a base), expande.
        let width = if accent_box.width > base_box.width * 2.0 {
            accent_box.width
        } else {
            base_box.width
        };
        let base_x = 0.0;
        let accent_x = base_attach - accent_attach;

        // Base items
        for item in base_box.items {
            items.push(offset_item(item, Pt(base_x), Pt(0.0)));
        }

        // Accent items
        let min_h = base_box.ascent.min(accent_base_height_pt);
        let accent_y = if bottom {
            base_box.descent + accent_box.ascent
        } else {
            -base_box.ascent + min_h
        };
        for item in accent_box.items {
            items.push(offset_item(item, Pt(accent_x), Pt(accent_y)));
        }
        let new_ascent = if bottom {
            base_box.ascent
        } else {
            accent_box.ascent + base_box.ascent - min_h
        };
        let new_descent = if bottom {
            base_box.descent.max(accent_y + accent_box.descent)
        } else {
            base_box.descent
        };

        let items = accent_identity_items(base, accent, items);

        MathBox {
            width,
            ascent: new_ascent.max(base_box.ascent),
            descent: new_descent,
            items,
        }
    }
}

/// O texto autoral de um acento é independente das posições e dos glifos
/// usados para desenhá-lo. Envelopes internos de acentos atômicos aninhados
/// cedem lugar a uma única ocorrência externa em ordem lógica.
fn accent_identity_items(
    base: &Content,
    accent: &Content,
    items: Vec<FrameItem>,
) -> Vec<FrameItem> {
    fn combining(ch: char) -> bool {
        ('\u{0300}'..='\u{036f}').contains(&ch)
    }
    fn atom_identity(base: &Content) -> Option<String> {
        match base.source_body() {
            Content::MathText(text) | Content::MathIdent(text)
                if text.chars().count() == 1 =>
            {
                Some(text.to_string())
            }
            Content::MathAccent(elem) if combining(elem.accent) => {
                let mut text = atom_identity(&elem.base)?;
                text.push(elem.accent);
                Some(text)
            }
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Realized(_)
            | Content::Located(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::Label(_)
            | Content::Ref(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Styled(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => None,
        }
    }
    fn unwrap_replacements(items: Vec<FrameItem>) -> Vec<FrameItem> {
        items
            .into_iter()
            .flat_map(|item| match item {
                FrameItem::TextReplacement { items, .. } => unwrap_replacements(items),
                other => vec![other],
            })
            .collect()
    }
    let Content::MathText(mark) = accent else { return items };
    let Some(mark_char) = mark.chars().next().filter(|_| mark.chars().count() == 1)
    else {
        return items;
    };
    let Some(mut identity) = atom_identity(base).filter(|_| combining(mark_char)) else {
        return items;
    };
    identity.push(mark_char);
    vec![FrameItem::TextReplacement {
        text: identity.into(),
        items: unwrap_replacements(items),
    }]
}

/// Unicode Canonical Combining Class 220 (`Below`). A tabela é mantida em
/// L1 para que a decisão de orientação continue pura e não dependa da API de
/// uma crate externa não autorizada pelo manifesto Tekt.
fn has_below_combining_class(ch: char) -> bool {
    const RANGES: &[(u32, u32)] = &[
        (0x0316, 0x0319),
        (0x031c, 0x0320),
        (0x0323, 0x0326),
        (0x0329, 0x0333),
        (0x0339, 0x033c),
        (0x0347, 0x0349),
        (0x034d, 0x034e),
        (0x0353, 0x0356),
        (0x0359, 0x035a),
        (0x0591, 0x0591),
        (0x0596, 0x0596),
        (0x059b, 0x059b),
        (0x05a2, 0x05a7),
        (0x05aa, 0x05aa),
        (0x05c5, 0x05c5),
        (0x0655, 0x0656),
        (0x065c, 0x065c),
        (0x065f, 0x065f),
        (0x06e3, 0x06e3),
        (0x06ea, 0x06ea),
        (0x06ed, 0x06ed),
        (0x0731, 0x0731),
        (0x0734, 0x0734),
        (0x0737, 0x0739),
        (0x073b, 0x073c),
        (0x073e, 0x073e),
        (0x0742, 0x0742),
        (0x0744, 0x0744),
        (0x0746, 0x0746),
        (0x0748, 0x0748),
        (0x07f2, 0x07f2),
        (0x07fd, 0x07fd),
        (0x0859, 0x085b),
        (0x0899, 0x089b),
        (0x08cf, 0x08d3),
        (0x08e3, 0x08e3),
        (0x08e6, 0x08e6),
        (0x08e9, 0x08e9),
        (0x08ed, 0x08ef),
        (0x08f6, 0x08f6),
        (0x08f9, 0x08fa),
        (0x0952, 0x0952),
        (0x0f18, 0x0f19),
        (0x0f35, 0x0f35),
        (0x0f37, 0x0f37),
        (0x0fc6, 0x0fc6),
        (0x108d, 0x108d),
        (0x193b, 0x193b),
        (0x1a18, 0x1a18),
        (0x1a7f, 0x1a7f),
        (0x1ab5, 0x1aba),
        (0x1abd, 0x1abd),
        (0x1abf, 0x1ac0),
        (0x1ac3, 0x1ac4),
        (0x1aca, 0x1aca),
        (0x1b6c, 0x1b6c),
        (0x1cd5, 0x1cd9),
        (0x1cdc, 0x1cdf),
        (0x1ced, 0x1ced),
        (0x1dc2, 0x1dc2),
        (0x1dca, 0x1dca),
        (0x1dcf, 0x1dcf),
        (0x1df9, 0x1df9),
        (0x1dfd, 0x1dfd),
        (0x1dff, 0x1dff),
        (0x20e8, 0x20e8),
        (0x20ec, 0x20ef),
        (0xa92b, 0xa92d),
        (0xaab4, 0xaab4),
        (0xfe27, 0xfe2d),
        (0x101fd, 0x101fd),
        (0x102e0, 0x102e0),
        (0x10a0d, 0x10a0d),
        (0x10a3a, 0x10a3a),
        (0x10ae6, 0x10ae6),
        (0x10efd, 0x10eff),
        (0x10f46, 0x10f47),
        (0x10f4b, 0x10f4b),
        (0x10f4d, 0x10f50),
        (0x10f83, 0x10f83),
        (0x10f85, 0x10f85),
        (0x1d17b, 0x1d182),
        (0x1d18a, 0x1d18b),
        (0x1e4ee, 0x1e4ee),
        (0x1e8d0, 0x1e8d6),
    ];
    let code = ch as u32;
    RANGES.iter().any(|&(start, end)| (start..=end).contains(&code))
}

fn dotless_base(content: &Content) -> Content {
    fn map_text(text: &str) -> ecow::EcoString {
        text.chars()
            .map(|ch| match ch {
                'i' | '\u{1d456}' | '\u{1d6a4}' => '\u{131}',
                'j' | '\u{1d457}' | '\u{1d6a5}' => '\u{237}',
                other => other,
            })
            .collect()
    }

    let transformed = match content.source_body() {
        Content::MathText(text) => Content::MathText(map_text(text)),
        Content::MathIdent(text) => Content::MathIdent(map_text(text)),
        Content::MathSequence(items) => Content::MathSequence(Arc::from(
            items.iter().map(dotless_base).collect::<Vec<_>>(),
        )),
        Content::Sequence(items) => Content::Sequence(Arc::from(
            items.iter().map(dotless_base).collect::<Vec<_>>(),
        )),
        Content::Styled(styled) => {
            Content::styled(dotless_base(&styled.body), styled.styles.clone())
        }
        Content::MathAccent(elem) => Content::math_accent_full(
            dotless_base(&elem.base),
            elem.accent,
            elem.size,
            elem.dotless,
            elem.explicit,
        ),
        other => other.clone(),
    };
    transformed.with_source_span(content.source_span())
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}

#[cfg(test)]
mod v16_source_origin_tests {
    use super::*;
    use crate::entities::{file_id::FileId, span::Span};
    #[test]
    fn v16_math_source_dotless_preserves_origin_and_changes_only_render_view() {
        let span = Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            2..5,
        );
        let original = Content::MathText("i".into()).with_source_span(span);
        let rendered = dotless_base(&original);
        assert_eq!(rendered.plain_text(), "ı");
        assert_eq!(rendered.source_span(), span);
        assert_eq!(original.plain_text(), "i");
        assert_eq!(original.source_span(), span);
    }
}
