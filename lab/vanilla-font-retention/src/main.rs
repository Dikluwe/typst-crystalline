//! Quarantined public-API experiment: persistent fonts, freshly invalidated files.
use sha2::{Digest, Sha256};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Instant,
};
use typst_core::{
    contracts::{
        plugin_host::PluginHost,
        world::{SysInputs, World},
    },
    entities::{
        duration::Duration,
        file_id::FileId,
        font_book::FontBook,
        package_spec::PackageSpec,
        path::RootedPath,
        source::Source,
        world_types::{Bytes, Datetime, FileResult, Font, Library},
    },
};
use typst_infra::{
    export::{PdfTags, StreamMode},
    pipeline::compile_to_pdf_bytes_with_timings,
    world::SystemWorld,
};

// File and library state belongs to this cycle. Only font operations delegate to
// the session's donor; neither component's production implementation is copied.
struct SplitWorld<'a> {
    files: SystemWorld,
    fonts: &'a SystemWorld,
    book_calls: AtomicUsize,
    first_book_ns: Mutex<Option<u128>>,
    book_pointer: AtomicUsize,
    font_indices: Mutex<Vec<usize>>,
}
impl World for SplitWorld<'_> {
    fn library(&self) -> &Library {
        self.files.library()
    }
    fn main(&self) -> FileId {
        self.files.main()
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        self.files.source(id)
    }
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.files.file(id)
    }
    fn book(&self) -> &FontBook {
        let first = self.book_calls.fetch_add(1, Ordering::Relaxed) == 0;
        if first {
            let start = Instant::now();
            let book = self.fonts.book();
            *self.first_book_ns.lock().unwrap() = Some(start.elapsed().as_nanos());
            self.book_pointer
                .store(book as *const FontBook as usize, Ordering::Relaxed);
            book
        } else {
            self.fonts.book()
        }
    }
    fn font(&self, i: usize) -> Option<Font> {
        self.font_indices.lock().unwrap().push(i);
        self.fonts.font(i)
    }
    fn candidates_for_char(&self, c: char) -> Vec<usize> {
        self.fonts.candidates_for_char(c)
    }
    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        self.files.today(offset)
    }
    fn resolve_path(&self, id: FileId, p: &str) -> Result<RootedPath, String> {
        self.files.resolve_path(id, p)
    }
    fn read_path(&self, p: &RootedPath) -> Result<Arc<Vec<u8>>, String> {
        self.files.read_path(p)
    }
    fn include_path(&self, p: &RootedPath) -> Result<Source, String> {
        self.files.include_path(p)
    }
    fn read_bytes(&self, id: FileId, p: &str) -> Result<Arc<Vec<u8>>, String> {
        self.files.read_bytes(id, p)
    }
    fn resolve_package(&self, p: &PackageSpec) -> Result<Source, String> {
        self.files.resolve_package(p)
    }
    fn inputs(&self) -> SysInputs {
        self.files.inputs()
    }
    fn plugin_host(&self) -> Option<Arc<dyn PluginHost>> {
        self.files.plugin_host()
    }
}
fn donor(root: &PathBuf) -> SystemWorld {
    SystemWorld::for_eval(root).with_fonts_and_system(&[])
}
fn main() {
    assert!(!cfg!(debug_assertions), "release measurements required");
    let args: Vec<_> = std::env::args().collect();
    let root = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);
    let policy = args[3].as_str();
    assert!(matches!(policy, "fresh" | "retained" | "prewarmed" | "direct"));
    std::fs::create_dir_all(&output).unwrap();
    let session_start = Instant::now();
    let store_start = Instant::now();
    let mut retained = (policy != "fresh").then(|| donor(&root));
    let store_setup_ms = store_start.elapsed().as_secs_f64() * 1000.0;
    let preload_start = Instant::now();
    if policy == "prewarmed" {
        std::hint::black_box(retained.as_ref().unwrap().book());
    }
    let preload_ms = preload_start.elapsed().as_secs_f64() * 1000.0;
    let mut cycles = Vec::new();
    let mut previous = String::new();
    let mut catalog_signature = None;
    for revision in 0..6 {
        let cycle_start = Instant::now();
        let current = root.join(format!("edit{revision}.typ"));
        let fresh = (policy == "fresh").then(|| donor(&root));
        let fonts = fresh.as_ref().or(retained.as_ref()).unwrap();
        let files = SystemWorld::new(&root, current.file_name().unwrap()).unwrap();
        let files =
            if policy == "direct" { files.with_fonts_and_system(&[]) } else { files };
        let source = files.source(files.main()).unwrap();
        let source_hash = format!("{:x}", Sha256::digest(source.text().as_bytes()));
        assert_ne!(source_hash, previous, "each cycle must observe its changed source");
        previous = source_hash.clone();
        let w = SplitWorld {
            files,
            fonts,
            book_calls: AtomicUsize::new(0),
            first_book_ns: Mutex::new(None),
            book_pointer: AtomicUsize::new(0),
            font_indices: Mutex::new(Vec::new()),
        };
        let setup_ms = cycle_start.elapsed().as_secs_f64() * 1000.0;
        let start = Instant::now();
        let (result, warnings, timings) = compile_to_pdf_bytes_with_timings(
            if policy == "direct" { &w.files as &dyn World } else { &w as &dyn World },
            &source,
            StreamMode::Verbose,
            PdfTags::Disabled,
        );
        let pipeline_ms = start.elapsed().as_secs_f64() * 1000.0;
        let data = result.expect("all revisions compile");
        let warning_count = warnings.len();
        let first_book_ms = w.first_book_ns.lock().unwrap().map(|v| v as f64 / 1e6);
        let book_calls = w.book_calls.load(Ordering::Relaxed);
        let pointer = w.book_pointer.load(Ordering::Relaxed);
        let mut indices = w.font_indices.lock().unwrap().clone();
        indices.sort_unstable();
        indices.dedup();
        // Data output and checks happen outside cycle timing. Record teardown
        // separately below; cycle cost includes setup+pipeline+teardown only.
        let timed_before_controls_ms = cycle_start.elapsed().as_secs_f64() * 1000.0;
        let control_start = Instant::now();
        let control_fonts = if policy == "direct" { &w.files } else { fonts };
        let book = control_fonts.book();
        let catalog_faces = book.len();
        let signature =
            format!("{:x}", Sha256::digest(format!("{:?}", book.infos()).as_bytes()));
        if let Some(ref previous) = catalog_signature {
            assert_eq!(previous, &signature);
        }
        catalog_signature = Some(signature.clone());
        let faces: Vec<_> = indices.iter().map(|&index| {
            let f = control_fonts.font(index).expect("used font handle");
            serde_json::json!({"index":index,"sha256":format!("{:x}",Sha256::digest(f.as_slice()))})
        }).collect();
        std::fs::write(output.join(format!("edit{revision}.pdf")), &data).unwrap();
        let pdf_hash = format!("{:x}", Sha256::digest(&data));
        let controls_ms = control_start.elapsed().as_secs_f64() * 1000.0;
        let teardown_start = Instant::now();
        drop(data);
        drop(source);
        drop(w);
        drop(warnings);
        drop(fresh);
        let teardown_ms = teardown_start.elapsed().as_secs_f64() * 1000.0;
        cycles.push(serde_json::json!({"revision":revision,"source_sha256":source_hash,
            "warning_count":warning_count,"setup_ms":setup_ms,"pipeline_ms":pipeline_ms,"teardown_ms":teardown_ms,
            "cycle_ms":timed_before_controls_ms+teardown_ms,
            "eval_ms":timings.eval_ms,"layout_ms":timings.layout_ms,
            "render_ms":timings.render_ms,"book_calls":book_calls,
            "first_book_ms":first_book_ms,"book_pointer":pointer,
            "catalog_sha256":signature,"catalog_faces":catalog_faces,
            "used_faces":faces,"pdf_sha256":pdf_hash,"controls_ms":controls_ms}));
    }
    let teardown_start = Instant::now();
    drop(retained.take());
    let retained_teardown_ms = teardown_start.elapsed().as_secs_f64() * 1000.0;
    let session_wall_ms = session_start.elapsed().as_secs_f64() * 1000.0;
    let compiled_session_ms = store_setup_ms
        + preload_ms
        + retained_teardown_ms
        + cycles
            .iter()
            .map(|row| row["cycle_ms"].as_f64().unwrap())
            .sum::<f64>();
    println!(
        "{}",
        serde_json::json!({"policy":policy,"store_setup_ms":store_setup_ms,
        "preload_ms":preload_ms,"retained_teardown_ms":retained_teardown_ms,
        "compiled_session_ms":compiled_session_ms,"session_wall_ms":session_wall_ms,
        "catalog_sha256":catalog_signature,"cycles":cycles})
    );
}
