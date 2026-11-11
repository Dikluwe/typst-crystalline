//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/stdlib_snapshot/tests.md
//! @prompt-hash a3e819c1
//! @layer L1

#![cfg(test)]

use super::super::make_stdlib_with_features;
use super::*;
use std::collections::HashMap;
use std::num::NonZeroU16;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use crate::compiler::eval::{
    eval_expression_with_features, eval_source_with_features_and_introspector,
    eval_with_full_error_target_and_features, EvalTarget,
};
use crate::contracts::world::{SysInputs, World};
use crate::entities::compiler_features::{Feature, Features};
use crate::entities::content::Content;
use crate::entities::element_registry::ElementRegistry;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::introspector::TagIntrospector;
use crate::entities::module::Module;
use crate::entities::package_spec::PackageSpec;
use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
use crate::entities::sink::Sink;
use crate::entities::source::Source;
use crate::entities::source_result::{
    Severity, SourceDiagnostic, SourceResult, Tracepoint,
};
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileResult, Font, Library, Route, Routines, Traced,
};
use comemo::Track;

fn inputs(pairs: &[(&str, &str)]) -> SysInputs {
    let mut out = SysInputs::default();
    for (key, value) in pairs {
        out.insert((*key).into(), (*value).into());
    }
    out
}
fn input_pairs(input: &SysInputs) -> Vec<(String, String)> {
    input.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}
fn flags(mask: u8) -> Features {
    let mut out = Features::empty();
    for (bit, flag) in
        [(1, Feature::Html), (2, Feature::A11yExtras), (4, Feature::Bundle)]
    {
        if mask & bit != 0 {
            out.enable(flag);
        }
    }
    out
}
fn id(raw: u16) -> FileId {
    FileId::from_raw(NonZeroU16::new(raw).unwrap())
}

struct MockWorld {
    library: Library,
    book: FontBook,
    main: Source,
    files: HashMap<String, Source>,
    input_script: Vec<SysInputs>,
    input_calls: AtomicUsize,
    observations: Mutex<Vec<Vec<(String, String)>>>,
    resolutions: Mutex<Vec<(FileId, String)>>,
    packages: Mutex<Vec<String>>,
    record_observations: bool,
}
impl MockWorld {
    fn new(main: &str, files: &[(&str, &str)]) -> Self {
        let files = files
            .iter()
            .enumerate()
            .map(|(i, (path, text))| {
                (
                    (*path).to_owned(),
                    Source::new(id(i as u16 + 2), (*text).to_owned()).unwrap(),
                )
            })
            .collect();
        Self {
            library: Library::new(),
            book: FontBook::new(),
            main: Source::new(id(1), main.to_owned()).unwrap(),
            files,
            input_script: vec![SysInputs::default()],
            input_calls: AtomicUsize::new(0),
            observations: Mutex::new(Vec::new()),
            resolutions: Mutex::new(Vec::new()),
            packages: Mutex::new(Vec::new()),
            record_observations: true,
        }
    }
    fn scripted(mut self, script: Vec<SysInputs>) -> Self {
        assert!(!script.is_empty());
        self.input_script = script;
        self
    }
    fn replace(&mut self, path: &str, text: &str) {
        if path == "main.typ" {
            self.main = Source::new(self.main.id(), text.to_owned()).unwrap();
        } else {
            let old = self.files[path].id();
            self.files
                .insert(path.to_owned(), Source::new(old, text.to_owned()).unwrap());
        }
    }
}
impl World for MockWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        self.main.id()
    }
    fn source(&self, file: FileId) -> FileResult<Source> {
        if file == self.main.id() {
            return Ok(self.main.clone());
        }
        self.files
            .values()
            .find(|s| s.id() == file)
            .cloned()
            .map_or_else(|| panic!("unexpected source request"), Ok)
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        panic!("unexpected binary file request")
    }
    fn font(&self, _: usize) -> Option<Font> {
        panic!("unexpected font request")
    }
    fn today(&self, _: Option<crate::entities::duration::Duration>) -> Option<Datetime> {
        panic!("unexpected date request")
    }
    fn include_source(&self, current: FileId, path: &str) -> Result<Source, String> {
        if self.record_observations {
            self.resolutions.lock().unwrap().push((current, path.to_owned()));
        }
        if path == "main.typ" {
            return Ok(self.main.clone());
        }
        Ok(self
            .files
            .get(path)
            .unwrap_or_else(|| panic!("unexpected include: {path}"))
            .clone())
    }
    fn resolve_path(&self, _: FileId, path: &str) -> Result<RootedPath, String> {
        Ok(RootedPath::new(
            VirtualRoot::Project,
            VirtualPath::new(path).map_err(|e| format!("{e:?}"))?,
        ))
    }
    fn include_path(&self, path: &RootedPath) -> Result<Source, String> {
        self.include_source(
            self.main.id(),
            path.vpath().get_with_slash().trim_start_matches('/'),
        )
    }
    fn read_path(&self, _: &RootedPath) -> Result<Arc<Vec<u8>>, String> {
        panic!("unexpected read_path request")
    }
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String> {
        self.packages.lock().unwrap().push(spec.to_string());
        self.files
            .get("package.typ")
            .cloned()
            .map_or_else(|| panic!("unexpected package request"), Ok)
    }
    fn inputs(&self) -> SysInputs {
        let number = self.input_calls.fetch_add(1, Ordering::SeqCst);
        let input = self.input_script[number.min(self.input_script.len() - 1)].clone();
        if self.record_observations {
            self.observations.lock().unwrap().push(input_pairs(&input));
        }
        input
    }
}
fn evaluate(
    world: &MockWorld,
    features: Features,
    target: EvalTarget,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
    evaluate_registry(world, features, target, &ElementRegistry::new())
}
fn evaluate_registry(
    world: &MockWorld,
    features: Features,
    target: EvalTarget,
    registry: &ElementRegistry,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
    let mut sink = Sink::new();
    let result = eval_with_full_error_target_and_features(
        &Routines::new(),
        world,
        Traced::default().track(),
        sink.track_mut(),
        Route::root().track(),
        &world.main,
        registry,
        false,
        target,
        features,
    );
    (result, sink.into_diagnostics())
}
fn successful(world: &MockWorld) -> Module {
    let (result, warnings) = evaluate(world, Features::empty(), EvalTarget::Paged);
    assert!(warnings.is_empty(), "unexpected warnings: {warnings:?}");
    result.unwrap_or_else(|errors| panic!("unexpected errors: {errors:?}"))
}
fn value(module: &Module, name: &str) -> Value {
    module
        .scope()
        .get(name)
        .unwrap_or_else(|| panic!("missing {name}"))
        .clone()
}
fn strings(items: &[&str]) -> Value {
    Value::Array(items.iter().map(|s| Value::Str((*s).into())).collect())
}
const EXPORT_MODULES: &str = "#let stdmod = std\n#let calcmod = calc\n#let symmod = sym\n#let emojimod = emoji\n#let mathmod = math\n#let sysmod = sys\n#let formatmod = format\n#let aliases = (std.calc == calc, std.sym == sym, std.emoji == emoji, std.math == math, std.sys == sys, std.format == format, std.pdf == format.pdf)\n";

