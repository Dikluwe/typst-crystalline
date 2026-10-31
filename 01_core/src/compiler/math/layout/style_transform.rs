//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/style_transform.md
//! @prompt-hash f03321a1
//! @layer L1
//! @updated 2026-10-07

use super::*;

// ── Passo 311b.4 — `apply_math_style` helper ─────────────────────────────
//
// Aplica recursivamente o variant glyph + flags (bold/italic) aos chars
// do body. P1291: no mesmo eixo, o setter interno ganha; eixos distintos
// continuam a compor. Para sub-arvores não-`MathStyled`
// (MathFrac/MathSequence/etc.), propaga o contexto recursivamente.

/// **P812** — aplicação do itálico matemático **por defeito** (P809) no topo
/// de `layout_equation`. Percorre a árvore mapeando folhas de 1 carácter
/// (`is_math_italic_default`) para o codepoint math italic, mas **preserva
/// os nós `MathStyled` intocáveis** — a composição de estilos explícitos
/// (incluindo o default de itálico para `bold(x)`) corre depois no handler
/// `MathStyled` de `layout_node`, que também aplica o factor de tamanho.
/// (Substitui o uso P809 de `apply_math_style` no topo, que consumia o
/// wrapper e destruía display/script/sscript — P812-A.)
fn map_attach_slot(
    slot: &MathAttachSlot,
    map: impl FnOnce(&Content) -> Content,
) -> MathAttachSlot {
    match slot {
        MathAttachSlot::Omitted => MathAttachSlot::Omitted,
        MathAttachSlot::ExplicitNone => MathAttachSlot::ExplicitNone,
        MathAttachSlot::Present(content) => MathAttachSlot::Present(map(content)),
    }
}

