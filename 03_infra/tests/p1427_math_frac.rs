//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/p1427-tests.md
//! @prompt-hash fecdbce0
//! @layer L3
//!
//! Testes independentes da superfície pública bilateral `math.frac` P1427.

use std::path::{Path, PathBuf};

use typst_core::compiler::layout::FontMetrics;
use typst_core::contracts::world::World;
use typst_core::entities::compiler_features::Features;
use typst_core::entities::layout_types::TextStyle;
use typst_infra::font_metrics::FontBookMetrics;
use typst_infra::pipeline::{
    compile_to_html_string_with_features, compile_to_svg_string,
};
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
        "typst-p1427-ab-{}-{}",
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
    compile(source).unwrap_or_else(|error| panic!("fixture failed: {source:?}: {error}"))
}

fn svg(source_text: &str) -> String {
    let path = std::env::temp_dir().join(format!(
        "typst-p1427-svg-{}-{}",
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
    compile_to_svg_string(&world, &source)
        .0
        .unwrap_or_else(|errors| panic!("SVG fixture {source_text:?}: {errors:?}"))
}

fn math_constant_offset(bytes: &[u8], constant_offset: usize) -> usize {
    let num_tables = u16::from_be_bytes([bytes[4], bytes[5]]) as usize;
    let record = (0..num_tables)
        .map(|index| 12 + index * 16)
        .find(|&offset| &bytes[offset..offset + 4] == b"MATH")
        .expect("MATH table record");
    let table = u32::from_be_bytes([
        bytes[record + 8],
        bytes[record + 9],
        bytes[record + 10],
        bytes[record + 11],
    ]) as usize;
    let constants = u16::from_be_bytes([bytes[table + 4], bytes[table + 5]]) as usize;
    table + constants + constant_offset
}

fn productive_constants(
    bytes: &[u8],
) -> typst_core::entities::math_constants::MathConstants {
    FontBookMetrics::from_bytes(bytes)
        .expect("valid font fixture")
        .math_constants(&TextStyle::default())
}

#[test]
fn p1427_tres_rotas_publicas_convergem() {
    for (source, expected) in [
        ("#repr(math.frac([a], [b]))", "frac(num: [a], denom: [b])"),
        ("#repr($frac(a,b)$.body)", "frac(num: a, denom: b)"),
        ("#repr($a/b$.body)", "frac(num: a, denom: b)"),
    ] {
        let actual = html(source);
        assert!(actual.contains(expected), "{source}: {actual}");
    }
    let chained = html("#repr($a/b/c$.body)");
    assert!(chained.contains("frac(num: frac(num: a, denom: b), denom: c)"), "{chained}");
}

#[test]
fn p1427_reprs_preservam_estilo_explicito_e_omissao() {
    for (source, expected) in [
        ("#repr(math.frac([a], [b]))", "frac(num: [a], denom: [b])"),
        (
            "#repr(math.frac([a], [b], style: \"vertical\"))",
            "frac(num: [a], denom: [b], style: \"vertical\")",
        ),
        (
            "#repr(math.frac(style: \"skewed\", [a], [b]))",
            "frac(num: [a], denom: [b], style: \"skewed\")",
        ),
        (
            "#repr(math.frac([a], [b], style: \"horizontal\"))",
            "frac(num: [a], denom: [b], style: \"horizontal\")",
        ),
        ("#repr(math.frac(none, none))", "frac(num: [], denom: [])"),
        ("#repr(math.frac(\"a\", \"b\"))", "frac(num: [a], denom: [b])"),
    ] {
        let actual = html(source);
        assert!(actual.contains(expected), "{source}: {actual}");
        assert!(!actual.contains("deparenthesized"), "causal bits leaked: {actual}");
    }
}

#[test]
fn p1427_assinatura_publica_rejeita_dezasseis_negativos() {
    for (source, expected) in [
        ("#math.frac()", "missing argument: num"),
        ("#math.frac([a])", "missing argument: denom"),
        ("#math.frac([a], [b], [c])", "unexpected argument"),
        ("#math.frac(num: [a], [b])", "the argument `num` is positional"),
        ("#math.frac([a], denom: [b])", "the argument `denom` is positional"),
        ("#math.frac([a], [b], foo: 1)", "unexpected argument: foo"),
        ("#math.frac([a], [b], style: \"vertical\", style: \"skewed\")", "duplicate"),
        ("#math.frac([a], [b], style: \"diagonal\")", "vertical"),
        ("#math.frac([a], [b], style: 1)", "integer"),
        ("#math.frac(1, [b])", "expected content, found integer"),
        ("#math.frac([a], 1)", "expected content, found integer"),
        ("$frac(a)$", "missing argument: denom"),
        ("$frac(a,b,c)$", "unexpected argument"),
        ("$frac(a,b,foo: 1)$", "unexpected argument"),
        ("#set math.frac(style: \"diagonal\"); $a/b$", "vertical"),
        ("#set math.frac(style: 1); $a/b$", "integer"),
    ] {
        let error = compile(source).unwrap_err();
        assert!(
            error.contains(expected),
            "{source}: expected {expected:?}, got {error:?}"
        );
    }
}

#[test]
fn p1427_parenteses_causais_so_aparecem_no_horizontal() {
    let numerator = svg("#set math.frac(style: \"horizontal\"); $(a)/b$");
    let denominator = svg("#set math.frac(style: \"horizontal\"); $a/(b)$");
    let neither = svg("#set math.frac(style: \"horizontal\"); $a/b$");
    let both = svg("#set math.frac(style: \"horizontal\"); $(a)/(b)$");
    assert_ne!(numerator, neither, "numerator bit must affect morphology");
    assert_ne!(denominator, neither, "denominator bit must affect morphology");
    assert_ne!(both, numerator, "denominator protection missing");
    assert_ne!(both, denominator, "numerator protection missing");
}

#[test]
fn p1427_extrai_skewed_gaps_newcmath() {
    let bytes = include_bytes!("../fixtures/fonts/NewCMMath-Book.otf");
    let face = ttf_parser::Face::parse(bytes, 0).unwrap();
    let constants = face
        .tables()
        .math
        .and_then(|math| math.constants)
        .expect("MATH constants");
    assert_eq!(constants.skewed_fraction_vertical_gap().value, 96);
    assert_eq!(constants.skewed_fraction_horizontal_gap().value, 350);

    let productive = productive_constants(bytes);
    assert_eq!(productive.skewed_fraction_vertical_gap, 96.0);
    assert_eq!(productive.skewed_fraction_horizontal_gap, 350.0);

    for constant_offset in [156usize, 160usize] {
        let mut zeroed = bytes.to_vec();
        let offset = math_constant_offset(&zeroed, constant_offset);
        zeroed[offset..offset + 2].copy_from_slice(&0i16.to_be_bytes());
        let face = ttf_parser::Face::parse(&zeroed, 0).unwrap();
        let raw = face.tables().math.unwrap().constants.unwrap();
        let extracted = productive_constants(&zeroed);
        if constant_offset == 156 {
            assert_eq!(raw.skewed_fraction_horizontal_gap().value, 0);
            assert_eq!(extracted.skewed_fraction_horizontal_gap, 0.0);
            assert_eq!(extracted.skewed_fraction_vertical_gap, 96.0);
        } else {
            assert_eq!(raw.skewed_fraction_vertical_gap().value, 0);
            assert_eq!(extracted.skewed_fraction_vertical_gap, 0.0);
            assert_eq!(extracted.skewed_fraction_horizontal_gap, 350.0);
        }
    }
}

#[test]
fn p1427_binom_ignora_style_de_frac() {
    let before = html("#repr($binom(n,k)$.body)");
    let after = html("#set math.frac(style: \"skewed\"); #repr($binom(n,k)$.body)");
    let expected = "binom(upper: [n], lower: ([k],))";
    assert!(before.contains(expected), "{before}");
    assert!(after.contains(expected), "{after}");
    assert!(!after.contains("style:") && !after.contains("frac("), "{after}");
}