#[test]
fn modules_keep_per_file_language_identity_and_within_file_aliases() {
    let world = MockWorld::new("#import \"a.typ\" as a\n#import \"b.typ\" as b\n#let out = (a.calcmod == calc, a.symmod == sym, a.emojimod == emoji, a.mathmod == math, a.sysmod == sys, a.formatmod == format, a.stdmod == std, a.calcmod == b.calcmod, a.formatmod == b.formatmod, a.stdmod == b.stdmod)\n#let aaliases = a.aliases\n#let baliases = b.aliases\n", &[("a.typ", EXPORT_MODULES), ("b.typ", EXPORT_MODULES)]);
    let module = successful(&world);
    assert_eq!(value(&module, "out"), Value::Array(vec![Value::Bool(false); 10]));
    assert_eq!(value(&module, "aaliases"), Value::Array(vec![Value::Bool(true); 7]));
    assert_eq!(value(&module, "baliases"), Value::Array(vec![Value::Bool(true); 7]));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
}

#[test]
fn repeated_imports_are_evaluated_and_keep_fresh_module_identity() {
    let world = MockWorld::new("#import \"a.typ\" as first\n#import \"a.typ\" as second\n#let out = (first == second, first.calcmod == second.calcmod, first.stdmod == second.stdmod)\n", &[("a.typ", EXPORT_MODULES)]);
    assert_eq!(
        value(&successful(&world), "out"),
        Value::Array(vec![Value::Bool(false); 3])
    );
    assert_eq!(
        world
            .resolutions
            .lock()
            .unwrap()
            .iter()
            .map(|(_, p)| p.as_str())
            .collect::<Vec<_>>(),
        ["a.typ", "a.typ"]
    );
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
}

#[test]
fn nested_and_package_imports_keep_inputs_and_namespace_behavior() {
    let world = MockWorld::new("#import \"middle.typ\" as middle\n#import \"@preview/probe:0.1.0\" as pkg\n#let out = (middle.answer, pkg.answer, middle.calcmod == calc, pkg.calcmod == calc)\n", &[("middle.typ", "#import \"leaf.typ\" as leaf\n#let answer = leaf.answer + 1\n#let calcmod = calc"), ("leaf.typ", "#let answer = int(sys.inputs.n)\n"), ("package.typ", "#let answer = int(sys.inputs.n)\n#let calcmod = calc")]).scripted(vec![inputs(&[("n", "41")])]);
    assert_eq!(
        value(&successful(&world), "out"),
        Value::Array(vec![
            Value::Int(42),
            Value::Int(41),
            Value::Bool(false),
            Value::Bool(false)
        ])
    );
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 4);
    assert_eq!(*world.packages.lock().unwrap(), ["@preview/probe:0.1.0"]);
    assert_eq!(world.resolutions.lock().unwrap()[1].0, world.files["middle.typ"].id());
}

#[test]
fn inputs_changed_or_reordered_per_request_are_not_replaced_by_the_root_snapshot() {
    let script = vec![
        inputs(&[("a", "A"), ("b", "B")]),
        inputs(&[("b", "B"), ("a", "A")]),
        inputs(&[("a", "changed"), ("c", "C")]),
    ];
    let world = MockWorld::new("#let rootkeys = sys.inputs.keys()\n#import \"child.typ\" as first\n#import \"child.typ\" as second\n#let out = (rootkeys, first.keys, first.values, second.keys, second.values)\n", &[("child.typ", "#let keys = sys.inputs.keys()\n#let values = sys.inputs.values()")]).scripted(script.clone());
    assert_eq!(
        value(&successful(&world), "out"),
        Value::Array(vec![
            strings(&["a", "b"]),
            strings(&["b", "a"]),
            strings(&["B", "A"]),
            strings(&["a", "c"]),
            strings(&["changed", "C"])
        ])
    );
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
    assert_eq!(
        *world.observations.lock().unwrap(),
        script.iter().map(input_pairs).collect::<Vec<_>>()
    );
}

#[test]
fn separate_eval_entries_and_worlds_do_not_reuse_old_inputs() {
    for reverse in [false, true] {
        let values = if reverse { ["B", "A", "B"] } else { ["A", "B", "A"] };
        let world = MockWorld::new("#let out = sys.inputs.tag", &[])
            .scripted(values.iter().map(|tag| inputs(&[("tag", tag)])).collect());
        for tag in values {
            assert_eq!(value(&successful(&world), "out"), Value::Str(tag.into()));
        }
        assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
    }
    for tag in ["world-A", "world-B", "world-A"] {
        let world = MockWorld::new("#let out = sys.inputs.tag", &[])
            .scripted(vec![inputs(&[("tag", tag)])]);
        assert_eq!(value(&successful(&world), "out"), Value::Str(tag.into()));
    }
}

#[test]
fn watch_like_re_evaluation_observes_main_and_import_edits() {
    let mut world = MockWorld::new(
        "#import \"child.typ\" as child\n#let out = child.answer + 1",
        &[("child.typ", "#let answer = 10")],
    );
    assert_eq!(value(&successful(&world), "out"), Value::Int(11));
    assert_eq!(value(&successful(&world), "out"), Value::Int(11));
    world.replace("child.typ", "#let answer = 40");
    assert_eq!(value(&successful(&world), "out"), Value::Int(41));
    world.replace(
        "main.typ",
        "#import \"child.typ\" as child\n#let out = child.answer + 2",
    );
    assert_eq!(value(&successful(&world), "out"), Value::Int(42));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 8);
}

