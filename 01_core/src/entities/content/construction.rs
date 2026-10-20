//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/construction.md
//! @prompt-hash 2261741c
//! @layer L1
//! @updated 2026-09-26

use super::*;

mod layout;
mod math;
mod references;
mod tables;

impl Content {
    /// Cria conteúdo de texto. **F-5b fatia 2 (P373)**: sem estilo assado — o render
    /// do `#set text`/`#set par` viaja na chain (`custom`), resolvido no layout.
    pub fn text(s: impl Into<EcoString>) -> Self {
        Self::Text(s.into())
    }

    /// Cria conteúdo vazio.
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Wrap content in an immutable shared style payload.
    pub fn styled(
        body: impl Into<Box<Content>>,
        styles: crate::entities::style::Styles,
    ) -> Self {
        Self::Styled(Arc::new(StyledContent { body: *body.into(), styles }))
    }

    /// Negrito (`*bold*`) — **F-5b fatia 1 (P371, §3a.12)**: produz a **variante
    /// própria** `Content::Strong` (modelo D), distinta de `Styled` (era
    /// `Styled([Bold])` no colapso P101, agora **superado** pelo modelo de
    /// variantes que a 0026 `:63`/0105-D prescrevem). Render (bold) replicado no
    /// layout; o `==`/`#show`/`morph_canon` distinguem `strong` de `#set text(bold)`
    /// (fidelidade ADR-0107).
    pub fn strong(body: Content) -> Self {
        Self::Strong(Arc::new(StrongElem::new(body)))
    }

    /// Itálico (`_italic_`) — **F-5b fatia 1 (P371, §3a.12)**: variante própria
    /// `Content::Emph` (era `Styled([Italic])`).
    pub fn emph(body: Content) -> Self {
        Self::Emph(Arc::new(EmphElem::new(body)))
    }
    pub fn heading(level: u8, body: Content) -> Self {
        Self::Heading(Arc::new(HeadingElem::new(level, body)))
    }

