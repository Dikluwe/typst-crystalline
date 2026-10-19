//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/system-world/tests.md
//! @prompt-hash 84df2fd4
//! @layer L3
//! @updated 2026-10-07

use super::*;
use std::num::NonZeroU16;
use typst_core::contracts::world::World;
use typst_core::entities::font_book::FontBook;
use typst_core::entities::package_spec::PackageSpec;
use typst_core::entities::source::Source;
use typst_core::entities::world_types::FileResult;
use typst_core::entities::world_types::{Bytes, Datetime, FileError, Font, Library};

// ── MockWorld para testes sem filesystem ──────────────────────────────

struct MockWorld {
    main_id: FileId,
    source: Source,
    library: Library,
    book: FontBook,
}

impl MockWorld {
    fn new(text: &str) -> Self {
        let id = FileId::from_raw(NonZeroU16::new(42).unwrap());
        Self {
            main_id: id,
            source: Source::new(id, text.to_string()),
            library: Library::new(),
            book: FontBook::new(),
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
        self.main_id
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
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
        _: Option<typst_core::entities::duration::Duration>,
    ) -> Option<Datetime> {
        None
    }
}

// ── Testes de MockWorld ───────────────────────────────────────────────

#[test]
fn mock_world_source_roundtrip() {
    let world = MockWorld::new("Hello *world*");
    let src = world.source(world.main()).unwrap();
    assert_eq!(src.text(), "Hello *world*");
}

#[test]
fn mock_world_source_not_found_other_id() {
    let world = MockWorld::new("text");
    let other = FileId::from_raw(NonZeroU16::new(99).unwrap());
    assert!(matches!(world.source(other), Err(FileError::NotFound)));
}

#[test]
fn mock_world_font_none() {
    let world = MockWorld::new("text");
    assert!(world.font(0).is_none());
}

#[test]
fn mock_world_today_none() {
    let world = MockWorld::new("text");
    assert!(world.today(None).is_none());
    assert!(world
        .today(Some(typst_core::entities::duration::Duration::from_hours(2)))
        .is_none());
}

// ── Teste de integração parse→world ──────────────────────────────────

#[test]
fn parse_via_mock_world() {
    use typst_core::entities::syntax_kind::SyntaxKind;

    let world = MockWorld::new("= Heading\n\nParagraph.");
    let src = world.source(world.main()).unwrap();

    assert_eq!(src.root().kind(), SyntaxKind::Markup);
    assert!(!src.root().erroneous());

    let has_heading = src.root().children().any(|n| n.kind() == SyntaxKind::Heading);
    assert!(has_heading);
}

// ── Testes de SystemWorld com ficheiro temporário ─────────────────────

#[test]
fn system_world_main_id_valid() {
    let dir = tempfile_write("hello.typ", "Hello world");
    let world = SystemWorld::new(dir.path(), "hello.typ").unwrap();
    let _ = world.main();
}

#[test]
fn system_world_source_reads_file() {
    let dir = tempfile_write("main.typ", "= Title");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let src = world.source(world.main()).unwrap();
    assert_eq!(src.text(), "= Title");
}

#[test]
fn system_world_source_not_found_unknown_id() {
    let dir = tempfile_write("main.typ", "text");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let unknown = FileId::from_raw(NonZeroU16::new(200).unwrap());
    assert!(matches!(world.source(unknown), Err(FileError::NotFound)));
}

#[test]
fn system_world_new_missing_file_errors() {
    let dir = tempdir();
    let result = SystemWorld::new(dir.path(), "nonexistent.typ");
    assert!(result.is_err());
}

#[test]
fn system_world_font_stub_returns_none() {
    let dir = tempfile_write("main.typ", "text");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    assert!(world.font(0).is_none());
}

#[test]
fn system_world_today_returns_some() {
    let dir = tempfile_write("main.typ", "text");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let dt = world.today(None);
    assert!(dt.is_some());
    // Ano razoável (> 2020) para verificar que é data real
    assert!(dt.unwrap().year().unwrap_or(0) > 2020);
}

#[test]
fn system_world_source_lazy_via_onclock() {
    // OnceLock: segunda chamada retorna o mesmo resultado
    let dir = tempfile_write("main.typ", "lazy content");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let src1 = world.source(world.main()).unwrap();
    let src2 = world.source(world.main()).unwrap();
    assert!(src1 == src2);
}

#[test]
fn system_world_register_file_returns_same_id() {
    let dir = tempfile_write("a.typ", "content a");
    let world = SystemWorld::new(dir.path(), "a.typ").unwrap();

    let extra = dir.path().join("extra.typ");
    std::fs::write(&extra, "extra").unwrap();

    let id1 = world.register_file(extra.clone());
    let id2 = world.register_file(extra);
    assert_eq!(id1, id2);
}

#[test]
fn system_world_with_system_fonts_nao_panic() {
    let dir = tempfile_write("main.typ", "text");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap().with_system_fonts();
    // Invariante: book.len() <= slots.len()
    assert!(world.book().len() <= world.font_catalog().0.len());
}

#[test]
fn system_world_with_fonts_and_system_inclui_projecto() {
    let dir = tempfile_write("main.typ", "text");
    let font_dir = tempdir();
    std::fs::write(font_dir.path().join("fake.ttf"), b"not a font").unwrap();

    let world = SystemWorld::new(dir.path(), "main.typ")
        .unwrap()
        .with_fonts_and_system(&[font_dir.path().to_path_buf()]);

    // Sistema pode ou não ter fontes; projecto adiciona 1 slot.
    assert!(!world.font_catalog().0.is_empty());
}

#[test]
fn system_world_with_fonts_and_system_preserva_ordem() {
    let dir = tempfile_write("main.typ", "text");
    let font_dir = tempdir();
    // Fonte VÁLIDA (cópia da fixture) — desde P839 (#28/I4) fontes sem
    // info extraível nem sequer entram nos slots.
    let nimbus = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/fonts/NimbusSans-Regular.otf"
    ))
    .unwrap();
    std::fs::write(font_dir.path().join("project.otf"), &nimbus).unwrap();

    let world = SystemWorld::new(dir.path(), "main.typ")
        .unwrap()
        .with_fonts_and_system(&[font_dir.path().to_path_buf()]);

    // O último slot deve ser o do projecto.
    let last = world.font_catalog().0.last().unwrap();
    assert_eq!(last.path.file_name().unwrap(), "project.otf");
}

