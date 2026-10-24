//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/bindings/field_access/content_methods.md
//! @prompt-hash e1a08b81
//! @layer L1
//! @updated 2026-10-07

use super::*;

// ── P829 — métodos de `content`: func/has/at/fields/location ───────────────

/// **P829** — estado de um campo de content para os métodos `has`/`at`/
/// `fields` (paridade vanilla `Content::has`/`at`/`fields`,
/// `foundations/content/mod.rs:510-590`): o vanilla distingue campo
/// **assente no constructor** de default resolvido pela chain — medido:
/// `heading[H].has("level")` → false, `heading(level: 2)[H].has("level")` →
/// true, markup `= H` assenta `depth` (não `level`).
pub(super) enum ContentField {
    /// Assente — `has` → true, `at` devolve o valor.
    Set(Value),
    /// Declarado no elemento mas não assente — `has` → false; `at` erra
    /// `field "{f}" in {elem} is not known at this point` (verbatim vanilla).
    Unset,
    /// Não existe no elemento — `at` erra `{elem} does not have field "{f}"`.
    Undeclared,
}

/// **P829** — classifica um campo de content para `has`/`at`. Distinto de
/// `Content::get_field` (field access de show rules, que devolve valores
/// baked): aqui só conta o que foi assente no constructor, espelhando o
/// `field.has()`/`get()` do vanilla. A máscara `set_fields` de `HeadingElem`
/// (P829), a máscara de Raw e os carriers de presença dos demais elementos
/// preservam "explicitamente assente". O fallback via `get_field` só é usado
/// nas famílias sem distinção adicional contratada. `label` fica
/// scope-out: no cristalino a label é um nó irmão (`Content::Label`), não
/// metadado do elemento — divergência registada no L0.
pub(super) fn content_field(
    c: &crate::entities::content::Content,
    field: &str,
) -> ContentField {
    use crate::entities::content::Content;
    use crate::entities::elements::heading::{
        HEADING_SET_DEPTH, HEADING_SET_LEVEL, HEADING_SET_OUTLINED,
    };
    if let Some((base, _)) = c.constructor_heading() {
        return match field {
            "supplement" | "numbering" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => content_field(base, field),
        };
    }
    match c {
        Content::Source(e) => content_field(&e.body, field),
        Content::Located(e) => content_field(&e.body, field),
        Content::ShowRecipe(e) => content_field(&e.body, field),
        Content::Realized(e) => content_field(&e.visual, field),
        Content::Styled(styled) => {
            if crate::compiler::realize::figure::is_figure(&styled.body) {
                return content_field(&styled.body, field);
            }
            if equation_descendant(c).is_none() {
                return ContentField::Undeclared;
            }
            let styles = &styled.styles;
            let child = &styled.body;
            let key = match field {
                "numbering" => Some("equation.numbering"),
                "number-align" => Some("equation.number-align"),
                "supplement" => Some("equation.supplement"),
                "alt" => Some("equation.alt"),
                _ => None,
            };
            if let Some(value) = key.and_then(|key| {
                styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(candidate, _)| candidate.as_str() == key)
                    .map(|(_, value)| value.clone())
            }) {
                ContentField::Set(value)
            } else {
                content_field(child, field)
            }
        }
        Content::Figure(_) => match field {
            "body" | "caption" | "kind" | "supplement" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::HSpace(e) => match field {
            "amount" => ContentField::Set(c.get_field(field).expect("spacing amount")),
            "weak" if e.weak_explicit => ContentField::Set(Value::Bool(e.weak)),
            "weak" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::VSpace(e) => match field {
            "amount" => ContentField::Set(Value::Length(e.amount)),
            "weak" if e.weak_explicit => ContentField::Set(Value::Bool(e.weak)),
            "weak" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::Boxed(_) => match field {
            "width" | "height" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::Block(_) => match field {
            "width" | "height" | "body" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::Transform(e) => {
            use crate::entities::elements::transform::TransformOperation;
            let declared = match e.operation {
                TransformOperation::Matrix(_) => false,
                TransformOperation::Move { .. } => matches!(field, "dx" | "dy" | "body"),
                TransformOperation::Rotate { .. } => {
                    matches!(field, "angle" | "origin" | "reflow" | "body")
                }
                TransformOperation::Scale { .. } => {
                    matches!(field, "x" | "y" | "origin" | "reflow" | "body")
                }
                TransformOperation::Skew { .. } => {
                    matches!(field, "ax" | "ay" | "origin" | "reflow" | "body")
                }
            };
            if declared {
                c.get_field(field)
                    .map(ContentField::Set)
                    .unwrap_or(ContentField::Unset)
            } else {
                ContentField::Undeclared
            }
        }
        Content::Raw(_) => match field {
            "text" | "block" | "lang" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::MathVec(_) => match field {
            "delim" | "align" | "gap" | "children" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::MathCases(_) => match field {
            "delim" | "reverse" | "gap" | "children" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::MathClassOverride(_) => match field {
            "class" | "body" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::MathMatrix(_) => match field {
            "rows" | "delim" | "align" | "augment" | "row-gap" | "column-gap" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::TableCell(_) => match field {
            "body" | "x" | "y" | "colspan" | "rowspan" | "inset" | "align" | "fill"
            | "stroke" | "breakable" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::SmartQuote(e) => {
            let present = match field {
                "double" => e.double.is_some(),
                "enabled" => e.enabled.is_some(),
                "alternative" => e.alternative.is_some(),
                "quotes" => e.quotes.is_some(),
                _ => return ContentField::Undeclared,
            };
            if present {
                ContentField::Set(c.get_field(field).expect("declared quote field"))
            } else {
                ContentField::Unset
            }
        }
        Content::Strong(e) => match field {
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            // `delta` é declarado em strong no vanilla mas nunca assente no
            // cristalino (`native_strong` não aceita named — scope-out).
            "delta" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::Emph(e) => match field {
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Title(t) => match field {
            "body" => ContentField::Set(Value::Content(t.body.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Text(t) | Content::Symbol(t) | Content::MathText(t) => match field {
            "text" => ContentField::Set(Value::Str(t.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Linebreak(e) => match field {
            "justify" if e.justify_explicit => ContentField::Set(Value::Bool(e.justify)),
            "justify" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::Heading(h) => match field {
            "body" => ContentField::Set(Value::Content(h.body.clone())),
            "level" => {
                if h.set_fields & HEADING_SET_LEVEL != 0 {
                    ContentField::Set(Value::Int(h.level as i64))
                } else {
                    ContentField::Unset
                }
            }
            "depth" => {
                if h.set_fields & HEADING_SET_DEPTH != 0 {
                    ContentField::Set(Value::Int(h.level as i64))
                } else {
                    ContentField::Undeclared
                }
            }
            "outlined" => {
                if h.set_fields & HEADING_SET_OUTLINED != 0 {
                    ContentField::Set(Value::Bool(h.outlined))
                } else {
                    ContentField::Unset
                }
            }
            "bookmarked" => match h.bookmarked {
                Some(b) => ContentField::Set(Value::Bool(b)),
                None => ContentField::Unset,
            },
            _ => ContentField::Undeclared,
        },
        Content::OutlineEntry(e) => match field {
            "level" | "element" => {
                ContentField::Set(c.get_field(field).expect("entry required field"))
            }
            "fill" if e.fill_set => {
                ContentField::Set(c.get_field(field).expect("entry explicit fill"))
            }
            "fill" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::Equation(e) => match field {
            "block" => ContentField::Set(Value::Bool(e.block)),
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            _ => ContentField::Undeclared,
        },
        // Existing getter projection only; no lexical defaults or new schemas.
        other @ (Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(..)
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::MathSequence(..)
        | Content::MathIdent(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..)) => match other.get_field(field) {
            Some(v) => ContentField::Set(v),
            None => ContentField::Undeclared,
        },
    }
}

/// **P829** — campos assentes de um content, na ordem de declaração do
/// vanilla (para heading: level, depth, outlined, bookmarked, body — medido
/// em b8/b18: `(level: 2, body: [H])`, `(depth: 1, body: [H])`).
pub(super) fn content_set_fields(
    c: &crate::entities::content::Content,
) -> Vec<(&'static str, Value)> {
    use crate::entities::content::Content;
    let c = c.source_body();
    if let Content::Styled(styled) = c {
        if crate::compiler::realize::figure::is_figure(&styled.body) {
            return content_set_fields(&styled.body);
        }
    }
    let candidates: &[&'static str] = match c {
        Content::Source(e) => return content_set_fields(&e.body),
        Content::ShowRecipe(e) => return content_set_fields(&e.body),
        Content::Located(e) => return content_set_fields(&e.body),
        Content::Realized(e) => return content_set_fields(&e.visual),
        Content::Figure(_) => &["body", "caption", "kind", "supplement"],
        Content::Metadata(_) => &["value"],
        Content::OutlineEntry(_) => &["level", "element", "fill"],
        Content::Empty | Content::Sequence(_) => &["children"],
        Content::HSpace(_) | Content::VSpace(_) => &["amount", "weak"],
        Content::Boxed(_) => &["width", "height"],
        Content::Block(_) => &["width", "height", "body"],
        Content::FigureCaption(_) => &["position", "separator", "body"],
        Content::MathVec(_) => &["delim", "align", "gap", "children"],
        Content::MathCases(_) => &["delim", "reverse", "gap", "children"],
        Content::MathClassOverride(_) => &["class", "body"],
        Content::MathText(_) => &["text"],
        Content::MathMatrix(_) => {
            &["delim", "align", "augment", "row-gap", "column-gap", "rows"]
        }
        Content::TableCell(_) => &[
            "body",
            "x",
            "y",
            "colspan",
            "rowspan",
            "inset",
            "align",
            "fill",
            "stroke",
            "breakable",
        ],
        Content::SmartQuote(_) => &["double", "enabled", "alternative", "quotes"],
        Content::Heading(_) => &["level", "depth", "outlined", "bookmarked", "body"],
        Content::Equation(_) => &["block", "body"],
        Content::MathPrimes(_) => &["count"],
        Content::MathLr(e) => {
            if e.size_explicit {
                &["size", "body"]
            } else {
                &["body"]
            }
        }
        Content::MathUnderbrace(e) => {
            if e.annotation_explicit {
                &["body", "annotation"]
            } else {
                &["body"]
            }
        }
        Content::MathUnderbracket(e) => {
            if e.annotation_explicit {
                &["body", "annotation"]
            } else {
                &["body"]
            }
        }
        Content::MathUnderparen(e) => {
            if e.annotation_explicit {
                &["body", "annotation"]
            } else {
                &["body"]
            }
        }
        Content::MathUndershell(e) => {
            if e.annotation_explicit {
                &["body", "annotation"]
            } else {
                &["body"]
            }
        }
        Content::MathStretch(e) => {
            if e.size_explicit {
                &["body", "size"]
            } else {
                &["body"]
            }
        }
        Content::Styled(_) => {
            if c.constructor_heading().is_some() {
                &[
                    "level",
                    "depth",
                    "numbering",
                    "supplement",
                    "outlined",
                    "bookmarked",
                    "body",
                ]
            } else if equation_descendant(c).is_some() {
                &["block", "numbering", "number-align", "supplement", "alt", "body"]
            } else {
                // Ordinary Styled retains its separate, currently incomplete strict schema.
                &[]
            }
        }
        Content::Text(_) | Content::Symbol(_) => &["text"],
        Content::Raw(_) => &["text", "block", "lang"],
        Content::Transform(e) => match e.operation {
            crate::entities::elements::transform::TransformOperation::Matrix(_) => &[],
            crate::entities::elements::transform::TransformOperation::Move { .. } => {
                &["dx", "dy", "body"]
            }
            crate::entities::elements::transform::TransformOperation::Rotate {
                ..
            } => &["angle", "origin", "reflow", "body"],
            crate::entities::elements::transform::TransformOperation::Scale {
                ..
            } => &["x", "y", "origin", "reflow", "body"],
            crate::entities::elements::transform::TransformOperation::Skew { .. } => {
                &["ax", "ay", "origin", "reflow", "body"]
            }
        },
        Content::Linebreak(_) => &["justify"],
        // Legacy body candidate, filtered by the getter; richer schemas remain uncertified.
        Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::MathSequence(..)
        | Content::MathIdent(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathRoot(..)
        | Content::MathDelimited(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => &["body"],
    };
    candidates
        .iter()
        .filter_map(|name| match content_field(c, name) {
            ContentField::Set(v) => Some((*name, v)),
            ContentField::Unset | ContentField::Undeclared => None,
        })
        .collect()
}

pub(super) fn equation_descendant(
    content: &crate::entities::content::Content,
) -> Option<&crate::entities::elements::equation::EquationElem> {
    use crate::entities::content::Content;
    match content {
        Content::Equation(e) => Some(e),
        Content::Styled(styled) => {
            let marker = styled
                .styles
                .delta()
                .custom
                .iter()
                .rev()
                .find(|(key, _)| key.as_str() == "equation.constructor");
            if matches!(marker, Some((_, Value::Bool(true)))) {
                equation_descendant(&styled.body)
            } else {
                None
            }
        }
        Content::Source(e) => equation_descendant(&e.body),
        Content::ShowRecipe(e) => equation_descendant(&e.body),
        Content::Located(e) => equation_descendant(&e.body),
        Content::Realized(e) => equation_descendant(&e.visual),
        Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(..)
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::MathSequence(..)
        | Content::Symbol(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => None,
    }
}

/// Constructor exposed only by the element identity returned from content.func().
pub(crate) fn native_sequence(
    _ctx: &mut crate::compiler::eval::EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    use crate::entities::content::Content;
    let occurrences = args.occurrences.as_deref();
    let positional =
        occurrences.and_then(|items| items.iter().find(|arg| arg.name.is_none()));
    let value_span =
        positional.map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span));
    let Some(value) = args.items.first() else {
        if args.named.contains_key("children") {
            let span = occurrences
                .and_then(|items| {
                    items.iter().find(|arg| arg.name.as_deref() == Some("children"))
                })
                .map_or(args.span, |arg| arg.span.or(args.span));
            return Err(vec![SourceDiagnostic::error(
                span,
                "the argument `children` is positional",
            )
            .with_hint("try removing `children:`")]);
        }
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "missing argument: children",
        )]);
    };
    let Value::Array(values) = value else {
        return Err(vec![SourceDiagnostic::error(
            value_span,
            format!("expected array, found {}", vanilla_type_name(value)),
        )]);
    };
    let children: Vec<Content> = values
        .iter()
        .map(|value| match value {
            Value::Content(content) => Ok(content.clone()),
            Value::LocatedContent(content, _) => Ok(content.content().clone()),
            Value::Str(text) => Ok(Content::text(text.clone())),
            Value::Symbol(symbol) => Ok(Content::Symbol(symbol.value.clone())),
            Value::None => Ok(Content::Empty),
            other @ (Value::Bool(..)
            | Value::Int(..)
            | Value::Float(..)
            | Value::Array(..)
            | Value::Dict(..)
            | Value::Module(..)
            | Value::Datetime(..)
            | Value::Func(..)
            | Value::Auto
            | Value::Length(..)
            | Value::Relative(..)
            | Value::Ratio(..)
            | Value::Angle(..)
            | Value::Color(..)
            | Value::Stroke(..)
            | Value::Fraction(..)
            | Value::Align(..)
            | Value::Location(..)
            | Value::Gradient(..)
            | Value::Regex(..)
            | Value::Tiling(..)
            | Value::Bytes(..)
            | Value::Decimal(..)
            | Value::Duration(..)
            | Value::Version(..)
            | Value::Selector(..)
            | Value::Args(..)
            | Value::State(..)
            | Value::Counter(..)
            | Value::Label(..)
            | Value::Dir(..)
            | Value::Path(..)
            | Value::Type(..)
            | Value::SpotColorant(..)) => Err(vec![SourceDiagnostic::error(
                value_span,
                format!("expected content, found {}", vanilla_type_name(other)),
            )]),
        })
        .collect::<SourceResult<_>>()?;
    let extra = if let Some(items) = occurrences {
        let mut consumed = false;
        items
            .iter()
            .find(|arg| {
                if arg.name.is_none() && !consumed {
                    consumed = true;
                    false
                } else {
                    true
                }
            })
            .map(|arg| (arg.span.or(args.span), arg.name.as_deref()))
    } else if args.items.len() > 1 {
        Some((args.span, None))
    } else {
        args.named.first().map(|(name, _)| (args.span, Some(name.as_str())))
    };
    if let Some((span, name)) = extra {
        let message = name.map_or_else(
            || "unexpected argument".to_string(),
            |name| format!("unexpected argument: {name}"),
        );
        return Err(vec![SourceDiagnostic::error(span, message)]);
    }
    Ok(Value::Content(Content::Sequence(children.into())))
}

/// **P829** — fallback de `func()` para variantes sem constructor nativo
/// exposto (Styled, Label, math, Dynamic, …): o `Value::Func`
/// existe (repr = nome do elemento — paridade medida de `func()`) mas a
/// chamada não é suportada. Caso não medido no vanilla (elementos internos
/// não expostos no scope); mensagem própria, registada no L0.
pub(super) fn content_func_not_callable(
    _ctx: &mut crate::compiler::eval::EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "calling this element function is not supported".to_string(),
    )])
}

/// Project the retained native callback without executing it. Public element
/// names describe the result; they never determine constructor identity.
pub(super) fn content_elem_func(c: &crate::entities::content::Content) -> Value {
    use crate::entities::content::Content;
    let call: fn(
        &mut crate::compiler::eval::EvalContext,
        &Args,
        &dyn crate::contracts::world::World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value> = match c {
        Content::Source(e) => return content_elem_func(&e.body),
        Content::ShowRecipe(e) => return content_elem_func(&e.body),
        Content::Located(e) => return content_elem_func(&e.body),
        Content::Realized(e) => return content_elem_func(&e.visual),
        Content::Equation(_) => crate::compiler::stdlib::native_math_equation,
        Content::Empty | Content::Sequence(_) => native_sequence,
        Content::Symbol(_) => native_symbol_element,
        Content::Text(_) => crate::compiler::stdlib::native_text,
        Content::MathMatrix(_) => crate::compiler::stdlib::native_math_mat,
        Content::TableCell(_) => crate::compiler::stdlib::native_table_cell,
        Content::Table(_) => crate::compiler::stdlib::native_table,
        Content::Strong(_) => crate::compiler::stdlib::native_strong,
        Content::Emph(_) => crate::compiler::stdlib::native_emph,
        Content::Heading(_) => crate::compiler::stdlib::native_heading,
        Content::Styled(_) => {
            if c.constructor_heading().is_some() {
                crate::compiler::stdlib::native_heading
            } else if equation_descendant(c).is_some() {
                crate::compiler::stdlib::native_math_equation
            } else {
                content_func_not_callable
            }
        }
        Content::Title(_) => crate::compiler::stdlib::native_title,
        Content::Raw(_) => crate::compiler::stdlib::native_raw,
        Content::Transform(e) => match e.operation {
            crate::entities::elements::transform::TransformOperation::Matrix(_) => {
                content_func_not_callable
            }
            crate::entities::elements::transform::TransformOperation::Move { .. } => {
                crate::compiler::stdlib::native_move
            }
            crate::entities::elements::transform::TransformOperation::Rotate {
                ..
            } => crate::compiler::stdlib::native_rotate,
            crate::entities::elements::transform::TransformOperation::Scale {
                ..
            } => crate::compiler::stdlib::native_scale,
            crate::entities::elements::transform::TransformOperation::Skew { .. } => {
                crate::compiler::stdlib::native_skew
            }
        },
        Content::Figure(_) => crate::compiler::stdlib::native_figure,
        Content::FigureCaption(_) => crate::compiler::stdlib::native_figure_caption,
        Content::Link(_) => crate::compiler::stdlib::native_link,
        Content::SmallCaps { .. } => crate::compiler::stdlib::native_smallcaps,
        Content::Linebreak(_) => crate::compiler::stdlib::native_linebreak,
        Content::OutlineEntry(_) => crate::compiler::stdlib::native_outline_entry,
        // Legacy noncallable disposition; additional native coverage remains uncertified.
        Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::MathSequence(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => content_func_not_callable,
    };
    let name =
        if equation_descendant(c).is_some() { "math.equation" } else { c.elem_name() };
    Value::Func(Func::native(name, call))
}

pub(crate) fn eval_outline_entry_body_method(
    content: &crate::entities::content::Content,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    finish_args(&args, span)?;
    let crate::entities::content::Content::OutlineEntry(entry) = content else {
        unreachable!("receiver validated as outline entry")
    };
    entry
        .body()
        .map(Value::Content)
        .map_err(|message| vec![SourceDiagnostic::error(span, message)])
}

/// **P829** — mirror dos métodos do `#[scope]` de `Content` do vanilla
/// (`foundations/content/mod.rs:510-590`): `func()`, `has(field)`,
/// `at(field, default:?)`, `fields()`, `location()`. Mensagens de erro de
/// argumentos verbatim (medidas b13–b17): `missing argument: field`,
/// `expected string, found {tipo}`, `unexpected argument`,
/// `unexpected argument: {nome}`.
///
/// `location()` devolve sempre `none`: o cristalino não retém metadados de
/// location em `Content` (medido: content inline → none nos dois binários;
/// content de show rule/query → `location(..)` no vanilla — divergência
/// registada no L0, requer introspecção de locations, fora do proporcional).
pub(crate) fn eval_content_method(
    c: &crate::entities::content::Content,
    method: &str,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    eval_content_method_at(c, None, method, args, span)
}

pub(crate) fn eval_content_method_at(
    c: &crate::entities::content::Content,
    location: Option<crate::entities::location::Location>,
    method: &str,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    content_method_with_fields(c, location, None, method, args, span)
}

pub(crate) fn eval_introspected_content_method_at(
    c: &crate::entities::value::IntrospectedContent,
    location: crate::entities::location::Location,
    method: &str,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    content_method_with_fields(
        c.content(),
        Some(location),
        c.fields(),
        method,
        args,
        span,
    )
}

pub(super) fn content_method_with_fields(
    c: &crate::entities::content::Content,
    location: Option<crate::entities::location::Location>,
    fields: Option<&IndexMap<EcoString, Value, FxBuildHasher>>,
    method: &str,
    mut args: Args,
    span: Span,
) -> SourceResult<Value> {
    use crate::entities::content::Content;
    match c {
        Content::Source(e) => {
            return content_method_with_fields(
                &e.body, location, fields, method, args, span,
            )
        }
        Content::Located(e) => {
            return content_method_with_fields(
                &e.body,
                location.or(Some(e.location)),
                fields,
                method,
                args,
                span,
            )
        }
        Content::ShowRecipe(e) => {
            return content_method_with_fields(
                &e.body, location, fields, method, args, span,
            )
        }
        Content::Realized(e) => {
            return content_method_with_fields(
                &e.visual, location, fields, method, args, span,
            )
        }
        // Semantic receivers keep their own method dispatch.
        Content::Empty
        | Content::Symbol(..)
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(..)
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::Equation(..)
        | Content::MathSequence(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Styled(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => {}
    }
    match method {
        "func" => {
            finish_args(&args, span)?;
            Ok(content_elem_func(c))
        }
        "fields" => {
            finish_args(&args, span)?;
            if let Some(fields) = fields {
                return Ok(Value::Dict(fields.clone()));
            }
            let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
            for (name, value) in content_set_fields(c) {
                dict.insert(name.into(), value);
            }
            Ok(Value::Dict(dict))
        }
        "location" => {
            finish_args(&args, span)?;
            Ok(location.map(Value::Location).unwrap_or(Value::None))
        }
        "has" | "at" => {
            let field_value = expect_positional(&mut args, span, "field")?;
            let field = match field_value {
                Value::Str(s) => s,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!("expected string, found {}", vanilla_type_name(&other)),
                    )]);
                }
            };
            let default =
                if method == "at" { args.remove_named("default") } else { None };
            finish_args(&args, span)?;
            let field = field.as_str();
            let resolved = match fields {
                Some(fields) => fields
                    .get(field)
                    .cloned()
                    .map(ContentField::Set)
                    .unwrap_or(ContentField::Undeclared),
                None => content_field(c, field),
            };
            match method {
                "has" => Ok(Value::Bool(matches!(resolved, ContentField::Set(_)))),
                _ => match resolved {
                    ContentField::Set(v) => Ok(v),
                    ContentField::Unset => match default {
                        Some(v) => Ok(v),
                        None => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "field \"{field}\" in {} is not known at this point and no default was specified",
                                c.elem_name()
                            ),
                        )]),
                    },
                    ContentField::Undeclared => match default {
                        Some(v) => Ok(v),
                        None => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "{} does not have field \"{field}\" and no default was specified",
                                c.elem_name()
                            ),
                        )]),
                    },
                },
            }
        }
        _ => unreachable!("eval_content_method: método desconhecido {method}"),
    }
}

pub(super) fn static_content_receiver(
    args: &Args,
    name: &str,
) -> SourceResult<(Value, Args)> {
    let Some(first) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{name} requires content as its first argument"),
        )]);
    };
    let content = match first {
        Value::Content(_) | Value::LocatedContent(_, _) => first.clone(),
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("{name} requires content as its first argument"),
            )]);
        }
    };
    let mut rest = args.clone();
    rest.remove_positional(0);
    Ok((content, rest))
}

