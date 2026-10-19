//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/p1426-tests.md
//! @prompt-hash c376da44
//! @layer L3
//!
//! Testes independentes da superfície pública bilateral `math.root` P1426.

use std::path::{Path, PathBuf};

use typst_core::contracts::world::World;
use typst_core::entities::compiler_features::Features;
use typst_infra::pipeline::compile_to_html_string_with_features;
use typst_infra::world::SystemWorld;

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

fn compile(source_text: &str) -> Result<String, String> {
    let path = std::env::temp_dir().join(format!(
        "typst-p1426-ab-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
    ));
    std::fs::create_dir_all(&path).unwrap();
    let dir = TempDir(path);
    std::fs::write(dir.path().join("main.typ"), source_text).unwrap();
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let source = world.source(world.main()).unwrap();
    compile_to_html_string_with_features(&world, &source, Features::html())
        .0
        .map_err(|errors| {
            errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("\n")
        })
}

fn html(source: &str) -> String {
    compile(source).unwrap_or_else(|error| panic!("fixture failed: {error}"))
}

#[test]
fn p1426_root_bare_e_qualificado_convergem() {
    let bare = html("#repr($root(3, x)$.body)");
    let qualified = html("#repr(math.root([3], [x]))");
    let expected = "root(index: [3], radicand: [x])";
    assert!(bare.contains(expected), "bare missing {expected:?}: {bare}");
    assert!(qualified.contains(expected), "qualified missing {expected:?}: {qualified}");
}

#[test]
fn p1426_tres_estados_publicos_nao_colapsam() {
    for (source, expected) in [
        ("#repr(math.sqrt([x]))", "root(radicand: [x])"),
        ("#repr(math.root(none, [x]))", "root(index: none, radicand: [x])"),
        ("#repr($root(, x)$.body)", "root(index: [], radicand: [x])"),
    ] {
        let actual = html(source);
        assert!(actual.contains(expected), "{source}: {actual}");
    }
}

#[test]
fn p1426_assinatura_publica_preserva_diagnosticos() {
    for (source, expected) in [
        ("#math.root()", "missing argument: radicand"),
        ("#math.root([x])", "missing argument: radicand"),
        ("#math.root([3], [x], [y])", "unexpected argument"),
        ("#math.root(3, [x])", "expected content, found integer"),
        ("#math.root([3], 1)", "expected content, found integer"),
        ("#math.root([3], radicand: [x])", "the argument `radicand` is positional"),
        ("#math.root([3], [x], foo: 1)", "unexpected argument: foo"),
        ("$root(x)$", "missing argument: radicand"),
    ] {
        let error = compile(source).unwrap_err();
        assert!(error.contains(expected), "{source}: {error}");
    }
}

#[test]
fn p1426_sqrt_permanece_controle_bilateral() {
    let bare = html("#repr($sqrt(x)$.body)");
    let qualified = html("#repr(math.sqrt([x]))");
    assert!(bare.contains("root(radicand: x)"), "bare sqrt morphology changed: {bare}");
    assert!(
        qualified.contains("root(radicand: [x])"),
        "qualified sqrt morphology changed: {qualified}"
    );
    assert!(!bare.contains("index: none"), "bare sqrt exposed presence: {bare}");
    assert!(
        !qualified.contains("index: none"),
        "qualified sqrt exposed presence: {qualified}"
    );
}
