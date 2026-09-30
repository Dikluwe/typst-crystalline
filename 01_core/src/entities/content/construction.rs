//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/construction.md
//! @prompt-hash 107d9af0
//! @layer L1
//! @updated 2026-09-26

use super::*;

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

    /// Cria uma sequência, normalizando casos degenerados.
    ///
    /// - 0 partes → `Empty`
    /// - 1 parte → desembrulha (evita `Sequence([x])`)
    /// - n > 1 → `Sequence(parts)`
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
    /// Construtor de `MathStyled` (Modelo D, P316).
    pub fn math_styled(
        kind: Option<MathStyleKind>,
        bold: Option<bool>,
        italic: Option<bool>,
        body: Content,
        cramped: Option<bool>,
    ) -> Self {
        Self::MathStyled(Arc::new(MathStyledElem { kind, bold, italic, body, cramped }))
    }

    // ── Construtores ergonómicos da família math (Modelo D, Lote 2 P317) ──────
    /// Construtor de `MathFrac`.
    pub fn math_frac(num: Content, den: Content) -> Self {
        Self::math_frac_full(num, den, None, false, false, true)
    }
    /// Construtor completo de `MathFrac`, incluindo causalidade sintáctica.
    pub fn math_frac_full(
        num: Content,
        den: Content,
        style: Option<crate::entities::elements::math_frac::MathFracStyle>,
        num_deparenthesized: bool,
        denom_deparenthesized: bool,
        line: bool,
    ) -> Self {
        Self::MathFrac(Arc::new(MathFracElem {
            num,
            den,
            style,
            num_deparenthesized,
            denom_deparenthesized,
            line,
        }))
    }
    /// Pilha frac-like sem barra, usada pela morfologia de `binom`.
    pub(crate) fn math_frac_unlined(num: Content, den: Content) -> Self {
        Self::math_frac_full(num, den, None, false, false, false)
    }
    /// Construtor de `MathAttach` (base + pre/pos-scripts opcionais).
    /// Construtor canónico de scripts (retrocompatibilidade com testes existentes)
    pub fn math_attach_scripts(
        base: Content,
        tl: MathAttachSlot,
        bl: MathAttachSlot,
        sub: MathAttachSlot,
        sup: MathAttachSlot,
    ) -> Self {
        Self::MathAttach(Arc::new(MathAttachElem {
            base,
            t: MathAttachSlot::Omitted,
            b: MathAttachSlot::Omitted,
            tl,
            bl,
            tr: sup,
            br: sub,
        }))
    }

    pub fn math_attach(
        base: Content,
        t: MathAttachSlot,
        b: MathAttachSlot,
        tl: MathAttachSlot,
        bl: MathAttachSlot,
        tr: MathAttachSlot,
        br: MathAttachSlot,
    ) -> Self {
        Self::MathAttach(Arc::new(MathAttachElem { base, t, b, tl, bl, tr, br }))
    }

    /// Construtor de `MathPrimes`.
    pub fn math_primes(count: usize) -> Self {
        Self::MathPrimes(Arc::new(MathPrimesElem { count }))
    }
    /// Construtor de `MathRoot` (`index: None` = raiz quadrada).
    pub fn math_root(
        index: Option<Content>,
        radicand: Content,
        index_explicit: bool,
    ) -> Self {
        Self::MathRoot(Arc::new(MathRootElem { index, radicand, index_explicit }))
    }
    /// Construtor canônico do elemento público `math.stretch`.
    pub fn math_stretch(body: Content, size: Rel<Length>, size_explicit: bool) -> Self {
        Self::MathStretch(Arc::new(MathStretchElem { body, size, size_explicit }))
    }
    /// Construtor de `MathDelimited`.
    pub fn math_delimited(open: char, body: Content, close: char) -> Self {
        Self::MathDelimited(Arc::new(MathDelimitedElem { open, body, close }))
    }
    /// Construtor canônico do elemento público `math.lr`.
    pub fn math_lr(
        body: Content,
        size: crate::entities::rel::Rel<crate::entities::layout_types::Length>,
        size_explicit: bool,
    ) -> Self {
        Self::MathLr(Arc::new(MathLrElem { body, size, size_explicit }))
    }
    /// Construtor canônico do elemento público `math.mid`.
    pub fn math_mid(body: Content) -> Self {
        Self::MathMid(Arc::new(MathMidElem { body }))
    }
    /// Construtor canônico do elemento público `math.overbrace`.
    pub fn math_overbrace(
        body: Content,
        annotation: Option<Content>,
        annotation_explicit: bool,
    ) -> Self {
        Self::MathOverbrace(Arc::new(MathOverbraceElem {
            body,
            annotation,
            annotation_explicit,
        }))
    }
    /// Construtor canônico do elemento público `math.underbrace`.
    pub fn math_underbrace(
        body: Content,
        annotation: Option<Content>,
        annotation_explicit: bool,
    ) -> Self {
        Self::MathUnderbrace(Arc::new(MathUnderbraceElem {
            body,
            annotation,
            annotation_explicit,
        }))
    }
    /// Construtor canônico do elemento público `math.underbracket`.
    pub fn math_underbracket(
        body: Content,
        annotation: Option<Content>,
        annotation_explicit: bool,
    ) -> Self {
        Self::MathUnderbracket(Arc::new(MathUnderbracketElem {
            body,
            annotation,
            annotation_explicit,
        }))
    }
    /// Construtor canônico do elemento público `math.underparen`.
    pub fn math_underparen(
        body: Content,
        annotation: Option<Content>,
        annotation_explicit: bool,
    ) -> Self {
        Self::MathUnderparen(Arc::new(MathUnderparenElem {
            body,
            annotation,
            annotation_explicit,
        }))
    }
    /// Construtor canônico do elemento público `math.undershell`.
    pub fn math_undershell(
        body: Content,
        annotation: Option<Content>,
        annotation_explicit: bool,
    ) -> Self {
        Self::MathUndershell(Arc::new(MathUndershellElem {
            body,
            annotation,
            annotation_explicit,
        }))
    }
    /// Construtor canônico do elemento público `math.overbracket`.
    pub fn math_overbracket(
        body: Content,
        annotation: Option<Content>,
        annotation_explicit: bool,
    ) -> Self {
        Self::MathOverbracket(Arc::new(MathOverbracketElem {
            body,
            annotation,
            annotation_explicit,
        }))
    }
    /// Construtor canônico do elemento público `math.overparen`.
    pub fn math_overparen(
        body: Content,
        annotation: Option<Content>,
        annotation_explicit: bool,
    ) -> Self {
        Self::MathOverparen(Arc::new(MathOverparenElem {
            body,
            annotation,
            annotation_explicit,
        }))
    }
    /// Construtor canônico do elemento público `math.overshell`.
    pub fn math_overshell(
        body: Content,
        annotation: Option<Content>,
        annotation_explicit: bool,
    ) -> Self {
        Self::MathOvershell(Arc::new(MathOvershellElem {
            body,
            annotation,
            annotation_explicit,
        }))
    }
    /// Construtor de `MathAlignPoint` (marcador `&`).
    pub fn math_align_point() -> Self {
        Self::MathAlignPoint(Arc::new(MathAlignPointElem))
    }
    /// Construtor de `MathMatrix`.
    pub fn math_matrix(rows: Vec<Vec<Content>>, delim: (char, char)) -> Self {
        Self::math_matrix_full(rows, delim, None, None, None, None)
    }
    /// Construtor completo de `MathMatrix`.
    pub fn math_matrix_full(
        rows: Vec<Vec<Content>>,
        delim: (char, char),
        row_gap: Option<crate::entities::layout_types::Length>,
        column_gap: Option<crate::entities::layout_types::Length>,
        gap: Option<crate::entities::layout_types::Length>,
        augment: Option<usize>,
    ) -> Self {
        use crate::entities::layout_types::{HAlign, Length};
        use crate::entities::rel::Rel;
        let to_rel = |value: Option<Length>, default_em: f64| Rel {
            rel: 0.0,
            abs: value.unwrap_or_else(|| Length::em(default_em)),
        };
        Self::math_matrix_complete(
            rows,
            ((delim.0 != '\0').then_some(delim.0), (delim.1 != '\0').then_some(delim.1)),
            HAlign::Center,
            augment.map(|offset| MathMatrixAugment {
                hline: vec![],
                vline: vec![offset as i64],
                stroke: None,
            }),
            to_rel(row_gap.or(gap), 0.2),
            to_rel(column_gap.or(gap), 0.5),
            MathMatrixExplicit {
                delim: true,
                align: false,
                augment: augment.is_some(),
                row_gap: row_gap.is_some() || gap.is_some(),
                column_gap: column_gap.is_some() || gap.is_some(),
            },
        )
    }

    /// Construtor canônico completo de `MathMatrix`.
    pub fn math_matrix_complete(
        rows: Vec<Vec<Content>>,
        delim: (Option<char>, Option<char>),
        align: crate::entities::layout_types::HAlign,
        augment: Option<MathMatrixAugment>,
        row_gap: crate::entities::rel::Rel<crate::entities::layout_types::Length>,
        column_gap: crate::entities::rel::Rel<crate::entities::layout_types::Length>,
        explicit: MathMatrixExplicit,
    ) -> Self {
        Self::MathMatrix(Arc::new(MathMatrixElem {
            rows,
            delim,
            align,
            augment,
            row_gap,
            column_gap,
            explicit,
        }))
    }
    /// Construtor de `MathCases`.
    pub fn math_cases(
        rows: Vec<Vec<Content>>,
        delim: (char, char),
        reverse: bool,
        gap: Option<crate::entities::layout_types::Length>,
    ) -> Self {
        let children = rows
            .into_iter()
            .map(|row| {
                let mut cells = row.into_iter();
                let Some(first) = cells.next() else { return Content::Empty };
                let mut items = vec![first];
                for cell in cells {
                    items.push(Content::math_align_point());
                    items.push(cell);
                }
                Content::MathSequence(Arc::from(items.into_boxed_slice()))
            })
            .collect();
        Self::math_cases_full(
            children,
            ((delim.0 != '\0').then_some(delim.0), (delim.1 != '\0').then_some(delim.1)),
            reverse,
            gap.map_or(
                crate::entities::rel::Rel {
                    rel: 0.0,
                    abs: crate::entities::layout_types::Length::em(0.2),
                },
                |abs| crate::entities::rel::Rel { rel: 0.0, abs },
            ),
            MathCasesExplicit { delim: true, reverse: true, gap: gap.is_some() },
        )
    }

    /// Construtor canónico de `MathCases`.
    pub fn math_cases_full(
        children: Vec<Content>,
        delim: (Option<char>, Option<char>),
        reverse: bool,
        gap: crate::entities::rel::Rel<crate::entities::layout_types::Length>,
        explicit: MathCasesExplicit,
    ) -> Self {
        Self::MathCases(Arc::new(MathCasesElem {
            children,
            delim,
            reverse,
            gap,
            explicit,
        }))
    }
    /// Construtor de `MathAccent`.
    pub fn math_accent(base: Content, accent: char) -> Self {
        Self::math_accent_full(
            base,
            accent,
            crate::entities::rel::Rel::from_percent(100.0),
            true,
            MathAccentExplicit::default(),
        )
    }

    pub fn math_accent_full(
        base: Content,
        accent: char,
        size: crate::entities::rel::Rel<crate::entities::layout_types::Length>,
        dotless: bool,
        explicit: MathAccentExplicit,
    ) -> Self {
        Self::MathAccent(Arc::new(MathAccentElem {
            base,
            accent,
            size,
            dotless,
            explicit,
        }))
    }
    /// Construtor de `MathCancel`.
    pub fn math_cancel(body: Content) -> Self {
        Self::math_cancel_full(
            body,
            crate::entities::rel::Rel::from_percent(100.0)
                + crate::entities::layout_types::Length::em(0.3),
            false,
            false,
            MathCancelAngle::Auto,
            None,
            false,
            crate::entities::span::Span::detached(),
            MathCancelExplicit::default(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn math_cancel_full(
        body: Content,
        length: crate::entities::rel::Rel<crate::entities::layout_types::Length>,
        inverted: bool,
        cross: bool,
        angle: MathCancelAngle,
        stroke: Option<crate::entities::geometry::Stroke>,
        background: bool,
        span: crate::entities::span::Span,
        explicit: MathCancelExplicit,
    ) -> Self {
        Self::MathCancel(Arc::new(MathCancelElem {
            body,
            length,
            inverted,
            cross,
            angle,
            stroke,
            background,
            span,
            explicit,
        }))
    }

    /// Construtor canónico do underline matemático.
    pub fn math_underline(body: Content) -> Self {
        Self::MathUnderline(Arc::new(MathUnderlineElem { body }))
    }

    /// Construtor canónico do overline matemático.
    pub fn math_overline(body: Content) -> Self {
        Self::MathOverline(Arc::new(MathOverlineElem { body }))
    }

    /// Construtor de conveniência de `MathVec`, com named omitidos.
    pub fn math_vec(children: Vec<Content>) -> Self {
        Self::math_vec_full(
            children,
            ('(', ')'),
            crate::entities::layout_types::HAlign::Center,
            crate::entities::rel::Rel {
                rel: 0.0,
                abs: crate::entities::layout_types::Length::em(0.2),
            },
            MathVecExplicit::default(),
        )
    }

    /// Construtor canónico de `MathVec`.
    pub fn math_vec_full(
        children: Vec<Content>,
        delim: (char, char),
        align: crate::entities::layout_types::HAlign,
        gap: crate::entities::rel::Rel<crate::entities::layout_types::Length>,
        explicit: MathVecExplicit,
    ) -> Self {
        Self::MathVec(Arc::new(MathVecElem { children, delim, align, gap, explicit }))
    }

    /// Construtor de `MathClassOverride` — `math.class(class, body)`.
    pub fn math_class_override(
        class: crate::entities::math_class::MathClass,
        body: Content,
    ) -> Self {
        Self::MathClassOverride(Arc::new(MathClassOverrideElem { class, body }))
    }
    /// Construtor de `MathLimitsOverride` — `limits(body, inline:)`/`scripts(body)`.
    pub fn math_limits_override(
        body: Content,
        limits: bool,
        inline: bool,
        inline_explicit: bool,
    ) -> Self {
        Self::MathLimitsOverride(Arc::new(MathLimitsOverrideElem {
            body,
            limits,
            inline,
            inline_explicit,
        }))
    }
    /// Construtor de `MathUnderover`.
    pub fn math_underover(
        base: Content,
        under: Option<Content>,
        over: Option<Content>,
    ) -> Self {
        Self::MathUnderover(Arc::new(MathUnderoverElem { base, under, over }))
    }
    /// Construtor de `MathOp` (`limits` é discriminador de layout).
    pub fn math_op(text: Content, limits: bool) -> Self {
        Self::MathOp(Arc::new(MathOpElem { text, limits }))
    }

    pub fn raw(text: impl Into<EcoString>, lang: Option<EcoString>, block: bool) -> Self {
        Self::Raw(Arc::new(RawElem { text: text.into(), lang, block }))
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

    /// `pad(body, sides)` — Passo 156C (ADR-0061 Fase 1) /
    /// Passo 156L (refino sides individualizadas per ADR-0064 Caso C).
    pub fn pad(body: Content, sides: Sides<Option<Length>>) -> Self {
        Self::Pad(Arc::new(PadElem { body, sides }))
    }

    /// **Lote 10 P325** — `Content::Equation` (equação matemática).
    pub fn equation(body: Content, block: bool) -> Self {
        Self::Equation(Arc::new(EquationElem::new(body, block)))
    }
    /// Equação numerada **na forma de transporte** (F-5a de-bake, P364,
    /// `f_fronteira_e1.md` §3a.9). Campo assado removido; a numeração vive **só
    /// na chain** — produz o `Content::Styled` com `custom("equation.numbering")
    /// ="(1)"`, a forma que `#set math.equation(numbering:)` gera em produção. O
    /// gate efetivo continua `block && numbering` no consumidor.
    /// P456: pattern default "(1)" (format_counter) em vez de Bool.
    pub fn equation_numbered(body: Content, block: bool) -> Self {
        use crate::entities::style::Styles;
        use crate::entities::value::Value;
        Self::styled(
            Box::new(Self::equation(body, block)),
            Styles::new().push_custom("equation.numbering", Value::Str("(1)".into())),
        )
    }

    /// **Lote 11 P326** — `Content::Footnote` (nota de rodapé).
    pub fn footnote(body: Content) -> Self {
        Self::Footnote(Arc::new(FootnoteElem { body, numbering: None }))
    }

    /// **P502** — `Content::Footnote` com `numbering` customizado.
    pub fn footnote_with_numbering(body: Content, numbering: Option<EcoString>) -> Self {
        Self::Footnote(Arc::new(FootnoteElem { body, numbering }))
    }

    /// **P1473** — `Content::FootnoteEntry` (`footnote.entry(...)` qualificado,
    /// escopo reduzido — construção sem realização).
    pub fn footnote_entry(
        note: Content,
        separator: Option<Content>,
        clearance: Option<crate::entities::layout_types::Length>,
        gap: Option<crate::entities::layout_types::Length>,
        indent: Option<crate::entities::layout_types::Length>,
    ) -> Self {
        Self::FootnoteEntry(Arc::new(FootnoteEntryElem {
            note,
            separator,
            clearance,
            gap,
            indent,
        }))
    }

    /// **Lote 11 P326** — `Content::Shape` (geometria).
    pub fn shape(
        kind: ShapeKind,
        width: Option<Box<crate::entities::value::Value>>,
        height: Option<Box<crate::entities::value::Value>>,
        fill: Option<crate::entities::paint::Paint>,
        stroke: Option<Stroke>,
    ) -> Self {
        Self::shape_with_radius(
            kind,
            width,
            height,
            fill,
            stroke,
            Corners::uniform(Length::ZERO),
        )
    }

    /// **P852** — `Content::Shape` com radius (rounded rect).
    pub fn shape_with_radius(
        kind: ShapeKind,
        width: Option<Box<crate::entities::value::Value>>,
        height: Option<Box<crate::entities::value::Value>>,
        fill: Option<crate::entities::paint::Paint>,
        stroke: Option<Stroke>,
        radius: Corners<Length>,
    ) -> Self {
        let kind = match kind {
            ShapeKind::Rect if radius != Corners::uniform(Length::ZERO) => {
                ShapeKind::RoundedRect { radii: radius }
            }
            other => other,
        };
        Self::Shape(Arc::new(ShapeElem {
            kind,
            width,
            height,
            fill,
            stroke,
            fill_rule: crate::entities::geometry::FillRule::NonZero,
        }))
    }

    /// `hide(body)` — Passo 156C (ADR-0061 Fase 1).
    pub fn hide(body: Content) -> Self {
        Self::Hide(Arc::new(HideElem { body }))
    }

    /// `h(amount, weak)` — Passo 156D (ADR-0061 Fase 1 sub-passo 2).
    pub fn h_space(amount: Length, weak: bool) -> Self {
        Self::h_space_with_weak_presence(amount, weak, weak)
    }

    /// `h(amount, weak)` preservando a presença do named `weak` (P1140.9).
    pub fn h_space_with_weak_presence(
        amount: Length,
        weak: bool,
        weak_explicit: bool,
    ) -> Self {
        Self::HSpace(Arc::new(HSpaceElem {
            amount: crate::entities::elements::h_space::Spacing::Absolute(amount),
            weak,
            weak_explicit,
        }))
    }

    /// **P842 (#38)** — `h(amount.fr, weak)`: fração do espaço restante da
    /// linha (paridade vanilla `Spacing::Fractional`, `layout/spacing.rs`).
    pub fn h_space_fraction(fr: f64, weak: bool) -> Self {
        Self::h_space_fraction_with_weak_presence(fr, weak, weak)
    }

    /// `h(amount.fr, weak)` preservando a presença do named `weak` (P1140.9).
    pub fn h_space_fraction_with_weak_presence(
        fr: f64,
        weak: bool,
        weak_explicit: bool,
    ) -> Self {
        Self::HSpace(Arc::new(HSpaceElem {
            amount: crate::entities::elements::h_space::Spacing::Fractional(fr),
            weak,
            weak_explicit,
        }))
    }

    /// `v(amount, weak)` — Passo 156D (ADR-0061 Fase 1 sub-passo 2).
    pub fn v_space(amount: Length, weak: bool) -> Self {
        Self::v_space_with_weak_presence(amount, weak, weak)
    }

    /// `v(amount, weak)` preservando a presença do named `weak` (P1140.9).
    pub fn v_space_with_weak_presence(
        amount: Length,
        weak: bool,
        weak_explicit: bool,
    ) -> Self {
        Self::VSpace(Arc::new(VSpaceElem { amount, weak, weak_explicit }))
    }

    /// `pagebreak(weak, to)` — Passo 156E (ADR-0061 Fase 1 sub-passo 3).
    pub fn pagebreak(weak: bool, to: Option<Parity>) -> Self {
        Self::pagebreak_with_weak_presence(weak, weak, to)
    }

    /// `pagebreak(weak, to)` preservando a presença do named `weak` (P1140.9).
    pub fn pagebreak_with_weak_presence(
        weak: bool,
        weak_explicit: bool,
        to: Option<Parity>,
    ) -> Self {
        Self::Pagebreak(Arc::new(PagebreakElem { weak, weak_explicit, to }))
    }

    /// Page-run lexical interno. O binding público será materializado em P1140.21.
    pub fn page_run_with_geometry(
        paper: Option<crate::entities::page_geometry::Paper>,
        flipped: Option<bool>,
        binding: Option<crate::entities::page_geometry::PageBinding>,
        width: Option<crate::entities::layout_types::PageDimension>,
        height: Option<crate::entities::layout_types::PageDimension>,
        margin: Option<crate::entities::layout_types::PageMarginSpec>,
        numbering: Option<Option<crate::entities::numbering::Numbering>>,
        columns: Option<usize>,
        body: Content,
    ) -> Self {
        Self::PageRun(Arc::new(PageRunElem {
            paper,
            flipped,
            binding,
            width,
            height,
            margin,
            numbering,
            number_align: None,
            header: None,
            header_ascent: None,
            footer: None,
            footer_descent: None,
            supplement: None,
            columns,
            bleed: None,
            fill: None,
            background: None,
            foreground: None,
            body,
        }))
    }

    pub fn page_run(
        width: Option<crate::entities::layout_types::PageDimension>,
        height: Option<crate::entities::layout_types::PageDimension>,
        margin: Option<crate::entities::layout_types::PageMarginSpec>,
        numbering: Option<Option<crate::entities::numbering::Numbering>>,
        columns: Option<usize>,
        body: Content,
    ) -> Self {
        Self::page_run_with_geometry(
            None, None, None, width, height, margin, numbering, columns, body,
        )
    }

    /// `colbreak(weak)` — Passo 220 (ADR-0078 PROPOSTO sub-fase b 4/4).
    pub fn colbreak(weak: bool) -> Self {
        Self::colbreak_with_weak_presence(weak, weak)
    }

    /// `colbreak(weak)` preservando a presença do named `weak` (P1140.9).
    pub fn colbreak_with_weak_presence(weak: bool, weak_explicit: bool) -> Self {
        Self::Colbreak(Arc::new(ColbreakElem { weak, weak_explicit }))
    }

    /// `linebreak()` — Modelo D (Lote 5 P320).
    pub fn linebreak() -> Self {
        Self::linebreak_with_justify_presence(false, false)
    }

    /// `linebreak(justify:)` preservando valor e presença do named (P1140.10).
    pub fn linebreak_with_justify_presence(
        justify: bool,
        justify_explicit: bool,
    ) -> Self {
        Self::Linebreak(Arc::new(LinebreakElem { justify, justify_explicit }))
    }
    /// `grid_header(body, repeat)` — Modelo D (Lote 5 P320).
    pub fn grid_header(body: Content, repeat: bool) -> Self {
        Self::GridHeader(Arc::new(GridHeaderElem { body, repeat }))
    }
    /// `grid_footer(body, repeat)` — Modelo D (Lote 5 P320).
    pub fn grid_footer(body: Content, repeat: bool) -> Self {
        Self::GridFooter(Arc::new(GridFooterElem { body, repeat }))
    }
    /// `grid_hline(start, end, row, stroke, position)` — Passo 512.
    pub fn grid_hline(
        start: usize,
        end: Option<usize>,
        row: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::GridHLine(Arc::new(GridHLineElem { start, end, row, stroke, position }))
    }
    /// `grid_vline(start, end, col, stroke, position)` — Passo 512.
    pub fn grid_vline(
        start: usize,
        end: Option<usize>,
        col: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::GridVLine(Arc::new(GridVLineElem { start, end, col, stroke, position }))
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
        let fig = Self::Figure(Arc::new(FigureElem { body, caption, kind }));
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

    /// `block(body, width, height, inset, breakable)` — Passo 156G
    /// (ADR-0061 Fase 2 sub-passo 1). Construtor com defaults sensatos
    /// (None/zero/true) para uso programático.
    pub fn block(
        body: Content,
        width: Option<Length>,
        height: Option<Length>,
        inset: Sides<Length>,
        breakable: bool,
    ) -> Self {
        Self::Block(Arc::new(BlockElem {
            body,
            width: width.map(|abs| crate::entities::rel::Rel { rel: 0.0, abs }),
            height: height.map(|abs| {
                crate::entities::elements::block::BlockHeight::Relative(
                    crate::entities::rel::Rel { rel: 0.0, abs },
                )
            }),
            inset,
            breakable,
            outset: Sides::new(
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
            ),
            // P242 — radius `Corners<Length>` substitui `Option<Length>` P231.
            radius: crate::entities::corners::Corners::uniform(Length::ZERO),
            clip: false,
            // P247 — fill/stroke default `None` (paridade pattern P242).
            fill: None,
            stroke: None,
            // P250 — spacing/above/below/sticky defaults (None×3 + false).
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }))
    }

    /// `box(body, width, height, inset, baseline)` — Passo 156H
    /// (ADR-0061 Fase 2 sub-passo 2). Naming `boxed` evita conflito com
    /// `std::boxed::Box`; stdlib expõe `#box(...)` (paridade vanilla).
    pub fn boxed(
        body: Content,
        width: Option<Length>,
        height: Option<Length>,
        inset: Sides<Length>,
        baseline: Length,
    ) -> Self {
        Self::Boxed(Arc::new(BoxedElem {
            body,
            width,
            width_fraction: None,
            height,
            inset,
            baseline,
            outset: Sides::new(
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
            ),
            // P242 — radius `Corners<Length>` substitui `Option<Length>` P231.
            radius: crate::entities::corners::Corners::uniform(Length::ZERO),
            clip: false,
            // P247 — fill/stroke default `None` paralelo Block.
            fill: None,
            stroke: None,
        }))
    }

    pub fn boxed_fractional(body: Content, width_fraction: f64) -> Self {
        Self::Boxed(Arc::new(BoxedElem {
            body,
            width: None,
            width_fraction: Some(width_fraction),
            height: None,
            inset: Sides::uniform(Length::ZERO),
            baseline: Length::ZERO,
            outset: Sides::uniform(Length::ZERO),
            radius: crate::entities::corners::Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
        }))
    }

    /// `stack(dir, spacing, ..children)` — Passo 156I (ADR-0061 Fase 2
    /// sub-passo 3). Atinge target 72% Layout. Aceita Vec<Content> que
    /// converte para `Arc<[Content]>` (clone O(1) per ADR-0026 revisão).
    pub fn stack(children: Vec<Content>, dir: Dir, spacing: Option<Length>) -> Self {
        Self::Stack(Arc::new(StackElem { children: children.into(), dir, spacing }))
    }

    /// **Lote 9 P324** — `Content::SmartQuote` (aspa lang-aware).
    pub fn smartquote(double: bool) -> Self {
        Self::SmartQuote(Arc::new(SmartQuoteElem {
            double,
            alternative: None,
            quotes: None,
        }))
    }

    pub fn pdf_attach(elem: PdfAttachElem) -> Self {
        Self::PdfAttach(Arc::new(elem))
    }

    pub fn pdf_artifact(elem: PdfArtifactElem) -> Self {
        Self::PdfArtifact(Arc::new(elem))
    }

    /// **Lote 9 P324** — `Content::Transform` (transformação afim 2D).
    pub fn transform(matrix: TransformMatrix, body: Content) -> Self {
        Self::Transform(Arc::new(TransformElem { matrix, body }))
    }

    /// **Lote 9 P324** — `Content::Place` (posicionamento absoluto/flutuante).
    #[allow(clippy::too_many_arguments)]
    pub fn place(
        alignment: Align2D,
        dx: f64,
        dy: f64,
        scope: PlaceScope,
        float: bool,
        clearance: Option<Length>,
        body: Content,
    ) -> Self {
        Self::Place(Arc::new(PlaceElem {
            alignment,
            dx,
            dy,
            scope,
            float,
            clearance,
            body,
        }))
    }

    /// Constrói uma sentinela `place.flush()` sem campos.
    pub fn flush() -> Self {
        Self::Flush(Arc::new(FlushElem))
    }

    /// `repeat(body, gap, justify)` — Passo 156J (ADR-0061 Fase 3
    /// sub-passo 1). **Primeira Fase 3**. Default `justify == true`
    /// (paridade vanilla); algoritmo dinâmico de quantidade-para-encher
    /// diferido per ADR-0054 graded.
    pub fn repeat(body: Content, gap: Option<Length>, justify: bool) -> Self {
        Self::Repeat(Arc::new(RepeatElem { body, gap, justify }))
    }

    /// **P217** — Construtor `Content::Columns` (multi-column container).
    /// Stdlib `native_columns` em P218 com validação `count >= 1`.
    /// Consumer multi-region real em P219.
    pub fn columns(body: Content, count: usize, gutter: Option<Length>) -> Self {
        Self::Columns(Arc::new(ColumnsElem { count, gutter, body, page_columns: false }))
    }

    /// **P462** — `Content::Ref` (referência cruzada `@label` / `ref("label")`).
    /// Nome `reference` (não `r#ref`: evita raw identifier nos call-sites).
    pub fn reference(name: impl Into<EcoString>) -> Self {
        Self::reference_with_supplement(name, None)
    }

    /// **P462** — `Content::Ref` com supplement explícito (ex: `ref("fig1", supplement: "Fig. ")`).
    pub fn reference_with_supplement(
        name: impl Into<EcoString>,
        supplement: Option<Content>,
    ) -> Self {
        Self::reference_with_form(
            name,
            supplement,
            crate::entities::elements::r#ref::RefForm::Normal,
        )
    }

    pub fn reference_with_form(
        name: impl Into<EcoString>,
        supplement: Option<Content>,
        form: crate::entities::elements::r#ref::RefForm,
    ) -> Self {
        Self::Ref(Arc::new(RefElem { name: name.into(), supplement, form }))
    }

    /// **Lote 8 P323** — `Content::Outline` (índice). Unit struct.
    /// P457: adiciona parâmetros `title`, `depth`, `indent` com defaults vanilla.
    /// P502: `indent` é `OutlineIndent` (auto/bool/length/function).
    pub fn outline() -> Self {
        use crate::entities::elements::outline::OutlineIndent;
        Self::outline_with(None, 3, OutlineIndent::Auto)
    }

    pub fn outline_entry(level: usize, element: Content, fill: Option<Content>) -> Self {
        Self::outline_entry_with_fill_state(level, element, fill, true)
    }

    pub fn outline_entry_with_fill_state(
        level: usize,
        element: Content,
        fill: Option<Content>,
        fill_set: bool,
    ) -> Self {
        Self::OutlineEntry(Arc::new(OutlineEntryElem::new(
            level, element, fill, fill_set,
        )))
    }

    pub fn realized_outline_entry(
        level: usize,
        element: Content,
        fill: Option<Content>,
        prefix: Option<Content>,
        element_location: crate::entities::location::Location,
    ) -> Self {
        Self::OutlineEntry(Arc::new(OutlineEntryElem::realized(
            level,
            element,
            fill,
            true,
            prefix,
            element_location,
        )))
    }

    pub fn outline_with(
        title: Option<Content>,
        depth: usize,
        indent: crate::entities::elements::outline::OutlineIndent,
    ) -> Self {
        Self::Outline(Arc::new(OutlineElem::new(title, depth, indent)))
    }

    /// **P472** — List of Figures (`lof()`).
    pub fn lof(title: Option<Content>) -> Self {
        use crate::entities::elements::outline::{OutlineIndent, OutlineTarget};
        Self::Outline(Arc::new(OutlineElem::with_target(
            title,
            1,
            OutlineIndent::Bool(false),
            OutlineTarget::Figures,
        )))
    }

    /// **P472** — List of Tables (`lot()`).
    pub fn lot(title: Option<Content>) -> Self {
        use crate::entities::elements::outline::{OutlineIndent, OutlineTarget};
        Self::Outline(Arc::new(OutlineElem::with_target(
            title,
            1,
            OutlineIndent::Bool(false),
            OutlineTarget::Tables,
        )))
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

    /// `table(columns, rows, ..children)` — Passo 157A (ADR-0060
    /// Fase 2 sub-passo 1; **primeiro sub-passo Model Fase 2**).
    /// Subset minimal: cells distribuídas como `Content::Grid`;
    /// TableCell estruturado + Header/Footer diferidos para P157B/C.
    pub fn table(
        columns: Vec<TrackSizing>,
        rows: Vec<TrackSizing>,
        children: Vec<Content>,
    ) -> Self {
        Self::Table(Arc::new(TableElem {
            columns,
            rows,
            children,
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            caption: None,
            summary: None,
            inset: crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(5.0),
            ),
            align: None,
        }))
    }

    /// `table_with_caption(columns, rows, children, caption)` — P459.
    /// Variante programática com caption opcional para numeração automática.
    pub fn table_with_caption(
        columns: Vec<TrackSizing>,
        rows: Vec<TrackSizing>,
        children: Vec<Content>,
        caption: Option<Content>,
    ) -> Self {
        Self::Table(Arc::new(TableElem {
            columns,
            rows,
            children,
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            caption,
            summary: None,
            inset: crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(5.0),
            ),
            align: None,
        }))
    }

    /// `table_cell(body, x, y, colspan, rowspan)` — Passo 157B
    /// (ADR-0060 Fase 2 sub-passo 2). `x`/`y` ADR-0064 Caso A;
    /// `colspan`/`rowspan` Caso C. Placement algorítmico diferido
    /// em DEBT-34e — fields armazenados mas ignorados em layout.
    pub fn table_cell(
        body: Content,
        x: Option<usize>,
        y: Option<usize>,
        colspan: Option<usize>,
        rowspan: Option<usize>,
    ) -> Self {
        Self::TableCell(Arc::new(TableCellElem {
            body,
            x,
            y,
            colspan,
            rowspan,
            stroke: None,
            fill: None,
            align: None,
            inset: None,
            breakable: None,
            kind: crate::entities::elements::table_cell::TableCellKind::Auto,
        }))
    }

    /// `table_header(body, repeat)` — Passo 157C (ADR-0060 Fase 2
    /// sub-passo 3 — **fecha table foundations**). `repeat: bool`
    /// ADR-0064 Caso D (default `true` paridade vanilla — primeira
    /// aplicação Caso D em Model). Algoritmo de repetição diferido
    /// em DEBT-56.
    pub fn table_header(body: Content, repeat: bool) -> Self {
        Self::TableHeader(Arc::new(TableHeaderElem { body, repeat }))
    }

    /// `table_footer(body, repeat)` — par simétrico de `table_header`
    /// (Passo 157C). Mesma decisão Caso D + DEBT-56.
    pub fn table_footer(body: Content, repeat: bool) -> Self {
        Self::TableFooter(Arc::new(TableFooterElem { body, repeat }))
    }
    /// `table_hline(start, end, row, stroke, position)` — Passo 512.
    pub fn table_hline(
        start: usize,
        end: Option<usize>,
        row: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::TableHLine(Arc::new(TableHLineElem { start, end, row, stroke, position }))
    }
    /// `table_vline(start, end, col, stroke, position)` — Passo 512.
    pub fn table_vline(
        start: usize,
        end: Option<usize>,
        col: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::TableVLine(Arc::new(TableVLineElem { start, end, col, stroke, position }))
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

    /// `bibliography(entries, title)` — Passo 159A (par acoplado
    /// com `cite`). Subset minimal per ADR-0054 graded; input
    /// cristalino literal `Vec<BibEntry>`; sem hayagriva.
    pub fn bibliography(
        entries: Vec<crate::entities::bib_entry::BibEntry>,
        title: Option<Content>,
    ) -> Self {
        Self::Bibliography(Arc::new(BibliographyElem {
            entries,
            path: None,
            title,
            style: None,
            locale: None,
        }))
    }

    /// **P418** — `bibliography(entries, title, style, locale)` com CSL.
    pub fn bibliography_with_style(
        entries: Vec<crate::entities::bib_entry::BibEntry>,
        title: Option<Content>,
        style: Option<EcoString>,
        locale: Option<EcoString>,
    ) -> Self {
        Self::Bibliography(Arc::new(BibliographyElem {
            entries,
            path: None,
            title,
            style,
            locale,
        }))
    }

    /// **P419** — `bibliography(path, title, style, locale)` carregado de disco.
    pub fn bibliography_from_path(
        path: impl Into<EcoString>,
        title: Option<Content>,
        style: Option<EcoString>,
        locale: Option<EcoString>,
    ) -> Self {
        Self::Bibliography(Arc::new(BibliographyElem {
            entries: Vec::new(),
            path: Some(path.into()),
            title,
            style,
            locale,
        }))
    }

    /// `cite(key, supplement, form, style)` — Passo 159A (par acoplado com
    /// `bibliography`) + Passo 159C (form variants) + P468 (style).
    /// Sem validação cross-reference (ADR-0017 Introspection runtime adiada).
    pub fn cite(
        key: impl Into<String>,
        supplement: Option<Content>,
        form: Option<crate::entities::citation_form::CitationForm>,
    ) -> Self {
        Self::cite_with_style(key, supplement, form, None)
    }

    /// **P468** — constructor de `Content::Cite` com estilo explícito.
    pub fn cite_with_style(
        key: impl Into<String>,
        supplement: Option<Content>,
        form: Option<crate::entities::citation_form::CitationForm>,
        style: Option<crate::entities::citation_style::CitationStyle>,
    ) -> Self {
        Self::Cite(Arc::new(CiteElem { key: key.into(), supplement, form, style }))
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
