//! Public-API controls for compilation-local stdlib reuse.
//! Run unchanged against baseline and candidate. Diagnostic-only tests require
//! --cfg stdlib_reuse_candidate and the explicitly specified laboratory portal.

use std::collections::HashMap;
use std::num::NonZeroU16;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use comemo::Track;
use ecow::EcoString;
use typst_core::compiler::eval::{
    eval_expression_with_features, eval_source_with_features_and_introspector,
    eval_with_full_error_target_and_features, EvalTarget,
};
use typst_core::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
use typst_core::contracts::world::{SysInputs, World};
use typst_core::entities::bytes::Bytes as PluginBytes;
use typst_core::entities::compiler_features::{Feature, Features};
use typst_core::entities::content::Content;
use typst_core::entities::element_registry::ElementRegistry;
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::FontBook;
use typst_core::entities::introspector::TagIntrospector;
use typst_core::entities::module::Module;
use typst_core::entities::package_spec::PackageSpec;
use typst_core::entities::path::{RootedPath, VirtualPath, VirtualRoot};
use typst_core::entities::sink::Sink;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::{Severity, SourceDiagnostic, SourceResult, Tracepoint};
use typst_core::entities::value::Value;
use typst_core::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced};

fn inputs(pairs: &[(&str, &str)]) -> SysInputs {
    let mut out = SysInputs::default();
    for (key, value) in pairs { out.insert((*key).into(), (*value).into()); }
    out
}
fn input_pairs(input: &SysInputs) -> Vec<(String, String)> {
    input.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}
fn flags(mask: u8) -> Features {
    let mut out = Features::empty();
    for (bit, flag) in [(1, Feature::Html), (2, Feature::A11yExtras), (4, Feature::Bundle)] {
        if mask & bit != 0 { out.enable(flag); }
    }
    out
}
fn id(raw: u16) -> FileId { FileId::from_raw(NonZeroU16::new(raw).unwrap()) }

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
    host: Option<Arc<dyn PluginHost>>,
    record_observations: bool,
}
impl MockWorld {
    fn new(main: &str, files: &[(&str, &str)]) -> Self {
        let files = files.iter().enumerate().map(|(i, (path, text))| {
            ((*path).to_owned(), Source::new(id(i as u16 + 2), (*text).to_owned()).unwrap())
        }).collect();
        Self { library: Library::new(), book: FontBook::new(), main: Source::new(id(1), main.to_owned()).unwrap(), files,
            input_script: vec![SysInputs::default()], input_calls: AtomicUsize::new(0), observations: Mutex::new(Vec::new()),
            resolutions: Mutex::new(Vec::new()), packages: Mutex::new(Vec::new()), host: None, record_observations: true }
    }
    fn scripted(mut self, script: Vec<SysInputs>) -> Self { assert!(!script.is_empty()); self.input_script = script; self }
    fn replace(&mut self, path: &str, text: &str) {
        if path == "main.typ" { self.main = Source::new(self.main.id(), text.to_owned()).unwrap(); }
        else { let old = self.files[path].id(); self.files.insert(path.to_owned(), Source::new(old, text.to_owned()).unwrap()); }
    }
}
impl World for MockWorld {
    fn library(&self) -> &Library { &self.library }
    fn book(&self) -> &FontBook { &self.book }
    fn main(&self) -> FileId { self.main.id() }
    fn source(&self, file: FileId) -> FileResult<Source> {
        if file == self.main.id() { return Ok(self.main.clone()); }
        self.files.values().find(|s| s.id() == file).cloned().ok_or(FileError::NotFound)
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
    fn font(&self, _: usize) -> Option<Font> { None }
    fn today(&self, _: Option<typst_core::entities::duration::Duration>) -> Option<Datetime> { None }
    fn include_source(&self, current: FileId, path: &str) -> Result<Source, String> {
        if self.record_observations { self.resolutions.lock().unwrap().push((current, path.to_owned())); }
        if path == "main.typ" { return Ok(self.main.clone()); }
        self.files.get(path).cloned().ok_or_else(|| format!("fixture missing: {path}"))
    }
    fn resolve_path(&self, _: FileId, path: &str) -> Result<RootedPath, String> {
        Ok(RootedPath::new(VirtualRoot::Project, VirtualPath::new(path).map_err(|e| format!("{e:?}"))?))
    }
    fn include_path(&self, path: &RootedPath) -> Result<Source, String> {
        self.include_source(self.main.id(), path.vpath().get_with_slash().trim_start_matches('/'))
    }
    fn read_path(&self, _: &RootedPath) -> Result<Arc<Vec<u8>>, String> { Ok(Arc::new(vec![0])) }
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String> {
        self.packages.lock().unwrap().push(spec.to_string());
        self.files.get("package.typ").cloned().ok_or_else(|| "fixture package missing".into())
    }
    fn inputs(&self) -> SysInputs {
        let number = self.input_calls.fetch_add(1, Ordering::SeqCst);
        let input = self.input_script[number.min(self.input_script.len() - 1)].clone();
        if self.record_observations { self.observations.lock().unwrap().push(input_pairs(&input)); }
        input
    }
    fn plugin_host(&self) -> Option<Arc<dyn PluginHost>> { self.host.clone() }
}
fn evaluate(world: &MockWorld, features: Features, target: EvalTarget) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
    evaluate_registry(world, features, target, &ElementRegistry::new())
}
fn evaluate_registry(world: &MockWorld, features: Features, target: EvalTarget, registry: &ElementRegistry) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
    let mut sink = Sink::new();
    let result = eval_with_full_error_target_and_features(&Routines::new(), world, Traced::default().track(), sink.track_mut(), Route::root().track(), &world.main, registry, false, target, features);
    (result, sink.into_diagnostics())
}
fn successful(world: &MockWorld) -> Module {
    let (result, warnings) = evaluate(world, Features::empty(), EvalTarget::Paged);
    assert!(warnings.is_empty(), "unexpected warnings: {warnings:?}");
    result.unwrap_or_else(|errors| panic!("unexpected errors: {errors:?}"))
}
fn value(module: &Module, name: &str) -> Value { module.scope().get(name).unwrap_or_else(|| panic!("missing {name}")).clone() }
fn strings(items: &[&str]) -> Value { Value::Array(items.iter().map(|s| Value::Str((*s).into())).collect()) }
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
    assert_eq!(value(&successful(&world), "out"), Value::Array(vec![Value::Bool(false); 3]));
    assert_eq!(world.resolutions.lock().unwrap().iter().map(|(_, p)| p.as_str()).collect::<Vec<_>>(), ["a.typ", "a.typ"]);
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
}

