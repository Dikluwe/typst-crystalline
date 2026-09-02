//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/tests/p1292_contract.md
//! @prompt-hash 05d0eea8
//! @layer L4
//! @updated 2026-09-01
//!
//! Oraculos black-box independentes do Passo 1292.
//!
//! Congelados no baseline 0eb39f8 antes da implementacao candidata.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_typst");

struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let path = env::temp_dir().join(format!(
            "typst-p1292-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id(),
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create P1292 scratch directory");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn eval(expr: &str) -> Output {
    Command::new(BIN)
        .args(["eval", expr, "--format", "json"])
        .output()
        .expect("run crystalline eval")
}

fn diagnostic(output: &Output) -> String {
    format!(
        "exit={:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}

fn assert_string(expr: &str, expected: &str) {
    let output = eval(expr);
    assert!(output.status.success(), "{}", diagnostic(&output));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        serde_json::to_string(expected).unwrap(),
    );
}

fn assert_repr_pair(
    syntax_expr: &str,
    qualified_expr: &str,
    expected: &str,
    label: &str,
) {
    let syntax = eval(syntax_expr);
    let qualified = eval(qualified_expr);
    assert!(syntax.status.success(), "{}", diagnostic(&syntax));
    assert!(qualified.status.success(), "{}", diagnostic(&qualified));
    let syntax = String::from_utf8_lossy(&syntax.stdout).trim().to_owned();
    let qualified = String::from_utf8_lossy(&qualified.stdout).trim().to_owned();
    let expected = serde_json::to_string(expected).unwrap();
    assert_eq!(syntax, expected, "{label} syntax morphology");
    assert_eq!(qualified, expected, "{label} qualified morphology");
    assert_eq!(syntax, qualified, "{label} syntax/qualified repr mismatch");
}

fn assert_error(expr: &str, expected: &str) {
    let output = eval(expr);
    assert!(!output.status.success(), "invalid expression unexpectedly succeeded");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(&format!("error: {expected}\n")),
        "wrong public diagnostic; expected {expected:?}\n{}",
        diagnostic(&output),
    );
}

fn compile(name: &str, source: &str, extension: &str) -> (Scratch, PathBuf, Output) {
    let scratch = Scratch::new(name);
    let input = scratch.0.join("main.typ");
    let output = scratch.0.join(format!("main.{extension}"));
    fs::write(&input, source).expect("write P1292 fixture");
    let result = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .output()
        .expect("run crystalline compile");
    (scratch, output, result)
}

fn compile_svg(name: &str, source: &str) -> String {
    let (_scratch, output, result) = compile(name, source, "svg");
    assert!(result.status.success(), "{}", diagnostic(&result));
    fs::read_to_string(output).expect("read candidate SVG")
}

fn tag_attr(tag: &str, name: &str) -> String {
    let needle = format!("{name}=\"");
    let start = tag
        .find(&needle)
        .unwrap_or_else(|| panic!("missing attribute {name:?} in {tag}"))
        + needle.len();
    let tail = &tag[start..];
    let end = tail.find('"').expect("closed XML attribute");
    tail[..end].to_owned()
}

fn parse_pt(value: &str) -> f64 {
    value
        .strip_suffix("pt")
        .unwrap_or(value)
        .parse()
        .unwrap_or_else(|_| panic!("invalid point value {value:?}"))
}

fn assert_close(actual: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{label}: expected {expected}±{tolerance}, got {actual}",
    );
}

fn svg_root_size(svg: &str) -> (f64, f64) {
    let end = svg.find('>').expect("SVG root end");
    let root = &svg[..=end];
    (parse_pt(&tag_attr(root, "width")), parse_pt(&tag_attr(root, "height")))
}

#[derive(Clone, Copy)]
struct SvgTransform {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

impl SvgTransform {
    const IDENTITY: Self = Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 };

    fn compose(self, inner: Self) -> Self {
        Self {
            a: self.a * inner.a + self.c * inner.b,
            b: self.b * inner.a + self.d * inner.b,
            c: self.a * inner.c + self.c * inner.d,
            d: self.b * inner.c + self.d * inner.d,
            e: self.a * inner.e + self.c * inner.f + self.e,
            f: self.b * inner.e + self.d * inner.f + self.f,
        }
    }
}