/// **P938** — `SystemWorld::candidates_for_char` preenche coverage exacta
/// lazy e devolve os índices correctos.
#[test]
fn p938_system_world_candidates_for_char_exacto_lazy() {
    use typst_core::contracts::world::World;

    let dir = tempfile_write("main.typ", "text");
    let font_dir = tempdir();
    let nimbus = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/fonts/NimbusSans-Regular.otf"
    ))
    .unwrap();
    std::fs::write(font_dir.path().join("project.otf"), &nimbus).unwrap();

    let slots = crate::fonts::discover_fonts(&[font_dir.path().to_path_buf()]);
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap().with_fonts(slots);

    // Apenas uma face no book/slots.
    assert_eq!(world.book().len(), 1);

    // 'A' (latim) é coberto por Nimbus Sans.
    let cands_a = world.candidates_for_char('A');
    assert_eq!(cands_a, vec![0], "Nimbus Sans cobre 'A'");

    // '你' (CJK) não é coberto por Nimbus Sans.
    let cands_cjk = world.candidates_for_char('你');
    assert!(cands_cjk.is_empty(), "Nimbus Sans não cobre CJK");
}

#[test]
fn system_world_font_invalid_slot_returns_none() {
    let dir = tempfile_write("main.typ", "text");
    let font_dir = tempdir();
    std::fs::write(font_dir.path().join("fake.ttf"), b"not a font").unwrap();

    let slots = crate::fonts::discover_fonts(&[font_dir.path().to_path_buf()]);
    assert_eq!(slots.len(), 1);

    let world = SystemWorld::new(dir.path(), "main.typ").unwrap().with_fonts(slots);

    // P839 (#28/I4) — o slot inválido é descartado no emparelhamento:
    // não há slot nenhum e qualquer índice retorna None.
    assert!(world.font(0).is_none());
    // Índice fora dos limites → None
    assert!(world.font(1).is_none());
}