#[test]
fn nested_and_package_imports_keep_inputs_and_namespace_behavior() {
    let world = MockWorld::new("#import \"middle.typ\" as middle\n#import \"@preview/probe:0.1.0\" as pkg\n#let out = (middle.answer, pkg.answer, middle.calcmod == calc, pkg.calcmod == calc)\n", &[("middle.typ", "#import \"leaf.typ\" as leaf\n#let answer = leaf.answer + 1\n#let calcmod = calc"), ("leaf.typ", "#let answer = int(sys.inputs.n)\n"), ("package.typ", "#let answer = int(sys.inputs.n)\n#let calcmod = calc")]).scripted(vec![inputs(&[("n", "41")])]);
    assert_eq!(value(&successful(&world), "out"), Value::Array(vec![Value::Int(42), Value::Int(41), Value::Bool(false), Value::Bool(false)]));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 4);
    assert_eq!(*world.packages.lock().unwrap(), ["@preview/probe:0.1.0"]);
    assert_eq!(world.resolutions.lock().unwrap()[1].0, world.files["middle.typ"].id());
}

#[test]
fn inputs_changed_or_reordered_per_request_are_not_replaced_by_the_root_snapshot() {
    let script = vec![inputs(&[("a", "A"), ("b", "B")]), inputs(&[("b", "B"), ("a", "A")]), inputs(&[("a", "changed"), ("c", "C")])];
    let world = MockWorld::new("#let rootkeys = sys.inputs.keys()\n#import \"child.typ\" as first\n#import \"child.typ\" as second\n#let out = (rootkeys, first.keys, first.values, second.keys, second.values)\n", &[("child.typ", "#let keys = sys.inputs.keys()\n#let values = sys.inputs.values()")]).scripted(script.clone());
    assert_eq!(value(&successful(&world), "out"), Value::Array(vec![strings(&["a", "b"]), strings(&["b", "a"]), strings(&["B", "A"]), strings(&["a", "c"]), strings(&["changed", "C"])]));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
    assert_eq!(*world.observations.lock().unwrap(), script.iter().map(input_pairs).collect::<Vec<_>>());
}

