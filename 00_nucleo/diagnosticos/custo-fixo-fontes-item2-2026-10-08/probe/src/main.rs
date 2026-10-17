//! Diagnostic only: measure requested live heap and resident pages by font stage.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};
use std::hint::black_box;
use typst_core::entities::font_book::{Coverage, FontBook};

struct Counting;
static LIVE: AtomicU64 = AtomicU64::new(0);
static PEAK: AtomicU64 = AtomicU64::new(0);
fn add(n: usize) {
    let live = LIVE.fetch_add(n as u64, Ordering::Relaxed) + n as u64;
    PEAK.fetch_max(live, Ordering::Relaxed);
}
fn sub(n: usize) { LIVE.fetch_sub(n as u64, Ordering::Relaxed); }
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = System.alloc(layout);
        if !p.is_null() { add(layout.size()); }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        sub(layout.size());
        System.dealloc(p, layout);
    }
    unsafe fn realloc(&self, p: *mut u8, old: Layout, new: usize) -> *mut u8 {
        let q = System.realloc(p, old, new);
        if !q.is_null() {
            if new >= old.size() { add(new - old.size()); }
            else { sub(old.size() - new); }
        }
        q
    }
}
#[global_allocator]
static ALLOC: Counting = Counting;

fn status() -> (u64, u64) {
    let s = std::fs::read_to_string("/proc/self/status").unwrap();
    let value = |name: &str| -> u64 {
        s.lines().find(|l| l.starts_with(name)).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap()
    };
    (value("VmRSS:"), value("VmHWM:"))
}
fn mappings() -> (u64, u64, u64) {
    let s = std::fs::read_to_string("/proc/self/smaps").unwrap();
    let mut name = String::new();
    let (mut heap, mut anonymous, mut executable) = (0, 0, 0);
    for line in s.lines() {
        if line.as_bytes().first().is_some_and(|b| b.is_ascii_hexdigit()) && line.contains('-') && line.contains(' ') {
            name = line.splitn(6, ' ').nth(5).unwrap_or("").to_owned();
        } else if let Some(v) = line.strip_prefix("Rss:") {
            let kib: u64 = v.split_whitespace().next().unwrap().parse().unwrap();
            if name == "[heap]" { heap += kib; }
            else if name.is_empty() { anonymous += kib; }
            else if name.ends_with("/font-memory-item2-probe") { executable += kib; }
        }
    }
    (heap, anonymous, executable)
}
fn sample(stage: &str, count: usize, source_bytes: usize, coverage_points: usize) {
    let (rss, hwm) = status();
    let (heap, anonymous, executable) = mappings();
    let live = LIVE.load(Ordering::Relaxed);
    let peak = PEAK.load(Ordering::Relaxed);
    println!("{}", serde_json::json!({"stage":stage,"count":count,"source_bytes":source_bytes,"coverage_points":coverage_points,"live_requested_bytes":live,"peak_requested_bytes":peak,"rss_kib":rss,"hwm_kib":hwm,"heap_rss_kib":heap,"anonymous_rss_kib":anonymous,"executable_rss_kib":executable}));
}
fn embedded() {
    sample("start",0,0,0);
    let sets = typst_infra::embedded_fonts::load_embedded_fonts();
    let count = sets.text_slots.len() + sets.math_code_slots.len();
    black_box(&sets);
    sample("embedded_catalog_held",count,0,0);
    let fonts: Vec<_> = sets.text_slots.iter().chain(sets.math_code_slots.iter()).filter_map(|s|s.get()).collect();
    let total: usize = fonts.iter().map(|f| f.as_slice().len()).sum();
    black_box(&fonts);
    sample("embedded_selected_all_held",fonts.len(),total,0);
}
fn system_db() {
    sample("start",0,0,0);
    let mut db = fontdb::Database::new(); db.load_system_fonts();
    let count = db.faces().count();
    black_box(&db);
    sample("fontdb_held",count,0,0);
    drop(db);
    sample("fontdb_dropped",count,0,0);
}
fn system_catalog(metadata_only: bool, with_slots: bool) {
    sample("start",0,0,0);
    if metadata_only {
        let mut db = fontdb::Database::new(); db.load_system_fonts();
        let mut book = FontBook::new();
        let mut slots = Vec::new();
        for face in db.faces() {
            if let Some(info) = db.with_face_data(face.id, typst_infra::fonts::font_info_from_bytes).flatten() {
                if with_slots {
                    let path = match &face.source {
                        fontdb::Source::File(path) | fontdb::Source::SharedFile(path, _) => path.clone(),
                        fontdb::Source::Binary(_) => continue,
                    };
                    slots.push(typst_infra::fonts::FontSlot::new(path, face.index));
                }
                book.push(info);
            }
        }
        let count = book.len();
        drop(db);
        black_box((&book,&slots));
        sample(if with_slots {"metadata_slots_held"} else {"metadata_book_held"},count,0,0);
    } else {
        let (slots, book) = typst_infra::fontdb::load_system_fonts();
        let count = book.len();
        let coverage_points = book.infos().iter().map(|i| i.coverage.iter().count()).sum();
        black_box((&slots,&book));
        sample("system_catalog_held",count,0,coverage_points);
    }
}
fn coverage_control() {
    sample("start",0,0,0);
    let mut db = fontdb::Database::new();db.load_system_fonts();
    let mut book = FontBook::new();
    for face in db.faces() {
        if let Some(info) = db.with_face_data(face.id, |data,index| {
            let mut info = typst_infra::fonts::font_info_from_bytes(data,index)?;
            let parsed = ttf_parser::Face::parse(data,index).ok()?;
            let mut points = Vec::new();
            if let Some(cmap) = parsed.tables().cmap {
                for table in cmap.subtables { if table.is_unicode() { table.codepoints(&mut |p| points.push(p)); } }
            }
            info.coverage = Coverage::from_codepoints(points);
            Some(info)
        }).flatten() {book.push(info);}
    }
    let count=book.len();let coverage_points=book.infos().iter().map(|i|i.coverage.iter().count()).sum();
    drop(db);black_box(&book);
    sample("legacy_coverage_book_held",count,0,coverage_points);
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("embedded") => embedded(),
        Some("system-db") => system_db(),
        Some("system-metadata") => system_catalog(true,false),
        Some("system-metadata-slots") => system_catalog(true,true),
        Some("system-catalog") => system_catalog(false,false),
        Some("coverage-control") => coverage_control(),
        _ => panic!("mode required"),
    }
}
