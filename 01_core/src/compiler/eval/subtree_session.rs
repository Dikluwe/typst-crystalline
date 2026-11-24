//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/subtree_session.md
//! @prompt-hash caf8da7d
//! @layer L1
//! @updated 2026-11-12

use super::*;
use crate::entities::{
    ast::{
        code::ForLoop,
        expr::{Arg, FuncCall, Pattern},
    },
    compiler_features::Features,
    style_chain::StyleDelta,
    syntax_node::LinkedNode,
};
use std::rc::Rc;

const MAX_ENTRIES: usize = 16;
const MAX_TOTAL: usize = 8 * 1024 * 1024;
const MAX_ENTRY: usize = 1024 * 1024;
const MAX_BODY: usize = 8 * 1024;
const MAX_ITEMS: usize = 4096;

/// Experimental watcher-owned retention of certified evaluation products.
#[derive(Default)]
pub struct SubtreeSession {
    stdlib: RefCell<Option<Arc<super::stdlib_snapshot::StdlibSnapshot>>>,
    memo: Rc<RefCell<Memo>>,
}
impl SubtreeSession {
    pub(super) fn stdlib(
        &self,
        inputs: &SysInputs,
        features: Features,
    ) -> Arc<super::stdlib_snapshot::StdlibSnapshot> {
        super::stdlib_snapshot::root_resolve(&self.stdlib, inputs, features)
    }
    pub fn new() -> Self {
        Self::default()
    }
    /// Cumulative hits, admitted misses, retained products.
    pub fn stats(&self) -> (usize, usize, usize) {
        let memo = self.memo.borrow();
        (memo.hits, memo.misses, memo.entries.len())
    }
    pub(super) fn bind(&self, ctx: &mut EvalContext) {
        ctx.subtree_memo = Some(self.memo.clone());
    }
    pub fn evaluate(
        &self,
        world: &dyn World,
        source: &Source,
        full_error: bool,
        target: EvalTarget,
        features: Features,
    ) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
        let mut sink = Sink::new();
        let result = super::eval_with_subtree_session(
            world,
            sink.track_mut(),
            source,
            &crate::entities::element_registry::ElementRegistry::new(),
            (full_error, target, features),
            Some(self),
        );
        (result, sink.into_diagnostics())
    }
}

