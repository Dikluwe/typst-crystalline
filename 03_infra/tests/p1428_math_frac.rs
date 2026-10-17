//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/p1428-tests.md
//! @prompt-hash ab94873a
//! @layer L3
//!
//! Recalibração independente dos observadores públicos `math.frac` P1428.

use std::path::{Path, PathBuf};

use typst_core::compiler::layout::FontMetrics;
use typst_core::contracts::world::World;
use typst_core::entities::compiler_features::Features;
use typst_core::entities::content::Content;
use typst_core::entities::layout_types::TextStyle;
use typst_infra::font_metrics::FontBookMetrics;
use typst_infra::pipeline::{
    compile_to_html_string_with_features, eval_to_module_with_sink,
};
use typst_infra::world::SystemWorld;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unknown {
    HtmlEntity,
    HtmlMarkup,
    MathFracCardinality,
    ContentTraversal,
}

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

fn fixture_world(prefix: &str, source_text: &str) -> (TempDir, SystemWorld) {
    let path = std::env::temp_dir().join(format!(
        "typst-p1428-{prefix}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
    ));
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(path.join("main.typ"), source_text).unwrap();
    let dir = TempDir(path);
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    (dir, world)
}

fn fixture_world_with_case(source_text: &str, case_text: &str) -> (TempDir, SystemWorld) {
    let (dir, world) = fixture_world("module-indirect", source_text);
    std::fs::write(dir.path().join("case.typ"), case_text).unwrap();
    (dir, world)
}

fn compile(source_text: &str) -> Result<String, String> {
    let (_dir, world) = fixture_world("html", source_text);
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

fn decode_text_node(input: &str) -> Result<String, Unknown> {
    let mut output = String::new();
    let mut cursor = 0;
    while let Some(relative) = input[cursor..].find('&') {
        let amp = cursor + relative;
        output.push_str(&input[cursor..amp]);
        let tail = &input[amp + 1..];
        let Some(end) = tail.find(';') else {
            return Err(Unknown::HtmlEntity);
        };
        let reference = &tail[..end];
        let decoded = match reference {
            "amp" => '&',
            "lt" => '<',
            "gt" => '>',
            "quot" => '"',
            "apos" => '\'',
            numeric
                if numeric
                    .get(..2)
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("#x")) =>
            {
                u32::from_str_radix(&numeric[2..], 16)
                    .ok()
                    .and_then(char::from_u32)
                    .ok_or(Unknown::HtmlEntity)?
            }
            numeric if numeric.starts_with('#') => numeric[1..]
                .parse::<u32>()
                .ok()
                .and_then(char::from_u32)
                .ok_or(Unknown::HtmlEntity)?,
            _ => return Err(Unknown::HtmlEntity),
        };
        output.push(decoded);
        cursor = amp + end + 2;
    }
    output.push_str(&input[cursor..]);
    Ok(output)
}

fn tag_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut quote = None;
    for (offset, byte) in bytes[start..].iter().copied().enumerate() {
        match (quote, byte) {
            (None, b'\'' | b'"') => quote = Some(byte),
            (Some(open), close) if open == close => quote = None,
            (None, b'>') => return Some(start + offset),
            _ => {}
        }
    }
    None
}

fn tag_name(token: &str) -> Option<&str> {
    let token = token.trim_start();
    let end = token
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .unwrap_or(token.len());
    (end > 0).then_some(&token[..end])
}