// ── Passo 450 — carregamento de .bib do disco ─────────────────────────

#[test]
fn system_world_load_bibliography_carrega_bib() {
    let dir = tempfile_write("main.typ", "#bibliography(\"refs.bib\")");
    std::fs::write(
        dir.path().join("refs.bib"),
        br#"@article{smith2024, author = {Smith, John}, title = {On Crystal Math}, year = {2024}, journal = {Journal of Examples}}"#,
    ).unwrap();

    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let entries = world.load_bibliography(world.main(), "refs.bib").unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, "smith2024");
    assert_eq!(entries[0].author, "Smith, John");
    assert_eq!(entries[0].title, "On Crystal Math");
    assert_eq!(entries[0].year, 2024);
}

#[test]
fn system_world_load_bibliography_path_relativo_ao_source() {
    let dir = tempdir();
    std::fs::create_dir(dir.path().join("sub")).unwrap();
    std::fs::write(dir.path().join("main.typ"), "#bibliography(\"sub/refs.bib\")")
        .unwrap();
    std::fs::write(
        dir.path().join("sub").join("refs.bib"),
        br#"@book{doe2023, author = {Doe, Jane}, title = {A Book}, year = {2023}}"#,
    )
    .unwrap();

    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let entries = world.load_bibliography(world.main(), "sub/refs.bib").unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, "doe2023");
}

// ── Passo 686 — caminhos absolutos `/...` ─────────────────────────────

#[test]
fn system_world_include_source_absoluto_no_projecto() {
    let dir = tempdir();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("main.typ"), "#include \"/src/util.typ\"").unwrap();
    std::fs::write(dir.path().join("src").join("util.typ"), "de util").unwrap();

    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let src = world.include_source(world.main(), "/src/util.typ").unwrap();
    assert_eq!(src.text(), "de util");
}

#[test]
fn system_world_read_bytes_absoluto_no_projecto() {
    let dir = tempdir();
    std::fs::create_dir_all(dir.path().join("assets")).unwrap();
    std::fs::write(dir.path().join("main.typ"), "main").unwrap();
    std::fs::write(dir.path().join("assets").join("data.bin"), b"\x01\x02\x03").unwrap();

    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let bytes = world.read_bytes(world.main(), "/assets/data.bin").unwrap();
    assert_eq!(bytes.as_slice(), &[1, 2, 3]);
}

#[test]
fn system_world_include_source_absoluto_em_pacote() {
    let dir = tempdir();
    // Pacote fake em {dir}/typst/packages/preview/foo/0.1.0
    let pkg = dir.path().join("typst/packages/preview/foo/0.1.0");
    std::fs::create_dir_all(pkg.join("src/sub")).unwrap();
    std::fs::write(
        pkg.join("typst.toml"),
        "[package]\nname = \"foo\"\nversion = \"0.1.0\"\nentrypoint = \"src/lib.typ\"\n",
    )
    .unwrap();
    std::fs::write(pkg.join("src/lib.typ"), "library").unwrap();
    std::fs::write(pkg.join("src/sub/x.typ"), "#import \"/src/lib.typ\"").unwrap();
    std::fs::write(dir.path().join("main.typ"), "main").unwrap();

    // package_candidate_dirs() lê XDG_DATA_HOME; apontamos para o temp dir
    // para que o pacote fake seja reconhecido. Restauramos no fim.
    let prev = std::env::var_os("XDG_DATA_HOME");
    std::env::set_var("XDG_DATA_HOME", dir.path());

    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let x_id = world.register_file(pkg.join("src/sub/x.typ"));
    let src = world.include_source(x_id, "/src/lib.typ");

    if let Some(v) = prev {
        std::env::set_var("XDG_DATA_HOME", v);
    } else {
        std::env::remove_var("XDG_DATA_HOME");
    }

    // Resolve à raiz do pacote (0.1.0/src/lib.typ), não a src/sub.
    assert_eq!(src.unwrap().text(), "library");
}

