//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/construction/math.md
//! @prompt-hash 99218d25
//! @layer L1
//! @updated 2026-09-30

use super::*;

impl Content {
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
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