pub(in crate::compiler::eval) fn content_type_field(field: &str) -> Option<Value> {
    let (name, call) = match field {
        "func" => ("content.func", native_content_func_static as _),
        "has" => ("content.has", native_content_has_static as _),
        "at" => ("content.at", native_content_at_static as _),
        "fields" => ("content.fields", native_content_fields_static as _),
        "location" => ("content.location", native_content_location_static as _),
        _ => return None,
    };
    Some(Value::Func(Func::native(name, call)))
}

pub(super) fn static_content_method(
    args: &Args,
    name: &str,
    method: &str,
) -> SourceResult<Value> {
    let (content, rest) = static_content_receiver(args, name)?;
    match content {
        Value::Content(content) => {
            eval_content_method_at(&content, None, method, rest, args.span)
        }
        Value::LocatedContent(content, location) => eval_introspected_content_method_at(
            &content, location, method, rest, args.span,
        ),
        _ => unreachable!("receiver was validated as content"),
    }
}

macro_rules! static_content_native {
    ($fn_name:ident, $public_name:literal, $method:literal) => {
        fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _file: crate::entities::file_id::FileId,
        ) -> SourceResult<Value> {
            static_content_method(args, $public_name, $method)
        }
    };
}

static_content_native!(native_content_func_static, "content.func", "func");
static_content_native!(native_content_has_static, "content.has", "has");
static_content_native!(native_content_at_static, "content.at", "at");
static_content_native!(native_content_fields_static, "content.fields", "fields");
static_content_native!(native_content_location_static, "content.location", "location");

