//! Independent public-API controls for root-only evaluation.
//! Compile unchanged against baseline, neutral and delayed stdlib artifacts.
use std::num::NonZeroU16;
use std::sync::atomic::{AtomicUsize, Ordering};

use comemo::Track;
use typst_core::compiler::eval::{eval_with_full_error_target_and_features, EvalTarget};
use typst_core::contracts::world::{SysInputs, World};
use typst_core::entities::compiler_features::{Feature, Features};
use typst_core::entities::element_registry::ElementRegistry;
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::FontBook;
use typst_core::entities::module::Module;
use typst_core::entities::sink::Sink;
use typst_core::entities::source::Source;
use typst_core::entities::value::Value;
use typst_core::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced};

struct RootWorld {
    library: Library,
    book: FontBook,
    source: Source,
    script: Vec<SysInputs>,
    calls: AtomicUsize,
}

impl RootWorld {
    fn new(script: Vec<SysInputs>) -> Self {
        assert!(!script.is_empty());
        let source = Source::new(FileId::from_raw(NonZeroU16::new(501).unwrap()),
            "#let observed = sys.inputs\n#let builtin = std\n#let line = raw.line\n#let partial = raw.line.with()\n#let alias = partial\n#let local = (std == std, raw.line == std.raw.line, partial == alias)\n".to_owned()).unwrap();
        Self { library: Library::new(), book: FontBook::new(), source, script, calls: AtomicUsize::new(0) }
    }
}

impl World for RootWorld {
    fn library(&self) -> &Library { &self.library }
    fn book(&self) -> &FontBook { &self.book }
    fn main(&self) -> FileId { self.source.id() }
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() { Ok(self.source.clone()) } else { Err(FileError::NotFound) }
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> { panic!("root-only case requested binary resource") }
    fn font(&self, _: usize) -> Option<Font> { panic!("root-only eval requested a font") }
    fn today(&self, _: Option<typst_core::entities::duration::Duration>) -> Option<Datetime> { None }
    fn include_source(&self, _: FileId, _: &str) -> Result<Source, String> { panic!("root-only case performed import") }
    fn inputs(&self) -> SysInputs {
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        self.script[call.min(self.script.len() - 1)].clone()
    }
}

fn pairs(count: usize, tag: &str) -> SysInputs {
    let mut out = SysInputs::default();
    for n in 0..count {
        out.insert(format!("key{n:05}").into(), format!("{tag}:{n}:{}", "á中🦀".repeat(32)).into());
    }
    out
}

fn value(module: &Module, name: &str) -> Value {
    module.scope().get(name).unwrap_or_else(|| panic!("missing {name}")).clone()
}

fn evaluate(world: &RootWorld, features: Features, target: EvalTarget) -> Module {
    let mut sink = Sink::new();
    let result = eval_with_full_error_target_and_features(
        &Routines::new(), world, Traced::default().track(), sink.track_mut(),
        Route::root().track(), &world.source, &ElementRegistry::new(), false, target, features);
    assert!(sink.into_diagnostics().is_empty(), "unexpected warning");
    let module = result.unwrap_or_else(|_| panic!("root-only evaluation failed"));
    assert_eq!(value(&module, "local"), Value::Array(vec![Value::Bool(true); 3]));
    module
}

fn check_inputs(module: &Module, expected: &SysInputs) {
    let Value::Dict(actual) = value(module, "observed") else { panic!("observed not dict") };
    assert_eq!(actual.len(), expected.len());
    for (n, ((key, value), (wanted_key, wanted_value))) in actual.iter().zip(expected.iter()).enumerate() {
        assert!(key == wanted_key, "key/order mismatch pair {n}");
        let Value::Str(value) = value else { panic!("nonstring pair {n}") };
        assert!(value == wanted_value, "value mismatch pair {n}");
    }
}

#[test]
fn root_only_sizes_preserve_all_input_pairs_and_single_world_query() {
    for count in [0, 1, 512, 4096] {
        let expected = pairs(count, "á-A");
        if count == 4096 {
            assert!(expected.iter().map(|(k, v)| k.len() + v.len()).sum::<usize>() > 1_048_576);
        }
        let world = RootWorld::new(vec![expected.clone()]);
        check_inputs(&evaluate(&world, Features::empty(), EvalTarget::Paged), &expected);
        assert_eq!(world.calls.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn repeated_roots_query_new_inputs_and_keep_new_builtin_identities() {
    let script = vec![pairs(512, "A"), pairs(512, "B"), pairs(512, "A")];
    let world = RootWorld::new(script.clone());
    let mut modules = Vec::new();
    for expected in &script {
        let module = evaluate(&world, Features::empty(), EvalTarget::Paged);
        check_inputs(&module, expected);
        for previous in &modules {
            for binding in ["builtin", "line", "partial"] {
                assert_ne!(value(previous, binding), value(&module, binding), "shared root identity: {binding}");
            }
        }
        modules.push(module);
    }
    assert_eq!(world.calls.load(Ordering::SeqCst), 3);
}

#[test]
fn root_inputs_order_and_last_pair_changes_remain_observable() {
    let seed = pairs(512, "A");
    let mut reverse = SysInputs::default();
    for (key, value) in seed.iter().rev() { reverse.insert(key.clone(), value.clone()); }
    let mut last = seed.clone();
    last.insert("key00511".into(), "last-pair-was-changed-🦀".into());
    let script = vec![seed, reverse, last];
    let world = RootWorld::new(script.clone());
    for expected in &script {
        check_inputs(&evaluate(&world, Features::empty(), EvalTarget::Paged), expected);
    }
    assert_eq!(world.calls.load(Ordering::SeqCst), 3);
}

#[test]
fn root_features_and_targets_do_not_leak_across_evaluations() {
    let expected = pairs(512, "A");
    let world = RootWorld::new(vec![expected.clone()]);
    for target in [EvalTarget::Paged, EvalTarget::Html] {
        for mask in 0..8 {
            let mut features = Features::empty();
            for (bit, feature) in [(1, Feature::Html), (2, Feature::A11yExtras), (4, Feature::Bundle)] {
                if mask & bit != 0 { features.enable(feature); }
            }
            check_inputs(&evaluate(&world, features, target), &expected);
        }
    }
    assert_eq!(world.calls.load(Ordering::SeqCst), 16);
}