fn optional_tag_attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!("{name}=\"");
    let start = tag.find(&needle)? + needle.len();
    let tail = &tag[start..];
    let end = tail.find('"').expect("closed XML attribute");
    Some(&tail[..end])
}

fn parse_svg_transform(value: &str) -> SvgTransform {
    let mut transform = SvgTransform::IDENTITY;
    let mut rest = value.trim();
    while !rest.is_empty() {
        let open = rest.find('(').expect("SVG transform opening parenthesis");
        let name = rest[..open].trim();
        let tail = &rest[open + 1..];
        let close = tail.find(')').expect("SVG transform closing parenthesis");
        let operands = tail[..close]
            .split(|character: char| character == ',' || character.is_whitespace())
            .filter(|operand| !operand.is_empty())
            .map(|operand| operand.parse::<f64>().expect("SVG transform operand"))
            .collect::<Vec<_>>();
        let next = match name {
            "translate" => {
                assert!(
                    matches!(operands.len(), 1 | 2),
                    "translate expects one or two operands"
                );
                SvgTransform {
                    e: operands[0],
                    f: operands.get(1).copied().unwrap_or(0.0),
                    ..SvgTransform::IDENTITY
                }
            }
            "matrix" => {
                assert_eq!(operands.len(), 6, "matrix expects six operands");
                SvgTransform {
                    a: operands[0],
                    b: operands[1],
                    c: operands[2],
                    d: operands[3],
                    e: operands[4],
                    f: operands[5],
                }
            }
            _ => panic!("unsupported SVG transform {name:?}"),
        };
        transform = transform.compose(next);
        rest = tail[close + 1..].trim_start_matches(|character: char| {
            character == ',' || character.is_whitespace()
        });
    }
    transform
}

fn svg_glyph_baselines(svg: &str) -> Vec<f64> {
    struct Frame {
        transform: SvgTransform,
        glyph_group: bool,
        captured: bool,
    }

    let mut baselines = Vec::new();
    let mut stack = Vec::<Frame>::new();
    let mut cursor = 0;
    while let Some(relative_start) = svg[cursor..].find('<') {
        let start = cursor + relative_start;
        let end = start + svg[start..].find('>').expect("complete SVG tag");
        let tag = &svg[start + 1..end];
        cursor = end + 1;

        if tag.starts_with('/') {
            stack.pop().expect("balanced SVG closing tag");
            continue;
        }
        if tag.starts_with(['!', '?']) {
            continue;
        }

        let self_closing = tag.trim_end().ends_with('/');
        let name = tag
            .split(|character: char| character.is_whitespace() || character == '/')
            .next()
            .expect("SVG tag name");
        let raw_transform = optional_tag_attr(tag, "transform");
        let local = raw_transform
            .map(parse_svg_transform)
            .unwrap_or(SvgTransform::IDENTITY);
        let global = stack
            .last()
            .map(|frame| frame.transform)
            .unwrap_or(SvgTransform::IDENTITY)
            .compose(local);

        if name == "use" {
            if let Some(frame) = stack
                .iter_mut()
                .rev()
                .find(|frame| frame.glyph_group && !frame.captured)
            {
                baselines.push(frame.transform.f);
                frame.captured = true;
            }
        }

        if !self_closing {
            stack.push(Frame {
                transform: global,
                glyph_group: name == "g"
                    && raw_transform
                        .is_some_and(|value| value.trim_start().starts_with("matrix(")),
                captured: false,
            });
        }
    }
    assert!(!baselines.is_empty(), "SVG contains no positioned glyphs");
    baselines
}

fn unique_baselines(svg: &str) -> Vec<f64> {
    let mut values = svg_glyph_baselines(svg);
    values.sort_by(f64::total_cmp);
    values.dedup_by(|left, right| (*left - *right).abs() < 0.0001);
    values
}