pub(super) fn apply_math_default(body: &Content) -> Content {
    match body {
        Content::Source(e) => apply_math_default(&e.body).with_source_span(e.span),
        Content::MathClassOverride(e) => {
            Content::math_class_override(e.class, apply_math_default(&e.body))
        }
        // Wrapper explícito: não tocar — o handler dedicado trata-o.
        Content::MathStyled(_) => body.clone(),
        // Folhas: default de itálico por codepoint (P809).
        Content::Symbol(text) => Content::Symbol(
            text.chars()
                .map(|c| {
                    if is_math_italic_default(c) {
                        map_glyph(c, MathStyleKind::Plain, false, true)
                    } else {
                        c
                    }
                })
                .collect(),
        ),
        Content::MathIdent(name) => {
            let mut chars = name.chars();
            if matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c))
            {
                let c = name.chars().next().unwrap();
                return Content::MathIdent(
                    map_glyph(c, MathStyleKind::Plain, false, true).into(),
                );
            }
            body.clone()
        }
        Content::MathText(text) => {
            let mut chars = text.chars();
            if matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c))
            {
                let c = text.chars().next().unwrap();
                return Content::MathText(
                    map_glyph(c, MathStyleKind::Plain, false, true).into(),
                );
            }
            body.clone()
        }
        // Containers: recursão (mesma cobertura de `apply_math_style`).
        Content::MathSequence(seq) => {
            let new_seq: Vec<Content> = seq.iter().map(apply_math_default).collect();
            Content::MathSequence(Arc::from(new_seq))
        }
        Content::MathMatrix(e) => Content::math_matrix_complete(
            e.rows
                .iter()
                .map(|row| row.iter().map(apply_math_default).collect())
                .collect(),
            e.delim,
            e.align,
            e.augment.clone(),
            e.row_gap,
            e.column_gap,
            e.explicit,
        ),
        Content::MathCases(e) => Content::math_cases_full(
            e.children.iter().map(apply_math_default).collect(),
            e.delim,
            e.reverse,
            e.gap,
            e.explicit,
        ),
        Content::MathFrac(e) => {
            // `MathText` é texto matemático já resolvido; identificadores
            // sintácticos chegam como `MathIdent` e continuam a receber o
            // itálico por defeito. Não reinterpretar payload textual do carrier.
            let num = match &e.num {
                Content::MathText(_) => e.num.clone(),
                other => apply_math_default(other),
            };
            let den = match &e.den {
                Content::MathText(_) => e.den.clone(),
                other => apply_math_default(other),
            };
            Content::math_frac_full(
                num,
                den,
                e.style,
                e.num_deparenthesized,
                e.denom_deparenthesized,
                e.line,
            )
        }
        Content::MathAttach(e) => Content::math_attach(
            apply_math_default(&e.base),
            map_attach_slot(&e.t, apply_math_default),
            map_attach_slot(&e.b, apply_math_default),
            map_attach_slot(&e.tl, apply_math_default),
            map_attach_slot(&e.bl, apply_math_default),
            map_attach_slot(&e.tr, apply_math_default),
            map_attach_slot(&e.br, apply_math_default),
        ),
        Content::MathRoot(e) => Content::math_root(
            e.index.as_ref().map(apply_math_default),
            apply_math_default(&e.radicand),
            e.index_explicit,
        ),
        Content::MathStretch(e) => {
            Content::math_stretch(apply_math_default(&e.body), e.size, e.size_explicit)
        }
        Content::MathDelimited(e) => {
            Content::math_delimited(e.open, apply_math_default(&e.body), e.close)
        }
        Content::MathLr(e) => {
            Content::math_lr(apply_math_default(&e.body), e.size, e.size_explicit)
        }
        Content::MathMid(e) => Content::math_mid(apply_math_default(&e.body)),
        Content::MathOverbrace(e) => Content::math_overbrace(
            apply_math_default(&e.body),
            e.annotation.as_ref().map(apply_math_default),
            e.annotation_explicit,
        ),
        Content::MathUnderbrace(e) => Content::math_underbrace(
            apply_math_default(&e.body),
            e.annotation.as_ref().map(apply_math_default),
            e.annotation_explicit,
        ),
        Content::MathUnderbracket(e) => Content::math_underbracket(
            apply_math_default(&e.body),
            e.annotation.as_ref().map(apply_math_default),
            e.annotation_explicit,
        ),
        Content::MathUnderparen(e) => Content::math_underparen(
            apply_math_default(&e.body),
            e.annotation.as_ref().map(apply_math_default),
            e.annotation_explicit,
        ),
        Content::MathUndershell(e) => Content::math_undershell(
            apply_math_default(&e.body),
            e.annotation.as_ref().map(apply_math_default),
            e.annotation_explicit,
        ),
        Content::MathOverbracket(e) => Content::math_overbracket(
            apply_math_default(&e.body),
            e.annotation.as_ref().map(apply_math_default),
            e.annotation_explicit,
        ),
        Content::MathOverparen(e) => Content::math_overparen(
            apply_math_default(&e.body),
            e.annotation.as_ref().map(apply_math_default),
            e.annotation_explicit,
        ),
        Content::MathOvershell(e) => Content::math_overshell(
            apply_math_default(&e.body),
            e.annotation.as_ref().map(apply_math_default),
            e.annotation_explicit,
        ),
        // **P961** — achado #5 de P906 + auditoria externa 2026-08-04: a
        // base de acentos (`hat(x)` etc.) nunca recebia o itálico por
        // defeito (caía em `other`). Recursão na base; o `accent` (combining
        // mark) passa inalterado. `MathUnderover`: recursão nos três
        // campos (a anotação de 1 letra também recebe itálico; a peça ⏟ não
        // é letra, inalterada na prática). Ver `_comum.md` §P961.
        Content::MathAccent(e) => Content::math_accent_full(
            apply_math_default(&e.base),
            e.accent,
            e.size,
            e.dotless,
            e.explicit,
        ),
        Content::MathUnderover(e) => Content::math_underover(
            apply_math_default(&e.base),
            e.under.as_ref().map(apply_math_default),
            e.over.as_ref().map(apply_math_default),
        ),
        Content::MathOp(_) => body.clone(),
        // **P992** — `limits(body)`/`scripts(body)`: o override afecta só o
        // discriminador de `layout_attach` (`is_limits`); o `body` recursa
        // normalmente para receber o itálico por defeito (achado: base de
        // 1 letra `limits(A)` deve continuar 𝐴, não "A" recto).
        Content::MathLimitsOverride(e) => Content::math_limits_override(
            apply_math_default(&e.body),
            e.limits,
            e.inline,
            e.inline_explicit,
        ),
        // **P990-C** — achado §8.1 da auditoria: `cancel(a+b)` e
        // `std.strike(a+b)` renderizavam o corpo em glifo RETO em vez de
        // itálico matemático — nenhum dos dois braços recursava no corpo
        // (caíam no catch-all `other`), mesma família de P961/P966.
        // `MathCancel`: recursão simples no corpo (espelho do braço
        // `MathAccent` de P961). `Strike`: recursão no corpo, `stroke`/
        // `offset`/`extent` preservados sem alteração. Ver `_comum.md`
        // §P990-C.
        Content::MathCancel(e) => Content::math_cancel_full(
            apply_math_default(&e.body),
            e.length,
            e.inverted,
            e.cross,
            e.angle.clone(),
            e.stroke.clone(),
            e.background,
            e.span,
            e.explicit,
        ),
        Content::MathUnderline(e) => Content::math_underline(apply_math_default(&e.body)),
        Content::MathOverline(e) => Content::math_overline(apply_math_default(&e.body)),
        Content::MathVec(e) => Content::math_vec_full(
            e.children.iter().map(apply_math_default).collect(),
            e.delim,
            e.align,
            e.gap,
            e.explicit,
        ),
        Content::Strike(e) => Content::strike(
            apply_math_default(&e.body),
            e.stroke.clone(),
            e.offset,
            e.extent,
        ),
        Content::Overline(e) => Content::overline(
            apply_math_default(&e.body),
            e.stroke.clone(),
            e.offset,
            e.extent,
        ),
        Content::Underline(e) => Content::underline(
            apply_math_default(&e.body),
            e.stroke.clone(),
            e.offset,
            e.extent,
        ),
        // **P966** — conteúdo de função de utilizador dentro de math chega
        // como containers de markup (`Sequence`/`Styled`) com folhas mistas
        // `Text`/`MathText`. Recursão nos filhos/corpo para as folhas
        // `MathText` de 1 carácter receberem o default de itálico; as folhas
        // `Text` (literais) caem no catch-all, inalteradas (paridade com o
        // `resolve_text` do vanilla). Ver `_comum.md` §P966.
        Content::Sequence(items) => {
            let new_items: Vec<Content> = items.iter().map(apply_math_default).collect();
            Content::Sequence(Arc::from(new_items))
        }
        Content::Styled(styled) => {
            Content::styled(apply_math_default(&styled.body), styled.styles.clone())
        }
        other => other.clone(),
    }
}