    /// Cria um título de documento (`#title(...)`).
    pub fn title(body: Content) -> Self {
        Self::Title(Arc::new(TitleElem::new(body)))
    }
    /// **P605/P606** — heading com controlo explícito de `outlined` e `bookmarked`.
    pub fn heading_with_outlined(level: u8, body: Content, outlined: bool) -> Self {
        Self::Heading(Arc::new(HeadingElem::new_with_outlined(level, body, outlined)))
    }
    /// **P606** — heading com controlo separado de `outlined` e `bookmarked`.
    pub fn heading_with_outlined_and_bookmarked(
        level: u8,
        body: Content,
        outlined: bool,
        bookmarked: Option<bool>,
    ) -> Self {
        Self::Heading(Arc::new(HeadingElem::new_with_outlined_and_bookmarked(
            level, body, outlined, bookmarked,
        )))
    }
    /// **P829** — heading via função nativa `#heading(...)`: regista na
    /// máscara `set_fields` quais campos foram explicitamente assentes
    /// (`HEADING_SET_*`) — paridade vanilla `has`/`at`/`fields`, que só vêem
    /// campos assentes no constructor (medido: `heading[H].has("level")` →
    /// false; `heading(level: 2)[H].has("level")` → true).
    pub fn heading_native(
        level: u8,
        body: Content,
        outlined: bool,
        bookmarked: Option<bool>,
        set_fields: u8,
    ) -> Self {
        let mut elem = HeadingElem::new_with_outlined_and_bookmarked(
            level, body, outlined, bookmarked,
        );
        elem.set_fields = set_fields;
        Self::Heading(Arc::new(elem))
    }
    /// Heading numerado **na forma de transporte** (F-5a de-bake, P364,
    /// `f_fronteira_e1.md` §3a.9). O campo assado `numbering_active` foi
    /// removido; a numeração vive **só na chain** — este construtor produz o
    /// `Content::Styled` que carrega `custom("heading.numbering")=true`, a forma
    /// canônica que `#set heading(numbering:)` gera em produção (transporte
    /// fatia-1). Consumidor (layout/introspect) lê o gate da chain. Usado pelos
    /// fixtures para construir um heading numerado sem passar pelo eval.
    pub fn heading_numbered(level: u8, body: Content) -> Self {
        Self::heading_numbered_with_pattern(level, body, None)
    }
    /// Heading numerado **na forma de transporte** com pattern configurável
    /// (P451). O pattern é transportado na chain como
    /// `custom("heading.numbering.pattern")`; o gate `heading.numbering` continua
    /// a ser o booleano de activação.
    pub fn heading_numbered_with_pattern(
        level: u8,
        body: Content,
        pattern: Option<ecow::EcoString>,
    ) -> Self {
        Self::heading_numbered_with_pattern_and_outlined(level, body, pattern, true)
    }
    /// **P605/P606** — variant numerado com controlo de `outlined`.
    pub fn heading_numbered_with_pattern_and_outlined(
        level: u8,
        body: Content,
        pattern: Option<ecow::EcoString>,
        outlined: bool,
    ) -> Self {
        Self::heading_numbered_with_pattern_outlined_bookmarked(
            level, body, pattern, outlined, None,
        )
    }
    /// **P606** — variant numerado com controlo separado de `outlined` e `bookmarked`.
    pub fn heading_numbered_with_pattern_outlined_bookmarked(
        level: u8,
        body: Content,
        pattern: Option<ecow::EcoString>,
        outlined: bool,
        bookmarked: Option<bool>,
    ) -> Self {
        Self::heading_numbered_native(level, body, pattern, outlined, bookmarked, 0)
    }
    /// **P829** — variant numerado para a função nativa `#heading(numbering:)`:
    /// como `heading_numbered_with_pattern_outlined_bookmarked` mas com a
    /// máscara `set_fields` dos campos explicitamente assentes (ver
    /// `heading_native`).
    pub fn heading_numbered_native(
        level: u8,
        body: Content,
        pattern: Option<ecow::EcoString>,
        outlined: bool,
        bookmarked: Option<bool>,
        set_fields: u8,
    ) -> Self {
        use crate::entities::style::Styles;
        use crate::entities::value::Value;
        let mut styles =
            Styles::new().push_custom("heading.numbering", Value::Bool(true));
        if let Some(pattern) = pattern {
            styles = styles.push_custom("heading.numbering.pattern", Value::Str(pattern));
        }
        Self::styled(
            Box::new(Self::heading_native(level, body, outlined, bookmarked, set_fields)),
            styles,
        )
    }
    /// Construtor do separador estrutural (Modelo D, P316).
    pub fn divider() -> Self {
        Self::Divider(Arc::new(DividerElem))
    }
    /// Construtor da fronteira de extensão E1 (Lote F-1, P334). Embrulha um
    /// elemento de utilizador (qualquer `impl Element`) na variante dinâmica.
    /// `Arc<E>` faz coerção para `Arc<dyn DynElement>` via o blanket.
    pub fn dynamic<E: DynElement>(elem: E) -> Self {
        Self::Dynamic(Arc::new(elem))
    }
    pub fn raw(text: impl Into<EcoString>, lang: Option<EcoString>, block: bool) -> Self {
        use crate::entities::elements::raw::{RAW_SET_BLOCK, RAW_SET_LANG};
        let set_fields = RAW_SET_BLOCK | if lang.is_some() { RAW_SET_LANG } else { 0 };
        Self::Raw(Arc::new(RawElem { text: text.into(), lang, block, set_fields }))
    }
    /// `align(alignment, body)` — Modelo D (Lote 7 P322).
    pub fn align(alignment: Align2D, body: Content) -> Self {
        Self::Align(Arc::new(AlignElem { alignment, body }))
    }
    /// `image(path, data, width, height, fit)` — Modelo D (Lote 7 P322) + P502.
    pub fn image(
        path: impl Into<String>,
        data: PtrEqArc<Vec<u8>>,
        width: Option<Box<crate::entities::value::Value>>,
        height: Option<Box<crate::entities::value::Value>>,
        fit: impl Into<EcoString>,
    ) -> Self {
        Self::Image(Arc::new(ImageElem {
            path: path.into(),
            alt: None,
            data,
            width,
            height,
            fit: fit.into(),
        }))
    }
    // ── Construtores ergonómicos família lista/termos (Modelo D, Lote 3 P318) ──
    pub fn list_item(body: Content) -> Self {
        Self::ListItem(Arc::new(ListItemElem {
            body,
            marker: None,
            marker_align: None,
            indent: None,
            body_indent: None,
            tight: None,
        }))
    }
    /// P470 — construtor com marcador customizado.
    pub fn list_item_with_marker(
        body: Content,
        marker: crate::entities::list_marker::ListMarker,
    ) -> Self {
        Self::ListItem(Arc::new(ListItemElem {
            body,
            marker: Some(marker),
            marker_align: None,
            indent: None,
            body_indent: None,
            tight: None,
        }))
    }
    /// **P504/P505** — construtor completo com marcador, alinhamento e indentação.
    pub fn list_item_full(
        body: Content,
        marker: Option<crate::entities::list_marker::ListMarker>,
        marker_align: Option<crate::entities::layout_types::Align2D>,
        indent: Option<crate::entities::layout_types::Length>,
        body_indent: Option<crate::entities::layout_types::Length>,
        tight: Option<bool>,
    ) -> Self {
        Self::ListItem(Arc::new(ListItemElem {
            body,
            marker,
            marker_align,
            indent,
            body_indent,
            tight,
        }))
    }
    pub fn enum_item(number: Option<u32>, body: Content) -> Self {
        Self::EnumItem(Arc::new(EnumItemElem {
            number,
            body,
            numbering: None,
            indent: None,
            body_indent: None,
            tight: None,
        }))
    }
    /// P470 — construtor com esquema de numeração.
    pub fn enum_item_with_numbering(
        number: Option<u32>,
        body: Content,
        numbering: crate::entities::enum_numbering::EnumNumbering,
    ) -> Self {
        Self::EnumItem(Arc::new(EnumItemElem {
            number,
            body,
            numbering: Some(numbering),
            indent: None,
            body_indent: None,
            tight: None,
        }))
    }
    /// **P505** — construtor completo com numeração e indentação.
    pub fn enum_item_full(
        number: Option<u32>,
        body: Content,
        numbering: Option<crate::entities::enum_numbering::EnumNumbering>,
        indent: Option<crate::entities::layout_types::Length>,
        body_indent: Option<crate::entities::layout_types::Length>,
        tight: Option<bool>,
    ) -> Self {
        Self::EnumItem(Arc::new(EnumItemElem {
            number,
            body,
            numbering,
            indent,
            body_indent,
            tight,
        }))
    }
    pub fn link(url: impl Into<EcoString>, body: Content) -> Self {
        Self::Link(Arc::new(LinkElem { url: url.into(), body }))
    }
    pub fn terms(items: Vec<Content>) -> Self {
        Self::Terms(Arc::new(TermsElem { items }))
    }
    pub fn term_item(term: Content, description: Content) -> Self {
        Self::TermItem(Arc::new(TermItemElem { term, description }))
    }