#[derive(Clone, PartialEq)]
enum Scalar {
    None,
    Auto,
    Bool(bool),
    Int(i64),
    Float(u64),
    Str(EcoString),
    Length(u64, u64),
    Ratio(u64),
}
impl Scalar {
    fn payload(&self) -> usize {
        if let Self::Str(text) = self {
            text.len()
        } else {
            32
        }
    }
}
fn scalar(value: &Value) -> Option<Scalar> {
    match value {
        Value::None => Some(Scalar::None),
        Value::Auto => Some(Scalar::Auto),
        Value::Bool(v) => Some(Scalar::Bool(*v)),
        Value::Int(v) => Some(Scalar::Int(*v)),
        Value::Float(v) => Some(Scalar::Float(v.to_bits())),
        Value::Str(v) => Some(Scalar::Str(v.clone())),
        Value::Length(v) => Some(Scalar::Length(v.abs.to_pt().to_bits(), v.em.to_bits())),
        Value::Ratio(v) => Some(Scalar::Ratio(v.0.to_bits())),
        Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(..)
        | Value::Relative(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => None,
    }
}
fn primitive(value: &Value) -> Option<Scalar> {
    let value = scalar(value)?;
    match value {
        Scalar::Length(..) | Scalar::Ratio(_) => None,
        Scalar::None
        | Scalar::Auto
        | Scalar::Bool(_)
        | Scalar::Int(_)
        | Scalar::Float(_)
        | Scalar::Str(_) => Some(value),
    }
}

#[derive(PartialEq)]
struct LexicalStyle {
    typed: StyleDelta,
    custom: Vec<(EcoString, Scalar)>,
    numeric_bits: Vec<u64>,
}

pub(super) struct Key {
    body: SyntaxNode,
    spans: Vec<Span>,
    loop_span: Span,
    file: FileId,
    binding: EcoString,
    items: Vec<Scalar>,
    reads: Vec<(EcoString, Scalar)>,
    calls: Vec<(EcoString, PureBuiltin)>,
    styles: Vec<LexicalStyle>,
    options: (EvalTarget, Features, bool, bool, bool),
    charge: usize,
}
impl Key {
    fn matches(&self, other: &Self) -> bool {
        self.file == other.file
            && self.loop_span == other.loop_span
            && self.binding == other.binding
            && self.items == other.items
            && self.reads == other.reads
            && self.calls == other.calls
            && self.options == other.options
            && self.styles == other.styles
            && self.spans == other.spans
            && self.body.spanless_eq(&other.body)
    }
}
struct Entry {
    key: Key,
    value: Value,
}
#[derive(Default)]
pub(super) struct Memo {
    entries: Vec<Entry>,
    charged: usize,
    hits: usize,
    misses: usize,
}
impl Memo {
    pub(super) fn lookup(&mut self, key: &Key) -> Option<Value> {
        if let Some(index) = self.entries.iter().position(|entry| entry.key.matches(key))
        {
            let entry = self.entries.remove(index);
            let value = entry.value.clone();
            self.entries.push(entry);
            self.hits += 1;
            Some(value)
        } else {
            self.misses += 1;
            None
        }
    }
    pub(super) fn store(&mut self, key: Key, value: Value) {
        while self.entries.len() >= MAX_ENTRIES || self.charged + key.charge > MAX_TOTAL {
            if self.entries.is_empty() {
                return;
            }
            self.charged -= self.entries.remove(0).key.charge;
        }
        self.charged += key.charge;
        self.entries.push(Entry { key, value });
    }
}

fn spans(node: &SyntaxNode, out: &mut Vec<Span>) {
    out.push(node.span());
    for child in node.children() {
        spans(child, out);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PureBuiltin {
    Lorem,
    Pagebreak,
}

fn pure_builtin(value: &Value) -> Option<PureBuiltin> {
    use crate::entities::func::FuncRepr;
    let Value::Func(func) = value else { return None };
    let native = match func.repr() {
        FuncRepr::Native(native) => native,
        FuncRepr::Closure(_)
        | FuncRepr::Format(_, _)
        | FuncRepr::NativeWithEngine(_)
        | FuncRepr::Element(_)
        | FuncRepr::Plugin(_)
        | FuncRepr::With(_) => return None,
    };
    if native.namespace.is_some() {
        return None;
    }
    if std::ptr::fn_addr_eq(
        native.call,
        crate::compiler::stdlib::native_lorem as fn(_, _, _, _) -> _,
    ) {
        Some(PureBuiltin::Lorem)
    } else if std::ptr::fn_addr_eq(
        native.call,
        crate::compiler::stdlib::native_pagebreak as fn(_, _, _, _) -> _,
    ) {
        Some(PureBuiltin::Pagebreak)
    } else {
        None
    }
}

#[derive(Default)]
struct BodyProof {
    reads: Vec<(EcoString, Scalar)>,
    calls: Vec<(EcoString, PureBuiltin)>,
    output_bytes: usize,
}

fn lorem_words(
    expr: Expr<'_>,
    binding: &str,
    scopes: &Scopes<'_>,
    items: &[Scalar],
) -> Option<usize> {
    fn bounded(value: i64) -> Option<usize> {
        (0..=4096).contains(&value).then_some(value as usize)
    }
    if let Expr::Int(value) = expr {
        return bounded(value.get());
    }
    let Expr::Ident(name) = expr else { return None };
    if name.as_str() == binding {
        items.iter().try_fold(0, |largest, value| {
            let Scalar::Int(n) = value else { return None };
            Some(largest.max(bounded(*n)?))
        })
    } else {
        let Value::Int(n) = scopes.get(name.as_str())? else { return None };
        bounded(*n)
    }
}

fn call_reads(
    call: FuncCall<'_>,
    binding: &str,
    scopes: &Scopes<'_>,
    items: &[Scalar],
    proof: &mut BodyProof,
) -> Option<()> {
    let Expr::Ident(callee) = call.callee() else { return None };
    let name = callee.as_str();
    if name == binding {
        return None;
    }
    let kind = pure_builtin(scopes.get(name)?)?;
    let args: Vec<_> = call.args().items().collect();
    let extra = match kind {
        PureBuiltin::Lorem => {
            let [Arg::Pos(expr)] = args.as_slice() else { return None };
            lorem_words(*expr, binding, scopes, items)?
                .checked_mul(64)?
                .checked_add(1)?
        }
        PureBuiltin::Pagebreak => {
            if args.iter().any(|arg| !matches!(arg, Arg::Named(_))) {
                return None;
            }
            0
        }
    };
    proof.output_bytes = proof.output_bytes.checked_add(extra)?;
    if !proof.calls.iter().any(|(old, _)| old == name) {
        proof.calls.push((name.into(), kind));
    }
    for arg in args {
        let expr = match arg {
            Arg::Pos(expr) => expr,
            Arg::Named(named) => named.expr(),
            Arg::Spread(_) => return None,
        };
        body_reads(expr.to_untyped(), binding, scopes, items, proof)?;
    }
    Some(())
}

fn body_reads(
    node: &SyntaxNode,
    binding: &str,
    scopes: &Scopes<'_>,
    items: &[Scalar],
    proof: &mut BodyProof,
) -> Option<()> {
    if let Some(call) = node.cast::<FuncCall<'_>>() {
        return call_reads(call, binding, scopes, items, proof);
    }
    if node.kind() == SyntaxKind::Ident {
        let name = node.text_str();
        if name != binding && !proof.reads.iter().any(|(old, _)| old == name) {
            proof.reads.push((name.into(), primitive(scopes.get(name)?)?));
        }
    } else if !matches!(
        node.kind(),
        SyntaxKind::ContentBlock
            | SyntaxKind::Markup
            | SyntaxKind::Text
            | SyntaxKind::Space
            | SyntaxKind::Parbreak
            | SyntaxKind::Hash
            | SyntaxKind::LeftBracket
            | SyntaxKind::RightBracket
            | SyntaxKind::Int
            | SyntaxKind::Float
            | SyntaxKind::Str
            | SyntaxKind::Bool
            | SyntaxKind::None
            | SyntaxKind::Auto
            | SyntaxKind::LineComment
            | SyntaxKind::BlockComment
            | SyntaxKind::Heading
            | SyntaxKind::HeadingMarker
            | SyntaxKind::Strong
            | SyntaxKind::Star
            | SyntaxKind::Emph
            | SyntaxKind::Underscore
    ) {
        return None;
    }
    for child in node.children() {
        body_reads(child, binding, scopes, items, proof)?;
    }
    Some(())
}

fn color_payload(
    color: &crate::entities::color::Color,
    bits: &mut Vec<u64>,
) -> Option<usize> {
    if let crate::entities::color::Color::Spot(spot) = color {
        bits.push(spot.tint.0.to_bits());
        let name = spot.colorant.name.as_ref().map_or(0, |name| name.len());
        name.checked_add(color_payload(&spot.colorant.fallback, bits)?)
    } else {
        Some(0)
    }
}

fn style_payload(delta: &StyleDelta, bits: &mut Vec<u64>) -> Option<usize> {
    use crate::entities::{font_list::FontNamePattern, layout_types::TextEdge};
    let mut charge = 0usize;
    for length in [
        delta.size,
        delta.tracking,
        delta.leading,
        delta.highlight_radius,
        delta.highlight_extent,
        delta.subscript_size,
        delta.superscript_size,
    ]
    .into_iter()
    .flatten()
    {
        bits.extend([length.abs.to_pt().to_bits(), length.em.to_bits()]);
    }
    for edge in [delta.top_edge.as_ref(), delta.bottom_edge.as_ref()]
        .into_iter()
        .flatten()
    {
        if let TextEdge::Length(length) = edge {
            bits.extend([length.abs.to_pt().to_bits(), length.em.to_bits()]);
        }
    }
    let fill = match &delta.fill {
        Some(crate::entities::paint::Paint::Solid(color)) => Some(color),
        Some(_) => return None,
        None => None,
    };
    for color in fill.into_iter().chain(delta.highlight.iter().flatten()) {
        charge = charge.checked_add(color_payload(color, bits)?)?;
    }
    if let Some(fonts) = &delta.font {
        for family in fonts.as_slice() {
            let name = match &family.name {
                FontNamePattern::Literal(name) => name.as_str(),
                FontNamePattern::Regex(regex) => regex.pattern(),
            };
            charge = charge.checked_add(name.len())?;
            for value in family
                .variants
                .iter()
                .chain(family.variant.iter())
                .chain(family.weight.iter())
                .chain(family.style.iter())
            {
                charge = charge.checked_add(value.len())?;
            }
        }
    }
    Some(charge)
}

pub(super) fn key(
    loop_expr: ForLoop<'_>,
    items: &[Value],
    scopes: &Scopes<'_>,
    ctx: &EvalContext,
    engine: &Engine<'_>,
) -> Option<Key> {
    if items.is_empty()
        || items.len() > MAX_ITEMS
        || ctx.flow.is_some()
        || ctx.in_context
        || ctx.current_location.is_some()
        || ctx.realizing_show_rules
        || ctx.recipe_math_depth != 0
        || ctx.occurrence_show_depth != 0
        || !engine.show_rules.is_empty()
        || !engine.active_guards.is_empty()
        || ctx.loop_iterations.checked_add(items.len())? > ctx.max_loop_iterations
    {
        return None;
    }
    let Pattern::Normal(Expr::Ident(binding)) = loop_expr.pattern() else { return None };
    let Expr::ContentBlock(body) = loop_expr.body() else { return None };
    let body = body.to_untyped();
    if body.len() > MAX_BODY || loop_expr.span().id() != Some(engine.current_file) {
        return None;
    }
    let source = engine.world.source(engine.current_file).ok()?;
    let current = LinkedNode::new(source.root()).find(body.span())?;
    if current.parent_kind() != Some(SyntaxKind::ForLoop)
        || !current.get().spanless_eq(body)
    {
        return None;
    }
    let mut source_spans = Vec::new();
    spans(current.get(), &mut source_spans);
    let mut body_spans = Vec::new();
    spans(body, &mut body_spans);
    if source_spans != body_spans {
        return None;
    }
    let items: Vec<Scalar> = items.iter().map(primitive).collect::<Option<_>>()?;
    let mut proof = BodyProof::default();
    body_reads(body, binding.as_str(), scopes, &items, &mut proof)?;
    let calls_charge = proof.calls.iter().try_fold(0usize, |charge, (name, _)| {
        charge
            .checked_add(name.len())?
            .checked_add(std::mem::size_of::<PureBuiltin>())
    })?;
    let reads = proof.reads;
    let mut styles = Vec::new();
    let mut style_charge = 0usize;
    for delta in engine.styles.raw_deltas() {
        let mut numeric_bits = Vec::new();
        style_charge =
            style_charge.checked_add(style_payload(delta, &mut numeric_bits)?)?;
        let mut typed = delta.clone();
        let mut custom = Vec::new();
        for (name, value) in &typed.custom {
            let value = scalar(value)?;
            style_charge = style_charge.checked_add(name.len() + value.payload())?;
            custom.push((name.clone(), value));
        }
        typed.custom.clear();
        styles.push(LexicalStyle { typed, custom, numeric_bits });
    }
    let largest = items
        .iter()
        .chain(reads.iter().map(|(_, v)| v))
        .map(Scalar::payload)
        .max()?;
    let nodes = body_spans.len();
    let charge = body
        .len()
        .checked_add(proof.output_bytes)?
        .checked_add(
            nodes
                .checked_mul(largest.checked_add(4 * std::mem::size_of::<Content>())?)?,
        )?
        .checked_mul(items.len())?
        .checked_add(style_charge)?
        .checked_add(calls_charge)?
        .checked_add(binding.as_str().len())?
        .checked_add(styles.len().checked_mul(std::mem::size_of::<StyleDelta>())?)?;
    if charge > MAX_ENTRY {
        return None;
    }
    Some(Key {
        body: body.clone(),
        spans: body_spans,
        loop_span: loop_expr.span(),
        file: engine.current_file,
        binding: binding.as_str().into(),
        items,
        reads,
        calls: proof.calls,
        styles,
        options: (
            ctx.target,
            ctx.features,
            ctx.full_error,
            ctx.apply_show_rules,
            ctx.defer_show_rules,
        ),
        charge,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::world_types::FileError;
    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
        inputs: SysInputs,
        available: bool,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text).unwrap(),
                library: Library::default(),
                book: FontBook::new(),
                inputs: SysInputs::default(),
                available: true,
            }
        }
        fn revise(&mut self, text: &str) {
            self.source = self.source.reparse(text.into()).unwrap();
        }
        fn text(&self, session: &SubtreeSession) -> String {
            let (result, warnings) = session.evaluate(
                self,
                &self.source,
                false,
                EvalTarget::Paged,
                Features::default(),
            );
            assert!(warnings.is_empty(), "{warnings:?}");
            result.unwrap().content().unwrap().plain_text()
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
        fn source(&self, id: FileId) -> FileResult<Source> {
            if self.available && id == self.source.id() {
                Ok(self.source.clone())
            } else {
                Err(FileError::NotFound)
            }
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
        fn inputs(&self) -> SysInputs {
            self.inputs.clone()
        }
    }

    fn pure_loop_check(world: &TestWorld, session: &SubtreeSession) {
        let (warm, warm_warnings) = session.evaluate(
            world,
            &world.source,
            false,
            EvalTarget::Paged,
            Features::default(),
        );
        let (cold, cold_warnings) = SubtreeSession::new().evaluate(
            world,
            &world.source,
            false,
            EvalTarget::Paged,
            Features::default(),
        );
        assert_eq!(warm_warnings, cold_warnings);
        let warm = warm.unwrap();
        let cold = cold.unwrap();
        assert_eq!(
            operators::eval_binary_op(
                crate::entities::ast::expr::BinOp::Eq,
                Value::Content(warm.content().unwrap().clone()),
                Value::Content(cold.content().unwrap().clone())
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            warm.content(),
            cold.content(),
            "warm morphology differs from fresh evaluation"
        );
    }
    #[test]
    fn pure_loop_reuses_heading_lorem_pagebreak_after_outside_edit() {
        let body = "#for i in range(12) [= Heading #i\n#lorem(30)\n#pagebreak()]";
        let mut world = TestWorld::new(&format!("Alpha\n{body}"));
        let session = SubtreeSession::new();
        pure_loop_check(&world, &session);
        world.revise(&format!("Bravo\n{body}\nSuffix"));
        pure_loop_check(&world, &session);
        assert!(session.stats().0 > 0, "intact structured loop was evaluated again");
    }
    #[test]
    fn pure_loop_reuses_aliases_variable_arguments_and_markup() {
        let world=TestWorld::new("#let make=lorem; #let next=pagebreak; #let n=10; #for i in (1,2,3) [= *Heading* #i _Italic_\n#make(n) #make(i) #next(weak:true)]");
        let session = SubtreeSession::new();
        pure_loop_check(&world, &session);
        pure_loop_check(&world, &session);
        assert!(session.stats().0 > 0, "certified aliases/markup were not reused");
    }
    #[test]
    fn pure_loop_shadowed_user_function_stays_on_normal_path() {
        let world =
            TestWorld::new("#let lorem(n)=[USER #n]; #for i in (1,2) [#lorem(i)]");
        let session = SubtreeSession::new();
        pure_loop_check(&world, &session);
        pure_loop_check(&world, &session);
        assert_eq!(session.stats().0, 0);
        assert!(world.text(&session).contains("USER"));
    }

    #[test]
    fn pure_loop_invalidates_current_inputs_arguments_and_body() {
        let mut world=TestWorld::new("#let n=int(sys.inputs.n); #for i in (1,2) [= Title #i\n#lorem(n) #pagebreak()]");
        let session = SubtreeSession::new();
        for n in ["3", "7", "2"] {
            world.inputs.insert("n".into(), n.into());
            let before = session.stats().0;
            pure_loop_check(&world, &session);
            assert_eq!(session.stats().0, before);
        }
        world.revise("#let n=int(sys.inputs.n); #for i in (1,2,3) [= Changed #i\n#lorem(n) #pagebreak()]");
        let before = session.stats().0;
        pure_loop_check(&world, &session);
        assert_eq!(session.stats().0, before);
    }
    #[test]
    fn pure_loop_alias_shadowing_cannot_replay_old_builtin_output() {
        let mut world = TestWorld::new("#let make=lorem; #for i in (1,2) [#make(i)]");
        let session = SubtreeSession::new();
        pure_loop_check(&world, &session);
        pure_loop_check(&world, &session);
        world.revise("#let make(n)=[USER #n]; #for i in (1,2) [#make(i)]");
        let before = session.stats().0;
        pure_loop_check(&world, &session);
        assert_eq!(session.stats().0, before);
        assert!(world.text(&session).contains("USER"));
    }
    #[test]
    fn pure_loop_invalid_arguments_keep_current_diagnostics() {
        let mut world = TestWorld::new("#let make=lorem; #for i in (1,2) [#make(i)]");
        let session = SubtreeSession::new();
        pure_loop_check(&world, &session);
        pure_loop_check(&world, &session);
        for text in [
            "#let make=pagebreak; #for i in (1,2) [#make(i)]",
            "#for i in (1,2) [#lorem(\"wrong\")]",
            "#for i in (1,2) [#pagebreak(weak:1)]",
            "#for i in (1,2) [#pagebreak(to:\"wrong\")]",
            "#for i in (1,2) [#pagebreak(unknown:1)]",
        ] {
            world.revise(text);
            let before = session.stats().0;
            let (warm, ww) = session.evaluate(
                &world,
                &world.source,
                false,
                EvalTarget::Paged,
                Features::default(),
            );
            let (cold, cw) = SubtreeSession::new().evaluate(
                &world,
                &world.source,
                false,
                EvalTarget::Paged,
                Features::default(),
            );
            assert_eq!(ww, cw);
            assert_eq!(warm.unwrap_err(), cold.unwrap_err());
            assert_eq!(session.stats().0, before);
        }
    }
    #[test]
    fn pure_loop_context_show_and_state_remain_uncached() {
        for text in [
            "#show heading: it => [CHANGED #it.body]\n#for i in (1,2) [= Title #i\n#lorem(3)]",
            "#for i in (1,2) [= Title #i\n#context [#i] #lorem(3)]",
            "#let s=state(\"probe\",0); #for i in (1,2) [#s.update(x => x + 1) #lorem(3)]",
            "#let partial=lorem.with(); #for i in (1,2) [#partial(i)]",
        ] {
            let world=TestWorld::new(text);let session=SubtreeSession::new();
            let first=world.text(&session);
            let second=world.text(&session);
            assert_eq!(first,second);
            assert_eq!(second,world.text(&SubtreeSession::new()));
            assert_eq!(session.stats().0,0);
        }
    }
    #[test]
    fn pure_loop_generated_payload_refuses_oversized_entry_without_truncation() {
        let world = TestWorld::new("#for i in range(8) [#lorem(4096)]");
        let session = SubtreeSession::new();
        pure_loop_check(&world, &session);
        pure_loop_check(&world, &session);
        assert_eq!(session.stats(), (0, 0, 0));
        assert!(world.text(&session).len() > 4096);
    }
    #[test]
    fn pure_loop_generated_content_keeps_loop_budget_failure() {
        let world =
            TestWorld::new("#for i in (1,2,3) [= Title #i\n#lorem(i) #pagebreak()]");
        let session = SubtreeSession::new();
        let (first, _) = budget_run(&world, Some(&session), 0, 10);
        first.unwrap();
        let (warm, used) = budget_run(&world, Some(&session), 0, 10);
        warm.unwrap();
        assert_eq!(used, 3);
        let (warm, wu) = budget_run(&world, Some(&session), 1, 3);
        let (cold, cu) = budget_run(&world, None, 1, 3);
        assert_eq!(warm.unwrap_err(), cold.unwrap_err());
        assert_eq!(wu, cu);
    }
    #[test]
    fn subtree_reuses_intact_body_inside_changed_root() {
        let base = "#for i in range(8) [Item #i\n\n]\nTail".replace("\\n", "\n");
        let mut world = TestWorld::new(&base);
        let session = SubtreeSession::new();
        assert_eq!(world.text(&session), world.text(&SubtreeSession::new()));
        world.revise(&format!("= Prefix\n{base}\n= Suffix"));
        assert_eq!(world.text(&session), world.text(&SubtreeSession::new()));
        assert!(session.stats().0 > 0, "intact body was executed again");
        assert!(session.stats().2 > 0);
    }
    #[test]
    fn subtree_invalidates_bindings_iterable_and_body() {
        let mut world = TestWorld::new("");
        let session = SubtreeSession::new();
        for text in [
            "#let s=\"old\"; #let n=3; #for i in range(n) [#s #i ]",
            "#let s=\"new\"; #let n=3; #for i in range(n) [#s #i ]",
            "#let s=\"new\"; #let n=5; #for i in range(n) [#s #i ]",
            "#let s=\"new\"; #let n=5; #for i in range(n) [changed #s #i ]",
        ] {
            world.revise(text);
            let before = session.stats().0;
            assert_eq!(world.text(&session), world.text(&SubtreeSession::new()));
            assert_eq!(session.stats().0, before);
        }
    }
    #[test]
    fn subtree_validates_current_inputs() {
        let mut world =
            TestWorld::new("#let s=sys.inputs.label; #for i in range(3) [#s #i ]");
        let session = SubtreeSession::new();
        for label in ["alpha", "bravo"] {
            world.inputs.insert("label".into(), label.into());
            let before = session.stats().0;
            let output = world.text(&session);
            assert!(output.contains(label));
            assert_eq!(output, world.text(&SubtreeSession::new()));
            assert_eq!(session.stats().0, before);
        }
    }
    #[test]
    fn subtree_refuses_effects_and_show_rules() {
        for text in [
            "#let n=0; #for i in range(3) { n += 1 }; #n",
            "#show \"Item\": \"Changed\"\n#for i in range(3) [Item #i ]",
            "#for i in range(3) [#context [#i]]",
        ] {
            let world = TestWorld::new(text);
            let session = SubtreeSession::new();
            let _ = world.text(&session);
            assert_eq!(world.text(&session), world.text(&SubtreeSession::new()));
            assert_eq!(session.stats().0, 0);
        }
    }
    #[test]
    fn subtree_invalidates_style_and_missing_origin() {
        let mut world =
            TestWorld::new("#set text(size:10pt)\n#for i in range(3) [Item #i ]");
        let session = SubtreeSession::new();
        let _ = world.text(&session);
        world.revise("#set text(size:20pt)\n#for i in range(3) [Item #i ]");
        let before = session.stats().0;
        assert_eq!(world.text(&session), world.text(&SubtreeSession::new()));
        assert_eq!(session.stats().0, before);
        world.available = false;
        let before = session.stats().0;
        assert_eq!(world.text(&session), world.text(&SubtreeSession::new()));
        assert_eq!(session.stats().0, before);
    }
    #[test]
    fn subtree_sessions_are_independent() {
        let world = TestWorld::new("#for i in range(3) [Item #i ]");
        let first = SubtreeSession::new();
        let _ = world.text(&first);
        let _ = world.text(&first);
        assert!(first.stats().0 > 0);
        let second = SubtreeSession::new();
        assert_eq!(second.stats(), (0, 0, 0));
        let _ = world.text(&second);
        assert_eq!(second.stats().0, 0);
    }

    #[test]
    fn subtree_bounds_retention_and_declines_oversized_bodies() {
        let mut world = TestWorld::new("");
        let session = SubtreeSession::new();
        for n in 0..40 {
            world.revise(&format!("#for i in range(2) [Different{n} #i ]"));
            let _ = world.text(&session);
        }
        assert_eq!(session.stats().2, MAX_ENTRIES);
        assert!(session.memo.borrow().charged <= MAX_TOTAL);
        let big = format!("#for i in range(1) [{}]", "x".repeat(MAX_BODY + 1));
        world.revise(&big);
        let before = session.stats();
        let _ = world.text(&session);
        assert_eq!(session.stats(), before);
        world.revise("#for i in range(4097) []");
        let _ = world.text(&session);
        assert_eq!(session.stats(), before);
    }

    fn budget_run(
        world: &TestWorld,
        session: Option<&SubtreeSession>,
        used: usize,
        limit: usize,
    ) -> (SourceResult<Value>, usize) {
        fn find(node: &SyntaxNode) -> Option<ForLoop<'_>> {
            if let Some(loop_expr) = node.cast::<ForLoop<'_>>() {
                return Some(loop_expr);
            }
            node.children().find_map(find)
        }
        let mut ctx = EvalContext::new();
        ctx.loop_iterations = used;
        ctx.max_loop_iterations = limit;
        if let Some(session) = session {
            session.bind(&mut ctx);
        }
        let library = Library::with_global(super::super::make_stdlib_with_features(
            &world.inputs(),
            Features::default(),
        ));
        let mut scopes = Scopes::new(Some(&library));
        let mut styles = StyleChain::default_chain();
        let mut rules = Arc::from([]);
        let mut guards = Vec::new();
        let route = Route::root().with_id(world.main());
        let mut sink = Sink::new();
        let mut tracked = sink.track_mut();
        let mut engine = Engine {
            world,
            font_metrics: &FixedMetrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked,
        };
        let result = super::super::control_flow::eval_for(
            find(world.source.root()).unwrap(),
            &mut scopes,
            &mut ctx,
            &mut engine,
        );
        (result, ctx.loop_iterations)
    }

    #[test]
    fn subtree_hit_replays_budget_and_failure_uses_cold_span() {
        let world = TestWorld::new("#for i in range(3) [Item #i ]");
        let session = SubtreeSession::new();
        let first = budget_run(&world, Some(&session), 0, 5);
        assert!(first.0.is_ok());
        assert_eq!(first.1, 3);
        let next = budget_run(&world, Some(&session), 1, 5);
        assert!(next.0.is_ok());
        assert_eq!(next.1, 4);
        assert_eq!(session.stats().0, 1);
        let warm = budget_run(&world, Some(&session), 3, 5);
        let cold = budget_run(&world, None, 3, 5);
        assert_eq!(warm.1, 6);
        assert_eq!(warm.1, cold.1);
        let a = warm.0.unwrap_err();
        let b = cold.0.unwrap_err();
        assert_eq!(a[0].span, b[0].span);
        assert_eq!(a[0].message, b[0].message);
        assert_eq!(session.stats().0, 1);
    }

    #[test]
    fn subtree_scalar_and_style_keys_keep_numeric_bits() {
        assert!(primitive(&Value::Int(1)) != primitive(&Value::Float(1.0)));
        assert!(primitive(&Value::Float(0.0)) != primitive(&Value::Float(-0.0)));
        let a = StyleDelta {
            size: Some(crate::entities::layout_types::Length::pt(0.0)),
            ..StyleDelta::empty()
        };
        let b = StyleDelta {
            size: Some(crate::entities::layout_types::Length::pt(-0.0)),
            ..StyleDelta::empty()
        };
        let (mut x, mut y) = (Vec::new(), Vec::new());
        style_payload(&a, &mut x).unwrap();
        style_payload(&b, &mut y).unwrap();
        assert_ne!(x, y);
    }
    #[test]
    fn builtin_root_pipeline_reuses_seed_with_current_inputs_and_identities() {
        let mut world = TestWorld::new("#let exposed = sym; #let element = raw.line; #let bound = raw.line.with(); #let alias = bound; #sys.inputs.label");
        world.inputs.insert("label".into(), "one".into());
        let session = SubtreeSession::new();
        let evaluate = |world: &TestWorld| {
            let (result, warnings) = session.evaluate(
                world,
                &world.source,
                false,
                EvalTarget::Paged,
                Features::default(),
            );
            assert!(warnings.is_empty());
            result.unwrap()
        };
        let first = evaluate(&world);
        assert!(
            session.stdlib.borrow().is_some(),
            "root path bypassed retained builtin seed"
        );
        let seed = session.stdlib.borrow().as_ref().unwrap().clone();
        let second = evaluate(&world);
        assert!(Arc::ptr_eq(&seed, session.stdlib.borrow().as_ref().unwrap()));
        assert_eq!(
            first.content().unwrap().plain_text(),
            second.content().unwrap().plain_text()
        );
        for name in ["exposed", "element"] {
            assert_eq!(first.scope().get(name), second.scope().get(name));
        }
        assert_ne!(first.scope().get("bound"), second.scope().get("bound"));
        assert_eq!(second.scope().get("bound"), second.scope().get("alias"));
        world.inputs.insert("label".into(), "two".into());
        let third = evaluate(&world);
        assert!(third.content().unwrap().plain_text().contains("two"));
        assert!(!Arc::ptr_eq(&seed, session.stdlib.borrow().as_ref().unwrap()));
        assert!(first.content().unwrap().plain_text().contains("one"));
    }
    #[test]
    fn builtin_root_pipeline_features_are_current_across_revisions() {
        let world = TestWorld::new("#let builtins = std;");
        let session = SubtreeSession::new();
        for enabled in [false, true, true, false] {
            let mut features = Features::empty();
            if enabled {
                features.enable(crate::entities::compiler_features::Feature::Bundle);
            }
            let (result, warnings) = session.evaluate(
                &world,
                &world.source,
                false,
                EvalTarget::Paged,
                features,
            );
            assert!(warnings.is_empty());
            let result = result.unwrap();
            let Value::Module(builtins) = result.scope().get("builtins").unwrap() else {
                panic!()
            };
            assert_eq!(builtins.scope().get("asset").is_some(), enabled);
            assert!(
                session.stdlib.borrow().is_some(),
                "pipeline did not install root template"
            );
        }
    }
    #[test]
    fn pure_loop_classifies_real_calls_not_names_or_wrappers() {
        use crate::entities::args::Args;
        fn unknown(
            _: &mut EvalContext,
            _: &Args,
            _: &dyn World,
            _: FileId,
        ) -> SourceResult<Value> {
            Ok(Value::Int(91))
        }
        fn with_engine(
            _: &mut EvalContext,
            _: &Args,
            _: &dyn World,
            _: FileId,
            _: &mut Scopes<'_>,
            _: &mut Engine<'_>,
        ) -> SourceResult<Value> {
            Ok(Value::Int(92))
        }
        let actual = Func::native("alias", crate::compiler::stdlib::native_lorem);
        assert_eq!(pure_builtin(&Value::Func(actual.clone())), Some(PureBuiltin::Lorem));
        assert_eq!(
            pure_builtin(&Value::Func(Func::native(
                "alias",
                crate::compiler::stdlib::native_pagebreak
            ))),
            Some(PureBuiltin::Pagebreak)
        );
        for func in [
            Func::native("lorem", unknown),
            Func::native_with_engine("lorem", with_engine),
            actual.with(Args::positional(vec![])),
            Func::native_with_namespace(
                "lorem",
                crate::compiler::stdlib::native_lorem,
                Arc::new(Scope::new()),
            ),
            Func::element("lorem", Arc::new(|_| Ok(Content::text("user")))),
        ] {
            assert_eq!(pure_builtin(&Value::Func(func)), None);
        }
    }
    #[test]
    fn pure_loop_lorem_corpus_and_generated_output_fit_charge_bound() {
        for corpus in [lipsum::LOREM_IPSUM, lipsum::LIBER_PRIMUS] {
            assert!(corpus.is_ascii());
            let words: Vec<_> = corpus.split_whitespace().collect();
            assert!(words.iter().all(|word| word.len() + 5 <= 64));
            assert!(words.windows(2).all(|pair| pair != ["--", "--"]));
        }
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        for n in [0, 1, 2, 30, 256, 4096] {
            let args = crate::entities::args::Args::positional(vec![Value::Int(n)]);
            let Value::Str(text) = crate::compiler::stdlib::native_lorem(
                &mut ctx,
                &args,
                &world,
                world.main(),
            )
            .unwrap() else {
                panic!()
            };
            assert!(text.len() <= 64 * n as usize + 1);
        }
    }
    #[test]
    fn pure_loop_lorem_admission_bound_preserves_output_and_fallback() {
        for n in [0, 4096, 4097] {
            let world = TestWorld::new(&format!("#for i in (1,) [#lorem({n})]"));
            let session = SubtreeSession::new();
            pure_loop_check(&world, &session);
            pure_loop_check(&world, &session);
            assert_eq!(session.stats().0 > 0, n <= 4096, "count {n}");
        }
    }
}
