//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/selector_matching.md
//! @prompt-hash bb4fa810
//! @layer L1
//! @updated 2026-10-07
//!
//! Matching de selectores de show rule contra conteúdo; conversão de query
//! selector para show selector. Extraído de `compiler/eval/rules.rs` no Passo
//! 1011 conforme ADR-0109 (atomização — forma B, free function no arquivo da
//! unidade).

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::func::Func;
use crate::entities::selector::Selector as QuerySelector;
use crate::entities::show::{NodeKind, Selector};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::{IntrospectedContent, Value};

pub(crate) use super::operators::equality::values_eq;

/// **P417 (M)** — Converte um `entities::selector::Selector` (query)
/// para um `entities::show::Selector` (show rule). P1339 transporta a
/// identidade nativa de `Element` e seu grupo ordenado, além das rotas
/// legadas Kind/Where/And/Or. Outros selectors conservam seu erro anterior.
pub(crate) fn query_selector_to_show_selector(
    sel: QuerySelector,
    span: Span,
) -> SourceResult<Selector> {
    fn kind_to_show(kind: ElementKind) -> Option<Selector> {
        match kind {
            ElementKind::Heading => Some(Selector::NodeKind(NodeKind::Heading)),
            ElementKind::Figure => Some(Selector::NodeKind(NodeKind::Figure)),
            ElementKind::Link => Some(Selector::NodeKind(NodeKind::Link)),
            ElementKind::Raw => Some(Selector::NodeKind(NodeKind::Raw)),
            ElementKind::Quote => Some(Selector::NodeKind(NodeKind::Quote)),
            ElementKind::Footnote => Some(Selector::NodeKind(NodeKind::Footnote)),
            ElementKind::List => Some(Selector::NodeKind(NodeKind::List)),
            ElementKind::Enum => Some(Selector::NodeKind(NodeKind::Enum)),
            ElementKind::Equation => Some(Selector::NodeKind(NodeKind::Equation)),
            ElementKind::Par => Some(Selector::NodeKind(NodeKind::Par)),
            ElementKind::Table => Some(Selector::NativeElement(Func::native(
                "table",
                crate::compiler::stdlib::native_table,
            ))),
            ElementKind::Metadata => Some(Selector::NativeElement(Func::native(
                "metadata",
                crate::compiler::stdlib::native_metadata,
            ))),
            ElementKind::Outline => Some(Selector::NativeElement(Func::native(
                "outline",
                crate::compiler::stdlib::native_outline,
            ))),
            ElementKind::OutlineEntry => Some(Selector::NativeElement(Func::native(
                "outline_entry",
                crate::compiler::stdlib::native_outline_entry,
            ))),
            ElementKind::Citation => Some(Selector::NativeElement(Func::native(
                "cite",
                crate::compiler::stdlib::native_cite,
            ))),
            ElementKind::Bibliography => Some(Selector::NativeElement(Func::native(
                "bibliography",
                crate::compiler::stdlib::native_bibliography,
            ))),
            ElementKind::State
            | ElementKind::StateUpdate
            | ElementKind::CounterUpdate
            | ElementKind::StateDisplay
            | ElementKind::CounterDisplay
            | ElementKind::ContextBlock => None,
        }
    }

    match sel {
        QuerySelector::NativeElement(function) => Ok(Selector::NativeElement(function)),
        QuerySelector::Element { function, fields } => {
            let mut selector = if function.element_name() == Some("raw.line") {
                Selector::DynKind("raw.line".to_string())
            } else {
                Selector::NativeElement(function)
            };
            for (field, value) in fields {
                selector = Selector::Where {
                    base: Box::new(selector),
                    field,
                    value: Box::new(value),
                };
            }
            Ok(selector)
        }
        QuerySelector::Kind(kind) => match kind_to_show(kind) {
            Some(selector) => Ok(selector),
            None => Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "selector de kind '{}' não é suportado em show rule",
                    kind.as_str()
                ),
            )]),
        },
        // P493c — Where encadeado: converte recursivamente a base.
        QuerySelector::Where { base, field, value } => {
            let base_sel = query_selector_to_show_selector(*base, span)?;
            if is_node_rule(&base_sel) {
                Ok(Selector::Where { base: Box::new(base_sel), field, value })
            } else {
                Err(vec![SourceDiagnostic::error(
                    span,
                    "selector where com base não-node não suportado em show rule"
                        .to_string(),
                )])
            }
        }
        // **P423 (S-M)** — combinadores And/Or convertidos recursivamente.
        QuerySelector::And(sels) => {
            let mut converted = Vec::with_capacity(sels.len());
            for s in sels.iter() {
                converted.push(query_selector_to_show_selector(s.clone(), span)?);
            }
            Ok(Selector::And(converted))
        }
        QuerySelector::Or(sels) => {
            let mut converted = Vec::with_capacity(sels.len());
            for s in sels.iter() {
                converted.push(query_selector_to_show_selector(s.clone(), span)?);
            }
            Ok(Selector::Or(converted))
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            "selector não suportado em show rule".to_string(),
        )]),
    }
}