#[test]
fn shadowing_and_imported_show_rules_do_not_contaminate_the_caller() {
    let world = MockWorld::new("#import \"child.typ\" as child\n#let out = (upper(\"abc\"), child.saved)\nx", &[("child.typ", "#let upper(x) = \"shadow\"\n#let std = \"shadow\"\n#let saved = upper(\"abc\")\n#show \"x\": [BAD]\n")]);
    let module = successful(&world);
    assert_eq!(
        value(&module, "out"),
        Value::Array(vec![Value::Str("ABC".into()), Value::Str("shadow".into())])
    );
    assert_eq!(module.content().unwrap().plain_text().trim(), "x");
    let world = MockWorld::new("#let std = (html: 1, asset: 2)\n#let format = (html: 3, bundle: 4)\n#let out = (std.html, std.asset, format.html, format.bundle)", &[]);
    assert_eq!(
        value(&successful(&world), "out"),
        Value::Array((1..=4).map(Value::Int).collect())
    );
}

#[test]
fn include_keeps_its_existing_caller_scope_semantics() {
    let world = MockWorld::new(
        "#let marker = 2\n#include \"included.typ\"\n#let out = (marker, newly)",
        &[("included.typ", "#let marker = 3\n#let newly = upper(\"ok\")")],
    );
    assert_eq!(
        value(&successful(&world), "out"),
        Value::Array(vec![Value::Int(3), Value::Str("OK".into())])
    );
    assert_eq!(
        world.input_calls.load(Ordering::SeqCst),
        1,
        "include does not construct an imported stdlib"
    );
}

#[test]
fn registry_bindings_do_not_leak_into_imports_or_other_evaluations() {
    let mut registry = ElementRegistry::new();
    registry.register("customprobe", Arc::new(|_| Ok(Content::text("registered"))));
    let root = MockWorld::new("#let out = customprobe", &[]);
    let (result, warnings) =
        evaluate_registry(&root, Features::empty(), EvalTarget::Paged, &registry);
    assert!(warnings.is_empty());
    assert!(matches!(value(&result.unwrap(), "out"), Value::Func(_)));
    let imported = MockWorld::new(
        "#import \"child.typ\" as child",
        &[("child.typ", "#let out = customprobe")],
    );
    let (result, _) =
        evaluate_registry(&imported, Features::empty(), EvalTarget::Paged, &registry);
    assert!(result.unwrap_err()[0].message.contains("customprobe"));
    let (result, _) = evaluate(&root, Features::empty(), EvalTarget::Paged);
    assert!(result.unwrap_err()[0].message.contains("customprobe"));
}

fn feature_case(
    mask: u8,
    target: EvalTarget,
    access: &str,
    field: &str,
    feature: Feature,
    label: &str,
) {
    let child = format!("#let out = {access}\n");
    let world = MockWorld::new(
        "#import \"child.typ\" as child\n#let out = child.out",
        &[("child.typ", &child)],
    );
    let (result, warnings) = evaluate(&world, flags(mask), target);
    assert!(warnings.is_empty());
    if flags(mask).contains(feature) {
        assert!(
            matches!(value(&result.unwrap(), "out"), Value::Func(_)),
            "mask {mask}/{access}"
        );
    } else {
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        let error = &errors[0];
        assert_eq!(error.message, format!("cannot access field `{field}` because the `{label}` feature is not enabled"));
        assert_eq!(
            error.hints,
            [
                format!("try enabling the `{label}` feature"),
                "see https://typst.app/help/compiler-features for more details"
                    .to_owned()
            ]
        );
        assert_eq!(error.span.id(), Some(world.files["child.typ"].id()));
        assert_eq!(
            error
                .trace
                .iter()
                .filter_map(|t| if let Tracepoint::Import(path) = &t.v {
                    Some(path.as_str())
                } else {
                    None
                })
                .collect::<Vec<_>>(),
            ["child.typ"]
        );
    }
}
#[test]
fn features_targets_and_builtin_markers_remain_independent_across_roots() {
    for mask in [0, 7, 1, 0, 4, 2, 6, 3, 5, 0] {
        for target in [EvalTarget::Paged, EvalTarget::Html, EvalTarget::Bundle] {
            feature_case(mask, target, "std.html", "html", Feature::Html, "html");
            feature_case(mask, target, "std.format.html", "html", Feature::Html, "html");
            feature_case(mask, target, "std.asset", "asset", Feature::Bundle, "bundle");
            feature_case(
                mask,
                target,
                "std.format.bundle",
                "bundle",
                Feature::Bundle,
                "bundle",
            );
        }
    }
}

#[test]
fn disabled_builtin_gates_precede_argument_effects_in_imports() {
    for access in ["std.html", "std.format.html", "std.asset", "std.format.bundle"] {
        let child = format!("#let out = {access}(panic(\"ARGUMENT RAN\"))\n");
        let world =
            MockWorld::new("#import \"child.typ\" as child", &[("child.typ", &child)]);
        let (result, _) = evaluate(&world, Features::empty(), EvalTarget::Paged);
        let errors = result.unwrap_err();
        assert!(!errors[0].message.contains("ARGUMENT RAN"), "{access}");
        assert!(
            errors[0].message.contains("feature is not enabled"),
            "{access}: {errors:?}"
        );
        let (result, _) = evaluate(&world, flags(7), EvalTarget::Paged);
        assert_eq!(result.unwrap_err()[0].message, "panicked with: ARGUMENT RAN");
    }
}

#[test]
fn syntax_failure_and_nested_trace_preserve_the_original_source() {
    let world = MockWorld::new(
        "#import \"middle.typ\" as middle",
        &[
            ("middle.typ", "#import \"bad.typ\" as bad"),
            ("bad.typ", "#panic(\"prefix executed\")\n#let or(x)=x"),
        ],
    );
    let bad = &world.files["bad.typ"];
    let expected = bad.root().errors();
    assert!(!expected.is_empty());
    let (result, _) = evaluate(&world, Features::empty(), EvalTarget::Paged);
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), expected.len());
    for (error, source_error) in errors.iter().zip(expected) {
        assert_eq!(error.severity, Severity::Error);
        assert_eq!(error.message, source_error.message.as_str());
        assert_eq!(
            error.hints,
            source_error.hints.iter().map(ToString::to_string).collect::<Vec<_>>()
        );
        assert_eq!(error.span, source_error.span);
        assert_eq!(
            error
                .trace
                .iter()
                .filter_map(|t| if let Tracepoint::Import(path) = &t.v {
                    Some(path.as_str())
                } else {
                    None
                })
                .collect::<Vec<_>>(),
            ["bad.typ", "middle.typ"]
        );
    }
    assert_eq!(
        world.input_calls.load(Ordering::SeqCst),
        2,
        "invalid child fails before stdlib acquisition"
    );
}

