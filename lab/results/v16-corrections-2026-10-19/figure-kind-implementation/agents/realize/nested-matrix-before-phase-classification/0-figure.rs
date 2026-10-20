//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/figure.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-10-19

use crate::compiler::eval::{call_dispatch, value_to_display_content, EvalContext};
use crate::compiler::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::content::{Content, LocatedContent};
use crate::entities::elements::figure::{FigureElem, FigureKind, FigureSupplement};
use crate::entities::engine::Engine;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use std::sync::Arc;

/// Detect Figure work in actual emitted output without preparing any occurrence.
pub(in crate::compiler) fn needs_preparation(content: &Content) -> bool {
    let mut found = false;
    content.visit_content_static(&mut |node| {
        if found {
            return false;
        }
        if matches!(node, Content::Figure(_)) {
            found = true;
            return false;
        }
        true
    });
    found
}

pub(in crate::compiler) fn is_figure(mut content: &Content) -> bool {
    loop {
        content = content.source_body();
        if let Content::Located(e) = content {
            content = &e.body;
        } else if let Content::Styled(e) = content {
            content = &e.body;
        } else {
            return matches!(content, Content::Figure(_));
        }
    }
}

fn resolve_kind(
    figure: &FigureElem,
    styles: &StyleChain,
    span: Span,
) -> SourceResult<FigureKind> {
    super::figure_kind::validate_body(&figure.body)?;
    let explicit = if let Some(kind) = &figure.kind {
        kind.clone()
    } else if let Some(value) = styles.custom("figure.kind") {
        super::figure_kind::kind_from_value(value, span)?
    } else {
        None
    };
    explicit
        .map(Ok)
        .unwrap_or_else(|| super::figure_kind::infer(&figure.body))
}

pub(in crate::compiler) fn preview(
    content: &Content,
    styles: &StyleChain,
) -> SourceResult<Content> {
    if let Content::Source(e) = content {
        return preview(&e.body, styles).map(|c| c.with_source_span(e.span));
    }
    if let Content::Located(e) = content {
        return preview(&e.body, styles).map(|body| {
            Content::Located(Arc::new(LocatedContent { body, location: e.location }))
        });
    }
    if let Content::Figure(e) = content {
        if e.resolved_supplement.is_some() {
            return Ok(content.clone());
        }
        let mut prepared = (**e).clone();
        prepared.resolved_kind = Some(resolve_kind(e, styles, content.source_span())?);
        return Ok(Content::Figure(Arc::new(prepared)));
    }
    Ok(content.clone())
}

pub(in crate::compiler) fn supplement_from_value(
    value: &Value,
    span: Span,
) -> SourceResult<FigureSupplement> {
    if matches!(value, Value::Auto) {
        return Ok(FigureSupplement::Auto);
    }
    if matches!(value, Value::None) {
        return Ok(FigureSupplement::None);
    }
    if let Value::Func(function) = value {
        return Ok(FigureSupplement::Func { function: function.clone(), span });
    }
    if let Value::Content(content) = value {
        return Ok(FigureSupplement::Content(content.clone()));
    }
    if let Value::LocatedContent(content, _) = value {
        return Ok(FigureSupplement::Content(content.content().clone()));
    }
    if let Value::Str(text) = value {
        return Ok(FigureSupplement::Content(Content::text(text.clone())));
    }
    if let Value::Symbol(symbol) = value {
        return Ok(FigureSupplement::Content(Content::text(symbol.value.clone())));
    }
    Err(vec![SourceDiagnostic::error(
        span,
        format!(
            "expected content, function, none, or auto, found {}",
            crate::compiler::eval::operators::error_formatting::vanilla_type_name(value)
        ),
    )])
}

fn localizable(kind: &FigureKind) -> Option<&'static str> {
    for (name, function) in [
        ("table", Func::native("table", crate::compiler::stdlib::native_table)),
        ("raw", Func::native("raw", crate::compiler::stdlib::native_raw)),
        ("image", Func::native("image", crate::compiler::stdlib::native_image)),
    ] {
        if *kind == FigureKind::Element(function) {
            return Some(name);
        }
    }
    None
}