#[test]
fn separate_eval_entries_and_worlds_do_not_reuse_old_inputs() {
    for reverse in [false, true] {
        let values = if reverse { ["B", "A", "B"] } else { ["A", "B", "A"] };
        let world = MockWorld::new("#let out = sys.inputs.tag", &[]).scripted(values.iter().map(|tag| inputs(&[("tag", tag)])).collect());
        for tag in values { assert_eq!(value(&successful(&world), "out"), Value::Str(tag.into())); }
        assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
    }
    for tag in ["world-A", "world-B", "world-A"] {
        let world = MockWorld::new("#let out = sys.inputs.tag", &[]).scripted(vec![inputs(&[("tag", tag)])]);
        assert_eq!(value(&successful(&world), "out"), Value::Str(tag.into()));
    }
}

#[test]
fn watch_like_re_evaluation_observes_main_and_import_edits() {
    let mut world = MockWorld::new("#import \"child.typ\" as child\n#let out = child.answer + 1", &[("child.typ", "#let answer = 10")]);
    assert_eq!(value(&successful(&world), "out"), Value::Int(11));
    assert_eq!(value(&successful(&world), "out"), Value::Int(11));
    world.replace("child.typ", "#let answer = 40");
    assert_eq!(value(&successful(&world), "out"), Value::Int(41));
    world.replace("main.typ", "#import \"child.typ\" as child\n#let out = child.answer + 2");
    assert_eq!(value(&successful(&world), "out"), Value::Int(42));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 8);
}

#[test]
fn shadowing_and_imported_show_rules_do_not_contaminate_the_caller() {
    let world = MockWorld::new("#import \"child.typ\" as child\n#let out = (upper(\"abc\"), child.saved)\nx", &[("child.typ", "#let upper(x) = \"shadow\"\n#let std = \"shadow\"\n#let saved = upper(\"abc\")\n#show \"x\": [BAD]\n")]);
    let module = successful(&world);
    assert_eq!(value(&module, "out"), Value::Array(vec![Value::Str("ABC".into()), Value::Str("shadow".into())]));
    assert_eq!(module.content().unwrap().plain_text().trim(), "x");
    let world = MockWorld::new("#let std = (html: 1, asset: 2)\n#let format = (html: 3, bundle: 4)\n#let out = (std.html, std.asset, format.html, format.bundle)", &[]);
    assert_eq!(value(&successful(&world), "out"), Value::Array((1..=4).map(Value::Int).collect()));
}

#[test]
fn include_keeps_its_existing_caller_scope_semantics() {
    let world = MockWorld::new("#let marker = 2\n#include \"included.typ\"\n#let out = (marker, newly)", &[("included.typ", "#let marker = 3\n#let newly = upper(\"ok\")")]);
    assert_eq!(value(&successful(&world), "out"), Value::Array(vec![Value::Int(3), Value::Str("OK".into())]));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 1, "include does not construct an imported stdlib");
}

#[test]
fn registry_bindings_do_not_leak_into_imports_or_other_evaluations() {
    let mut registry = ElementRegistry::new();
    registry.register("customprobe", Arc::new(|_| Ok(Content::text("registered"))));
    let root = MockWorld::new("#let out = customprobe", &[]);
    let (result, warnings) = evaluate_registry(&root, Features::empty(), EvalTarget::Paged, &registry);
    assert!(warnings.is_empty());
    assert!(matches!(value(&result.unwrap(), "out"), Value::Func(_)));
    let imported = MockWorld::new("#import \"child.typ\" as child", &[("child.typ", "#let out = customprobe")]);
    let (result, _) = evaluate_registry(&imported, Features::empty(), EvalTarget::Paged, &registry);
    assert!(result.unwrap_err()[0].message.contains("customprobe"));
    let (result, _) = evaluate(&root, Features::empty(), EvalTarget::Paged);
    assert!(result.unwrap_err()[0].message.contains("customprobe"));
}