#[test]
fn cyclic_import_remains_an_error_instead_of_becoming_a_cached_module() {
    let world = MockWorld::new(
        "#import \"child.typ\" as child",
        &[("child.typ", "#import \"main.typ\" as parent")],
    );
    let (result, _) = evaluate(&world, Features::empty(), EvalTarget::Paged);
    assert!(result.unwrap_err()[0]
        .message
        .contains("ciclo de importação detectado"));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 2);
}

#[test]
fn expression_and_source_facades_remain_compilation_local() {
    let world = MockWorld::new("", &[]).scripted(vec![
        inputs(&[("tag", "first")]),
        inputs(&[("tag", "second")]),
        inputs(&[("tag", "third")]),
    ]);
    let (first, warnings) =
        eval_expression_with_features(&world, "sys.inputs.tag", Features::empty());
    assert!(warnings.is_empty());
    assert_eq!(first.unwrap(), Value::Str("first".into()));
    let source = Source::new_with_parser(
        id(1),
        "sys.inputs.tag".to_owned(),
        crate::compiler::parse::parse_code,
    )
    .unwrap();
    let (second, warnings) = eval_source_with_features_and_introspector(
        &world,
        &source,
        Features::empty(),
        &TagIntrospector::empty(),
    );
    assert!(warnings.is_empty());
    assert_eq!(second.unwrap(), Value::Str("second".into()));
    let (third, warnings) =
        eval_expression_with_features(&world, "sys.inputs.tag", Features::empty());
    assert!(warnings.is_empty());
    assert_eq!(third.unwrap(), Value::Str("third".into()));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
}

#[test]
fn empty_inputs_and_ordinary_homonymous_modules_remain_distinct() {
    for script in [
        vec![SysInputs::default(), inputs(&[("tag", "new")])],
        vec![inputs(&[("tag", "old")]), SysInputs::default()],
    ] {
        let expected = strings(&script[1].keys().map(|k| k.as_str()).collect::<Vec<_>>());
        let world = MockWorld::new(
            "#import \"child.typ\" as child\n#let out = child.keys",
            &[("child.typ", "#let keys = sys.inputs.keys()")],
        )
        .scripted(script);
        assert_eq!(value(&successful(&world), "out"), expected);
    }
    let world = MockWorld::new("#import \"format.typ\" as ordinary_format\n#import \"global.typ\" as ordinary_global\n#let out = (ordinary_format.html, ordinary_format.bundle, ordinary_global.html, ordinary_global.asset)", &[("format.typ", "#let html = 1\n#let bundle = 2"), ("global.typ", "#let html = 3\n#let asset = 4")]);
    assert_eq!(
        value(&successful(&world), "out"),
        Value::Array((1..=4).map(Value::Int).collect())
    );
}

#[test]
fn element_functions_in_builtin_namespaces_keep_per_file_language_identity() {
    // raw is native-by-name, but raw.line is an Element function whose
    // identity is its individual Func Arc. Sharing one finished namespace
    // across files would change these language-level == results.
    let exports =
        "#let rawline = raw.line\n#let stdline = std.raw.line\n#let alias = rawline\n";
    let world = MockWorld::new(
        "#let local = raw.line\n#import \"a.typ\" as first\n#import \"b.typ\" as sibling\n#import \"a.typ\" as repeated\n#let out = (local == raw.line, local == std.raw.line, first.rawline == first.stdline, first.rawline == first.alias, first.rawline == local, first.rawline == sibling.rawline, first.rawline == repeated.rawline)\n",
        &[("a.typ", exports), ("b.typ", exports)],
    );
    assert_eq!(
        value(&successful(&world), "out"),
        Value::Array(vec![
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(false),
            Value::Bool(false),
            Value::Bool(false),
        ]),
    );
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 4);
}

// Additional qualification uses only the same public World/eval APIs above.
// No diagnostic cfg, policy method, builder counter or private snapshot portal.
fn qualification_inputs(count: usize) -> SysInputs {
    let mut result = SysInputs::default();
    let payload = "á中🦀".repeat(28); // 252 UTF-8 bytes, plus distinct index prefix
    for index in 0..count {
        result
            .insert(format!("key{index:05}").into(), format!("{index}:{payload}").into());
    }
    result
}
fn assert_dict_matches_script(actual: &Value, expected: &SysInputs, label: &str) {
    let Value::Dict(actual) = actual else {
        panic!("{label}: expected a dictionary");
    };
    assert_eq!(actual.len(), expected.len(), "{label}: pair count");
    for (index, ((actual_key, actual_value), (expected_key, expected_value))) in
        actual.iter().zip(expected.iter()).enumerate()
    {
        let Value::Str(actual_value) = actual_value else {
            panic!("{label}: pair {index} value must be string");
        };
        assert!(
            actual_key.as_str() == expected_key.as_str(),
            "{label}: key/order mismatch at pair {index}"
        );
        assert!(
            actual_value.as_str() == expected_value.as_str(),
            "{label}: value mismatch at pair {index}"
        );
    }
}
fn assert_world_observations(world: &MockWorld, script: &[SysInputs]) {
    assert_eq!(
        world.input_calls.load(Ordering::SeqCst),
        script.len(),
        "one query per source evaluation"
    );
    let observations = world.observations.lock().unwrap();
    assert_eq!(observations.len(), script.len());
    for (call, (observed, expected)) in observations.iter().zip(script).enumerate() {
        assert_eq!(observed.len(), expected.len(), "call {call}: observed count");
        for (index, ((key, value), (wanted_key, wanted_value))) in
            observed.iter().zip(expected.iter()).enumerate()
        {
            assert!(
                key.as_str() == wanted_key.as_str(),
                "call {call}: requested key/order at pair {index}"
            );
            assert!(
                value.as_str() == wanted_value.as_str(),
                "call {call}: requested value at pair {index}"
            );
        }
    }
}
fn reverse_inputs(input: &SysInputs) -> SysInputs {
    let mut result = SysInputs::default();
    for (key, value) in input.iter().rev() {
        result.insert(key.clone(), value.clone());
    }
    result
}
fn modify_value(input: &SysInputs, index: usize) -> SysInputs {
    let mut result = input.clone();
    let key = result.keys().nth(index).unwrap().clone();
    // Keep Unicode and UTF-8 length stable. Guard must compare all contents.
    let previous = result.get(&key).unwrap();
    let replacement = previous.replacen("á", "é", 1);
    assert_eq!(replacement.len(), previous.len());
    result.insert(key, replacement);
    result
}
fn exercise_qualification_script(
    script: Vec<SysInputs>,
    features: Features,
    target: EvalTarget,
) {
    assert_eq!(script.len(), 5);
    let world = MockWorld::new(
        "#let observed = sys.inputs\n#import \"middle.typ\" as middle\n#import \"child.typ\" as first\n#import \"child.typ\" as second\n#let out = (observed, middle.observed, middle.leaf.observed, first.observed, second.observed)",
        &[("middle.typ", "#let observed = sys.inputs\n#import \"leaf.typ\" as leaf"),
          ("leaf.typ", "#let observed = sys.inputs"), ("child.typ", "#let observed = sys.inputs")],
    ).scripted(script.clone());
    let (result, warnings) = evaluate(&world, features, target);
    assert!(warnings.is_empty(), "qualification warnings");
    let module =
        result.unwrap_or_else(|_| panic!("qualification evaluation unexpectedly failed"));
    let Value::Array(actual) = value(&module, "out") else {
        panic!("out must be array");
    };
    assert_eq!(actual.len(), 5);
    for (call, (actual, expected)) in actual.iter().zip(&script).enumerate() {
        assert_dict_matches_script(actual, expected, &format!("source {call}"));
    }
    assert_world_observations(&world, &script);
    assert_eq!(
        world
            .resolutions
            .lock()
            .unwrap()
            .iter()
            .map(|(_, path)| path.as_str())
            .collect::<Vec<_>>(),
        ["middle.typ", "leaf.typ", "child.typ", "child.typ"]
    );
}