    // ── Construtores ergonómicos decorações de texto (Modelo D, Lote 4 P319) ──
    pub fn overline(
        body: Content,
        stroke: Option<Color>,
        offset: Option<Length>,
        extent: Option<Length>,
    ) -> Self {
        Self::Overline(Arc::new(OverlineElem { body, stroke, offset, extent }))
    }
    pub fn strike(
        body: Content,
        stroke: Option<Color>,
        offset: Option<Length>,
        extent: Option<Length>,
    ) -> Self {
        Self::Strike(Arc::new(StrikeElem { body, stroke, offset, extent }))
    }
    pub fn underline(
        body: Content,
        stroke: Option<Color>,
        offset: Option<Length>,
        extent: Option<Length>,
    ) -> Self {
        Self::Underline(Arc::new(UnderlineElem { body, stroke, offset, extent }))
    }

    /// **Passo 408** — `smallcaps(body)`.
    ///
    /// Consumer real de small caps requer shaping OpenType (`smcp`/`c2sc`);
    /// até lá, o layout trata como stub transparente (ADR-0054 graded).
    pub fn smallcaps(body: Content) -> Self {
        Self::SmallCaps { body: Box::new(body) }
    }

    /// **Passo 448** — `sub(body)`.
    pub fn sub(body: Content) -> Self {
        use crate::entities::style::{Style, Styles};
        Self::styled(Box::new(body), Styles::from_iter([Style::subscript(true)]))
    }

    /// **P471** — `sub(body, size: length)` com tamanho explícito.
    pub fn sub_with_size(
        body: Content,
        size: Option<crate::entities::layout_types::Length>,
    ) -> Self {
        use crate::entities::style::{Style, Styles};
        let mut styles = vec![Style::subscript(true)];
        if let Some(s) = size {
            styles.push(Style::subscript_size(s));
        }
        Self::styled(Box::new(body), Styles::from_iter(styles))
    }