fn compile_pdf_bbox(name: &str, source: &str) -> String {
    let (scratch, pdf, result) = compile(name, source, "pdf");
    assert!(result.status.success(), "{}", diagnostic(&result));
    let html = scratch.0.join("bbox.html");
    let bbox = Command::new("pdftotext")
        .args(["-bbox"])
        .arg(&pdf)
        .arg(&html)
        .output()
        .expect("pdftotext -bbox must be available");
    assert!(bbox.status.success(), "{}", diagnostic(&bbox));
    fs::read_to_string(html).expect("read pdftotext bbox HTML")
}

fn bbox_page_count(html: &str) -> usize {
    html.matches("<page ").count()
}

fn bbox_word(html: &str, token: &str) -> (usize, f64) {
    let needle = format!(">{token}</word>");
    let occurrences = html.match_indices(&needle).collect::<Vec<_>>();
    assert_eq!(occurrences.len(), 1, "token {token:?} must occur exactly once");
    let at = occurrences[0].0;
    let page = html[..at].matches("<page ").count();
    let start = html[..at].rfind("<word ").expect("word tag start");
    let end = html[start..].find('>').expect("word tag end") + start;
    let word = &html[start..=end];
    (page, tag_attr(word, "yMin").parse().expect("bbox yMin"))
}

fn bbox_words(html: &str) -> Vec<(usize, String, String, String)> {
    let mut words = Vec::new();
    for (page_index, page) in html.split("<page ").skip(1).enumerate() {
        let mut rest = page;
        while let Some(start) = rest.find("<word ") {
            rest = &rest[start..];
            let open_end = rest.find('>').expect("word opening end");
            let close = rest.find("</word>").expect("word closing");
            let tag = &rest[..=open_end];
            words.push((
                page_index + 1,
                rest[open_end + 1..close].to_owned(),
                tag_attr(tag, "xMin"),
                tag_attr(tag, "yMin"),
            ));
            rest = &rest[close + "</word>".len()..];
        }
    }
    words
}

fn svg_lines(svg: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut rest = svg;
    while let Some(start) = rest.find("<line ") {
        rest = &rest[start..];
        let end = rest.find("/>").expect("complete SVG line element") + 2;
        lines.push(&rest[..end]);
        rest = &rest[end..];
    }
    lines.sort_unstable();
    lines
}

#[test]
fn p1292_a_cancel_surface_repr_and_errors() {
    assert_string("repr(math.cancel)", "cancel");
    assert_string("repr(math.cancel([x]))", "cancel(body: [x])");
    assert_string(
        "repr(math.cancel([x], length: 80% + 1em, inverted: true, cross: true, angle: 45deg, stroke: red + 1pt, background: true))",
        "cancel(\n  body: [x],\n  length: 80% + 1em,\n  inverted: true,\n  cross: true,\n  angle: 45deg,\n  stroke: 1pt + rgb(\"#ff4136\"),\n  background: true,\n)",
    );
    assert_error("repr(math.cancel())", "missing argument: body");
    assert_error("repr(math.cancel([x], nope: 1))", "unexpected argument: nope");
    assert_error(
        "repr(math.cancel([x], length: 1))",
        "expected relative length, found integer",
    );
    assert_error(
        "repr(math.cancel([x], angle: 1))",
        "expected angle, function, or auto, found integer",
    );
}

