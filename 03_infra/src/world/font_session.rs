//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/system-world/font_session.md
//! @prompt-hash 7dfa7e7d
//! @layer L3
//! @updated 2026-10-09

use crate::fontdb::generation::{FastKey, GenerationKey, PreparedDiscovery};
use crate::fonts::FontSlot;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use typst_core::entities::font_book::FontBook;
type Catalog = (Vec<FontSlot>, FontBook);
struct Cached {
    key: GenerationKey,
    fast: Option<FastKey>,
    catalog: Arc<Catalog>,
}
/// Resources eligible for reuse across fresh compilation worlds.
pub struct FontCatalogSession {
    cached: Mutex<Option<Cached>>,
}
impl std::default::Default for FontCatalogSession {
    fn default() -> Self {
        Self::new()
    }
}

impl FontCatalogSession {
    pub fn new() -> Self {
        Self { cached: Mutex::new(None) }
    }
    pub(super) fn catalog(&self, paths: &[PathBuf]) -> Arc<Catalog> {
        self.catalog_with_fast(
            || crate::fontdb::generation::fast_key(paths),
            |key| crate::fontdb::generation::fast_key_unchanged(key, paths),
            || crate::fontdb::generation::prepare(paths),
            super::fonts::prepared_catalog,
            || super::fonts::fresh_catalog(paths),
        )
    }
    #[cfg(all(test, unix))]
    fn catalog_with(
        &self,
        prepare: impl FnOnce() -> Option<PreparedDiscovery>,
        build: impl FnOnce(PreparedDiscovery) -> Catalog,
        fresh: impl FnOnce() -> Catalog,
    ) -> Arc<Catalog> {
        self.catalog_with_fast(|| None, |_| false, prepare, build, fresh)
    }
    fn catalog_with_fast(
        &self,
        mut probe: impl FnMut() -> Option<FastKey>,
        verify: impl Fn(&FastKey) -> bool,
        prepare: impl FnOnce() -> Option<PreparedDiscovery>,
        build: impl FnOnce(PreparedDiscovery) -> Catalog,
        fresh: impl FnOnce() -> Catalog,
    ) -> Arc<Catalog> {
        let Ok(mut cached) = self.cached.lock() else {
            return Arc::new(fresh());
        };
        if let Some(old) = cached.as_ref() {
            if old.fast.as_ref().is_some_and(&verify) && old.key.unchanged() {
                return old.catalog.clone();
            }
        }
        let before = probe();
        let Some(prepared) = prepare() else {
            *cached = None;
            drop(cached);
            return Arc::new(fresh());
        };
        if !prepared.unchanged() {
            *cached = None;
            drop(cached);
            return Arc::new(fresh());
        }
        if let Some(old) = cached.as_ref().filter(|old| old.key == prepared.key) {
            // A full discovery can reuse a catalog only when its observation
            // stayed stable. A changed probe forces a fresh generation.
            if before.as_ref().is_none_or(&verify) {
                return old.catalog.clone();
            }
        }
        let key = prepared.key.clone();
        let catalog = Arc::new(build(prepared));
        if !key.unchanged() || !before.as_ref().is_none_or(verify) {
            *cached = None;
            drop(cached);
            return Arc::new(fresh());
        }
        *cached = Some(Cached { key, fast: before, catalog: catalog.clone() });
        catalog
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::super::SystemWorld;
    use super::*;
    use crate::fontdb::generation::fixture_prepare;
    use typst_core::contracts::world::World;
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "watch-session-{}-{:016x}",
                std::process::id(),
                fastrand::u64(..)
            ));
            std::fs::create_dir(&p).unwrap();
            std::fs::write(
                p.join("a.ttf"),
                include_bytes!("../../fixtures/fonts/NotoSans-Regular.ttf"),
            )
            .unwrap();
            Self(p)
        }
        fn paths(&self) -> Vec<PathBuf> {
            vec![self.0.clone()]
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    fn build(p: PreparedDiscovery) -> Catalog {
        crate::fonts::pair_slots_with_book(p.project_slots)
    }
    fn empty() -> Catalog {
        (vec![], FontBook::new())
    }
    #[test]
    fn default_contract_font_session_is_lazy_reusable_and_independent() {
        let session = <FontCatalogSession as Default>::default();
        let untouched = <FontCatalogSession as Default>::default();
        let control = FontCatalogSession::new();
        assert!(session.cached.lock().unwrap().is_none());
        assert!(untouched.cached.lock().unwrap().is_none());
        assert!(control.cached.lock().unwrap().is_none());
        let fixture = Fixture::new();
        let paths = fixture.paths();
        let catalog = session.catalog_with(|| fixture_prepare(&paths), build, empty);
        assert_eq!(catalog.0.len(), 1);
        assert_eq!(catalog.0.len(), catalog.1.len());
        let reused = session.catalog_with(
            || fixture_prepare(&paths),
            |_| panic!("unchanged Default session must reuse its catalog"),
            empty,
        );
        assert!(Arc::ptr_eq(&catalog, &reused));
        assert!(untouched.cached.lock().unwrap().is_none());
        assert!(control.cached.lock().unwrap().is_none());
    }
    #[test]
    fn watch_session_reuses_and_renews_generation() {
        let f = Fixture::new();
        let s = FontCatalogSession::new();
        let paths = f.paths();
        let a = s.catalog_with(|| fixture_prepare(&paths), build, empty);
        let b = s.catalog_with(
            || fixture_prepare(&paths),
            |_| panic!("metadata reconstructed"),
            empty,
        );
        assert!(Arc::ptr_eq(&a, &b));
        std::fs::write(
            f.0.join("b.ttf"),
            include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf"),
        )
        .unwrap();
        let c = s.catalog_with(|| fixture_prepare(&paths), build, empty);
        assert!(!Arc::ptr_eq(&a, &c));
        assert_eq!(a.0.len(), 1);
        assert_eq!(c.0.len(), 2);
        assert_eq!(c.0.len(), c.1.len());
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn watch_session_fast_proof_skips_discovery_and_recovers_on_addition() {
        let f = Fixture::new();
        let s = FontCatalogSession::new();
        let paths = f.paths();
        let proof = || crate::fontdb::generation::fixture_fast_key(&paths);
        let verify = crate::fontdb::generation::fixture_fast_key_unchanged;
        let a =
            s.catalog_with_fast(proof, verify, || fixture_prepare(&paths), build, empty);
        let b = s.catalog_with_fast(
            proof,
            verify,
            || panic!("unchanged proof must skip discovery"),
            |_| panic!("unchanged proof must skip metadata"),
            empty,
        );
        assert!(Arc::ptr_eq(&a, &b));
        std::fs::write(
            f.0.join("b.ttf"),
            include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf"),
        )
        .unwrap();
        let c =
            s.catalog_with_fast(proof, verify, || fixture_prepare(&paths), build, empty);
        assert!(!Arc::ptr_eq(&a, &c));
        assert_eq!(c.0.len(), 2);
        assert_eq!(c.0.len(), c.1.len());
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn watch_session_fast_proof_refuses_change_during_build() {
        let f = Fixture::new();
        let s = FontCatalogSession::new();
        let paths = f.paths();
        let c = s.catalog_with_fast(
            || crate::fontdb::generation::fixture_fast_key(&paths),
            crate::fontdb::generation::fixture_fast_key_unchanged,
            || fixture_prepare(&paths),
            |prepared| {
                std::fs::remove_file(f.0.join("a.ttf")).unwrap();
                build(prepared)
            },
            empty,
        );
        assert!(c.0.is_empty());
        assert!(s.cached.lock().unwrap().is_none());
    }
    #[test]
    fn watch_session_refusal_clears_previous() {
        let f = Fixture::new();
        let s = FontCatalogSession::new();
        let paths = f.paths();
        s.catalog_with(|| fixture_prepare(&paths), build, empty);
        let fallback = s.catalog_with(|| None, build, empty);
        assert_eq!(fallback.0.len(), 0);
        assert!(s.cached.lock().unwrap().is_none());
    }
    #[test]
    fn watch_session_changed_during_build_not_cached() {
        let f = Fixture::new();
        let s = FontCatalogSession::new();
        let paths = f.paths();
        let c = s.catalog_with(
            || fixture_prepare(&paths),
            |p| {
                std::fs::remove_file(f.0.join("a.ttf")).unwrap();
                build(p)
            },
            empty,
        );
        assert_eq!(c.0.len(), 0);
        assert!(s.cached.lock().unwrap().is_none());
    }
    #[test]
    fn watch_session_poison_uses_fresh() {
        let s = FontCatalogSession::new();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = s.cached.lock().unwrap();
            panic!("fixture poison")
        }));
        let c = s.catalog_with(|| panic!("poison must bypass prepare"), build, empty);
        assert_eq!(c.0.len(), 0);
    }
    #[test]
    fn watch_session_builder_lazy_and_replacement() {
        let s = Arc::new(FontCatalogSession::new());
        let world = SystemWorld::for_eval(".").with_font_catalog_session(s.clone(), &[]);
        assert!(s.cached.lock().unwrap().is_none());
        assert!(world.font_catalog.get().is_none());
        assert!(!world.book().is_empty());
        assert!(s.cached.lock().unwrap().is_some());
        let replaced = world.with_fonts(vec![]);
        assert_eq!(replaced.book().len(), 0);
    }
    #[test]
    fn watch_session_files_and_negative_reads_are_fresh() {
        let f = Fixture::new();
        let s = Arc::new(FontCatalogSession::new());
        std::fs::write(f.0.join("main.typ"), "First").unwrap();
        let a = SystemWorld::new(&f.0, "main.typ")
            .unwrap()
            .with_font_catalog_session(s.clone(), &[]);
        assert!(a.read_bytes(a.main(), "missing.txt").is_err());
        std::fs::write(f.0.join("main.typ"), "Second").unwrap();
        std::fs::write(f.0.join("missing.txt"), "Present").unwrap();
        let b = SystemWorld::new(&f.0, "main.typ")
            .unwrap()
            .with_font_catalog_session(s, &[]);
        assert_eq!(a.source(a.main()).unwrap().text(), "First");
        assert_eq!(b.source(b.main()).unwrap().text(), "Second");
        assert_eq!(&*b.read_bytes(b.main(), "missing.txt").unwrap(), b"Present");
    }
    #[test]
    fn watch_session_concurrent_generation_is_paired() {
        let f = Fixture::new();
        let s = FontCatalogSession::new();
        let paths = f.paths();
        std::thread::scope(|scope| {
            let handles = (0..4)
                .map(|_| {
                    scope.spawn(|| {
                        s.catalog_with(|| fixture_prepare(&paths), build, empty)
                    })
                })
                .collect::<Vec<_>>();
            let catalogs =
                handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();
            for c in &catalogs {
                assert_eq!(c.0.len(), c.1.len());
                assert!(Arc::ptr_eq(c, &catalogs[0]));
            }
        });
    }
    #[test]
    fn watch_session_negative_font_recovers_in_new_generation() {
        let f = Fixture::new();
        let s = FontCatalogSession::new();
        let prep = || crate::fontdb::generation::fixture_system_prepare(&f.0);
        let sys = |p: PreparedDiscovery| crate::fontdb::catalog_from_database(&p.db);
        let a = s.catalog_with(prep, sys, empty);
        std::fs::remove_file(f.0.join("a.ttf")).unwrap();
        assert!(a.0[0].get().is_none());
        std::fs::write(
            f.0.join("a.ttf"),
            include_bytes!("../../fixtures/fonts/NotoSans-Regular.ttf"),
        )
        .unwrap();
        let b = s.catalog_with(prep, sys, empty);
        assert!(!Arc::ptr_eq(&a, &b));
        assert!(b.0[0].get().is_some());
        assert!(a.0[0].get().is_none());
    }
    #[test]
    fn watch_session_change_between_prepare_and_reuse_declines() {
        let f = Fixture::new();
        let s = FontCatalogSession::new();
        let paths = f.paths();
        s.catalog_with(|| fixture_prepare(&paths), build, empty);
        let c = s.catalog_with(
            || {
                let p = fixture_prepare(&paths);
                std::fs::remove_file(f.0.join("a.ttf")).unwrap();
                p
            },
            build,
            empty,
        );
        assert!(c.0.is_empty());
        assert!(s.cached.lock().unwrap().is_none());
    }
    #[test]
    fn watch_session_different_configs_keep_live_generations() {
        let a = Fixture::new();
        let b = Fixture::new();
        std::fs::write(
            b.0.join("b.otf"),
            include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf"),
        )
        .unwrap();
        let s = FontCatalogSession::new();
        std::thread::scope(|scope| {
            let x = scope
                .spawn(|| s.catalog_with(|| fixture_prepare(&a.paths()), build, empty));
            let y = scope
                .spawn(|| s.catalog_with(|| fixture_prepare(&b.paths()), build, empty));
            let x = x.join().unwrap();
            let y = y.join().unwrap();
            assert_eq!(x.0.len(), 1);
            assert_eq!(y.0.len(), 2);
            assert_eq!(x.0.len(), x.1.len());
            assert_eq!(y.0.len(), y.1.len());
        });
    }
}