#[test]
fn qualification_sizes_and_unicode_mebibyte_preserve_every_pair_and_request() {
    for count in [0, 1, 64, 512, 4096] {
        let input = qualification_inputs(count);
        if count == 4096 {
            let bytes: usize =
                input.iter().map(|(key, value)| key.len() + value.len()).sum();
            assert!(bytes >= 1_048_576, "large-input control must reach one MiB");
        }
        exercise_qualification_script(
            vec![input; 5],
            Features::empty(),
            EvalTarget::Paged,
        );
    }
}

#[test]
fn qualification_first_middle_last_mismatch_is_not_replaced_by_import_template() {
    for count in [1, 64, 512, 4096] {
        let seed = qualification_inputs(count);
        for index in [0, count / 2, count - 1] {
            let changed = modify_value(&seed, index);
            exercise_qualification_script(
                vec![seed.clone(), seed.clone(), changed, seed.clone(), seed.clone()],
                Features::empty(),
                EvalTarget::Paged,
            );
        }
    }
}

#[test]
fn qualification_reorder_remove_add_and_duplicate_names_keep_indexmap_semantics() {
    for count in [0, 1, 64, 512, 4096] {
        let seed = qualification_inputs(count);
        let reversed = reverse_inputs(&seed);
        let mut removed = SysInputs::default();
        for (index, (key, value)) in seed.iter().enumerate() {
            if index != count / 2 {
                removed.insert(key.clone(), value.clone());
            }
        }
        let mut added = seed.clone();
        added.insert("additional-key".into(), "novo-á中🦀".into());
        exercise_qualification_script(
            vec![seed.clone(), seed, reversed, removed, added],
            Features::empty(),
            EvalTarget::Paged,
        );
    }
    let duplicated = inputs(&[
        ("first", "old"),
        ("middle", "M"),
        ("first", "new"),
        ("last", "L"),
        ("middle", "changed"),
    ]);
    assert_eq!(
        input_pairs(&duplicated),
        [
            ("first".to_owned(), "new".to_owned()),
            ("middle".to_owned(), "changed".to_owned()),
            ("last".to_owned(), "L".to_owned())
        ]
    );
    exercise_qualification_script(
        vec![duplicated; 5],
        Features::empty(),
        EvalTarget::Paged,
    );
}

#[test]
fn qualification_features_targets_and_large_inputs_remain_local_between_roots() {
    let large = qualification_inputs(512);
    for target in [EvalTarget::Paged, EvalTarget::Html] {
        for mask in 0..8 {
            exercise_qualification_script(vec![large.clone(); 5], flags(mask), target);
        }
    }
    // Existing language-level gate controls also cover all feature combinations
    // and imported namespaces. Features are immutable within one EvalContext;
    // this public test does not fabricate an unreachable feature mismatch.
    features_targets_and_builtin_markers_remain_independent_across_roots();
}

#[test]
fn qualification_repeated_large_world_roots_never_reuse_old_inputs() {
    let seed = qualification_inputs(4096);
    let changed = modify_value(&seed, 2048);
    let script = vec![seed.clone(), changed.clone(), seed.clone()];
    let world = MockWorld::new("#let out = sys.inputs", &[]).scripted(script.clone());
    for (root, expected) in script.iter().enumerate() {
        assert_dict_matches_script(
            &value(&successful(&world), "out"),
            expected,
            &format!("root {root}"),
        );
    }
    assert_world_observations(&world, &script);
}

#[test]
fn qualification_partial_functions_alias_locally_and_remain_fresh_across_imports() {
    let world = MockWorld::new(
        "#let bound = raw.line.with()\n#let alias = bound\n#import \"child.typ\" as first\n#import \"child.typ\" as second\n#let out = (bound == alias, first.bound == first.alias, first.bound == second.bound, bound == first.bound)",
        &[("child.typ", "#let bound = raw.line.with()\n#let alias = bound")],
    );
    assert_eq!(
        value(&successful(&world), "out"),
        Value::Array(vec![
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(false),
            Value::Bool(false)
        ])
    );
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
}