#[test]
fn system_world_include_source_relativo_sem_regressao() {
    let dir = tempdir();
    std::fs::create_dir_all(dir.path().join("sub")).unwrap();
    std::fs::write(dir.path().join("main.typ"), "main").unwrap();
    std::fs::write(dir.path().join("sub").join("x.typ"), "relativo").unwrap();

    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let src = world.include_source(world.main(), "sub/x.typ").unwrap();
    assert_eq!(src.text(), "relativo");
}

// ── P827 — paridade da mensagem "package not found" (não-preview) ─────

#[test]
fn system_world_resolve_package_nao_preview_mensagem_vanilla() {
    use std::str::FromStr;

    let dir = tempfile_write("main.typ", "text");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let spec = PackageSpec::from_str("@local/inexistente:1.0.0").unwrap();

    let err = world.resolve_package(&spec).unwrap_err();
    // Vanilla `PackageError::NotFound` (typst-library/src/diag.rs):
    // `package not found (searched for @local/inexistente:1.0.0)`.
    assert_eq!(err, "package not found (searched for @local/inexistente:1.0.0)");
}

#[test]
fn synthetic_main_needs_no_physical_file() {
    let dir = tempdir();
    let world = SystemWorld::for_eval(dir.path());
    assert_eq!(world.source(world.main()).unwrap().text(), "");
    assert_eq!(world.directory_of(world.main()), dir.path());
    assert!(!dir.path().join("<input-expression>").exists());
}

#[test]
fn source_cache_preserves_first_read_and_first_error() {
    let dir = tempfile_write("main.typ", "original");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    std::fs::write(dir.path().join("main.typ"), "changed").unwrap();
    assert_eq!(world.source(world.main()).unwrap().text(), "original");
    let missing = dir.path().join("missing.typ");
    let id = world.register_file(missing.clone());
    assert!(matches!(world.source(id), Err(FileError::NotFound)));
    std::fs::write(missing, "appeared later").unwrap();
    assert!(matches!(world.source(id), Err(FileError::NotFound)));
}

#[test]
fn byte_cache_is_shared_by_legacy_and_rooted_reads() {
    let dir = tempfile_write("main.typ", "main");
    let asset = dir.path().join("asset.bin");
    std::fs::write(&asset, b"original").unwrap();
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let legacy = world.read_bytes(world.main(), "asset.bin").unwrap();
    let path = world.resolve_path(world.main(), "/asset.bin").unwrap();
    let rooted = world.read_path(&path).unwrap();
    assert!(Arc::ptr_eq(&legacy, &rooted));
    std::fs::write(&asset, b"changed").unwrap();
    let id = world.register_file(asset);
    assert_eq!(world.file(id).unwrap().as_slice(), b"original");
    assert!(Arc::ptr_eq(&legacy, &world.read_path(&path).unwrap()));
}

#[test]
fn byte_read_errors_are_not_cached() {
    let dir = tempfile_write("main.typ", "main");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    assert!(world.read_bytes(world.main(), "later.bin").is_err());
    std::fs::write(dir.path().join("later.bin"), b"available").unwrap();
    assert_eq!(
        world.read_bytes(world.main(), "later.bin").unwrap().as_slice(),
        b"available"
    );
}

#[test]
fn canonical_ids_and_dependency_inventory_stay_stable() {
    let dir = tempfile_write("main.typ", "main");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    assert_eq!(world.register_file(dir.path().join("./main.typ")), world.main());
    let missing = dir.path().join("missing.bin");
    assert!(world.read_bytes(world.main(), "missing.bin").is_err());
    let mut expected = vec![dir.path().join("main.typ").canonicalize().unwrap(), missing];
    expected.sort();
    assert_eq!(world.dependencies(), expected);
}