/// Casa um nó contra um selector de element rule (`NodeKind`/`NativeElement`/`DynKind`).
/// **Partilhado** (P352) pelo loop α (transformação func/content) e pela passagem
/// de show-set: ambos usam exatamente o mesmo critério de match. `Selector::Text`
/// nunca casa aqui (tratado por `map_text`).
pub(crate) fn selector_matches(work: &Content, selector: &Selector) -> bool {
    let work = match work {
        Content::Source(e) => &e.body,
        Content::ShowRecipe(e) => &e.body,
        Content::Located(e) => &e.body,
        Content::Realized(e) => &e.visual,
        other => other,
    };
    // F-5b fatia 1 (P371): `show strong/emph` casam as **variantes próprias**
    // `Content::Strong`/`Emph` (S1, por tipo). P431 (DEBT-50) acrescenta a
    // distinção de origem no `Style` para o cenário pós-bake-in (wrapping):
    // um `Content::Styled` com `Bold { from_strong: true }` casa `show strong`,
    // mas `Bold { from_strong: false }` (de `#set text(bold)`) não casa.
    match selector {
        Selector::NativeElement(function) => native_element_matches(work, function),
        Selector::NodeKind(kind) => {
            matches!(
                (work, kind),
                (Content::Heading(_), NodeKind::Heading)
                | (Content::Figure(_), NodeKind::Figure)
                | (Content::Raw { .. }, NodeKind::Raw)
                | (Content::Equation { .. }, NodeKind::Equation)
                | (Content::Par { .. }, NodeKind::Par)
                | (Content::ListItem(_), NodeKind::ListItem)
                | (Content::Strong(_), NodeKind::Strong)
                | (Content::Emph(_), NodeKind::Emph)
                | (Content::Underline(_), NodeKind::Underline)
                | (Content::Strike(_), NodeKind::Strike)
                | (Content::Overline(_), NodeKind::Overline)
                | (Content::SmallCaps { .. }, NodeKind::Smallcaps)
                // P494
                | (Content::Link(_), NodeKind::Link)
                | (Content::Quote(_), NodeKind::Quote)
                | (Content::Footnote(_), NodeKind::Footnote)
            ) || matches!(
                (work, kind),
                (_, NodeKind::List)
                    if matches!(work, Content::Sequence(seq) if seq.iter().all(|c| matches!(c, Content::ListItem(_))))
                    || matches!(work, Content::ListItem(_))
            ) || matches!(
                (work, kind),
                (_, NodeKind::Enum)
                    if matches!(work, Content::Sequence(seq) if seq.iter().all(|c| matches!(c, Content::EnumItem(_))))
                    || matches!(work, Content::EnumItem(_))
            ) || matches!(
                (work, kind),
                (_, NodeKind::Strong) if is_styled_origin(work, true, false, false, false, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Emph) if is_styled_origin(work, false, true, false, false, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Subscript) if is_styled_origin(work, false, false, true, false, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Superscript) if is_styled_origin(work, false, false, false, true, false)
            ) || matches!(
                (work, kind),
                (_, NodeKind::Highlight) if is_styled_origin(work, false, false, false, false, true)
            )
        }
        Selector::DynKind(name) => {
            matches!(work, Content::Dynamic(e) if e.dyn_kind() == name)
        }
        Selector::Text(pattern) => {
            !pattern.is_empty()
                && matches!(work, Content::Text(text) if text.contains(pattern.as_str()))
        }
        Selector::Regex(regex) => matches!(work, Content::Text(text)
            if regex.captures_all(text.as_str()).iter().any(|m| m.start < m.end)),
        // P791 — regras de label NÃO casam na travessia principal: são
        // aplicadas por `intercept_labelled` no ponto de associação
        // retroactiva (evita dupla aplicação das outras regras e garante
        // `it` = corpo, não o wrapper).
        Selector::Label(_) => false,
        Selector::Where { base, field, value } => {
            if !selector_matches(work, base) {
                return false;
            }
            if native_element_base(base).is_some() {
                native_element_field(work, field.as_str())
                    .is_some_and(|actual| values_eq(&actual, value.as_ref()))
            } else {
                work.get_field(field.as_str())
                    .is_some_and(|actual| values_eq_semantic(&actual, value.as_ref()))
            }
        }
        // **P423 (S-M)** — combinadores And/Or com curto-circuito.
        // And/Or vazios retornam `false` (Opção A fixada em P209C/P423).
        Selector::And(sels) => {
            !sels.is_empty() && sels.iter().all(|s| selector_matches(work, s))
        }
        Selector::Or(sels) => {
            !sels.is_empty() && sels.iter().any(|s| selector_matches(work, s))
        }
    }
}

fn native_element_base(selector: &Selector) -> Option<&Func> {
    match selector {
        Selector::NativeElement(function) => Some(function),
        Selector::Where { base, .. } => native_element_base(base),
        Selector::NodeKind(_)
        | Selector::DynKind(_)
        | Selector::Text(_)
        | Selector::Regex(_)
        | Selector::And(_)
        | Selector::Or(_)
        | Selector::Label(_) => None,
    }
}

/// Sequence identity is recognized without executing or spelling the callback.
pub(crate) fn is_sequence_rule(selector: &Selector) -> bool {
    match selector {
        Selector::NativeElement(function) => {
            function.native_fn_addr().is_some_and(|call| {
                std::ptr::fn_addr_eq(
                    call,
                    crate::compiler::eval::bindings::native_sequence
                        as fn(_, _, _, _) -> _,
                )
            })
        }
        Selector::Where { base, .. } => is_sequence_rule(base),
        Selector::And(children) | Selector::Or(children) => {
            children.iter().any(is_sequence_rule)
        }
        Selector::NodeKind(_)
        | Selector::DynKind(_)
        | Selector::Text(_)
        | Selector::Regex(_)
        | Selector::Label(_) => false,
    }
}

/// Preserve the special paragraph path, including filters over its new base.
pub(crate) fn is_par_rule(selector: &Selector) -> bool {
    matches!(selector, Selector::NodeKind(NodeKind::Par))
        || native_element_base(selector)
            .and_then(Func::native_fn_addr)
            .is_some_and(|addr| {
                std::ptr::fn_addr_eq(
                    addr,
                    crate::compiler::stdlib::native_par as fn(_, _, _, _) -> _,
                )
            })
}

