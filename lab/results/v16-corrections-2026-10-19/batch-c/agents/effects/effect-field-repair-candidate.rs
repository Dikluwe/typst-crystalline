fn content_has_state_or_counter(content: &crate::entities::content::Content) -> bool {
    fn value_has_update(value: &Value) -> bool {
        match value {
            Value::Content(content) => content_has_state_or_counter(content),
            Value::LocatedContent(content, _) => {
                content_has_state_or_counter(content.content())
            }
            Value::Array(values) => values.iter().any(value_has_update),
            Value::Float(..)
            | Value::Decimal(..)
            | Value::None
            | Value::Bool(..)
            | Value::Int(..)
            | Value::Str(..)
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
            | Value::Duration(..)
            | Value::Version(..)
            | Value::Selector(..)
            | Value::Symbol(..)
            | Value::Args(..)
            | Value::State(..)
            | Value::Counter(..)
            | Value::Label(..)
            | Value::Dir(..)
            | Value::Path(..)
            | Value::Type(..)
            | Value::SpotColorant(..) => false,
        }
    }
    let mut found = false;
    content.visit_content_static(&mut |node| {
        if found {
            return false;
        }
        found = match node {
            Content::StateUpdate(_) | Content::CounterUpdate(_) => true,
            Content::Metadata(metadata) => value_has_update(&metadata.value),
            Content::Ref(reference) => reference
                .supplement
                .as_ref()
                .is_some_and(content_has_state_or_counter),
            Content::Dynamic(element) => element
                .as_any()
                .downcast_ref::<crate::entities::elements::raw::RawLineElem>()
                .is_some_and(|line| content_has_state_or_counter(&line.body)),
            Content::Styled(_) => {
                node.constructor_heading().is_some_and(|(_, styles)| {
                    styles
                        .delta()
                        .custom
                        .iter()
                        .rev()
                        .find(|(key, _)| key.as_str() == "heading.constructor.supplement")
                        .is_some_and(|(_, value)| value_has_update(value))
                })
            }
            Content::PageRun(page) => {
                // Page properties are deferred; the static public child is its body.
                found = content_has_state_or_counter(&page.body);
                return false;
            }
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Par { .. }
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
            | Content::CounterDisplay(_)
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
            | Content::State(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_) => false,
        };
        !found
    });
    found
}

/// **P718** — lookahead para o hint de `Expr::Array` (mirror do
/// `items.all(...)` de `ast::Array::eval`, vanilla `code.rs:253-265`):
/// devolve `true` sse todos os itens restantes forem `Spread` cuja
/// expressão avalia a `Value::Dict`. Qualquer `Pos`, spread de outro tipo,
/// ou erro de avaliação → `false` (short-circuit, sem consumir o resto).
fn remaining_are_dict_spreads(
    items: &[ArrayItem],
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> bool {
    for item in items {
        match item {
            ArrayItem::Spread(s) => match eval_expr(s.expr(), scopes, ctx, engine) {
                Ok(Value::Dict(_)) => continue,
                _ => return false,
            },
            ArrayItem::Pos(_) => return false,
        }
    }
    true
}
