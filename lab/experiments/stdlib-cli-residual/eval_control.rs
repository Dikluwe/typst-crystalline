//! Laboratory public-API evaluation control. No product source/cfg/portal edits.
//! Parsing, fixture/input I/O and semantic assertions occur outside the clock.

use std::hint::black_box;
use std::num::NonZeroU16;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use comemo::Track;
use typst_core::compiler::eval::{eval_with_full_error_target_and_features, EvalTarget};
use typst_core::contracts::world::{SysInputs, World};
use typst_core::entities::compiler_features::Features;
use typst_core::entities::element_registry::ElementRegistry;
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::FontBook;
use typst_core::entities::module::Module;
use typst_core::entities::sink::Sink;
use typst_core::entities::source::Source;
use typst_core::entities::value::Value;
use typst_core::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
};

struct ControlWorld {
    library: Library,
    book: FontBook,
    source: Source,
    inputs: SysInputs,
    calls: AtomicUsize,
}

impl World for ControlWorld {
    fn library(&self) -> &Library { &self.library }
    fn book(&self) -> &FontBook { &self.book }
    fn main(&self) -> FileId { self.source.id() }
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() { Ok(self.source.clone()) } else { Err(FileError::NotFound) }
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
    fn font(&self, _: usize) -> Option<Font> { None }
    fn today(&self, _: Option<typst_core::entities::duration::Duration>) -> Option<Datetime> { None }
    fn inputs(&self) -> SysInputs {
        self.calls.fetch_add(1, Ordering::Relaxed);
        self.inputs.clone()
    }
}

fn evaluate(world: &ControlWorld, routines: &Routines, registry: &ElementRegistry) -> Module {
    let mut sink = Sink::new();
    let result = eval_with_full_error_target_and_features(
        routines, world, Traced::default().track(), sink.track_mut(), Route::root().track(),
        &world.source, registry, false, EvalTarget::Paged, Features::empty(),
    );
    // These failure checks are identical in each ELF. They do not inspect or
    // format diagnostics on success; a failure is never a timing observation.
    assert!(sink.into_diagnostics().is_empty(), "unexpected evaluation diagnostic");
    result.unwrap_or_else(|errors| panic!("evaluation failed: {errors:?}"))
}

fn validate(module: &Module, inputs: &SysInputs) {
    assert_eq!(module.scope().get("qualification_ok"), Some(&Value::Bool(true)));
    let observed = match module.scope().get("qualification_inputs") {
        Some(Value::Dict(values)) => values,
        other => panic!("missing input observation: {other:?}"),
    };
    assert_eq!(observed.len(), inputs.len());
    let actual = observed.iter().map(|(k, v)| {
        let text = match v { Value::Str(text) => text, other => panic!("non-string input: {other:?}") };
        (k.as_str(), text.as_str())
    }).collect::<Vec<_>>();
    let expected = inputs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<_>>();
    assert_eq!(actual, expected, "ordered input pairs differ");
}

fn options() -> (PathBuf, PathBuf, usize, usize, usize) {
    let mut source = None;
    let mut inputs = None;
    let mut iterations = 100usize;
    let mut samples = 10usize;
    let mut warmup = 10usize;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let value = args.next().expect("option requires a value");
        match arg.as_str() {
            "--source" => source = Some(PathBuf::from(value)),
            "--input-file" => inputs = Some(PathBuf::from(value)),
            "--iterations" => iterations = value.parse().expect("invalid iterations"),
            "--samples" => samples = value.parse().expect("invalid samples"),
            "--warmup" => warmup = value.parse().expect("invalid warmup"),
            _ => panic!("unknown option: {arg}"),
        }
    }
    assert!((1..=100_000).contains(&iterations) && (1..=1000).contains(&samples));
    assert!(warmup <= 100_000);
    (source.expect("--source required"), inputs.expect("--input-file required"), iterations, samples, warmup)
}

fn main() {
    let (source, inputs, iterations, samples, warmup) = options();
    let text = std::fs::read_to_string(source).expect("read source");
    // The core contract owns de-duplication/insertion order. Match CLI's
    // ordered string-pair input, without sorting, truncation or coercion.
    let pairs: Vec<(String, String)> = serde_json::from_str(
        &std::fs::read_to_string(inputs).expect("read input fixture"),
    ).expect("input fixture must be an array of string pairs");
    let mut input_map = SysInputs::default();
    for (key, value) in pairs { input_map.insert(key.into(), value.into()); }
    let world = ControlWorld {
        library: Library::new(), book: FontBook::new(),
        source: Source::new(FileId::from_raw(NonZeroU16::new(1).unwrap()), text).expect("source parse"),
        inputs: input_map, calls: AtomicUsize::new(0),
    };
    let routines = Routines::new();
    let registry = ElementRegistry::new();
    validate(&evaluate(&world, &routines, &registry), &world.inputs);
    for _ in 0..warmup { black_box(evaluate(&world, &routines, &registry)); }
    println!("sample,iterations,input_pairs,elapsed_ns,world_input_calls");
    for sample in 0..samples {
        let before = world.calls.load(Ordering::Relaxed);
        let started = Instant::now();
        for _ in 0..iterations { black_box(evaluate(&world, &routines, &registry)); }
        let elapsed = started.elapsed().as_nanos();
        let calls = world.calls.load(Ordering::Relaxed) - before;
        assert_eq!(calls, iterations, "unexpected imports or skipped World.inputs");
        println!("{sample},{iterations},{},{elapsed},{calls}", world.inputs.len());
    }
    validate(&evaluate(&world, &routines, &registry), &world.inputs);
}