/// Recognize represented native element identities by executable, never by alias name.
/// NodeKind remains authoritative for semantic style origins and list morphology.
pub(crate) fn native_element_matches(content: &Content, function: &Func) -> bool {
    use crate::compiler::eval::bindings::native_sequence;
    use crate::compiler::stdlib::*;
    let content = match content {
        Content::Source(e) => return native_element_matches(&e.body, function),
        Content::ShowRecipe(e) => return native_element_matches(&e.body, function),
        Content::Located(e) => return native_element_matches(&e.body, function),
        Content::Realized(e) => return native_element_matches(&e.visual, function),
        other => other,
    };
    let addr = function.native_fn_addr();
    macro_rules! native {
        ($function:ident) => {
            addr.is_some_and(|addr| {
                std::ptr::fn_addr_eq(addr, $function as fn(_, _, _, _) -> _)
            })
        };
    }
    macro_rules! node_kinds {
        ($($function:ident => $kind:ident),* $(,)?) => {
            $(if native!($function) {
                return selector_matches(content, &Selector::NodeKind(NodeKind::$kind));
            })*
        };
    }
    node_kinds! {
        native_heading => Heading, native_figure => Figure,
        native_strong => Strong, native_emph => Emph, native_raw => Raw,
        native_par => Par, native_link => Link, native_quote => Quote,
        native_footnote => Footnote, native_list => List, native_enum => Enum,
        native_underline => Underline, native_overline => Overline,
        native_strike => Strike, native_smallcaps => Smallcaps,
        native_subscript => Subscript, native_superscript => Superscript,
        native_highlight => Highlight,
    }
    match content {
        Content::Source(e) => native_element_matches(&e.body, function),
        Content::ShowRecipe(e) => native_element_matches(&e.body, function),
        Content::Located(e) => native_element_matches(&e.body, function),
        Content::Realized(e) => native_element_matches(&e.visual, function),
        Content::Styled(e) => {
            native!(native_math_equation) && native_element_matches(&e.body, function)
        }
        Content::Empty | Content::Sequence(_) => native!(native_sequence),
        Content::Equation(_) => native!(native_math_equation),
        Content::Text(_) => native!(native_text),
        Content::Table(_) => native!(native_table),
        Content::Grid(_) => native!(native_grid),
        Content::Metadata(_) => native!(native_metadata),
        Content::Terms(_) => native!(native_terms),
        Content::Outline(_) => native!(native_outline),
        Content::OutlineEntry(_) => native!(native_outline_entry),
        Content::Cite(_) => native!(native_cite),
        Content::Bibliography(_) => native!(native_bibliography),
        Content::Ref(_) => native!(native_ref),
        Content::Image(_) => native!(native_image),
        Content::Align(_) => native!(native_align),
        Content::Block(_) => native!(native_block),
        Content::Boxed(_) => native!(native_box),
        Content::Columns(_) => native!(native_columns),
        Content::Pad(_) => native!(native_pad),
        Content::Place(_) => native!(native_place),
        Content::Hide(_) => native!(native_hide),
        Content::Repeat(_) => native!(native_repeat),
        Content::Stack(_) => native!(native_stack),
        Content::HSpace(_) => native!(native_h),
        Content::VSpace(_) => native!(native_v),
        Content::Pagebreak(_) => native!(native_pagebreak),
        Content::Colbreak(_) => native!(native_colbreak),
        Content::Linebreak(_) => native!(native_linebreak),
        Content::Parbreak => native!(native_parbreak),
        Content::SmartQuote(_) => native!(native_smartquote),
        // These names overlap in the public namespace: executable identity is required.
        Content::TableHeader(_) => native!(native_table_header),
        Content::TableFooter(_) => native!(native_table_footer),
        Content::TableHLine(_) => native!(native_table_hline),
        Content::TableVLine(_) => native!(native_table_vline),
        Content::TableCell(_) => native!(native_table_cell),
        Content::GridHeader(_) => native!(native_grid_header),
        Content::GridFooter(_) => native!(native_grid_footer),
        Content::GridHLine(_) => native!(native_grid_hline),
        Content::GridVLine(_) => native!(native_grid_vline),
        Content::GridCell(_) => native!(native_grid_cell),
        // The antecedent erased constructor identity in these representations.
        // P1339 does not infer square/rect, circle/ellipse or transforms from geometry.
        Content::Shape(_) => false,
        Content::Transform(e) => match e.operation {
            crate::entities::elements::transform::TransformOperation::Matrix(_) => false,
            crate::entities::elements::transform::TransformOperation::Move { .. } => {
                native!(native_move)
            }
            crate::entities::elements::transform::TransformOperation::Rotate {
                ..
            } => native!(native_rotate),
            crate::entities::elements::transform::TransformOperation::Scale {
                ..
            } => native!(native_scale),
            crate::entities::elements::transform::TransformOperation::Skew { .. } => {
                native!(native_skew)
            }
        },
        Content::MathFrac(e) => e.line && native!(native_math_frac),
        Content::MathAttach(_) => native!(native_math_attach),
        Content::MathPrimes(_) => native!(native_math_primes),
        Content::MathRoot(_) => native!(native_math_root),
        Content::MathStretch(_) => native!(native_math_stretch),
        Content::MathLr(_) => native!(native_math_lr),
        Content::MathMid(_) => native!(native_math_mid),
        Content::MathOverbrace(_) => native!(native_math_overbrace),
        Content::MathUnderbrace(_) => native!(native_math_underbrace),
        Content::MathUnderbracket(_) => native!(native_math_underbracket),
        Content::MathUnderparen(_) => native!(native_math_underparen),
        Content::MathUndershell(_) => native!(native_math_undershell),
        Content::MathOverbracket(_) => native!(native_math_overbracket),
        Content::MathOverparen(_) => native!(native_math_overparen),
        Content::MathOvershell(_) => native!(native_math_overshell),
        Content::MathMatrix(_) => native!(native_math_mat),
        Content::MathCases(_) => native!(native_math_cases),
        Content::MathAccent(_) => native!(native_accent),
        Content::MathCancel(_) => native!(native_cancel),
        Content::MathUnderline(_) => native!(native_math_underline),
        Content::MathOverline(_) => native!(native_math_overline),
        Content::MathVec(_) => native!(native_math_vec),
        Content::MathClassOverride(_) => native!(native_math_class_for_metadata),
        Content::MathLimitsOverride(e) => {
            if e.limits {
                native!(native_math_limits)
            } else {
                native!(native_math_scripts)
            }
        }
        Content::PageRun(_) => match function.repr() {
            crate::entities::func::FuncRepr::NativeWithEngine(native) => {
                std::ptr::fn_addr_eq(
                    native.call,
                    native_page as fn(_, _, _, _, _, _) -> _,
                )
            }
            crate::entities::func::FuncRepr::Closure(_)
            | crate::entities::func::FuncRepr::Native(_)
            | crate::entities::func::FuncRepr::Format(_, _)
            | crate::entities::func::FuncRepr::Element(_)
            | crate::entities::func::FuncRepr::Plugin(_)
            | crate::entities::func::FuncRepr::With(_) => false,
        },
        Content::MathOp(_) => match function.repr() {
            crate::entities::func::FuncRepr::NativeWithEngine(native) => {
                std::ptr::fn_addr_eq(native.call, native_op as fn(_, _, _, _, _, _) -> _)
            }
            crate::entities::func::FuncRepr::Closure(_)
            | crate::entities::func::FuncRepr::Native(_)
            | crate::entities::func::FuncRepr::Format(_, _)
            | crate::entities::func::FuncRepr::Element(_)
            | crate::entities::func::FuncRepr::Plugin(_)
            | crate::entities::func::FuncRepr::With(_) => false,
        },
        Content::Space
        | Content::SpaceFromNewline
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
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathDelimited(_)
        | Content::MathAlignPoint(_)
        | Content::MathUnderover(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Curve(_)
        | Content::SetPage(_)
        | Content::Flush(_)
        | Content::Divider(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => false,
    }
}

/// Linguistic fields for native filters; the legacy Where projector stays unchanged.
fn native_element_field(content: &Content, field: &str) -> Option<Value> {
    match content {
        Content::Text(text) => match field {
            "text" => Some(Value::Str(text.clone())),
            _ => content.get_field(field),
        },
        Content::Raw(raw) => match field {
            "text" => Some(Value::Str(raw.text.clone())),
            "lang" => Some(raw.lang.clone().map(Value::Str).unwrap_or(Value::None)),
            "block" => Some(Value::Bool(raw.block)),
            _ => content.get_field(field),
        },
        Content::Styled(styled) => {
            if field == "body" && is_styled_origin(content, true, true, true, true, true)
            {
                Some(Value::Content(styled.body.clone()))
            } else {
                content.get_field(field)
            }
        }
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
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
        | Content::Dynamic(_) => content.get_field(field),
    }
}

/// A present occurrence snapshot is authoritative, including a missing field.
pub(crate) fn element_selector_matches(
    entry: &IntrospectedContent,
    function: &Func,
    fields: &[(ecow::EcoString, Value)],
) -> bool {
    native_element_matches(entry.content(), function)
        && fields.iter().all(|(name, expected)| match entry.fields() {
            Some(snapshot) => {
                snapshot.get(name).is_some_and(|actual| values_eq(actual, expected))
            }
            None => native_element_field(entry.content(), name)
                .is_some_and(|actual| values_eq(&actual, expected)),
        })
}

/// **P417 (M)** — Igualdade semântica de `Value` para matching de `Where`.
/// Replica ADR-0025 (coerção Int↔Float em comparações) e ADR-0107
/// (paridade comportamental, não mecânica).
fn values_eq_semantic(actual: &Value, expected: &Value) -> bool {
    match (actual, expected) {
        (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
        (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
        (a, b) => a == b,
    }
}

/// **P417/P423** — Verifica se um selector de show rule deve viajar pela
/// travessia de nós (`map_content`). Recursivo para `Where` com base
/// `NodeKind`/`DynKind`; P423 estende a `And`/`Or` (todos os sub-selectors
/// devem ser node-like).
pub(crate) fn is_node_rule(selector: &Selector) -> bool {
    match selector {
        Selector::NodeKind(_) | Selector::NativeElement(_) | Selector::DynKind(_) => true,
        Selector::Where { base, .. } => is_node_rule(base.as_ref()),
        // **P423 (S-M)** — combinadores viajam pela travessia de nós sse
        // todos os sub-selectors forem node-like.
        Selector::And(sels) | Selector::Or(sels) => sels.iter().all(is_node_rule),
        Selector::Text(_) | Selector::Regex(_) => false,
        // P791 — regras de label não viajam pela travessia de nós
        // (aplicação dedicada em `intercept_labelled`).
        Selector::Label(_) => false,
    }
}

mod textual;
pub(crate) use self::textual::{
    is_text_rule, splice_regex_rule_matches, splice_text_rule_matches,
    textual_match_ranges, textual_selector_matches,
};

/// Helper partilhado para casamento de origem sintática de estilos.
/// Um `Content::Styled` produzido por `*strong*`/`_emph_`/etc. marca a
/// origem no delta de estilos; um `Content::Styled` produzido por `#set`
/// não casa os selectors de elemento correspondentes.
fn is_styled_origin(
    work: &Content,
    bold_from_strong: bool,
    italic_from_emph: bool,
    subscript: bool,
    superscript: bool,
    highlight: bool,
) -> bool {
    match work {
        Content::Styled(styled) => {
            let styles = &styled.styles;
            let d = styles.delta();
            let mut ok = false;
            if bold_from_strong {
                ok |= d.bold == Some(true) && d.bold_from_strong == Some(true);
            }
            if italic_from_emph {
                ok |= d.italic == Some(true) && d.italic_from_emph == Some(true);
            }
            if subscript {
                ok |= d.subscript == Some(true);
            }
            if superscript {
                ok |= d.superscript == Some(true);
            }
            if highlight {
                ok |= d.highlight.is_some();
            }
            ok
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::show::Selector;
    use crate::entities::value::Value;
    use ecow::EcoString;

    #[test]
    fn v16_show_selector_native_base_preserves_identity_and_composition_boundary() {
        use crate::entities::label::Label;
        use crate::entities::regex::Regex;

        let function = Func::native("alias", crate::compiler::stdlib::native_heading);
        let native = Selector::NativeElement(function.clone());
        let filtered = Selector::Where {
            base: Box::new(Selector::Where {
                base: Box::new(native.clone()),
                field: "level".into(),
                value: Box::new(Value::Int(99)),
            }),
            field: "body".into(),
            value: Box::new(Value::None),
        };
        for (selector, has_base) in [
            (native.clone(), true),
            (filtered, true),
            (Selector::NodeKind(NodeKind::Heading), false),
            (Selector::DynKind("heading".into()), false),
            (Selector::Text("heading".into()), false),
            (Selector::Regex(Regex::new("heading").unwrap()), false),
            (Selector::And(vec![native.clone()]), false),
            (Selector::Or(vec![native]), false),
            (Selector::Label(Label("heading".into())), false),
        ] {
            let base = native_element_base(&selector);
            assert_eq!(base.is_some(), has_base, "{selector:?}");
            if let Some(base) = base {
                assert!(std::sync::Arc::ptr_eq(&base.0, &function.0));
            }
        }
        let par = Selector::NativeElement(Func::native(
            "alias",
            crate::compiler::stdlib::native_par,
        ));
        let fake = Selector::NativeElement(Func::native(
            "par",
            crate::compiler::stdlib::native_heading,
        ));
        assert!(is_par_rule(&par));
        assert!(!is_par_rule(&fake));
        assert!(!is_par_rule(&Selector::And(vec![par])));
    }

    #[test]
    fn p1339_snapshot_is_authoritative_and_absence_differs_from_none() {
        let function = Func::native("heading", crate::compiler::stdlib::native_heading);
        let content = Content::heading(1, Content::text("Intro"));
        let fields = [("level".into(), Value::Float(2.0))];
        let snapshot =
            [("level".into(), Value::Int(2)), ("numbering".into(), Value::None)]
                .into_iter()
                .collect();
        let captured = IntrospectedContent::new(content.clone(), Some(snapshot));
        assert!(element_selector_matches(&captured, &function, &fields));
        assert!(element_selector_matches(
            &captured,
            &function,
            &[("numbering".into(), Value::None)]
        ));
        let missing = IntrospectedContent::new(content.clone(), Some(Default::default()));
        assert!(!element_selector_matches(
            &missing,
            &function,
            &[("level".into(), Value::Int(1))]
        ));
        assert!(!element_selector_matches(
            &missing,
            &function,
            &[("numbering".into(), Value::None)]
        ));
        let projected = IntrospectedContent::new(content, None);
        assert!(element_selector_matches(
            &projected,
            &function,
            &[("level".into(), Value::Float(1.0))]
        ));
    }

    #[test]
    fn p1339_native_text_is_a_whole_node_and_names_do_not_spoof_identity() {
        let text = Func::native("text", crate::compiler::stdlib::native_text);
        let query = QuerySelector::Element {
            function: text,
            fields: [("text".into(), Value::Str("ab".into()))].into_iter().collect(),
        };
        let show = query_selector_to_show_selector(query, Span::detached()).unwrap();
        assert!(is_node_rule(&show));
        assert!(selector_matches(&Content::text("ab"), &show));
        assert!(!selector_matches(&Content::text("abc"), &show));
        let fake = Func::native("text", crate::compiler::stdlib::native_heading);
        assert!(!native_element_matches(&Content::text("ab"), &fake));
        let alias = Func::native("alias", crate::compiler::stdlib::native_text);
        assert!(native_element_matches(&Content::text("ab"), &alias));
    }

    #[test]
    fn p1339_native_par_filter_keeps_the_paragraph_path() {
        let query = QuerySelector::Element {
            function: Func::native("par", crate::compiler::stdlib::native_par),
            fields: [("body".into(), Value::Content(Content::text("ab")))]
                .into_iter()
                .collect(),
        };
        let show = query_selector_to_show_selector(query, Span::detached()).unwrap();
        assert!(is_par_rule(&show));
        assert!(!is_par_rule(&Selector::And(vec![show])));
    }

    #[test]
    fn p1339_equal_native_names_do_not_confuse_table_and_grid_occurrences() {
        let table = Content::TableHeader(std::sync::Arc::new(
            crate::entities::elements::table_header::TableHeaderElem {
                body: Content::text("body"),
                repeat: true,
            },
        ));
        let grid = Content::GridHeader(std::sync::Arc::new(
            crate::entities::elements::grid_header::GridHeaderElem {
                body: Content::text("body"),
                repeat: true,
            },
        ));
        let table_function =
            Func::native("header", crate::compiler::stdlib::native_table_header);
        let grid_function =
            Func::native("header", crate::compiler::stdlib::native_grid_header);
        assert!(native_element_matches(&table, &table_function));
        assert!(!native_element_matches(&grid, &table_function));
        assert!(native_element_matches(&grid, &grid_function));
        assert!(!native_element_matches(&table, &grid_function));
    }

    #[test]
    fn p1339_native_body_and_raw_fields_match_without_legacy_projection_changes() {
        let body = Content::text("body");
        for (function, content) in [
            (
                Func::native("strong", crate::compiler::stdlib::native_strong),
                Content::strong(body.clone()),
            ),
            (
                Func::native("emph", crate::compiler::stdlib::native_emph),
                Content::emph(body.clone()),
            ),
        ] {
            let show = query_selector_to_show_selector(
                QuerySelector::Element {
                    function,
                    fields: [("body".into(), Value::Content(body.clone()))]
                        .into_iter()
                        .collect(),
                },
                Span::detached(),
            )
            .unwrap();
            assert!(selector_matches(&content, &show));
            assert!(!selector_matches(&body, &show));
        }
        let raw = Content::raw("body", None, true);
        let function = Func::native("raw", crate::compiler::stdlib::native_raw);
        let filters = [
            ("text".into(), Value::Str("body".into())),
            ("lang".into(), Value::None),
            ("block".into(), Value::Bool(true)),
        ];
        assert!(element_selector_matches(
            &IntrospectedContent::new(raw.clone(), None),
            &function,
            &filters
        ));
        let show = query_selector_to_show_selector(
            QuerySelector::Element { function, fields: filters.into_iter().collect() },
            Span::detached(),
        )
        .unwrap();
        assert!(selector_matches(&raw, &show));
        assert!(!selector_matches(&Content::raw("body", None, false), &show));
        let legacy = Selector::Where {
            base: Box::new(Selector::NodeKind(NodeKind::Raw)),
            field: "lang".into(),
            value: Box::new(Value::None),
        };
        assert!(!selector_matches(&raw, &legacy));
    }

    #[test]
    fn p1339_where_public_empty_group_and_numeric_equality() {
        use crate::contracts::world::World;
        use crate::entities::world_types::{Bytes, FileError, FileResult, Font, Library};
        use crate::entities::{file_id::FileId, font_book::FontBook, source::Source};
        struct TestWorld {
            library: Library,
            book: FontBook,
            source: Source,
        }
        impl World for TestWorld {
            fn library(&self) -> &Library {
                &self.library
            }
            fn book(&self) -> &FontBook {
                &self.book
            }
            fn main(&self) -> FileId {
                self.source.id()
            }
            fn source(&self, _: FileId) -> FileResult<Source> {
                Ok(self.source.clone())
            }
            fn file(&self, _: FileId) -> FileResult<Bytes> {
                Err(FileError::NotFound)
            }
            fn font(&self, _: usize) -> Option<Font> {
                None
            }
            fn today(
                &self,
                _: Option<crate::entities::duration::Duration>,
            ) -> Option<crate::entities::world_types::Datetime> {
                None
            }
        }
        let source = Source::new(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            "#let a = strong.where()\n#let printed = repr(a)\n#let same = heading.where(level: 1) == heading.where(level: 1.0)".into(),
        );
        let world = TestWorld {
            library: Library::new(),
            book: FontBook::new(),
            source,
        };
        let module = super::super::tests::eval_for_test(&world, &world.source)
            .expect("P1339 public where accepts an explicitly empty element filter");
        assert_eq!(
            module.scope().get("printed"),
            Some(&Value::Str("strong.where(:)".into()))
        );
        assert_eq!(module.scope().get("same"), Some(&Value::Bool(true)));
    }

    fn where_selector(field: &str, value: Value) -> Selector {
        Selector::Where {
            base: Box::new(Selector::NodeKind(NodeKind::Heading)),
            field: EcoString::from(field),
            value: Box::new(value),
        }
    }

    #[test]
    fn p417_matches_where_positivo() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("level", Value::Int(1));
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_valor_errado() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("level", Value::Int(2));
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_campo_inexistente() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("inexistente", Value::Int(1));
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_base_nao_casa() {
        let content =
            Content::figure(Content::text("Fig"), None, Some("image".to_string()), None);
        let sel = where_selector("level", Value::Int(1));
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_int_float_coerce() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("level", Value::Float(1.0));
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p417_matches_where_body_content() {
        let content = Content::heading(1, Content::text("Intro"));
        let sel = where_selector("body", Value::Content(Content::text("Intro")));
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p417_extract_field_heading_level() {
        let content = Content::heading(2, Content::text("X"));
        assert_eq!(content.get_field("level"), Some(Value::Int(2)));
    }

    #[test]
    fn p417_extract_field_heading_body() {
        let content = Content::heading(1, Content::text("Intro"));
        assert_eq!(
            content.get_field("body"),
            Some(Value::Content(Content::text("Intro")))
        );
    }

    #[test]
    fn p417_extract_field_heading_inexistente() {
        let content = Content::heading(1, Content::text("X"));
        assert_eq!(content.get_field("inexistente"), None);
    }

    // ── P423 (S-M) — combinadores And/Or em show rules ──────────────────────

    fn or_selector(a: Selector, b: Selector) -> Selector {
        Selector::Or(vec![a, b])
    }

    fn and_selector(a: Selector, b: Selector) -> Selector {
        Selector::And(vec![a, b])
    }

    #[test]
    fn p423_matches_or_positivo_heading() {
        let content = Content::heading(1, Content::text("T"));
        let sel = or_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_or_positivo_figure() {
        let content =
            Content::figure(Content::text("F"), None, Some("image".to_string()), None);
        let sel = or_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_or_negativo_paragraph() {
        let content = Content::text("par");
        let sel = or_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_and_positivo_where_plus_kind() {
        let content = Content::heading(1, Content::text("T"));
        let sel = and_selector(
            Selector::Where {
                base: Box::new(Selector::NodeKind(NodeKind::Heading)),
                field: "level".into(),
                value: Box::new(Value::Int(1)),
            },
            Selector::NodeKind(NodeKind::Heading),
        );
        assert!(selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_and_negativo_contraditorio() {
        let content = Content::heading(1, Content::text("T"));
        let sel = and_selector(
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        );
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_and_vazio_false() {
        let content = Content::heading(1, Content::text("T"));
        let sel = Selector::And(vec![]);
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_matches_or_vazio_false() {
        let content = Content::heading(1, Content::text("T"));
        let sel = Selector::Or(vec![]);
        assert!(!selector_matches(&content, &sel));
    }

    #[test]
    fn p423_is_node_rule_and_or_com_node_kinds() {
        let sel = Selector::And(vec![
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        ]);
        assert!(is_node_rule(&sel));
        let sel = Selector::Or(vec![
            Selector::NodeKind(NodeKind::Heading),
            Selector::NodeKind(NodeKind::Figure),
        ]);
        assert!(is_node_rule(&sel));
    }

    #[test]
    fn p423_is_node_rule_and_nao_node_rejeita() {
        let sel = Selector::And(vec![
            Selector::NodeKind(NodeKind::Heading),
            Selector::Text("x".to_string()),
        ]);
        assert!(!is_node_rule(&sel));
    }

    // ── Passo 444 (P284) — selectors para text decoration ───────────────────

    #[test]
    fn p444_selector_underline_casa_content_underline() {
        let content = Content::underline(Content::text("x"), None, None, None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strike)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Overline)));
    }

    #[test]
    fn p444_selector_strike_casa_content_strike() {
        let content = Content::strike(Content::text("x"), None, None, None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Strike)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
    }

    #[test]
    fn p444_selector_overline_casa_content_overline() {
        let content = Content::overline(Content::text("x"), None, None, None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Overline)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
    }

    #[test]
    fn p444_selector_decoration_nao_casa_texto_plano() {
        let content = Content::text("x");
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Underline)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strike)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Overline)));
    }

    // ── Passo 446 (P408) — selector para smallcaps ──────────────────────────

    #[test]
    fn p446_selector_smallcaps_casa_content_smallcaps() {
        let content = Content::smallcaps(Content::text("x"));
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Smallcaps)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strong)));
    }

    #[test]
    fn p446_selector_smallcaps_nao_casa_texto_plano() {
        let content = Content::text("x");
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Smallcaps)));
    }

    // ── Passo 449 (P449) — selector para highlight ────────────────────────

    #[test]
    fn p449_selector_highlight_casa_styled_highlight() {
        let content = Content::highlight(Content::text("x"), None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Highlight)));
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Strong)));
    }

    #[test]
    fn p449_selector_highlight_nao_casa_texto_plano() {
        let content = Content::text("x");
        assert!(!selector_matches(&content, &Selector::NodeKind(NodeKind::Highlight)));
    }

    #[test]
    fn p449_selector_highlight_fill_none_ainda_casa() {
        // `fill: none` é um highlight "desactivado"; continua a casar o selector
        // de tipo, exactamente como um `set` vazio não remove a origem sintática.
        let content = Content::highlight(Content::text("x"), None);
        assert!(selector_matches(&content, &Selector::NodeKind(NodeKind::Highlight)));
    }

    #[test]
    fn p1285_selector_literal_casa_ocorrencia_local_sem_virar_node_rule() {
        let selector = Selector::Text("b".to_string());
        assert!(selector_matches(&Content::text("abc"), &selector));
        assert!(!selector_matches(&Content::text("azc"), &selector));
        assert!(!selector_matches(&Content::strong(Content::text("abc")), &selector,));
        assert!(!is_node_rule(&selector));
    }

    #[test]
    fn p1285_selector_regex_casa_match_nao_vazio_e_rejeita_negativos() {
        let selector =
            Selector::Regex(crate::entities::regex::Regex::new("[0-9]+").unwrap());
        assert!(selector_matches(&Content::text("abc123"), &selector));
        assert!(!selector_matches(&Content::text("abc"), &selector));
        assert!(!selector_matches(&Content::strong(Content::text("123")), &selector,));
        assert!(!is_node_rule(&selector));

        let empty_match =
            Selector::Regex(crate::entities::regex::Regex::new("a*").unwrap());
        assert!(!selector_matches(&Content::text("bbb"), &empty_match));
        assert!(!is_node_rule(&empty_match));
    }

    #[test]
    fn p1285_splice_regex_substitui_todas_as_ocorrencias_em_ordem() {
        let regex = crate::entities::regex::Regex::new("f.o").unwrap();
        let mut matches = vec![];
        let actual = splice_regex_rule_matches("foo fxo", &regex, |matched: &str| {
            matches.push(matched.to_string());
            Ok(Content::strong(Content::text(matched)))
        })
        .unwrap()
        .expect("regex com duas ocorrências deve produzir splice");

        assert_eq!(matches, ["foo", "fxo"]);
        assert_eq!(
            actual,
            Content::sequence(vec![
                Content::strong(Content::text("foo")),
                Content::text(" "),
                Content::strong(Content::text("fxo")),
            ])
        );
    }

    #[test]
    fn p1285_splice_regex_preserva_no_match_e_ignora_match_somente_vazio() {
        let mut calls = 0;
        for regex in [
            crate::entities::regex::Regex::new("z+").unwrap(),
            crate::entities::regex::Regex::new("a*").unwrap(),
        ] {
            let actual = splice_regex_rule_matches("bbb", &regex, |_: &str| {
                calls += 1;
                Ok(Content::text("substituído"))
            })
            .unwrap();
            assert_eq!(actual, None);
        }
        assert_eq!(calls, 0, "replacement não deve receber match vazio");
    }
}