pub(super) fn apply_math_style(
    body: &Content,
    kind: Option<MathStyleKind>,
    bold: Option<bool>,
    italic: Option<bool>,
) -> Content {
    match body {
        Content::Source(e) => {
            apply_math_style(&e.body, kind, bold, italic).with_source_span(e.span)
        }
        Content::MathClassOverride(e) => Content::math_class_override(
            e.class,
            apply_math_style(&e.body, kind, bold, italic),
        ),
        // P1291 — setters internos vencem no mesmo eixo. Tamanho e cramped
        // não pertencem à transformação de glifo: preserva-se um wrapper
        // mínimo para o handler aplicar esses eixos no subtree correcto.
        Content::MathStyled(m) => {
            let outer_glyph = kind.filter(|k| !k.is_size_variant());
            let inner_glyph = m.kind.filter(|k| !k.is_size_variant());
            let transformed = apply_math_style(
                &m.body,
                inner_glyph.or(outer_glyph),
                m.bold.or(bold),
                m.italic.or(italic),
            );
            let size_kind = m.kind.filter(|k| k.is_size_variant());
            if size_kind.is_some() || m.cramped.is_some() {
                Content::math_styled(size_kind, None, None, transformed, m.cramped)
            } else {
                transformed
            }
        }
        // Folha textual: aplica map_glyph char-by-char.
        Content::Symbol(text) => {
            let k = kind.unwrap_or(MathStyleKind::Plain);
            let glyph_kind = if k.is_size_variant() { MathStyleKind::Plain } else { k };
            let mapped = text
                .chars()
                .flat_map(|c| {
                    let glyph = map_glyph(
                        c,
                        glyph_kind,
                        bold.unwrap_or(false),
                        italic.unwrap_or_else(|| is_math_italic_default(c)),
                    );
                    std::iter::once(glyph).chain(map_glyph_vs(c, glyph_kind))
                })
                .collect();
            Content::Symbol(mapped)
        }
        Content::MathIdent(name) => {
            // **P812** — variants de tamanho (Display/Inline/Script/SScript)
            // não têm mapping de glifo próprio: tratam-se como Plain no eixo
            // glifo (o itálico por defeito atravessa o wrapper de tamanho —
            // `$script(x)$` → 𝑥, medido no vanilla).
            let k = kind.unwrap_or(MathStyleKind::Plain);
            let k_glyph = if k.is_size_variant() { MathStyleKind::Plain } else { k };
            let b = bold.unwrap_or(false);
            // **P809** — default de italic: `is_math_italic_default` para
            // folhas de 1 carácter (paridade codex `MathStyle::select`,
            // medida: `bold(x)` → bold-italic U+1D499). `italic: Some(_)`
            // explícito prevalece (upright continua plain).
            let i = italic.unwrap_or_else(|| {
                let mut chars = name.chars();
                matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c))
            });
            let new: EcoString = name
                .chars()
                .flat_map(|c| {
                    let mapped = map_glyph(c, k_glyph, b, i);
                    // P812-C — variation selector de cal/scr (codex: VS após
                    // cada letra latina mapeada).
                    match map_glyph_vs(c, k_glyph) {
                        Some(vs) => [mapped, vs],
                        None => [mapped, '\0'],
                    }
                })
                .filter(|c| *c != '\0')
                .collect();
            Content::MathIdent(new)
        }
        Content::MathText(text) => {
            let k = kind.unwrap_or(MathStyleKind::Plain);
            let k_glyph = if k.is_size_variant() { MathStyleKind::Plain } else { k };
            let b = bold.unwrap_or(false);
            let i = italic.unwrap_or_else(|| {
                let mut chars = text.chars();
                matches!((chars.next(), chars.next()), (Some(c), None) if is_math_italic_default(c))
            });
            let new: EcoString = text
                .chars()
                .flat_map(|c| {
                    let mapped = map_glyph(c, k_glyph, b, i);
                    match map_glyph_vs(c, k_glyph) {
                        Some(vs) => [mapped, vs],
                        None => [mapped, '\0'],
                    }
                })
                .filter(|c| *c != '\0')
                .collect();
            Content::MathText(new)
        }
        // Containers math: propaga context.
        Content::MathSequence(seq) => {
            let new_seq: Vec<Content> =
                seq.iter().map(|c| apply_math_style(c, kind, bold, italic)).collect();
            Content::MathSequence(Arc::from(new_seq))
        }
        // **P809** — matrix/cases: recursão por célula (estavam no braço
        // `other` — as células não eram estilizadas, ex.: a,b,c,d plain).
        Content::MathMatrix(e) => Content::MathMatrix(e.clone()),
        Content::MathCases(e) => Content::math_cases_full(
            e.children
                .iter()
                .map(|child| apply_math_style(child, kind, bold, italic))
                .collect(),
            e.delim,
            e.reverse,
            e.gap,
            e.explicit,
        ),
        // Modelo D (Lote 2 P317): destructuring de `Arc<…Elem>` + reconstrução
        // via construtor ergonómico — mesma lógica recursiva.
        Content::MathFrac(e) => {
            let num = apply_math_style(&e.num, kind, bold, italic);
            let den = apply_math_style(&e.den, kind, bold, italic);
            Content::math_frac_full(
                num,
                den,
                e.style,
                e.num_deparenthesized,
                e.denom_deparenthesized,
                e.line,
            )
        }
        Content::MathAttach(e) => Content::math_attach(
            apply_math_style(&e.base, kind, bold, italic),
            map_attach_slot(&e.t, |c| apply_math_style(c, kind, bold, italic)),
            map_attach_slot(&e.b, |c| apply_math_style(c, kind, bold, italic)),
            map_attach_slot(&e.tl, |c| apply_math_style(c, kind, bold, italic)),
            map_attach_slot(&e.bl, |c| apply_math_style(c, kind, bold, italic)),
            map_attach_slot(&e.tr, |c| apply_math_style(c, kind, bold, italic)),
            map_attach_slot(&e.br, |c| apply_math_style(c, kind, bold, italic)),
        ),
        Content::MathRoot(e) => Content::math_root(
            e.index.as_ref().map(|c| apply_math_style(c, kind, bold, italic)),
            apply_math_style(&e.radicand, kind, bold, italic),
            e.index_explicit,
        ),
        Content::MathStretch(e) => Content::math_stretch(
            apply_math_style(&e.body, kind, bold, italic),
            e.size,
            e.size_explicit,
        ),
        Content::MathDelimited(e) => Content::math_delimited(
            e.open,
            apply_math_style(&e.body, kind, bold, italic),
            e.close,
        ),
        Content::MathLr(e) => Content::math_lr(
            apply_math_style(&e.body, kind, bold, italic),
            e.size,
            e.size_explicit,
        ),
        Content::MathMid(e) => {
            Content::math_mid(apply_math_style(&e.body, kind, bold, italic))
        }
        Content::MathOverbrace(e) => Content::math_overbrace(
            apply_math_style(&e.body, kind, bold, italic),
            e.annotation
                .as_ref()
                .map(|annotation| apply_math_style(annotation, kind, bold, italic)),
            e.annotation_explicit,
        ),
        Content::MathUnderbrace(e) => Content::math_underbrace(
            apply_math_style(&e.body, kind, bold, italic),
            e.annotation
                .as_ref()
                .map(|annotation| apply_math_style(annotation, kind, bold, italic)),
            e.annotation_explicit,
        ),
        Content::MathUnderbracket(e) => Content::math_underbracket(
            apply_math_style(&e.body, kind, bold, italic),
            e.annotation
                .as_ref()
                .map(|annotation| apply_math_style(annotation, kind, bold, italic)),
            e.annotation_explicit,
        ),
        Content::MathUnderparen(e) => Content::math_underparen(
            apply_math_style(&e.body, kind, bold, italic),
            e.annotation
                .as_ref()
                .map(|annotation| apply_math_style(annotation, kind, bold, italic)),
            e.annotation_explicit,
        ),
        Content::MathUndershell(e) => Content::math_undershell(
            apply_math_style(&e.body, kind, bold, italic),
            e.annotation
                .as_ref()
                .map(|annotation| apply_math_style(annotation, kind, bold, italic)),
            e.annotation_explicit,
        ),
        Content::MathOverbracket(e) => Content::math_overbracket(
            apply_math_style(&e.body, kind, bold, italic),
            e.annotation
                .as_ref()
                .map(|annotation| apply_math_style(annotation, kind, bold, italic)),
            e.annotation_explicit,
        ),
        Content::MathOverparen(e) => Content::math_overparen(
            apply_math_style(&e.body, kind, bold, italic),
            e.annotation
                .as_ref()
                .map(|annotation| apply_math_style(annotation, kind, bold, italic)),
            e.annotation_explicit,
        ),
        Content::MathOvershell(e) => Content::math_overshell(
            apply_math_style(&e.body, kind, bold, italic),
            e.annotation
                .as_ref()
                .map(|annotation| apply_math_style(annotation, kind, bold, italic)),
            e.annotation_explicit,
        ),
        Content::MathAccent(e) => Content::math_accent_full(
            apply_math_style(&e.base, kind, bold, italic),
            e.accent,
            e.size,
            e.dotless,
            e.explicit,
        ),
        Content::MathUnderline(e) => {
            Content::math_underline(apply_math_style(&e.body, kind, bold, italic))
        }
        Content::MathOverline(e) => {
            Content::math_overline(apply_math_style(&e.body, kind, bold, italic))
        }
        Content::MathVec(e) => Content::math_vec_full(
            e.children
                .iter()
                .map(|child| apply_math_style(child, kind, bold, italic))
                .collect(),
            e.delim,
            e.align,
            e.gap,
            e.explicit,
        ),
        // `MathOp` (operadores texto) passa-through — variant não aplica.
        // Operadores como "sin"/"lim" mantêm aparência normal mesmo dentro
        // de `bb(...)` (paridade vanilla).
        Content::MathOp(_) => body.clone(),
        // Outros chars / variants não-math: clone literal.
        other => other.clone(),
    }
}