    /// **Passo 448** — `super(body)` (`super` é keyword em Rust, logo o
    /// construtor chama-se `superscript`).
    pub fn superscript(body: Content) -> Self {
        use crate::entities::style::{Style, Styles};
        Self::styled(Box::new(body), Styles::from_iter([Style::superscript(true)]))
    }

    /// **P471** — `super(body, size: length)` com tamanho explícito.
    pub fn superscript_with_size(
        body: Content,
        size: Option<crate::entities::layout_types::Length>,
    ) -> Self {
        use crate::entities::style::{Style, Styles};
        let mut styles = vec![Style::superscript(true)];
        if let Some(s) = size {
            styles.push(Style::superscript_size(s));
        }
        Self::styled(Box::new(body), Styles::from_iter(styles))
    }

    /// **Passo 449** — `highlight(body, fill)`.
    pub fn highlight(
        body: Content,
        fill: Option<crate::entities::layout_types::Color>,
    ) -> Self {
        use crate::entities::style::{Style, Styles};
        Self::styled(Box::new(body), Styles::from_iter([Style::highlight(fill)]))
    }

    /// **P471** — `highlight(body, fill, radius, extent)` com parâmetros cosmésticos.
    pub fn highlight_full(
        body: Content,
        fill: Option<crate::entities::layout_types::Color>,
        radius: Option<crate::entities::layout_types::Length>,
        extent: Option<crate::entities::layout_types::Length>,
    ) -> Self {
        use crate::entities::style::{Style, Styles};
        let mut styles = vec![Style::highlight(fill)];
        if let Some(r) = radius {
            styles.push(Style::highlight_radius(r));
        }
        if let Some(e) = extent {
            styles.push(Style::highlight_extent(e));
        }
        Self::styled(Box::new(body), Styles::from_iter(styles))
    }

    // ── Construtores ergonómicos família state/counter (Modelo D, Lote 6 P321) ──
    /// **P460/P464** — `Content::Label` criado explicitamente pelo utilizador
    /// (`auto: false`).
    pub fn label(name: impl Into<EcoString>, body: Content) -> Self {
        Self::Label(Arc::new(LabelElem { name: name.into(), body, auto: false }))
    }

    /// **P464** — `Content::Label` gerado automaticamente pela sintaxe `<label>`
    /// (`auto: true`).
    pub fn label_auto(name: impl Into<EcoString>, body: Content) -> Self {
        Self::Label(Arc::new(LabelElem { name: name.into(), body, auto: true }))
    }

    /// **Lote 13 P328** — `Content::Figure` (figura locatável M1).
    ///
    /// **F-5a de-bake (P365, `f_fronteira_e1.md` §3a.9):** o campo assado
    /// `numbering` foi removido de `FigureElem`. Quando `numbering` é `Some(padrão)`
    /// este construtor produz a **forma de transporte** (`Content::Styled` com
    /// `custom("figure.numbering")`), a forma canônica que `#set figure(numbering:)`
    /// gera — usada pelos fixtures. `None` → figura simples. Em produção
    /// `native_figure` passa `None` (a fatia-1 carrega o gate).
    pub fn figure(
        body: Content,
        caption: Option<Content>,
        kind: Option<String>,
        numbering: Option<String>,
    ) -> Self {
        let fig = Self::Figure(Arc::new(FigureElem {
            body,
            caption,
            kind: kind.map(|name| {
                Some(crate::entities::elements::figure::FigureKind::Name(name.into()))
            }),
            resolved_kind: None,
            supplement: None,
            resolved_supplement: None,
        }));
        match numbering {
            Some(pat) => {
                use crate::entities::style::Styles;
                use crate::entities::value::Value;
                Self::styled(
                    Box::new(fig),
                    Styles::new().push_custom("figure.numbering", Value::Str(pat.into())),
                )
            }
            None => fig,
        }
    }

    /// **P1473** — `Content::FigureCaption` (`figure.caption(...)` qualificado,
    /// escopo reduzido — construção sem síntese).
    pub fn figure_caption(
        body: Content,
        position: Option<crate::entities::elements::figure_caption::CaptionPosition>,
        separator: Option<Content>,
    ) -> Self {
        Self::FigureCaption(Arc::new(FigureCaptionElem { position, separator, body }))
    }