/// Native content element constructor; distinct from the raw symbol value type.
pub(crate) fn native_symbol_element(
    _ctx: &mut crate::compiler::eval::EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    if args.named.contains_key("text") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `text` is positional",
        )
        .with_hint("try removing `text:`")]);
    }
    let Some(value) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: text")]);
    };
    let value_span = args
        .occurrences
        .as_ref()
        .and_then(|items| items.iter().find(|arg| arg.name.is_none()))
        .map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span));
    let Value::Str(text) = value else {
        return Err(vec![SourceDiagnostic::error(
            value_span,
            format!("expected string, found {}", vanilla_type_name(value)),
        )]);
    };
    if args.items.len() > 1 || !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    Ok(Value::Content(crate::entities::content::Content::Symbol(text.clone())))
}

#[cfg(test)]
mod matrix_morphology_tests {
    use super::*;
    #[test]
    fn matrix_morphology_fields_presence_and_native_identity() {
        let matrix = crate::entities::content::Content::math_matrix(
            vec![vec![crate::entities::content::Content::text("a")]],
            ('(', ')'),
        );
        assert_eq!(
            content_set_fields(&matrix)
                .iter()
                .map(|(n, _)| *n)
                .collect::<Vec<_>>(),
            vec!["delim", "rows"]
        );
        assert!(matches!(content_field(&matrix, "align"), ContentField::Unset));
        assert!(matches!(content_field(&matrix, "gap"), ContentField::Undeclared));
        let Value::Func(projected) = content_elem_func(&matrix) else {
            panic!("not function")
        };
        let Value::Module(math) = crate::compiler::stdlib::make_math_module() else {
            panic!("not module")
        };
        let Some(Value::Func(native)) = math.scope().get("mat") else {
            panic!("missing native")
        };
        assert!(std::ptr::fn_addr_eq(
            projected.native_fn_addr().unwrap(),
            native.native_fn_addr().unwrap()
        ));
    }
}