#[cfg(test)]
mod symbol_identity_tests {
    use super::*;

    #[test]
    fn symbol_identity_multichar_default_glyphs() {
        let body = Content::Symbol("sum".into());
        let Content::Symbol(text) = apply_math_default(&body) else {
            panic!("native variant lost")
        };
        assert_eq!(text.as_str(), "𝑠𝑢𝑚");
        let Content::Symbol(text) = apply_math_default(&Content::Symbol("x︀".into()))
        else {
            panic!("native variant lost")
        };
        assert_eq!(text.as_str(), "𝑥︀");
    }

    #[test]
    fn symbol_identity_multichar_explicit_glyphs() {
        let body = Content::Symbol("xy".into());
        let Content::Symbol(text) = apply_math_style(&body, None, Some(true), None)
        else {
            panic!("native variant lost")
        };
        assert_eq!(text.as_str(), "𝒙𝒚");
        let Content::Symbol(text) = apply_math_style(&body, None, None, Some(false))
        else {
            panic!("native variant lost")
        };
        assert_eq!(text.as_str(), "xy");
    }
}

#[cfg(test)]
mod readiness_math_tests {
    use super::*;
    use crate::entities::math_class::MathClass;
    fn body_symbol(content: &Content) -> &str {
        let Content::MathClassOverride(e) = content else { panic!("class carrier lost") };
        assert_eq!(e.class, MathClass::Relation);
        let Content::Symbol(text) = &e.body else { panic!("symbol body lost") };
        text.as_str()
    }
    #[test]
    fn readiness_math_class_default_reaches_math_symbol() {
        let c = Content::math_class_override(
            MathClass::Relation,
            Content::Symbol("x".into()),
        );
        assert_eq!(body_symbol(&apply_math_default(&c)), "𝑥");
    }
    #[test]
    fn readiness_math_class_outer_bold_reaches_math_symbol() {
        let c = Content::math_class_override(
            MathClass::Relation,
            Content::Symbol("x".into()),
        );
        assert_eq!(body_symbol(&apply_math_style(&c, None, Some(true), None)), "𝒙");
        assert_eq!(body_symbol(&apply_math_style(&c, None, None, Some(false))), "x");
    }
    #[test]
    fn readiness_math_class_nested_and_inner_upright() {
        let c = Content::math_class_override(
            MathClass::Binary,
            Content::math_class_override(
                MathClass::Relation,
                Content::Symbol("x".into()),
            ),
        );
        let Content::MathClassOverride(outer) = apply_math_default(&c) else {
            panic!("outer lost")
        };
        assert_eq!(outer.class, MathClass::Binary);
        assert_eq!(body_symbol(&outer.body), "𝑥");
        let inner = Content::math_styled(
            None,
            None,
            Some(false),
            Content::Symbol("x".into()),
            None,
        );
        let c = Content::math_class_override(MathClass::Relation, inner);
        assert_eq!(body_symbol(&apply_math_style(&c, None, Some(true), None)), "𝐱");
    }
    #[test]
    fn readiness_math_class_literal_text_keeps_normal_default() {
        let c = Content::math_class_override(MathClass::Relation, Content::text("x"));
        let Content::MathClassOverride(e) = apply_math_default(&c) else {
            panic!("class lost")
        };
        assert!(matches!(&e.body,Content::Text(text) if text=="x"));
    }
}