#[test]
fn p1292_a_cancel_callback_cross_context_span_and_background() {
    let callback = compile_svg(
        "cancel-callback",
        "#set page(width:100pt,height:100pt,margin:10pt)\n#set text(size:19pt)\n$cancel(x, angle: a => context if text.size == 19pt { 0deg } else { 90deg })$",
    );
    let explicit = compile_svg(
        "cancel-explicit",
        "#set page(width:100pt,height:100pt,margin:10pt)\n#set text(size:19pt)\n$cancel(x, angle: 0deg)$",
    );
    assert_eq!(callback, explicit, "callback must observe effective context");

    let cross = compile_svg(
        "cancel-cross",
        "#set page(width:100pt,height:100pt,margin:10pt)\n$cancel(x, cross:#true, inverted:#true, angle:a => a)$",
    );
    assert_eq!(svg_lines(&cross).len(), 2, "cross must emit two strokes");

    let foreground = compile_svg(
        "cancel-foreground",
        "#set page(width:100pt,height:100pt,margin:10pt)\n$cancel(x, angle:0deg, background:#false)$",
    );
    let background = compile_svg(
        "cancel-background",
        "#set page(width:100pt,height:100pt,margin:10pt)\n$cancel(x, angle:0deg, background:#true)$",
    );
    assert_ne!(foreground, background, "background must change paint order");

    let scratch = Scratch::new("cancel-invalid-callback");
    let input = scratch.0.join("main.typ");
    let output = scratch.0.join("main.pdf");
    fs::write(&input, "\n\n\n$cancel(x, angle: a => 1)$").unwrap();
    let result = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(!output.exists(), "callback error must not export a document");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("4:"), "callback error lost call span: {stderr}");
    assert!(!stderr.contains("panicked at") && !stderr.contains("thread '"));
}

#[test]
fn p1292_b_math_underline_surface_identity_and_errors() {
    assert_string("repr(math.underline)", "underline");
    assert_string("repr(math.underline([x]))", "underline(body: [x])");
    let identity = eval("math.underline == underline");
    assert!(identity.status.success(), "{}", diagnostic(&identity));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&identity.stdout).unwrap(),
        serde_json::Value::Bool(false),
        "identity must be a boolean, not the string \"false\"",
    );
    assert_error("repr(math.underline())", "missing argument: body");
    assert_error("repr(math.underline(1))", "expected content, found integer");
    assert_error("repr(math.underline([x], [y]))", "unexpected argument");
    assert_error("repr(math.underline([x], nope: 1))", "unexpected argument: nope");
}

#[test]
fn p1292_b_math_underline_math_constants_styles_and_italic_correction() {
    let cases = [
        ("text", "$underline(x)$", Some(6.292), None, 7.513),
        ("display", "$ underline(x) $", Some(6.292), None, 7.359),
        ("script", "$x_(underline(y))$", None, Some("$x_(y)$"), 8.459),
        ("cramped", "$1 / underline(y)$", None, Some("$1/y$"), 9.537),
    ];
    for (name, body, expected_width, base_body, expected_height) in cases {
        let svg = compile_svg(
            &format!("underline-{name}"),
            &format!("#set page(width:auto,height:auto,margin:0pt)\n{body}"),
        );
        let (width, height) = svg_root_size(&svg);
        if let Some(expected_width) = expected_width {
            assert_close(width, expected_width, 0.0001, &format!("{name} width"));
        }
        if let Some(base_body) = base_body {
            let base_svg = compile_svg(
                &format!("base-{name}"),
                &format!("#set page(width:auto,height:auto,margin:0pt)\n{base_body}"),
            );
            let (base_width, _) = svg_root_size(&base_svg);
            assert_close(
                width - base_width,
                0.0,
                0.0001,
                &format!("{name} underline width delta"),
            );
        }
        assert_close(height, expected_height, 0.0001, &format!("{name} inline extent"));
        let lines = svg_lines(&svg);
        if name == "cramped" {
            assert_eq!(lines.len(), 2, "cramped must emit underline and fraction bar");
            let mut widths = lines
                .iter()
                .map(|line| {
                    (parse_pt(&tag_attr(line, "x2")) - parse_pt(&tag_attr(line, "x1")))
                        .abs()
                })
                .collect::<Vec<_>>();
            widths.sort_by(f64::total_cmp);
            assert_close(widths[0], 4.4583, 0.0001, "cramped underline width");
            assert_close(widths[1], 4.5276, 0.0001, "cramped fraction bar width");
        } else {
            assert_eq!(lines.len(), 1, "{name} underline must emit exactly one rule",);
        }
    }
}