    pub fn counter_display(
        kind: impl Into<crate::entities::counter::CounterKey>,
    ) -> Self {
        Self::CounterDisplay(Arc::new(CounterDisplayElem { kind: kind.into() }))
    }
    pub fn counter_update(
        key: impl Into<crate::entities::counter::CounterKey>,
        action: CounterAction,
    ) -> Self {
        Self::CounterUpdate(Arc::new(CounterUpdateElem { key: key.into(), action }))
    }
    pub fn metadata(value: crate::entities::value::Value) -> Self {
        Self::Metadata(Arc::new(MetadataElem { value: Box::new(value) }))
    }
    pub fn state(key: impl Into<String>, init: crate::entities::value::Value) -> Self {
        Self::State(Arc::new(StateElem { key: key.into(), init: Box::new(init) }))
    }
    pub fn state_update(
        key: impl Into<String>,
        update: crate::entities::state_update::StateUpdate,
    ) -> Self {
        Self::StateUpdate(Arc::new(StateUpdateElem { key: key.into(), update }))
    }
    pub fn state_display(
        key: impl Into<String>,
        callback: Option<crate::entities::func::Func>,
    ) -> Self {
        Self::StateDisplay(Arc::new(StateDisplayElem { key: key.into(), callback }))
    }
    pub fn counter_display_callback(
        key: impl Into<crate::entities::counter::CounterKey>,
        callback: Option<crate::entities::func::Func>,
    ) -> Self {
        Self::CounterDisplayCallback(Arc::new(CounterDisplayCallbackElem {
            key: key.into(),
            callback,
        }))
    }

    /// **Lote 8 P323** — `Content::Quote` (citação).
    pub fn quote(
        body: Content,
        attribution: Option<Content>,
        block: bool,
        quotes: bool,
    ) -> Self {
        Self::Quote(Arc::new(QuoteElem { body, attribution, block, quotes }))
    }

    /// **Passo 397** — `Content::Document` (metadata pura).
    pub fn document(
        path: EcoString,
        body: Content,
        format: Option<EcoString>,
        title: Option<Content>,
        author: Vec<EcoString>,
        date: Option<Datetime>,
        keywords: Vec<EcoString>,
    ) -> Self {
        Self::Document {
            path,
            body: Box::new(body),
            format,
            title: title.map(Box::new),
            author,
            date,
            keywords,
        }
    }

    /// **Passo 397** — `Content::Asset` (placeholder de resource).
    pub fn asset(path: EcoString, data: Vec<u8>) -> Self {
        Self::Asset { path, data: Arc::from(data) }
    }

    // ── Passo 513 — curve elements ────────────────────────────────────────────
    /// `curve.move(point)`.
    pub fn curve_move(x: Length, y: Length) -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Move(
                crate::entities::elements::curve::CurvePoint { x, y },
            )],
        }))
    }

    /// `curve.line(point)`.
    pub fn curve_line(x: Length, y: Length) -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Line(
                crate::entities::elements::curve::CurvePoint { x, y },
            )],
        }))
    }

    /// `curve.cubic(control1, control2, end)`.
    pub fn curve_cubic(
        c1x: Length,
        c1y: Length,
        c2x: Length,
        c2y: Length,
        ex: Length,
        ey: Length,
    ) -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Cubic(
                crate::entities::elements::curve::CurvePoint { x: c1x, y: c1y },
                crate::entities::elements::curve::CurvePoint { x: c2x, y: c2y },
                crate::entities::elements::curve::CurvePoint { x: ex, y: ey },
            )],
        }))
    }

    /// `curve.quad(control, end)`.
    pub fn curve_quad(cx: Length, cy: Length, ex: Length, ey: Length) -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Quad(
                crate::entities::elements::curve::CurvePoint { x: cx, y: cy },
                crate::entities::elements::curve::CurvePoint { x: ex, y: ey },
            )],
        }))
    }

    /// `curve.close()`.
    pub fn curve_close(mode: crate::entities::elements::curve::CloseMode) -> Self {
        Self::Curve(Arc::new(crate::entities::elements::curve::CurveElem {
            segments: vec![crate::entities::elements::curve::CurveSegment::Close(mode)],
        }))
    }

    /// **P863** — Cria um contentor de parágrafo.
    pub fn par(body: Content) -> Self {
        Self::Par { body: Box::new(body) }
    }
}

#[cfg(test)]
mod construction_tests {
    use super::Content;

    #[test]
    fn text_constructor_preserves_payload() {
        assert_eq!(Content::text("identity"), Content::Text("identity".into()));
    }
}