#[cfg(test)]
mod caption_shape_tests {
    use super::*;
    #[test]
    fn caption_shape_fields_and_function_use_native_identity() {
        use crate::entities::content::Content;
        use crate::entities::elements::figure_caption::CaptionPosition;
        let c = Content::figure_caption(
            Content::text("C"),
            Some(CaptionPosition::Top),
            Some(Content::text("S")),
        );
        let fields = content_set_fields(&c);
        assert_eq!(
            fields.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            vec!["position", "separator", "body"]
        );
        let Value::Func(f) = content_elem_func(&c) else { panic!("expected function") };
        assert!(std::ptr::fn_addr_eq(
            f.native_fn_addr().unwrap(),
            crate::compiler::stdlib::native_figure_caption as fn(_, _, _, _) -> _
        ));
        let (name, fields, locatable) =
            crate::compiler::eval::bindings::native_element_fields(&f).unwrap();
        assert_eq!(name, "caption");
        assert!(fields.contains(&"body"));
        assert!(fields.contains(&"kind"));
        assert!(locatable);
    }
}

#[cfg(test)]
mod v16_e_reflection_tests {
    use super::*;
    use crate::entities::content::{
        Content, LocatedContent, RealizedContent, ShowRecipeContent, SourceContent,
    };
    use crate::entities::locator::Locator;
    use crate::entities::style::Styles;
    use crate::entities::style_chain::StyleChain;
    use std::sync::Arc;