fn feature_case(mask: u8, target: EvalTarget, access: &str, field: &str, feature: Feature, label: &str) {
    let child = format!("#let out = {access}\n");
    let world = MockWorld::new("#import \"child.typ\" as child\n#let out = child.out", &[("child.typ", &child)]);
    let (result, warnings) = evaluate(&world, flags(mask), target);
    assert!(warnings.is_empty());
    if flags(mask).contains(feature) { assert!(matches!(value(&result.unwrap(), "out"), Value::Func(_)), "mask {mask}/{access}"); }
    else {
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        let error = &errors[0];
        assert_eq!(error.message, format!("cannot access field `{field}` because the `{label}` feature is not enabled"));
        assert_eq!(error.hints, [format!("try enabling the `{label}` feature"), "see https://typst.app/help/compiler-features for more details".to_owned()]);
        assert_eq!(error.span.id(), Some(world.files["child.typ"].id()));
        assert_eq!(error.trace.iter().filter_map(|t| if let Tracepoint::Import(path) = &t.v { Some(path.as_str()) } else { None }).collect::<Vec<_>>(), ["child.typ"]);
    }
}
#[test]
fn features_targets_and_builtin_markers_remain_independent_across_roots() {
    for mask in [0, 7, 1, 0, 4, 2, 6, 3, 5, 0] {
        for target in [EvalTarget::Paged, EvalTarget::Html, EvalTarget::Bundle] {
            feature_case(mask, target, "std.html", "html", Feature::Html, "html");
            feature_case(mask, target, "std.format.html", "html", Feature::Html, "html");
            feature_case(mask, target, "std.asset", "asset", Feature::Bundle, "bundle");
            feature_case(mask, target, "std.format.bundle", "bundle", Feature::Bundle, "bundle");
        }
    }
}

#[test]
fn disabled_builtin_gates_precede_argument_effects_in_imports() {
    for access in ["std.html", "std.format.html", "std.asset", "std.format.bundle"] {
        let child = format!("#let out = {access}(panic(\"ARGUMENT RAN\"))\n");
        let world = MockWorld::new("#import \"child.typ\" as child", &[("child.typ", &child)]);
        let (result, _) = evaluate(&world, Features::empty(), EvalTarget::Paged);
        let errors = result.unwrap_err();
        assert!(!errors[0].message.contains("ARGUMENT RAN"), "{access}");
        assert!(errors[0].message.contains("feature is not enabled"), "{access}: {errors:?}");
        let (result, _) = evaluate(&world, flags(7), EvalTarget::Paged);
        assert_eq!(result.unwrap_err()[0].message, "panicked with: ARGUMENT RAN");
    }
}

#[test]
fn syntax_failure_and_nested_trace_preserve_the_original_source() {
    let world = MockWorld::new("#import \"middle.typ\" as middle", &[("middle.typ", "#import \"bad.typ\" as bad"), ("bad.typ", "#panic(\"prefix executed\")\n#let or(x)=x")]);
    let bad = &world.files["bad.typ"];
    let expected = bad.root().errors();
    assert!(!expected.is_empty());
    let (result, _) = evaluate(&world, Features::empty(), EvalTarget::Paged);
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), expected.len());
    for (error, source_error) in errors.iter().zip(expected) {
        assert_eq!(error.severity, Severity::Error);
        assert_eq!(error.message, source_error.message.as_str());
        assert_eq!(error.hints, source_error.hints.iter().map(ToString::to_string).collect::<Vec<_>>());
        assert_eq!(error.span, source_error.span);
        assert_eq!(error.trace.iter().filter_map(|t| if let Tracepoint::Import(path) = &t.v { Some(path.as_str()) } else { None }).collect::<Vec<_>>(), ["bad.typ", "middle.typ"]);
    }
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 2, "invalid child fails before stdlib acquisition");
}

#[test]
fn cyclic_import_remains_an_error_instead_of_becoming_a_cached_module() {
    let world = MockWorld::new("#import \"child.typ\" as child", &[("child.typ", "#import \"main.typ\" as parent")]);
    let (result, _) = evaluate(&world, Features::empty(), EvalTarget::Paged);
    assert!(result.unwrap_err()[0].message.contains("ciclo de importação detectado"));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 2);
}

