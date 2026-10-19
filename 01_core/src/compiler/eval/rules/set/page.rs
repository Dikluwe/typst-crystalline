//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/set/page.md
//! @prompt-hash e9af2257
//! @layer L1
//! @updated 2026-09-30
//!
//! Branch `#set page`.

use super::*;

pub(super) fn eval_set_page(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // #set page(width: .., height: .., margin: ..) — Passo 81.
    // Valores ausentes ficam None e preservam o valor actual em layout.
    // P636: rejeitar tipos inválidos em vez de ignorar silenciosamente.
    // **P757** — `em` em dimensões de página deve resolver contra o
    // tamanho de fonte activo na chain (default 11 pt), não contra zero.
    // Anteriormente usava-se `l.abs.to_pt()`, que descartava a componente
    // `em` e produzia `0pt` silenciosamente para valores como `7em`.
    // **P867** — `width`/`height` aceitam `auto`; `margin` mantém-se
    // length/float/int/none neste passo.
    let size_pt = engine.styles.size();
    fn extract_page_dimension(
        val: &Value,
        span: Span,
        size_pt: f64,
    ) -> SourceResult<Option<crate::entities::layout_types::PageDimension>> {
        match val {
            Value::Length(l) => {
                Ok(Some(crate::entities::layout_types::PageDimension::Length(
                    l.resolve_pt(size_pt),
                )))
            }
            Value::Float(f) => {
                Ok(Some(crate::entities::layout_types::PageDimension::Length(*f)))
            }
            Value::Int(i) => {
                Ok(Some(crate::entities::layout_types::PageDimension::Length(*i as f64)))
            }
            Value::Auto => Ok(Some(crate::entities::layout_types::PageDimension::Auto)),
            Value::None => Ok(None),
            other => Err(vec![type_mismatch("length, float, int, or auto", other, span)]),
        }
    }
    fn extract_pt(val: &Value, span: Span, size_pt: f64) -> SourceResult<Option<f64>> {
        match val {
            Value::Length(l) => Ok(Some(l.resolve_pt(size_pt))),
            Value::Float(f) => Ok(Some(*f)),
            Value::Int(i) => Ok(Some(*i as f64)),
            Value::None => Ok(None),
            other => Err(vec![type_mismatch("length, float, or int", other, span)]),
        }
    }
    fn extract_rel_length(
        val: &Value,
        span: Span,
    ) -> SourceResult<crate::entities::rel::Rel<crate::entities::layout_types::Length>>
    {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        match val {
            Value::Relative(value) => Ok(*value),
            Value::Ratio(value) => Ok(Rel { rel: value.get(), abs: Length::ZERO }),
            Value::Length(value) => Ok(Rel { rel: 0.0, abs: *value }),
            Value::Float(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value) }),
            Value::Int(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value as f64) }),
            other => Err(vec![type_mismatch("relative length", other, span)]),
        }
    }
    let mut width: Option<crate::entities::layout_types::PageDimension> = None;
    let mut paper = None;
    let mut flipped = None;
    let mut binding = None;
    let mut height: Option<crate::entities::layout_types::PageDimension> = None;
    let mut margin: Option<crate::entities::layout_types::PageMarginSpec> = None;
    let mut margin_left = None;
    let mut margin_right = None;
    let mut margin_top = None;
    let mut margin_bottom = None;
    let mut numbering = None;
    let mut number_align = None;
    let mut header = None;
    let mut header_ascent = None;
    let mut footer = None;
    let mut footer_descent = None;
    let mut supplement = None;
    let mut columns: Option<usize> = None;
    let mut bleed = None;
    let mut fill = None;
    let mut background = None;
    let mut foreground = None;
    for arg in set.args().items() {
        if let Arg::Named(named) = arg {
            let key = named.name().as_str();
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            let span = named.expr().span();
            match key {
                "paper" => {
                    paper = match val {
                        Value::Str(s) => {
                            crate::entities::page_geometry::Paper::from_name(&s)
                                .ok_or_else(|| {
                                    vec![SourceDiagnostic::error(
                                        span,
                                        "unknown paper size",
                                    )]
                                })
                                .map(Some)?
                        }
                        other => {
                            return Err(vec![type_mismatch("string", &other, span)]);
                        }
                    }
                }
                "flipped" => {
                    flipped = match val {
                        Value::Bool(v) => Some(v),
                        other => {
                            return Err(vec![type_mismatch("bool", &other, span)]);
                        }
                    }
                }
                "binding" => {
                    binding = match val {
                        Value::Auto => {
                            Some(crate::entities::page_geometry::PageBinding::Auto)
                        }
                        Value::Align(a)
                            if matches!(
                                (a.v, a.h),
                                (None, Some(crate::entities::layout_types::HAlign::Left))
                            ) =>
                        {
                            Some(crate::entities::page_geometry::PageBinding::Left)
                        }
                        Value::Align(a)
                            if matches!(
                                (a.v, a.h),
                                (
                                    None,
                                    Some(crate::entities::layout_types::HAlign::Right)
                                )
                            ) =>
                        {
                            Some(crate::entities::page_geometry::PageBinding::Right)
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, left, or right",
                                &other,
                                span,
                            )]);
                        }
                    }
                }
                "width" => width = extract_page_dimension(&val, span, size_pt)?,
                "height" => height = extract_page_dimension(&val, span, size_pt)?,
                "margin" => match &val {
                    Value::Dict(d) => {
                        if let Some(key) = d.keys().find(|k| {
                            !matches!(
                                k.as_str(),
                                "left"
                                    | "right"
                                    | "top"
                                    | "bottom"
                                    | "inside"
                                    | "outside"
                                    | "x"
                                    | "y"
                                    | "rest"
                            )
                        }) {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                format!("unknown margin key: {key}"),
                            )]);
                        }
                        let logical =
                            d.get("inside").is_some() || d.get("outside").is_some();
                        let physical =
                            d.get("left").is_some() || d.get("right").is_some();
                        if logical && physical {
                            return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "`inside` and `outside` are mutually exclusive with `left` and `right`",
                                )]);
                        }
                        let extract_field = |k: &str| -> SourceResult<Option<f64>> {
                            if let Some(v) = d.get(k) {
                                extract_pt(v, span, size_pt)
                            } else {
                                Ok(None)
                            }
                        };
                        let mx = extract_field("x")?;
                        let my = extract_field("y")?;
                        let rest = extract_field("rest")?;
                        margin_left =
                            extract_field(if logical { "inside" } else { "left" })?
                                .or(mx)
                                .or(rest);
                        margin_right =
                            extract_field(if logical { "outside" } else { "right" })?
                                .or(mx)
                                .or(rest);
                        margin_top = extract_field("top")?.or(my).or(rest);
                        margin_bottom = extract_field("bottom")?.or(my).or(rest);
                        margin = Some(crate::entities::layout_types::PageMarginSpec {
                            left: margin_left,
                            right: margin_right,
                            top: margin_top,
                            bottom: margin_bottom,
                            two_sided: if logical || physical {
                                Some(logical)
                            } else {
                                None
                            },
                        });
                    }
                    Value::Auto => {
                        margin =
                            Some(crate::entities::layout_types::PageMarginSpec::auto());
                    }
                    other => {
                        if let Some(m) = extract_pt(other, span, size_pt)? {
                            margin_left = Some(m);
                            margin_right = Some(m);
                            margin_top = Some(m);
                            margin_bottom = Some(m);
                            margin = Some(
                                crate::entities::layout_types::PageMarginSpec::uniform(m),
                            );
                        }
                    }
                },
                "numbering" => {
                    numbering = match val {
                        Value::Str(s) => {
                            Some(Some(crate::entities::numbering::Numbering::Pattern(s)))
                        }
                        Value::Func(f) => {
                            Some(Some(crate::entities::numbering::Numbering::Func(f)))
                        }
                        Value::None => Some(None),
                        other => {
                            return Err(vec![type_mismatch(
                                "string, function, or none",
                                &other,
                                span,
                            )]);
                        }
                    };
                }
                "number-align" => {
                    let Value::Align(align) = val else {
                        return Err(vec![type_mismatch("alignment", &val, span)]);
                    };
                    let horizontal =
                        align.h.unwrap_or(crate::entities::layout_types::HAlign::Center);
                    let vertical = match align
                        .v
                        .unwrap_or(crate::entities::layout_types::VAlign::Bottom)
                    {
                        crate::entities::layout_types::VAlign::Top => {
                            crate::entities::page_running::PageNumberVAlign::Top
                        }
                        crate::entities::layout_types::VAlign::Bottom => {
                            crate::entities::page_running::PageNumberVAlign::Bottom
                        }
                        crate::entities::layout_types::VAlign::Horizon => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                "page number-align cannot use horizon".to_string(),
                            )]);
                        }
                    };
                    number_align = Some(crate::entities::page_running::PageNumberAlign {
                        horizontal,
                        vertical,
                    });
                }
                "header" => {
                    header = Some(match val {
                        Value::Auto => crate::entities::page_running::PageMarginal::Auto,
                        Value::None => crate::entities::page_running::PageMarginal::None,
                        Value::Content(content) => {
                            crate::entities::page_running::PageMarginal::Content(
                                std::sync::Arc::new(content),
                            )
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, none, or content",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "header-ascent" => header_ascent = Some(extract_rel_length(&val, span)?),
                "footer" => {
                    footer = Some(match val {
                        Value::Auto => crate::entities::page_running::PageMarginal::Auto,
                        Value::None => crate::entities::page_running::PageMarginal::None,
                        Value::Content(content) => {
                            crate::entities::page_running::PageMarginal::Content(
                                std::sync::Arc::new(content),
                            )
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, none, or content",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "footer-descent" => {
                    footer_descent = Some(extract_rel_length(&val, span)?)
                }
                "supplement" => {
                    supplement = Some(match val {
                        Value::Auto => {
                            crate::entities::page_supplement::PageSupplement::Auto
                        }
                        Value::None => {
                            crate::entities::page_supplement::PageSupplement::None
                        }
                        Value::Content(content) => {
                            crate::entities::page_supplement::PageSupplement::Content(
                                std::sync::Arc::new(content),
                            )
                        }
                        Value::Str(text) => {
                            crate::entities::page_supplement::PageSupplement::Content(
                                std::sync::Arc::new(Content::text(text)),
                            )
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, none, string, or content",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "columns" => {
                    columns = match val {
                        Value::Int(n) if n >= 1 => Some(n as usize),
                        Value::Int(n) if n < 1 => {
                            return Err(vec![SourceDiagnostic::error(
                                named.span(),
                                "columns must be at least 1".to_string(),
                            )]);
                        }
                        Value::None => None,
                        other => {
                            return Err(vec![type_mismatch("int", &other, span)]);
                        }
                    };
                }
                "bleed" => {
                    use crate::entities::page_canvas::PageBleedSpec;
                    bleed = Some(match &val {
                        Value::Dict(dict) => {
                            if let Some(key) = dict.keys().find(|key| {
                                !matches!(
                                    key.as_str(),
                                    "left"
                                        | "right"
                                        | "top"
                                        | "bottom"
                                        | "inside"
                                        | "outside"
                                        | "x"
                                        | "y"
                                        | "rest"
                                )
                            }) {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("unknown bleed key: {key}"),
                                )]);
                            }
                            let logical = dict.get("inside").is_some()
                                || dict.get("outside").is_some();
                            let physical =
                                dict.get("left").is_some() || dict.get("right").is_some();
                            if logical && physical {
                                return Err(vec![SourceDiagnostic::error(
                                        span,
                                        "`inside` and `outside` are mutually exclusive with `left` and `right`".to_string(),
                                    )]);
                            }
                            let field = |key: &str| -> SourceResult<
                                Option<
                                    crate::entities::rel::Rel<
                                        crate::entities::layout_types::Length,
                                    >,
                                >,
                            > {
                                dict.get(key)
                                    .map(|value| extract_rel_length(value, span))
                                    .transpose()
                            };
                            let x = field("x")?;
                            let y = field("y")?;
                            let rest = field("rest")?;
                            PageBleedSpec {
                                left: field(if logical { "inside" } else { "left" })?
                                    .or(x)
                                    .or(rest),
                                right: field(if logical { "outside" } else { "right" })?
                                    .or(x)
                                    .or(rest),
                                top: field("top")?.or(y).or(rest),
                                bottom: field("bottom")?.or(y).or(rest),
                                two_sided: if logical || physical {
                                    Some(logical)
                                } else {
                                    None
                                },
                            }
                        }
                        Value::Auto => {
                            return Err(vec![type_mismatch(
                                "relative length or dictionary",
                                &val,
                                span,
                            )]);
                        }
                        Value::None
                        | Value::Bool(..)
                        | Value::Int(..)
                        | Value::Float(..)
                        | Value::Str(..)
                        | Value::Array(..)
                        | Value::Module(..)
                        | Value::Datetime(..)
                        | Value::Func(..)
                        | Value::Content(..)
                        | Value::LocatedContent(..)
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
                        | Value::Symbol(..)
                        | Value::Args(..)
                        | Value::State(..)
                        | Value::Counter(..)
                        | Value::Label(..)
                        | Value::Dir(..)
                        | Value::Path(..)
                        | Value::Type(..)
                        | Value::SpotColorant(..) => {
                            PageBleedSpec::uniform(extract_rel_length(&val, span)?)
                        }
                    });
                }
                "fill" => {
                    fill = Some(match val {
                        Value::Auto => crate::entities::page_canvas::PageFill::Auto,
                        Value::None => crate::entities::page_canvas::PageFill::None,
                        Value::Color(color) => {
                            crate::entities::page_canvas::PageFill::Paint(color.into())
                        }
                        Value::Gradient(gradient) => {
                            crate::entities::page_canvas::PageFill::Paint(gradient.into())
                        }
                        Value::Tiling(tiling) => {
                            crate::entities::page_canvas::PageFill::Paint(
                                (*tiling).clone().into(),
                            )
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, none, or paint",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "background" => {
                    background = Some(match val {
                        Value::None => None,
                        Value::Content(content) => Some(std::sync::Arc::new(content)),
                        other => {
                            return Err(vec![type_mismatch(
                                "content or none",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "foreground" => {
                    foreground = Some(match val {
                        Value::None => None,
                        Value::Content(content) => Some(std::sync::Arc::new(content)),
                        other => {
                            return Err(vec![type_mismatch(
                                "content or none",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        named.span(),
                        format!("unknown page property: {key}"),
                    )]);
                }
            }
        }
    }

    // Empurrar as dimensões da página para a StyleChain para que o layout() possa ler
    if let Some(w) = width {
        let v = match w {
            crate::entities::layout_types::PageDimension::Auto => Value::Auto,
            crate::entities::layout_types::PageDimension::Length(x) => Value::Float(x),
        };
        *engine.styles = engine.styles.push_custom("page.width", v);
    }
    if let Some(h) = height {
        let v = match h {
            crate::entities::layout_types::PageDimension::Auto => Value::Auto,
            crate::entities::layout_types::PageDimension::Length(x) => Value::Float(x),
        };
        *engine.styles = engine.styles.push_custom("page.height", v);
    }
    if let Some(ml) = margin_left {
        *engine.styles = engine.styles.push_custom("page.margin-left", Value::Float(ml));
    }
    if let Some(mr) = margin_right {
        *engine.styles = engine.styles.push_custom("page.margin-right", Value::Float(mr));
    }
    if let Some(mt) = margin_top {
        *engine.styles = engine.styles.push_custom("page.margin-top", Value::Float(mt));
    }
    if let Some(mb) = margin_bottom {
        *engine.styles =
            engine.styles.push_custom("page.margin-bottom", Value::Float(mb));
    }

    Ok(Value::Content(Content::SetPage(std::sync::Arc::new(
        crate::entities::content::SetPageDelta {
            paper,
            flipped,
            binding,
            width,
            height,
            margin,
            numbering,
            number_align,
            header,
            header_ascent,
            footer,
            footer_descent,
            supplement,
            columns,
            bleed,
            fill,
            background,
            foreground,
        },
    ))))
}

#[cfg(test)]
mod v16_c_structural_tests {
    use super::*;

    use crate::contracts::world::World;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use crate::entities::{file_id::FileId, font_book::FontBook, source::Source};
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                ),
            }
        }
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
        ) -> Option<Datetime> {
            None
        }
        fn include_source(&self, _: FileId, _: &str) -> Result<Source, String> {
            panic!("refused include accessed World")
        }
        fn include_path(
            &self,
            _: &crate::entities::path::RootedPath,
        ) -> Result<Source, String> {
            panic!("refused include accessed World")
        }
    }

    fn with_engine(run: impl FnOnce(&mut Scopes<'_>, &mut EvalContext, &mut Engine<'_>)) {
        use comemo::Track;
        let world = TestWorld::new("");
        let library =
            Library::with_global(crate::compiler::stdlib::make_stdlib_with_features(
                &crate::contracts::world::SysInputs::default(),
                crate::entities::compiler_features::Features::empty(),
            ));
        let mut scopes = Scopes::new(Some(&library));
        let mut ctx = EvalContext::new();
        let mut styles = crate::entities::style_chain::StyleChain::default_chain();
        let mut rules = std::sync::Arc::from([]);
        let mut guards = Vec::new();
        let route = crate::entities::world_types::Route::root().with_id(world.main());
        let metrics = crate::compiler::layout::FixedMetrics;
        let mut sink = crate::entities::sink::Sink::new();
        let mut tracked_sink = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked_sink,
        };
        run(&mut scopes, &mut ctx, &mut engine);
    }

    #[test]
    fn v16_c_page_bleed_casts_and_dictionary_precedence() {
        use crate::entities::ast::AstNode;
        for (value, expected) in [
            ("2pt", Some((2.0, 0.0))),
            ("2", Some((2.0, 0.0))),
            ("2.0", Some((2.0, 0.0))),
            ("10%", Some((0.0, 0.1))),
            ("2pt+10%", Some((2.0, 0.1))),
            ("auto", None),
            ("none", None),
            ("true", None),
        ] {
            let text = format!("set page(bleed:{value})");
            let source = Source::new_with_parser(
                TestWorld::new("").main(),
                text.clone(),
                crate::compiler::parse::parse_code,
            );
            let set = source
                .root()
                .children()
                .find_map(|n| {
                    if let Some(Expr::SetRule(s)) = Expr::from_untyped(n) {
                        Some(s)
                    } else {
                        None
                    }
                })
                .unwrap();
            with_engine(|scopes, ctx, engine| {
                let result = eval_set_page(set, scopes, ctx, engine);
                if let Some((abs, rel)) = expected {
                    let Value::Content(Content::SetPage(delta)) = result.unwrap() else {
                        panic!("page delta")
                    };
                    let bleed = delta.bleed.as_ref().unwrap();
                    for side in [bleed.left, bleed.right, bleed.top, bleed.bottom] {
                        let side = side.unwrap();
                        assert_eq!(
                            side.abs,
                            crate::entities::layout_types::Length::pt(abs)
                        );
                        assert_eq!(side.rel, rel);
                    }
                } else {
                    let error = result.unwrap_err();
                    assert!(error[0].message.contains(if value == "auto" {
                        "relative length or dictionary"
                    } else {
                        "relative length"
                    }));
                    assert_eq!(
                        source.span_byte_range(error[0].span),
                        Some(15..text.len() - 1)
                    );
                }
            });
        }
        for (value, message) in [
            ("(inside:1pt,left:2pt)", "mutually exclusive"),
            ("(invalid:1pt)", "unknown bleed key"),
            ("(left:true)", "relative length"),
        ] {
            let world = TestWorld::new(&format!("#set page(bleed:{value})"));
            let error =
                crate::compiler::eval::tests::eval_for_test(&world, &world.source)
                    .unwrap_err();
            assert!(error[0].message.contains(message));
        }
    }
    #[test]
    fn v16_c_page_bleed_sides_override_axes_and_rest() {
        use crate::entities::ast::AstNode;
        let source = Source::new_with_parser(
            TestWorld::new("").main(),
            "set page(bleed: (left: 1pt, x: 2pt, top: 3pt, y: 4pt, rest: 5pt))".into(),
            crate::compiler::parse::parse_code,
        );
        let set = source
            .root()
            .children()
            .find_map(|n| {
                if let Some(Expr::SetRule(s)) = Expr::from_untyped(n) {
                    Some(s)
                } else {
                    None
                }
            })
            .unwrap();
        with_engine(|scopes, ctx, engine| {
            let Value::Content(Content::SetPage(delta)) =
                eval_set_page(set, scopes, ctx, engine).unwrap()
            else {
                panic!("page delta")
            };
            let bleed = delta.bleed.as_ref().unwrap();
            for (side, points) in [
                (bleed.left, 1.0),
                (bleed.right, 2.0),
                (bleed.top, 3.0),
                (bleed.bottom, 4.0),
            ] {
                assert_eq!(
                    side.unwrap().abs,
                    crate::entities::layout_types::Length::pt(points)
                );
            }
            assert_eq!(bleed.two_sided, Some(false));
        });
    }
}