    fn carrier(kind: usize, body: Content) -> Content {
        match kind {
            0 => {
                Content::Source(Arc::new(SourceContent { body, span: Span::detached() }))
            }
            1 => Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body,
                textual: false,
                context_id_floor: 0,
                rules: Arc::from([]),
                styles: StyleChain::default_chain(),
                guards: vec![],
                file: crate::entities::file_id::FileId::from_raw(
                    std::num::NonZeroU16::new(7).unwrap(),
                ),
            })),
            2 => Content::Located(Arc::new(LocatedContent {
                body,
                location: Locator::new().next(),
            })),
            3 => Content::Realized(Arc::new(RealizedContent {
                original: Content::text("original is not an equation"),
                visual: body,
            })),
            _ => panic!("invalid test carrier"),
        }
    }

    fn equation_styles() -> Styles {
        Styles::new()
            .push_custom("equation.constructor", Value::Bool(true))
            .push_custom("equation.numbering", Value::Str("(1)".into()))
            .push_custom("equation.number-align", Value::None)
            .push_custom("equation.supplement", Value::Content(Content::text("Eq")))
            .push_custom("equation.alt", Value::None)
    }

    fn method(c: &Content, name: &str, values: Vec<Value>) -> SourceResult<Value> {
        eval_content_method(c, name, Args::positional(values), Span::detached())
    }

    fn fields(c: &Content) -> IndexMap<EcoString, Value, FxBuildHasher> {
        let Value::Dict(values) = method(c, "fields", vec![]).unwrap() else {
            panic!("fields did not return a dictionary")
        };
        values
    }

    #[test]
    fn v16_e_nested_equation_carriers_preserve_fields() {
        let equation = Content::equation(Content::text("x"), true);
        let reference = Content::styled(equation.clone(), equation_styles());
        let expected = fields(&reference);
        assert_eq!(
            expected.keys().map(|name| name.as_str()).collect::<Vec<_>>(),
            vec!["block", "numbering", "number-align", "supplement", "alt", "body"]
        );
        for kind in 0..4 {
            let nested = Content::styled(
                carrier(
                    kind,
                    Content::styled(
                        equation.clone(),
                        Styles::new()
                            .push_custom("equation.constructor", Value::Bool(true)),
                    ),
                ),
                equation_styles(),
            );
            let actual = fields(&nested);
            assert_eq!(
                actual.keys().collect::<Vec<_>>(),
                expected.keys().collect::<Vec<_>>(),
                "carrier {kind} dropped language field keys"
            );
            for (name, value) in &expected {
                assert_eq!(actual.get(name), Some(value), "carrier {kind}/{name}");
            }
        }
    }

    #[test]
    fn v16_e_equation_has_at_precedence_and_explicit_none() {
        let equation = Content::equation(Content::text("x"), true);
        for kind in 0..4 {
            let inner = Content::styled(
                equation.clone(),
                Styles::new()
                    .push_custom("equation.constructor", Value::Bool(true))
                    .push_custom("equation.alt", Value::Str("inner".into())),
            );
            let outer = Content::styled(carrier(kind, inner), equation_styles());
            for key in ["alt", "number-align"] {
                assert_eq!(
                    method(&outer, "has", vec![Value::Str(key.into())]).unwrap(),
                    Value::Bool(true)
                );
                assert_eq!(
                    method(&outer, "at", vec![Value::Str(key.into())]).unwrap(),
                    Value::None
                );
            }
            assert_eq!(
                method(&outer, "has", vec![Value::Str("missing".into())]).unwrap(),
                Value::Bool(false)
            );
            assert!(method(&outer, "at", vec![Value::Str("missing".into())]).is_err());
        }
    }

    #[test]
    fn v16_e_semantic_containers_remain_opaque_to_equation_fields() {
        let equation = Content::equation(Content::text("x"), true);
        for kind in 0..4 {
            let sequence = Content::Sequence(vec![equation.clone()].into());
            assert_eq!(
                fields(&carrier(kind, sequence.clone()))
                    .keys()
                    .map(|name| name.as_str())
                    .collect::<Vec<_>>(),
                vec!["children"]
            );
            for body in [Content::text("x"), sequence, Content::strong(equation.clone())]
            {
                let styled = Content::styled(carrier(kind, body), equation_styles());
                let observed = fields(&styled);
                for key in ["block", "alt", "numbering", "children"] {
                    assert!(!observed.contains_key(key), "carrier {kind}/{key}");
                    assert_eq!(
                        method(&styled, "has", vec![Value::Str(key.into())]).unwrap(),
                        Value::Bool(false),
                        "carrier {kind}/{key}"
                    );
                }
            }
        }
        let reversed = Content::styled(
            Content::Realized(Arc::new(RealizedContent {
                original: equation,
                visual: Content::text("visual is not an equation"),
            })),
            equation_styles(),
        );
        let observed = fields(&reversed);
        for key in ["block", "alt", "numbering", "body"] {
            assert!(!observed.contains_key(key));
        }
    }

    #[test]
    fn v16_e_external_styled_equation_does_not_expose_equation_schema() {
        let equation = Content::styled(
            Content::equation(Content::text("x"), false),
            equation_styles(),
        );
        let styles = Styles::from_iter([crate::entities::style::Style::Fill(
            crate::entities::color::Color::rgb(255, 0, 0),
        )]);
        let external = Content::styled(equation, styles);
        for field in ["body", "block", "numbering", "alt"] {
            assert_eq!(
                method(&external, "has", vec![Value::Str(field.into())]).unwrap(),
                Value::Bool(false),
                "ordinary Styled leaked Equation field {field}"
            );
            assert!(!fields(&external).contains_key(field));
        }
    }

    #[test]
    fn v16_e_snapshots_and_location_precedence_survive_carriers() {
        let mut locator = Locator::new();
        let carried = locator.next();
        let supplied = locator.next();
        let content = carrier(
            0,
            Content::Located(Arc::new(LocatedContent {
                body: carrier(1, carrier(3, Content::text("x"))),
                location: carried,
            })),
        );
        let empty = IndexMap::default();
        let observe = |location, method, args| {
            content_method_with_fields(
                &content,
                location,
                Some(&empty),
                method,
                args,
                Span::detached(),
            )
        };
        assert_eq!(
            observe(None, "fields", Args::positional(vec![])).unwrap(),
            Value::Dict(IndexMap::default())
        );
        assert_eq!(
            observe(None, "has", Args::positional(vec![Value::Str("text".into())]))
                .unwrap(),
            Value::Bool(false)
        );
        let mut args = Args::positional(vec![Value::Str("text".into())]);
        args.named.insert("default".into(), Value::Int(42));
        assert_eq!(observe(None, "at", args).unwrap(), Value::Int(42));
        assert_eq!(
            observe(None, "location", Args::positional(vec![])).unwrap(),
            Value::Location(carried)
        );
        assert_eq!(
            observe(Some(supplied), "location", Args::positional(vec![])).unwrap(),
            Value::Location(supplied)
        );
    }

    #[test]
    fn v16_e_prescribed_native_identities_survive_carriers() {
        for original in
            [Content::Empty, Content::text("x"), Content::strong(Content::text("x"))]
        {
            let Value::Func(expected) = method(&original, "func", vec![]).unwrap() else {
                panic!("expected native function")
            };
            for kind in 0..4 {
                let Value::Func(actual) =
                    method(&carrier(kind, original.clone()), "func", vec![]).unwrap()
                else {
                    panic!("expected native function")
                };
                assert!(std::ptr::fn_addr_eq(
                    actual.native_fn_addr().unwrap(),
                    expected.native_fn_addr().unwrap()
                ));
            }
        }
    }

    #[test]
    fn v16_e_equation_native_identity_requires_valid_constructor_origin() {
        let equation = Content::equation(Content::text("x"), false);
        let marked = Content::styled(equation.clone(), equation_styles());
        for original in [equation, marked.clone()] {
            for kind in 0..4 {
                let Value::Func(actual) =
                    method(&carrier(kind, original.clone()), "func", vec![]).unwrap()
                else {
                    panic!("expected function")
                };
                assert!(std::ptr::fn_addr_eq(
                    actual.native_fn_addr().unwrap(),
                    crate::compiler::stdlib::native_math_equation as fn(_, _, _, _) -> _
                ));
            }
        }
        for marker in [Value::Bool(false), Value::Str("true".into()), Value::None] {
            let external = Content::styled(
                marked.clone(),
                equation_styles().push_custom("equation.constructor", marker),
            );
            assert!(!fields(&external).contains_key("block"));
            let Value::Func(actual) = method(&external, "func", vec![]).unwrap() else {
                panic!("expected function")
            };
            assert!(!std::ptr::fn_addr_eq(
                actual.native_fn_addr().unwrap(),
                crate::compiler::stdlib::native_math_equation as fn(_, _, _, _) -> _
            ));
        }
        assert!(!fields(&marked).contains_key("equation.constructor"));
    }

    #[test]
    fn v16_e_equation_native_aliases_keep_language_identity() {
        use crate::compiler::eval::operators::eval_binary_op;
        use crate::compiler::eval::repr::repr_value;
        use crate::entities::ast::expr::BinOp;

        let Value::Module(math) = crate::compiler::stdlib::make_math_module() else {
            panic!("expected math module")
        };
        let native = math.scope().get("equation").expect("native Equation").clone();
        let alias = native.clone();
        let unrelated = math.scope().get("mat").expect("native Matrix").clone();
        let bare = Content::equation(Content::text("x"), false);
        let marked = Content::styled(bare.clone(), equation_styles());
        for original in [bare, marked.clone()] {
            for kind in 0..4 {
                let projected = method(&carrier(kind, original.clone()), "func", vec![])
                    .expect("Equation function projection");
                for reference in [&native, &alias] {
                    assert_eq!(
                        eval_binary_op(BinOp::Eq, projected.clone(), reference.clone()),
                        Ok(Value::Bool(true)),
                        "carrier {kind} lost Equation language identity"
                    );
                }
                assert_eq!(repr_value(&projected), "equation");
                assert_eq!(
                    eval_binary_op(BinOp::Eq, projected, unrelated.clone()),
                    Ok(Value::Bool(false))
                );
            }
        }
        let external = Content::styled(marked, Styles::new());
        let projected = method(&external, "func", vec![]).unwrap();
        assert_eq!(eval_binary_op(BinOp::Eq, projected, native), Ok(Value::Bool(false)));
    }

    #[test]
    fn v16_e_figure_known_caption_and_metadata_value_are_enumerated() {
        use crate::entities::elements::{figure::FigureElem, metadata::MetadataElem};
        for caption in [Content::Empty, Content::text("caption")] {
            let figure = Content::Figure(Arc::new(FigureElem {
                body: Content::text("body"),
                caption: Some(caption.clone()),
                kind: None,
                resolved_kind: None,
                supplement: None,
                resolved_supplement: None,
            }));
            let observed = fields(&figure);
            assert_eq!(
                observed.keys().map(|key| key.as_str()).collect::<Vec<_>>(),
                vec!["body", "caption"]
            );
            let Some(Value::Content(Content::FigureCaption(value))) =
                observed.get("caption")
            else {
                panic!("missing known caption")
            };
            assert_eq!(value.body, caption);
        }
        let absent = Content::Figure(Arc::new(FigureElem {
            body: Content::text("body"),
            caption: None,
            kind: None,
            resolved_kind: None,
            supplement: None,
            resolved_supplement: None,
        }));
        assert!(!fields(&absent).contains_key("caption"));
        assert!(matches!(content_field(&absent, "kind"), ContentField::Unset));
        for value in
            [Value::None, Value::Int(7), Value::Content(Content::text("metadata"))]
        {
            let metadata = Content::Metadata(Arc::new(MetadataElem {
                value: Box::new(value.clone()),
            }));
            let observed = fields(&metadata);
            assert_eq!(
                observed.keys().map(|key| key.as_str()).collect::<Vec<_>>(),
                vec!["value"]
            );
            assert_eq!(observed.get("value"), Some(&value));
            assert!(matches!(content_field(&metadata, "body"), ContentField::Undeclared));
        }
    }

    #[test]
    fn v16_e_outline_entry_fill_preserves_lexical_presence() {
        use crate::entities::elements::outline_entry::OutlineEntryElem;
        let heading = Content::heading(1, Content::text("heading"));
        for (fill, explicit) in [
            (Some(Content::text("default fill")), false),
            (None, true),
            (Some(Content::text("explicit fill")), true),
        ] {
            let entry = Content::OutlineEntry(Arc::new(OutlineEntryElem::new(
                1,
                heading.clone(),
                fill.clone(),
                explicit,
            )));
            let observed = fields(&entry);
            let expected_keys = if explicit {
                vec!["level", "element", "fill"]
            } else {
                vec!["level", "element"]
            };
            assert_eq!(
                observed.keys().map(|key| key.as_str()).collect::<Vec<_>>(),
                expected_keys
            );
            assert_eq!(observed.get("level"), Some(&Value::Int(1)));
            assert_eq!(observed.get("element"), Some(&Value::Content(heading.clone())));
            if explicit {
                assert_eq!(
                    observed.get("fill"),
                    Some(&fill.map(Value::Content).unwrap_or(Value::None))
                );
            } else {
                assert!(matches!(content_field(&entry, "fill"), ContentField::Unset));
                assert_eq!(
                    method(&entry, "has", vec![Value::Str("fill".into())]).unwrap(),
                    Value::Bool(false)
                );
            }
            assert!(matches!(content_field(&entry, "body"), ContentField::Undeclared));
        }
    }
}