#[test]
fn rooted_paths_preserve_origin_and_reject_lexical_escape() {
    let dir = tempfile_write("main.typ", "main");
    std::fs::create_dir(dir.path().join("sub")).unwrap();
    std::fs::write(dir.path().join("sub/other.typ"), "other").unwrap();
    std::fs::write(dir.path().join("sub/data.bin"), b"local").unwrap();
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let other = world.register_file(dir.path().join("sub/other.typ"));
    let path = world.resolve_path(other, "data.bin").unwrap();
    assert_eq!(world.read_path(&path).unwrap().as_slice(), b"local");
    assert_eq!(
        world.resolve_path(world.main(), "../outside").unwrap_err(),
        "path `\"../outside\"` would escape the project root"
    );
}

#[test]
fn precomputed_coverage_needs_no_font_bytes() {
    let dir = tempdir();
    let data = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/fonts/NimbusSans-Regular.otf"
    ))
    .unwrap();
    let mut info = crate::fonts::font_info_from_bytes(&data, 0).unwrap();
    info.coverage = Coverage::from_codepoints(vec!['A' as u32]);
    let mut world = SystemWorld::for_eval(dir.path());
    let catalog = Arc::get_mut(world.font_catalog.get_mut().unwrap()).unwrap();
    catalog.1.push(info);
    catalog.0.push(FontSlot::new(dir.path().join("absent.otf"), 0));
    assert_eq!(world.candidates_for_char('A'), vec![0]);
    assert!(world.candidates_for_char('你').is_empty());
    assert!(world.coverage_cache.lock().unwrap().is_empty());
    assert!(world.font(0).is_none());
}

// ── Utilitários de teste ──────────────────────────────────────────────

struct TempDir(PathBuf);

impl TempDir {
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn tempdir() -> TempDir {
    let path = std::env::temp_dir().join(format!(
        "typst-crystalline-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&path).unwrap();
    TempDir(path)
}

fn tempfile_write(name: &str, content: &str) -> TempDir {
    let dir = tempdir();
    std::fs::write(dir.path().join(name), content).unwrap();
    dir
}

#[test]
fn font_lifecycle_discovery_waits_for_first_catalog_request() {
    let dir = tempfile_write("main.typ", "#rect(width: 10pt, height: 10pt)");
    let font_dir = tempdir();
    let world = SystemWorld::new(dir.path(), "main.typ")
        .unwrap()
        .with_fonts_and_system(&[font_dir.path().to_path_buf()]);
    let data = include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf");
    let expected = crate::fonts::font_info_from_bytes(data, 0).unwrap().family;
    std::fs::write(font_dir.path().join("late.otf"), data).unwrap();
    assert_eq!(world.book().infos().last().unwrap().family, expected);
}

#[test]
fn font_lifecycle_retains_one_catalog_under_concurrent_access() {
    let dir = tempfile_write("main.typ", "text");
    let world = SystemWorld::new(dir.path(), "main.typ")
        .unwrap()
        .with_fonts_and_system(&[]);
    assert!(world.font_catalog.get().is_none());
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4).map(|_| scope.spawn(|| world.book())).collect();
        let books: Vec<_> =
            handles.into_iter().map(|handle| handle.join().unwrap()).collect();
        assert!(books.iter().all(|book| std::ptr::eq(*book, books[0])));
    });
    assert!(world.font_catalog.get().is_some());
    let book = world.book();
    assert!(std::ptr::eq(book, world.book()));
}

#[test]
fn font_lifecycle_builder_replacement_clears_previous_coverage() {
    let dir = tempfile_write("main.typ", "text");
    let font_path = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/fonts/NimbusSans-Regular.otf"
    ));
    let world = SystemWorld::new(dir.path(), "main.typ")
        .unwrap()
        .with_fonts(vec![FontSlot::new(font_path, 0)]);
    assert_eq!(world.candidates_for_char('A'), vec![0]);
    assert!(!world.coverage_cache.lock().unwrap().is_empty());
    let world = world.with_fonts(Vec::new());
    assert!(world.coverage_cache.lock().unwrap().is_empty());
    assert!(world.book().is_empty());
    assert!(world.candidates_for_char('A').is_empty());
}