#[test]
fn p1292_b_plain_mixed_baselines_and_following_line_advance() {
    let plain = compile_svg(
        "plain-x-control",
        "#set page(width:auto,height:auto,margin:0pt)\n$x$",
    );
    let mixed_plain = compile_svg(
        "mixed-plain-control",
        "#set page(width:auto,height:auto,margin:0pt)\nA $x$ B",
    );
    let mixed_underline = compile_svg(
        "mixed-underline",
        "#set page(width:auto,height:auto,margin:0pt)\nA $underline(x)$ B",
    );
    let two_lines_plain = compile_svg(
        "two-lines-plain-control",
        concat!(
            "#set page(width:auto,height:auto,margin:0pt)\n",
            "A $x$ B \\\n",
            "C plain D",
        ),
    );
    let two_lines_underline = compile_svg(
        "two-lines-underline",
        concat!(
            "#set page(width:auto,height:auto,margin:0pt)\n",
            "A $underline(x)$ B \\\n",
            "C plain D",
        ),
    );

    let plain_size = svg_root_size(&plain);
    let mixed_plain_size = svg_root_size(&mixed_plain);
    let mixed_underline_size = svg_root_size(&mixed_underline);
    let two_lines_plain_size = svg_root_size(&two_lines_plain);
    let two_lines_underline_size = svg_root_size(&two_lines_underline);
    let mixed_plain_baselines = unique_baselines(&mixed_plain);
    let mixed_underline_baselines = unique_baselines(&mixed_underline);
    let two_lines_plain_baselines = unique_baselines(&two_lines_plain);
    let two_lines_underline_baselines = unique_baselines(&two_lines_underline);

    assert_close(plain_size.0, 6.292, 0.0001, "plain x width");
    assert_close(plain_size.1, 7.513, 0.0001, "plain x normalized extent");
    assert!(svg_lines(&plain).is_empty(), "plain x must not emit a rule");

    for (name, size, baselines) in [
        ("mixed plain", mixed_plain_size, mixed_plain_baselines),
        ("mixed underline", mixed_underline_size, mixed_underline_baselines),
    ] {
        assert_close(size.0, 25.905, 0.0001, &format!("{name} width"));
        assert_close(size.1, 7.513, 0.0001, &format!("{name} extent"));
        assert_eq!(baselines.len(), 1, "{name} glyphs must share one baseline");
        assert_close(baselines[0], 7.513, 0.0001, &format!("{name} baseline"));
    }
    assert_eq!(svg_lines(&mixed_underline).len(), 1);

    for (name, size, baselines) in [
        ("two lines plain", two_lines_plain_size, two_lines_plain_baselines),
        ("two lines underline", two_lines_underline_size, two_lines_underline_baselines),
    ] {
        assert_close(size.0, 42.9, 0.0001, &format!("{name} width"));
        assert_close(size.1, 21.901, 0.0001, &format!("{name} extent"));
        assert_eq!(baselines.len(), 2, "{name} must expose two baselines");
        assert_close(baselines[0], 7.513, 0.0001, &format!("{name} first baseline"));
        assert_close(baselines[1], 21.901, 0.0001, &format!("{name} following baseline"));
    }
}

#[test]
fn p1292_b_italic_f_preserves_frame_and_shortens_only_rule() {
    let italic = compile_svg(
        "underline-italic",
        "#set page(width:auto,height:auto,margin:0pt)\n$underline(f)$",
    );
    let size = svg_root_size(&italic);
    let line = svg_lines(&italic);
    assert_eq!(line.len(), 1, "underline must emit exactly one rule");
    let x1 = parse_pt(&tag_attr(line[0], "x1"));
    let x2 = parse_pt(&tag_attr(line[0], "x2"));
    let rule_width = x2 - x1;
    assert_close(size.0, 6.38, 0.0001, "italic f frame width");
    assert_close(size.1, 7.513, 0.0001, "italic f normalized frame height");
    assert_close(rule_width, 5.39, 0.0001, "italic f rule width");
    assert!(
        size.0 > rule_width,
        "italic correction must shorten only the rule, not the frame",
    );
}