pub(in crate::compiler) fn prepare(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if let Content::Source(e) = content {
        return prepare(&e.body, ctx, engine)
            .map(|c| c.with_source_span(e.span))
            .map_err(|mut errors| {
                for error in &mut errors {
                    if error.span.is_detached() {
                        error.span = e.span;
                    }
                }
                errors
            });
    }
    if let Content::Located(e) = content {
        return prepare(&e.body, ctx, engine).map(|body| {
            Content::Located(Arc::new(LocatedContent { body, location: e.location }))
        });
    }
    let Content::Figure(e) = content else {
        return Ok(content.clone());
    };
    if e.resolved_supplement.is_some() {
        return Ok(content.clone());
    }
    let kind = resolve_kind(e, engine.styles, content.source_span())?;
    let supplement = if let Some(supplement) = &e.supplement {
        supplement.clone()
    } else if let Some(value) = engine.styles.custom("figure.supplement") {
        let span = if let Some(Value::Args(origin)) =
            engine.styles.custom("figure.supplement-origin")
        {
            origin.span
        } else if let Value::Func(function) = value {
            function.diagnostic_span()
        } else {
            content.source_span()
        };
        supplement_from_value(value, span)?
    } else {
        FigureSupplement::Auto
    };
    let resolved = match supplement {
        FigureSupplement::None => None,
        FigureSupplement::Content(content) => Some(content),
        FigureSupplement::Auto => {
            if let Some(name) = localizable(&kind) {
                Some(Content::text(
                    crate::compiler::lang::figure_supplement::figure_supplement_for_lang(
                        name,
                        engine.styles.lang().as_ref(),
                    ),
                ))
            } else if engine
                .styles
                .custom("figure.numbering")
                .is_none_or(|value| !matches!(value, Value::None))
            {
                return Err(vec![SourceDiagnostic::error(
                    content.source_span(),
                    "please specify the figure's supplement",
                )]);
            } else {
                Some(Content::Empty)
            }
        }
        FigureSupplement::Func { function, span } => {
            let target = super::figure_kind::find(&e.body, &kind)?
                .unwrap_or_else(|| e.body.clone());
            let previous_context = std::mem::replace(&mut ctx.in_context, true);
            let previous_location = ctx.current_location.take();
            let mut scopes = Scopes::new(None);
            let mut args = Args::positional(vec![Value::Content(target)]);
            args.span = span;
            let result =
                call_dispatch::apply_func(function, args, &mut scopes, ctx, engine);
            ctx.in_context = previous_context;
            ctx.current_location = previous_location;
            Some(value_to_display_content(result?).unwrap_or(Content::Empty))
        }
    };
    let mut prepared = (**e).clone();
    prepared.resolved_kind = Some(kind);
    prepared.resolved_supplement = Some(resolved);
    Ok(Content::Figure(Arc::new(prepared)))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn raw_figure() -> Content {
        Content::Figure(Arc::new(FigureElem {
            body: Content::raw("body", None, false),
            caption: None,
            kind: None,
            resolved_kind: None,
            supplement: None,
            resolved_supplement: None,
        }))
    }
    fn kind(content: &Content) -> &FigureKind {
        let Content::Figure(e) = content else { panic!("not a Figure") };
        e.resolved_kind.as_ref().unwrap()
    }

    #[test]
    fn figure_kind_preview_distinguishes_omitted_auto_and_explicit_style_precedence() {
        let unprepared = raw_figure();
        let chain = StyleChain::default_chain().push_custom(
            "figure.kind",
            Value::Func(Func::native("table", crate::compiler::stdlib::native_table)),
        );
        assert_eq!(
            *kind(&preview(&unprepared, &chain).unwrap()),
            FigureKind::Element(Func::native(
                "table",
                crate::compiler::stdlib::native_table
            ))
        );
        let e = if let Content::Figure(e) = &unprepared { e } else { unreachable!() };
        assert!(e.resolved_kind.is_none());
        assert!(e.resolved_supplement.is_none());
        let mut explicit = (**e).clone();
        explicit.kind = Some(None);
        assert_eq!(
            *kind(&preview(&Content::Figure(Arc::new(explicit)), &chain).unwrap()),
            FigureKind::Element(Func::native("raw", crate::compiler::stdlib::native_raw))
        );
        let second = StyleChain::default_chain()
            .push_custom("figure.kind", Value::Str("custom".into()));
        assert_eq!(
            *kind(&preview(&unprepared, &second).unwrap()),
            FigureKind::Name("custom".into())
        );
    }

    #[test]
    fn figure_kind_preview_has_no_supplement_callback_and_preserves_prepared_alias() {
        let Content::Figure(e) = raw_figure() else { unreachable!() };
        let mut figure = (*e).clone();
        figure.supplement = Some(FigureSupplement::Func {
            function: Func::native("panic", |_, _, _, _| {
                panic!("preview executed supplement")
            }),
            span: Span::detached(),
        });
        let previewed = preview(
            &Content::Figure(Arc::new(figure.clone())),
            &StyleChain::default_chain(),
        )
        .unwrap();
        let Content::Figure(previewed) = previewed else { unreachable!() };
        assert!(previewed.resolved_supplement.is_none());
        figure.resolved_kind = Some(FigureKind::Name("retained".into()));
        figure.resolved_supplement = Some(None);
        let prepared = Content::Figure(Arc::new(figure));
        assert_eq!(
            *kind(&preview(&prepared, &StyleChain::default_chain()).unwrap()),
            FigureKind::Name("retained".into())
        );
    }

    #[test]
    fn figure_kind_localization_is_nominal_and_pointer_based() {
        assert_eq!(
            localizable(&FigureKind::Element(Func::native(
                "alias",
                crate::compiler::stdlib::native_table
            ))),
            Some("table")
        );
        assert_eq!(localizable(&FigureKind::Name("table".into())), None);
        assert_eq!(
            localizable(&FigureKind::Element(Func::native(
                "table",
                crate::compiler::stdlib::native_heading
            ))),
            None
        );
    }
}