#[test]
fn infra_review_concurrent_registration_has_one_identity() {
    let world = SystemWorld::for_eval(std::env::temp_dir());
    for iteration in 0..32 {
        let path = std::env::temp_dir()
            .join(format!("infra-registration-{}-{iteration}", std::process::id()));
        let barrier = std::sync::Barrier::new(32);
        let ids = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..32)
                .map(|_| {
                    let w = &world;
                    let p = &path;
                    let b = &barrier;
                    scope.spawn(move || {
                        b.wait();
                        w.register_file(p.clone())
                    })
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>()
        });
        assert!(
            ids.iter().all(|id| *id == ids[0]),
            "one path acquired multiple IDs: {ids:?}"
        );
        assert!(world.slots.lock().unwrap().contains_key(&ids[0]));
    }
}

#[test]
fn expression_source_identity_registered_sources_and_physical_names() {
    let dir = tempdir();
    let world = SystemWorld::for_stdin(dir.path(), "= Açúcar".into());
    let a = world.source_for_expression("1 + 2".into()).unwrap();
    let b = world.source_for_expression("1 + 2".into()).unwrap();
    assert_ne!(a.id(), b.id());
    assert_ne!(a.id(), world.main());
    assert_eq!(world.source(a.id()).unwrap().text(), "1 + 2");
    assert_eq!(world.file(a.id()).unwrap().as_slice(), b"1 + 2");
    assert_eq!(world.file(world.main()).unwrap().as_slice(), "= Açúcar".as_bytes());
    assert!(world.dependencies().is_empty());
    for name in ["<stdin>", "<input-expression>"] {
        let path = dir.path().join(name);
        std::fs::write(&path, "physical").unwrap();
        let id = world.register_file(path);
        assert_ne!(id, a.id());
        assert_ne!(id, world.main());
        assert_eq!(world.source(id).unwrap().text(), "physical");
    }
    assert_eq!(world.dependencies().len(), 2);
    assert_eq!(world.source(world.main()).unwrap().text(), "= Açúcar");
    let empty = SystemWorld::for_eval(dir.path());
    assert!(empty.dependencies().is_empty());
    assert!(empty.file(empty.main()).unwrap().as_slice().is_empty());
}

#[test]
fn expression_source_identity_exhaustion_never_overwrites_or_poisons() {
    let dir = tempdir();
    let world = SystemWorld::for_eval(dir.path());
    *world.next_id.lock().unwrap() = u16::MAX.into();
    let last = world.source_for_expression("42".into()).unwrap();
    assert_eq!(last.id().into_raw().get(), u16::MAX);
    assert!(
        matches!(world.source_for_expression("0".into()), Err(FileError::Other(message)) if message == "FileId counter exhausted")
    );
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        world.register_file(dir.path().join("new.typ"))
    }));
    assert!(result.is_err());
    assert_eq!(world.source(last.id()).unwrap().text(), "42");
    assert_eq!(world.file(last.id()).unwrap().as_slice(), b"42");
    assert_eq!(world.source(world.main()).unwrap().text(), "");
    assert!(world.dependencies().is_empty());
}

#[test]
fn expression_source_identity_concurrent_publication_and_reads() {
    let dir = tempdir();
    let physical = dir.path().join("shared.typ");
    std::fs::write(&physical, "physical").unwrap();
    let world = SystemWorld::for_eval(dir.path());
    let all = std::thread::scope(|scope| {
        let handles = (0..12)
            .map(|_| {
                let world = &world;
                let physical = &physical;
                scope.spawn(move || {
                    let physical_id = world.register_file(physical.clone());
                    for _ in 0..8 {
                        assert_eq!(
                            world.file(physical_id).unwrap().as_slice(),
                            b"physical"
                        );
                    }
                    let source = world.source_for_expression("42".into()).unwrap();
                    assert_eq!(world.file(source.id()).unwrap().as_slice(), b"42");
                    (physical_id, source.id())
                })
            })
            .collect::<Vec<_>>();
        handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>()
    });
    let physical_id = all[0].0;
    let unique = all
        .iter()
        .map(|pair| pair.1)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(unique.len(), 12);
    assert!(all.iter().all(|pair| pair.0 == physical_id && pair.1 != physical_id));
    assert_eq!(world.dependencies(), vec![physical]);
}