fn helper_inputs(scope: &Scope, expected: &SysInputs) {
    let Value::Module(sys) = scope.get("sys").unwrap() else { panic!("sys module") };
    assert_dict_matches_script(sys.scope().get("inputs").unwrap(), expected, "helper");
}
fn helper_features(scope: &Scope, features: Features) {
    assert!(scope.get("std").is_none(), "builder precedes lexical std installation");
    assert_eq!(scope.get("asset").is_some(), features.contains(Feature::Bundle));
    assert!(scope.get("html").is_some());
    let Value::Func(pdf) = scope.get("pdf").unwrap() else { panic!("pdf function") };
    for name in ["table-summary", "header-cell", "data-cell"] {
        assert_eq!(
            pdf.namespace().unwrap().get(name).is_some(),
            features.contains(Feature::A11yExtras),
            "pdf.{name}"
        );
    }
    let Value::Module(format) = scope.get("format").unwrap() else {
        panic!("format module")
    };
    assert_eq!(scope.get("pdf"), format.scope().get("pdf"));
    if let Some(asset) = scope.get("asset") {
        let Value::Func(bundle) = format.scope().get("bundle").unwrap() else {
            panic!("bundle function")
        };
        assert_eq!(Some(asset), bundle.namespace().unwrap().get("asset"));
    }
}
fn raw_line(scope: &Scope) -> Value {
    let Value::Func(raw) = scope.get("raw").unwrap() else { panic!("raw function") };
    raw.namespace().unwrap().get("line").unwrap().clone()
}
fn distinct_files(first: &Scope, second: &Scope) {
    for name in ["calc", "sym", "emoji", "math", "sys", "format"] {
        assert_ne!(first.get(name), second.get(name), "per-file {name} identity");
    }
    assert_ne!(raw_line(first), raw_line(second), "per-file raw.line identity");
}
#[test]
fn cold_slot_installs_seed_and_equal_queries_reuse_only_seed_identity() {
    let input = qualification_inputs(1);
    let slot = RefCell::new(None);
    assert!(slot.borrow().is_none());
    let (first, seed) = resolve(&slot, &input, Features::empty());
    let seed = seed.expect("first import must create a reusable seed");
    assert!(Arc::ptr_eq(slot.borrow().as_ref().unwrap(), &seed));
    helper_inputs(seed.scope(), &input);
    let (second, next) = resolve(&slot, &input, Features::empty());
    assert!(Arc::ptr_eq(&seed, &next.unwrap()));
    helper_inputs(&first, &input);
    helper_inputs(&second, &input);
    distinct_files(&first, &second);
    helper_features(&first, Features::empty());
    helper_features(&second, Features::empty());
}
#[test]
fn all_ordered_input_pairs_guard_seed_and_mismatch_subtree_ancestry() {
    for count in [0, 1, 512, 4096] {
        let original = qualification_inputs(count);
        if count == 4096 {
            assert!(
                original.iter().map(|(k, v)| k.len() + v.len()).sum::<usize>()
                    > 1_048_576
            );
        }
        let slot = RefCell::new(None);
        let (_, parent) = resolve(&slot, &original, Features::empty());
        let parent = parent.unwrap();
        let mut variants = vec![];
        if count > 0 {
            for index in [0, count / 2, count - 1] {
                variants.push(modify_value(&original, index));
            }
            let mut removed = SysInputs::default();
            for (index, (k, v)) in original.iter().enumerate() {
                if index != count / 2 {
                    removed.insert(k.clone(), v.clone());
                }
            }
            variants.push(removed);
        }
        if count > 1 {
            variants.push(reverse_inputs(&original));
        }
        let mut added = original.clone();
        added.insert("extra".into(), "novo-á中🦀".into());
        variants.push(added);
        for requested in variants {
            let (scope, child) = resolve(&slot, &requested, Features::empty());
            helper_inputs(&scope, &requested);
            let child = child.expect("mismatch subtree must get its own seed");
            assert!(!Arc::ptr_eq(&parent, &child));
            assert!(Arc::ptr_eq(slot.borrow().as_ref().unwrap(), &parent));
            helper_inputs(child.scope(), &requested);
            let child_slot = RefCell::new(Some(child.clone()));
            let (nested, nested_seed) =
                resolve(&child_slot, &requested, Features::empty());
            assert!(Arc::ptr_eq(&child, &nested_seed.unwrap()));
            helper_inputs(&nested, &requested);
            distinct_files(&scope, &nested);
            let (back, back_seed) = resolve(&slot, &original, Features::empty());
            assert!(Arc::ptr_eq(&parent, &back_seed.unwrap()));
            helper_inputs(&back, &original);
            distinct_files(&scope, &back);
        }
    }
}
#[test]
fn all_complete_feature_transitions_guard_seed_and_actual_pdf_membership() {
    let input = qualification_inputs(1);
    for origin in 0..8 {
        let slot = RefCell::new(Some(Arc::new(StdlibSnapshot::new(
            input.clone(),
            flags(origin),
            make_stdlib_with_features(&input, flags(origin)),
        ))));
        let parent = slot.borrow().as_ref().unwrap().clone();
        for requested in 0..8 {
            let (scope, child) = resolve(&slot, &input, flags(requested));
            helper_features(&scope, flags(requested));
            helper_inputs(&scope, &input);
            assert_eq!(Arc::ptr_eq(&parent, &child.unwrap()), origin == requested);
            assert!(Arc::ptr_eq(slot.borrow().as_ref().unwrap(), &parent));
            let (back, back_seed) = resolve(&slot, &input, flags(origin));
            assert!(Arc::ptr_eq(&parent, &back_seed.unwrap()));
            helper_features(&back, flags(origin));
            distinct_files(&scope, &back);
        }
    }
}