#[cfg(test)]
mod nested_supplement_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::{
        font_book::FontBook,
        source::Source,
        world_types::{
            Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Sink,
            Traced,
        },
    };
    use comemo::Track;

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> crate::entities::file_id::FileId {
            self.source.id()
        }
        fn source(&self, _: crate::entities::file_id::FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: crate::entities::file_id::FileId) -> FileResult<Bytes> {
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
    }
    fn evaluate(source: &str) -> SourceResult<Content> {
        let world = TestWorld {
            source: Source::detached(source),
            library: Library::new(),
            book: FontBook::new(),
        };
        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        crate::compiler::eval::eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &world.source,
            &crate::entities::element_registry::ElementRegistry::new(),
        )
        .map(|module| module.content().cloned().unwrap_or(Content::Empty))
    }

    #[test]
    fn figure_kind_nested_supplement_prepares_only_when_caption_or_reference_emits_it() {
        let inner="figure([INNER],kind:table,caption:[INNER-CAP],supplement:it=>panic(\"INNER_PREPARATION\"))";
        for supplement in [inner.to_string(), format!("it=>{inner}")] {
            for (name,body,expected_error) in [
                ("caption",format!("#figure([OUTER],caption:[OUTER-CAP],supplement:{supplement})"),true),
                ("no-caption",format!("#figure([OUTER],supplement:{supplement})"),false),
                ("numbering-none",format!("#figure([OUTER],numbering:none,caption:[OUTER-CAP],supplement:{supplement})"),false),
                ("reference",format!("#figure([OUTER],supplement:{supplement}) <outer>\n@outer"),true),
                ("unused",format!("#let unused=figure([OUTER],caption:[OUTER-CAP],supplement:{supplement})\n[VISIBLE]"),false),
                ("own-hidden",format!("#show figure:none\n#figure([OUTER],caption:[OUTER-CAP],supplement:{supplement})"),false),
                ("parent-hidden",format!("#show block:none\n#block(figure([OUTER],caption:[OUTER-CAP],supplement:{supplement}))"),false),
            ] {
                let source=format!("#show figure:it=>it\n{body}");
                let result=evaluate(&source);
                if expected_error {
                    let errors=result.expect_err(name);
                    assert!(errors.iter().any(|error|error.message.contains("INNER_PREPARATION")),"{name}: {errors:?}");
                } else { assert!(result.is_ok(),"{name}: {result:?}"); }
            }
        }
    }
}