#[cfg(test)]
mod equation_where_tests {
    use super::*;
    #[test]
    fn equation_where_filters_match_identity_body_block_and_captured_fields() {
        let Value::Module(math) = crate::compiler::stdlib::make_math_module() else {
            panic!("math module")
        };
        let Some(Value::Func(eq)) = math.scope().get("equation") else {
            panic!("missing equation")
        };
        let alias = Func::native("alias", eq.native_fn_addr().unwrap());
        let body = Content::text("a");
        let content = Content::equation(body.clone(), true);
        let fields =
            [("block".into(), Value::Bool(true)), ("body".into(), Value::Content(body))];
        for function in [eq.clone(), alias] {
            assert!(native_element_matches(&content, &function));
            let styled = Content::styled(
                Box::new(content.clone()),
                crate::entities::style::Styles::default(),
            );
            assert!(element_selector_matches(
                &IntrospectedContent::new(styled, None),
                &function,
                &fields
            ));
            assert!(element_selector_matches(
                &IntrospectedContent::new(content.clone(), None),
                &function,
                &fields
            ));
            assert!(!element_selector_matches(
                &IntrospectedContent::new(content.clone(), Some(Default::default())),
                &function,
                &fields
            ));
            let show = query_selector_to_show_selector(
                QuerySelector::Element {
                    function,
                    fields: fields.clone().into_iter().collect(),
                },
                Span::detached(),
            )
            .unwrap();
            assert!(selector_matches(&content, &show));
            assert!(!selector_matches(
                &Content::equation(Content::text("a"), false),
                &show
            ));
        }
        let fake = Func::native("math.equation", crate::compiler::stdlib::native_heading);
        assert!(!native_element_matches(&content, &fake));
    }
}