#[test]
fn expression_and_source_facades_remain_compilation_local() {
    let world = MockWorld::new("", &[]).scripted(vec![inputs(&[("tag", "first")]), inputs(&[("tag", "second")]), inputs(&[("tag", "third")])]);
    let (first, warnings) = eval_expression_with_features(&world, "sys.inputs.tag", Features::empty());
    assert!(warnings.is_empty());
    assert_eq!(first.unwrap(), Value::Str("first".into()));
    let source = Source::new_with_parser(id(1), "sys.inputs.tag".to_owned(), typst_core::compiler::parse::parse_code).unwrap();
    let (second, warnings) = eval_source_with_features_and_introspector(&world, &source, Features::empty(), &TagIntrospector::empty());
    assert!(warnings.is_empty());
    assert_eq!(second.unwrap(), Value::Str("second".into()));
    let (third, warnings) = eval_expression_with_features(&world, "sys.inputs.tag", Features::empty());
    assert!(warnings.is_empty());
    assert_eq!(third.unwrap(), Value::Str("third".into()));
    assert_eq!(world.input_calls.load(Ordering::SeqCst), 3);
}

struct TaggedHost { tag: &'static str, loads: AtomicUsize }
impl PluginHost for TaggedHost {
    fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> { self.loads.fetch_add(1, Ordering::SeqCst); Ok(PluginModuleId(1)) }
    fn exports(&self, _: PluginModuleId) -> Result<Vec<EcoString>, PluginError> { Ok(vec!["run".into()]) }
    fn call(&self, _: PluginModuleId, _: &str, _: &[PluginBytes]) -> Result<PluginBytes, PluginError> { Ok(PluginBytes::new(self.tag.as_bytes().to_vec())) }
    fn transition(&self, _: PluginModuleId, _: &str, _: &[PluginBytes]) -> Result<PluginModuleId, PluginError> { Ok(PluginModuleId(2)) }
}
#[test]
fn reusable_builtins_do_not_capture_the_previous_world_plugin_host() {
    for tag in ["host A", "host B", "host A"] {
        let host = Arc::new(TaggedHost { tag, loads: AtomicUsize::new(0) });
        let mut world = MockWorld::new("#let out = plugin(\"probe.wasm\").run()", &[]);
        world.host = Some(host.clone());
        assert_eq!(value(&successful(&world), "out"), Value::Bytes(PluginBytes::new(tag.as_bytes().to_vec())));
        assert_eq!(host.loads.load(Ordering::SeqCst), 1);
    }
}

#[cfg(stdlib_reuse_candidate)]
#[test]
fn diagnostic_helper_checks_all_feature_transitions_in_both_directions() {
    use typst_core::compiler::eval::__lab_resolve_stdlib;
    for seed in 0..8 {
        for requested in 0..8 {
            let result = __lab_resolve_stdlib(inputs(&[("a", "A")]), flags(seed), inputs(&[("a", "A")]), flags(requested));
            assert_eq!(result.get("asset").is_some(), flags(requested).contains(Feature::Bundle), "seed={seed} requested={requested}");
            assert!(result.get("html").is_some());
        }
    }
}
#[cfg(stdlib_reuse_candidate)]
#[test]
fn diagnostic_helper_checks_input_order_and_values_without_hash_only_equality() {
    use typst_core::compiler::eval::__lab_resolve_stdlib;
    let seed = inputs(&[("a", "A"), ("b", "B")]);
    for requested in [SysInputs::default(), inputs(&[("b", "B"), ("a", "A")]), inputs(&[("a", "changed"), ("b", "B")]), inputs(&[("a", "A"), ("c", "C")])] {
        let result = __lab_resolve_stdlib(seed.clone(), Features::empty(), requested.clone(), Features::empty());
        let Value::Module(sys) = result.get("sys").unwrap() else { panic!("sys module"); };
        let Value::Dict(actual) = sys.scope().get("inputs").unwrap() else { panic!("sys.inputs dict"); };
        assert_eq!(actual.iter().map(|(k, v)| { let Value::Str(v) = v else { panic!("input string"); }; (k.to_string(), v.to_string()) }).collect::<Vec<_>>(), input_pairs(&requested));
    }
}


#[test]
fn empty_inputs_and_ordinary_homonymous_modules_remain_distinct() {
    for script in [vec![SysInputs::default(), inputs(&[("tag", "new")])], vec![inputs(&[("tag", "old")]), SysInputs::default()]] {
        let expected = strings(&script[1].keys().map(|k| k.as_str()).collect::<Vec<_>>());
        let world = MockWorld::new("#import \"child.typ\" as child\n#let out = child.keys", &[("child.typ", "#let keys = sys.inputs.keys()")]).scripted(script);
        assert_eq!(value(&successful(&world), "out"), expected);
    }
    let world = MockWorld::new("#import \"format.typ\" as ordinary_format\n#import \"global.typ\" as ordinary_global\n#let out = (ordinary_format.html, ordinary_format.bundle, ordinary_global.html, ordinary_global.asset)", &[("format.typ", "#let html = 1\n#let bundle = 2"), ("global.typ", "#let html = 3\n#let asset = 4")]);
    assert_eq!(value(&successful(&world), "out"), Value::Array((1..=4).map(Value::Int).collect()));
}

fn cost_world(count: usize, shape: &str) -> MockWorld {
    let mut files: Vec<(String, String)> = Vec::new();
    let mut main = String::new();
    if shape == "nested" && count > 0 {
        main.push_str("#import \"m00.typ\" as first\n#let out = first.answer\n");
        for i in 0..count {
            let code = if i + 1 == count { "#let answer = 1\n".to_owned() }
                else { format!("#import \"m{:02}.typ\" as next\n#let answer = next.answer + 1\n", i + 1) };
            files.push((format!("m{i:02}.typ"), code));
        }
    } else {
        let mut terms = vec!["0".to_owned()];
        for i in 0..count {
            let file = if shape == "repeated" { "shared.typ".to_owned() } else { format!("m{i:02}.typ") };
            main.push_str(&format!("#import \"{file}\" as m{i:02}\n"));
            terms.push(format!("m{i:02}.answer"));
            if shape != "repeated" || i == 0 { files.push((file, "#let answer = 1\n".to_owned())); }
        }
        main.push_str(&format!("#let out = {}\n", terms.join(" + ")));
    }
    let refs: Vec<_> = files.iter().map(|(path, code)| (path.as_str(), code.as_str())).collect();
    let mut world = MockWorld::new(&main, &refs);
    world.record_observations = false;
    world
}

#[test]
#[ignore = "isolated release timing; use the same already-parsed World on both sides, no allocation counter"]
fn measure_eval_with_preparsed_sources() {
    use std::hint::black_box;
    use std::time::Instant;
    let iterations = std::env::var("TEKT_STDLIB_ITERATIONS").ok().map(|x| x.parse::<usize>().unwrap()).unwrap_or(4);
    assert!((1..=10000).contains(&iterations));
    for (count, shape) in [(0, "flat"), (1, "flat"), (4, "flat"), (8, "flat"), (16, "flat"), (64, "flat"), (16, "nested"), (64, "nested"), (16, "repeated"), (64, "repeated")] {
        let world = cost_world(count, shape);
        let started = Instant::now();
        for _ in 0..iterations {
            let module = black_box(successful(black_box(&world)));
            assert_eq!(value(&module, "out"), Value::Int(count as i64));
            drop(module);
        }
        let elapsed = started.elapsed().as_nanos();
        assert_eq!(world.input_calls.load(Ordering::SeqCst), iterations * (count + 1));
        println!("operation=eval_preparsed shape={shape} imports={count} iterations={iterations} elapsed_ns={elapsed} expected_out={count} world_log=false sources_preparsed=true full_result_drop=true");
    }
}

#[test]
fn element_functions_in_builtin_namespaces_keep_per_file_language_identity() {
    // raw is native-by-name, but raw.line is an Element function whose
    // identity is its individual Func Arc. Sharing one finished namespace
    // across files would change these language-level == results.
    let exports = "#let rawline = raw.line\n#let stdline = std.raw.line\n#let alias = rawline\n";
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
