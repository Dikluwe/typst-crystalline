//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/p1424-tests.md
//! @prompt-hash 48506c64
//! @layer L3
//!
//! Testes A/B independentes da integração HTML P1424.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use typst_core::contracts::world::World;
use typst_core::entities::compiler_features::Features;
use typst_core::entities::content::Content;
use typst_core::entities::html::HtmlFrameElem;
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

fn compile(source_text: &str, features: Features) -> Result<String, String> {
    let path = std::env::temp_dir().join(format!(
        "typst-p1424-ab-{}-{}",
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
    compile_to_html_string_with_features(&world, &source, features)
        .0
        .map_err(|errors| {
            errors.into_iter().map(|e| e.message).collect::<Vec<_>>().join("\n")
        })
}

fn html(source: &str) -> String {
    compile(source, Features::html())
        .unwrap_or_else(|error| panic!("fixture falhou: {error}"))
}

fn svg_root(document: &str) -> &str {
    let start = document.find("<svg").expect("SVG inline ausente");
    let end = document[start..].find('>').expect("raiz SVG incompleta") + start + 1;
    &document[start..end]
}

fn attr<'a>(tag: &'a str, name: &str) -> &'a str {
    let needle = format!("{name}=\"");
    let start = tag
        .find(&needle)
        .unwrap_or_else(|| panic!("atributo {name} ausente em {tag}"))
        + needle.len();
    let end = tag[start..].find('"').unwrap() + start;
    &tag[start..end]
}

fn number_with_optional_pt(value: &str) -> f64 {
    value.strip_suffix("pt").unwrap_or(value).parse().unwrap()
}

fn svg_projection(document: &str) -> (Vec<f64>, f64, f64, bool, bool) {
    let root = svg_root(document);
    let view_box = attr(root, "viewBox")
        .split_whitespace()
        .map(|part| part.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    (
        view_box,
        number_with_optional_pt(attr(root, "width")),
        number_with_optional_pt(attr(root, "height")),
        root.contains("display: block") || root.contains("display:block"),
        document.contains("#ff0000"),
    )
}

#[test]
fn p1424_html_feature_binding_e_assinatura_publica() {
    let off = compile("#html.frame([x])", Features::default()).unwrap_err();
    assert!(
        off.contains("html export is only available when `--features html` is passed")
    );

    let repr = html("#repr(html.frame)");
    assert!(repr.contains("<body><p>frame</p></body>"), "repr inesperado: {repr}");

    for (source, expected) in [
        ("#html.frame()", "missing argument: body"),
        ("#html.frame(body: [x])", "the argument `body` is positional"),
        ("#html.frame([x], [y])", "unexpected argument"),
    ] {
        let error = compile(source, Features::html()).unwrap_err();
        assert!(error.contains(expected), "{source}: {error}");
    }
}

#[test]
fn p1424_html_texto_e_nesting_formam_uma_unica_ilha_intrinseca() {
    let simple = html("#html.frame([x])");
    let nested = html("#html.frame([#html.frame([x])])");

    for document in [&simple, &nested] {
        assert_eq!(document.matches("<svg").count(), 1, "{document}");
        assert!(!document.contains("<frame"), "frame primitivo virou tag: {document}");
        assert!(!document.contains("&lt;svg"), "SVG foi escapado: {document}");
        let (view_box, width, height, display, _) = svg_projection(document);
        assert_eq!(view_box.len(), 4);
        assert!(view_box[2] > 0.0 && view_box[3] > 0.0 && width > 0.0 && height > 0.0);
        assert!(display, "frame deve ser block-level: {}", svg_root(document));
        assert!(
            document.contains(">x</text>") || document.contains(">x</tspan>"),
            "o SVG deve conter texto visível x: {document}"
        );
        assert!(width < 500.0 && height < 500.0, "canvas A4 indevido: {document}");
    }

    let a = svg_projection(&simple);
    let b = svg_projection(&nested);
    assert_eq!(a.0, b.0);
    assert!((a.1 - b.1).abs() <= 0.001);
    assert!((a.2 - b.2).abs() <= 0.001);
}

#[test]
fn p1424_html_rect_preserva_geometria_paint_e_transparencia() {
    let document = html("#html.frame([#rect(width: 10pt, height: 5pt, fill: red)])");
    assert_eq!(document.matches("<svg").count(), 1);
    let (view_box, width, height, display, red) = svg_projection(&document);
    assert_eq!(view_box.len(), 4);
    for (actual, expected) in view_box.iter().zip([0.0, 0.0, 10.0, 5.0]) {
        assert!((actual - expected).abs() <= 0.001, "viewBox inesperada: {view_box:?}");
    }
    assert!((width - 10.0).abs() <= 0.001, "width={width}");
    assert!((height - 5.0).abs() <= 0.001, "height={height}");
    assert!(display);
    assert!(red, "paint vermelho ausente: {document}");
    assert!(!document.contains("#ffffff"), "fundo de página indevido: {document}");
}

#[test]
fn p1424_html_literal_sem_frame_e_unresolved_fail_closed() {
    let literal = html("#html.elem(\"frame\")[x]");
    assert!(literal.contains("<frame>x</frame>"), "tag literal perdida: {literal}");
    assert!(!literal.contains("<svg"));

    let plain = html("sentinela");
    assert!(plain.contains("<body><p>sentinela</p></body>"));
    assert!(!plain.contains("<svg"));

    let unresolved =
        Content::HtmlFrame(Arc::new(HtmlFrameElem { body: Content::text("x") }));
    let error = typst_infra::export::export_html(&unresolved)
        .expect_err("exporter sem resolvedor não pode omitir HtmlFrame");
    assert!(
        error.message.contains("frame") || error.message.contains("resolve"),
        "diagnóstico explícito ausente: {error:?}"
    );
}
