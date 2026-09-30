//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/repr/content.md
//! @prompt-hash de5b106d
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Representação de um `Content`.
///
/// **P843 (F2)** — formato estrutural do vanilla para os variantes
/// centrais (medido em `temp/p843/f2_*.typ`): Text → `[texto]` (cru, sem
/// aspas — `TextElem::repr`), Space → `[ ]`, vazio/sequência vazia → `[]`,
/// sequência → `sequence(...)` com [`pretty_array_like`], Strong/Emph →
/// `strong(body: ...)` / `emph(body: ...)`. Os restantes variantes mantêm a
/// forma cristalina prévia (scope-out documentado no relatório do passo).
pub fn repr_content(c: &Content) -> String {
    match c {
        Content::Empty => "[]".to_string(),
        Content::Text(t) => format!("[{}]", t.as_str()),
        Content::Space | Content::SpaceFromNewline => "[ ]".to_string(),
        Content::Parbreak => "parbreak()".to_string(),
        Content::Par { body } => format!("par(body: {})", repr_content(body)),
        Content::HtmlElem(elem) => {
            let mut fields = vec![format!("tag: {:?}", elem.tag.as_str())];
            if let Some(attrs) = &elem.attrs {
                let attrs = attrs
                    .iter()
                    .map(|(key, value)| format!("{}: {:?}", key, value.as_str()))
                    .collect::<Vec<_>>();
                fields.push(format!("attrs: {}", pretty_array_like(&attrs, false)));
            }
            match &elem.body {
                crate::entities::html::HtmlBody::Unset => {}
                crate::entities::html::HtmlBody::None => fields.push("body: none".into()),
                crate::entities::html::HtmlBody::Content(body) => {
                    fields.push(format!("body: {}", repr_content(body)));
                }
            }
            format!("elem{}", pretty_array_like(&fields, false))
        }
        Content::HtmlFrame(elem) => {
            format!("frame(body: {})", repr_content(&elem.body))
        }
        Content::PageRun(e) => {
            let sequence = format!(
                "sequence(\n  pagebreak(weak: true),\n  flush(),\n  {},\n  pagebreak(weak: true),\n)",
                repr_content(&e.body)
            );
            let styled = e.paper.is_some()
                || e.flipped.is_some()
                || e.binding.is_some()
                || e.width.is_some()
                || e.height.is_some()
                || e.margin.is_some()
                || e.numbering.is_some()
                || e.number_align.is_some()
                || e.header.is_some()
                || e.header_ascent.is_some()
                || e.footer.is_some()
                || e.footer_descent.is_some()
                || e.supplement.is_some()
                || e.columns.is_some()
                || e.bleed.is_some()
                || e.fill.is_some()
                || e.background.is_some()
                || e.foreground.is_some();
            if styled {
                format!("styled(child: {sequence}, ..)")
            } else {
                sequence
            }
        }
        Content::Sequence(seq) => {
            if seq.is_empty() {
                return "[]".to_string();
            }
            let parts: Vec<String> = seq.iter().map(repr_content).collect();
            format!("sequence{}", pretty_array_like(&parts, false))
        }
        Content::Heading(h) => {
            format!("heading(level: {})[{}]", h.level, repr_content(&h.body))
        }
        Content::Title(t) => format!("title[{}]", repr_content(&t.body)),
        Content::Strong(s) => format!("strong(body: {})", repr_content(&s.body)),
        Content::Emph(e) => format!("emph(body: {})", repr_content(&e.body)),
        Content::Raw(r) => format!("`{}`", r.text),
        Content::ListItem(li) => format!("- {}", repr_content(&li.body)),
        Content::EnumItem(ei) => format!("+ {}", repr_content(&ei.body)),
        Content::Link(l) => format!("link(\"{}\")", l.url),
        Content::Equation(e) => {
            if e.block {
                format!("equation(block: true, body: {})", repr_content(&e.body))
            } else {
                format!("equation(body: {})", repr_content(&e.body))
            }
        }
        Content::MathSequence(seq) => seq.iter().map(repr_content).collect(),
        Content::MathIdent(s) => s.to_string(),
        Content::MathText(s) => format!("\"{}\"", s.as_str()),
        Content::MathFrac(f) => {
            if !f.line {
                format!("({})/({})", repr_content(&f.num), repr_content(&f.den))
            } else {
                use crate::entities::elements::math_frac::MathFracStyle;
                let mut fields = vec![
                    format!("num: {}", repr_frac_field(&f.num)),
                    format!("denom: {}", repr_frac_field(&f.den)),
                ];
                if let Some(style) = f.style {
                    let style = match style {
                        MathFracStyle::Vertical => "vertical",
                        MathFracStyle::Skewed => "skewed",
                        MathFracStyle::Horizontal => "horizontal",
                    };
                    fields.push(format!("style: \"{style}\""));
                }
                format!("frac({})", fields.join(", "))
            }
        }
        Content::MathAttach(a) => repr_math_attach(a),
        Content::MathPrimes(p) => format!("primes(count: {})", p.count),
        Content::MathRoot(r) => match (&r.index, r.index_explicit) {
            (Some(index), true) => format!(
                "root(index: {}, radicand: {})",
                repr_p1293_math_field(index),
                repr_p1293_math_field(&r.radicand)
            ),
            (None, true) => {
                format!(
                    "root(index: none, radicand: {})",
                    repr_p1293_math_field(&r.radicand)
                )
            }
            (_, false) => format!("root(radicand: {})", repr_content(&r.radicand)),
        },
        Content::MathStretch(stretch) => {
            let mut fields = vec![format!("body: {}", repr_math_lr_body(&stretch.body))];
            if stretch.size_explicit {
                fields.push(format!("size: {}", repr_math_lr_size(&stretch.size)));
            }
            format!("stretch({})", fields.join(", "))
        }
        Content::MathDelimited(d) => repr_canonical_binom(d)
            .unwrap_or_else(|| format!("{}{}{}", d.open, repr_content(&d.body), d.close)),
        Content::MathLr(lr) => {
            let mut fields = Vec::with_capacity(2);
            if lr.size_explicit {
                fields.push(format!("size: {}", repr_math_lr_size(&lr.size)));
            }
            fields.push(format!("body: {}", repr_math_lr_body(&lr.body)));
            format!("lr({})", fields.join(", "))
        }
        Content::MathMid(mid) => {
            format!("mid(body: {})", repr_math_lr_body(&mid.body))
        }
        Content::MathOverbrace(overbrace) => {
            let mut fields =
                vec![format!("body: {}", repr_math_lr_body(&overbrace.body))];
            if overbrace.annotation_explicit {
                let value = overbrace
                    .annotation
                    .as_ref()
                    .map(repr_math_lr_body)
                    .unwrap_or_else(|| "none".to_string());
                fields.push(format!("annotation: {value}"));
            }
            format!("overbrace({})", fields.join(", "))
        }
        Content::MathUnderbrace(underbrace) => {
            let mut fields =
                vec![format!("body: {}", repr_math_lr_body(&underbrace.body))];
            if underbrace.annotation_explicit {
                let value = underbrace
                    .annotation
                    .as_ref()
                    .map(repr_math_lr_body)
                    .unwrap_or_else(|| "none".to_string());
                fields.push(format!("annotation: {value}"));
            }
            format!("underbrace({})", fields.join(", "))
        }
        Content::MathUnderbracket(underbracket) => {
            let mut fields =
                vec![format!("body: {}", repr_math_lr_body(&underbracket.body))];
            if underbracket.annotation_explicit {
                let value = underbracket
                    .annotation
                    .as_ref()
                    .map(repr_math_lr_body)
                    .unwrap_or_else(|| "none".to_string());
                fields.push(format!("annotation: {value}"));
            }
            format!("underbracket({})", fields.join(", "))
        }
        Content::MathUnderparen(underparen) => {
            let mut fields =
                vec![format!("body: {}", repr_math_lr_body(&underparen.body))];
            if underparen.annotation_explicit {
                let value = underparen
                    .annotation
                    .as_ref()
                    .map(repr_math_lr_body)
                    .unwrap_or_else(|| "none".to_string());
                fields.push(format!("annotation: {value}"));
            }
            format!("underparen({})", fields.join(", "))
        }
        Content::MathUndershell(undershell) => {
            let mut fields =
                vec![format!("body: {}", repr_math_lr_body(&undershell.body))];
            if undershell.annotation_explicit {
                let value = undershell
                    .annotation
                    .as_ref()
                    .map(repr_math_lr_body)
                    .unwrap_or_else(|| "none".to_string());
                fields.push(format!("annotation: {value}"));
            }
            format!("undershell({})", fields.join(", "))
        }
        Content::MathOverbracket(overbracket) => {
            let mut fields =
                vec![format!("body: {}", repr_math_lr_body(&overbracket.body))];
            if overbracket.annotation_explicit {
                let value = overbracket
                    .annotation
                    .as_ref()
                    .map(repr_math_lr_body)
                    .unwrap_or_else(|| "none".to_string());
                fields.push(format!("annotation: {value}"));
            }
            format!("overbracket({})", fields.join(", "))
        }
        Content::MathOverparen(overparen) => {
            let mut fields =
                vec![format!("body: {}", repr_math_lr_body(&overparen.body))];
            if overparen.annotation_explicit {
                let value = overparen
                    .annotation
                    .as_ref()
                    .map(repr_math_lr_body)
                    .unwrap_or_else(|| "none".to_string());
                fields.push(format!("annotation: {value}"));
            }
            format!("overparen({})", fields.join(", "))
        }
        Content::MathOvershell(overshell) => {
            let mut fields =
                vec![format!("body: {}", repr_math_lr_body(&overshell.body))];
            if overshell.annotation_explicit {
                let value = overshell
                    .annotation
                    .as_ref()
                    .map(repr_math_lr_body)
                    .unwrap_or_else(|| "none".to_string());
                fields.push(format!("annotation: {value}"));
            }
            format!("overshell({})", fields.join(", "))
        }
        Content::MathAlignPoint(_) => "&".to_string(),
        Content::Linebreak(e) => {
            if e.justify_explicit {
                format!("linebreak(justify: {})", e.justify)
            } else {
                "linebreak()".to_string()
            }
        }
        Content::MathMatrix(matrix) => repr_math_matrix(matrix),
        Content::MathCases(cases) => {
            fn repr_cases_child(content: &Content) -> String {
                match content {
                    Content::MathIdent(text)
                    | Content::MathText(text)
                    | Content::Text(text) => text.to_string(),
                    Content::MathAlignPoint(_) => " & ".to_string(),
                    Content::MathSequence(nodes) => {
                        nodes.iter().map(repr_cases_child).collect::<String>()
                    }
                    other => repr_content(other),
                }
            }

            let mut fields = Vec::new();
            if cases.explicit.delim {
                let repr_delim = |value: Option<char>| {
                    value.map_or_else(
                        || "none".to_string(),
                        |value| format!("\"{}\"", value.escape_default()),
                    )
                };
                fields.push(format!(
                    "delim: ({}, {})",
                    repr_delim(cases.delim.0),
                    repr_delim(cases.delim.1)
                ));
            }
            if cases.explicit.reverse {
                fields.push(format!("reverse: {}", cases.reverse));
            }
            if cases.explicit.gap {
                fields.push(format!(
                    "gap: {} + {}",
                    format_float_with_unit(cases.gap.rel * 100.0, "%"),
                    repr_length(&cases.gap.abs)
                ));
            }
            let children = cases
                .children
                .iter()
                .map(|child| match child {
                    Content::MathIdent(_) | Content::MathText(_) => {
                        repr_cases_child(child)
                    }
                    Content::MathSequence(_) => repr_cases_child(child),
                    other => repr_content(other),
                })
                .collect::<Vec<_>>();
            fields.push(format!(
                "children: {}",
                pretty_array_like(&children, children.len() == 1)
            ));
            format!("cases({})", fields.join(", "))
        }
        Content::MathAccent(a) => {
            let mut fields = vec![
                format!("base: {}", repr_content(&a.base)),
                format!(
                    "accent: {}",
                    repr_value(&Value::Str(a.accent.to_string().into()))
                ),
            ];
            if a.explicit.size {
                fields.push(format!(
                    "size: {} + {}",
                    format_float_with_unit(a.size.rel * 100.0, "%"),
                    repr_length(&a.size.abs)
                ));
            }
            if a.explicit.dotless {
                fields.push(format!("dotless: {}", a.dotless));
            }
            format!("accent({})", fields.join(", "))
        }
        Content::MathCancel(c) => {
            let mut fields = vec![format!("body: {}", repr_content(&c.body))];
            if c.explicit.length {
                fields.push(format!("length: {}", repr_relative(&c.length)));
            }
            if c.explicit.inverted {
                fields.push(format!("inverted: {}", c.inverted));
            }
            if c.explicit.cross {
                fields.push(format!("cross: {}", c.cross));
            }
            if c.explicit.angle {
                let angle = match &c.angle {
                    crate::entities::elements::math_cancel::MathCancelAngle::Auto => {
                        "auto".to_string()
                    }
                    crate::entities::elements::math_cancel::MathCancelAngle::Angle(
                        value,
                    ) => repr_angle(*value),
                    crate::entities::elements::math_cancel::MathCancelAngle::Func(
                        value,
                    ) => repr_value(&Value::Func(value.clone())),
                };
                fields.push(format!("angle: {angle}"));
            }
            if c.explicit.stroke {
                let stroke =
                    c.stroke.as_ref().map_or_else(|| "none".to_string(), repr_stroke);
                fields.push(format!("stroke: {stroke}"));
            }
            if c.explicit.background {
                fields.push(format!("background: {}", c.background));
            }
            format!("cancel{}", pretty_array_like(&fields, false))
        }
        Content::MathUnderline(u) => {
            format!("underline(body: {})", repr_content(&u.body))
        }
        Content::MathOverline(o) => {
            format!("overline(body: {})", repr_math_lr_body(&o.body))
        }
        Content::MathVec(v) => {
            let mut fields = Vec::new();
            if v.explicit.delim {
                let repr_delim = |value: char| {
                    if value == '\0' {
                        "none".to_string()
                    } else {
                        format!("\"{}\"", value.escape_default())
                    }
                };
                fields.push(format!(
                    "delim: ({}, {})",
                    repr_delim(v.delim.0),
                    repr_delim(v.delim.1)
                ));
            }
            if v.explicit.align {
                let align = match v.align {
                    HAlign::Left => "left",
                    HAlign::Center => "center",
                    HAlign::Right => "right",
                    HAlign::Start => "start",
                    HAlign::End => "end",
                };
                fields.push(format!("align: {align}"));
            }
            if v.explicit.gap {
                fields.push(format!(
                    "gap: {} + {}",
                    format_float_with_unit(v.gap.rel * 100.0, "%"),
                    repr_length(&v.gap.abs)
                ));
            }
            let children = v
                .children
                .iter()
                .map(|child| match child {
                    // P1292-C v7: uma folha math direta continua armazenada
                    // como math (logo layout/igualdade não mudam), mas sua
                    // projeção pública é a de Content textual `[x]`.
                    Content::MathIdent(_) | Content::MathText(_) => {
                        repr_p1293_math_field(child)
                    }
                    // Estruturas e markup conservam o formatter próprio; a
                    // projeção estreita não recursa nos seus descendentes.
                    other => repr_content(other),
                })
                .collect::<Vec<_>>();
            fields.push(format!(
                "children: {}",
                pretty_array_like(&children, children.len() == 1)
            ));
            format!("vec{}", pretty_array_like(&fields, false))
        }
        Content::MathClassOverride(c) => {
            let fields = [
                format!(
                    "class: {:?}",
                    crate::entities::math_class::math_class_name(c.class)
                ),
                format!("body: {}", repr_content(&c.body)),
            ];
            format!("class{}", pretty_array_like(&fields, false))
        }
        Content::MathLimitsOverride(o) => {
            if o.limits {
                let mut fields =
                    vec![format!("body: {}", repr_math_limits_body(&o.body))];
                if o.inline_explicit {
                    fields.push(format!("inline: {}", o.inline));
                }
                format!("limits{}", pretty_array_like(&fields, false))
            } else {
                format!("scripts(body: {})", repr_math_limits_body(&o.body))
            }
        }
        Content::MathUnderover(u) => {
            let mut out = repr_content(&u.base);
            if let Some(under) = &u.under {
                out.push_str(&format!("under({})", repr_content(under)));
            }
            if let Some(over) = &u.over {
                out.push_str(&format!("over({})", repr_content(over)));
            }
            out
        }
        Content::MathOp(o) => {
            let fields = [
                format!("text: {}", repr_content(&o.text)),
                format!("limits: {}", o.limits),
            ];
            format!("op{}", pretty_array_like(&fields, false))
        }
        Content::MathStyled(s) => {
            if s.kind.is_some()
                || s.bold.is_some()
                || s.italic.is_some()
                || s.cramped.is_some()
            {
                format!("styled(child: {}, ..)", repr_p1293_math_field(&s.body))
            } else {
                repr_content(&s.body)
            }
        }
        Content::Label(l) if l.auto => format!("label(\"{}\")", l.name),
        Content::Label(l) => format!("label(\"{}\", {})", l.name, repr_content(&l.body)),
        Content::Ref(r) => format!("ref(<{}>)", r.name),
        Content::CounterDisplay(_) => "counter.display".to_string(),
        Content::CounterUpdate(_) => "counter.update".to_string(),
        Content::Outline(_) => "outline".to_string(),
        Content::OutlineEntry(e) => {
            let mut fields = vec![
                format!("level: {}", e.level),
                format!("element: {}", repr_content(&e.element)),
            ];
            if e.fill_set {
                fields.push(format!(
                    "fill: {}",
                    e.fill.as_ref().map(repr_content).unwrap_or_else(|| "none".into())
                ));
            }
            format!("entry{}", pretty_array_like(&fields, false))
        }
        Content::Figure(f) => format!("figure[{}]", repr_content(&f.body)),
        // P1473 — mesmo estilo simples de `Figure` (sem síntese nesta fatia).
        Content::FigureCaption(fc) => {
            format!("figure.caption[{}]", repr_content(&fc.body))
        }
        Content::Image(i) => format!("image(\"{}\")", i.path),
        Content::Shape(_) => "shape".to_string(),
        Content::Curve(_) => "curve".to_string(),
        Content::Transform(t) => format!("transform[{}]", repr_content(&t.body)),
        Content::Grid(_) => "grid".to_string(),
        Content::GridHeader(_) => "grid.header".to_string(),
        Content::GridFooter(_) => "grid.footer".to_string(),
        Content::GridCell(_) => "grid.cell".to_string(),
        Content::GridHLine(_) => "grid.hline".to_string(),
        Content::GridVLine(_) => "grid.vline".to_string(),
        Content::Align(a) => format!("align({:?})[...]", a.alignment),
        Content::Place(p) => format!("place({:?})[...]", p.scope),
        Content::Flush(_) => "flush()".to_string(),
        Content::Styled(styled) => {
            let child = &styled.body;
            let styles = &styled.styles;
            if let Content::Equation(e) = child {
                let numbering = styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(key, _)| key.as_str() == "equation.numbering")
                    .map(|(_, value)| value);
                let number_align = styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(key, _)| key.as_str() == "equation.number-align")
                    .map(|(_, value)| value);
                let supplement = styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(key, _)| key.as_str() == "equation.supplement")
                    .map(|(_, value)| value);
                let alt = styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(key, _)| key.as_str() == "equation.alt")
                    .map(|(_, value)| value);
                if numbering.is_some()
                    || number_align.is_some()
                    || supplement.is_some()
                    || alt.is_some()
                {
                    let mut fields = Vec::new();
                    if e.block {
                        fields.push("block: true".to_string());
                    }
                    if let Some(numbering) = numbering {
                        fields.push(format!("numbering: {}", repr_value(numbering)));
                    }
                    if let Some(number_align) = number_align {
                        fields
                            .push(format!("number-align: {}", repr_value(number_align)));
                    }
                    if let Some(supplement) = supplement {
                        fields.push(format!("supplement: {}", repr_value(supplement)));
                    }
                    if let Some(alt) = alt {
                        fields.push(format!("alt: {}", repr_value(alt)));
                    }
                    fields.push(format!("body: {}", repr_content(&e.body)));
                    format!("equation({})", fields.join(", "))
                } else {
                    repr_content(child)
                }
            } else {
                repr_content(child)
            }
        }
        Content::Divider(_) => "divider".to_string(),
        Content::Terms(_) => "terms".to_string(),
        Content::TermItem(ti) => {
            format!("/ {}: {}", repr_content(&ti.term), repr_content(&ti.description))
        }
        Content::Quote(q) => format!("quote[{}]", repr_content(&q.body)),
        Content::SmartQuote(_) => "\"".to_string(),
        Content::Underline(u) => format!("underline[{}]", repr_content(&u.body)),
        Content::Strike(s) => format!("strike[{}]", repr_content(&s.body)),
        Content::Overline(o) => format!("overline[{}]", repr_content(&o.body)),
        Content::SmallCaps { body } => format!("smallcaps[{}]", repr_content(body)),
        Content::Pad(p) => format!("pad[{}]", repr_content(&p.body)),
        Content::Hide(h) => format!("hide[{}]", repr_content(&h.body)),
        Content::HSpace(h) => {
            use crate::entities::elements::h_space::Spacing;
            let amount = match h.amount {
                Spacing::Absolute(length) => repr_length(&length),
                Spacing::Fractional(fr) => format_float_with_unit(fr, "fr"),
            };
            if h.weak_explicit {
                format!("h(amount: {amount}, weak: {})", h.weak)
            } else {
                format!("h(amount: {amount})")
            }
        }
        Content::VSpace(v) => {
            let amount = repr_length(&v.amount);
            if v.weak_explicit {
                format!("v(amount: {amount}, weak: {})", v.weak)
            } else {
                format!("v(amount: {amount})")
            }
        }
        Content::Pagebreak(p) => {
            let mut fields = Vec::new();
            if p.weak_explicit {
                fields.push(format!("weak: {}", p.weak));
            }
            if let Some(parity) = p.to {
                let parity = match parity {
                    crate::entities::parity::Parity::Even => "even",
                    crate::entities::parity::Parity::Odd => "odd",
                };
                fields.push(format!("to: \"{parity}\""));
            }
            format!("pagebreak({})", fields.join(", "))
        }
        Content::Colbreak(c) => {
            if c.weak_explicit {
                format!("colbreak(weak: {})", c.weak)
            } else {
                "colbreak()".to_string()
            }
        }
        Content::Stack(_) => "stack".to_string(),
        Content::Boxed(b) => match b.width_fraction {
            Some(fr) => format!(
                "box(width: {}, body: {})",
                format_float_with_unit(fr, "fr"),
                repr_content(&b.body)
            ),
            None => format!("box[{}]", repr_content(&b.body)),
        },
        Content::Block(b) => format!("block[{}]", repr_content(&b.body)),
        Content::TableCell(_) => "table.cell".to_string(),
        Content::Bibliography(b) => {
            let path = b.path.as_ref().map(|p| p.as_str()).unwrap_or("");
            format!("bibliography(\"{}\")", path)
        }
        Content::Cite(c) => format!("cite(<{}>)", c.key.as_str()),
        Content::Footnote(f) => format!("footnote[{}]", repr_content(&f.body)),
        // P1473 — mesmo estilo simples de `Footnote` (sem realização nesta fatia).
        Content::FootnoteEntry(fe) => {
            format!("footnote.entry[{}]", repr_content(&fe.note))
        }
        Content::TableHeader(_) => "table.header".to_string(),
        Content::TableFooter(_) => "table.footer".to_string(),
        Content::TableHLine(_) => "table.hline".to_string(),
        Content::TableVLine(_) => "table.vline".to_string(),
        Content::Table(_) => "table".to_string(),
        Content::Repeat(repeat) => {
            let mut fields = vec![format!("body: {}", repr_content(&repeat.body))];
            if let Some(gap) = repeat.gap {
                fields.push(format!("gap: {}", repr_length(&gap)));
            }
            if !repeat.justify {
                fields.push("justify: false".into());
            }
            format!("repeat{}", pretty_array_like(&fields, false))
        }
        Content::Columns(_) => "columns".to_string(),
        Content::Metadata(_) => "metadata".to_string(),
        Content::State(_) => "state".to_string(),
        Content::StateUpdate(_) => "state.update".to_string(),
        Content::StateDisplay(_) => "state.display".to_string(),
        Content::CounterDisplayCallback(_) => "counter.display.callback".to_string(),
        Content::ContextBlock(_) => "context()".to_string(),
        Content::Dynamic(_) => "dynamic".to_string(),
        Content::SetPage(_) => "setpage".to_string(),
        Content::Document { path, body, .. } => {
            format!("document(path: \"{}\", body: {})", path, repr_content(body))
        }
        Content::Asset { path, data } => {
            format!("asset(path: \"{}\", data: bytes({}))", path, data.len())
        }
        Content::PdfAttach(e) => format!("pdf.attach(\"{}\")", e.path),
        Content::PdfArtifact(e) => {
            if e.kind == crate::entities::elements::pdf_artifact::ArtifactKind::Other {
                format!("pdf.artifact[{}]", repr_content(&e.body))
            } else {
                format!(
                    "pdf.artifact(kind: \"{}\")[{}]",
                    e.kind.as_str(),
                    repr_content(&e.body)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_and_text_content_keep_distinct_forms() {
        assert_eq!(repr_content(&Content::Empty), "[]");
        assert_eq!(repr_content(&Content::text("hello")), "[hello]");
    }
}