fn inspect_scope(scope: &Scope, depth: usize, elements: &mut usize) {
    assert!(depth < 64, "builtin registry graph must be acyclic and bounded");
    for (_, binding) in scope.iter() {
        inspect_value(binding.value(), depth + 1, elements);
    }
}
fn inspect_value(value: &Value, depth: usize, elements: &mut usize) {
    use crate::entities::func::FuncRepr;
    assert!(depth < 64, "registry graph depth/cycle");
    match value {
        Value::Module(module) => inspect_scope(module.scope(), depth + 1, elements),
        Value::Array(items) => {
            for item in items {
                inspect_value(item, depth + 1, elements);
            }
        }
        Value::Dict(items) => {
            for item in items.values() {
                inspect_value(item, depth + 1, elements);
            }
        }
        Value::Args(args) => {
            for occurrence in args.occurrence_sequence() {
                inspect_value(&occurrence.value, depth + 1, elements);
            }
        }
        Value::Func(function) => match function.repr() {
            FuncRepr::Native(_)
            | FuncRepr::NativeWithEngine(_)
            | FuncRepr::Format(_, _) => {
                if let Some(namespace) = function.namespace() {
                    inspect_scope(namespace, depth + 1, elements);
                }
            }
            FuncRepr::Element(element) => {
                assert_eq!(element.name, "raw.line");
                *elements += 1;
            }
            FuncRepr::With(_) => {
                panic!("registry added With: qualify bounded graph before expansion")
            }
            FuncRepr::Closure(_) | FuncRepr::Plugin(_) => {
                panic!("registry cannot contain closures/plugins")
            }
        },
        Value::None
        | Value::Auto
        | Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Str(_)
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Dir(_)
        | Value::Type(_)
        | Value::Symbol(_)
        | Value::Version(_) => {}
        Value::Content(content) => inspect_builtin_content(content, depth + 1),
        Value::Datetime(_)
        | Value::LocatedContent(_, _)
        | Value::Stroke(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Selector(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Path(_)
        | Value::SpotColorant(_) => {
            panic!("registry leaf outside qualified builtin graph")
        }
    }
}
#[test]
fn complete_real_registry_graph_is_bounded_in_every_feature_profile() {
    for mask in 0..8 {
        let scope = make_stdlib_with_features(&qualification_inputs(1), flags(mask));
        let mut elements = 0;
        inspect_scope(&scope, 0, &mut elements);
        assert!(elements > 0, "raw.line graph control");
        helper_features(&scope, flags(mask));
    }
}
#[test]
fn private_with_fixture_preserves_aliases_occurrences_and_diagnostic_spans() {
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::func::FuncRepr;
    use crate::entities::span::Span;
    let input = SysInputs::default();
    let mut scope = make_stdlib_with_features(&input, Features::empty());
    let Value::Func(element) = raw_line(&scope) else { panic!("raw.line function") };
    let span = Span::from_range(id(43), 10..25);
    let value_span = Span::from_range(id(43), 18..24);
    let args = Args::from_occurrences(
        span,
        vec![
            ArgOccurrence {
                name: None,
                value: Value::Func(element.clone()),
                span,
                value_span,
            },
            ArgOccurrence {
                name: Some("same".into()),
                value: Value::Int(1),
                span,
                value_span,
            },
            ArgOccurrence {
                name: Some("same".into()),
                value: Value::Int(2),
                span: value_span,
                value_span: span,
            },
        ],
    );
    let bound = element.with_diagnostic_span(span).with(args.clone());
    scope.define("bound", Value::Func(bound.clone()));
    scope.define("alias", Value::Func(bound));
    let slot = RefCell::new(Some(Arc::new(StdlibSnapshot::new(
        input.clone(),
        Features::empty(),
        scope,
    ))));
    let (first, _) = resolve(&slot, &input, Features::empty());
    let (second, _) = resolve(&slot, &input, Features::empty());
    for scope in [&first, &second] {
        assert_eq!(scope.get("bound"), scope.get("alias"));
        let Value::Func(bound) = scope.get("bound").unwrap() else {
            panic!("With function")
        };
        assert_eq!(bound.diagnostic_span(), span);
        let FuncRepr::With(pair) = bound.repr() else { panic!("With representation") };
        assert_eq!(Value::Func(pair.0.clone()), raw_line(scope));
        assert_eq!(pair.0.diagnostic_span(), span);
        assert_eq!(pair.1.span, span);
        assert_eq!(pair.1.items, vec![raw_line(scope)]);
        assert_eq!(pair.1.named.get("same"), Some(&Value::Int(2)));
        let actual = pair.1.occurrences.as_ref().expect("causal occurrences");
        assert_eq!(actual.len(), 3);
        for (actual, expected) in actual.iter().zip(args.occurrences.as_ref().unwrap()) {
            assert_eq!(actual.name, expected.name);
            assert_eq!(actual.span, expected.span);
            assert_eq!(actual.value_span, expected.value_span);
        }
        assert_eq!(actual[0].value, raw_line(scope));
        assert_eq!(actual[1].value, Value::Int(1));
        assert_eq!(actual[2].value, Value::Int(2));
    }
    assert_ne!(first.get("bound"), second.get("bound"));
    distinct_files(&first, &second);
}

fn inspect_builtin_content(content: &Content, depth: usize) {
    assert!(depth < 64, "builtin content must be acyclic and bounded");
    match content {
        Content::Text(_) | Content::MathText(_) | Content::HSpace(_) => {}
        Content::Sequence(items) => {
            for item in items.iter() {
                inspect_builtin_content(item, depth + 1);
            }
        }
        Content::MathOp(op) => inspect_builtin_content(&op.text, depth + 1),
        Content::MathClassOverride(elem) => {
            inspect_builtin_content(&elem.body, depth + 1)
        }
        Content::MathStyled(elem) => inspect_builtin_content(&elem.body, depth + 1),
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Symbol(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
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
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::MathIdent(_)
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
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
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
        | Content::Styled(_)
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
        | Content::Dynamic(_) => {
            panic!("content outside qualified immutable math builtin graph")
        }
    }
}

#[test]
fn full_error_diagnostics_and_causal_trace_survive_repeated_fresh_roots() {
    let world = MockWorld::new(
        "#import \"middle.typ\" as middle",
        &[
            ("middle.typ", "#import \"leaf.typ\" as leaf"),
            ("leaf.typ", "#let fail() = panic(\"causal\")\n#fail()"),
        ],
    );
    let mut previous = None;
    for _ in 0..3 {
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            &world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &world.main,
            &ElementRegistry::new(),
            true,
            EvalTarget::Paged,
            Features::empty(),
        );
        assert!(sink.into_diagnostics().is_empty());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].severity, Severity::Error);
        assert_eq!(errors[0].message, "panicked with: causal");
        assert!(errors[0].hints.is_empty());
        assert_eq!(errors[0].span.id(), Some(world.files["leaf.typ"].id()));
        assert_eq!(
            errors[0]
                .trace
                .iter()
                .filter_map(|t| match &t.v {
                    Tracepoint::Import(path) => Some(path.as_str()),
                    Tracepoint::Call(_)
                    | Tracepoint::Show(_)
                    | Tracepoint::Include(_) => None,
                })
                .collect::<Vec<_>>(),
            ["leaf.typ", "middle.typ"]
        );
        assert!(errors[0]
            .trace
            .iter()
            .any(|t| matches!(&t.v, Tracepoint::Call(Some(name)) if name == "fail")));
        if let Some(expected) = &previous {
            assert_eq!(&errors, expected);
        }
        previous = Some(errors);
    }
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 9);
}

// Cardinality controls use the real private instantiator; capacity remains
// private to Scope's own inline tests.
fn known_cardinality_primitive_scope(count: usize) -> Scope {
    let mut scope = Scope::new();
    for index in 0..count {
        scope.define(format!("key-{:05}", count - index), Value::Int(index as i64));
    }
    scope
}

fn known_cardinality_assert_primitives(scope: &Scope, count: usize) {
    assert_eq!(scope.len(), count);
    assert_eq!(scope.is_empty(), count == 0);
    for (index, (name, binding)) in scope.iter().enumerate() {
        assert_eq!(name, format!("key-{:05}", count - index));
        assert_eq!(binding.value(), &Value::Int(index as i64));
    }
}

