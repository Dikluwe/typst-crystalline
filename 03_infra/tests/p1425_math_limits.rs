//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/p1425-tests.md
//! @prompt-hash c9cce1f3
//! @layer L3
//!
//! Testes A/B independentes da convergência pública `math.limits` P1425.

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
        "typst-p1425-ab-{}-{}",
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
fn p1425_bare_e_qualificado_convergem_nos_tres_estados() {
    for (bare, qualified, expected) in [
        ("#repr($limits(x)$.body)", "#repr(math.limits([x]))", "limits(body: [x])"),
        (
            "#repr($limits(x, inline: true)$.body)",
            "#repr(math.limits([x], inline: true))",
            "limits(body: [x], inline: true)",
        ),
        (
            "#repr($limits(x, inline: false)$.body)",
            "#repr(math.limits([x], inline: false))",
            "limits(body: [x], inline: false)",
        ),
    ] {
        let bare = html(bare);
        let qualified = html(qualified);
        assert!(bare.contains(expected), "bare missing {expected:?}: {bare}");
        assert!(
            qualified.contains(expected),
            "qualified missing {expected:?}: {qualified}"
        );
    }
}

#[test]
fn p1425_qualificado_preserva_cinco_diagnosticos_publicos() {
    for (source, expected) in [
        ("#math.limits()", "missing argument: body"),
        ("#math.limits([x], inline: 1)", "expected boolean, found integer"),
        ("#math.limits([x], nope: true)", "unexpected argument: nope"),
        ("#math.limits([x], [y])", "unexpected argument"),
        ("#math.limits(body: [x])", "the argument `body` is positional"),
    ] {
        let error = compile(source).unwrap_err();
        assert!(error.contains(expected), "{source}: {error}");
    }
}

#[test]
fn p1425_scripts_permanece_controle_bilateral() {
    let bare = html("#repr($scripts(x)$.body)");
    let qualified = html("#repr(math.scripts([x]))");
    for actual in [bare, qualified] {
        assert!(
            actual.contains("scripts(body: [x])"),
            "scripts morphology changed: {actual}"
        );
    }
}
