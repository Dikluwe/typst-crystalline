//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/reference_supplements.md
//! @prompt-hash f6e44b59
//! @layer L1
//! @updated 2026-10-19

use super::context_occurrences;
use crate::compiler::eval::{EvalContext, EvalTarget};
use crate::compiler::scopes::Scopes;
use crate::contracts::world::World;
use crate::entities::{
    args::Args,
    content::{Content, LocatedContent, RealizedContent},
    counter::Counter,
    elements::{
        context_block::ContextBlockElem,
        r#ref::{RefElem, RefForm},
    },
    engine::Engine,
    file_id::FileId,
    func::{Func, FuncRepr},
    introspector::Introspector,
    label::Label,
    selector::Selector,
    source_result::{SourceDiagnostic, SourceResult},
    span::Span,
    style::Styles,
    value::Value,
};
use std::sync::Arc;

const TOKEN: &str = "eval.reference.prepared";
const PASSTHROUGH: &str = "eval.reference.passthrough";
const ORIGIN: &str = "eval.reference.origin";

pub(in crate::compiler) fn needs_preparation(content: &Content) -> bool {
    let mut found = false;
    content.visit_content_static(&mut |node| {
        if found {
            return false;
        }
        if matches!(node, Content::Ref(reference) if reference.form == RefForm::Normal) {
            found = true;
            return false;
        }
        true
    });
    found
}

fn token(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn World,
    _file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "internal reference token is not callable",
    )])
}

fn passthrough_token(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn World,
    _file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "internal reference passthrough token is not callable",
    )])
}

fn token_matches(styles: &Styles, passthrough: bool) -> bool {
    let delta = styles.delta();
    let [(key, Value::Func(function))] = delta.custom.as_slice() else { return false };
    if key.as_str() != if passthrough { PASSTHROUGH } else { TOKEN } {
        return false;
    }
    let FuncRepr::Native(native) = function.repr() else { return false };
    let expected: fn(&mut EvalContext, &Args, &dyn World, FileId) -> SourceResult<Value> =
        if passthrough { passthrough_token } else { token };
    if native.namespace.is_some() || !std::ptr::fn_addr_eq(native.call, expected) {
        return false;
    }
    let mut other = delta.clone();
    other.custom.clear();
    other.is_empty()
}

fn guarded_reference(content: &Content, passthrough: bool) -> Option<&RefElem> {
    let Content::Styled(styled) = content else { return None };
    if !token_matches(&styled.styles, passthrough) {
        return None;
    }
    let Content::Ref(reference) = &styled.body else { return None };
    Some(reference)
}

pub(super) fn already_prepared(content: &Content) -> bool {
    guarded_reference(content, true).is_some()
        || prepared_reference_supplement(content).is_some()
}

// The synthetic With invocation replaces Args.span. Keep the lexical origin
// inside its one immutable raw Ref argument, using the existing Source carrier.
fn raw_reference(content: &Content) -> Option<(&Arc<RefElem>, Span)> {
    if let Content::Ref(reference) = content {
        return Some((reference, Span::detached()));
    }
    if let Content::Source(source) = content {
        if let Content::Ref(reference) = &source.body {
            return Some((reference, source.span));
        }
    }
    None
}

/// Recognize only this compiler's generated contextual reference producer.
#[doc(hidden)]
pub fn is_reference_supplement_producer(block: &ContextBlockElem) -> bool {
    let FuncRepr::With(bound) = block.closure.repr() else { return false };
    let FuncRepr::NativeWithEngine(native) = bound.0.repr() else { return false };
    let expected: fn(
        &mut EvalContext,
        &Args,
        &dyn World,
        FileId,
        &mut Scopes<'_>,
        &mut Engine<'_>,
    ) -> SourceResult<Value> = produce;
    native.namespace.is_none()
        && std::ptr::fn_addr_eq(native.call, expected)
        && bound.1.named.is_empty()
        && bound.1.occurrences.is_none()
        && matches!(bound.1.items.as_slice(), [Value::Content(content)] if raw_reference(content).is_some())
}

/// Borrow the single actual selected child of an exact prepared Ref transport.
#[doc(hidden)]
pub fn prepared_reference_supplement(content: &Content) -> Option<&Content> {
    guarded_reference(content, false)?.supplement.as_ref()
}

/// Map its actual child once, preserving the compiler's root-only transport.
#[doc(hidden)]
pub fn map_prepared_reference_supplement<F>(
    content: &Content,
    transform: F,
) -> SourceResult<Option<Content>>
where
    F: FnOnce(&Content) -> SourceResult<Content>,
{
    let Some(child) = prepared_reference_supplement(content) else { return Ok(None) };
    let replacement = transform(child)?;
    let Content::Styled(styled) = content else { unreachable!("classified transport") };
    let Content::Ref(reference) = &styled.body else { unreachable!("classified Ref") };
    let mut reference = (**reference).clone();
    reference.supplement = Some(replacement);
    Ok(Some(Content::styled(Content::Ref(Arc::new(reference)), styled.styles.clone())))
}