#[test]
fn known_cardinality_instances_preserve_sizes_order_and_immutable_sources() {
    let mut instance = Instance::default();
    for count in [0usize, 1, 17, 257, 4096, 3, 0] {
        let original = known_cardinality_primitive_scope(count);
        let mut fresh = instance.scope(&original);
        known_cardinality_assert_primitives(&original, count);
        known_cardinality_assert_primitives(&fresh, count);
        if count > 0 {
            let first = format!("key-{count:05}");
            *fresh.get_mut(&first).unwrap() = Value::Bool(false);
            assert_eq!(original.get(&first), Some(&Value::Int(0)));
        }
        fresh.define("fresh-only", Value::None);
        assert!(original.get("fresh-only").is_none());
        known_cardinality_assert_primitives(&original, count);
    }
    // This source's capacity exceeds its actual length. Only binding semantics
    // are asserted here; the reservation expression is reviewed independently.
    let mut sparse = Scope::with_capacity(4096);
    sparse.define("z", Value::Int(3));
    sparse.define("a", Value::Bool(true));
    let fresh = instance.scope(&sparse);
    for scope in [&sparse, &fresh] {
        assert_eq!(scope.len(), 2);
        assert_eq!(scope.iter().map(|(name, _)| name).collect::<Vec<_>>(), ["z", "a"]);
        assert_eq!(scope.get("z"), Some(&Value::Int(3)));
        assert_eq!(scope.get("a"), Some(&Value::Bool(true)));
    }
    assert!(instance.scope(&Scope::new()).is_empty());
}

#[test]
fn known_cardinality_nested_unequal_scopes_keep_aliases_and_fresh_identity() {
    let mut root = Scope::new();
    for count in [0usize, 1, 17, 257] {
        let module = Module::new(format!("module-{count}"),
            known_cardinality_primitive_scope(count));
        root.define(format!("module-{count}"), Value::Module(module.clone()));
        root.define(format!("alias-{count}"), Value::Module(module));
    }
    let outer = Module::new("outer", root);
    let mut template = Scope::new();
    template.define("z", Value::Module(outer.clone()));
    template.define("a", Value::Module(outer.clone()));
    let first = Instance::default().scope(&template);
    let second = Instance::default().scope(&template);
    for fresh in [&first, &second] {
        assert_eq!(fresh.iter().map(|(name, _)| name).collect::<Vec<_>>(), ["z", "a"]);
        assert_eq!(fresh.get("z"), fresh.get("a"));
        assert_ne!(fresh.get("z"), template.get("z"));
        let Value::Module(copied_outer) = fresh.get("z").unwrap() else {
            panic!("outer module");
        };
        assert_eq!(copied_outer.name(), outer.name());
        assert_eq!(copied_outer.scope().iter().map(|(name, _)| name).collect::<Vec<_>>(),
            outer.scope().iter().map(|(name, _)| name).collect::<Vec<_>>());
        for count in [0usize, 1, 17, 257] {
            let name = format!("module-{count}");
            let alias = format!("alias-{count}");
            assert_eq!(copied_outer.scope().get(&name), copied_outer.scope().get(&alias));
            assert_ne!(copied_outer.scope().get(&name), outer.scope().get(&name));
            let Value::Module(child) = copied_outer.scope().get(&name).unwrap() else {
                panic!("nested module");
            };
            assert_eq!(child.name(), name);
            known_cardinality_assert_primitives(child.scope(), count);
            let Value::Module(original_child) = outer.scope().get(&name).unwrap() else {
                panic!("source child");
            };
            known_cardinality_assert_primitives(original_child.scope(), count);
        }
    }
    assert_ne!(first.get("z"), second.get("z"));
    let Value::Module(first_outer) = first.get("z").unwrap() else { panic!("first") };
    let Value::Module(second_outer) = second.get("z").unwrap() else { panic!("second") };
    for count in [0usize, 1, 17, 257] {
        let name = format!("module-{count}");
        assert_ne!(first_outer.scope().get(&name), second_outer.scope().get(&name));
    }
}

#[test]
fn known_cardinality_native_namespaces_and_with_aliases_survive_root_size_changes() {
    use crate::entities::args::Args;
    use crate::entities::func::FuncRepr;
    let registry = make_stdlib_with_features(&SysInputs::default(), Features::empty());
    let Value::Func(raw) = registry.get("raw").unwrap() else { panic!("raw") };
    let Value::Func(line) = raw_line(&registry) else { panic!("raw.line") };
    let bound = line.clone().with(Args::positional(vec![Value::Func(line.clone())]));
    let source_namespace = raw.namespace().expect("qualified raw namespace");
    let mut instance = Instance::default();
    for padding in [257usize, 1, 17, 0] {
        let mut template = known_cardinality_primitive_scope(padding);
        template.define("raw", Value::Func(raw.clone()));
        template.define("bound", Value::Func(bound.clone()));
        template.define("bound-alias", Value::Func(bound.clone()));
        let fresh = instance.scope(&template);
        let separate = Instance::default().scope(&template);
        for scope in [&fresh, &separate] {
            assert_eq!(scope.len(), padding + 3);
            assert_eq!(scope.iter().map(|(name, _)| name).collect::<Vec<_>>(),
                template.iter().map(|(name, _)| name).collect::<Vec<_>>());
            let Value::Func(copied_raw) = scope.get("raw").unwrap() else { panic!("raw") };
            let namespace = copied_raw.namespace().expect("copied raw namespace");
            assert_eq!(namespace.len(), source_namespace.len());
            assert_eq!(namespace.iter().map(|(name, _)| name).collect::<Vec<_>>(),
                source_namespace.iter().map(|(name, _)| name).collect::<Vec<_>>());
            assert_eq!(scope.get("bound"), scope.get("bound-alias"));
            assert_ne!(scope.get("bound"), template.get("bound"));
            assert_ne!(raw_line(scope), Value::Func(line.clone()));
            let Value::Func(copied_bound) = scope.get("bound").unwrap() else {
                panic!("bound");
            };
            let FuncRepr::With(pair) = copied_bound.repr() else { panic!("With") };
            assert_eq!(Value::Func(pair.0.clone()), raw_line(scope));
            assert_eq!(pair.1.items, vec![raw_line(scope)]);
        }
        assert_ne!(fresh.get("bound"), separate.get("bound"));
        assert_ne!(raw_line(&fresh), raw_line(&separate));
        assert_eq!(template.get("bound"), Some(&Value::Func(bound.clone())));
        assert_eq!(raw_line(&template), Value::Func(line.clone()));
    }
}