#[test]
fn p1292_b_underbar_fallback_constants_are_not_generic_line_defaults() {
    let c = typst_core::entities::math_constants::MathConstants::fallback();
    assert_eq!(c.underbar_vertical_gap, 166.0);
    assert_eq!(c.underbar_rule_thickness, 66.0);
    assert_eq!(c.underbar_extra_descender, 66.0);
    let at_12pt =
        |du: f64| c.to_pt(du, typst_core::entities::layout_types::Pt(12.0)).val();
    assert!((at_12pt(c.underbar_vertical_gap) - 1.992).abs() < 1e-9);
    assert!((at_12pt(c.underbar_rule_thickness) - 0.792).abs() < 1e-9);
    assert!((at_12pt(c.underbar_extra_descender) - 0.792).abs() < 1e-9);
}

#[test]
fn p1292_c_vec_cardinality_defaults_repr_and_errors() {
    assert_string("repr(math.vec)", "vec");
    assert_string("repr(math.vec())", "vec(children: ())");
    assert_string("repr(math.vec([a]))", "vec(children: ([a],))");
    assert_string("repr(math.vec([a], [b]))", "vec(children: ([a], [b]))");
    assert_string(
        "repr(math.vec([a], [b], delim: \"[\", align: left, gap: 1em))",
        "vec(\n  delim: (\"[\", \"]\"),\n  align: left,\n  gap: 0% + 1em,\n  children: ([a], [b]),\n)",
    );
    assert_error("repr(math.vec([a], nope: 1))", "unexpected argument: nope");
    assert_error(
        "repr(math.vec([a], gap: 1))",
        "expected relative length, found integer",
    );
    assert_error("repr(math.vec([a], align: 1))", "expected alignment, found integer");
    assert_error(
        "repr(math.vec([a], delim: 1))",
        "expected array, none, symbol, or string, found integer",
    );
}

#[test]
fn p1292_c_vec_gap_finite_auto_absolute_and_mixed() {
    let cases = [
        ("finite-zero", "0%", "100pt", 7.6692),
        ("finite-ten", "10%", "100pt", 17.6692),
        ("finite-em", "1em", "100pt", 18.6692),
        ("finite-mixed", "10% + 1em", "100pt", 28.6692),
        ("auto-zero", "0%", "auto", 7.6692),
        ("auto-ten", "10%", "auto", 7.6692),
        ("auto-em", "1em", "auto", 18.6692),
        ("auto-mixed", "10% + 1em", "auto", 18.6692),
    ];
    for (name, gap, region, expected_delta) in cases {
        let svg = compile_svg(
            &format!("vec-gap-{name}"),
            &format!(
                "#set page(width:auto,height:{region},margin:0pt)\n\
                 #math.equation(math.vec([A], [B], gap: {gap}))"
            ),
        );
        let (root_width, root_height) = svg_root_size(&svg);
        assert!(
            root_width.is_finite() && root_height.is_finite(),
            "{name} non-finite root"
        );
        if region == "100pt" {
            assert_close(root_height, 100.0, 0.0001, &format!("{name} root height"));
        } else {
            assert!(
                root_height > 0.0,
                "{name} auto root must have positive finite height"
            );
        }
        let baselines = unique_baselines(&svg);
        assert_eq!(baselines.len(), 2, "{name} must expose exactly two child baselines");
        assert_close(
            baselines[1] - baselines[0],
            expected_delta,
            0.0001,
            &format!("{name} child baseline delta"),
        );
    }

    assert_repr_pair(
        "repr($vec(a,b)$.body)",
        "repr(math.vec([a],[b]))",
        "vec(children: ([a], [b]))",
        "identifier children",
    );
    assert_repr_pair(
        "repr($vec(1,23)$.body)",
        "repr(math.vec([1],[23]))",
        "vec(children: ([1], [23]))",
        "numeric MathText children",
    );
    assert_repr_pair(
        "repr($vec(alpha,beta)$.body)",
        "repr(math.vec([α],[β]))",
        "vec(children: ([α], [β]))",
        "resolved multigrapheme children",
    );
    assert_repr_pair(
        "repr($vec(\"foo\",\"bar\")$.body)",
        "repr(math.vec([foo],[bar]))",
        "vec(children: ([foo], [bar]))",
        "quoted text children",
    );
    let structured_syntax = eval("repr($vec((a+b),(c))$.body)");
    let structured_qualified = eval("repr(math.vec($(a+b)$.body,$(c)$.body))");
    assert!(structured_syntax.status.success(), "{}", diagnostic(&structured_syntax));
    assert!(
        structured_qualified.status.success(),
        "{}",
        diagnostic(&structured_qualified)
    );
    let structured_syntax =
        String::from_utf8_lossy(&structured_syntax.stdout).trim().to_owned();
    let structured_qualified = String::from_utf8_lossy(&structured_qualified.stdout)
        .trim()
        .to_owned();
    assert_eq!(
        structured_syntax, structured_qualified,
        "structured syntax/qualified repr must converge"
    );
    assert_ne!(
        structured_syntax,
        serde_json::to_string("vec(children: ([(a+b)], [(c)]))").unwrap(),
        "direct leaf bracket projection must not recurse into structured children"
    );
    assert_repr_pair(
        "repr($vec(#strong([a]),#emph([b]))$.body)",
        "repr(math.vec(strong([a]),emph([b])))",
        "vec(children: (strong(body: [a]), emph(body: [b])))",
        "markup children",
    );

    let syntax = compile_svg(
        "vec-syntax",
        "#set page(width:100pt,height:100pt,margin:10pt)\n$vec(a,b)$",
    );
    let none = compile_svg(
        "vec-delim-none",
        "#set page(width:100pt,height:100pt,margin:10pt)\n$vec(a,b, delim:#none)$",
    );
    assert_ne!(syntax, none, "delim must affect visible fences");
    let left = compile_svg(
        "vec-align-left",
        "#set page(width:100pt,height:100pt,margin:10pt)\n$vec(a, wide, align:left)$",
    );
    let right = compile_svg(
        "vec-align-right",
        "#set page(width:100pt,height:100pt,margin:10pt)\n$vec(a, wide, align:right)$",
    );
    assert_ne!(left, right, "align must change unequal-row placement");
}