fn marked(reference: RefElem, passthrough: bool) -> Content {
    let (key, function) = if passthrough {
        (PASSTHROUGH, Func::native("reference-passthrough-token", passthrough_token))
    } else {
        (TOKEN, Func::native("reference-prepared-token", token))
    };
    Content::styled(
        Content::Ref(Arc::new(reference)),
        Styles::new().push_custom(key, Value::Func(function)),
    )
}

pub(super) fn source_styles(
    styles: &crate::entities::style_chain::StyleChain,
    span: Span,
) -> crate::entities::style_chain::StyleChain {
    let mut origin = Args::positional(Vec::new());
    origin.span = span;
    styles.push_custom(ORIGIN, Value::Args(origin))
}

pub(in crate::compiler) fn preserve_origin(
    content: &Content,
    styles: &crate::entities::style_chain::StyleChain,
) -> Content {
    if matches!(content.source_body(), Content::Ref(_)) {
        if let Some(Value::Args(origin)) = styles.custom(ORIGIN) {
            return content.clone().with_source_span(origin.span);
        }
    }
    content.clone()
}

pub(super) fn capture(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let Content::Ref(reference) = content else { return Ok(content.clone()) };
    if ctx.target == EvalTarget::Html || reference.form != RefForm::Normal {
        return Ok(content.clone());
    }
    let span = match engine.styles.custom(ORIGIN) {
        Some(Value::Args(origin)) => origin.span,
        _ => content.source_span(),
    };
    let mut args =
        Args::positional(vec![Value::Content(content.clone().with_source_span(span))]);
    args.span = span;
    let closure =
        Func::native_with_engine("reference-supplement-producer", produce).with(args);
    let block = Content::ContextBlock(Arc::new(ContextBlockElem {
        id: context_occurrences::next_context_id(ctx),
        closure,
    }));
    let located = super::locate(block, ctx);
    let Content::Located(at) = &located else { unreachable!("located producer") };
    let original = Content::Located(Arc::new(LocatedContent {
        body: content.clone(),
        location: at.location,
    }));
    Ok(Content::Realized(Arc::new(RealizedContent {
        original,
        visual: super::prepare_context_recipe(located, ctx, engine),
    })))
}

pub(in crate::compiler) fn observe_target_eligibility(
    ctx: &EvalContext,
    label: &Label,
    span: Span,
) -> SourceResult<Value> {
    let location = ctx
        .introspector
        .query_by_label(label)
        .map(Value::Location)
        .unwrap_or(Value::None);
    let key = ctx
        .introspector
        .counter_key_for_label(label)
        .cloned()
        .map(|key| Value::Counter(Counter { key }))
        .unwrap_or(Value::None);
    let bibliography =
        Value::Bool(ctx.introspector.bib_entry_for_key(&label.0).is_some());
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::ReferenceTargetEligibility {
            label: label.clone(),
        },
        span,
        Ok(Value::Array(vec![location, key, bibliography])),
    )
}