fn extract_html_text_once(serialized: &str) -> Result<String, Unknown> {
    let bytes = serialized.as_bytes();
    let mut output = String::new();
    let mut stack: Vec<String> = Vec::new();
    let mut cursor = 0;
    let mut starts = 0usize;
    let mut matched = 0usize;

    while cursor < bytes.len() {
        if bytes[cursor] != b'<' {
            let end = serialized[cursor..]
                .find('<')
                .map_or(bytes.len(), |offset| cursor + offset);
            output.push_str(&decode_text_node(&serialized[cursor..end])?);
            cursor = end;
            continue;
        }

        let end = tag_end(bytes, cursor + 1).ok_or(Unknown::HtmlMarkup)?;
        let token = &serialized[cursor + 1..end];
        let trimmed = token.trim();
        if trimmed.starts_with("!--") || trimmed.starts_with('?') {
            return Err(Unknown::HtmlMarkup);
        }
        if trimmed.starts_with('!') {
            if !trimmed.to_ascii_lowercase().starts_with("!doctype ") {
                return Err(Unknown::HtmlMarkup);
            }
        } else if let Some(rest) = trimmed.strip_prefix('/') {
            let name = tag_name(rest).ok_or(Unknown::HtmlMarkup)?.to_ascii_lowercase();
            if rest[name.len()..].trim().is_empty()
                && stack.last().is_some_and(|open| open == &name)
            {
                stack.pop();
                matched += 1;
            } else {
                return Err(Unknown::HtmlMarkup);
            }
        } else {
            let name = tag_name(trimmed).ok_or(Unknown::HtmlMarkup)?.to_ascii_lowercase();
            if matches!(name.as_str(), "script" | "style" | "textarea" | "title") {
                return Err(Unknown::HtmlMarkup);
            }
            starts += 1;
            let is_void = matches!(
                name.as_str(),
                "area"
                    | "base"
                    | "br"
                    | "col"
                    | "embed"
                    | "hr"
                    | "img"
                    | "input"
                    | "link"
                    | "meta"
                    | "param"
                    | "source"
                    | "track"
                    | "wbr"
            );
            let self_closing = trimmed.ends_with('/');
            if !is_void && !self_closing {
                stack.push(name);
            }
        }
        cursor = end + 1;
    }

    if starts == 0 || matched == 0 || !stack.is_empty() {
        return Err(Unknown::HtmlMarkup);
    }
    Ok(output)
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

fn evaluate_module_carriers(
    source_text: &str,
) -> Result<Vec<(bool, bool, bool, bool)>, Unknown> {
    let (_dir, world) = fixture_world("module", source_text);
    let source = world.source(world.main()).unwrap();
    let module = eval_to_module_with_sink(&world, &source)
        .0
        .unwrap_or_else(|errors| panic!("module fixture {source_text:?}: {errors:?}"));
    module.content().map_or_else(|| Ok(Vec::new()), collect_math_fracs)
}

fn evaluate_indirect_module_carriers(
    case_text: &str,
) -> Result<Vec<(bool, bool, bool, bool)>, Unknown> {
    let main = "#include \"case.typ\"";
    let (_dir, world) = fixture_world_with_case(main, case_text);
    let source = world.source(world.main()).unwrap();
    let module = eval_to_module_with_sink(&world, &source)
        .0
        .unwrap_or_else(|errors| panic!("indirect module fixture: {errors:?}"));
    module.content().map_or_else(|| Ok(Vec::new()), collect_math_fracs)
}

fn evaluate_unique_frac(source_text: &str) -> Result<(bool, bool), Unknown> {
    unique_frac_bits_from_carriers(&evaluate_module_carriers(source_text)?)
}

fn evaluate_indirect_unique_frac(case_text: &str) -> Result<(bool, bool), Unknown> {
    unique_frac_bits_from_carriers(&evaluate_indirect_module_carriers(case_text)?)
}

fn unique_frac_bits_from_carriers(
    carriers: &[(bool, bool, bool, bool)],
) -> Result<(bool, bool), Unknown> {
    let [(line, style_is_none, numerator, denominator)] = carriers else {
        return Err(Unknown::MathFracCardinality);
    };
    assert!(*line, "unique MathFrac must retain line=true");
    assert!(*style_is_none, "slash carrier must retain style=None");
    Ok((*numerator, *denominator))
}

fn collect_math_fracs(
    content: &Content,
) -> Result<Vec<(bool, bool, bool, bool)>, Unknown> {
    let mut carriers = Vec::new();
    content
        .map_content(&mut |node| {
            if let Content::MathFrac(frac) = node {
                carriers.push((
                    frac.line,
                    frac.style.is_none(),
                    frac.num_deparenthesized,
                    frac.denom_deparenthesized,
                ));
            }
            Ok(None)
        })
        .map_err(|_| Unknown::ContentTraversal)?;
    Ok(carriers)
}

#[test]
fn p1428_repr_compara_texto_html_decodificado() {
    for style in ["vertical", "skewed", "horizontal"] {
        let serialized = html(&format!("#repr(math.frac([a], [b], style: \"{style}\"))"));
        let text = extract_html_text_once(&serialized).unwrap_or_else(|unknown| {
            panic!("{style}: unexpected {unknown:?}: {serialized}")
        });
        let expected = format!("style: \"{style}\"");
        assert!(text.contains(&expected), "{style}: {text}");
    }
}

#[test]
fn v17_numeric_html_entities_keep_unicode_validation_and_single_decode() {
    assert_eq!(decode_text_node("&#X22;&#x1F600;&#128512;"), Ok("\"😀😀".into()));
    assert_eq!(decode_text_node("&amp;#X22;"), Ok("&#X22;".into()));
    for invalid in [
        "&#x;",
        "&#X;",
        "&#;",
        "&#xD800;",
        "&#1114112;",
        "&#x110000;",
        "&éx22;",
        "&#é22;",
        "&＃x22;",
        "&#x2g;",
        "&#X22",
    ] {
        assert_eq!(decode_text_node(invalid), Err(Unknown::HtmlEntity), "{invalid}");
    }
}

#[test]
fn p1428_decoder_html_e_fail_closed() {
    assert_eq!(
        extract_html_text_once("<p>style: &quot;vertical&quot;</p>"),
        Ok("style: \"vertical\"".into())
    );
    assert_eq!(extract_html_text_once("<p>&amp;quot;</p>"), Ok("&quot;".into()));
    assert_eq!(extract_html_text_once("<p>&#34;&#x22;</p>"), Ok("\"\"".into()));
    assert_eq!(extract_html_text_once("<p>&bogus;</p>"), Err(Unknown::HtmlEntity));
    assert_eq!(
        extract_html_text_once("style: &quot;vertical&quot;"),
        Err(Unknown::HtmlMarkup)
    );
    assert_eq!(extract_html_text_once("<p>unbalanced"), Err(Unknown::HtmlMarkup));
}

#[test]
fn p1428_modulo_expoe_frac_unica_e_quatro_bits() {
    for (source, expected) in [
        ("#set math.frac(style: \"horizontal\")\n$a / b$", (false, false)),
        ("#set math.frac(style: \"horizontal\")\n$(a) / b$", (true, false)),
        ("#set math.frac(style: \"horizontal\")\n$a / (b)$", (false, true)),
        ("#set math.frac(style: \"horizontal\")\n$(a) / (b)$", (true, true)),
    ] {
        assert_eq!(evaluate_unique_frac(source), Ok(expected), "{source}");
    }
    assert_eq!(
        evaluate_unique_frac("plain text without a fraction"),
        Err(Unknown::MathFracCardinality)
    );
    assert_eq!(
        evaluate_unique_frac("$a/b$ and $c/d$"),
        Err(Unknown::MathFracCardinality)
    );
    assert_eq!(
        evaluate_indirect_unique_frac(
            "#set math.frac(style: \"horizontal\")\n$(a) / (b)$"
        ),
        Ok((true, true)),
        "carrier bits must come from the evaluated dependency, not main source spelling"
    );
}

#[test]
fn p1428_controles_rotas_diagnosticos_constantes_e_binom() {
    for (source, expected) in [
        ("#repr(math.frac([a], [b]))", "frac(num: [a], denom: [b])"),
        ("#repr($frac(a,b)$.body)", "frac(num: [a], denom: [b])"),
        ("#repr($a/b$.body)", "frac(num: [a], denom: [b])"),
    ] {
        let text = extract_html_text_once(&html(source)).unwrap();
        assert!(text.contains(expected), "{source}: {text}");
    }

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

    let bytes = include_bytes!("../fixtures/fonts/NewCMMath-Book.otf");
    let productive = productive_constants(bytes);
    assert_eq!(productive.skewed_fraction_vertical_gap, 96.0);
    assert_eq!(productive.skewed_fraction_horizontal_gap, 350.0);
    for constant_offset in [156usize, 160usize] {
        let mut zeroed = bytes.to_vec();
        let offset = math_constant_offset(&zeroed, constant_offset);
        zeroed[offset..offset + 2].copy_from_slice(&0i16.to_be_bytes());
        let extracted = productive_constants(&zeroed);
        if constant_offset == 156 {
            assert_eq!(extracted.skewed_fraction_horizontal_gap, 0.0);
            assert_eq!(extracted.skewed_fraction_vertical_gap, 96.0);
        } else {
            assert_eq!(extracted.skewed_fraction_vertical_gap, 0.0);
            assert_eq!(extracted.skewed_fraction_horizontal_gap, 350.0);
        }
    }

    let before = extract_html_text_once(&html("#repr($binom(n,k)$.body)")).unwrap();
    let after = extract_html_text_once(&html(
        "#set math.frac(style: \"skewed\"); #repr($binom(n,k)$.body)",
    ))
    .unwrap();
    let expected = "binom(upper: [n], lower: ([k],))";
    assert!(before.contains(expected), "{before}");
    assert!(after.contains(expected), "{after}");
    assert!(!after.contains("style:") && !after.contains("frac("), "{after}");
}