#[test]
fn p1292_d_place_flush_namespace_repr_and_errors() {
    assert_string("repr(place)", "place");
    assert_string("repr(place.flush)", "flush");
    assert_string("repr(place.flush())", "flush()");
    assert_string("repr(place.with(dx: 1pt).flush)", "flush");
    assert_string("repr(place.with(dx: 1pt).flush())", "flush()");
    assert_error("repr(place.flush([x]))", "unexpected argument");
    assert_error("repr(place.flush(nope: 1))", "unexpected argument: nope");
}

#[test]
fn p1292_d_flush_prefix_suffix_clearance_nested_and_noop() {
    let transport = compile_pdf_bbox(
        "flush-transport-control",
        "#set page(width:100pt,height:100pt,margin:0pt)\n\
         #place([PAGE_ONE])\n\
         #pagebreak()\n\
         #place([PAGE_TWO])\n\
         #pagebreak()\n\
         #place([PAGE_THREE])",
    );
    assert_eq!(bbox_page_count(&transport), 3, "PDF transport lost pages");
    assert_eq!(bbox_word(&transport, "PAGE_ONE").0, 1);
    assert_eq!(bbox_word(&transport, "PAGE_TWO").0, 2);
    assert_eq!(bbox_word(&transport, "PAGE_THREE").0, 3);

    let actual = compile_pdf_bbox(
        "flush-prefix-suffix",
        "#set page(width:100pt,height:100pt,margin:0pt)\n\
         #block(height:30pt)[before]\n\
         #place(bottom,float:true,block(width:100pt,height:80pt)[#place([FLOAT_BEFORE])])\n\
         #place.flush()\n\
         #block(height:10pt)[after #place([AFTER_MARKER])]\n\
         #place(bottom,float:true,block(width:100pt,height:10pt)[#place([FLOAT_AFTER])])",
    );
    assert_eq!(bbox_page_count(&actual), 3);
    let float_before = bbox_word(&actual, "FLOAT_BEFORE");
    let after = bbox_word(&actual, "AFTER_MARKER");
    let float_after = bbox_word(&actual, "FLOAT_AFTER");
    assert_eq!(float_before.0, 2);
    assert_close(float_before.1, 17.404, 0.002, "FLOAT_BEFORE yMin");
    assert_eq!(after.0, 3);
    assert_close(after.1, 4.642, 0.002, "AFTER_MARKER yMin");
    assert_eq!(float_after.0, 3);
    assert_close(float_after.1, 87.404, 0.002, "FLOAT_AFTER yMin");
    assert!(float_before.0 < after.0);
    assert!(after.0 < float_after.0 || after.1 < float_after.1);

    let no_flush = compile_pdf_bbox(
        "flush-absent-control",
        "#set page(width:100pt,height:100pt,margin:0pt)\n\
         #block(height:30pt)[before]\n\
         #place(bottom,float:true,block(width:100pt,height:80pt)[#place([FLOAT_BEFORE])])\n\
         #block(height:10pt)[after #place([AFTER_MARKER])]",
    );
    assert_eq!(bbox_page_count(&no_flush), 2);
    let no_flush_after = bbox_word(&no_flush, "AFTER_MARKER");
    assert_eq!(no_flush_after.0, 1);
    assert_close(no_flush_after.1, 47.842, 0.002, "no-flush AFTER_MARKER yMin");

    let mixed = compile_pdf_bbox(
        "flush-top-bottom",
        "#set page(width:100pt,height:100pt,margin:0pt)\n\
         #block(height:30pt)[before]\n\
         #place(top,float:true,block(width:100pt,height:20pt)[#place([TOP_FLOAT])])\n\
         #place(bottom,float:true,block(width:100pt,height:30pt)[#place([BOTTOM_FLOAT])])\n\
         #place.flush()\n\
         #block(height:10pt)[after #place([AFTER_MIXED])]",
    );
    let top = bbox_word(&mixed, "TOP_FLOAT");
    let bottom = bbox_word(&mixed, "BOTTOM_FLOAT");
    let mixed_after = bbox_word(&mixed, "AFTER_MIXED");
    assert_eq!(top.0, 1);
    assert_close(top.1, -2.596, 0.002, "TOP_FLOAT yMin");
    assert_eq!(bottom.0, 2);
    assert_close(bottom.1, 67.404, 0.002, "BOTTOM_FLOAT yMin");
    assert_eq!(mixed_after.0, 2);
    assert_close(mixed_after.1, 4.642, 0.002, "AFTER_MIXED yMin");
    assert!(top.0 < bottom.0 && top.0 < mixed_after.0);

    let plain = compile_pdf_bbox(
        "flush-no-floats-control",
        "#set page(width:100pt,height:100pt,margin:0pt)\nalpha beta",
    );
    let flushed = compile_pdf_bbox(
        "flush-no-floats",
        "#set page(width:100pt,height:100pt,margin:0pt)\nalpha#place.flush() beta",
    );
    assert_eq!(bbox_page_count(&plain), bbox_page_count(&flushed));
    assert_eq!(
        bbox_words(&plain),
        bbox_words(&flushed),
        "flush without floats must preserve visible text and positions",
    );

    let nested = compile_pdf_bbox(
        "flush-nested",
        "#set page(width:100pt,height:100pt,margin:0pt)\n\
         #block(height:40pt)[#place(bottom,float:true,block(width:100pt,height:30pt)[#place([NESTED_FLOAT])]) #place.flush() after #place([NESTED_AFTER])]",
    );
    let nested_float = bbox_word(&nested, "NESTED_FLOAT");
    let nested_after = bbox_word(&nested, "NESTED_AFTER");
    assert_eq!(nested_float.0, 1);
    assert_close(nested_float.1, 7.404, 0.002, "NESTED_FLOAT yMin");
    assert_eq!(nested_after.0, 1);
    assert_close(nested_after.1, 4.642, 0.002, "NESTED_AFTER yMin");
}