fn produce(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    file: FileId,
    _scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let invalid = || {
        vec![SourceDiagnostic::error(args.span, "invalid internal reference producer")]
    };
    let [Value::Content(captured)] = args.items.as_slice() else {
        return Err(invalid());
    };
    let Some((reference, origin)) = raw_reference(captured) else {
        return Err(invalid());
    };
    let span = if origin.is_detached() { args.span } else { origin };
    crate::entities::world_types::check_call_depth(engine.route).map_err(
        |mut errors| {
            for error in &mut errors {
                if error.span.is_detached() {
                    error.span = span;
                }
            }
            errors
        },
    )?;
    let original = Content::Ref(Arc::clone(reference));
    let passthrough = |reference: &RefElem| {
        Value::Content(Content::Realized(Arc::new(RealizedContent {
            original: original.clone(),
            visual: marked(reference.clone(), true).with_source_span(span),
        })))
    };
    if ctx.target == EvalTarget::Html || reference.form != RefForm::Normal {
        return Ok(passthrough(reference));
    }
    let label = Label(reference.name.to_string());
    observe_target_eligibility(ctx, &label, span)?;
    if ctx.introspector.bib_entry_for_key(reference.name.as_str()).is_some() {
        return Ok(passthrough(reference));
    }
    let mut query =
        Args::positional(vec![Value::Selector(Selector::Label(label.clone()))]);
    query.span = span;
    let result = crate::compiler::stdlib::native_query(ctx, &query, world, file)?;
    let Value::Array(targets) = result else { unreachable!("query returns array") };
    if targets.len() > 1 {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("label `<{}>` occurs multiple times in the document", reference.name),
        )]);
    }
    let target = targets.into_iter().next().ok_or_else(|| {
        vec![SourceDiagnostic::error(
            span,
            format!("label `<{}>` does not exist in the document", reference.name),
        )]
    })?;
    let target = match target {
        Value::LocatedContent(content, _) => content.into_content(),
        Value::Content(content) => content,
        Value::None
        | Value::Bool(..)
        | Value::Int(..)
        | Value::Float(..)
        | Value::Str(..)
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
        | Value::Symbol(..)
        | Value::Args(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..) => return Ok(passthrough(reference)),
    };
    let Content::Figure(figure) = target.source_body() else {
        return Ok(passthrough(reference));
    };
    if figure.resolved_kind.is_none() {
        return Ok(passthrough(reference));
    }
    if ctx.introspector.counter_key_for_label(&label).is_none() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "cannot reference figure without numbering",
        )]);
    }
    let selected = reference
        .supplement
        .clone()
        .or_else(|| figure.resolved_supplement.clone().flatten())
        .unwrap_or(Content::Empty);
    let selected = context_occurrences::with_template(ctx, |ctx| {
        super::realize_math_insertions(selected, ctx, engine)
    })?;
    let mut prepared = (**reference).clone();
    prepared.supplement = Some(selected);
    Ok(Value::Content(Content::Realized(Arc::new(RealizedContent {
        original,
        visual: marked(prepared, false).with_source_span(span),
    }))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{counter::CounterKey, locator::Locator};
    use std::cell::Cell;

    fn raw() -> Content {
        Content::reference("outer")
    }
    fn block(function: Func, args: Args) -> ContextBlockElem {
        ContextBlockElem { id: 1, closure: function.with(args) }
    }
    fn unrelated(
        _ctx: &mut EvalContext,
        _args: &Args,
        _world: &dyn World,
        _file: FileId,
        _scopes: &mut Scopes<'_>,
        _engine: &mut Engine<'_>,
    ) -> SourceResult<Value> {
        Ok(Value::None)
    }

    #[test]
    fn figure_reference_show_input_retains_alias_origin_without_overriding_new_ref() {
        let file = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let original = Span::from_range(file, 20..28);
        let replacement = Span::from_range(file, 5..14);
        let styles = source_styles(
            &crate::entities::style_chain::StyleChain::default_chain(),
            original,
        );
        let input = raw();
        let argument = preserve_origin(&input, &styles);
        assert_eq!(argument.source_span(), original);
        // eval_expr assigns its own identifier origin only if none was kept.
        assert_eq!(argument.with_source_span(replacement).source_span(), original);
        let created = raw().with_source_span(replacement);
        assert_eq!(preserve_origin(&created, &styles).source_span(), replacement);
        let text = Content::text("X");
        assert!(preserve_origin(&text, &styles).source_span().is_detached());
        let mut reference = RefElem {
            name: "outer".into(),
            form: RefForm::Normal,
            supplement: Some(Content::Empty),
        };
        let selected = marked(reference.clone(), false).with_source_span(original);
        assert_eq!(selected.source_span(), original);
        assert!(prepared_reference_supplement(&selected).is_none());
        assert!(matches!(
            prepared_reference_supplement(selected.source_body()),
            Some(Content::Empty)
        ));
        reference.supplement = None;
        let passthrough = marked(reference, true).with_source_span(original);
        assert_eq!(passthrough.source_span(), original);
        assert!(already_prepared(passthrough.source_body()));
    }

    #[test]
    fn figure_reference_bound_content_retains_source_through_detached_with_invocation() {
        let file = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let origin = Span::from_range(file, 6..14);
        let captured = raw().with_source_span(origin);
        let mut bound = Args::positional(vec![Value::Content(captured.clone())]);
        bound.span = origin;
        assert!(is_reference_supplement_producer(&block(
            Func::native_with_engine("alias", produce),
            bound.clone()
        )));
        // merge_with_args deliberately uses the final invocation's span.
        let invoked =
            Args::from_occurrences(Span::detached(), bound.occurrence_sequence());
        assert!(invoked.span.is_detached());
        let [Value::Content(retained)] = invoked.items.as_slice() else { unreachable!() };
        let (reference, retained_span) = raw_reference(retained).unwrap();
        assert_eq!(reference.name, "outer");
        assert_eq!(retained_span, origin);
        let detached_input = raw();
        assert!(raw_reference(&detached_input).unwrap().1.is_detached());
        let twice = Content::Source(Arc::new(crate::entities::content::SourceContent {
            body: captured.clone(),
            span: origin,
        }));
        for invalid in [
            twice,
            Content::strong(captured),
            Content::text("not Ref").with_source_span(origin),
        ] {
            assert!(raw_reference(&invalid).is_none());
            assert!(!is_reference_supplement_producer(&block(
                Func::native_with_engine("alias", produce),
                Args::positional(vec![Value::Content(invalid)])
            )));
        }
    }

    #[test]
    fn figure_reference_producer_classifier_is_typed_and_rejects_homonyms_and_extra_args()
    {
        let args = Args::positional(vec![Value::Content(raw())]);
        assert!(is_reference_supplement_producer(&block(
            Func::native_with_engine("diagnostic-alias", produce),
            args.clone()
        )));
        assert!(!is_reference_supplement_producer(&block(
            Func::native_with_engine("reference-supplement-producer", unrelated),
            args.clone()
        )));
        assert!(!is_reference_supplement_producer(&block(
            Func::native_with_engine("reference-supplement-producer", produce),
            Args::positional(vec![Value::Content(raw()), Value::None])
        )));
        assert!(!is_reference_supplement_producer(&block(
            Func::native_with_engine("reference-supplement-producer", produce),
            Args::positional(vec![Value::Content(Content::text("not Ref"))])
        )));
    }

    #[test]
    fn figure_reference_transport_maps_one_real_child_and_preserves_ref_morphology() {
        let reference = RefElem {
            name: "outer".into(),
            form: RefForm::Normal,
            supplement: Some(Content::strong(Content::text("X"))),
        };
        let content = marked(reference.clone(), false);
        let calls = Cell::new(0);
        let mapped = map_prepared_reference_supplement(&content, |child| {
            calls.set(calls.get() + 1);
            assert!(matches!(child, Content::Strong(_)));
            Ok(Content::text("changed"))
        })
        .unwrap()
        .unwrap();
        assert_eq!(calls.get(), 1);
        assert_eq!(
            prepared_reference_supplement(&mapped).unwrap().plain_text(),
            "changed"
        );
        let mapped_ref = guarded_reference(&mapped, false).unwrap();
        assert_eq!(mapped_ref.name, reference.name);
        assert_eq!(mapped_ref.form, reference.form);
        assert!(matches!(
            prepared_reference_supplement(&content),
            Some(Content::Strong(_))
        ));
        let errors = map_prepared_reference_supplement(&content, |_| {
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "original selected child error",
            )])
        })
        .unwrap_err();
        assert_eq!(errors[0].message, "original selected child error");
    }

    #[test]
    fn figure_reference_empty_child_is_canonical_but_none_passthrough_and_unmarked_are_not(
    ) {
        let reference = RefElem {
            name: "outer".into(),
            form: RefForm::Normal,
            supplement: None,
        };
        for content in
            [marked(reference.clone(), false), marked(reference.clone(), true), raw()]
        {
            assert!(prepared_reference_supplement(&content).is_none());
            assert!(map_prepared_reference_supplement(&content, |_| panic!(
                "no actual selected child"
            ))
            .unwrap()
            .is_none());
        }
        let mut empty = reference;
        empty.supplement = Some(Content::Empty);
        let content = marked(empty, false);
        assert!(matches!(prepared_reference_supplement(&content), Some(Content::Empty)));
        assert!(map_prepared_reference_supplement(&content, |_| Ok(Content::Empty))
            .unwrap()
            .is_some());
    }

    #[test]
    fn figure_reference_eligibility_distinguishes_numbering_and_first_target_with_equal_numeric_state(
    ) {
        let label = Label("outer".into());
        let mut locator = Locator::new();
        let a = locator.next();
        let b = locator.next();
        let mut ctx = EvalContext::new();
        ctx.introspector.labels.add(label.clone(), a);
        let absent = observe_target_eligibility(&ctx, &label, Span::detached()).unwrap();
        ctx.introspector
            .label_to_counter_key
            .insert(label.clone(), CounterKey::Str("same-values".into()));
        let numbered =
            observe_target_eligibility(&ctx, &label, Span::detached()).unwrap();
        assert_ne!(
            absent, numbered,
            "eligibility changes even without a numeric-state change"
        );
        ctx.introspector.label_to_counter_key.remove(&label);
        assert_eq!(
            absent,
            observe_target_eligibility(&ctx, &label, Span::detached()).unwrap()
        );
        ctx.introspector.labels = crate::entities::label_registry::LabelRegistry::empty();
        ctx.introspector.labels.add(label.clone(), b);
        assert_ne!(
            absent,
            observe_target_eligibility(&ctx, &label, Span::detached()).unwrap()
        );
        ctx.introspector.bib_store.add_bibliography(vec![
            crate::entities::bib_entry::BibEntry::new("outer", "Author", "Title", 2026),
        ]);
        let bib = observe_target_eligibility(&ctx, &label, Span::detached()).unwrap();
        let Value::Array(fields) = bib else { unreachable!() };
        assert_eq!(fields[2], Value::Bool(true));
    }
}