#[cfg(test)]
mod r4_tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::elements::math_cases::MathCasesExplicit;
    use crate::entities::elements::math_vec::MathVecExplicit;
    use crate::entities::layout_types::{HAlign, Length};
    use crate::entities::rel::Rel;

    #[test]
    fn r4_math_fields_preserve_presence_and_native_children() {
        let symbol = Content::Symbol("α".into());
        let gap = Rel { rel: 0.0, abs: Length::em(1.0) };
        let cases = [
            (Content::MathText("12".into()), vec!["text"]),
            (Content::math_vec(vec![symbol.clone()]), vec!["children"]),
            (
                Content::math_cases_full(
                    vec![symbol.clone()],
                    (Some('{'), None),
                    false,
                    gap,
                    MathCasesExplicit::default(),
                ),
                vec!["children"],
            ),
            (
                Content::math_class_override(
                    crate::entities::math_class::MathClass::Relation,
                    symbol.clone(),
                ),
                vec!["class", "body"],
            ),
            (
                Content::math_vec_full(
                    vec![symbol.clone()],
                    ('[', ']'),
                    HAlign::Left,
                    gap,
                    MathVecExplicit { delim: true, align: true, gap: true },
                ),
                vec!["delim", "align", "gap", "children"],
            ),
            (
                Content::math_cases_full(
                    vec![symbol.clone()],
                    (Some('['), Some(']')),
                    true,
                    gap,
                    MathCasesExplicit { delim: true, reverse: true, gap: true },
                ),
                vec!["delim", "reverse", "gap", "children"],
            ),
        ];
        let mut failures = Vec::new();
        for (content, expected) in cases {
            let observed = content_set_fields(&content);
            let keys: Vec<_> = observed.iter().map(|(key, _)| *key).collect();
            if keys != expected {
                failures.push(format!("{content:?}: {keys:?} != {expected:?}"));
            }
            for field in expected {
                if !matches!(content_field(&content, field), ContentField::Set(_))
                    || content.get_field(field).is_none()
                {
                    failures.push(format!("{content:?}: missing {field}"));
                }
            }
            assert!(matches!(
                content_field(&content, "unknown"),
                ContentField::Undeclared
            ));
            if let Some((_, Value::Array(children))) =
                observed.iter().find(|(k, _)| *k == "children")
            {
                assert_eq!(children, &vec![Value::Content(symbol.clone())]);
            }
        }
        for content in [
            Content::math_vec(vec![]),
            Content::math_cases_full(
                vec![],
                (Some('{'), None),
                false,
                gap,
                MathCasesExplicit::default(),
            ),
        ] {
            for field in ["delim", "gap"] {
                if !matches!(content_field(&content, field), ContentField::Unset) {
                    failures.push(format!("{content:?}: {field} must be Unset"));
                }
            }
        }
        assert!(failures.is_empty(), "{failures:?}");
    }
}