#[cfg(test)]
mod sequence_constructor_tests {
    use super::*;
    use crate::entities::args::Args;

    #[test]
    fn sequence_constructor_matcher_covers_empty_sequence_and_false_identity() {
        let Value::Func(f) = crate::compiler::eval::bindings::eval_content_method(
            &Content::Empty,
            "func",
            Args::positional(vec![]),
            Span::detached(),
        )
        .unwrap() else {
            panic!("function")
        };
        for content in [
            Content::Empty,
            Content::Sequence(vec![Content::text("A")].into()),
            Content::sequence(vec![Content::text("A"), Content::text("B")]),
        ] {
            assert!(native_element_matches(&content, &f));
        }
        assert!(!native_element_matches(&Content::text(""), &f));
        let fake = Func::native("sequence", |_, _, _, _| Ok(Value::None));
        assert!(!native_element_matches(&Content::Empty, &fake));
    }
}

#[cfg(test)]
mod batch_c_selector_matching_tests {
    use super::*;
    use crate::compiler::stdlib::*;
    use crate::entities::content::{
        LocatedContent, RealizedContent, ShowRecipeContent, SourceContent,
    };
    use crate::entities::location::Location;
    use std::sync::Arc;

    fn carriers(body: Content) -> Vec<Content> {
        vec![
            body.clone(),
            Content::Source(Arc::new(SourceContent {
                body: body.clone(),
                span: Span::detached(),
            })),
            Content::Located(Arc::new(LocatedContent {
                body: body.clone(),
                location: Location::from_raw(5),
            })),
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("different"),
                visual: body.clone(),
            })),
            Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body,
                textual: false,
                context_id_floor: 0,
                rules: vec![].into(),
                styles: crate::entities::style_chain::StyleChain::empty(),
                guards: vec![],
                file: crate::entities::file_id::FileId::from_raw(
                    std::num::NonZeroU16::new(1).unwrap(),
                ),
            })),
        ]
    }

    #[test]
    fn v16_c_matching_nested_causal_carriers_preserve_native_heading_identity() {
        let heading = Content::heading(1, Content::text("H"));
        let mut nested = heading;
        for index in [3, 2, 4, 1] {
            nested = carriers(nested).swap_remove(index);
        }
        let function = Func::native("alias", native_heading);
        assert!(native_element_matches(&nested, &function));
        assert!(selector_matches(&nested, &Selector::NativeElement(function)));
    }

    #[test]
    fn v16_c_matching_preserves_fields_fallback_and_snapshot_authority() {
        let text = Content::text("text");
        assert_eq!(native_element_field(&text, "text"), Some(Value::Str("text".into())));
        assert_eq!(native_element_field(&text, "missing"), text.get_field("missing"));
        let styled = Content::styled(text.clone(), crate::entities::style::Styles::new());
        assert_eq!(native_element_field(&styled, "body"), styled.get_field("body"));
        let strong = Content::strong(text.clone());
        assert_eq!(native_element_field(&strong, "body"), strong.get_field("body"));
        let function = Func::native("text", native_text);
        let fields = vec![("text".into(), Value::Str("text".into()))];
        let entry = IntrospectedContent::new(text.clone(), None);
        assert!(element_selector_matches(&entry, &function, &fields));
        let captured = IntrospectedContent::new(text, Some(Default::default()));
        assert!(!element_selector_matches(&captured, &function, &fields));
    }

    #[test]
    fn v16_c_matching_kind_bridge_admits_represented_elements_and_rejects_effect_kinds() {
        for kind in [
            ElementKind::Heading,
            ElementKind::Figure,
            ElementKind::Equation,
            ElementKind::Par,
            ElementKind::Table,
            ElementKind::Metadata,
            ElementKind::Outline,
            ElementKind::OutlineEntry,
            ElementKind::Citation,
            ElementKind::Bibliography,
        ] {
            assert!(
                query_selector_to_show_selector(
                    QuerySelector::Kind(kind),
                    Span::detached()
                )
                .is_ok(),
                "{kind:?}"
            );
        }
        for kind in [
            ElementKind::State,
            ElementKind::StateUpdate,
            ElementKind::CounterUpdate,
            ElementKind::StateDisplay,
            ElementKind::CounterDisplay,
            ElementKind::ContextBlock,
        ] {
            let errors = query_selector_to_show_selector(
                QuerySelector::Kind(kind),
                Span::detached(),
            )
            .unwrap_err();
            assert!(errors[0].message.contains(kind.as_str()));
            assert_eq!(errors[0].span, Span::detached());
        }
    }

    #[test]
    fn v16_c_matching_math_identity_is_typed_and_preserves_limits_distinction() {
        let body = Content::text("x");
        let cases = [
            (Content::math_mid(body.clone()), Func::native("unrelated", native_math_mid)),
            (
                Content::math_matrix(vec![vec![body.clone()]], ('(', ')')),
                Func::native("unrelated", native_math_mat),
            ),
            (
                Content::math_cases(vec![vec![body.clone()]], ('{', '}'), false, None),
                Func::native("unrelated", native_math_cases),
            ),
            (
                Content::math_vec(vec![body.clone()]),
                Func::native("unrelated", native_math_vec),
            ),
            (
                Content::math_class_override(
                    crate::entities::math_class::MathClass::Relation,
                    body.clone(),
                ),
                Func::native("unrelated", native_math_class_for_metadata),
            ),
            (
                Content::math_attach_scripts(
                    body.clone(),
                    crate::entities::elements::math_attach::MathAttachSlot::Omitted,
                    crate::entities::elements::math_attach::MathAttachSlot::Omitted,
                    crate::entities::elements::math_attach::MathAttachSlot::Omitted,
                    crate::entities::elements::math_attach::MathAttachSlot::Omitted,
                ),
                Func::native("unrelated", native_math_attach),
            ),
            (
                Content::math_stretch(
                    body.clone(),
                    crate::entities::rel::Rel::from_percent(100.0),
                    false,
                ),
                Func::native("unrelated", native_math_stretch),
            ),
            (
                Content::math_lr(
                    body.clone(),
                    crate::entities::rel::Rel::from_percent(100.0),
                    false,
                ),
                Func::native("unrelated", native_math_lr),
            ),
            (
                Content::math_overbrace(body.clone(), None, false),
                Func::native("unrelated", native_math_overbrace),
            ),
            (
                Content::math_underbrace(body.clone(), None, false),
                Func::native("unrelated", native_math_underbrace),
            ),
            (
                Content::math_underbracket(body.clone(), None, false),
                Func::native("unrelated", native_math_underbracket),
            ),
            (
                Content::math_underparen(body.clone(), None, false),
                Func::native("unrelated", native_math_underparen),
            ),
            (
                Content::math_undershell(body.clone(), None, false),
                Func::native("unrelated", native_math_undershell),
            ),
            (
                Content::math_overbracket(body.clone(), None, false),
                Func::native("unrelated", native_math_overbracket),
            ),
            (
                Content::math_overparen(body.clone(), None, false),
                Func::native("unrelated", native_math_overparen),
            ),
            (
                Content::math_overshell(body.clone(), None, false),
                Func::native("unrelated", native_math_overshell),
            ),
            (
                Content::math_frac(body.clone(), body.clone()),
                Func::native("unrelated", native_math_frac),
            ),
            (
                Content::math_root(None, body.clone(), false),
                Func::native("unrelated", native_math_root),
            ),
            (Content::math_primes(2), Func::native("unrelated", native_math_primes)),
            (
                Content::math_underline(body.clone()),
                Func::native("unrelated", native_math_underline),
            ),
            (
                Content::math_overline(body.clone()),
                Func::native("unrelated", native_math_overline),
            ),
            (
                Content::math_cancel(body.clone()),
                Func::native("unrelated", native_cancel),
            ),
            (
                Content::math_accent(body.clone(), '~'),
                Func::native("unrelated", native_accent),
            ),
            (
                Content::math_limits_override(body.clone(), true, false, false),
                Func::native("unrelated", native_math_limits),
            ),
            (
                Content::math_limits_override(body.clone(), false, false, false),
                Func::native("unrelated", native_math_scripts),
            ),
        ];
        let mut missing = Vec::new();
        for (index, (content, alias)) in cases.into_iter().enumerate() {
            for (carrier_index, carrier) in
                carriers(content.clone()).into_iter().enumerate()
            {
                if !native_element_matches(&carrier, &alias) {
                    missing.push((index, carrier_index));
                }
            }
            assert!(!native_element_matches(
                &content,
                &Func::native("unrelated", native_heading)
            ));
            assert!(!native_element_matches(
                &Content::styled(content, crate::entities::style::Styles::new()),
                &alias
            ));
        }
        assert!(
            missing.is_empty(),
            "unrecognized math identity/carrier cases: {missing:?}"
        );
        assert!(!native_element_matches(
            &Content::math_limits_override(body.clone(), true, false, false),
            &Func::native("limits", native_math_scripts)
        ));
        assert!(!native_element_matches(
            &Content::math_limits_override(body, false, false, false),
            &Func::native("scripts", native_math_limits)
        ));
    }

    #[test]
    fn v16_c_matching_engine_identity_crosses_causal_carriers_without_original_fallback()
    {
        let page = Content::page_run(None, None, None, None, None, Content::text("x"));
        let op = Content::math_op(Content::text("custom"), false);
        for (content, function, wrong) in [
            (
                page,
                Func::native_with_engine("alias", native_page),
                Func::native_with_engine("page", native_op),
            ),
            (
                op,
                Func::native_with_engine("alias", native_op),
                Func::native_with_engine("op", native_page),
            ),
        ] {
            for carrier in carriers(content.clone()) {
                assert!(native_element_matches(&carrier, &function));
                assert!(!native_element_matches(&carrier, &wrong));
            }
            let visual = Content::Realized(Arc::new(RealizedContent {
                original: content,
                visual: Content::text("no match"),
            }));
            assert!(!native_element_matches(&visual, &function));
        }
    }
}

#[cfg(test)]
mod batch_c_math_lowering_controls {
    use super::*;
    #[test]
    fn v16_c_matching_preserves_binom_lowering_without_inventing_frac_identity() {
        let mechanical =
            Content::math_frac_unlined(Content::text("3"), Content::text("2"));
        assert!(!native_element_matches(
            &mechanical,
            &Func::native("frac", crate::compiler::stdlib::native_math_frac)
        ));
    }
}
