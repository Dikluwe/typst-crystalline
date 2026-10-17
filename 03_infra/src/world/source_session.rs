//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/system-world/source_session.md
//! @prompt-hash 67095be6
//! @layer L3
//! @updated 2026-10-08

use std::path::PathBuf;
use std::sync::Mutex;
use typst_core::entities::{file_id::FileId, source::Source};

const MAX_ENTRIES: usize = 8;
const MAX_BYTES: usize = 1024 * 1024;
const MAX_ENTRY_BYTES: usize = 512 * 1024;

struct Entry {
    path: PathBuf,
    source: Source,
}
/// Bounded syntax-only retention belonging to one watch invocation.
pub struct SourceSession {
    entries: Mutex<Vec<Entry>>,
}
impl std::default::Default for SourceSession {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceSession {
    pub fn new() -> Self {
        Self { entries: Mutex::new(Vec::new()) }
    }
    pub(super) fn get_or_parse(&self, path: PathBuf, id: FileId, text: String) -> Source {
        let mut entries = self.entries.lock().unwrap();
        if let Some(index) =
            entries.iter().position(|e| e.path == path && e.source.id() == id)
        {
            let entry = entries.remove(index);
            if entry.source.text() == text {
                let source = entry.source.clone();
                entries.push(entry);
                return source;
            }
        }
        if !path.is_absolute() || text.len() > MAX_ENTRY_BYTES {
            drop(entries);
            return Source::new(id, text);
        }
        let source = Source::new(id, text);
        entries.push(Entry { path, source: source.clone() });
        while entries.len() > MAX_ENTRIES
            || entries.iter().map(|e| e.source.len_bytes()).sum::<usize>() > MAX_BYTES
        {
            entries.remove(0);
        }
        source
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::SystemWorld;
    use std::{fs, sync::Arc};
    use typst_core::contracts::world::World;
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir()
                .join(format!("tekt-p6-{}-{stamp}", std::process::id()));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
        fn file(&self, name: &str, text: &str) -> PathBuf {
            let p = self.0.join(name);
            fs::write(&p, text).unwrap();
            p
        }
        fn world(&self, name: &str, s: &Arc<SourceSession>) -> SystemWorld {
            SystemWorld::new_with_source_session(&self.0, name, s.clone()).unwrap()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn same(a: &Source, b: &Source) -> bool {
        std::ptr::eq(a.root(), b.root())
    }
    fn id(n: u16) -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(n).unwrap())
    }
    #[test]
    fn default_contract_source_session_reuses_validated_text_in_fresh_worlds() {
        let session = Arc::new(<SourceSession as Default>::default());
        let independent = Arc::new(<SourceSession as Default>::default());
        assert!(session.entries.lock().unwrap().is_empty());
        assert!(independent.entries.lock().unwrap().is_empty());
        assert!(SourceSession::new().entries.lock().unwrap().is_empty());
        let fixture = Fixture::new();
        let path = fixture.file("main.typ", "Alpha");
        let first_world = fixture.world("main.typ", &session);
        let first = first_world.source(first_world.main()).unwrap();
        let next_world = fixture.world("main.typ", &session);
        let reused = next_world.source(next_world.main()).unwrap();
        assert!(same(&first, &reused));
        assert!(next_world.read_cache.lock().unwrap().is_empty());
        assert!(independent.entries.lock().unwrap().is_empty());
        fs::write(path, "Bravo").unwrap();
        let independent_world = fixture.world("main.typ", &independent);
        assert_eq!(
            independent_world.source(independent_world.main()).unwrap().text(),
            "Bravo"
        );
        assert_eq!(first.text(), "Alpha");
        assert_eq!(session.entries.lock().unwrap()[0].source.text(), "Alpha");
        let updated_world = fixture.world("main.typ", &session);
        let updated = updated_world.source(updated_world.main()).unwrap();
        assert_eq!(updated.text(), "Bravo");
        assert!(!same(&first, &updated));
    }
    #[test]
    fn default_contract_source_session_preserves_retention_bounds() {
        let fixture = Fixture::new();
        let session = <SourceSession as Default>::default();
        for n in 0..=MAX_ENTRIES {
            session.get_or_parse(fixture.0.join(format!("{n}.typ")), id(1), "a".into());
        }
        assert_eq!(session.entries.lock().unwrap().len(), MAX_ENTRIES);
        assert!(!session
            .entries
            .lock()
            .unwrap()
            .iter()
            .any(|entry| entry.path == fixture.0.join("0.typ")));
        let oversized = "a".repeat(MAX_ENTRY_BYTES + 1);
        let source =
            session.get_or_parse(fixture.0.join("large.typ"), id(1), oversized.clone());
        assert_eq!(source.text(), oversized);
        assert_eq!(session.entries.lock().unwrap().len(), MAX_ENTRIES);
        assert!(!session
            .entries
            .lock()
            .unwrap()
            .iter()
            .any(|entry| entry.path == fixture.0.join("large.typ")));
        let aggregate = <SourceSession as Default>::default();
        for n in 0..3 {
            aggregate.get_or_parse(
                fixture.0.join(format!("aggregate{n}.typ")),
                id(1),
                "a".repeat(MAX_ENTRY_BYTES),
            );
        }
        let retained = aggregate.entries.lock().unwrap();
        assert_eq!(retained.len(), 2);
        assert_eq!(
            retained.iter().map(|entry| entry.source.len_bytes()).sum::<usize>(),
            MAX_BYTES
        );
    }
    #[test]
    fn unchanged_main_and_import_share_cst_but_worlds_stay_fresh() {
        let f = Fixture::new();
        f.file("main.typ", "main");
        let import = f.file("lib.typ", "#let value = 4");
        let session = Arc::new(SourceSession::new());
        let w = f.world("main.typ", &session);
        let main = w.source(w.main()).unwrap();
        let imported = w.source(w.register_file(import.clone())).unwrap();
        let w2 = f.world("main.typ", &session);
        let main2 = w2.source(w2.main()).unwrap();
        assert!(same(&main, &main2));
        assert_eq!(w2.dependencies(), vec![f.0.join("main.typ")]);
        let imported2 = w2.source(w2.register_file(import)).unwrap();
        assert!(same(&imported, &imported2));
        assert_eq!(w2.dependencies().len(), 2);
        assert!(w2.read_cache.lock().unwrap().is_empty());
    }
    #[test]
    fn same_size_and_restored_mtime_cannot_validate_different_text() {
        let f = Fixture::new();
        let p = f.file("main.typ", "Alpha");
        let session = Arc::new(SourceSession::new());
        let a = f.world("main.typ", &session).source(id(1)).unwrap();
        let old = fs::metadata(&p).unwrap().modified().unwrap();
        fs::write(&p, "Bravo").unwrap();
        fs::File::options()
            .write(true)
            .open(&p)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(old))
            .unwrap();
        let w = f.world("main.typ", &session);
        let b = w.source(id(1)).unwrap();
        assert_eq!(b.text(), "Bravo");
        assert!(!same(&a, &b));
        assert_eq!(a.text(), "Alpha");
    }
    #[test]
    fn current_file_id_and_path_are_required() {
        let f = Fixture::new();
        let p = f.file("one.typ", "same");
        let q = f.file("two.typ", "same");
        let session = SourceSession::new();
        let a = session.get_or_parse(p.clone(), id(2), "same".into());
        let b = session.get_or_parse(p.clone(), id(3), "same".into());
        let c = session.get_or_parse(q, id(2), "same".into());
        assert!(!same(&a, &b));
        assert!(!same(&a, &c));
        assert_eq!(b.id(), id(3));
        let d = session.get_or_parse(p, id(2), "same".into());
        assert!(same(&a, &d));
    }
    #[test]
    fn read_error_utf8_and_recreation_are_validated_in_each_world() {
        let f = Fixture::new();
        let p = f.file("main.typ", "café");
        let session = Arc::new(SourceSession::new());
        let a = f.world("main.typ", &session).source(id(1)).unwrap();
        fs::remove_file(&p).unwrap();
        assert!(SystemWorld::new_with_source_session(&f.0, "main.typ", session.clone())
            .is_err());
        fs::write(&p, [0xff]).unwrap();
        assert!(SystemWorld::new_with_source_session(&f.0, "main.typ", session.clone())
            .is_err());
        fs::write(&p, "café").unwrap();
        let b = f.world("main.typ", &session).source(id(1)).unwrap();
        assert!(same(&a, &b));
        assert_eq!(b.text(), "café");
    }
    #[test]
    fn changed_import_order_does_not_restore_an_old_id() {
        let f = Fixture::new();
        f.file("main.typ", "main");
        let p = f.file("one.typ", "one");
        let q = f.file("two.typ", "two");
        let session = Arc::new(SourceSession::new());
        let w = f.world("main.typ", &session);
        let a = w.source(w.register_file(p.clone())).unwrap();
        let w2 = f.world("main.typ", &session);
        w2.register_file(q);
        let b = w2.source(w2.register_file(p)).unwrap();
        assert_ne!(a.id(), b.id());
        assert!(!same(&a, &b));
        assert_eq!(b.id(), id(3));
    }
    #[test]
    fn lru_evicts_oldest_and_preserves_touched_entry() {
        let f = Fixture::new();
        let session = SourceSession::new();
        let p = f.0.join("0.typ");
        let a = session.get_or_parse(p.clone(), id(1), "a".into());
        for n in 1..8 {
            session.get_or_parse(f.0.join(format!("{n}.typ")), id(1), "a".into());
        }
        assert!(same(&a, &session.get_or_parse(p, id(1), "a".into())));
        session.get_or_parse(f.0.join("8.typ"), id(1), "a".into());
        let entries = session.entries.lock().unwrap();
        assert_eq!(entries.len(), MAX_ENTRIES);
        assert!(!entries.iter().any(|e| e.path == f.0.join("1.typ")));
    }
    #[test]
    fn aggregate_and_per_entry_limits_do_not_change_returned_source() {
        let f = Fixture::new();
        let session = SourceSession::new();
        let text = "a".repeat(MAX_ENTRY_BYTES);
        for n in 0..3 {
            session.get_or_parse(f.0.join(format!("{n}.typ")), id(1), text.clone());
        }
        let entries = session.entries.lock().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries.iter().map(|e| e.source.len_bytes()).sum::<usize>(),
            MAX_BYTES
        );
        drop(entries);
        let path = f.0.join("large.typ");
        let large = "a".repeat(MAX_ENTRY_BYTES + 1);
        let a = session.get_or_parse(path.clone(), id(1), large.clone());
        let b = session.get_or_parse(path, id(1), large);
        assert!(!same(&a, &b));
        assert_eq!(a.len_bytes(), MAX_ENTRY_BYTES + 1);
        assert_eq!(session.entries.lock().unwrap().len(), 2);
    }
    #[test]
    fn syntax_diagnostics_and_transients_preserve_their_own_sources() {
        let f = Fixture::new();
        f.file("main.typ", "#let = (");
        let session = Arc::new(SourceSession::new());
        let a = f.world("main.typ", &session).source(id(1)).unwrap();
        let b = f.world("main.typ", &session).source(id(1)).unwrap();
        assert!(same(&a, &b));
        assert!(!a.root().errors().is_empty());
        let w = SystemWorld::for_stdin(&f.0, "stdin".into());
        let first = w.source_for_expression("1".into()).unwrap();
        let second = w.source_for_expression("1".into()).unwrap();
        assert_ne!(first.id(), second.id());
        assert!(!same(&first, &second));
        assert_eq!(session.entries.lock().unwrap().len(), 1);
    }
    #[test]
    fn concurrent_worlds_never_mix_text_or_identity() {
        let f = Fixture::new();
        let p = f.file("main.typ", "shared");
        let session = Arc::new(SourceSession::new());
        let _ = session.get_or_parse(p.clone(), id(1), "shared".into());
        let threads = (0..8)
            .map(|n| {
                let s = session.clone();
                let p = p.clone();
                std::thread::spawn(move || {
                    let text = format!("value{n}");
                    let source = s.get_or_parse(p, id(n + 1), text.clone());
                    assert_eq!(source.id(), id(n + 1));
                    assert_eq!(source.text(), text);
                })
            })
            .collect::<Vec<_>>();
        for t in threads {
            t.join().unwrap();
        }
        assert!(session.entries.lock().unwrap().len() <= MAX_ENTRIES);
    }
}
